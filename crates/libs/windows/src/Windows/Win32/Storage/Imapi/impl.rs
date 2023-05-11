#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DDiscFormat2DataEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Update(&self, object: ::core::option::Option<&super::super::System::Com::IDispatch>, progress: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for DDiscFormat2DataEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DDiscFormat2DataEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: DDiscFormat2DataEvents_Impl, const OFFSET: isize>() -> DDiscFormat2DataEvents_Vtbl {
        unsafe extern "system" fn Update<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: DDiscFormat2DataEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, progress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Update(::windows_core::from_raw_borrowed(&object), ::windows_core::from_raw_borrowed(&progress)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Update: Update::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<DDiscFormat2DataEvents as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DDiscFormat2EraseEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Update(&self, object: ::core::option::Option<&super::super::System::Com::IDispatch>, elapsedseconds: i32, estimatedtotalseconds: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for DDiscFormat2EraseEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DDiscFormat2EraseEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: DDiscFormat2EraseEvents_Impl, const OFFSET: isize>() -> DDiscFormat2EraseEvents_Vtbl {
        unsafe extern "system" fn Update<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: DDiscFormat2EraseEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, elapsedseconds: i32, estimatedtotalseconds: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Update(::windows_core::from_raw_borrowed(&object), ::core::mem::transmute_copy(&elapsedseconds), ::core::mem::transmute_copy(&estimatedtotalseconds)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Update: Update::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<DDiscFormat2EraseEvents as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DDiscFormat2RawCDEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Update(&self, object: ::core::option::Option<&super::super::System::Com::IDispatch>, progress: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for DDiscFormat2RawCDEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DDiscFormat2RawCDEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: DDiscFormat2RawCDEvents_Impl, const OFFSET: isize>() -> DDiscFormat2RawCDEvents_Vtbl {
        unsafe extern "system" fn Update<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: DDiscFormat2RawCDEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, progress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Update(::windows_core::from_raw_borrowed(&object), ::windows_core::from_raw_borrowed(&progress)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Update: Update::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<DDiscFormat2RawCDEvents as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DDiscFormat2TrackAtOnceEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Update(&self, object: ::core::option::Option<&super::super::System::Com::IDispatch>, progress: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for DDiscFormat2TrackAtOnceEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DDiscFormat2TrackAtOnceEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: DDiscFormat2TrackAtOnceEvents_Impl, const OFFSET: isize>() -> DDiscFormat2TrackAtOnceEvents_Vtbl {
        unsafe extern "system" fn Update<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: DDiscFormat2TrackAtOnceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, progress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Update(::windows_core::from_raw_borrowed(&object), ::windows_core::from_raw_borrowed(&progress)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Update: Update::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<DDiscFormat2TrackAtOnceEvents as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DDiscMaster2Events_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn NotifyDeviceAdded(&self, object: ::core::option::Option<&super::super::System::Com::IDispatch>, uniqueid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn NotifyDeviceRemoved(&self, object: ::core::option::Option<&super::super::System::Com::IDispatch>, uniqueid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for DDiscMaster2Events {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DDiscMaster2Events_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: DDiscMaster2Events_Impl, const OFFSET: isize>() -> DDiscMaster2Events_Vtbl {
        unsafe extern "system" fn NotifyDeviceAdded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: DDiscMaster2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, uniqueid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyDeviceAdded(::windows_core::from_raw_borrowed(&object), ::core::mem::transmute(&uniqueid)).into()
        }
        unsafe extern "system" fn NotifyDeviceRemoved<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: DDiscMaster2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, uniqueid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyDeviceRemoved(::windows_core::from_raw_borrowed(&object), ::core::mem::transmute(&uniqueid)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            NotifyDeviceAdded: NotifyDeviceAdded::<Identity, Impl, OFFSET>,
            NotifyDeviceRemoved: NotifyDeviceRemoved::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<DDiscMaster2Events as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DFileSystemImageEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Update(&self, object: ::core::option::Option<&super::super::System::Com::IDispatch>, currentfile: &::windows_core::BSTR, copiedsectors: i32, totalsectors: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for DFileSystemImageEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DFileSystemImageEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: DFileSystemImageEvents_Impl, const OFFSET: isize>() -> DFileSystemImageEvents_Vtbl {
        unsafe extern "system" fn Update<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: DFileSystemImageEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, currentfile: ::std::mem::MaybeUninit<::windows_core::BSTR>, copiedsectors: i32, totalsectors: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Update(::windows_core::from_raw_borrowed(&object), ::core::mem::transmute(&currentfile), ::core::mem::transmute_copy(&copiedsectors), ::core::mem::transmute_copy(&totalsectors)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Update: Update::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<DFileSystemImageEvents as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DFileSystemImageImportEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn UpdateImport(&self, object: ::core::option::Option<&super::super::System::Com::IDispatch>, filesystem: FsiFileSystems, currentitem: &::windows_core::BSTR, importeddirectoryitems: i32, totaldirectoryitems: i32, importedfileitems: i32, totalfileitems: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for DFileSystemImageImportEvents {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DFileSystemImageImportEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: DFileSystemImageImportEvents_Impl, const OFFSET: isize>() -> DFileSystemImageImportEvents_Vtbl {
        unsafe extern "system" fn UpdateImport<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: DFileSystemImageImportEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, filesystem: FsiFileSystems, currentitem: ::std::mem::MaybeUninit<::windows_core::BSTR>, importeddirectoryitems: i32, totaldirectoryitems: i32, importedfileitems: i32, totalfileitems: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateImport(::windows_core::from_raw_borrowed(&object), ::core::mem::transmute_copy(&filesystem), ::core::mem::transmute(&currentitem), ::core::mem::transmute_copy(&importeddirectoryitems), ::core::mem::transmute_copy(&totaldirectoryitems), ::core::mem::transmute_copy(&importedfileitems), ::core::mem::transmute_copy(&totalfileitems)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), UpdateImport: UpdateImport::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<DFileSystemImageImportEvents as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait DWriteEngine2Events_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Update(&self, object: ::core::option::Option<&super::super::System::Com::IDispatch>, progress: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for DWriteEngine2Events {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl DWriteEngine2Events_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: DWriteEngine2Events_Impl, const OFFSET: isize>() -> DWriteEngine2Events_Vtbl {
        unsafe extern "system" fn Update<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: DWriteEngine2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: *mut ::core::ffi::c_void, progress: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Update(::windows_core::from_raw_borrowed(&object), ::windows_core::from_raw_borrowed(&progress)).into()
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), Update: Update::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<DWriteEngine2Events as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IBlockRange_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StartLba(&self) -> ::windows_core::Result<i32>;
    fn EndLba(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IBlockRange {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IBlockRange_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBlockRange_Impl, const OFFSET: isize>() -> IBlockRange_Vtbl {
        unsafe extern "system" fn StartLba<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBlockRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartLba() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndLba<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBlockRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndLba() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            StartLba: StartLba::<Identity, Impl, OFFSET>,
            EndLba: EndLba::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IBlockRange as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IBlockRangeList_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn BlockRanges(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IBlockRangeList {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IBlockRangeList_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBlockRangeList_Impl, const OFFSET: isize>() -> IBlockRangeList_Vtbl {
        unsafe extern "system" fn BlockRanges<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBlockRangeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BlockRanges() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(), BlockRanges: BlockRanges::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IBlockRangeList as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IBootOptions_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn BootImage(&self) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn Manufacturer(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetManufacturer(&self, newval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn PlatformId(&self) -> ::windows_core::Result<PlatformId>;
    fn SetPlatformId(&self, newval: PlatformId) -> ::windows_core::Result<()>;
    fn Emulation(&self) -> ::windows_core::Result<EmulationType>;
    fn SetEmulation(&self, newval: EmulationType) -> ::windows_core::Result<()>;
    fn ImageSize(&self) -> ::windows_core::Result<u32>;
    fn AssignBootImage(&self, newval: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IBootOptions {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IBootOptions_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBootOptions_Impl, const OFFSET: isize>() -> IBootOptions_Vtbl {
        unsafe extern "system" fn BootImage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BootImage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Manufacturer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Manufacturer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManufacturer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetManufacturer(::core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn PlatformId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut PlatformId) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PlatformId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlatformId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: PlatformId) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPlatformId(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Emulation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut EmulationType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Emulation() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmulation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: EmulationType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEmulation(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ImageSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ImageSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssignBootImage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AssignBootImage(::windows_core::from_raw_borrowed(&newval)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            BootImage: BootImage::<Identity, Impl, OFFSET>,
            Manufacturer: Manufacturer::<Identity, Impl, OFFSET>,
            SetManufacturer: SetManufacturer::<Identity, Impl, OFFSET>,
            PlatformId: PlatformId::<Identity, Impl, OFFSET>,
            SetPlatformId: SetPlatformId::<Identity, Impl, OFFSET>,
            Emulation: Emulation::<Identity, Impl, OFFSET>,
            SetEmulation: SetEmulation::<Identity, Impl, OFFSET>,
            ImageSize: ImageSize::<Identity, Impl, OFFSET>,
            AssignBootImage: AssignBootImage::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IBootOptions as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"implement\"`*"]
pub trait IBurnVerification_Impl: Sized {
    fn SetBurnVerificationLevel(&self, value: IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows_core::Result<()>;
    fn BurnVerificationLevel(&self) -> ::windows_core::Result<IMAPI_BURN_VERIFICATION_LEVEL>;
}
impl ::windows_core::RuntimeName for IBurnVerification {}
impl IBurnVerification_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBurnVerification_Impl, const OFFSET: isize>() -> IBurnVerification_Vtbl {
        unsafe extern "system" fn SetBurnVerificationLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBurnVerification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBurnVerificationLevel(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn BurnVerificationLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IBurnVerification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BurnVerificationLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetBurnVerificationLevel: SetBurnVerificationLevel::<Identity, Impl, OFFSET>,
            BurnVerificationLevel: BurnVerificationLevel::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IBurnVerification as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn IsRecorderSupported(&self, recorder: ::core::option::Option<&IDiscRecorder2>) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsCurrentMediaSupported(&self, recorder: ::core::option::Option<&IDiscRecorder2>) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MediaPhysicallyBlank(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MediaHeuristicallyBlank(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SupportedMediaTypes(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IDiscFormat2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscFormat2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2_Impl, const OFFSET: isize>() -> IDiscFormat2_Vtbl {
        unsafe extern "system" fn IsRecorderSupported<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recorder: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsRecorderSupported(::windows_core::from_raw_borrowed(&recorder)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCurrentMediaSupported<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recorder: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsCurrentMediaSupported(::windows_core::from_raw_borrowed(&recorder)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaPhysicallyBlank<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MediaPhysicallyBlank() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaHeuristicallyBlank<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MediaHeuristicallyBlank() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedMediaTypes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportedMediaTypes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsRecorderSupported: IsRecorderSupported::<Identity, Impl, OFFSET>,
            IsCurrentMediaSupported: IsCurrentMediaSupported::<Identity, Impl, OFFSET>,
            MediaPhysicallyBlank: MediaPhysicallyBlank::<Identity, Impl, OFFSET>,
            MediaHeuristicallyBlank: MediaHeuristicallyBlank::<Identity, Impl, OFFSET>,
            SupportedMediaTypes: SupportedMediaTypes::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDiscFormat2 as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2Data_Impl: Sized + IDiscFormat2_Impl {
    fn SetRecorder(&self, value: ::core::option::Option<&IDiscRecorder2>) -> ::windows_core::Result<()>;
    fn Recorder(&self) -> ::windows_core::Result<IDiscRecorder2>;
    fn SetBufferUnderrunFreeDisabled(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn BufferUnderrunFreeDisabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetPostgapAlreadyInImage(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn PostgapAlreadyInImage(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CurrentMediaStatus(&self) -> ::windows_core::Result<IMAPI_FORMAT2_DATA_MEDIA_STATE>;
    fn WriteProtectStatus(&self) -> ::windows_core::Result<IMAPI_MEDIA_WRITE_PROTECT_STATE>;
    fn TotalSectorsOnMedia(&self) -> ::windows_core::Result<i32>;
    fn FreeSectorsOnMedia(&self) -> ::windows_core::Result<i32>;
    fn NextWritableAddress(&self) -> ::windows_core::Result<i32>;
    fn StartAddressOfPreviousSession(&self) -> ::windows_core::Result<i32>;
    fn LastWrittenAddressOfPreviousSession(&self) -> ::windows_core::Result<i32>;
    fn SetForceMediaToBeClosed(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn ForceMediaToBeClosed(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetDisableConsumerDvdCompatibilityMode(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn DisableConsumerDvdCompatibilityMode(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CurrentPhysicalMediaType(&self) -> ::windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SetClientName(&self, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ClientName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RequestedWriteSpeed(&self) -> ::windows_core::Result<i32>;
    fn RequestedRotationTypeIsPureCAV(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CurrentWriteSpeed(&self) -> ::windows_core::Result<i32>;
    fn CurrentRotationTypeIsPureCAV(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SupportedWriteSpeeds(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedWriteSpeedDescriptors(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetForceOverwrite(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn ForceOverwrite(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn MultisessionInterfaces(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn Write(&self, data: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn CancelWrite(&self) -> ::windows_core::Result<()>;
    fn SetWriteSpeed(&self, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IDiscFormat2Data {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscFormat2Data_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>() -> IDiscFormat2Data_Vtbl {
        unsafe extern "system" fn SetRecorder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRecorder(::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn Recorder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Recorder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBufferUnderrunFreeDisabled(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BufferUnderrunFreeDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostgapAlreadyInImage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPostgapAlreadyInImage(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn PostgapAlreadyInImage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PostgapAlreadyInImage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentMediaStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_DATA_MEDIA_STATE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentMediaStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteProtectStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_WRITE_PROTECT_STATE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WriteProtectStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TotalSectorsOnMedia() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FreeSectorsOnMedia() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextWritableAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NextWritableAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAddressOfPreviousSession<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartAddressOfPreviousSession() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastWrittenAddressOfPreviousSession<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastWrittenAddressOfPreviousSession() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceMediaToBeClosed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetForceMediaToBeClosed(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ForceMediaToBeClosed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ForceMediaToBeClosed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisableConsumerDvdCompatibilityMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisableConsumerDvdCompatibilityMode(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn DisableConsumerDvdCompatibilityMode<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisableConsumerDvdCompatibilityMode() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentPhysicalMediaType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClientName(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ClientName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClientName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedWriteSpeed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RequestedWriteSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RequestedRotationTypeIsPureCAV() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentWriteSpeed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentWriteSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentRotationTypeIsPureCAV() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportedWriteSpeeds() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supportedspeeds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportedWriteSpeedDescriptors() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supportedspeeddescriptors, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceOverwrite<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetForceOverwrite(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ForceOverwrite<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ForceOverwrite() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MultisessionInterfaces<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MultisessionInterfaces() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Write(::windows_core::from_raw_borrowed(&data)).into()
        }
        unsafe extern "system" fn CancelWrite<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelWrite().into()
        }
        unsafe extern "system" fn SetWriteSpeed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWriteSpeed(::core::mem::transmute_copy(&requestedsectorspersecond), ::core::mem::transmute_copy(&rotationtypeispurecav)).into()
        }
        Self {
            base__: IDiscFormat2_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetRecorder: SetRecorder::<Identity, Impl, OFFSET>,
            Recorder: Recorder::<Identity, Impl, OFFSET>,
            SetBufferUnderrunFreeDisabled: SetBufferUnderrunFreeDisabled::<Identity, Impl, OFFSET>,
            BufferUnderrunFreeDisabled: BufferUnderrunFreeDisabled::<Identity, Impl, OFFSET>,
            SetPostgapAlreadyInImage: SetPostgapAlreadyInImage::<Identity, Impl, OFFSET>,
            PostgapAlreadyInImage: PostgapAlreadyInImage::<Identity, Impl, OFFSET>,
            CurrentMediaStatus: CurrentMediaStatus::<Identity, Impl, OFFSET>,
            WriteProtectStatus: WriteProtectStatus::<Identity, Impl, OFFSET>,
            TotalSectorsOnMedia: TotalSectorsOnMedia::<Identity, Impl, OFFSET>,
            FreeSectorsOnMedia: FreeSectorsOnMedia::<Identity, Impl, OFFSET>,
            NextWritableAddress: NextWritableAddress::<Identity, Impl, OFFSET>,
            StartAddressOfPreviousSession: StartAddressOfPreviousSession::<Identity, Impl, OFFSET>,
            LastWrittenAddressOfPreviousSession: LastWrittenAddressOfPreviousSession::<Identity, Impl, OFFSET>,
            SetForceMediaToBeClosed: SetForceMediaToBeClosed::<Identity, Impl, OFFSET>,
            ForceMediaToBeClosed: ForceMediaToBeClosed::<Identity, Impl, OFFSET>,
            SetDisableConsumerDvdCompatibilityMode: SetDisableConsumerDvdCompatibilityMode::<Identity, Impl, OFFSET>,
            DisableConsumerDvdCompatibilityMode: DisableConsumerDvdCompatibilityMode::<Identity, Impl, OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Identity, Impl, OFFSET>,
            SetClientName: SetClientName::<Identity, Impl, OFFSET>,
            ClientName: ClientName::<Identity, Impl, OFFSET>,
            RequestedWriteSpeed: RequestedWriteSpeed::<Identity, Impl, OFFSET>,
            RequestedRotationTypeIsPureCAV: RequestedRotationTypeIsPureCAV::<Identity, Impl, OFFSET>,
            CurrentWriteSpeed: CurrentWriteSpeed::<Identity, Impl, OFFSET>,
            CurrentRotationTypeIsPureCAV: CurrentRotationTypeIsPureCAV::<Identity, Impl, OFFSET>,
            SupportedWriteSpeeds: SupportedWriteSpeeds::<Identity, Impl, OFFSET>,
            SupportedWriteSpeedDescriptors: SupportedWriteSpeedDescriptors::<Identity, Impl, OFFSET>,
            SetForceOverwrite: SetForceOverwrite::<Identity, Impl, OFFSET>,
            ForceOverwrite: ForceOverwrite::<Identity, Impl, OFFSET>,
            MultisessionInterfaces: MultisessionInterfaces::<Identity, Impl, OFFSET>,
            Write: Write::<Identity, Impl, OFFSET>,
            CancelWrite: CancelWrite::<Identity, Impl, OFFSET>,
            SetWriteSpeed: SetWriteSpeed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDiscFormat2Data as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IDiscFormat2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2DataEventArgs_Impl: Sized + IWriteEngine2EventArgs_Impl {
    fn ElapsedTime(&self) -> ::windows_core::Result<i32>;
    fn RemainingTime(&self) -> ::windows_core::Result<i32>;
    fn TotalTime(&self) -> ::windows_core::Result<i32>;
    fn CurrentAction(&self) -> ::windows_core::Result<IMAPI_FORMAT2_DATA_WRITE_ACTION>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IDiscFormat2DataEventArgs {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscFormat2DataEventArgs_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>() -> IDiscFormat2DataEventArgs_Vtbl {
        unsafe extern "system" fn ElapsedTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ElapsedTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemainingTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RemainingTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TotalTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_DATA_WRITE_ACTION) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentAction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWriteEngine2EventArgs_Vtbl::new::<Identity, Impl, OFFSET>(),
            ElapsedTime: ElapsedTime::<Identity, Impl, OFFSET>,
            RemainingTime: RemainingTime::<Identity, Impl, OFFSET>,
            TotalTime: TotalTime::<Identity, Impl, OFFSET>,
            CurrentAction: CurrentAction::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDiscFormat2DataEventArgs as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IWriteEngine2EventArgs as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2Erase_Impl: Sized + IDiscFormat2_Impl {
    fn SetRecorder(&self, value: ::core::option::Option<&IDiscRecorder2>) -> ::windows_core::Result<()>;
    fn Recorder(&self) -> ::windows_core::Result<IDiscRecorder2>;
    fn SetFullErase(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn FullErase(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CurrentPhysicalMediaType(&self) -> ::windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SetClientName(&self, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ClientName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn EraseMedia(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IDiscFormat2Erase {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscFormat2Erase_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Erase_Impl, const OFFSET: isize>() -> IDiscFormat2Erase_Vtbl {
        unsafe extern "system" fn SetRecorder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRecorder(::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn Recorder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Recorder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFullErase<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFullErase(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn FullErase<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FullErase() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentPhysicalMediaType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClientName(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ClientName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClientName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EraseMedia<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EraseMedia().into()
        }
        Self {
            base__: IDiscFormat2_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetRecorder: SetRecorder::<Identity, Impl, OFFSET>,
            Recorder: Recorder::<Identity, Impl, OFFSET>,
            SetFullErase: SetFullErase::<Identity, Impl, OFFSET>,
            FullErase: FullErase::<Identity, Impl, OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Identity, Impl, OFFSET>,
            SetClientName: SetClientName::<Identity, Impl, OFFSET>,
            ClientName: ClientName::<Identity, Impl, OFFSET>,
            EraseMedia: EraseMedia::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDiscFormat2Erase as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IDiscFormat2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2RawCD_Impl: Sized + IDiscFormat2_Impl {
    fn PrepareMedia(&self) -> ::windows_core::Result<()>;
    fn WriteMedia(&self, data: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn WriteMedia2(&self, data: ::core::option::Option<&super::super::System::Com::IStream>, streamleadinsectors: i32) -> ::windows_core::Result<()>;
    fn CancelWrite(&self) -> ::windows_core::Result<()>;
    fn ReleaseMedia(&self) -> ::windows_core::Result<()>;
    fn SetWriteSpeed(&self, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetRecorder(&self, value: ::core::option::Option<&IDiscRecorder2>) -> ::windows_core::Result<()>;
    fn Recorder(&self) -> ::windows_core::Result<IDiscRecorder2>;
    fn SetBufferUnderrunFreeDisabled(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn BufferUnderrunFreeDisabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn StartOfNextSession(&self) -> ::windows_core::Result<i32>;
    fn LastPossibleStartOfLeadout(&self) -> ::windows_core::Result<i32>;
    fn CurrentPhysicalMediaType(&self) -> ::windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SupportedSectorTypes(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetRequestedSectorType(&self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows_core::Result<()>;
    fn RequestedSectorType(&self) -> ::windows_core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE>;
    fn SetClientName(&self, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ClientName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RequestedWriteSpeed(&self) -> ::windows_core::Result<i32>;
    fn RequestedRotationTypeIsPureCAV(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CurrentWriteSpeed(&self) -> ::windows_core::Result<i32>;
    fn CurrentRotationTypeIsPureCAV(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SupportedWriteSpeeds(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedWriteSpeedDescriptors(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IDiscFormat2RawCD {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscFormat2RawCD_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>() -> IDiscFormat2RawCD_Vtbl {
        unsafe extern "system" fn PrepareMedia<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PrepareMedia().into()
        }
        unsafe extern "system" fn WriteMedia<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteMedia(::windows_core::from_raw_borrowed(&data)).into()
        }
        unsafe extern "system" fn WriteMedia2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, streamleadinsectors: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteMedia2(::windows_core::from_raw_borrowed(&data), ::core::mem::transmute_copy(&streamleadinsectors)).into()
        }
        unsafe extern "system" fn CancelWrite<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelWrite().into()
        }
        unsafe extern "system" fn ReleaseMedia<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseMedia().into()
        }
        unsafe extern "system" fn SetWriteSpeed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWriteSpeed(::core::mem::transmute_copy(&requestedsectorspersecond), ::core::mem::transmute_copy(&rotationtypeispurecav)).into()
        }
        unsafe extern "system" fn SetRecorder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRecorder(::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn Recorder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Recorder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBufferUnderrunFreeDisabled(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BufferUnderrunFreeDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartOfNextSession<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartOfNextSession() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastPossibleStartOfLeadout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastPossibleStartOfLeadout() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentPhysicalMediaType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedSectorTypes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportedSectorTypes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestedSectorType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRequestedSectorType(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn RequestedSectorType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RequestedSectorType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClientName(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ClientName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClientName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedWriteSpeed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RequestedWriteSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RequestedRotationTypeIsPureCAV() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentWriteSpeed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentWriteSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentRotationTypeIsPureCAV() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportedWriteSpeeds() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supportedspeeds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportedWriteSpeedDescriptors() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supportedspeeddescriptors, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDiscFormat2_Vtbl::new::<Identity, Impl, OFFSET>(),
            PrepareMedia: PrepareMedia::<Identity, Impl, OFFSET>,
            WriteMedia: WriteMedia::<Identity, Impl, OFFSET>,
            WriteMedia2: WriteMedia2::<Identity, Impl, OFFSET>,
            CancelWrite: CancelWrite::<Identity, Impl, OFFSET>,
            ReleaseMedia: ReleaseMedia::<Identity, Impl, OFFSET>,
            SetWriteSpeed: SetWriteSpeed::<Identity, Impl, OFFSET>,
            SetRecorder: SetRecorder::<Identity, Impl, OFFSET>,
            Recorder: Recorder::<Identity, Impl, OFFSET>,
            SetBufferUnderrunFreeDisabled: SetBufferUnderrunFreeDisabled::<Identity, Impl, OFFSET>,
            BufferUnderrunFreeDisabled: BufferUnderrunFreeDisabled::<Identity, Impl, OFFSET>,
            StartOfNextSession: StartOfNextSession::<Identity, Impl, OFFSET>,
            LastPossibleStartOfLeadout: LastPossibleStartOfLeadout::<Identity, Impl, OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Identity, Impl, OFFSET>,
            SupportedSectorTypes: SupportedSectorTypes::<Identity, Impl, OFFSET>,
            SetRequestedSectorType: SetRequestedSectorType::<Identity, Impl, OFFSET>,
            RequestedSectorType: RequestedSectorType::<Identity, Impl, OFFSET>,
            SetClientName: SetClientName::<Identity, Impl, OFFSET>,
            ClientName: ClientName::<Identity, Impl, OFFSET>,
            RequestedWriteSpeed: RequestedWriteSpeed::<Identity, Impl, OFFSET>,
            RequestedRotationTypeIsPureCAV: RequestedRotationTypeIsPureCAV::<Identity, Impl, OFFSET>,
            CurrentWriteSpeed: CurrentWriteSpeed::<Identity, Impl, OFFSET>,
            CurrentRotationTypeIsPureCAV: CurrentRotationTypeIsPureCAV::<Identity, Impl, OFFSET>,
            SupportedWriteSpeeds: SupportedWriteSpeeds::<Identity, Impl, OFFSET>,
            SupportedWriteSpeedDescriptors: SupportedWriteSpeedDescriptors::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDiscFormat2RawCD as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IDiscFormat2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2RawCDEventArgs_Impl: Sized + IWriteEngine2EventArgs_Impl {
    fn CurrentAction(&self) -> ::windows_core::Result<IMAPI_FORMAT2_RAW_CD_WRITE_ACTION>;
    fn ElapsedTime(&self) -> ::windows_core::Result<i32>;
    fn RemainingTime(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IDiscFormat2RawCDEventArgs {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscFormat2RawCDEventArgs_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>() -> IDiscFormat2RawCDEventArgs_Vtbl {
        unsafe extern "system" fn CurrentAction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_WRITE_ACTION) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentAction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElapsedTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ElapsedTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemainingTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RemainingTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWriteEngine2EventArgs_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentAction: CurrentAction::<Identity, Impl, OFFSET>,
            ElapsedTime: ElapsedTime::<Identity, Impl, OFFSET>,
            RemainingTime: RemainingTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDiscFormat2RawCDEventArgs as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IWriteEngine2EventArgs as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2TrackAtOnce_Impl: Sized + IDiscFormat2_Impl {
    fn PrepareMedia(&self) -> ::windows_core::Result<()>;
    fn AddAudioTrack(&self, data: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn CancelAddTrack(&self) -> ::windows_core::Result<()>;
    fn ReleaseMedia(&self) -> ::windows_core::Result<()>;
    fn SetWriteSpeed(&self, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn SetRecorder(&self, value: ::core::option::Option<&IDiscRecorder2>) -> ::windows_core::Result<()>;
    fn Recorder(&self) -> ::windows_core::Result<IDiscRecorder2>;
    fn SetBufferUnderrunFreeDisabled(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn BufferUnderrunFreeDisabled(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn NumberOfExistingTracks(&self) -> ::windows_core::Result<i32>;
    fn TotalSectorsOnMedia(&self) -> ::windows_core::Result<i32>;
    fn FreeSectorsOnMedia(&self) -> ::windows_core::Result<i32>;
    fn UsedSectorsOnMedia(&self) -> ::windows_core::Result<i32>;
    fn SetDoNotFinalizeMedia(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn DoNotFinalizeMedia(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ExpectedTableOfContents(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CurrentPhysicalMediaType(&self) -> ::windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SetClientName(&self, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ClientName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn RequestedWriteSpeed(&self) -> ::windows_core::Result<i32>;
    fn RequestedRotationTypeIsPureCAV(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn CurrentWriteSpeed(&self) -> ::windows_core::Result<i32>;
    fn CurrentRotationTypeIsPureCAV(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SupportedWriteSpeeds(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedWriteSpeedDescriptors(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IDiscFormat2TrackAtOnce {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscFormat2TrackAtOnce_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>() -> IDiscFormat2TrackAtOnce_Vtbl {
        unsafe extern "system" fn PrepareMedia<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PrepareMedia().into()
        }
        unsafe extern "system" fn AddAudioTrack<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddAudioTrack(::windows_core::from_raw_borrowed(&data)).into()
        }
        unsafe extern "system" fn CancelAddTrack<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelAddTrack().into()
        }
        unsafe extern "system" fn ReleaseMedia<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseMedia().into()
        }
        unsafe extern "system" fn SetWriteSpeed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWriteSpeed(::core::mem::transmute_copy(&requestedsectorspersecond), ::core::mem::transmute_copy(&rotationtypeispurecav)).into()
        }
        unsafe extern "system" fn SetRecorder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRecorder(::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn Recorder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Recorder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBufferUnderrunFreeDisabled(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BufferUnderrunFreeDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfExistingTracks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NumberOfExistingTracks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TotalSectorsOnMedia() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FreeSectorsOnMedia() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsedSectorsOnMedia<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UsedSectorsOnMedia() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDoNotFinalizeMedia<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDoNotFinalizeMedia(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn DoNotFinalizeMedia<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DoNotFinalizeMedia() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpectedTableOfContents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExpectedTableOfContents() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentPhysicalMediaType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetClientName(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn ClientName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ClientName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedWriteSpeed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RequestedWriteSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RequestedRotationTypeIsPureCAV() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentWriteSpeed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentWriteSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentRotationTypeIsPureCAV() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportedWriteSpeeds() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supportedspeeds, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportedWriteSpeedDescriptors() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(supportedspeeddescriptors, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IDiscFormat2_Vtbl::new::<Identity, Impl, OFFSET>(),
            PrepareMedia: PrepareMedia::<Identity, Impl, OFFSET>,
            AddAudioTrack: AddAudioTrack::<Identity, Impl, OFFSET>,
            CancelAddTrack: CancelAddTrack::<Identity, Impl, OFFSET>,
            ReleaseMedia: ReleaseMedia::<Identity, Impl, OFFSET>,
            SetWriteSpeed: SetWriteSpeed::<Identity, Impl, OFFSET>,
            SetRecorder: SetRecorder::<Identity, Impl, OFFSET>,
            Recorder: Recorder::<Identity, Impl, OFFSET>,
            SetBufferUnderrunFreeDisabled: SetBufferUnderrunFreeDisabled::<Identity, Impl, OFFSET>,
            BufferUnderrunFreeDisabled: BufferUnderrunFreeDisabled::<Identity, Impl, OFFSET>,
            NumberOfExistingTracks: NumberOfExistingTracks::<Identity, Impl, OFFSET>,
            TotalSectorsOnMedia: TotalSectorsOnMedia::<Identity, Impl, OFFSET>,
            FreeSectorsOnMedia: FreeSectorsOnMedia::<Identity, Impl, OFFSET>,
            UsedSectorsOnMedia: UsedSectorsOnMedia::<Identity, Impl, OFFSET>,
            SetDoNotFinalizeMedia: SetDoNotFinalizeMedia::<Identity, Impl, OFFSET>,
            DoNotFinalizeMedia: DoNotFinalizeMedia::<Identity, Impl, OFFSET>,
            ExpectedTableOfContents: ExpectedTableOfContents::<Identity, Impl, OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Identity, Impl, OFFSET>,
            SetClientName: SetClientName::<Identity, Impl, OFFSET>,
            ClientName: ClientName::<Identity, Impl, OFFSET>,
            RequestedWriteSpeed: RequestedWriteSpeed::<Identity, Impl, OFFSET>,
            RequestedRotationTypeIsPureCAV: RequestedRotationTypeIsPureCAV::<Identity, Impl, OFFSET>,
            CurrentWriteSpeed: CurrentWriteSpeed::<Identity, Impl, OFFSET>,
            CurrentRotationTypeIsPureCAV: CurrentRotationTypeIsPureCAV::<Identity, Impl, OFFSET>,
            SupportedWriteSpeeds: SupportedWriteSpeeds::<Identity, Impl, OFFSET>,
            SupportedWriteSpeedDescriptors: SupportedWriteSpeedDescriptors::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDiscFormat2TrackAtOnce as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IDiscFormat2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscFormat2TrackAtOnceEventArgs_Impl: Sized + IWriteEngine2EventArgs_Impl {
    fn CurrentTrackNumber(&self) -> ::windows_core::Result<i32>;
    fn CurrentAction(&self) -> ::windows_core::Result<IMAPI_FORMAT2_TAO_WRITE_ACTION>;
    fn ElapsedTime(&self) -> ::windows_core::Result<i32>;
    fn RemainingTime(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IDiscFormat2TrackAtOnceEventArgs {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscFormat2TrackAtOnceEventArgs_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>() -> IDiscFormat2TrackAtOnceEventArgs_Vtbl {
        unsafe extern "system" fn CurrentTrackNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentTrackNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAction<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_TAO_WRITE_ACTION) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentAction() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElapsedTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ElapsedTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemainingTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RemainingTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IWriteEngine2EventArgs_Vtbl::new::<Identity, Impl, OFFSET>(),
            CurrentTrackNumber: CurrentTrackNumber::<Identity, Impl, OFFSET>,
            CurrentAction: CurrentAction::<Identity, Impl, OFFSET>,
            ElapsedTime: ElapsedTime::<Identity, Impl, OFFSET>,
            RemainingTime: RemainingTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDiscFormat2TrackAtOnceEventArgs as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IWriteEngine2EventArgs as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"implement\"`*"]
pub trait IDiscMaster_Impl: Sized {
    fn Open(&self) -> ::windows_core::Result<()>;
    fn EnumDiscMasterFormats(&self) -> ::windows_core::Result<IEnumDiscMasterFormats>;
    fn GetActiveDiscMasterFormat(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn SetActiveDiscMasterFormat(&self, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::Result<()>;
    fn EnumDiscRecorders(&self) -> ::windows_core::Result<IEnumDiscRecorders>;
    fn GetActiveDiscRecorder(&self) -> ::windows_core::Result<IDiscRecorder>;
    fn SetActiveDiscRecorder(&self, precorder: ::core::option::Option<&IDiscRecorder>) -> ::windows_core::Result<()>;
    fn ClearFormatContent(&self) -> ::windows_core::Result<()>;
    fn ProgressAdvise(&self, pevents: ::core::option::Option<&IDiscMasterProgressEvents>) -> ::windows_core::Result<usize>;
    fn ProgressUnadvise(&self, vcookie: usize) -> ::windows_core::Result<()>;
    fn RecordDisc(&self, bsimulate: u8, bejectafterburn: u8) -> ::windows_core::Result<()>;
    fn Close(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDiscMaster {}
impl IDiscMaster_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMaster_Impl, const OFFSET: isize>() -> IDiscMaster_Vtbl {
        unsafe extern "system" fn Open<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Open().into()
        }
        unsafe extern "system" fn EnumDiscMasterFormats<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumDiscMasterFormats() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveDiscMasterFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetActiveDiscMasterFormat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(lpiid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveDiscMasterFormat<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows_core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetActiveDiscMasterFormat(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn EnumDiscRecorders<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumDiscRecorders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveDiscRecorder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprecorder: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetActiveDiscRecorder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprecorder, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveDiscRecorder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precorder: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetActiveDiscRecorder(::windows_core::from_raw_borrowed(&precorder)).into()
        }
        unsafe extern "system" fn ClearFormatContent<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearFormatContent().into()
        }
        unsafe extern "system" fn ProgressAdvise<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevents: *mut ::core::ffi::c_void, pvcookie: *mut usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProgressAdvise(::windows_core::from_raw_borrowed(&pevents)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvcookie, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgressUnadvise<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcookie: usize) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ProgressUnadvise(::core::mem::transmute_copy(&vcookie)).into()
        }
        unsafe extern "system" fn RecordDisc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsimulate: u8, bejectafterburn: u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RecordDisc(::core::mem::transmute_copy(&bsimulate), ::core::mem::transmute_copy(&bejectafterburn)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Open: Open::<Identity, Impl, OFFSET>,
            EnumDiscMasterFormats: EnumDiscMasterFormats::<Identity, Impl, OFFSET>,
            GetActiveDiscMasterFormat: GetActiveDiscMasterFormat::<Identity, Impl, OFFSET>,
            SetActiveDiscMasterFormat: SetActiveDiscMasterFormat::<Identity, Impl, OFFSET>,
            EnumDiscRecorders: EnumDiscRecorders::<Identity, Impl, OFFSET>,
            GetActiveDiscRecorder: GetActiveDiscRecorder::<Identity, Impl, OFFSET>,
            SetActiveDiscRecorder: SetActiveDiscRecorder::<Identity, Impl, OFFSET>,
            ClearFormatContent: ClearFormatContent::<Identity, Impl, OFFSET>,
            ProgressAdvise: ProgressAdvise::<Identity, Impl, OFFSET>,
            ProgressUnadvise: ProgressUnadvise::<Identity, Impl, OFFSET>,
            RecordDisc: RecordDisc::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDiscMaster as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscMaster2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows_core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn get_Item(&self, index: i32) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn IsSupportedEnvironment(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IDiscMaster2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscMaster2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMaster2_Impl, const OFFSET: isize>() -> IDiscMaster2_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppunk, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupportedEnvironment<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSupportedEnvironment() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            IsSupportedEnvironment: IsSupportedEnvironment::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDiscMaster2 as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"implement\"`*"]
pub trait IDiscMasterProgressEvents_Impl: Sized {
    fn QueryCancel(&self) -> ::windows_core::Result<u8>;
    fn NotifyPnPActivity(&self) -> ::windows_core::Result<()>;
    fn NotifyAddProgress(&self, ncompletedsteps: i32, ntotalsteps: i32) -> ::windows_core::Result<()>;
    fn NotifyBlockProgress(&self, ncompleted: i32, ntotal: i32) -> ::windows_core::Result<()>;
    fn NotifyTrackProgress(&self, ncurrenttrack: i32, ntotaltracks: i32) -> ::windows_core::Result<()>;
    fn NotifyPreparingBurn(&self, nestimatedseconds: i32) -> ::windows_core::Result<()>;
    fn NotifyClosingDisc(&self, nestimatedseconds: i32) -> ::windows_core::Result<()>;
    fn NotifyBurnComplete(&self, status: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
    fn NotifyEraseComplete(&self, status: ::windows_core::HRESULT) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IDiscMasterProgressEvents {}
impl IDiscMasterProgressEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>() -> IDiscMasterProgressEvents_Vtbl {
        unsafe extern "system" fn QueryCancel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcancel: *mut u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.QueryCancel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbcancel, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyPnPActivity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyPnPActivity().into()
        }
        unsafe extern "system" fn NotifyAddProgress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncompletedsteps: i32, ntotalsteps: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyAddProgress(::core::mem::transmute_copy(&ncompletedsteps), ::core::mem::transmute_copy(&ntotalsteps)).into()
        }
        unsafe extern "system" fn NotifyBlockProgress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncompleted: i32, ntotal: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyBlockProgress(::core::mem::transmute_copy(&ncompleted), ::core::mem::transmute_copy(&ntotal)).into()
        }
        unsafe extern "system" fn NotifyTrackProgress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncurrenttrack: i32, ntotaltracks: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyTrackProgress(::core::mem::transmute_copy(&ncurrenttrack), ::core::mem::transmute_copy(&ntotaltracks)).into()
        }
        unsafe extern "system" fn NotifyPreparingBurn<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nestimatedseconds: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyPreparingBurn(::core::mem::transmute_copy(&nestimatedseconds)).into()
        }
        unsafe extern "system" fn NotifyClosingDisc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nestimatedseconds: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyClosingDisc(::core::mem::transmute_copy(&nestimatedseconds)).into()
        }
        unsafe extern "system" fn NotifyBurnComplete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyBurnComplete(::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn NotifyEraseComplete<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: ::windows_core::HRESULT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NotifyEraseComplete(::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            QueryCancel: QueryCancel::<Identity, Impl, OFFSET>,
            NotifyPnPActivity: NotifyPnPActivity::<Identity, Impl, OFFSET>,
            NotifyAddProgress: NotifyAddProgress::<Identity, Impl, OFFSET>,
            NotifyBlockProgress: NotifyBlockProgress::<Identity, Impl, OFFSET>,
            NotifyTrackProgress: NotifyTrackProgress::<Identity, Impl, OFFSET>,
            NotifyPreparingBurn: NotifyPreparingBurn::<Identity, Impl, OFFSET>,
            NotifyClosingDisc: NotifyClosingDisc::<Identity, Impl, OFFSET>,
            NotifyBurnComplete: NotifyBurnComplete::<Identity, Impl, OFFSET>,
            NotifyEraseComplete: NotifyEraseComplete::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDiscMasterProgressEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IDiscRecorder_Impl: Sized {
    fn Init(&self, pbyuniqueid: *const u8, nulidsize: u32, nuldrivenumber: u32) -> ::windows_core::Result<()>;
    fn GetRecorderGUID(&self, pbyuniqueid: *mut u8, ulbuffersize: u32, pulreturnsizerequired: *mut u32) -> ::windows_core::Result<()>;
    fn GetRecorderType(&self) -> ::windows_core::Result<RECORDER_TYPES>;
    fn GetDisplayNames(&self, pbstrvendorid: *mut ::windows_core::BSTR, pbstrproductid: *mut ::windows_core::BSTR, pbstrrevision: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetBasePnPID(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetPath(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetRecorderProperties(&self) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::IPropertyStorage>;
    fn SetRecorderProperties(&self, ppropstg: ::core::option::Option<&super::super::System::Com::StructuredStorage::IPropertyStorage>) -> ::windows_core::Result<()>;
    fn GetRecorderState(&self) -> ::windows_core::Result<DISC_RECORDER_STATE_FLAGS>;
    fn OpenExclusive(&self) -> ::windows_core::Result<()>;
    fn QueryMediaType(&self, fmediatype: *mut MEDIA_TYPES, fmediaflags: *mut MEDIA_FLAGS) -> ::windows_core::Result<()>;
    fn QueryMediaInfo(&self, pbsessions: *mut u8, pblasttrack: *mut u8, ulstartaddress: *mut u32, ulnextwritable: *mut u32, ulfreeblocks: *mut u32) -> ::windows_core::Result<()>;
    fn Eject(&self) -> ::windows_core::Result<()>;
    fn Erase(&self, bfullerase: u8) -> ::windows_core::Result<()>;
    fn Close(&self) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::RuntimeName for IDiscRecorder {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IDiscRecorder_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: isize>() -> IDiscRecorder_Vtbl {
        unsafe extern "system" fn Init<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbyuniqueid: *const u8, nulidsize: u32, nuldrivenumber: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Init(::core::mem::transmute_copy(&pbyuniqueid), ::core::mem::transmute_copy(&nulidsize), ::core::mem::transmute_copy(&nuldrivenumber)).into()
        }
        unsafe extern "system" fn GetRecorderGUID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbyuniqueid: *mut u8, ulbuffersize: u32, pulreturnsizerequired: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetRecorderGUID(::core::mem::transmute_copy(&pbyuniqueid), ::core::mem::transmute_copy(&ulbuffersize), ::core::mem::transmute_copy(&pulreturnsizerequired)).into()
        }
        unsafe extern "system" fn GetRecorderType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ftypecode: *mut RECORDER_TYPES) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRecorderType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ftypecode, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayNames<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrvendorid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrproductid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>, pbstrrevision: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDisplayNames(::core::mem::transmute_copy(&pbstrvendorid), ::core::mem::transmute_copy(&pbstrproductid), ::core::mem::transmute_copy(&pbstrrevision)).into()
        }
        unsafe extern "system" fn GetBasePnPID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbasepnpid: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBasePnPID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrbasepnpid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpath: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrpath, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecorderProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropstg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRecorderProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropstg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecorderProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropstg: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRecorderProperties(::windows_core::from_raw_borrowed(&ppropstg)).into()
        }
        unsafe extern "system" fn GetRecorderState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puldevstateflags: *mut DISC_RECORDER_STATE_FLAGS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRecorderState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(puldevstateflags, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenExclusive<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.OpenExclusive().into()
        }
        unsafe extern "system" fn QueryMediaType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmediatype: *mut MEDIA_TYPES, fmediaflags: *mut MEDIA_FLAGS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryMediaType(::core::mem::transmute_copy(&fmediatype), ::core::mem::transmute_copy(&fmediaflags)).into()
        }
        unsafe extern "system" fn QueryMediaInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsessions: *mut u8, pblasttrack: *mut u8, ulstartaddress: *mut u32, ulnextwritable: *mut u32, ulfreeblocks: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.QueryMediaInfo(::core::mem::transmute_copy(&pbsessions), ::core::mem::transmute_copy(&pblasttrack), ::core::mem::transmute_copy(&ulstartaddress), ::core::mem::transmute_copy(&ulnextwritable), ::core::mem::transmute_copy(&ulfreeblocks)).into()
        }
        unsafe extern "system" fn Eject<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Eject().into()
        }
        unsafe extern "system" fn Erase<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfullerase: u8) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Erase(::core::mem::transmute_copy(&bfullerase)).into()
        }
        unsafe extern "system" fn Close<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Close().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Init: Init::<Identity, Impl, OFFSET>,
            GetRecorderGUID: GetRecorderGUID::<Identity, Impl, OFFSET>,
            GetRecorderType: GetRecorderType::<Identity, Impl, OFFSET>,
            GetDisplayNames: GetDisplayNames::<Identity, Impl, OFFSET>,
            GetBasePnPID: GetBasePnPID::<Identity, Impl, OFFSET>,
            GetPath: GetPath::<Identity, Impl, OFFSET>,
            GetRecorderProperties: GetRecorderProperties::<Identity, Impl, OFFSET>,
            SetRecorderProperties: SetRecorderProperties::<Identity, Impl, OFFSET>,
            GetRecorderState: GetRecorderState::<Identity, Impl, OFFSET>,
            OpenExclusive: OpenExclusive::<Identity, Impl, OFFSET>,
            QueryMediaType: QueryMediaType::<Identity, Impl, OFFSET>,
            QueryMediaInfo: QueryMediaInfo::<Identity, Impl, OFFSET>,
            Eject: Eject::<Identity, Impl, OFFSET>,
            Erase: Erase::<Identity, Impl, OFFSET>,
            Close: Close::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDiscRecorder as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IDiscRecorder2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn EjectMedia(&self) -> ::windows_core::Result<()>;
    fn CloseTray(&self) -> ::windows_core::Result<()>;
    fn AcquireExclusiveAccess(&self, force: super::super::Foundation::VARIANT_BOOL, __midl__idiscrecorder20000: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ReleaseExclusiveAccess(&self) -> ::windows_core::Result<()>;
    fn DisableMcn(&self) -> ::windows_core::Result<()>;
    fn EnableMcn(&self) -> ::windows_core::Result<()>;
    fn InitializeDiscRecorder(&self, recorderuniqueid: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ActiveDiscRecorder(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn VendorId(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ProductId(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn ProductRevision(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn VolumeName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn VolumePathNames(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn DeviceCanLoadMedia(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn LegacyDeviceNumber(&self) -> ::windows_core::Result<i32>;
    fn SupportedFeaturePages(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CurrentFeaturePages(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedProfiles(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CurrentProfiles(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedModePages(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ExclusiveAccessOwner(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IDiscRecorder2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IDiscRecorder2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: isize>() -> IDiscRecorder2_Vtbl {
        unsafe extern "system" fn EjectMedia<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EjectMedia().into()
        }
        unsafe extern "system" fn CloseTray<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseTray().into()
        }
        unsafe extern "system" fn AcquireExclusiveAccess<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, force: super::super::Foundation::VARIANT_BOOL, __midl__idiscrecorder20000: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AcquireExclusiveAccess(::core::mem::transmute_copy(&force), ::core::mem::transmute(&__midl__idiscrecorder20000)).into()
        }
        unsafe extern "system" fn ReleaseExclusiveAccess<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReleaseExclusiveAccess().into()
        }
        unsafe extern "system" fn DisableMcn<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DisableMcn().into()
        }
        unsafe extern "system" fn EnableMcn<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableMcn().into()
        }
        unsafe extern "system" fn InitializeDiscRecorder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recorderuniqueid: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.InitializeDiscRecorder(::core::mem::transmute(&recorderuniqueid)).into()
        }
        unsafe extern "system" fn ActiveDiscRecorder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ActiveDiscRecorder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VendorId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VendorId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProductId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductRevision<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProductRevision() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VolumeName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumePathNames<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VolumePathNames() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceCanLoadMedia<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DeviceCanLoadMedia() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LegacyDeviceNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, legacydevicenumber: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LegacyDeviceNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(legacydevicenumber, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedFeaturePages<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportedFeaturePages() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFeaturePages<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentFeaturePages() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedProfiles<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportedProfiles() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentProfiles<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CurrentProfiles() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedModePages<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SupportedModePages() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExclusiveAccessOwner<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExclusiveAccessOwner() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            EjectMedia: EjectMedia::<Identity, Impl, OFFSET>,
            CloseTray: CloseTray::<Identity, Impl, OFFSET>,
            AcquireExclusiveAccess: AcquireExclusiveAccess::<Identity, Impl, OFFSET>,
            ReleaseExclusiveAccess: ReleaseExclusiveAccess::<Identity, Impl, OFFSET>,
            DisableMcn: DisableMcn::<Identity, Impl, OFFSET>,
            EnableMcn: EnableMcn::<Identity, Impl, OFFSET>,
            InitializeDiscRecorder: InitializeDiscRecorder::<Identity, Impl, OFFSET>,
            ActiveDiscRecorder: ActiveDiscRecorder::<Identity, Impl, OFFSET>,
            VendorId: VendorId::<Identity, Impl, OFFSET>,
            ProductId: ProductId::<Identity, Impl, OFFSET>,
            ProductRevision: ProductRevision::<Identity, Impl, OFFSET>,
            VolumeName: VolumeName::<Identity, Impl, OFFSET>,
            VolumePathNames: VolumePathNames::<Identity, Impl, OFFSET>,
            DeviceCanLoadMedia: DeviceCanLoadMedia::<Identity, Impl, OFFSET>,
            LegacyDeviceNumber: LegacyDeviceNumber::<Identity, Impl, OFFSET>,
            SupportedFeaturePages: SupportedFeaturePages::<Identity, Impl, OFFSET>,
            CurrentFeaturePages: CurrentFeaturePages::<Identity, Impl, OFFSET>,
            SupportedProfiles: SupportedProfiles::<Identity, Impl, OFFSET>,
            CurrentProfiles: CurrentProfiles::<Identity, Impl, OFFSET>,
            SupportedModePages: SupportedModePages::<Identity, Impl, OFFSET>,
            ExclusiveAccessOwner: ExclusiveAccessOwner::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDiscRecorder2 as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait IDiscRecorder2Ex_Impl: Sized {
    fn SendCommandNoData(&self, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32) -> ::windows_core::Result<()>;
    fn SendCommandSendDataToDevice(&self, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *const u8, buffersize: u32) -> ::windows_core::Result<()>;
    fn SendCommandGetDataFromDevice(&self, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *mut u8, buffersize: u32, bufferfetched: *mut u32) -> ::windows_core::Result<()>;
    fn ReadDvdStructure(&self, format: u32, address: u32, layer: u32, agid: u32, data: *mut *mut u8, count: *mut u32) -> ::windows_core::Result<()>;
    fn SendDvdStructure(&self, format: u32, data: *const u8, count: u32) -> ::windows_core::Result<()>;
    fn GetAdapterDescriptor(&self, data: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::Result<()>;
    fn GetDeviceDescriptor(&self, data: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::Result<()>;
    fn GetDiscInformation(&self, discinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::Result<()>;
    fn GetTrackInformation(&self, address: u32, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::Result<()>;
    fn GetFeaturePage(&self, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::Result<()>;
    fn GetModePage(&self, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::Result<()>;
    fn SetModePage(&self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: *const u8, bytesize: u32) -> ::windows_core::Result<()>;
    fn GetSupportedFeaturePages(&self, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut u32) -> ::windows_core::Result<()>;
    fn GetSupportedProfiles(&self, currentonly: super::super::Foundation::BOOLEAN, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut u32) -> ::windows_core::Result<()>;
    fn GetSupportedModePages(&self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut u32) -> ::windows_core::Result<()>;
    fn GetByteAlignmentMask(&self) -> ::windows_core::Result<u32>;
    fn GetMaximumNonPageAlignedTransferSize(&self) -> ::windows_core::Result<u32>;
    fn GetMaximumPageAlignedTransferSize(&self) -> ::windows_core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for IDiscRecorder2Ex {}
#[cfg(feature = "Win32_Foundation")]
impl IDiscRecorder2Ex_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>() -> IDiscRecorder2Ex_Vtbl {
        unsafe extern "system" fn SendCommandNoData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendCommandNoData(::core::mem::transmute_copy(&cdb), ::core::mem::transmute_copy(&cdbsize), ::core::mem::transmute_copy(&sensebuffer), ::core::mem::transmute_copy(&timeout)).into()
        }
        unsafe extern "system" fn SendCommandSendDataToDevice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *const u8, buffersize: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendCommandSendDataToDevice(::core::mem::transmute_copy(&cdb), ::core::mem::transmute_copy(&cdbsize), ::core::mem::transmute_copy(&sensebuffer), ::core::mem::transmute_copy(&timeout), ::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&buffersize)).into()
        }
        unsafe extern "system" fn SendCommandGetDataFromDevice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *mut u8, buffersize: u32, bufferfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendCommandGetDataFromDevice(::core::mem::transmute_copy(&cdb), ::core::mem::transmute_copy(&cdbsize), ::core::mem::transmute_copy(&sensebuffer), ::core::mem::transmute_copy(&timeout), ::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&buffersize), ::core::mem::transmute_copy(&bufferfetched)).into()
        }
        unsafe extern "system" fn ReadDvdStructure<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: u32, address: u32, layer: u32, agid: u32, data: *mut *mut u8, count: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ReadDvdStructure(::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&address), ::core::mem::transmute_copy(&layer), ::core::mem::transmute_copy(&agid), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SendDvdStructure<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: u32, data: *const u8, count: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SendDvdStructure(::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetAdapterDescriptor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAdapterDescriptor(::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetDeviceDescriptor<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDeviceDescriptor(::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetDiscInformation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDiscInformation(::core::mem::transmute_copy(&discinformation), ::core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetTrackInformation<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: u32, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTrackInformation(::core::mem::transmute_copy(&address), ::core::mem::transmute_copy(&addresstype), ::core::mem::transmute_copy(&trackinformation), ::core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetFeaturePage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetFeaturePage(::core::mem::transmute_copy(&requestedfeature), ::core::mem::transmute_copy(&currentfeatureonly), ::core::mem::transmute_copy(&featuredata), ::core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetModePage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut u8, bytesize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetModePage(::core::mem::transmute_copy(&requestedmodepage), ::core::mem::transmute_copy(&requesttype), ::core::mem::transmute_copy(&modepagedata), ::core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn SetModePage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: *const u8, bytesize: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetModePage(::core::mem::transmute_copy(&requesttype), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetSupportedFeaturePages<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSupportedFeaturePages(::core::mem::transmute_copy(&currentfeatureonly), ::core::mem::transmute_copy(&featuredata), ::core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetSupportedProfiles<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentonly: super::super::Foundation::BOOLEAN, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSupportedProfiles(::core::mem::transmute_copy(&currentonly), ::core::mem::transmute_copy(&profiletypes), ::core::mem::transmute_copy(&validprofiles)).into()
        }
        unsafe extern "system" fn GetSupportedModePages<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSupportedModePages(::core::mem::transmute_copy(&requesttype), ::core::mem::transmute_copy(&modepagetypes), ::core::mem::transmute_copy(&validpages)).into()
        }
        unsafe extern "system" fn GetByteAlignmentMask<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetByteAlignmentMask() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaximumNonPageAlignedTransferSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaximumNonPageAlignedTransferSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaximumPageAlignedTransferSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetMaximumPageAlignedTransferSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SendCommandNoData: SendCommandNoData::<Identity, Impl, OFFSET>,
            SendCommandSendDataToDevice: SendCommandSendDataToDevice::<Identity, Impl, OFFSET>,
            SendCommandGetDataFromDevice: SendCommandGetDataFromDevice::<Identity, Impl, OFFSET>,
            ReadDvdStructure: ReadDvdStructure::<Identity, Impl, OFFSET>,
            SendDvdStructure: SendDvdStructure::<Identity, Impl, OFFSET>,
            GetAdapterDescriptor: GetAdapterDescriptor::<Identity, Impl, OFFSET>,
            GetDeviceDescriptor: GetDeviceDescriptor::<Identity, Impl, OFFSET>,
            GetDiscInformation: GetDiscInformation::<Identity, Impl, OFFSET>,
            GetTrackInformation: GetTrackInformation::<Identity, Impl, OFFSET>,
            GetFeaturePage: GetFeaturePage::<Identity, Impl, OFFSET>,
            GetModePage: GetModePage::<Identity, Impl, OFFSET>,
            SetModePage: SetModePage::<Identity, Impl, OFFSET>,
            GetSupportedFeaturePages: GetSupportedFeaturePages::<Identity, Impl, OFFSET>,
            GetSupportedProfiles: GetSupportedProfiles::<Identity, Impl, OFFSET>,
            GetSupportedModePages: GetSupportedModePages::<Identity, Impl, OFFSET>,
            GetByteAlignmentMask: GetByteAlignmentMask::<Identity, Impl, OFFSET>,
            GetMaximumNonPageAlignedTransferSize: GetMaximumNonPageAlignedTransferSize::<Identity, Impl, OFFSET>,
            GetMaximumPageAlignedTransferSize: GetMaximumPageAlignedTransferSize::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IDiscRecorder2Ex as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"implement\"`*"]
pub trait IEnumDiscMasterFormats_Impl: Sized {
    fn Next(&self, cformats: u32, lpiidformatid: *mut ::windows_core::GUID, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self, cformats: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IEnumDiscMasterFormats>;
}
impl ::windows_core::RuntimeName for IEnumDiscMasterFormats {}
impl IEnumDiscMasterFormats_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDiscMasterFormats_Impl, const OFFSET: isize>() -> IEnumDiscMasterFormats_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDiscMasterFormats_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cformats: u32, lpiidformatid: *mut ::windows_core::GUID, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&cformats), ::core::mem::transmute_copy(&lpiidformatid), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDiscMasterFormats_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cformats: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&cformats)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDiscMasterFormats_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDiscMasterFormats_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumDiscMasterFormats as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"implement\"`*"]
pub trait IEnumDiscRecorders_Impl: Sized {
    fn Next(&self, crecorders: u32, pprecorder: *mut ::core::option::Option<IDiscRecorder>, pcfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self, crecorders: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IEnumDiscRecorders>;
}
impl ::windows_core::RuntimeName for IEnumDiscRecorders {}
impl IEnumDiscRecorders_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDiscRecorders_Impl, const OFFSET: isize>() -> IEnumDiscRecorders_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDiscRecorders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crecorders: u32, pprecorder: *mut *mut ::core::ffi::c_void, pcfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&crecorders), ::core::mem::transmute_copy(&pprecorder), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDiscRecorders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crecorders: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&crecorders)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDiscRecorders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumDiscRecorders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumDiscRecorders as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumFsiItems_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IFsiItem>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IEnumFsiItems>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IEnumFsiItems {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumFsiItems_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumFsiItems_Impl, const OFFSET: isize>() -> IEnumFsiItems_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumFsiItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumFsiItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumFsiItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumFsiItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumFsiItems as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumProgressItems_Impl: Sized {
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<IProgressItem>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IEnumProgressItems>;
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows_core::RuntimeName for IEnumProgressItems {}
#[cfg(feature = "Win32_System_Com")]
impl IEnumProgressItems_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumProgressItems_Impl, const OFFSET: isize>() -> IEnumProgressItems_Vtbl {
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumProgressItems as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFileSystemImage_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Root(&self) -> ::windows_core::Result<IFsiDirectoryItem>;
    fn SessionStartBlock(&self) -> ::windows_core::Result<i32>;
    fn SetSessionStartBlock(&self, newval: i32) -> ::windows_core::Result<()>;
    fn FreeMediaBlocks(&self) -> ::windows_core::Result<i32>;
    fn SetFreeMediaBlocks(&self, newval: i32) -> ::windows_core::Result<()>;
    fn SetMaxMediaBlocksFromDevice(&self, discrecorder: ::core::option::Option<&IDiscRecorder2>) -> ::windows_core::Result<()>;
    fn UsedBlocks(&self) -> ::windows_core::Result<i32>;
    fn VolumeName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetVolumeName(&self, newval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ImportedVolumeName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn BootImageOptions(&self) -> ::windows_core::Result<IBootOptions>;
    fn SetBootImageOptions(&self, newval: ::core::option::Option<&IBootOptions>) -> ::windows_core::Result<()>;
    fn FileCount(&self) -> ::windows_core::Result<i32>;
    fn DirectoryCount(&self) -> ::windows_core::Result<i32>;
    fn WorkingDirectory(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetWorkingDirectory(&self, newval: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn ChangePoint(&self) -> ::windows_core::Result<i32>;
    fn StrictFileSystemCompliance(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetStrictFileSystemCompliance(&self, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn UseRestrictedCharacterSet(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetUseRestrictedCharacterSet(&self, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn FileSystemsToCreate(&self) -> ::windows_core::Result<FsiFileSystems>;
    fn SetFileSystemsToCreate(&self, newval: FsiFileSystems) -> ::windows_core::Result<()>;
    fn FileSystemsSupported(&self) -> ::windows_core::Result<FsiFileSystems>;
    fn SetUDFRevision(&self, newval: i32) -> ::windows_core::Result<()>;
    fn UDFRevision(&self) -> ::windows_core::Result<i32>;
    fn UDFRevisionsSupported(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ChooseImageDefaults(&self, discrecorder: ::core::option::Option<&IDiscRecorder2>) -> ::windows_core::Result<()>;
    fn ChooseImageDefaultsForMediaType(&self, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_core::Result<()>;
    fn SetISO9660InterchangeLevel(&self, newval: i32) -> ::windows_core::Result<()>;
    fn ISO9660InterchangeLevel(&self) -> ::windows_core::Result<i32>;
    fn ISO9660InterchangeLevelsSupported(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CreateResultImage(&self) -> ::windows_core::Result<IFileSystemImageResult>;
    fn Exists(&self, fullpath: &::windows_core::BSTR) -> ::windows_core::Result<FsiItemType>;
    fn CalculateDiscIdentifier(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn IdentifyFileSystemsOnDisc(&self, discrecorder: ::core::option::Option<&IDiscRecorder2>) -> ::windows_core::Result<FsiFileSystems>;
    fn GetDefaultFileSystemForImport(&self, filesystems: FsiFileSystems) -> ::windows_core::Result<FsiFileSystems>;
    fn ImportFileSystem(&self) -> ::windows_core::Result<FsiFileSystems>;
    fn ImportSpecificFileSystem(&self, filesystemtouse: FsiFileSystems) -> ::windows_core::Result<()>;
    fn RollbackToChangePoint(&self, changepoint: i32) -> ::windows_core::Result<()>;
    fn LockInChangePoint(&self) -> ::windows_core::Result<()>;
    fn CreateDirectoryItem(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<IFsiDirectoryItem>;
    fn CreateFileItem(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<IFsiFileItem>;
    fn VolumeNameUDF(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn VolumeNameJoliet(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn VolumeNameISO9660(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn StageFiles(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetStageFiles(&self, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn MultisessionInterfaces(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetMultisessionInterfaces(&self, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IFileSystemImage {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFileSystemImage_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>() -> IFileSystemImage_Vtbl {
        unsafe extern "system" fn Root<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Root() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionStartBlock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SessionStartBlock() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSessionStartBlock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSessionStartBlock(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn FreeMediaBlocks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FreeMediaBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFreeMediaBlocks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFreeMediaBlocks(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn SetMaxMediaBlocksFromDevice<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discrecorder: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxMediaBlocksFromDevice(::windows_core::from_raw_borrowed(&discrecorder)).into()
        }
        unsafe extern "system" fn UsedBlocks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UsedBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VolumeName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolumeName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVolumeName(::core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn ImportedVolumeName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ImportedVolumeName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BootImageOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BootImageOptions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBootImageOptions<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBootImageOptions(::windows_core::from_raw_borrowed(&newval)).into()
        }
        unsafe extern "system" fn FileCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FileCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DirectoryCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DirectoryCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WorkingDirectory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WorkingDirectory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWorkingDirectory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetWorkingDirectory(::core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn ChangePoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ChangePoint() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrictFileSystemCompliance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StrictFileSystemCompliance() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrictFileSystemCompliance<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStrictFileSystemCompliance(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn UseRestrictedCharacterSet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UseRestrictedCharacterSet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseRestrictedCharacterSet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUseRestrictedCharacterSet(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn FileSystemsToCreate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut FsiFileSystems) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FileSystemsToCreate() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileSystemsToCreate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: FsiFileSystems) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetFileSystemsToCreate(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn FileSystemsSupported<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut FsiFileSystems) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FileSystemsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUDFRevision<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUDFRevision(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn UDFRevision<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UDFRevision() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UDFRevisionsSupported<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UDFRevisionsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChooseImageDefaults<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discrecorder: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ChooseImageDefaults(::windows_core::from_raw_borrowed(&discrecorder)).into()
        }
        unsafe extern "system" fn ChooseImageDefaultsForMediaType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ChooseImageDefaultsForMediaType(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetISO9660InterchangeLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetISO9660InterchangeLevel(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ISO9660InterchangeLevel<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ISO9660InterchangeLevel() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ISO9660InterchangeLevelsSupported<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ISO9660InterchangeLevelsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResultImage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateResultImage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resultstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Exists<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fullpath: ::std::mem::MaybeUninit<::windows_core::BSTR>, itemtype: *mut FsiItemType) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Exists(::core::mem::transmute(&fullpath)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(itemtype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CalculateDiscIdentifier<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discidentifier: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CalculateDiscIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(discidentifier, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IdentifyFileSystemsOnDisc<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discrecorder: *mut ::core::ffi::c_void, filesystems: *mut FsiFileSystems) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IdentifyFileSystemsOnDisc(::windows_core::from_raw_borrowed(&discrecorder)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(filesystems, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultFileSystemForImport<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesystems: FsiFileSystems, importdefault: *mut FsiFileSystems) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDefaultFileSystemForImport(::core::mem::transmute_copy(&filesystems)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(importdefault, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportFileSystem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, importedfilesystem: *mut FsiFileSystems) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ImportFileSystem() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(importedfilesystem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportSpecificFileSystem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesystemtouse: FsiFileSystems) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ImportSpecificFileSystem(::core::mem::transmute_copy(&filesystemtouse)).into()
        }
        unsafe extern "system" fn RollbackToChangePoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changepoint: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RollbackToChangePoint(::core::mem::transmute_copy(&changepoint)).into()
        }
        unsafe extern "system" fn LockInChangePoint<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.LockInChangePoint().into()
        }
        unsafe extern "system" fn CreateDirectoryItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, newitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateDirectoryItem(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileItem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, newitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateFileItem(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeNameUDF<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VolumeNameUDF() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeNameJoliet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VolumeNameJoliet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeNameISO9660<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.VolumeNameISO9660() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StageFiles<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StageFiles() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStageFiles<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStageFiles(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn MultisessionInterfaces<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MultisessionInterfaces() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultisessionInterfaces<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMultisessionInterfaces(::core::mem::transmute_copy(&newval)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Root: Root::<Identity, Impl, OFFSET>,
            SessionStartBlock: SessionStartBlock::<Identity, Impl, OFFSET>,
            SetSessionStartBlock: SetSessionStartBlock::<Identity, Impl, OFFSET>,
            FreeMediaBlocks: FreeMediaBlocks::<Identity, Impl, OFFSET>,
            SetFreeMediaBlocks: SetFreeMediaBlocks::<Identity, Impl, OFFSET>,
            SetMaxMediaBlocksFromDevice: SetMaxMediaBlocksFromDevice::<Identity, Impl, OFFSET>,
            UsedBlocks: UsedBlocks::<Identity, Impl, OFFSET>,
            VolumeName: VolumeName::<Identity, Impl, OFFSET>,
            SetVolumeName: SetVolumeName::<Identity, Impl, OFFSET>,
            ImportedVolumeName: ImportedVolumeName::<Identity, Impl, OFFSET>,
            BootImageOptions: BootImageOptions::<Identity, Impl, OFFSET>,
            SetBootImageOptions: SetBootImageOptions::<Identity, Impl, OFFSET>,
            FileCount: FileCount::<Identity, Impl, OFFSET>,
            DirectoryCount: DirectoryCount::<Identity, Impl, OFFSET>,
            WorkingDirectory: WorkingDirectory::<Identity, Impl, OFFSET>,
            SetWorkingDirectory: SetWorkingDirectory::<Identity, Impl, OFFSET>,
            ChangePoint: ChangePoint::<Identity, Impl, OFFSET>,
            StrictFileSystemCompliance: StrictFileSystemCompliance::<Identity, Impl, OFFSET>,
            SetStrictFileSystemCompliance: SetStrictFileSystemCompliance::<Identity, Impl, OFFSET>,
            UseRestrictedCharacterSet: UseRestrictedCharacterSet::<Identity, Impl, OFFSET>,
            SetUseRestrictedCharacterSet: SetUseRestrictedCharacterSet::<Identity, Impl, OFFSET>,
            FileSystemsToCreate: FileSystemsToCreate::<Identity, Impl, OFFSET>,
            SetFileSystemsToCreate: SetFileSystemsToCreate::<Identity, Impl, OFFSET>,
            FileSystemsSupported: FileSystemsSupported::<Identity, Impl, OFFSET>,
            SetUDFRevision: SetUDFRevision::<Identity, Impl, OFFSET>,
            UDFRevision: UDFRevision::<Identity, Impl, OFFSET>,
            UDFRevisionsSupported: UDFRevisionsSupported::<Identity, Impl, OFFSET>,
            ChooseImageDefaults: ChooseImageDefaults::<Identity, Impl, OFFSET>,
            ChooseImageDefaultsForMediaType: ChooseImageDefaultsForMediaType::<Identity, Impl, OFFSET>,
            SetISO9660InterchangeLevel: SetISO9660InterchangeLevel::<Identity, Impl, OFFSET>,
            ISO9660InterchangeLevel: ISO9660InterchangeLevel::<Identity, Impl, OFFSET>,
            ISO9660InterchangeLevelsSupported: ISO9660InterchangeLevelsSupported::<Identity, Impl, OFFSET>,
            CreateResultImage: CreateResultImage::<Identity, Impl, OFFSET>,
            Exists: Exists::<Identity, Impl, OFFSET>,
            CalculateDiscIdentifier: CalculateDiscIdentifier::<Identity, Impl, OFFSET>,
            IdentifyFileSystemsOnDisc: IdentifyFileSystemsOnDisc::<Identity, Impl, OFFSET>,
            GetDefaultFileSystemForImport: GetDefaultFileSystemForImport::<Identity, Impl, OFFSET>,
            ImportFileSystem: ImportFileSystem::<Identity, Impl, OFFSET>,
            ImportSpecificFileSystem: ImportSpecificFileSystem::<Identity, Impl, OFFSET>,
            RollbackToChangePoint: RollbackToChangePoint::<Identity, Impl, OFFSET>,
            LockInChangePoint: LockInChangePoint::<Identity, Impl, OFFSET>,
            CreateDirectoryItem: CreateDirectoryItem::<Identity, Impl, OFFSET>,
            CreateFileItem: CreateFileItem::<Identity, Impl, OFFSET>,
            VolumeNameUDF: VolumeNameUDF::<Identity, Impl, OFFSET>,
            VolumeNameJoliet: VolumeNameJoliet::<Identity, Impl, OFFSET>,
            VolumeNameISO9660: VolumeNameISO9660::<Identity, Impl, OFFSET>,
            StageFiles: StageFiles::<Identity, Impl, OFFSET>,
            SetStageFiles: SetStageFiles::<Identity, Impl, OFFSET>,
            MultisessionInterfaces: MultisessionInterfaces::<Identity, Impl, OFFSET>,
            SetMultisessionInterfaces: SetMultisessionInterfaces::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFileSystemImage as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFileSystemImage2_Impl: Sized + IFileSystemImage_Impl {
    fn BootImageOptionsArray(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetBootImageOptionsArray(&self, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IFileSystemImage2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFileSystemImage2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage2_Impl, const OFFSET: isize>() -> IFileSystemImage2_Vtbl {
        unsafe extern "system" fn BootImageOptionsArray<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BootImageOptionsArray() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBootImageOptionsArray<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBootImageOptionsArray(::core::mem::transmute_copy(&newval)).into()
        }
        Self {
            base__: IFileSystemImage_Vtbl::new::<Identity, Impl, OFFSET>(),
            BootImageOptionsArray: BootImageOptionsArray::<Identity, Impl, OFFSET>,
            SetBootImageOptionsArray: SetBootImageOptionsArray::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFileSystemImage2 as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IFileSystemImage as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFileSystemImage3_Impl: Sized + IFileSystemImage2_Impl {
    fn CreateRedundantUdfMetadataFiles(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetCreateRedundantUdfMetadataFiles(&self, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn ProbeSpecificFileSystem(&self, filesystemtoprobe: FsiFileSystems) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IFileSystemImage3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFileSystemImage3_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage3_Impl, const OFFSET: isize>() -> IFileSystemImage3_Vtbl {
        unsafe extern "system" fn CreateRedundantUdfMetadataFiles<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRedundantUdfMetadataFiles() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreateRedundantUdfMetadataFiles<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCreateRedundantUdfMetadataFiles(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ProbeSpecificFileSystem<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesystemtoprobe: FsiFileSystems, isappendable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProbeSpecificFileSystem(::core::mem::transmute_copy(&filesystemtoprobe)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(isappendable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IFileSystemImage2_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateRedundantUdfMetadataFiles: CreateRedundantUdfMetadataFiles::<Identity, Impl, OFFSET>,
            SetCreateRedundantUdfMetadataFiles: SetCreateRedundantUdfMetadataFiles::<Identity, Impl, OFFSET>,
            ProbeSpecificFileSystem: ProbeSpecificFileSystem::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFileSystemImage3 as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IFileSystemImage as ::windows_core::ComInterface>::IID || iid == &<IFileSystemImage2 as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFileSystemImageResult_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ImageStream(&self) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn ProgressItems(&self) -> ::windows_core::Result<IProgressItems>;
    fn TotalBlocks(&self) -> ::windows_core::Result<i32>;
    fn BlockSize(&self) -> ::windows_core::Result<i32>;
    fn DiscId(&self) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IFileSystemImageResult {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFileSystemImageResult_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImageResult_Impl, const OFFSET: isize>() -> IFileSystemImageResult_Vtbl {
        unsafe extern "system" fn ImageStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ImageStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgressItems<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProgressItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalBlocks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TotalBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BlockSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BlockSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DiscId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            ImageStream: ImageStream::<Identity, Impl, OFFSET>,
            ProgressItems: ProgressItems::<Identity, Impl, OFFSET>,
            TotalBlocks: TotalBlocks::<Identity, Impl, OFFSET>,
            BlockSize: BlockSize::<Identity, Impl, OFFSET>,
            DiscId: DiscId::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFileSystemImageResult as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFileSystemImageResult2_Impl: Sized + IFileSystemImageResult_Impl {
    fn ModifiedBlocks(&self) -> ::windows_core::Result<IBlockRangeList>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IFileSystemImageResult2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFileSystemImageResult2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImageResult2_Impl, const OFFSET: isize>() -> IFileSystemImageResult2_Vtbl {
        unsafe extern "system" fn ModifiedBlocks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFileSystemImageResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ModifiedBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IFileSystemImageResult_Vtbl::new::<Identity, Impl, OFFSET>(), ModifiedBlocks: ModifiedBlocks::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFileSystemImageResult2 as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IFileSystemImageResult as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsiDirectoryItem_Impl: Sized + IFsiItem_Impl {
    fn _NewEnum(&self) -> ::windows_core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn get_Item(&self, path: &::windows_core::BSTR) -> ::windows_core::Result<IFsiItem>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn EnumFsiItems(&self) -> ::windows_core::Result<IEnumFsiItems>;
    fn AddDirectory(&self, path: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn AddFile(&self, path: &::windows_core::BSTR, filedata: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn AddTree(&self, sourcedirectory: &::windows_core::BSTR, includebasedirectory: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn Add(&self, item: ::core::option::Option<&IFsiItem>) -> ::windows_core::Result<()>;
    fn Remove(&self, path: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn RemoveTree(&self, path: &::windows_core::BSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IFsiDirectoryItem {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsiDirectoryItem_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>() -> IFsiDirectoryItem_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, item: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFsiItems<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumFsiItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDirectory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddDirectory(::core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn AddFile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>, filedata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddFile(::core::mem::transmute(&path), ::windows_core::from_raw_borrowed(&filedata)).into()
        }
        unsafe extern "system" fn AddTree<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcedirectory: ::std::mem::MaybeUninit<::windows_core::BSTR>, includebasedirectory: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddTree(::core::mem::transmute(&sourcedirectory), ::core::mem::transmute_copy(&includebasedirectory)).into()
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::windows_core::from_raw_borrowed(&item)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute(&path)).into()
        }
        unsafe extern "system" fn RemoveTree<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveTree(::core::mem::transmute(&path)).into()
        }
        Self {
            base__: IFsiItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            EnumFsiItems: EnumFsiItems::<Identity, Impl, OFFSET>,
            AddDirectory: AddDirectory::<Identity, Impl, OFFSET>,
            AddFile: AddFile::<Identity, Impl, OFFSET>,
            AddTree: AddTree::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            RemoveTree: RemoveTree::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFsiDirectoryItem as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IFsiItem as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsiDirectoryItem2_Impl: Sized + IFsiDirectoryItem_Impl {
    fn AddTreeWithNamedStreams(&self, sourcedirectory: &::windows_core::BSTR, includebasedirectory: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IFsiDirectoryItem2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsiDirectoryItem2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiDirectoryItem2_Impl, const OFFSET: isize>() -> IFsiDirectoryItem2_Vtbl {
        unsafe extern "system" fn AddTreeWithNamedStreams<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiDirectoryItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcedirectory: ::std::mem::MaybeUninit<::windows_core::BSTR>, includebasedirectory: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddTreeWithNamedStreams(::core::mem::transmute(&sourcedirectory), ::core::mem::transmute_copy(&includebasedirectory)).into()
        }
        Self { base__: IFsiDirectoryItem_Vtbl::new::<Identity, Impl, OFFSET>(), AddTreeWithNamedStreams: AddTreeWithNamedStreams::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFsiDirectoryItem2 as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IFsiItem as ::windows_core::ComInterface>::IID || iid == &<IFsiDirectoryItem as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsiFileItem_Impl: Sized + IFsiItem_Impl {
    fn DataSize(&self) -> ::windows_core::Result<i64>;
    fn DataSize32BitLow(&self) -> ::windows_core::Result<i32>;
    fn DataSize32BitHigh(&self) -> ::windows_core::Result<i32>;
    fn Data(&self) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn SetData(&self, newval: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IFsiFileItem {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsiFileItem_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiFileItem_Impl, const OFFSET: isize>() -> IFsiFileItem_Vtbl {
        unsafe extern "system" fn DataSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DataSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataSize32BitLow<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DataSize32BitLow() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataSize32BitHigh<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DataSize32BitHigh() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Data() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetData(::windows_core::from_raw_borrowed(&newval)).into()
        }
        Self {
            base__: IFsiItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            DataSize: DataSize::<Identity, Impl, OFFSET>,
            DataSize32BitLow: DataSize32BitLow::<Identity, Impl, OFFSET>,
            DataSize32BitHigh: DataSize32BitHigh::<Identity, Impl, OFFSET>,
            Data: Data::<Identity, Impl, OFFSET>,
            SetData: SetData::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFsiFileItem as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IFsiItem as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsiFileItem2_Impl: Sized + IFsiFileItem_Impl {
    fn FsiNamedStreams(&self) -> ::windows_core::Result<IFsiNamedStreams>;
    fn IsNamedStream(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn AddStream(&self, name: &::windows_core::BSTR, streamdata: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn RemoveStream(&self, name: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn IsRealTime(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsRealTime(&self, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IFsiFileItem2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsiFileItem2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiFileItem2_Impl, const OFFSET: isize>() -> IFsiFileItem2_Vtbl {
        unsafe extern "system" fn FsiNamedStreams<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streams: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FsiNamedStreams() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(streams, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNamedStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsNamedStream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>, streamdata: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddStream(::core::mem::transmute(&name), ::windows_core::from_raw_borrowed(&streamdata)).into()
        }
        unsafe extern "system" fn RemoveStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveStream(::core::mem::transmute(&name)).into()
        }
        unsafe extern "system" fn IsRealTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsRealTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRealTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsRealTime(::core::mem::transmute_copy(&newval)).into()
        }
        Self {
            base__: IFsiFileItem_Vtbl::new::<Identity, Impl, OFFSET>(),
            FsiNamedStreams: FsiNamedStreams::<Identity, Impl, OFFSET>,
            IsNamedStream: IsNamedStream::<Identity, Impl, OFFSET>,
            AddStream: AddStream::<Identity, Impl, OFFSET>,
            RemoveStream: RemoveStream::<Identity, Impl, OFFSET>,
            IsRealTime: IsRealTime::<Identity, Impl, OFFSET>,
            SetIsRealTime: SetIsRealTime::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFsiFileItem2 as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IFsiItem as ::windows_core::ComInterface>::IID || iid == &<IFsiFileItem as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsiItem_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn FullPath(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn CreationTime(&self) -> ::windows_core::Result<f64>;
    fn SetCreationTime(&self, newval: f64) -> ::windows_core::Result<()>;
    fn LastAccessedTime(&self) -> ::windows_core::Result<f64>;
    fn SetLastAccessedTime(&self, newval: f64) -> ::windows_core::Result<()>;
    fn LastModifiedTime(&self) -> ::windows_core::Result<f64>;
    fn SetLastModifiedTime(&self, newval: f64) -> ::windows_core::Result<()>;
    fn IsHidden(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetIsHidden(&self, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn FileSystemName(&self, filesystem: FsiFileSystems) -> ::windows_core::Result<::windows_core::BSTR>;
    fn FileSystemPath(&self, filesystem: FsiFileSystems) -> ::windows_core::Result<::windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IFsiItem {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsiItem_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiItem_Impl, const OFFSET: isize>() -> IFsiItem_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FullPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FullPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreationTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreationTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreationTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCreationTime(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn LastAccessedTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastAccessedTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastAccessedTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLastAccessedTime(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn LastModifiedTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastModifiedTime() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastModifiedTime<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: f64) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLastModifiedTime(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn IsHidden<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsHidden() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHidden<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsHidden(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn FileSystemName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesystem: FsiFileSystems, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FileSystemName(::core::mem::transmute_copy(&filesystem)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileSystemPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesystem: FsiFileSystems, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FileSystemPath(::core::mem::transmute_copy(&filesystem)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            FullPath: FullPath::<Identity, Impl, OFFSET>,
            CreationTime: CreationTime::<Identity, Impl, OFFSET>,
            SetCreationTime: SetCreationTime::<Identity, Impl, OFFSET>,
            LastAccessedTime: LastAccessedTime::<Identity, Impl, OFFSET>,
            SetLastAccessedTime: SetLastAccessedTime::<Identity, Impl, OFFSET>,
            LastModifiedTime: LastModifiedTime::<Identity, Impl, OFFSET>,
            SetLastModifiedTime: SetLastModifiedTime::<Identity, Impl, OFFSET>,
            IsHidden: IsHidden::<Identity, Impl, OFFSET>,
            SetIsHidden: SetIsHidden::<Identity, Impl, OFFSET>,
            FileSystemName: FileSystemName::<Identity, Impl, OFFSET>,
            FileSystemPath: FileSystemPath::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFsiItem as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IFsiNamedStreams_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows_core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn get_Item(&self, index: i32) -> ::windows_core::Result<IFsiFileItem2>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn EnumNamedStreams(&self) -> ::windows_core::Result<IEnumFsiItems>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IFsiNamedStreams {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IFsiNamedStreams_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiNamedStreams_Impl, const OFFSET: isize>() -> IFsiNamedStreams_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, item: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumNamedStreams<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumNamedStreams() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            EnumNamedStreams: EnumNamedStreams::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IFsiNamedStreams as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IIsoImageManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Path(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn Stream(&self) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn SetPath(&self, val: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn SetStream(&self, data: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn Validate(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IIsoImageManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IIsoImageManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIsoImageManager_Impl, const OFFSET: isize>() -> IIsoImageManager_Vtbl {
        unsafe extern "system" fn Path<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Path() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pval, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Stream() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(data, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetPath(::core::mem::transmute(&val)).into()
        }
        unsafe extern "system" fn SetStream<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStream(::windows_core::from_raw_borrowed(&data)).into()
        }
        unsafe extern "system" fn Validate<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Validate().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Path: Path::<Identity, Impl, OFFSET>,
            Stream: Stream::<Identity, Impl, OFFSET>,
            SetPath: SetPath::<Identity, Impl, OFFSET>,
            SetStream: SetStream::<Identity, Impl, OFFSET>,
            Validate: Validate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IIsoImageManager as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_System_Com_StructuredStorage\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IJolietDiscMaster_Impl: Sized {
    fn GetTotalDataBlocks(&self) -> ::windows_core::Result<i32>;
    fn GetUsedDataBlocks(&self) -> ::windows_core::Result<i32>;
    fn GetDataBlockSize(&self) -> ::windows_core::Result<i32>;
    fn AddData(&self, pstorage: ::core::option::Option<&super::super::System::Com::StructuredStorage::IStorage>, lfileoverwrite: i32) -> ::windows_core::Result<()>;
    fn GetJolietProperties(&self) -> ::windows_core::Result<super::super::System::Com::StructuredStorage::IPropertyStorage>;
    fn SetJolietProperties(&self, ppropstg: ::core::option::Option<&super::super::System::Com::StructuredStorage::IPropertyStorage>) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl ::windows_core::RuntimeName for IJolietDiscMaster {}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IJolietDiscMaster_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJolietDiscMaster_Impl, const OFFSET: isize>() -> IJolietDiscMaster_Vtbl {
        unsafe extern "system" fn GetTotalDataBlocks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTotalDataBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnblocks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUsedDataBlocks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUsedDataBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnblocks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataBlockSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblockbytes: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDataBlockSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnblockbytes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddData<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstorage: *mut ::core::ffi::c_void, lfileoverwrite: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddData(::windows_core::from_raw_borrowed(&pstorage), ::core::mem::transmute_copy(&lfileoverwrite)).into()
        }
        unsafe extern "system" fn GetJolietProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropstg: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetJolietProperties() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropstg, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJolietProperties<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropstg: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetJolietProperties(::windows_core::from_raw_borrowed(&ppropstg)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTotalDataBlocks: GetTotalDataBlocks::<Identity, Impl, OFFSET>,
            GetUsedDataBlocks: GetUsedDataBlocks::<Identity, Impl, OFFSET>,
            GetDataBlockSize: GetDataBlockSize::<Identity, Impl, OFFSET>,
            AddData: AddData::<Identity, Impl, OFFSET>,
            GetJolietProperties: GetJolietProperties::<Identity, Impl, OFFSET>,
            SetJolietProperties: SetJolietProperties::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IJolietDiscMaster as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMultisession_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn IsSupportedOnCurrentMediaState(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetInUse(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn InUse(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn ImportRecorder(&self) -> ::windows_core::Result<IDiscRecorder2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IMultisession {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMultisession_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultisession_Impl, const OFFSET: isize>() -> IMultisession_Vtbl {
        unsafe extern "system" fn IsSupportedOnCurrentMediaState<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultisession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsSupportedOnCurrentMediaState() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInUse<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultisession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetInUse(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn InUse<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultisession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InUse() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportRecorder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultisession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ImportRecorder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsSupportedOnCurrentMediaState: IsSupportedOnCurrentMediaState::<Identity, Impl, OFFSET>,
            SetInUse: SetInUse::<Identity, Impl, OFFSET>,
            InUse: InUse::<Identity, Impl, OFFSET>,
            ImportRecorder: ImportRecorder::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IMultisession as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMultisessionRandomWrite_Impl: Sized + IMultisession_Impl {
    fn WriteUnitSize(&self) -> ::windows_core::Result<i32>;
    fn LastWrittenAddress(&self) -> ::windows_core::Result<i32>;
    fn TotalSectorsOnMedia(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IMultisessionRandomWrite {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMultisessionRandomWrite_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultisessionRandomWrite_Impl, const OFFSET: isize>() -> IMultisessionRandomWrite_Vtbl {
        unsafe extern "system" fn WriteUnitSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultisessionRandomWrite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WriteUnitSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastWrittenAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultisessionRandomWrite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastWrittenAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultisessionRandomWrite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TotalSectorsOnMedia() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IMultisession_Vtbl::new::<Identity, Impl, OFFSET>(),
            WriteUnitSize: WriteUnitSize::<Identity, Impl, OFFSET>,
            LastWrittenAddress: LastWrittenAddress::<Identity, Impl, OFFSET>,
            TotalSectorsOnMedia: TotalSectorsOnMedia::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IMultisessionRandomWrite as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IMultisession as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMultisessionSequential_Impl: Sized + IMultisession_Impl {
    fn IsFirstDataSession(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn StartAddressOfPreviousSession(&self) -> ::windows_core::Result<i32>;
    fn LastWrittenAddressOfPreviousSession(&self) -> ::windows_core::Result<i32>;
    fn NextWritableAddress(&self) -> ::windows_core::Result<i32>;
    fn FreeSectorsOnMedia(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IMultisessionSequential {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMultisessionSequential_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultisessionSequential_Impl, const OFFSET: isize>() -> IMultisessionSequential_Vtbl {
        unsafe extern "system" fn IsFirstDataSession<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsFirstDataSession() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAddressOfPreviousSession<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartAddressOfPreviousSession() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastWrittenAddressOfPreviousSession<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastWrittenAddressOfPreviousSession() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextWritableAddress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NextWritableAddress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FreeSectorsOnMedia() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IMultisession_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsFirstDataSession: IsFirstDataSession::<Identity, Impl, OFFSET>,
            StartAddressOfPreviousSession: StartAddressOfPreviousSession::<Identity, Impl, OFFSET>,
            LastWrittenAddressOfPreviousSession: LastWrittenAddressOfPreviousSession::<Identity, Impl, OFFSET>,
            NextWritableAddress: NextWritableAddress::<Identity, Impl, OFFSET>,
            FreeSectorsOnMedia: FreeSectorsOnMedia::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IMultisessionSequential as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IMultisession as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IMultisessionSequential2_Impl: Sized + IMultisessionSequential_Impl {
    fn WriteUnitSize(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IMultisessionSequential2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IMultisessionSequential2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultisessionSequential2_Impl, const OFFSET: isize>() -> IMultisessionSequential2_Vtbl {
        unsafe extern "system" fn WriteUnitSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IMultisessionSequential2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WriteUnitSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IMultisessionSequential_Vtbl::new::<Identity, Impl, OFFSET>(), WriteUnitSize: WriteUnitSize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IMultisessionSequential2 as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<IMultisession as ::windows_core::ComInterface>::IID || iid == &<IMultisessionSequential as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IProgressItem_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Description(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn FirstBlock(&self) -> ::windows_core::Result<u32>;
    fn LastBlock(&self) -> ::windows_core::Result<u32>;
    fn BlockCount(&self) -> ::windows_core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IProgressItem {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IProgressItem_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProgressItem_Impl, const OFFSET: isize>() -> IProgressItem_Vtbl {
        unsafe extern "system" fn Description<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProgressItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(desc, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstBlock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProgressItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, block: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FirstBlock() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(block, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastBlock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProgressItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, block: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastBlock() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(block, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BlockCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProgressItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blocks: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BlockCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(blocks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Description: Description::<Identity, Impl, OFFSET>,
            FirstBlock: FirstBlock::<Identity, Impl, OFFSET>,
            LastBlock: LastBlock::<Identity, Impl, OFFSET>,
            BlockCount: BlockCount::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IProgressItem as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IProgressItems_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows_core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn get_Item(&self, index: i32) -> ::windows_core::Result<IProgressItem>;
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn ProgressItemFromBlock(&self, block: u32) -> ::windows_core::Result<IProgressItem>;
    fn ProgressItemFromDescription(&self, description: &::windows_core::BSTR) -> ::windows_core::Result<IProgressItem>;
    fn EnumProgressItems(&self) -> ::windows_core::Result<IEnumProgressItems>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IProgressItems {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IProgressItems_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProgressItems_Impl, const OFFSET: isize>() -> IProgressItems_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, item: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(count, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgressItemFromBlock<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, block: u32, item: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProgressItemFromBlock(::core::mem::transmute_copy(&block)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgressItemFromDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: ::std::mem::MaybeUninit<::windows_core::BSTR>, item: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ProgressItemFromDescription(::core::mem::transmute(&description)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(item, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumProgressItems<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EnumProgressItems() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(newenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            ProgressItemFromBlock: ProgressItemFromBlock::<Identity, Impl, OFFSET>,
            ProgressItemFromDescription: ProgressItemFromDescription::<Identity, Impl, OFFSET>,
            EnumProgressItems: EnumProgressItems::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IProgressItems as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRawCDImageCreator_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CreateResultImage(&self) -> ::windows_core::Result<super::super::System::Com::IStream>;
    fn AddTrack(&self, datatype: IMAPI_CD_SECTOR_TYPE, data: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<i32>;
    fn AddSpecialPregap(&self, data: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn AddSubcodeRWGenerator(&self, subcode: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn SetResultingImageType(&self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows_core::Result<()>;
    fn ResultingImageType(&self) -> ::windows_core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE>;
    fn StartOfLeadout(&self) -> ::windows_core::Result<i32>;
    fn SetStartOfLeadoutLimit(&self, value: i32) -> ::windows_core::Result<()>;
    fn StartOfLeadoutLimit(&self) -> ::windows_core::Result<i32>;
    fn SetDisableGaplessAudio(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn DisableGaplessAudio(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetMediaCatalogNumber(&self, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn MediaCatalogNumber(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetStartingTrackNumber(&self, value: i32) -> ::windows_core::Result<()>;
    fn StartingTrackNumber(&self) -> ::windows_core::Result<i32>;
    fn get_TrackInfo(&self, trackindex: i32) -> ::windows_core::Result<IRawCDImageTrackInfo>;
    fn NumberOfExistingTracks(&self) -> ::windows_core::Result<i32>;
    fn LastUsedUserSectorInImage(&self) -> ::windows_core::Result<i32>;
    fn ExpectedTableOfContents(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IRawCDImageCreator {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRawCDImageCreator_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>() -> IRawCDImageCreator_Vtbl {
        unsafe extern "system" fn CreateResultImage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultstream: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateResultImage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(resultstream, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTrack<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatype: IMAPI_CD_SECTOR_TYPE, data: *mut ::core::ffi::c_void, trackindex: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AddTrack(::core::mem::transmute_copy(&datatype), ::windows_core::from_raw_borrowed(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(trackindex, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSpecialPregap<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddSpecialPregap(::windows_core::from_raw_borrowed(&data)).into()
        }
        unsafe extern "system" fn AddSubcodeRWGenerator<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subcode: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddSubcodeRWGenerator(::windows_core::from_raw_borrowed(&subcode)).into()
        }
        unsafe extern "system" fn SetResultingImageType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetResultingImageType(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ResultingImageType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ResultingImageType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartOfLeadout<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartOfLeadout() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartOfLeadoutLimit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStartOfLeadoutLimit(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn StartOfLeadoutLimit<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartOfLeadoutLimit() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisableGaplessAudio<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDisableGaplessAudio(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn DisableGaplessAudio<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DisableGaplessAudio() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMediaCatalogNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMediaCatalogNumber(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn MediaCatalogNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MediaCatalogNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartingTrackNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStartingTrackNumber(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn StartingTrackNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartingTrackNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get_TrackInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trackindex: i32, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_TrackInfo(::core::mem::transmute_copy(&trackindex)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfExistingTracks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NumberOfExistingTracks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastUsedUserSectorInImage<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastUsedUserSectorInImage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpectedTableOfContents<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ExpectedTableOfContents() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            CreateResultImage: CreateResultImage::<Identity, Impl, OFFSET>,
            AddTrack: AddTrack::<Identity, Impl, OFFSET>,
            AddSpecialPregap: AddSpecialPregap::<Identity, Impl, OFFSET>,
            AddSubcodeRWGenerator: AddSubcodeRWGenerator::<Identity, Impl, OFFSET>,
            SetResultingImageType: SetResultingImageType::<Identity, Impl, OFFSET>,
            ResultingImageType: ResultingImageType::<Identity, Impl, OFFSET>,
            StartOfLeadout: StartOfLeadout::<Identity, Impl, OFFSET>,
            SetStartOfLeadoutLimit: SetStartOfLeadoutLimit::<Identity, Impl, OFFSET>,
            StartOfLeadoutLimit: StartOfLeadoutLimit::<Identity, Impl, OFFSET>,
            SetDisableGaplessAudio: SetDisableGaplessAudio::<Identity, Impl, OFFSET>,
            DisableGaplessAudio: DisableGaplessAudio::<Identity, Impl, OFFSET>,
            SetMediaCatalogNumber: SetMediaCatalogNumber::<Identity, Impl, OFFSET>,
            MediaCatalogNumber: MediaCatalogNumber::<Identity, Impl, OFFSET>,
            SetStartingTrackNumber: SetStartingTrackNumber::<Identity, Impl, OFFSET>,
            StartingTrackNumber: StartingTrackNumber::<Identity, Impl, OFFSET>,
            get_TrackInfo: get_TrackInfo::<Identity, Impl, OFFSET>,
            NumberOfExistingTracks: NumberOfExistingTracks::<Identity, Impl, OFFSET>,
            LastUsedUserSectorInImage: LastUsedUserSectorInImage::<Identity, Impl, OFFSET>,
            ExpectedTableOfContents: ExpectedTableOfContents::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IRawCDImageCreator as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IRawCDImageTrackInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StartingLba(&self) -> ::windows_core::Result<i32>;
    fn SectorCount(&self) -> ::windows_core::Result<i32>;
    fn TrackNumber(&self) -> ::windows_core::Result<i32>;
    fn SectorType(&self) -> ::windows_core::Result<IMAPI_CD_SECTOR_TYPE>;
    fn ISRC(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetISRC(&self, value: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn DigitalAudioCopySetting(&self) -> ::windows_core::Result<IMAPI_CD_TRACK_DIGITAL_COPY_SETTING>;
    fn SetDigitalAudioCopySetting(&self, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows_core::Result<()>;
    fn AudioHasPreemphasis(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetAudioHasPreemphasis(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn TrackIndexes(&self) -> ::windows_core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn AddTrackIndex(&self, lbaoffset: i32) -> ::windows_core::Result<()>;
    fn ClearTrackIndex(&self, lbaoffset: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IRawCDImageTrackInfo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IRawCDImageTrackInfo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>() -> IRawCDImageTrackInfo_Vtbl {
        unsafe extern "system" fn StartingLba<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartingLba() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SectorCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SectorCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrackNumber<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TrackNumber() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SectorType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_CD_SECTOR_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SectorType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ISRC<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ISRC() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetISRC<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetISRC(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn DigitalAudioCopySetting<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DigitalAudioCopySetting() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDigitalAudioCopySetting<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDigitalAudioCopySetting(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn AudioHasPreemphasis<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AudioHasPreemphasis() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioHasPreemphasis<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAudioHasPreemphasis(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn TrackIndexes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TrackIndexes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTrackIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbaoffset: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddTrackIndex(::core::mem::transmute_copy(&lbaoffset)).into()
        }
        unsafe extern "system" fn ClearTrackIndex<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbaoffset: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearTrackIndex(::core::mem::transmute_copy(&lbaoffset)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            StartingLba: StartingLba::<Identity, Impl, OFFSET>,
            SectorCount: SectorCount::<Identity, Impl, OFFSET>,
            TrackNumber: TrackNumber::<Identity, Impl, OFFSET>,
            SectorType: SectorType::<Identity, Impl, OFFSET>,
            ISRC: ISRC::<Identity, Impl, OFFSET>,
            SetISRC: SetISRC::<Identity, Impl, OFFSET>,
            DigitalAudioCopySetting: DigitalAudioCopySetting::<Identity, Impl, OFFSET>,
            SetDigitalAudioCopySetting: SetDigitalAudioCopySetting::<Identity, Impl, OFFSET>,
            AudioHasPreemphasis: AudioHasPreemphasis::<Identity, Impl, OFFSET>,
            SetAudioHasPreemphasis: SetAudioHasPreemphasis::<Identity, Impl, OFFSET>,
            TrackIndexes: TrackIndexes::<Identity, Impl, OFFSET>,
            AddTrackIndex: AddTrackIndex::<Identity, Impl, OFFSET>,
            ClearTrackIndex: ClearTrackIndex::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IRawCDImageTrackInfo as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"implement\"`*"]
pub trait IRedbookDiscMaster_Impl: Sized {
    fn GetTotalAudioTracks(&self) -> ::windows_core::Result<i32>;
    fn GetTotalAudioBlocks(&self) -> ::windows_core::Result<i32>;
    fn GetUsedAudioBlocks(&self) -> ::windows_core::Result<i32>;
    fn GetAvailableAudioTrackBlocks(&self) -> ::windows_core::Result<i32>;
    fn GetAudioBlockSize(&self) -> ::windows_core::Result<i32>;
    fn CreateAudioTrack(&self, nblocks: i32) -> ::windows_core::Result<()>;
    fn AddAudioTrackBlocks(&self, pby: *const u8, cb: i32) -> ::windows_core::Result<()>;
    fn CloseAudioTrack(&self) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for IRedbookDiscMaster {}
impl IRedbookDiscMaster_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRedbookDiscMaster_Impl, const OFFSET: isize>() -> IRedbookDiscMaster_Vtbl {
        unsafe extern "system" fn GetTotalAudioTracks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pntracks: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTotalAudioTracks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pntracks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTotalAudioBlocks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTotalAudioBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnblocks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUsedAudioBlocks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUsedAudioBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnblocks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAvailableAudioTrackBlocks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAvailableAudioTrackBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnblocks, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioBlockSize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblockbytes: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAudioBlockSize() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnblockbytes, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAudioTrack<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nblocks: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CreateAudioTrack(::core::mem::transmute_copy(&nblocks)).into()
        }
        unsafe extern "system" fn AddAudioTrackBlocks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pby: *const u8, cb: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddAudioTrackBlocks(::core::mem::transmute_copy(&pby), ::core::mem::transmute_copy(&cb)).into()
        }
        unsafe extern "system" fn CloseAudioTrack<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseAudioTrack().into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetTotalAudioTracks: GetTotalAudioTracks::<Identity, Impl, OFFSET>,
            GetTotalAudioBlocks: GetTotalAudioBlocks::<Identity, Impl, OFFSET>,
            GetUsedAudioBlocks: GetUsedAudioBlocks::<Identity, Impl, OFFSET>,
            GetAvailableAudioTrackBlocks: GetAvailableAudioTrackBlocks::<Identity, Impl, OFFSET>,
            GetAudioBlockSize: GetAudioBlockSize::<Identity, Impl, OFFSET>,
            CreateAudioTrack: CreateAudioTrack::<Identity, Impl, OFFSET>,
            AddAudioTrackBlocks: AddAudioTrackBlocks::<Identity, Impl, OFFSET>,
            CloseAudioTrack: CloseAudioTrack::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IRedbookDiscMaster as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IStreamConcatenate_Impl: Sized + super::super::System::Com::IStream_Impl {
    fn Initialize(&self, stream1: ::core::option::Option<&super::super::System::Com::IStream>, stream2: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn Initialize2(&self, streams: *const ::core::option::Option<super::super::System::Com::IStream>, streamcount: u32) -> ::windows_core::Result<()>;
    fn Append(&self, stream: ::core::option::Option<&super::super::System::Com::IStream>) -> ::windows_core::Result<()>;
    fn Append2(&self, streams: *const ::core::option::Option<super::super::System::Com::IStream>, streamcount: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IStreamConcatenate {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IStreamConcatenate_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStreamConcatenate_Impl, const OFFSET: isize>() -> IStreamConcatenate_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream1: *mut ::core::ffi::c_void, stream2: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::windows_core::from_raw_borrowed(&stream1), ::windows_core::from_raw_borrowed(&stream2)).into()
        }
        unsafe extern "system" fn Initialize2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streams: *const *mut ::core::ffi::c_void, streamcount: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize2(::core::mem::transmute_copy(&streams), ::core::mem::transmute_copy(&streamcount)).into()
        }
        unsafe extern "system" fn Append<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Append(::windows_core::from_raw_borrowed(&stream)).into()
        }
        unsafe extern "system" fn Append2<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streams: *const *mut ::core::ffi::c_void, streamcount: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Append2(::core::mem::transmute_copy(&streams), ::core::mem::transmute_copy(&streamcount)).into()
        }
        Self {
            base__: super::super::System::Com::IStream_Vtbl::new::<Identity, Impl, OFFSET>(),
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            Initialize2: Initialize2::<Identity, Impl, OFFSET>,
            Append: Append::<Identity, Impl, OFFSET>,
            Append2: Append2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IStreamConcatenate as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::ISequentialStream as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IStream as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IStreamInterleave_Impl: Sized + super::super::System::Com::IStream_Impl {
    fn Initialize(&self, streams: *const ::core::option::Option<super::super::System::Com::IStream>, interleavesizes: *const u32, streamcount: u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IStreamInterleave {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IStreamInterleave_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStreamInterleave_Impl, const OFFSET: isize>() -> IStreamInterleave_Vtbl {
        unsafe extern "system" fn Initialize<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStreamInterleave_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streams: *const *mut ::core::ffi::c_void, interleavesizes: *const u32, streamcount: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&streams), ::core::mem::transmute_copy(&interleavesizes), ::core::mem::transmute_copy(&streamcount)).into()
        }
        Self { base__: super::super::System::Com::IStream_Vtbl::new::<Identity, Impl, OFFSET>(), Initialize: Initialize::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IStreamInterleave as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::ISequentialStream as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IStream as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IStreamPseudoRandomBased_Impl: Sized + super::super::System::Com::IStream_Impl {
    fn SetSeed(&self, value: u32) -> ::windows_core::Result<()>;
    fn Seed(&self) -> ::windows_core::Result<u32>;
    fn put_ExtendedSeed(&self, values: *const u32, ecount: u32) -> ::windows_core::Result<()>;
    fn get_ExtendedSeed(&self, values: *mut *mut u32, ecount: *mut u32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IStreamPseudoRandomBased {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IStreamPseudoRandomBased_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStreamPseudoRandomBased_Impl, const OFFSET: isize>() -> IStreamPseudoRandomBased_Vtbl {
        unsafe extern "system" fn SetSeed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSeed(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Seed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Seed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn put_ExtendedSeed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: *const u32, ecount: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.put_ExtendedSeed(::core::mem::transmute_copy(&values), ::core::mem::transmute_copy(&ecount)).into()
        }
        unsafe extern "system" fn get_ExtendedSeed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: *mut *mut u32, ecount: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.get_ExtendedSeed(::core::mem::transmute_copy(&values), ::core::mem::transmute_copy(&ecount)).into()
        }
        Self {
            base__: super::super::System::Com::IStream_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetSeed: SetSeed::<Identity, Impl, OFFSET>,
            Seed: Seed::<Identity, Impl, OFFSET>,
            put_ExtendedSeed: put_ExtendedSeed::<Identity, Impl, OFFSET>,
            get_ExtendedSeed: get_ExtendedSeed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IStreamPseudoRandomBased as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::ISequentialStream as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IStream as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWriteEngine2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn WriteSection(&self, data: ::core::option::Option<&super::super::System::Com::IStream>, startingblockaddress: i32, numberofblocks: i32) -> ::windows_core::Result<()>;
    fn CancelWrite(&self) -> ::windows_core::Result<()>;
    fn SetRecorder(&self, value: ::core::option::Option<&IDiscRecorder2Ex>) -> ::windows_core::Result<()>;
    fn Recorder(&self) -> ::windows_core::Result<IDiscRecorder2Ex>;
    fn SetUseStreamingWrite12(&self, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn UseStreamingWrite12(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn SetStartingSectorsPerSecond(&self, value: i32) -> ::windows_core::Result<()>;
    fn StartingSectorsPerSecond(&self) -> ::windows_core::Result<i32>;
    fn SetEndingSectorsPerSecond(&self, value: i32) -> ::windows_core::Result<()>;
    fn EndingSectorsPerSecond(&self) -> ::windows_core::Result<i32>;
    fn SetBytesPerSector(&self, value: i32) -> ::windows_core::Result<()>;
    fn BytesPerSector(&self) -> ::windows_core::Result<i32>;
    fn WriteInProgress(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IWriteEngine2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWriteEngine2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: isize>() -> IWriteEngine2_Vtbl {
        unsafe extern "system" fn WriteSection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut ::core::ffi::c_void, startingblockaddress: i32, numberofblocks: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.WriteSection(::windows_core::from_raw_borrowed(&data), ::core::mem::transmute_copy(&startingblockaddress), ::core::mem::transmute_copy(&numberofblocks)).into()
        }
        unsafe extern "system" fn CancelWrite<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CancelWrite().into()
        }
        unsafe extern "system" fn SetRecorder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRecorder(::windows_core::from_raw_borrowed(&value)).into()
        }
        unsafe extern "system" fn Recorder<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Recorder() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseStreamingWrite12<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetUseStreamingWrite12(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn UseStreamingWrite12<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UseStreamingWrite12() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartingSectorsPerSecond<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetStartingSectorsPerSecond(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn StartingSectorsPerSecond<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartingSectorsPerSecond() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndingSectorsPerSecond<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetEndingSectorsPerSecond(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn EndingSectorsPerSecond<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.EndingSectorsPerSecond() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBytesPerSector<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBytesPerSector(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn BytesPerSector<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BytesPerSector() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteInProgress<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WriteInProgress() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            WriteSection: WriteSection::<Identity, Impl, OFFSET>,
            CancelWrite: CancelWrite::<Identity, Impl, OFFSET>,
            SetRecorder: SetRecorder::<Identity, Impl, OFFSET>,
            Recorder: Recorder::<Identity, Impl, OFFSET>,
            SetUseStreamingWrite12: SetUseStreamingWrite12::<Identity, Impl, OFFSET>,
            UseStreamingWrite12: UseStreamingWrite12::<Identity, Impl, OFFSET>,
            SetStartingSectorsPerSecond: SetStartingSectorsPerSecond::<Identity, Impl, OFFSET>,
            StartingSectorsPerSecond: StartingSectorsPerSecond::<Identity, Impl, OFFSET>,
            SetEndingSectorsPerSecond: SetEndingSectorsPerSecond::<Identity, Impl, OFFSET>,
            EndingSectorsPerSecond: EndingSectorsPerSecond::<Identity, Impl, OFFSET>,
            SetBytesPerSector: SetBytesPerSector::<Identity, Impl, OFFSET>,
            BytesPerSector: BytesPerSector::<Identity, Impl, OFFSET>,
            WriteInProgress: WriteInProgress::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWriteEngine2 as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWriteEngine2EventArgs_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StartLba(&self) -> ::windows_core::Result<i32>;
    fn SectorCount(&self) -> ::windows_core::Result<i32>;
    fn LastReadLba(&self) -> ::windows_core::Result<i32>;
    fn LastWrittenLba(&self) -> ::windows_core::Result<i32>;
    fn TotalSystemBuffer(&self) -> ::windows_core::Result<i32>;
    fn UsedSystemBuffer(&self) -> ::windows_core::Result<i32>;
    fn FreeSystemBuffer(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IWriteEngine2EventArgs {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWriteEngine2EventArgs_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWriteEngine2EventArgs_Impl, const OFFSET: isize>() -> IWriteEngine2EventArgs_Vtbl {
        unsafe extern "system" fn StartLba<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.StartLba() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SectorCount<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.SectorCount() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastReadLba<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastReadLba() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastWrittenLba<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LastWrittenLba() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalSystemBuffer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TotalSystemBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsedSystemBuffer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UsedSystemBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeSystemBuffer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.FreeSystemBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            StartLba: StartLba::<Identity, Impl, OFFSET>,
            SectorCount: SectorCount::<Identity, Impl, OFFSET>,
            LastReadLba: LastReadLba::<Identity, Impl, OFFSET>,
            LastWrittenLba: LastWrittenLba::<Identity, Impl, OFFSET>,
            TotalSystemBuffer: TotalSystemBuffer::<Identity, Impl, OFFSET>,
            UsedSystemBuffer: UsedSystemBuffer::<Identity, Impl, OFFSET>,
            FreeSystemBuffer: FreeSystemBuffer::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWriteEngine2EventArgs as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Storage_Imapi\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IWriteSpeedDescriptor_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn MediaType(&self) -> ::windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn RotationTypeIsPureCAV(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn WriteSpeed(&self) -> ::windows_core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IWriteSpeedDescriptor {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IWriteSpeedDescriptor_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWriteSpeedDescriptor_Impl, const OFFSET: isize>() -> IWriteSpeedDescriptor_Vtbl {
        unsafe extern "system" fn MediaType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWriteSpeedDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MediaType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotationTypeIsPureCAV<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWriteSpeedDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RotationTypeIsPureCAV() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteSpeed<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IWriteSpeedDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.WriteSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(value, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            MediaType: MediaType::<Identity, Impl, OFFSET>,
            RotationTypeIsPureCAV: RotationTypeIsPureCAV::<Identity, Impl, OFFSET>,
            WriteSpeed: WriteSpeed::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IWriteSpeedDescriptor as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
