#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DDiscFormat2DataEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Update(&mut self, object: ::core::option::Option<super::super::System::Com::IDispatch>, progress: ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DDiscFormat2DataEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DDiscFormat2DataEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DDiscFormat2DataEvents_Vtbl {
        unsafe extern "system" fn Update<Impl: DDiscFormat2DataEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, progress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update(::core::mem::transmute(&object), ::core::mem::transmute(&progress)).into()
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Update: Update::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DDiscFormat2DataEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DDiscFormat2EraseEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Update(&mut self, object: ::core::option::Option<super::super::System::Com::IDispatch>, elapsedseconds: i32, estimatedtotalseconds: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DDiscFormat2EraseEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DDiscFormat2EraseEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DDiscFormat2EraseEvents_Vtbl {
        unsafe extern "system" fn Update<Impl: DDiscFormat2EraseEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, elapsedseconds: i32, estimatedtotalseconds: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update(::core::mem::transmute(&object), ::core::mem::transmute_copy(&elapsedseconds), ::core::mem::transmute_copy(&estimatedtotalseconds)).into()
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Update: Update::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DDiscFormat2EraseEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DDiscFormat2RawCDEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Update(&mut self, object: ::core::option::Option<super::super::System::Com::IDispatch>, progress: ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DDiscFormat2RawCDEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DDiscFormat2RawCDEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DDiscFormat2RawCDEvents_Vtbl {
        unsafe extern "system" fn Update<Impl: DDiscFormat2RawCDEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, progress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update(::core::mem::transmute(&object), ::core::mem::transmute(&progress)).into()
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Update: Update::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DDiscFormat2RawCDEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DDiscFormat2TrackAtOnceEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Update(&mut self, object: ::core::option::Option<super::super::System::Com::IDispatch>, progress: ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DDiscFormat2TrackAtOnceEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DDiscFormat2TrackAtOnceEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DDiscFormat2TrackAtOnceEvents_Vtbl {
        unsafe extern "system" fn Update<Impl: DDiscFormat2TrackAtOnceEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, progress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update(::core::mem::transmute(&object), ::core::mem::transmute(&progress)).into()
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Update: Update::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DDiscFormat2TrackAtOnceEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DDiscMaster2Events_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn NotifyDeviceAdded(&mut self, object: ::core::option::Option<super::super::System::Com::IDispatch>, uniqueid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NotifyDeviceRemoved(&mut self, object: ::core::option::Option<super::super::System::Com::IDispatch>, uniqueid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DDiscMaster2Events_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DDiscMaster2Events_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DDiscMaster2Events_Vtbl {
        unsafe extern "system" fn NotifyDeviceAdded<Impl: DDiscMaster2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, uniqueid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyDeviceAdded(::core::mem::transmute(&object), ::core::mem::transmute_copy(&uniqueid)).into()
        }
        unsafe extern "system" fn NotifyDeviceRemoved<Impl: DDiscMaster2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, uniqueid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyDeviceRemoved(::core::mem::transmute(&object), ::core::mem::transmute_copy(&uniqueid)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            NotifyDeviceAdded: NotifyDeviceAdded::<Impl, IMPL_OFFSET>,
            NotifyDeviceRemoved: NotifyDeviceRemoved::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DDiscMaster2Events as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DFileSystemImageEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Update(&mut self, object: ::core::option::Option<super::super::System::Com::IDispatch>, currentfile: super::super::Foundation::BSTR, copiedsectors: i32, totalsectors: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DFileSystemImageEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DFileSystemImageEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DFileSystemImageEvents_Vtbl {
        unsafe extern "system" fn Update<Impl: DFileSystemImageEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, currentfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, copiedsectors: i32, totalsectors: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update(::core::mem::transmute(&object), ::core::mem::transmute_copy(&currentfile), ::core::mem::transmute_copy(&copiedsectors), ::core::mem::transmute_copy(&totalsectors)).into()
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Update: Update::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DFileSystemImageEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DFileSystemImageImportEvents_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn UpdateImport(&mut self, object: ::core::option::Option<super::super::System::Com::IDispatch>, filesystem: FsiFileSystems, currentitem: super::super::Foundation::BSTR, importeddirectoryitems: i32, totaldirectoryitems: i32, importedfileitems: i32, totalfileitems: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DFileSystemImageImportEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DFileSystemImageImportEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DFileSystemImageImportEvents_Vtbl {
        unsafe extern "system" fn UpdateImport<Impl: DFileSystemImageImportEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, filesystem: FsiFileSystems, currentitem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, importeddirectoryitems: i32, totaldirectoryitems: i32, importedfileitems: i32, totalfileitems: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateImport(::core::mem::transmute(&object), ::core::mem::transmute_copy(&filesystem), ::core::mem::transmute_copy(&currentitem), ::core::mem::transmute_copy(&importeddirectoryitems), ::core::mem::transmute_copy(&totaldirectoryitems), ::core::mem::transmute_copy(&importedfileitems), ::core::mem::transmute_copy(&totalfileitems)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            UpdateImport: UpdateImport::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DFileSystemImageImportEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DWriteEngine2Events_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Update(&mut self, object: ::core::option::Option<super::super::System::Com::IDispatch>, progress: ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DWriteEngine2Events_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DWriteEngine2Events_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DWriteEngine2Events_Vtbl {
        unsafe extern "system" fn Update<Impl: DWriteEngine2Events_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, progress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Update(::core::mem::transmute(&object), ::core::mem::transmute(&progress)).into()
        }
        Self { base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Update: Update::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DWriteEngine2Events as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IBlockRange_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StartLba(&mut self) -> ::windows::core::Result<i32>;
    fn EndLba(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IBlockRange_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBlockRange_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBlockRange_Vtbl {
        unsafe extern "system" fn StartLba<Impl: IBlockRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartLba() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndLba<Impl: IBlockRange_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndLba() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            StartLba: StartLba::<Impl, IMPL_OFFSET>,
            EndLba: EndLba::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBlockRange as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IBlockRangeList_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn BlockRanges(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IBlockRangeList_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBlockRangeList_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBlockRangeList_Vtbl {
        unsafe extern "system" fn BlockRanges<Impl: IBlockRangeList_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BlockRanges() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BlockRanges: BlockRanges::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBlockRangeList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IBootOptions_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn BootImage(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn Manufacturer(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetManufacturer(&mut self, newval: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn PlatformId(&mut self) -> ::windows::core::Result<PlatformId>;
    fn SetPlatformId(&mut self, newval: PlatformId) -> ::windows::core::Result<()>;
    fn Emulation(&mut self) -> ::windows::core::Result<EmulationType>;
    fn SetEmulation(&mut self, newval: EmulationType) -> ::windows::core::Result<()>;
    fn ImageSize(&mut self) -> ::windows::core::Result<u32>;
    fn AssignBootImage(&mut self, newval: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IBootOptions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBootOptions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBootOptions_Vtbl {
        unsafe extern "system" fn BootImage<Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BootImage() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Manufacturer<Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Manufacturer() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManufacturer<Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetManufacturer(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn PlatformId<Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut PlatformId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlatformId() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlatformId<Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: PlatformId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPlatformId(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn Emulation<Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut EmulationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Emulation() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmulation<Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: EmulationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEmulation(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ImageSize<Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssignBootImage<Impl: IBootOptions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AssignBootImage(::core::mem::transmute(&newval)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BootImage: BootImage::<Impl, IMPL_OFFSET>,
            Manufacturer: Manufacturer::<Impl, IMPL_OFFSET>,
            SetManufacturer: SetManufacturer::<Impl, IMPL_OFFSET>,
            PlatformId: PlatformId::<Impl, IMPL_OFFSET>,
            SetPlatformId: SetPlatformId::<Impl, IMPL_OFFSET>,
            Emulation: Emulation::<Impl, IMPL_OFFSET>,
            SetEmulation: SetEmulation::<Impl, IMPL_OFFSET>,
            ImageSize: ImageSize::<Impl, IMPL_OFFSET>,
            AssignBootImage: AssignBootImage::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBootOptions as ::windows::core::Interface>::IID
    }
}
pub trait IBurnVerification_Impl: Sized {
    fn SetBurnVerificationLevel(&mut self, value: IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows::core::Result<()>;
    fn BurnVerificationLevel(&mut self) -> ::windows::core::Result<IMAPI_BURN_VERIFICATION_LEVEL>;
}
impl IBurnVerification_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBurnVerification_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBurnVerification_Vtbl {
        unsafe extern "system" fn SetBurnVerificationLevel<Impl: IBurnVerification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBurnVerificationLevel(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn BurnVerificationLevel<Impl: IBurnVerification_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BurnVerificationLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SetBurnVerificationLevel: SetBurnVerificationLevel::<Impl, IMPL_OFFSET>,
            BurnVerificationLevel: BurnVerificationLevel::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBurnVerification as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDiscFormat2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn IsRecorderSupported(&mut self, recorder: ::core::option::Option<IDiscRecorder2>) -> ::windows::core::Result<i16>;
    fn IsCurrentMediaSupported(&mut self, recorder: ::core::option::Option<IDiscRecorder2>) -> ::windows::core::Result<i16>;
    fn MediaPhysicallyBlank(&mut self) -> ::windows::core::Result<i16>;
    fn MediaHeuristicallyBlank(&mut self) -> ::windows::core::Result<i16>;
    fn SupportedMediaTypes(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscFormat2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscFormat2_Vtbl {
        unsafe extern "system" fn IsRecorderSupported<Impl: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recorder: ::windows::core::RawPtr, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRecorderSupported(::core::mem::transmute(&recorder)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCurrentMediaSupported<Impl: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recorder: ::windows::core::RawPtr, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCurrentMediaSupported(::core::mem::transmute(&recorder)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaPhysicallyBlank<Impl: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaPhysicallyBlank() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaHeuristicallyBlank<Impl: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaHeuristicallyBlank() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedMediaTypes<Impl: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedMediaTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            IsRecorderSupported: IsRecorderSupported::<Impl, IMPL_OFFSET>,
            IsCurrentMediaSupported: IsCurrentMediaSupported::<Impl, IMPL_OFFSET>,
            MediaPhysicallyBlank: MediaPhysicallyBlank::<Impl, IMPL_OFFSET>,
            MediaHeuristicallyBlank: MediaHeuristicallyBlank::<Impl, IMPL_OFFSET>,
            SupportedMediaTypes: SupportedMediaTypes::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscFormat2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDiscFormat2Data_Impl: Sized + super::super::System::Com::IDispatch_Impl + IDiscFormat2_Impl {
    fn SetRecorder(&mut self, value: ::core::option::Option<IDiscRecorder2>) -> ::windows::core::Result<()>;
    fn Recorder(&mut self) -> ::windows::core::Result<IDiscRecorder2>;
    fn SetBufferUnderrunFreeDisabled(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn BufferUnderrunFreeDisabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetPostgapAlreadyInImage(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn PostgapAlreadyInImage(&mut self) -> ::windows::core::Result<i16>;
    fn CurrentMediaStatus(&mut self) -> ::windows::core::Result<IMAPI_FORMAT2_DATA_MEDIA_STATE>;
    fn WriteProtectStatus(&mut self) -> ::windows::core::Result<IMAPI_MEDIA_WRITE_PROTECT_STATE>;
    fn TotalSectorsOnMedia(&mut self) -> ::windows::core::Result<i32>;
    fn FreeSectorsOnMedia(&mut self) -> ::windows::core::Result<i32>;
    fn NextWritableAddress(&mut self) -> ::windows::core::Result<i32>;
    fn StartAddressOfPreviousSession(&mut self) -> ::windows::core::Result<i32>;
    fn LastWrittenAddressOfPreviousSession(&mut self) -> ::windows::core::Result<i32>;
    fn SetForceMediaToBeClosed(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn ForceMediaToBeClosed(&mut self) -> ::windows::core::Result<i16>;
    fn SetDisableConsumerDvdCompatibilityMode(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn DisableConsumerDvdCompatibilityMode(&mut self) -> ::windows::core::Result<i16>;
    fn CurrentPhysicalMediaType(&mut self) -> ::windows::core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SetClientName(&mut self, value: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ClientName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RequestedWriteSpeed(&mut self) -> ::windows::core::Result<i32>;
    fn RequestedRotationTypeIsPureCAV(&mut self) -> ::windows::core::Result<i16>;
    fn CurrentWriteSpeed(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentRotationTypeIsPureCAV(&mut self) -> ::windows::core::Result<i16>;
    fn SupportedWriteSpeeds(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedWriteSpeedDescriptors(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetForceOverwrite(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn ForceOverwrite(&mut self) -> ::windows::core::Result<i16>;
    fn MultisessionInterfaces(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn Write(&mut self, data: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn CancelWrite(&mut self) -> ::windows::core::Result<()>;
    fn SetWriteSpeed(&mut self, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscFormat2Data_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Data_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscFormat2Data_Vtbl {
        unsafe extern "system" fn SetRecorder<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRecorder(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Recorder<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recorder() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBufferUnderrunFreeDisabled(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferUnderrunFreeDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostgapAlreadyInImage<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPostgapAlreadyInImage(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn PostgapAlreadyInImage<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostgapAlreadyInImage() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentMediaStatus<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_DATA_MEDIA_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentMediaStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteProtectStatus<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_WRITE_PROTECT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteProtectStatus() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalSectorsOnMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FreeSectorsOnMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextWritableAddress<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextWritableAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAddressOfPreviousSession<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartAddressOfPreviousSession() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastWrittenAddressOfPreviousSession<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastWrittenAddressOfPreviousSession() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceMediaToBeClosed<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceMediaToBeClosed(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ForceMediaToBeClosed<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForceMediaToBeClosed() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisableConsumerDvdCompatibilityMode<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisableConsumerDvdCompatibilityMode(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn DisableConsumerDvdCompatibilityMode<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableConsumerDvdCompatibilityMode() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPhysicalMediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientName<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientName(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ClientName<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientName() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedWriteSpeed<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedWriteSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedRotationTypeIsPureCAV() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentWriteSpeed<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentWriteSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentRotationTypeIsPureCAV() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedWriteSpeeds() {
                ::core::result::Result::Ok(ok__) => {
                    *supportedspeeds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedWriteSpeedDescriptors() {
                ::core::result::Result::Ok(ok__) => {
                    *supportedspeeddescriptors = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceOverwrite<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetForceOverwrite(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ForceOverwrite<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForceOverwrite() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MultisessionInterfaces<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MultisessionInterfaces() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Write(::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn CancelWrite<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelWrite().into()
        }
        unsafe extern "system" fn SetWriteSpeed<Impl: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWriteSpeed(::core::mem::transmute_copy(&requestedsectorspersecond), ::core::mem::transmute_copy(&rotationtypeispurecav)).into()
        }
        Self {
            base: IDiscFormat2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetRecorder: SetRecorder::<Impl, IMPL_OFFSET>,
            Recorder: Recorder::<Impl, IMPL_OFFSET>,
            SetBufferUnderrunFreeDisabled: SetBufferUnderrunFreeDisabled::<Impl, IMPL_OFFSET>,
            BufferUnderrunFreeDisabled: BufferUnderrunFreeDisabled::<Impl, IMPL_OFFSET>,
            SetPostgapAlreadyInImage: SetPostgapAlreadyInImage::<Impl, IMPL_OFFSET>,
            PostgapAlreadyInImage: PostgapAlreadyInImage::<Impl, IMPL_OFFSET>,
            CurrentMediaStatus: CurrentMediaStatus::<Impl, IMPL_OFFSET>,
            WriteProtectStatus: WriteProtectStatus::<Impl, IMPL_OFFSET>,
            TotalSectorsOnMedia: TotalSectorsOnMedia::<Impl, IMPL_OFFSET>,
            FreeSectorsOnMedia: FreeSectorsOnMedia::<Impl, IMPL_OFFSET>,
            NextWritableAddress: NextWritableAddress::<Impl, IMPL_OFFSET>,
            StartAddressOfPreviousSession: StartAddressOfPreviousSession::<Impl, IMPL_OFFSET>,
            LastWrittenAddressOfPreviousSession: LastWrittenAddressOfPreviousSession::<Impl, IMPL_OFFSET>,
            SetForceMediaToBeClosed: SetForceMediaToBeClosed::<Impl, IMPL_OFFSET>,
            ForceMediaToBeClosed: ForceMediaToBeClosed::<Impl, IMPL_OFFSET>,
            SetDisableConsumerDvdCompatibilityMode: SetDisableConsumerDvdCompatibilityMode::<Impl, IMPL_OFFSET>,
            DisableConsumerDvdCompatibilityMode: DisableConsumerDvdCompatibilityMode::<Impl, IMPL_OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Impl, IMPL_OFFSET>,
            SetClientName: SetClientName::<Impl, IMPL_OFFSET>,
            ClientName: ClientName::<Impl, IMPL_OFFSET>,
            RequestedWriteSpeed: RequestedWriteSpeed::<Impl, IMPL_OFFSET>,
            RequestedRotationTypeIsPureCAV: RequestedRotationTypeIsPureCAV::<Impl, IMPL_OFFSET>,
            CurrentWriteSpeed: CurrentWriteSpeed::<Impl, IMPL_OFFSET>,
            CurrentRotationTypeIsPureCAV: CurrentRotationTypeIsPureCAV::<Impl, IMPL_OFFSET>,
            SupportedWriteSpeeds: SupportedWriteSpeeds::<Impl, IMPL_OFFSET>,
            SupportedWriteSpeedDescriptors: SupportedWriteSpeedDescriptors::<Impl, IMPL_OFFSET>,
            SetForceOverwrite: SetForceOverwrite::<Impl, IMPL_OFFSET>,
            ForceOverwrite: ForceOverwrite::<Impl, IMPL_OFFSET>,
            MultisessionInterfaces: MultisessionInterfaces::<Impl, IMPL_OFFSET>,
            Write: Write::<Impl, IMPL_OFFSET>,
            CancelWrite: CancelWrite::<Impl, IMPL_OFFSET>,
            SetWriteSpeed: SetWriteSpeed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscFormat2Data as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDiscFormat2DataEventArgs_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWriteEngine2EventArgs_Impl {
    fn ElapsedTime(&mut self) -> ::windows::core::Result<i32>;
    fn RemainingTime(&mut self) -> ::windows::core::Result<i32>;
    fn TotalTime(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentAction(&mut self) -> ::windows::core::Result<IMAPI_FORMAT2_DATA_WRITE_ACTION>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscFormat2DataEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2DataEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscFormat2DataEventArgs_Vtbl {
        unsafe extern "system" fn ElapsedTime<Impl: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElapsedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemainingTime<Impl: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemainingTime() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalTime<Impl: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalTime() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAction<Impl: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_DATA_WRITE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAction() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWriteEngine2EventArgs_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ElapsedTime: ElapsedTime::<Impl, IMPL_OFFSET>,
            RemainingTime: RemainingTime::<Impl, IMPL_OFFSET>,
            TotalTime: TotalTime::<Impl, IMPL_OFFSET>,
            CurrentAction: CurrentAction::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscFormat2DataEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDiscFormat2Erase_Impl: Sized + super::super::System::Com::IDispatch_Impl + IDiscFormat2_Impl {
    fn SetRecorder(&mut self, value: ::core::option::Option<IDiscRecorder2>) -> ::windows::core::Result<()>;
    fn Recorder(&mut self) -> ::windows::core::Result<IDiscRecorder2>;
    fn SetFullErase(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn FullErase(&mut self) -> ::windows::core::Result<i16>;
    fn CurrentPhysicalMediaType(&mut self) -> ::windows::core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SetClientName(&mut self, value: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ClientName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn EraseMedia(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscFormat2Erase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Erase_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscFormat2Erase_Vtbl {
        unsafe extern "system" fn SetRecorder<Impl: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRecorder(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Recorder<Impl: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recorder() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFullErase<Impl: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFullErase(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn FullErase<Impl: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FullErase() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Impl: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPhysicalMediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientName<Impl: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientName(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ClientName<Impl: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientName() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EraseMedia<Impl: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EraseMedia().into()
        }
        Self {
            base: IDiscFormat2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetRecorder: SetRecorder::<Impl, IMPL_OFFSET>,
            Recorder: Recorder::<Impl, IMPL_OFFSET>,
            SetFullErase: SetFullErase::<Impl, IMPL_OFFSET>,
            FullErase: FullErase::<Impl, IMPL_OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Impl, IMPL_OFFSET>,
            SetClientName: SetClientName::<Impl, IMPL_OFFSET>,
            ClientName: ClientName::<Impl, IMPL_OFFSET>,
            EraseMedia: EraseMedia::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscFormat2Erase as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDiscFormat2RawCD_Impl: Sized + super::super::System::Com::IDispatch_Impl + IDiscFormat2_Impl {
    fn PrepareMedia(&mut self) -> ::windows::core::Result<()>;
    fn WriteMedia(&mut self, data: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn WriteMedia2(&mut self, data: ::core::option::Option<super::super::System::Com::IStream>, streamleadinsectors: i32) -> ::windows::core::Result<()>;
    fn CancelWrite(&mut self) -> ::windows::core::Result<()>;
    fn ReleaseMedia(&mut self) -> ::windows::core::Result<()>;
    fn SetWriteSpeed(&mut self, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::core::Result<()>;
    fn SetRecorder(&mut self, value: ::core::option::Option<IDiscRecorder2>) -> ::windows::core::Result<()>;
    fn Recorder(&mut self) -> ::windows::core::Result<IDiscRecorder2>;
    fn SetBufferUnderrunFreeDisabled(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn BufferUnderrunFreeDisabled(&mut self) -> ::windows::core::Result<i16>;
    fn StartOfNextSession(&mut self) -> ::windows::core::Result<i32>;
    fn LastPossibleStartOfLeadout(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentPhysicalMediaType(&mut self) -> ::windows::core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SupportedSectorTypes(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetRequestedSectorType(&mut self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::core::Result<()>;
    fn RequestedSectorType(&mut self) -> ::windows::core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE>;
    fn SetClientName(&mut self, value: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ClientName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RequestedWriteSpeed(&mut self) -> ::windows::core::Result<i32>;
    fn RequestedRotationTypeIsPureCAV(&mut self) -> ::windows::core::Result<i16>;
    fn CurrentWriteSpeed(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentRotationTypeIsPureCAV(&mut self) -> ::windows::core::Result<i16>;
    fn SupportedWriteSpeeds(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedWriteSpeedDescriptors(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscFormat2RawCD_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCD_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscFormat2RawCD_Vtbl {
        unsafe extern "system" fn PrepareMedia<Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrepareMedia().into()
        }
        unsafe extern "system" fn WriteMedia<Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteMedia(::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn WriteMedia2<Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, streamleadinsectors: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteMedia2(::core::mem::transmute(&data), ::core::mem::transmute_copy(&streamleadinsectors)).into()
        }
        unsafe extern "system" fn CancelWrite<Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelWrite().into()
        }
        unsafe extern "system" fn ReleaseMedia<Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseMedia().into()
        }
        unsafe extern "system" fn SetWriteSpeed<Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWriteSpeed(::core::mem::transmute_copy(&requestedsectorspersecond), ::core::mem::transmute_copy(&rotationtypeispurecav)).into()
        }
        unsafe extern "system" fn SetRecorder<Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRecorder(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Recorder<Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recorder() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBufferUnderrunFreeDisabled(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferUnderrunFreeDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartOfNextSession<Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartOfNextSession() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastPossibleStartOfLeadout<Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastPossibleStartOfLeadout() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPhysicalMediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedSectorTypes<Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedSectorTypes() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestedSectorType<Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRequestedSectorType(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn RequestedSectorType<Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedSectorType() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientName<Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientName(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ClientName<Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientName() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedWriteSpeed<Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedWriteSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedRotationTypeIsPureCAV() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentWriteSpeed<Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentWriteSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentRotationTypeIsPureCAV() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedWriteSpeeds() {
                ::core::result::Result::Ok(ok__) => {
                    *supportedspeeds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Impl: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedWriteSpeedDescriptors() {
                ::core::result::Result::Ok(ok__) => {
                    *supportedspeeddescriptors = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDiscFormat2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PrepareMedia: PrepareMedia::<Impl, IMPL_OFFSET>,
            WriteMedia: WriteMedia::<Impl, IMPL_OFFSET>,
            WriteMedia2: WriteMedia2::<Impl, IMPL_OFFSET>,
            CancelWrite: CancelWrite::<Impl, IMPL_OFFSET>,
            ReleaseMedia: ReleaseMedia::<Impl, IMPL_OFFSET>,
            SetWriteSpeed: SetWriteSpeed::<Impl, IMPL_OFFSET>,
            SetRecorder: SetRecorder::<Impl, IMPL_OFFSET>,
            Recorder: Recorder::<Impl, IMPL_OFFSET>,
            SetBufferUnderrunFreeDisabled: SetBufferUnderrunFreeDisabled::<Impl, IMPL_OFFSET>,
            BufferUnderrunFreeDisabled: BufferUnderrunFreeDisabled::<Impl, IMPL_OFFSET>,
            StartOfNextSession: StartOfNextSession::<Impl, IMPL_OFFSET>,
            LastPossibleStartOfLeadout: LastPossibleStartOfLeadout::<Impl, IMPL_OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Impl, IMPL_OFFSET>,
            SupportedSectorTypes: SupportedSectorTypes::<Impl, IMPL_OFFSET>,
            SetRequestedSectorType: SetRequestedSectorType::<Impl, IMPL_OFFSET>,
            RequestedSectorType: RequestedSectorType::<Impl, IMPL_OFFSET>,
            SetClientName: SetClientName::<Impl, IMPL_OFFSET>,
            ClientName: ClientName::<Impl, IMPL_OFFSET>,
            RequestedWriteSpeed: RequestedWriteSpeed::<Impl, IMPL_OFFSET>,
            RequestedRotationTypeIsPureCAV: RequestedRotationTypeIsPureCAV::<Impl, IMPL_OFFSET>,
            CurrentWriteSpeed: CurrentWriteSpeed::<Impl, IMPL_OFFSET>,
            CurrentRotationTypeIsPureCAV: CurrentRotationTypeIsPureCAV::<Impl, IMPL_OFFSET>,
            SupportedWriteSpeeds: SupportedWriteSpeeds::<Impl, IMPL_OFFSET>,
            SupportedWriteSpeedDescriptors: SupportedWriteSpeedDescriptors::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscFormat2RawCD as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDiscFormat2RawCDEventArgs_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWriteEngine2EventArgs_Impl {
    fn CurrentAction(&mut self) -> ::windows::core::Result<IMAPI_FORMAT2_RAW_CD_WRITE_ACTION>;
    fn ElapsedTime(&mut self) -> ::windows::core::Result<i32>;
    fn RemainingTime(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscFormat2RawCDEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCDEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscFormat2RawCDEventArgs_Vtbl {
        unsafe extern "system" fn CurrentAction<Impl: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_WRITE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAction() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElapsedTime<Impl: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElapsedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemainingTime<Impl: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemainingTime() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWriteEngine2EventArgs_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CurrentAction: CurrentAction::<Impl, IMPL_OFFSET>,
            ElapsedTime: ElapsedTime::<Impl, IMPL_OFFSET>,
            RemainingTime: RemainingTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscFormat2RawCDEventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDiscFormat2TrackAtOnce_Impl: Sized + super::super::System::Com::IDispatch_Impl + IDiscFormat2_Impl {
    fn PrepareMedia(&mut self) -> ::windows::core::Result<()>;
    fn AddAudioTrack(&mut self, data: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn CancelAddTrack(&mut self) -> ::windows::core::Result<()>;
    fn ReleaseMedia(&mut self) -> ::windows::core::Result<()>;
    fn SetWriteSpeed(&mut self, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::core::Result<()>;
    fn SetRecorder(&mut self, value: ::core::option::Option<IDiscRecorder2>) -> ::windows::core::Result<()>;
    fn Recorder(&mut self) -> ::windows::core::Result<IDiscRecorder2>;
    fn SetBufferUnderrunFreeDisabled(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn BufferUnderrunFreeDisabled(&mut self) -> ::windows::core::Result<i16>;
    fn NumberOfExistingTracks(&mut self) -> ::windows::core::Result<i32>;
    fn TotalSectorsOnMedia(&mut self) -> ::windows::core::Result<i32>;
    fn FreeSectorsOnMedia(&mut self) -> ::windows::core::Result<i32>;
    fn UsedSectorsOnMedia(&mut self) -> ::windows::core::Result<i32>;
    fn SetDoNotFinalizeMedia(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn DoNotFinalizeMedia(&mut self) -> ::windows::core::Result<i16>;
    fn ExpectedTableOfContents(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CurrentPhysicalMediaType(&mut self) -> ::windows::core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SetClientName(&mut self, value: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ClientName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn RequestedWriteSpeed(&mut self) -> ::windows::core::Result<i32>;
    fn RequestedRotationTypeIsPureCAV(&mut self) -> ::windows::core::Result<i16>;
    fn CurrentWriteSpeed(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentRotationTypeIsPureCAV(&mut self) -> ::windows::core::Result<i16>;
    fn SupportedWriteSpeeds(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedWriteSpeedDescriptors(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscFormat2TrackAtOnce_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnce_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscFormat2TrackAtOnce_Vtbl {
        unsafe extern "system" fn PrepareMedia<Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PrepareMedia().into()
        }
        unsafe extern "system" fn AddAudioTrack<Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAudioTrack(::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn CancelAddTrack<Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelAddTrack().into()
        }
        unsafe extern "system" fn ReleaseMedia<Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseMedia().into()
        }
        unsafe extern "system" fn SetWriteSpeed<Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWriteSpeed(::core::mem::transmute_copy(&requestedsectorspersecond), ::core::mem::transmute_copy(&rotationtypeispurecav)).into()
        }
        unsafe extern "system" fn SetRecorder<Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRecorder(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Recorder<Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recorder() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBufferUnderrunFreeDisabled(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferUnderrunFreeDisabled() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfExistingTracks<Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfExistingTracks() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalSectorsOnMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FreeSectorsOnMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsedSectorsOnMedia<Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsedSectorsOnMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDoNotFinalizeMedia<Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDoNotFinalizeMedia(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn DoNotFinalizeMedia<Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoNotFinalizeMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpectedTableOfContents<Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpectedTableOfContents() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPhysicalMediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientName<Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetClientName(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ClientName<Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientName() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedWriteSpeed<Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedWriteSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedRotationTypeIsPureCAV() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentWriteSpeed<Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentWriteSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentRotationTypeIsPureCAV() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedWriteSpeeds() {
                ::core::result::Result::Ok(ok__) => {
                    *supportedspeeds = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Impl: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedWriteSpeedDescriptors() {
                ::core::result::Result::Ok(ok__) => {
                    *supportedspeeddescriptors = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IDiscFormat2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            PrepareMedia: PrepareMedia::<Impl, IMPL_OFFSET>,
            AddAudioTrack: AddAudioTrack::<Impl, IMPL_OFFSET>,
            CancelAddTrack: CancelAddTrack::<Impl, IMPL_OFFSET>,
            ReleaseMedia: ReleaseMedia::<Impl, IMPL_OFFSET>,
            SetWriteSpeed: SetWriteSpeed::<Impl, IMPL_OFFSET>,
            SetRecorder: SetRecorder::<Impl, IMPL_OFFSET>,
            Recorder: Recorder::<Impl, IMPL_OFFSET>,
            SetBufferUnderrunFreeDisabled: SetBufferUnderrunFreeDisabled::<Impl, IMPL_OFFSET>,
            BufferUnderrunFreeDisabled: BufferUnderrunFreeDisabled::<Impl, IMPL_OFFSET>,
            NumberOfExistingTracks: NumberOfExistingTracks::<Impl, IMPL_OFFSET>,
            TotalSectorsOnMedia: TotalSectorsOnMedia::<Impl, IMPL_OFFSET>,
            FreeSectorsOnMedia: FreeSectorsOnMedia::<Impl, IMPL_OFFSET>,
            UsedSectorsOnMedia: UsedSectorsOnMedia::<Impl, IMPL_OFFSET>,
            SetDoNotFinalizeMedia: SetDoNotFinalizeMedia::<Impl, IMPL_OFFSET>,
            DoNotFinalizeMedia: DoNotFinalizeMedia::<Impl, IMPL_OFFSET>,
            ExpectedTableOfContents: ExpectedTableOfContents::<Impl, IMPL_OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Impl, IMPL_OFFSET>,
            SetClientName: SetClientName::<Impl, IMPL_OFFSET>,
            ClientName: ClientName::<Impl, IMPL_OFFSET>,
            RequestedWriteSpeed: RequestedWriteSpeed::<Impl, IMPL_OFFSET>,
            RequestedRotationTypeIsPureCAV: RequestedRotationTypeIsPureCAV::<Impl, IMPL_OFFSET>,
            CurrentWriteSpeed: CurrentWriteSpeed::<Impl, IMPL_OFFSET>,
            CurrentRotationTypeIsPureCAV: CurrentRotationTypeIsPureCAV::<Impl, IMPL_OFFSET>,
            SupportedWriteSpeeds: SupportedWriteSpeeds::<Impl, IMPL_OFFSET>,
            SupportedWriteSpeedDescriptors: SupportedWriteSpeedDescriptors::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscFormat2TrackAtOnce as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDiscFormat2TrackAtOnceEventArgs_Impl: Sized + super::super::System::Com::IDispatch_Impl + IWriteEngine2EventArgs_Impl {
    fn CurrentTrackNumber(&mut self) -> ::windows::core::Result<i32>;
    fn CurrentAction(&mut self) -> ::windows::core::Result<IMAPI_FORMAT2_TAO_WRITE_ACTION>;
    fn ElapsedTime(&mut self) -> ::windows::core::Result<i32>;
    fn RemainingTime(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscFormat2TrackAtOnceEventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnceEventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscFormat2TrackAtOnceEventArgs_Vtbl {
        unsafe extern "system" fn CurrentTrackNumber<Impl: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentTrackNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAction<Impl: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_TAO_WRITE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAction() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElapsedTime<Impl: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElapsedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemainingTime<Impl: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemainingTime() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IWriteEngine2EventArgs_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CurrentTrackNumber: CurrentTrackNumber::<Impl, IMPL_OFFSET>,
            CurrentAction: CurrentAction::<Impl, IMPL_OFFSET>,
            ElapsedTime: ElapsedTime::<Impl, IMPL_OFFSET>,
            RemainingTime: RemainingTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscFormat2TrackAtOnceEventArgs as ::windows::core::Interface>::IID
    }
}
pub trait IDiscMaster_Impl: Sized {
    fn Open(&mut self) -> ::windows::core::Result<()>;
    fn EnumDiscMasterFormats(&mut self) -> ::windows::core::Result<IEnumDiscMasterFormats>;
    fn GetActiveDiscMasterFormat(&mut self) -> ::windows::core::Result<::windows::core::GUID>;
    fn SetActiveDiscMasterFormat(&mut self, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::Result<()>;
    fn EnumDiscRecorders(&mut self) -> ::windows::core::Result<IEnumDiscRecorders>;
    fn GetActiveDiscRecorder(&mut self) -> ::windows::core::Result<IDiscRecorder>;
    fn SetActiveDiscRecorder(&mut self, precorder: ::core::option::Option<IDiscRecorder>) -> ::windows::core::Result<()>;
    fn ClearFormatContent(&mut self) -> ::windows::core::Result<()>;
    fn ProgressAdvise(&mut self, pevents: ::core::option::Option<IDiscMasterProgressEvents>) -> ::windows::core::Result<usize>;
    fn ProgressUnadvise(&mut self, vcookie: usize) -> ::windows::core::Result<()>;
    fn RecordDisc(&mut self, bsimulate: u8, bejectafterburn: u8) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
impl IDiscMaster_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMaster_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscMaster_Vtbl {
        unsafe extern "system" fn Open<Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Open().into()
        }
        unsafe extern "system" fn EnumDiscMasterFormats<Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDiscMasterFormats() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveDiscMasterFormat<Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveDiscMasterFormat() {
                ::core::result::Result::Ok(ok__) => {
                    *lpiid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveDiscMasterFormat<Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActiveDiscMasterFormat(::core::mem::transmute_copy(&riid), ::core::mem::transmute_copy(&ppunk)).into()
        }
        unsafe extern "system" fn EnumDiscRecorders<Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDiscRecorders() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveDiscRecorder<Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprecorder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveDiscRecorder() {
                ::core::result::Result::Ok(ok__) => {
                    *pprecorder = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveDiscRecorder<Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precorder: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetActiveDiscRecorder(::core::mem::transmute(&precorder)).into()
        }
        unsafe extern "system" fn ClearFormatContent<Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearFormatContent().into()
        }
        unsafe extern "system" fn ProgressAdvise<Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevents: ::windows::core::RawPtr, pvcookie: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProgressAdvise(::core::mem::transmute(&pevents)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvcookie = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgressUnadvise<Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcookie: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProgressUnadvise(::core::mem::transmute_copy(&vcookie)).into()
        }
        unsafe extern "system" fn RecordDisc<Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsimulate: u8, bejectafterburn: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RecordDisc(::core::mem::transmute_copy(&bsimulate), ::core::mem::transmute_copy(&bejectafterburn)).into()
        }
        unsafe extern "system" fn Close<Impl: IDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Open: Open::<Impl, IMPL_OFFSET>,
            EnumDiscMasterFormats: EnumDiscMasterFormats::<Impl, IMPL_OFFSET>,
            GetActiveDiscMasterFormat: GetActiveDiscMasterFormat::<Impl, IMPL_OFFSET>,
            SetActiveDiscMasterFormat: SetActiveDiscMasterFormat::<Impl, IMPL_OFFSET>,
            EnumDiscRecorders: EnumDiscRecorders::<Impl, IMPL_OFFSET>,
            GetActiveDiscRecorder: GetActiveDiscRecorder::<Impl, IMPL_OFFSET>,
            SetActiveDiscRecorder: SetActiveDiscRecorder::<Impl, IMPL_OFFSET>,
            ClearFormatContent: ClearFormatContent::<Impl, IMPL_OFFSET>,
            ProgressAdvise: ProgressAdvise::<Impl, IMPL_OFFSET>,
            ProgressUnadvise: ProgressUnadvise::<Impl, IMPL_OFFSET>,
            RecordDisc: RecordDisc::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscMaster as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDiscMaster2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn IsSupportedEnvironment(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscMaster2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMaster2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscMaster2_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppunk = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupportedEnvironment<Impl: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupportedEnvironment() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            IsSupportedEnvironment: IsSupportedEnvironment::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscMaster2 as ::windows::core::Interface>::IID
    }
}
pub trait IDiscMasterProgressEvents_Impl: Sized {
    fn QueryCancel(&mut self) -> ::windows::core::Result<u8>;
    fn NotifyPnPActivity(&mut self) -> ::windows::core::Result<()>;
    fn NotifyAddProgress(&mut self, ncompletedsteps: i32, ntotalsteps: i32) -> ::windows::core::Result<()>;
    fn NotifyBlockProgress(&mut self, ncompleted: i32, ntotal: i32) -> ::windows::core::Result<()>;
    fn NotifyTrackProgress(&mut self, ncurrenttrack: i32, ntotaltracks: i32) -> ::windows::core::Result<()>;
    fn NotifyPreparingBurn(&mut self, nestimatedseconds: i32) -> ::windows::core::Result<()>;
    fn NotifyClosingDisc(&mut self, nestimatedseconds: i32) -> ::windows::core::Result<()>;
    fn NotifyBurnComplete(&mut self, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
    fn NotifyEraseComplete(&mut self, status: ::windows::core::HRESULT) -> ::windows::core::Result<()>;
}
impl IDiscMasterProgressEvents_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMasterProgressEvents_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscMasterProgressEvents_Vtbl {
        unsafe extern "system" fn QueryCancel<Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcancel: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryCancel() {
                ::core::result::Result::Ok(ok__) => {
                    *pbcancel = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyPnPActivity<Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyPnPActivity().into()
        }
        unsafe extern "system" fn NotifyAddProgress<Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncompletedsteps: i32, ntotalsteps: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyAddProgress(::core::mem::transmute_copy(&ncompletedsteps), ::core::mem::transmute_copy(&ntotalsteps)).into()
        }
        unsafe extern "system" fn NotifyBlockProgress<Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncompleted: i32, ntotal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyBlockProgress(::core::mem::transmute_copy(&ncompleted), ::core::mem::transmute_copy(&ntotal)).into()
        }
        unsafe extern "system" fn NotifyTrackProgress<Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncurrenttrack: i32, ntotaltracks: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyTrackProgress(::core::mem::transmute_copy(&ncurrenttrack), ::core::mem::transmute_copy(&ntotaltracks)).into()
        }
        unsafe extern "system" fn NotifyPreparingBurn<Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nestimatedseconds: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyPreparingBurn(::core::mem::transmute_copy(&nestimatedseconds)).into()
        }
        unsafe extern "system" fn NotifyClosingDisc<Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nestimatedseconds: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyClosingDisc(::core::mem::transmute_copy(&nestimatedseconds)).into()
        }
        unsafe extern "system" fn NotifyBurnComplete<Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyBurnComplete(::core::mem::transmute_copy(&status)).into()
        }
        unsafe extern "system" fn NotifyEraseComplete<Impl: IDiscMasterProgressEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NotifyEraseComplete(::core::mem::transmute_copy(&status)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            QueryCancel: QueryCancel::<Impl, IMPL_OFFSET>,
            NotifyPnPActivity: NotifyPnPActivity::<Impl, IMPL_OFFSET>,
            NotifyAddProgress: NotifyAddProgress::<Impl, IMPL_OFFSET>,
            NotifyBlockProgress: NotifyBlockProgress::<Impl, IMPL_OFFSET>,
            NotifyTrackProgress: NotifyTrackProgress::<Impl, IMPL_OFFSET>,
            NotifyPreparingBurn: NotifyPreparingBurn::<Impl, IMPL_OFFSET>,
            NotifyClosingDisc: NotifyClosingDisc::<Impl, IMPL_OFFSET>,
            NotifyBurnComplete: NotifyBurnComplete::<Impl, IMPL_OFFSET>,
            NotifyEraseComplete: NotifyEraseComplete::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscMasterProgressEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IDiscRecorder_Impl: Sized {
    fn Init(&mut self, pbyuniqueid: *const u8, nulidsize: u32, nuldrivenumber: u32) -> ::windows::core::Result<()>;
    fn GetRecorderGUID(&mut self, pbyuniqueid: *mut u8, ulbuffersize: u32, pulreturnsizerequired: *mut u32) -> ::windows::core::Result<()>;
    fn GetRecorderType(&mut self) -> ::windows::core::Result<RECORDER_TYPES>;
    fn GetDisplayNames(&mut self, pbstrvendorid: *mut super::super::Foundation::BSTR, pbstrproductid: *mut super::super::Foundation::BSTR, pbstrrevision: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetBasePnPID(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetRecorderProperties(&mut self) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::IPropertyStorage>;
    fn SetRecorderProperties(&mut self, ppropstg: ::core::option::Option<super::super::System::Com::StructuredStorage::IPropertyStorage>) -> ::windows::core::Result<()>;
    fn GetRecorderState(&mut self) -> ::windows::core::Result<DISC_RECORDER_STATE_FLAGS>;
    fn OpenExclusive(&mut self) -> ::windows::core::Result<()>;
    fn QueryMediaType(&mut self, fmediatype: *mut MEDIA_TYPES, fmediaflags: *mut MEDIA_FLAGS) -> ::windows::core::Result<()>;
    fn QueryMediaInfo(&mut self, pbsessions: *mut u8, pblasttrack: *mut u8, ulstartaddress: *mut u32, ulnextwritable: *mut u32, ulfreeblocks: *mut u32) -> ::windows::core::Result<()>;
    fn Eject(&mut self) -> ::windows::core::Result<()>;
    fn Erase(&mut self, bfullerase: u8) -> ::windows::core::Result<()>;
    fn Close(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IDiscRecorder_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscRecorder_Vtbl {
        unsafe extern "system" fn Init<Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbyuniqueid: *const u8, nulidsize: u32, nuldrivenumber: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Init(::core::mem::transmute_copy(&pbyuniqueid), ::core::mem::transmute_copy(&nulidsize), ::core::mem::transmute_copy(&nuldrivenumber)).into()
        }
        unsafe extern "system" fn GetRecorderGUID<Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbyuniqueid: *mut u8, ulbuffersize: u32, pulreturnsizerequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetRecorderGUID(::core::mem::transmute_copy(&pbyuniqueid), ::core::mem::transmute_copy(&ulbuffersize), ::core::mem::transmute_copy(&pulreturnsizerequired)).into()
        }
        unsafe extern "system" fn GetRecorderType<Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ftypecode: *mut RECORDER_TYPES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecorderType() {
                ::core::result::Result::Ok(ok__) => {
                    *ftypecode = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayNames<Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrvendorid: *mut super::super::Foundation::BSTR, pbstrproductid: *mut super::super::Foundation::BSTR, pbstrrevision: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDisplayNames(::core::mem::transmute_copy(&pbstrvendorid), ::core::mem::transmute_copy(&pbstrproductid), ::core::mem::transmute_copy(&pbstrrevision)).into()
        }
        unsafe extern "system" fn GetBasePnPID<Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbasepnpid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBasePnPID() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrbasepnpid = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPath<Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrpath = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecorderProperties<Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecorderProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropstg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecorderProperties<Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRecorderProperties(::core::mem::transmute(&ppropstg)).into()
        }
        unsafe extern "system" fn GetRecorderState<Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puldevstateflags: *mut DISC_RECORDER_STATE_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecorderState() {
                ::core::result::Result::Ok(ok__) => {
                    *puldevstateflags = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenExclusive<Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OpenExclusive().into()
        }
        unsafe extern "system" fn QueryMediaType<Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmediatype: *mut MEDIA_TYPES, fmediaflags: *mut MEDIA_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryMediaType(::core::mem::transmute_copy(&fmediatype), ::core::mem::transmute_copy(&fmediaflags)).into()
        }
        unsafe extern "system" fn QueryMediaInfo<Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsessions: *mut u8, pblasttrack: *mut u8, ulstartaddress: *mut u32, ulnextwritable: *mut u32, ulfreeblocks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).QueryMediaInfo(::core::mem::transmute_copy(&pbsessions), ::core::mem::transmute_copy(&pblasttrack), ::core::mem::transmute_copy(&ulstartaddress), ::core::mem::transmute_copy(&ulnextwritable), ::core::mem::transmute_copy(&ulfreeblocks)).into()
        }
        unsafe extern "system" fn Eject<Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Eject().into()
        }
        unsafe extern "system" fn Erase<Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfullerase: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Erase(::core::mem::transmute_copy(&bfullerase)).into()
        }
        unsafe extern "system" fn Close<Impl: IDiscRecorder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Close().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Init: Init::<Impl, IMPL_OFFSET>,
            GetRecorderGUID: GetRecorderGUID::<Impl, IMPL_OFFSET>,
            GetRecorderType: GetRecorderType::<Impl, IMPL_OFFSET>,
            GetDisplayNames: GetDisplayNames::<Impl, IMPL_OFFSET>,
            GetBasePnPID: GetBasePnPID::<Impl, IMPL_OFFSET>,
            GetPath: GetPath::<Impl, IMPL_OFFSET>,
            GetRecorderProperties: GetRecorderProperties::<Impl, IMPL_OFFSET>,
            SetRecorderProperties: SetRecorderProperties::<Impl, IMPL_OFFSET>,
            GetRecorderState: GetRecorderState::<Impl, IMPL_OFFSET>,
            OpenExclusive: OpenExclusive::<Impl, IMPL_OFFSET>,
            QueryMediaType: QueryMediaType::<Impl, IMPL_OFFSET>,
            QueryMediaInfo: QueryMediaInfo::<Impl, IMPL_OFFSET>,
            Eject: Eject::<Impl, IMPL_OFFSET>,
            Erase: Erase::<Impl, IMPL_OFFSET>,
            Close: Close::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscRecorder as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IDiscRecorder2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn EjectMedia(&mut self) -> ::windows::core::Result<()>;
    fn CloseTray(&mut self) -> ::windows::core::Result<()>;
    fn AcquireExclusiveAccess(&mut self, force: i16, __midl__idiscrecorder20000: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ReleaseExclusiveAccess(&mut self) -> ::windows::core::Result<()>;
    fn DisableMcn(&mut self) -> ::windows::core::Result<()>;
    fn EnableMcn(&mut self) -> ::windows::core::Result<()>;
    fn InitializeDiscRecorder(&mut self, recorderuniqueid: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ActiveDiscRecorder(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn VendorId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ProductId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ProductRevision(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn VolumeName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn VolumePathNames(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn DeviceCanLoadMedia(&mut self) -> ::windows::core::Result<i16>;
    fn LegacyDeviceNumber(&mut self) -> ::windows::core::Result<i32>;
    fn SupportedFeaturePages(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CurrentFeaturePages(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedProfiles(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CurrentProfiles(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SupportedModePages(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ExclusiveAccessOwner(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscRecorder2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscRecorder2_Vtbl {
        unsafe extern "system" fn EjectMedia<Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EjectMedia().into()
        }
        unsafe extern "system" fn CloseTray<Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseTray().into()
        }
        unsafe extern "system" fn AcquireExclusiveAccess<Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, force: i16, __midl__idiscrecorder20000: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AcquireExclusiveAccess(::core::mem::transmute_copy(&force), ::core::mem::transmute_copy(&__midl__idiscrecorder20000)).into()
        }
        unsafe extern "system" fn ReleaseExclusiveAccess<Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReleaseExclusiveAccess().into()
        }
        unsafe extern "system" fn DisableMcn<Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DisableMcn().into()
        }
        unsafe extern "system" fn EnableMcn<Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).EnableMcn().into()
        }
        unsafe extern "system" fn InitializeDiscRecorder<Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recorderuniqueid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).InitializeDiscRecorder(::core::mem::transmute_copy(&recorderuniqueid)).into()
        }
        unsafe extern "system" fn ActiveDiscRecorder<Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActiveDiscRecorder() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VendorId<Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VendorId() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductId<Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductId() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductRevision<Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductRevision() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeName<Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VolumeName() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumePathNames<Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VolumePathNames() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceCanLoadMedia<Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceCanLoadMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LegacyDeviceNumber<Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, legacydevicenumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LegacyDeviceNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *legacydevicenumber = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedFeaturePages<Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedFeaturePages() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFeaturePages<Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentFeaturePages() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedProfiles<Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedProfiles() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentProfiles<Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentProfiles() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedModePages<Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedModePages() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExclusiveAccessOwner<Impl: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExclusiveAccessOwner() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            EjectMedia: EjectMedia::<Impl, IMPL_OFFSET>,
            CloseTray: CloseTray::<Impl, IMPL_OFFSET>,
            AcquireExclusiveAccess: AcquireExclusiveAccess::<Impl, IMPL_OFFSET>,
            ReleaseExclusiveAccess: ReleaseExclusiveAccess::<Impl, IMPL_OFFSET>,
            DisableMcn: DisableMcn::<Impl, IMPL_OFFSET>,
            EnableMcn: EnableMcn::<Impl, IMPL_OFFSET>,
            InitializeDiscRecorder: InitializeDiscRecorder::<Impl, IMPL_OFFSET>,
            ActiveDiscRecorder: ActiveDiscRecorder::<Impl, IMPL_OFFSET>,
            VendorId: VendorId::<Impl, IMPL_OFFSET>,
            ProductId: ProductId::<Impl, IMPL_OFFSET>,
            ProductRevision: ProductRevision::<Impl, IMPL_OFFSET>,
            VolumeName: VolumeName::<Impl, IMPL_OFFSET>,
            VolumePathNames: VolumePathNames::<Impl, IMPL_OFFSET>,
            DeviceCanLoadMedia: DeviceCanLoadMedia::<Impl, IMPL_OFFSET>,
            LegacyDeviceNumber: LegacyDeviceNumber::<Impl, IMPL_OFFSET>,
            SupportedFeaturePages: SupportedFeaturePages::<Impl, IMPL_OFFSET>,
            CurrentFeaturePages: CurrentFeaturePages::<Impl, IMPL_OFFSET>,
            SupportedProfiles: SupportedProfiles::<Impl, IMPL_OFFSET>,
            CurrentProfiles: CurrentProfiles::<Impl, IMPL_OFFSET>,
            SupportedModePages: SupportedModePages::<Impl, IMPL_OFFSET>,
            ExclusiveAccessOwner: ExclusiveAccessOwner::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscRecorder2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_Foundation")]
pub trait IDiscRecorder2Ex_Impl: Sized {
    fn SendCommandNoData(&mut self, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32) -> ::windows::core::Result<()>;
    fn SendCommandSendDataToDevice(&mut self, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *const u8, buffersize: u32) -> ::windows::core::Result<()>;
    fn SendCommandGetDataFromDevice(&mut self, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *mut u8, buffersize: u32, bufferfetched: *mut u32) -> ::windows::core::Result<()>;
    fn ReadDvdStructure(&mut self, format: u32, address: u32, layer: u32, agid: u32, data: *mut *mut u8, count: *mut u32) -> ::windows::core::Result<()>;
    fn SendDvdStructure(&mut self, format: u32, data: *const u8, count: u32) -> ::windows::core::Result<()>;
    fn GetAdapterDescriptor(&mut self, data: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::Result<()>;
    fn GetDeviceDescriptor(&mut self, data: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::Result<()>;
    fn GetDiscInformation(&mut self, discinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::Result<()>;
    fn GetTrackInformation(&mut self, address: u32, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::Result<()>;
    fn GetFeaturePage(&mut self, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::Result<()>;
    fn GetModePage(&mut self, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::Result<()>;
    fn SetModePage(&mut self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: *const u8, bytesize: u32) -> ::windows::core::Result<()>;
    fn GetSupportedFeaturePages(&mut self, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut u32) -> ::windows::core::Result<()>;
    fn GetSupportedProfiles(&mut self, currentonly: super::super::Foundation::BOOLEAN, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut u32) -> ::windows::core::Result<()>;
    fn GetSupportedModePages(&mut self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut u32) -> ::windows::core::Result<()>;
    fn GetByteAlignmentMask(&mut self) -> ::windows::core::Result<u32>;
    fn GetMaximumNonPageAlignedTransferSize(&mut self) -> ::windows::core::Result<u32>;
    fn GetMaximumPageAlignedTransferSize(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(feature = "Win32_Foundation")]
impl IDiscRecorder2Ex_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2Ex_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscRecorder2Ex_Vtbl {
        unsafe extern "system" fn SendCommandNoData<Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendCommandNoData(::core::mem::transmute_copy(&cdb), ::core::mem::transmute_copy(&cdbsize), ::core::mem::transmute_copy(&sensebuffer), ::core::mem::transmute_copy(&timeout)).into()
        }
        unsafe extern "system" fn SendCommandSendDataToDevice<Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *const u8, buffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendCommandSendDataToDevice(::core::mem::transmute_copy(&cdb), ::core::mem::transmute_copy(&cdbsize), ::core::mem::transmute_copy(&sensebuffer), ::core::mem::transmute_copy(&timeout), ::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&buffersize)).into()
        }
        unsafe extern "system" fn SendCommandGetDataFromDevice<Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *mut u8, buffersize: u32, bufferfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendCommandGetDataFromDevice(::core::mem::transmute_copy(&cdb), ::core::mem::transmute_copy(&cdbsize), ::core::mem::transmute_copy(&sensebuffer), ::core::mem::transmute_copy(&timeout), ::core::mem::transmute_copy(&buffer), ::core::mem::transmute_copy(&buffersize), ::core::mem::transmute_copy(&bufferfetched)).into()
        }
        unsafe extern "system" fn ReadDvdStructure<Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: u32, address: u32, layer: u32, agid: u32, data: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ReadDvdStructure(::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&address), ::core::mem::transmute_copy(&layer), ::core::mem::transmute_copy(&agid), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn SendDvdStructure<Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: u32, data: *const u8, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SendDvdStructure(::core::mem::transmute_copy(&format), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&count)).into()
        }
        unsafe extern "system" fn GetAdapterDescriptor<Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAdapterDescriptor(::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetDeviceDescriptor<Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDeviceDescriptor(::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetDiscInformation<Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetDiscInformation(::core::mem::transmute_copy(&discinformation), ::core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetTrackInformation<Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: u32, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetTrackInformation(::core::mem::transmute_copy(&address), ::core::mem::transmute_copy(&addresstype), ::core::mem::transmute_copy(&trackinformation), ::core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetFeaturePage<Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetFeaturePage(::core::mem::transmute_copy(&requestedfeature), ::core::mem::transmute_copy(&currentfeatureonly), ::core::mem::transmute_copy(&featuredata), ::core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetModePage<Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetModePage(::core::mem::transmute_copy(&requestedmodepage), ::core::mem::transmute_copy(&requesttype), ::core::mem::transmute_copy(&modepagedata), ::core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn SetModePage<Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: *const u8, bytesize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetModePage(::core::mem::transmute_copy(&requesttype), ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetSupportedFeaturePages<Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSupportedFeaturePages(::core::mem::transmute_copy(&currentfeatureonly), ::core::mem::transmute_copy(&featuredata), ::core::mem::transmute_copy(&bytesize)).into()
        }
        unsafe extern "system" fn GetSupportedProfiles<Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentonly: super::super::Foundation::BOOLEAN, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSupportedProfiles(::core::mem::transmute_copy(&currentonly), ::core::mem::transmute_copy(&profiletypes), ::core::mem::transmute_copy(&validprofiles)).into()
        }
        unsafe extern "system" fn GetSupportedModePages<Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSupportedModePages(::core::mem::transmute_copy(&requesttype), ::core::mem::transmute_copy(&modepagetypes), ::core::mem::transmute_copy(&validpages)).into()
        }
        unsafe extern "system" fn GetByteAlignmentMask<Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetByteAlignmentMask() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaximumNonPageAlignedTransferSize<Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaximumNonPageAlignedTransferSize() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaximumPageAlignedTransferSize<Impl: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaximumPageAlignedTransferSize() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            SendCommandNoData: SendCommandNoData::<Impl, IMPL_OFFSET>,
            SendCommandSendDataToDevice: SendCommandSendDataToDevice::<Impl, IMPL_OFFSET>,
            SendCommandGetDataFromDevice: SendCommandGetDataFromDevice::<Impl, IMPL_OFFSET>,
            ReadDvdStructure: ReadDvdStructure::<Impl, IMPL_OFFSET>,
            SendDvdStructure: SendDvdStructure::<Impl, IMPL_OFFSET>,
            GetAdapterDescriptor: GetAdapterDescriptor::<Impl, IMPL_OFFSET>,
            GetDeviceDescriptor: GetDeviceDescriptor::<Impl, IMPL_OFFSET>,
            GetDiscInformation: GetDiscInformation::<Impl, IMPL_OFFSET>,
            GetTrackInformation: GetTrackInformation::<Impl, IMPL_OFFSET>,
            GetFeaturePage: GetFeaturePage::<Impl, IMPL_OFFSET>,
            GetModePage: GetModePage::<Impl, IMPL_OFFSET>,
            SetModePage: SetModePage::<Impl, IMPL_OFFSET>,
            GetSupportedFeaturePages: GetSupportedFeaturePages::<Impl, IMPL_OFFSET>,
            GetSupportedProfiles: GetSupportedProfiles::<Impl, IMPL_OFFSET>,
            GetSupportedModePages: GetSupportedModePages::<Impl, IMPL_OFFSET>,
            GetByteAlignmentMask: GetByteAlignmentMask::<Impl, IMPL_OFFSET>,
            GetMaximumNonPageAlignedTransferSize: GetMaximumNonPageAlignedTransferSize::<Impl, IMPL_OFFSET>,
            GetMaximumPageAlignedTransferSize: GetMaximumPageAlignedTransferSize::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDiscRecorder2Ex as ::windows::core::Interface>::IID
    }
}
pub trait IEnumDiscMasterFormats_Impl: Sized {
    fn Next(&mut self, cformats: u32, lpiidformatid: *mut ::windows::core::GUID, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, cformats: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumDiscMasterFormats>;
}
impl IEnumDiscMasterFormats_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDiscMasterFormats_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumDiscMasterFormats_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumDiscMasterFormats_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cformats: u32, lpiidformatid: *mut ::windows::core::GUID, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&cformats), ::core::mem::transmute_copy(&lpiidformatid), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumDiscMasterFormats_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cformats: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&cformats)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumDiscMasterFormats_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumDiscMasterFormats_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDiscMasterFormats as ::windows::core::Interface>::IID
    }
}
pub trait IEnumDiscRecorders_Impl: Sized {
    fn Next(&mut self, crecorders: u32, pprecorder: *mut ::core::option::Option<IDiscRecorder>, pcfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, crecorders: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumDiscRecorders>;
}
impl IEnumDiscRecorders_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDiscRecorders_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumDiscRecorders_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumDiscRecorders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crecorders: u32, pprecorder: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&crecorders), ::core::mem::transmute_copy(&pprecorder), ::core::mem::transmute_copy(&pcfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumDiscRecorders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crecorders: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&crecorders)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumDiscRecorders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumDiscRecorders_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumDiscRecorders as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumFsiItems_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<IFsiItem>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumFsiItems>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumFsiItems_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumFsiItems_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumFsiItems_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumFsiItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumFsiItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumFsiItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumFsiItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumFsiItems as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IEnumProgressItems_Impl: Sized {
    fn Next(&mut self, celt: u32, rgelt: *mut ::core::option::Option<IProgressItem>, pceltfetched: *mut u32) -> ::windows::core::Result<()>;
    fn Skip(&mut self, celt: u32) -> ::windows::core::Result<()>;
    fn Reset(&mut self) -> ::windows::core::Result<()>;
    fn Clone(&mut self) -> ::windows::core::Result<IEnumProgressItems>;
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumProgressItems_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumProgressItems_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumProgressItems_Vtbl {
        unsafe extern "system" fn Next<Impl: IEnumProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Impl: IEnumProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Impl: IEnumProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Reset().into()
        }
        unsafe extern "system" fn Clone<Impl: IEnumProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            Next: Next::<Impl, IMPL_OFFSET>,
            Skip: Skip::<Impl, IMPL_OFFSET>,
            Reset: Reset::<Impl, IMPL_OFFSET>,
            Clone: Clone::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IEnumProgressItems as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFileSystemImage_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Root(&mut self) -> ::windows::core::Result<IFsiDirectoryItem>;
    fn SessionStartBlock(&mut self) -> ::windows::core::Result<i32>;
    fn SetSessionStartBlock(&mut self, newval: i32) -> ::windows::core::Result<()>;
    fn FreeMediaBlocks(&mut self) -> ::windows::core::Result<i32>;
    fn SetFreeMediaBlocks(&mut self, newval: i32) -> ::windows::core::Result<()>;
    fn SetMaxMediaBlocksFromDevice(&mut self, discrecorder: ::core::option::Option<IDiscRecorder2>) -> ::windows::core::Result<()>;
    fn UsedBlocks(&mut self) -> ::windows::core::Result<i32>;
    fn VolumeName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetVolumeName(&mut self, newval: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ImportedVolumeName(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn BootImageOptions(&mut self) -> ::windows::core::Result<IBootOptions>;
    fn SetBootImageOptions(&mut self, newval: ::core::option::Option<IBootOptions>) -> ::windows::core::Result<()>;
    fn FileCount(&mut self) -> ::windows::core::Result<i32>;
    fn DirectoryCount(&mut self) -> ::windows::core::Result<i32>;
    fn WorkingDirectory(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetWorkingDirectory(&mut self, newval: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ChangePoint(&mut self) -> ::windows::core::Result<i32>;
    fn StrictFileSystemCompliance(&mut self) -> ::windows::core::Result<i16>;
    fn SetStrictFileSystemCompliance(&mut self, newval: i16) -> ::windows::core::Result<()>;
    fn UseRestrictedCharacterSet(&mut self) -> ::windows::core::Result<i16>;
    fn SetUseRestrictedCharacterSet(&mut self, newval: i16) -> ::windows::core::Result<()>;
    fn FileSystemsToCreate(&mut self) -> ::windows::core::Result<FsiFileSystems>;
    fn SetFileSystemsToCreate(&mut self, newval: FsiFileSystems) -> ::windows::core::Result<()>;
    fn FileSystemsSupported(&mut self) -> ::windows::core::Result<FsiFileSystems>;
    fn SetUDFRevision(&mut self, newval: i32) -> ::windows::core::Result<()>;
    fn UDFRevision(&mut self) -> ::windows::core::Result<i32>;
    fn UDFRevisionsSupported(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn ChooseImageDefaults(&mut self, discrecorder: ::core::option::Option<IDiscRecorder2>) -> ::windows::core::Result<()>;
    fn ChooseImageDefaultsForMediaType(&mut self, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::Result<()>;
    fn SetISO9660InterchangeLevel(&mut self, newval: i32) -> ::windows::core::Result<()>;
    fn ISO9660InterchangeLevel(&mut self) -> ::windows::core::Result<i32>;
    fn ISO9660InterchangeLevelsSupported(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn CreateResultImage(&mut self) -> ::windows::core::Result<IFileSystemImageResult>;
    fn Exists(&mut self, fullpath: super::super::Foundation::BSTR) -> ::windows::core::Result<FsiItemType>;
    fn CalculateDiscIdentifier(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn IdentifyFileSystemsOnDisc(&mut self, discrecorder: ::core::option::Option<IDiscRecorder2>) -> ::windows::core::Result<FsiFileSystems>;
    fn GetDefaultFileSystemForImport(&mut self, filesystems: FsiFileSystems) -> ::windows::core::Result<FsiFileSystems>;
    fn ImportFileSystem(&mut self) -> ::windows::core::Result<FsiFileSystems>;
    fn ImportSpecificFileSystem(&mut self, filesystemtouse: FsiFileSystems) -> ::windows::core::Result<()>;
    fn RollbackToChangePoint(&mut self, changepoint: i32) -> ::windows::core::Result<()>;
    fn LockInChangePoint(&mut self) -> ::windows::core::Result<()>;
    fn CreateDirectoryItem(&mut self, name: super::super::Foundation::BSTR) -> ::windows::core::Result<IFsiDirectoryItem>;
    fn CreateFileItem(&mut self, name: super::super::Foundation::BSTR) -> ::windows::core::Result<IFsiFileItem>;
    fn VolumeNameUDF(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn VolumeNameJoliet(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn VolumeNameISO9660(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn StageFiles(&mut self) -> ::windows::core::Result<i16>;
    fn SetStageFiles(&mut self, newval: i16) -> ::windows::core::Result<()>;
    fn MultisessionInterfaces(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetMultisessionInterfaces(&mut self, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFileSystemImage_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileSystemImage_Vtbl {
        unsafe extern "system" fn Root<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Root() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionStartBlock<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionStartBlock() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSessionStartBlock<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSessionStartBlock(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn FreeMediaBlocks<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FreeMediaBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFreeMediaBlocks<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFreeMediaBlocks(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn SetMaxMediaBlocksFromDevice<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discrecorder: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxMediaBlocksFromDevice(::core::mem::transmute(&discrecorder)).into()
        }
        unsafe extern "system" fn UsedBlocks<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsedBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeName<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VolumeName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolumeName<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVolumeName(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ImportedVolumeName<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportedVolumeName() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BootImageOptions<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BootImageOptions() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBootImageOptions<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBootImageOptions(::core::mem::transmute(&newval)).into()
        }
        unsafe extern "system" fn FileCount<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DirectoryCount<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DirectoryCount() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WorkingDirectory<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WorkingDirectory() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWorkingDirectory<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetWorkingDirectory(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ChangePoint<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangePoint() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrictFileSystemCompliance<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrictFileSystemCompliance() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrictFileSystemCompliance<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStrictFileSystemCompliance(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn UseRestrictedCharacterSet<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseRestrictedCharacterSet() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseRestrictedCharacterSet<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUseRestrictedCharacterSet(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn FileSystemsToCreate<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileSystemsToCreate() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileSystemsToCreate<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetFileSystemsToCreate(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn FileSystemsSupported<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileSystemsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUDFRevision<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUDFRevision(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn UDFRevision<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UDFRevision() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UDFRevisionsSupported<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UDFRevisionsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChooseImageDefaults<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discrecorder: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChooseImageDefaults(::core::mem::transmute(&discrecorder)).into()
        }
        unsafe extern "system" fn ChooseImageDefaultsForMediaType<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ChooseImageDefaultsForMediaType(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn SetISO9660InterchangeLevel<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetISO9660InterchangeLevel(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ISO9660InterchangeLevel<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ISO9660InterchangeLevel() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ISO9660InterchangeLevelsSupported<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ISO9660InterchangeLevelsSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResultImage<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateResultImage() {
                ::core::result::Result::Ok(ok__) => {
                    *resultstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Exists<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fullpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, itemtype: *mut FsiItemType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Exists(::core::mem::transmute_copy(&fullpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *itemtype = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CalculateDiscIdentifier<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discidentifier: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CalculateDiscIdentifier() {
                ::core::result::Result::Ok(ok__) => {
                    *discidentifier = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IdentifyFileSystemsOnDisc<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discrecorder: ::windows::core::RawPtr, filesystems: *mut FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IdentifyFileSystemsOnDisc(::core::mem::transmute(&discrecorder)) {
                ::core::result::Result::Ok(ok__) => {
                    *filesystems = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultFileSystemForImport<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesystems: FsiFileSystems, importdefault: *mut FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultFileSystemForImport(::core::mem::transmute_copy(&filesystems)) {
                ::core::result::Result::Ok(ok__) => {
                    *importdefault = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportFileSystem<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, importedfilesystem: *mut FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportFileSystem() {
                ::core::result::Result::Ok(ok__) => {
                    *importedfilesystem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportSpecificFileSystem<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesystemtouse: FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ImportSpecificFileSystem(::core::mem::transmute_copy(&filesystemtouse)).into()
        }
        unsafe extern "system" fn RollbackToChangePoint<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changepoint: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RollbackToChangePoint(::core::mem::transmute_copy(&changepoint)).into()
        }
        unsafe extern "system" fn LockInChangePoint<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).LockInChangePoint().into()
        }
        unsafe extern "system" fn CreateDirectoryItem<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDirectoryItem(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *newitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileItem<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFileItem(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *newitem = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeNameUDF<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VolumeNameUDF() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeNameJoliet<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VolumeNameJoliet() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeNameISO9660<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VolumeNameISO9660() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StageFiles<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StageFiles() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStageFiles<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStageFiles(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn MultisessionInterfaces<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MultisessionInterfaces() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultisessionInterfaces<Impl: IFileSystemImage_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMultisessionInterfaces(::core::mem::transmute_copy(&newval)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Root: Root::<Impl, IMPL_OFFSET>,
            SessionStartBlock: SessionStartBlock::<Impl, IMPL_OFFSET>,
            SetSessionStartBlock: SetSessionStartBlock::<Impl, IMPL_OFFSET>,
            FreeMediaBlocks: FreeMediaBlocks::<Impl, IMPL_OFFSET>,
            SetFreeMediaBlocks: SetFreeMediaBlocks::<Impl, IMPL_OFFSET>,
            SetMaxMediaBlocksFromDevice: SetMaxMediaBlocksFromDevice::<Impl, IMPL_OFFSET>,
            UsedBlocks: UsedBlocks::<Impl, IMPL_OFFSET>,
            VolumeName: VolumeName::<Impl, IMPL_OFFSET>,
            SetVolumeName: SetVolumeName::<Impl, IMPL_OFFSET>,
            ImportedVolumeName: ImportedVolumeName::<Impl, IMPL_OFFSET>,
            BootImageOptions: BootImageOptions::<Impl, IMPL_OFFSET>,
            SetBootImageOptions: SetBootImageOptions::<Impl, IMPL_OFFSET>,
            FileCount: FileCount::<Impl, IMPL_OFFSET>,
            DirectoryCount: DirectoryCount::<Impl, IMPL_OFFSET>,
            WorkingDirectory: WorkingDirectory::<Impl, IMPL_OFFSET>,
            SetWorkingDirectory: SetWorkingDirectory::<Impl, IMPL_OFFSET>,
            ChangePoint: ChangePoint::<Impl, IMPL_OFFSET>,
            StrictFileSystemCompliance: StrictFileSystemCompliance::<Impl, IMPL_OFFSET>,
            SetStrictFileSystemCompliance: SetStrictFileSystemCompliance::<Impl, IMPL_OFFSET>,
            UseRestrictedCharacterSet: UseRestrictedCharacterSet::<Impl, IMPL_OFFSET>,
            SetUseRestrictedCharacterSet: SetUseRestrictedCharacterSet::<Impl, IMPL_OFFSET>,
            FileSystemsToCreate: FileSystemsToCreate::<Impl, IMPL_OFFSET>,
            SetFileSystemsToCreate: SetFileSystemsToCreate::<Impl, IMPL_OFFSET>,
            FileSystemsSupported: FileSystemsSupported::<Impl, IMPL_OFFSET>,
            SetUDFRevision: SetUDFRevision::<Impl, IMPL_OFFSET>,
            UDFRevision: UDFRevision::<Impl, IMPL_OFFSET>,
            UDFRevisionsSupported: UDFRevisionsSupported::<Impl, IMPL_OFFSET>,
            ChooseImageDefaults: ChooseImageDefaults::<Impl, IMPL_OFFSET>,
            ChooseImageDefaultsForMediaType: ChooseImageDefaultsForMediaType::<Impl, IMPL_OFFSET>,
            SetISO9660InterchangeLevel: SetISO9660InterchangeLevel::<Impl, IMPL_OFFSET>,
            ISO9660InterchangeLevel: ISO9660InterchangeLevel::<Impl, IMPL_OFFSET>,
            ISO9660InterchangeLevelsSupported: ISO9660InterchangeLevelsSupported::<Impl, IMPL_OFFSET>,
            CreateResultImage: CreateResultImage::<Impl, IMPL_OFFSET>,
            Exists: Exists::<Impl, IMPL_OFFSET>,
            CalculateDiscIdentifier: CalculateDiscIdentifier::<Impl, IMPL_OFFSET>,
            IdentifyFileSystemsOnDisc: IdentifyFileSystemsOnDisc::<Impl, IMPL_OFFSET>,
            GetDefaultFileSystemForImport: GetDefaultFileSystemForImport::<Impl, IMPL_OFFSET>,
            ImportFileSystem: ImportFileSystem::<Impl, IMPL_OFFSET>,
            ImportSpecificFileSystem: ImportSpecificFileSystem::<Impl, IMPL_OFFSET>,
            RollbackToChangePoint: RollbackToChangePoint::<Impl, IMPL_OFFSET>,
            LockInChangePoint: LockInChangePoint::<Impl, IMPL_OFFSET>,
            CreateDirectoryItem: CreateDirectoryItem::<Impl, IMPL_OFFSET>,
            CreateFileItem: CreateFileItem::<Impl, IMPL_OFFSET>,
            VolumeNameUDF: VolumeNameUDF::<Impl, IMPL_OFFSET>,
            VolumeNameJoliet: VolumeNameJoliet::<Impl, IMPL_OFFSET>,
            VolumeNameISO9660: VolumeNameISO9660::<Impl, IMPL_OFFSET>,
            StageFiles: StageFiles::<Impl, IMPL_OFFSET>,
            SetStageFiles: SetStageFiles::<Impl, IMPL_OFFSET>,
            MultisessionInterfaces: MultisessionInterfaces::<Impl, IMPL_OFFSET>,
            SetMultisessionInterfaces: SetMultisessionInterfaces::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSystemImage as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFileSystemImage2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFileSystemImage_Impl {
    fn BootImageOptionsArray(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn SetBootImageOptionsArray(&mut self, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFileSystemImage2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileSystemImage2_Vtbl {
        unsafe extern "system" fn BootImageOptionsArray<Impl: IFileSystemImage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BootImageOptionsArray() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBootImageOptionsArray<Impl: IFileSystemImage2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBootImageOptionsArray(::core::mem::transmute_copy(&newval)).into()
        }
        Self {
            base: IFileSystemImage_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BootImageOptionsArray: BootImageOptionsArray::<Impl, IMPL_OFFSET>,
            SetBootImageOptionsArray: SetBootImageOptionsArray::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSystemImage2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFileSystemImage3_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFileSystemImage_Impl + IFileSystemImage2_Impl {
    fn CreateRedundantUdfMetadataFiles(&mut self) -> ::windows::core::Result<i16>;
    fn SetCreateRedundantUdfMetadataFiles(&mut self, newval: i16) -> ::windows::core::Result<()>;
    fn ProbeSpecificFileSystem(&mut self, filesystemtoprobe: FsiFileSystems) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFileSystemImage3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileSystemImage3_Vtbl {
        unsafe extern "system" fn CreateRedundantUdfMetadataFiles<Impl: IFileSystemImage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRedundantUdfMetadataFiles() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreateRedundantUdfMetadataFiles<Impl: IFileSystemImage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCreateRedundantUdfMetadataFiles(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn ProbeSpecificFileSystem<Impl: IFileSystemImage3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesystemtoprobe: FsiFileSystems, isappendable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProbeSpecificFileSystem(::core::mem::transmute_copy(&filesystemtoprobe)) {
                ::core::result::Result::Ok(ok__) => {
                    *isappendable = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IFileSystemImage2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateRedundantUdfMetadataFiles: CreateRedundantUdfMetadataFiles::<Impl, IMPL_OFFSET>,
            SetCreateRedundantUdfMetadataFiles: SetCreateRedundantUdfMetadataFiles::<Impl, IMPL_OFFSET>,
            ProbeSpecificFileSystem: ProbeSpecificFileSystem::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSystemImage3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFileSystemImageResult_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn ImageStream(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn ProgressItems(&mut self) -> ::windows::core::Result<IProgressItems>;
    fn TotalBlocks(&mut self) -> ::windows::core::Result<i32>;
    fn BlockSize(&mut self) -> ::windows::core::Result<i32>;
    fn DiscId(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFileSystemImageResult_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImageResult_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileSystemImageResult_Vtbl {
        unsafe extern "system" fn ImageStream<Impl: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageStream() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgressItems<Impl: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProgressItems() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalBlocks<Impl: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BlockSize<Impl: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BlockSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscId<Impl: IFileSystemImageResult_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiscId() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ImageStream: ImageStream::<Impl, IMPL_OFFSET>,
            ProgressItems: ProgressItems::<Impl, IMPL_OFFSET>,
            TotalBlocks: TotalBlocks::<Impl, IMPL_OFFSET>,
            BlockSize: BlockSize::<Impl, IMPL_OFFSET>,
            DiscId: DiscId::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSystemImageResult as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFileSystemImageResult2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFileSystemImageResult_Impl {
    fn ModifiedBlocks(&mut self) -> ::windows::core::Result<IBlockRangeList>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFileSystemImageResult2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImageResult2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileSystemImageResult2_Vtbl {
        unsafe extern "system" fn ModifiedBlocks<Impl: IFileSystemImageResult2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModifiedBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IFileSystemImageResult_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), ModifiedBlocks: ModifiedBlocks::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSystemImageResult2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsiDirectoryItem_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsiItem_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn Item(&mut self, path: super::super::Foundation::BSTR) -> ::windows::core::Result<IFsiItem>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn EnumFsiItems(&mut self) -> ::windows::core::Result<IEnumFsiItems>;
    fn AddDirectory(&mut self, path: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddFile(&mut self, path: super::super::Foundation::BSTR, filedata: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn AddTree(&mut self, sourcedirectory: super::super::Foundation::BSTR, includebasedirectory: i16) -> ::windows::core::Result<()>;
    fn Add(&mut self, item: ::core::option::Option<IFsiItem>) -> ::windows::core::Result<()>;
    fn Remove(&mut self, path: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoveTree(&mut self, path: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsiDirectoryItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsiDirectoryItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsiDirectoryItem_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&path)) {
                ::core::result::Result::Ok(ok__) => {
                    *item = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFsiItems<Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumFsiItems() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDirectory<Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDirectory(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn AddFile<Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filedata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddFile(::core::mem::transmute_copy(&path), ::core::mem::transmute(&filedata)).into()
        }
        unsafe extern "system" fn AddTree<Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcedirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, includebasedirectory: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTree(::core::mem::transmute_copy(&sourcedirectory), ::core::mem::transmute_copy(&includebasedirectory)).into()
        }
        unsafe extern "system" fn Add<Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Add(::core::mem::transmute(&item)).into()
        }
        unsafe extern "system" fn Remove<Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&path)).into()
        }
        unsafe extern "system" fn RemoveTree<Impl: IFsiDirectoryItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveTree(::core::mem::transmute_copy(&path)).into()
        }
        Self {
            base: IFsiItem_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            EnumFsiItems: EnumFsiItems::<Impl, IMPL_OFFSET>,
            AddDirectory: AddDirectory::<Impl, IMPL_OFFSET>,
            AddFile: AddFile::<Impl, IMPL_OFFSET>,
            AddTree: AddTree::<Impl, IMPL_OFFSET>,
            Add: Add::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            RemoveTree: RemoveTree::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsiDirectoryItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsiDirectoryItem2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsiItem_Impl + IFsiDirectoryItem_Impl {
    fn AddTreeWithNamedStreams(&mut self, sourcedirectory: super::super::Foundation::BSTR, includebasedirectory: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsiDirectoryItem2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsiDirectoryItem2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsiDirectoryItem2_Vtbl {
        unsafe extern "system" fn AddTreeWithNamedStreams<Impl: IFsiDirectoryItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcedirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, includebasedirectory: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTreeWithNamedStreams(::core::mem::transmute_copy(&sourcedirectory), ::core::mem::transmute_copy(&includebasedirectory)).into()
        }
        Self {
            base: IFsiDirectoryItem_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddTreeWithNamedStreams: AddTreeWithNamedStreams::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsiDirectoryItem2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsiFileItem_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsiItem_Impl {
    fn DataSize(&mut self) -> ::windows::core::Result<i64>;
    fn DataSize32BitLow(&mut self) -> ::windows::core::Result<i32>;
    fn DataSize32BitHigh(&mut self) -> ::windows::core::Result<i32>;
    fn Data(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn SetData(&mut self, newval: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsiFileItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsiFileItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsiFileItem_Vtbl {
        unsafe extern "system" fn DataSize<Impl: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataSize32BitLow<Impl: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataSize32BitLow() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataSize32BitHigh<Impl: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataSize32BitHigh() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Impl: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IFsiFileItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetData(::core::mem::transmute(&newval)).into()
        }
        Self {
            base: IFsiItem_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            DataSize: DataSize::<Impl, IMPL_OFFSET>,
            DataSize32BitLow: DataSize32BitLow::<Impl, IMPL_OFFSET>,
            DataSize32BitHigh: DataSize32BitHigh::<Impl, IMPL_OFFSET>,
            Data: Data::<Impl, IMPL_OFFSET>,
            SetData: SetData::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsiFileItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsiFileItem2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IFsiItem_Impl + IFsiFileItem_Impl {
    fn FsiNamedStreams(&mut self) -> ::windows::core::Result<IFsiNamedStreams>;
    fn IsNamedStream(&mut self) -> ::windows::core::Result<i16>;
    fn AddStream(&mut self, name: super::super::Foundation::BSTR, streamdata: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn RemoveStream(&mut self, name: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsRealTime(&mut self) -> ::windows::core::Result<i16>;
    fn SetIsRealTime(&mut self, newval: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsiFileItem2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsiFileItem2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsiFileItem2_Vtbl {
        unsafe extern "system" fn FsiNamedStreams<Impl: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FsiNamedStreams() {
                ::core::result::Result::Ok(ok__) => {
                    *streams = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNamedStream<Impl: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsNamedStream() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStream<Impl: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, streamdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStream(::core::mem::transmute_copy(&name), ::core::mem::transmute(&streamdata)).into()
        }
        unsafe extern "system" fn RemoveStream<Impl: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveStream(::core::mem::transmute_copy(&name)).into()
        }
        unsafe extern "system" fn IsRealTime<Impl: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRealTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRealTime<Impl: IFsiFileItem2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsRealTime(::core::mem::transmute_copy(&newval)).into()
        }
        Self {
            base: IFsiFileItem_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            FsiNamedStreams: FsiNamedStreams::<Impl, IMPL_OFFSET>,
            IsNamedStream: IsNamedStream::<Impl, IMPL_OFFSET>,
            AddStream: AddStream::<Impl, IMPL_OFFSET>,
            RemoveStream: RemoveStream::<Impl, IMPL_OFFSET>,
            IsRealTime: IsRealTime::<Impl, IMPL_OFFSET>,
            SetIsRealTime: SetIsRealTime::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsiFileItem2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsiItem_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn FullPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn CreationTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetCreationTime(&mut self, newval: f64) -> ::windows::core::Result<()>;
    fn LastAccessedTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetLastAccessedTime(&mut self, newval: f64) -> ::windows::core::Result<()>;
    fn LastModifiedTime(&mut self) -> ::windows::core::Result<f64>;
    fn SetLastModifiedTime(&mut self, newval: f64) -> ::windows::core::Result<()>;
    fn IsHidden(&mut self) -> ::windows::core::Result<i16>;
    fn SetIsHidden(&mut self, newval: i16) -> ::windows::core::Result<()>;
    fn FileSystemName(&mut self, filesystem: FsiFileSystems) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn FileSystemPath(&mut self, filesystem: FsiFileSystems) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsiItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsiItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsiItem_Vtbl {
        unsafe extern "system" fn Name<Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FullPath<Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FullPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreationTime<Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreationTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreationTime<Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetCreationTime(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn LastAccessedTime<Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastAccessedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastAccessedTime<Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastAccessedTime(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn LastModifiedTime<Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastModifiedTime() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastModifiedTime<Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLastModifiedTime(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn IsHidden<Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHidden() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHidden<Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsHidden(::core::mem::transmute_copy(&newval)).into()
        }
        unsafe extern "system" fn FileSystemName<Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesystem: FsiFileSystems, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileSystemName(::core::mem::transmute_copy(&filesystem)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileSystemPath<Impl: IFsiItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesystem: FsiFileSystems, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileSystemPath(::core::mem::transmute_copy(&filesystem)) {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            FullPath: FullPath::<Impl, IMPL_OFFSET>,
            CreationTime: CreationTime::<Impl, IMPL_OFFSET>,
            SetCreationTime: SetCreationTime::<Impl, IMPL_OFFSET>,
            LastAccessedTime: LastAccessedTime::<Impl, IMPL_OFFSET>,
            SetLastAccessedTime: SetLastAccessedTime::<Impl, IMPL_OFFSET>,
            LastModifiedTime: LastModifiedTime::<Impl, IMPL_OFFSET>,
            SetLastModifiedTime: SetLastModifiedTime::<Impl, IMPL_OFFSET>,
            IsHidden: IsHidden::<Impl, IMPL_OFFSET>,
            SetIsHidden: SetIsHidden::<Impl, IMPL_OFFSET>,
            FileSystemName: FileSystemName::<Impl, IMPL_OFFSET>,
            FileSystemPath: FileSystemPath::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsiItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsiNamedStreams_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<IFsiFileItem2>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn EnumNamedStreams(&mut self) -> ::windows::core::Result<IEnumFsiItems>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsiNamedStreams_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsiNamedStreams_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsiNamedStreams_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *item = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumNamedStreams<Impl: IFsiNamedStreams_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumNamedStreams() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            EnumNamedStreams: EnumNamedStreams::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsiNamedStreams as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IIsoImageManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Path(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn Stream(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn SetPath(&mut self, val: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn SetStream(&mut self, data: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn Validate(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IIsoImageManager_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIsoImageManager_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIsoImageManager_Vtbl {
        unsafe extern "system" fn Path<Impl: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path() {
                ::core::result::Result::Ok(ok__) => {
                    *pval = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stream<Impl: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stream() {
                ::core::result::Result::Ok(ok__) => {
                    *data = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Impl: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPath(::core::mem::transmute_copy(&val)).into()
        }
        unsafe extern "system" fn SetStream<Impl: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStream(::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn Validate<Impl: IIsoImageManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Validate().into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Path: Path::<Impl, IMPL_OFFSET>,
            Stream: Stream::<Impl, IMPL_OFFSET>,
            SetPath: SetPath::<Impl, IMPL_OFFSET>,
            SetStream: SetStream::<Impl, IMPL_OFFSET>,
            Validate: Validate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IIsoImageManager as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
pub trait IJolietDiscMaster_Impl: Sized {
    fn GetTotalDataBlocks(&mut self) -> ::windows::core::Result<i32>;
    fn GetUsedDataBlocks(&mut self) -> ::windows::core::Result<i32>;
    fn GetDataBlockSize(&mut self) -> ::windows::core::Result<i32>;
    fn AddData(&mut self, pstorage: ::core::option::Option<super::super::System::Com::StructuredStorage::IStorage>, lfileoverwrite: i32) -> ::windows::core::Result<()>;
    fn GetJolietProperties(&mut self) -> ::windows::core::Result<super::super::System::Com::StructuredStorage::IPropertyStorage>;
    fn SetJolietProperties(&mut self, ppropstg: ::core::option::Option<super::super::System::Com::StructuredStorage::IPropertyStorage>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IJolietDiscMaster_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJolietDiscMaster_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJolietDiscMaster_Vtbl {
        unsafe extern "system" fn GetTotalDataBlocks<Impl: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTotalDataBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    *pnblocks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUsedDataBlocks<Impl: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUsedDataBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    *pnblocks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataBlockSize<Impl: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblockbytes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataBlockSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pnblockbytes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddData<Impl: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstorage: ::windows::core::RawPtr, lfileoverwrite: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddData(::core::mem::transmute(&pstorage), ::core::mem::transmute_copy(&lfileoverwrite)).into()
        }
        unsafe extern "system" fn GetJolietProperties<Impl: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJolietProperties() {
                ::core::result::Result::Ok(ok__) => {
                    *pppropstg = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJolietProperties<Impl: IJolietDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetJolietProperties(::core::mem::transmute(&ppropstg)).into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetTotalDataBlocks: GetTotalDataBlocks::<Impl, IMPL_OFFSET>,
            GetUsedDataBlocks: GetUsedDataBlocks::<Impl, IMPL_OFFSET>,
            GetDataBlockSize: GetDataBlockSize::<Impl, IMPL_OFFSET>,
            AddData: AddData::<Impl, IMPL_OFFSET>,
            GetJolietProperties: GetJolietProperties::<Impl, IMPL_OFFSET>,
            SetJolietProperties: SetJolietProperties::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IJolietDiscMaster as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMultisession_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn IsSupportedOnCurrentMediaState(&mut self) -> ::windows::core::Result<i16>;
    fn SetInUse(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn InUse(&mut self) -> ::windows::core::Result<i16>;
    fn ImportRecorder(&mut self) -> ::windows::core::Result<IDiscRecorder2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMultisession_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultisession_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultisession_Vtbl {
        unsafe extern "system" fn IsSupportedOnCurrentMediaState<Impl: IMultisession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupportedOnCurrentMediaState() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInUse<Impl: IMultisession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetInUse(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn InUse<Impl: IMultisession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InUse() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportRecorder<Impl: IMultisession_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportRecorder() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            IsSupportedOnCurrentMediaState: IsSupportedOnCurrentMediaState::<Impl, IMPL_OFFSET>,
            SetInUse: SetInUse::<Impl, IMPL_OFFSET>,
            InUse: InUse::<Impl, IMPL_OFFSET>,
            ImportRecorder: ImportRecorder::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultisession as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMultisessionRandomWrite_Impl: Sized + super::super::System::Com::IDispatch_Impl + IMultisession_Impl {
    fn WriteUnitSize(&mut self) -> ::windows::core::Result<i32>;
    fn LastWrittenAddress(&mut self) -> ::windows::core::Result<i32>;
    fn TotalSectorsOnMedia(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMultisessionRandomWrite_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultisessionRandomWrite_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultisessionRandomWrite_Vtbl {
        unsafe extern "system" fn WriteUnitSize<Impl: IMultisessionRandomWrite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteUnitSize() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastWrittenAddress<Impl: IMultisessionRandomWrite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastWrittenAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Impl: IMultisessionRandomWrite_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalSectorsOnMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IMultisession_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            WriteUnitSize: WriteUnitSize::<Impl, IMPL_OFFSET>,
            LastWrittenAddress: LastWrittenAddress::<Impl, IMPL_OFFSET>,
            TotalSectorsOnMedia: TotalSectorsOnMedia::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultisessionRandomWrite as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMultisessionSequential_Impl: Sized + super::super::System::Com::IDispatch_Impl + IMultisession_Impl {
    fn IsFirstDataSession(&mut self) -> ::windows::core::Result<i16>;
    fn StartAddressOfPreviousSession(&mut self) -> ::windows::core::Result<i32>;
    fn LastWrittenAddressOfPreviousSession(&mut self) -> ::windows::core::Result<i32>;
    fn NextWritableAddress(&mut self) -> ::windows::core::Result<i32>;
    fn FreeSectorsOnMedia(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMultisessionSequential_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultisessionSequential_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultisessionSequential_Vtbl {
        unsafe extern "system" fn IsFirstDataSession<Impl: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFirstDataSession() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAddressOfPreviousSession<Impl: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartAddressOfPreviousSession() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastWrittenAddressOfPreviousSession<Impl: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastWrittenAddressOfPreviousSession() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextWritableAddress<Impl: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextWritableAddress() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Impl: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FreeSectorsOnMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IMultisession_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            IsFirstDataSession: IsFirstDataSession::<Impl, IMPL_OFFSET>,
            StartAddressOfPreviousSession: StartAddressOfPreviousSession::<Impl, IMPL_OFFSET>,
            LastWrittenAddressOfPreviousSession: LastWrittenAddressOfPreviousSession::<Impl, IMPL_OFFSET>,
            NextWritableAddress: NextWritableAddress::<Impl, IMPL_OFFSET>,
            FreeSectorsOnMedia: FreeSectorsOnMedia::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultisessionSequential as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IMultisessionSequential2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IMultisession_Impl + IMultisessionSequential_Impl {
    fn WriteUnitSize(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMultisessionSequential2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultisessionSequential2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultisessionSequential2_Vtbl {
        unsafe extern "system" fn WriteUnitSize<Impl: IMultisessionSequential2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteUnitSize() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IMultisessionSequential_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), WriteUnitSize: WriteUnitSize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultisessionSequential2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IProgressItem_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn FirstBlock(&mut self) -> ::windows::core::Result<u32>;
    fn LastBlock(&mut self) -> ::windows::core::Result<u32>;
    fn BlockCount(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IProgressItem_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProgressItem_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProgressItem_Vtbl {
        unsafe extern "system" fn Description<Impl: IProgressItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *desc = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstBlock<Impl: IProgressItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, block: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirstBlock() {
                ::core::result::Result::Ok(ok__) => {
                    *block = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastBlock<Impl: IProgressItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, block: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastBlock() {
                ::core::result::Result::Ok(ok__) => {
                    *block = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BlockCount<Impl: IProgressItem_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blocks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BlockCount() {
                ::core::result::Result::Ok(ok__) => {
                    *blocks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Description: Description::<Impl, IMPL_OFFSET>,
            FirstBlock: FirstBlock::<Impl, IMPL_OFFSET>,
            LastBlock: LastBlock::<Impl, IMPL_OFFSET>,
            BlockCount: BlockCount::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProgressItem as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IProgressItems_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&mut self) -> ::windows::core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn Item(&mut self, index: i32) -> ::windows::core::Result<IProgressItem>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn ProgressItemFromBlock(&mut self, block: u32) -> ::windows::core::Result<IProgressItem>;
    fn ProgressItemFromDescription(&mut self, description: super::super::Foundation::BSTR) -> ::windows::core::Result<IProgressItem>;
    fn EnumProgressItems(&mut self) -> ::windows::core::Result<IEnumProgressItems>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IProgressItems_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProgressItems_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProgressItems_Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: IProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *item = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *count = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgressItemFromBlock<Impl: IProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, block: u32, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProgressItemFromBlock(::core::mem::transmute_copy(&block)) {
                ::core::result::Result::Ok(ok__) => {
                    *item = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgressItemFromDescription<Impl: IProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProgressItemFromDescription(::core::mem::transmute_copy(&description)) {
                ::core::result::Result::Ok(ok__) => {
                    *item = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumProgressItems<Impl: IProgressItems_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumProgressItems() {
                ::core::result::Result::Ok(ok__) => {
                    *newenum = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            ProgressItemFromBlock: ProgressItemFromBlock::<Impl, IMPL_OFFSET>,
            ProgressItemFromDescription: ProgressItemFromDescription::<Impl, IMPL_OFFSET>,
            EnumProgressItems: EnumProgressItems::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IProgressItems as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRawCDImageCreator_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn CreateResultImage(&mut self) -> ::windows::core::Result<super::super::System::Com::IStream>;
    fn AddTrack(&mut self, datatype: IMAPI_CD_SECTOR_TYPE, data: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<i32>;
    fn AddSpecialPregap(&mut self, data: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn AddSubcodeRWGenerator(&mut self, subcode: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn SetResultingImageType(&mut self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::core::Result<()>;
    fn ResultingImageType(&mut self) -> ::windows::core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE>;
    fn StartOfLeadout(&mut self) -> ::windows::core::Result<i32>;
    fn SetStartOfLeadoutLimit(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn StartOfLeadoutLimit(&mut self) -> ::windows::core::Result<i32>;
    fn SetDisableGaplessAudio(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn DisableGaplessAudio(&mut self) -> ::windows::core::Result<i16>;
    fn SetMediaCatalogNumber(&mut self, value: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn MediaCatalogNumber(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetStartingTrackNumber(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn StartingTrackNumber(&mut self) -> ::windows::core::Result<i32>;
    fn TrackInfo(&mut self, trackindex: i32) -> ::windows::core::Result<IRawCDImageTrackInfo>;
    fn NumberOfExistingTracks(&mut self) -> ::windows::core::Result<i32>;
    fn LastUsedUserSectorInImage(&mut self) -> ::windows::core::Result<i32>;
    fn ExpectedTableOfContents(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRawCDImageCreator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageCreator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawCDImageCreator_Vtbl {
        unsafe extern "system" fn CreateResultImage<Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateResultImage() {
                ::core::result::Result::Ok(ok__) => {
                    *resultstream = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTrack<Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatype: IMAPI_CD_SECTOR_TYPE, data: ::windows::core::RawPtr, trackindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddTrack(::core::mem::transmute_copy(&datatype), ::core::mem::transmute(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    *trackindex = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSpecialPregap<Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSpecialPregap(::core::mem::transmute(&data)).into()
        }
        unsafe extern "system" fn AddSubcodeRWGenerator<Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subcode: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddSubcodeRWGenerator(::core::mem::transmute(&subcode)).into()
        }
        unsafe extern "system" fn SetResultingImageType<Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetResultingImageType(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn ResultingImageType<Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResultingImageType() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartOfLeadout<Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartOfLeadout() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartOfLeadoutLimit<Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartOfLeadoutLimit(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn StartOfLeadoutLimit<Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartOfLeadoutLimit() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisableGaplessAudio<Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDisableGaplessAudio(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn DisableGaplessAudio<Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableGaplessAudio() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMediaCatalogNumber<Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMediaCatalogNumber(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn MediaCatalogNumber<Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaCatalogNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartingTrackNumber<Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartingTrackNumber(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn StartingTrackNumber<Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartingTrackNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrackInfo<Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trackindex: i32, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrackInfo(::core::mem::transmute_copy(&trackindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfExistingTracks<Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfExistingTracks() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastUsedUserSectorInImage<Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastUsedUserSectorInImage() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpectedTableOfContents<Impl: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpectedTableOfContents() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            CreateResultImage: CreateResultImage::<Impl, IMPL_OFFSET>,
            AddTrack: AddTrack::<Impl, IMPL_OFFSET>,
            AddSpecialPregap: AddSpecialPregap::<Impl, IMPL_OFFSET>,
            AddSubcodeRWGenerator: AddSubcodeRWGenerator::<Impl, IMPL_OFFSET>,
            SetResultingImageType: SetResultingImageType::<Impl, IMPL_OFFSET>,
            ResultingImageType: ResultingImageType::<Impl, IMPL_OFFSET>,
            StartOfLeadout: StartOfLeadout::<Impl, IMPL_OFFSET>,
            SetStartOfLeadoutLimit: SetStartOfLeadoutLimit::<Impl, IMPL_OFFSET>,
            StartOfLeadoutLimit: StartOfLeadoutLimit::<Impl, IMPL_OFFSET>,
            SetDisableGaplessAudio: SetDisableGaplessAudio::<Impl, IMPL_OFFSET>,
            DisableGaplessAudio: DisableGaplessAudio::<Impl, IMPL_OFFSET>,
            SetMediaCatalogNumber: SetMediaCatalogNumber::<Impl, IMPL_OFFSET>,
            MediaCatalogNumber: MediaCatalogNumber::<Impl, IMPL_OFFSET>,
            SetStartingTrackNumber: SetStartingTrackNumber::<Impl, IMPL_OFFSET>,
            StartingTrackNumber: StartingTrackNumber::<Impl, IMPL_OFFSET>,
            TrackInfo: TrackInfo::<Impl, IMPL_OFFSET>,
            NumberOfExistingTracks: NumberOfExistingTracks::<Impl, IMPL_OFFSET>,
            LastUsedUserSectorInImage: LastUsedUserSectorInImage::<Impl, IMPL_OFFSET>,
            ExpectedTableOfContents: ExpectedTableOfContents::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRawCDImageCreator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IRawCDImageTrackInfo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StartingLba(&mut self) -> ::windows::core::Result<i32>;
    fn SectorCount(&mut self) -> ::windows::core::Result<i32>;
    fn TrackNumber(&mut self) -> ::windows::core::Result<i32>;
    fn SectorType(&mut self) -> ::windows::core::Result<IMAPI_CD_SECTOR_TYPE>;
    fn ISRC(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetISRC(&mut self, value: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DigitalAudioCopySetting(&mut self) -> ::windows::core::Result<IMAPI_CD_TRACK_DIGITAL_COPY_SETTING>;
    fn SetDigitalAudioCopySetting(&mut self, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows::core::Result<()>;
    fn AudioHasPreemphasis(&mut self) -> ::windows::core::Result<i16>;
    fn SetAudioHasPreemphasis(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn TrackIndexes(&mut self) -> ::windows::core::Result<*mut super::super::System::Com::SAFEARRAY>;
    fn AddTrackIndex(&mut self, lbaoffset: i32) -> ::windows::core::Result<()>;
    fn ClearTrackIndex(&mut self, lbaoffset: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRawCDImageTrackInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageTrackInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawCDImageTrackInfo_Vtbl {
        unsafe extern "system" fn StartingLba<Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartingLba() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SectorCount<Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SectorCount() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrackNumber<Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrackNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SectorType<Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_CD_SECTOR_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SectorType() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ISRC<Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ISRC() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetISRC<Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetISRC(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn DigitalAudioCopySetting<Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DigitalAudioCopySetting() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDigitalAudioCopySetting<Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDigitalAudioCopySetting(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn AudioHasPreemphasis<Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioHasPreemphasis() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioHasPreemphasis<Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAudioHasPreemphasis(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn TrackIndexes<Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrackIndexes() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTrackIndex<Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbaoffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTrackIndex(::core::mem::transmute_copy(&lbaoffset)).into()
        }
        unsafe extern "system" fn ClearTrackIndex<Impl: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbaoffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ClearTrackIndex(::core::mem::transmute_copy(&lbaoffset)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            StartingLba: StartingLba::<Impl, IMPL_OFFSET>,
            SectorCount: SectorCount::<Impl, IMPL_OFFSET>,
            TrackNumber: TrackNumber::<Impl, IMPL_OFFSET>,
            SectorType: SectorType::<Impl, IMPL_OFFSET>,
            ISRC: ISRC::<Impl, IMPL_OFFSET>,
            SetISRC: SetISRC::<Impl, IMPL_OFFSET>,
            DigitalAudioCopySetting: DigitalAudioCopySetting::<Impl, IMPL_OFFSET>,
            SetDigitalAudioCopySetting: SetDigitalAudioCopySetting::<Impl, IMPL_OFFSET>,
            AudioHasPreemphasis: AudioHasPreemphasis::<Impl, IMPL_OFFSET>,
            SetAudioHasPreemphasis: SetAudioHasPreemphasis::<Impl, IMPL_OFFSET>,
            TrackIndexes: TrackIndexes::<Impl, IMPL_OFFSET>,
            AddTrackIndex: AddTrackIndex::<Impl, IMPL_OFFSET>,
            ClearTrackIndex: ClearTrackIndex::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRawCDImageTrackInfo as ::windows::core::Interface>::IID
    }
}
pub trait IRedbookDiscMaster_Impl: Sized {
    fn GetTotalAudioTracks(&mut self) -> ::windows::core::Result<i32>;
    fn GetTotalAudioBlocks(&mut self) -> ::windows::core::Result<i32>;
    fn GetUsedAudioBlocks(&mut self) -> ::windows::core::Result<i32>;
    fn GetAvailableAudioTrackBlocks(&mut self) -> ::windows::core::Result<i32>;
    fn GetAudioBlockSize(&mut self) -> ::windows::core::Result<i32>;
    fn CreateAudioTrack(&mut self, nblocks: i32) -> ::windows::core::Result<()>;
    fn AddAudioTrackBlocks(&mut self, pby: *const u8, cb: i32) -> ::windows::core::Result<()>;
    fn CloseAudioTrack(&mut self) -> ::windows::core::Result<()>;
}
impl IRedbookDiscMaster_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRedbookDiscMaster_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRedbookDiscMaster_Vtbl {
        unsafe extern "system" fn GetTotalAudioTracks<Impl: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pntracks: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTotalAudioTracks() {
                ::core::result::Result::Ok(ok__) => {
                    *pntracks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTotalAudioBlocks<Impl: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTotalAudioBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    *pnblocks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUsedAudioBlocks<Impl: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUsedAudioBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    *pnblocks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAvailableAudioTrackBlocks<Impl: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAvailableAudioTrackBlocks() {
                ::core::result::Result::Ok(ok__) => {
                    *pnblocks = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioBlockSize<Impl: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblockbytes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioBlockSize() {
                ::core::result::Result::Ok(ok__) => {
                    *pnblockbytes = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAudioTrack<Impl: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nblocks: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CreateAudioTrack(::core::mem::transmute_copy(&nblocks)).into()
        }
        unsafe extern "system" fn AddAudioTrackBlocks<Impl: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pby: *const u8, cb: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAudioTrackBlocks(::core::mem::transmute_copy(&pby), ::core::mem::transmute_copy(&cb)).into()
        }
        unsafe extern "system" fn CloseAudioTrack<Impl: IRedbookDiscMaster_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseAudioTrack().into()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            GetTotalAudioTracks: GetTotalAudioTracks::<Impl, IMPL_OFFSET>,
            GetTotalAudioBlocks: GetTotalAudioBlocks::<Impl, IMPL_OFFSET>,
            GetUsedAudioBlocks: GetUsedAudioBlocks::<Impl, IMPL_OFFSET>,
            GetAvailableAudioTrackBlocks: GetAvailableAudioTrackBlocks::<Impl, IMPL_OFFSET>,
            GetAudioBlockSize: GetAudioBlockSize::<Impl, IMPL_OFFSET>,
            CreateAudioTrack: CreateAudioTrack::<Impl, IMPL_OFFSET>,
            AddAudioTrackBlocks: AddAudioTrackBlocks::<Impl, IMPL_OFFSET>,
            CloseAudioTrack: CloseAudioTrack::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IRedbookDiscMaster as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IStreamConcatenate_Impl: Sized + super::super::System::Com::ISequentialStream_Impl + super::super::System::Com::IStream_Impl {
    fn Initialize(&mut self, stream1: ::core::option::Option<super::super::System::Com::IStream>, stream2: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn Initialize2(&mut self, streams: *const ::core::option::Option<super::super::System::Com::IStream>, streamcount: u32) -> ::windows::core::Result<()>;
    fn Append(&mut self, stream: ::core::option::Option<super::super::System::Com::IStream>) -> ::windows::core::Result<()>;
    fn Append2(&mut self, streams: *const ::core::option::Option<super::super::System::Com::IStream>, streamcount: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IStreamConcatenate_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamConcatenate_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamConcatenate_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream1: ::windows::core::RawPtr, stream2: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute(&stream1), ::core::mem::transmute(&stream2)).into()
        }
        unsafe extern "system" fn Initialize2<Impl: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streams: *const ::windows::core::RawPtr, streamcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize2(::core::mem::transmute_copy(&streams), ::core::mem::transmute_copy(&streamcount)).into()
        }
        unsafe extern "system" fn Append<Impl: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append(::core::mem::transmute(&stream)).into()
        }
        unsafe extern "system" fn Append2<Impl: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streams: *const ::windows::core::RawPtr, streamcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Append2(::core::mem::transmute_copy(&streams), ::core::mem::transmute_copy(&streamcount)).into()
        }
        Self {
            base: super::super::System::Com::IStream_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            Initialize2: Initialize2::<Impl, IMPL_OFFSET>,
            Append: Append::<Impl, IMPL_OFFSET>,
            Append2: Append2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamConcatenate as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IStreamInterleave_Impl: Sized + super::super::System::Com::ISequentialStream_Impl + super::super::System::Com::IStream_Impl {
    fn Initialize(&mut self, streams: *const ::core::option::Option<super::super::System::Com::IStream>, interleavesizes: *const u32, streamcount: u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IStreamInterleave_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamInterleave_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamInterleave_Vtbl {
        unsafe extern "system" fn Initialize<Impl: IStreamInterleave_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streams: *const ::windows::core::RawPtr, interleavesizes: *const u32, streamcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&streams), ::core::mem::transmute_copy(&interleavesizes), ::core::mem::transmute_copy(&streamcount)).into()
        }
        Self { base: super::super::System::Com::IStream_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Initialize: Initialize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamInterleave as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IStreamPseudoRandomBased_Impl: Sized + super::super::System::Com::ISequentialStream_Impl + super::super::System::Com::IStream_Impl {
    fn SetSeed(&mut self, value: u32) -> ::windows::core::Result<()>;
    fn Seed(&mut self) -> ::windows::core::Result<u32>;
    fn SetExtendedSeed(&mut self, values: *const u32, ecount: u32) -> ::windows::core::Result<()>;
    fn ExtendedSeed(&mut self, values: *mut *mut u32, ecount: *mut u32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IStreamPseudoRandomBased_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamPseudoRandomBased_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamPseudoRandomBased_Vtbl {
        unsafe extern "system" fn SetSeed<Impl: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetSeed(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn Seed<Impl: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Seed() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtendedSeed<Impl: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: *const u32, ecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetExtendedSeed(::core::mem::transmute_copy(&values), ::core::mem::transmute_copy(&ecount)).into()
        }
        unsafe extern "system" fn ExtendedSeed<Impl: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: *mut *mut u32, ecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ExtendedSeed(::core::mem::transmute_copy(&values), ::core::mem::transmute_copy(&ecount)).into()
        }
        Self {
            base: super::super::System::Com::IStream_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetSeed: SetSeed::<Impl, IMPL_OFFSET>,
            Seed: Seed::<Impl, IMPL_OFFSET>,
            SetExtendedSeed: SetExtendedSeed::<Impl, IMPL_OFFSET>,
            ExtendedSeed: ExtendedSeed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamPseudoRandomBased as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWriteEngine2_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn WriteSection(&mut self, data: ::core::option::Option<super::super::System::Com::IStream>, startingblockaddress: i32, numberofblocks: i32) -> ::windows::core::Result<()>;
    fn CancelWrite(&mut self) -> ::windows::core::Result<()>;
    fn SetRecorder(&mut self, value: ::core::option::Option<IDiscRecorder2Ex>) -> ::windows::core::Result<()>;
    fn Recorder(&mut self) -> ::windows::core::Result<IDiscRecorder2Ex>;
    fn SetUseStreamingWrite12(&mut self, value: i16) -> ::windows::core::Result<()>;
    fn UseStreamingWrite12(&mut self) -> ::windows::core::Result<i16>;
    fn SetStartingSectorsPerSecond(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn StartingSectorsPerSecond(&mut self) -> ::windows::core::Result<i32>;
    fn SetEndingSectorsPerSecond(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn EndingSectorsPerSecond(&mut self) -> ::windows::core::Result<i32>;
    fn SetBytesPerSector(&mut self, value: i32) -> ::windows::core::Result<()>;
    fn BytesPerSector(&mut self) -> ::windows::core::Result<i32>;
    fn WriteInProgress(&mut self) -> ::windows::core::Result<i16>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWriteEngine2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWriteEngine2_Vtbl {
        unsafe extern "system" fn WriteSection<Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, startingblockaddress: i32, numberofblocks: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).WriteSection(::core::mem::transmute(&data), ::core::mem::transmute_copy(&startingblockaddress), ::core::mem::transmute_copy(&numberofblocks)).into()
        }
        unsafe extern "system" fn CancelWrite<Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CancelWrite().into()
        }
        unsafe extern "system" fn SetRecorder<Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRecorder(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Recorder<Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recorder() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseStreamingWrite12<Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetUseStreamingWrite12(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn UseStreamingWrite12<Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseStreamingWrite12() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartingSectorsPerSecond<Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetStartingSectorsPerSecond(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn StartingSectorsPerSecond<Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartingSectorsPerSecond() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndingSectorsPerSecond<Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetEndingSectorsPerSecond(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn EndingSectorsPerSecond<Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndingSectorsPerSecond() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBytesPerSector<Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBytesPerSector(::core::mem::transmute_copy(&value)).into()
        }
        unsafe extern "system" fn BytesPerSector<Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BytesPerSector() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteInProgress<Impl: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteInProgress() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            WriteSection: WriteSection::<Impl, IMPL_OFFSET>,
            CancelWrite: CancelWrite::<Impl, IMPL_OFFSET>,
            SetRecorder: SetRecorder::<Impl, IMPL_OFFSET>,
            Recorder: Recorder::<Impl, IMPL_OFFSET>,
            SetUseStreamingWrite12: SetUseStreamingWrite12::<Impl, IMPL_OFFSET>,
            UseStreamingWrite12: UseStreamingWrite12::<Impl, IMPL_OFFSET>,
            SetStartingSectorsPerSecond: SetStartingSectorsPerSecond::<Impl, IMPL_OFFSET>,
            StartingSectorsPerSecond: StartingSectorsPerSecond::<Impl, IMPL_OFFSET>,
            SetEndingSectorsPerSecond: SetEndingSectorsPerSecond::<Impl, IMPL_OFFSET>,
            EndingSectorsPerSecond: EndingSectorsPerSecond::<Impl, IMPL_OFFSET>,
            SetBytesPerSector: SetBytesPerSector::<Impl, IMPL_OFFSET>,
            BytesPerSector: BytesPerSector::<Impl, IMPL_OFFSET>,
            WriteInProgress: WriteInProgress::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWriteEngine2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWriteEngine2EventArgs_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StartLba(&mut self) -> ::windows::core::Result<i32>;
    fn SectorCount(&mut self) -> ::windows::core::Result<i32>;
    fn LastReadLba(&mut self) -> ::windows::core::Result<i32>;
    fn LastWrittenLba(&mut self) -> ::windows::core::Result<i32>;
    fn TotalSystemBuffer(&mut self) -> ::windows::core::Result<i32>;
    fn UsedSystemBuffer(&mut self) -> ::windows::core::Result<i32>;
    fn FreeSystemBuffer(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWriteEngine2EventArgs_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2EventArgs_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWriteEngine2EventArgs_Vtbl {
        unsafe extern "system" fn StartLba<Impl: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartLba() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SectorCount<Impl: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SectorCount() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastReadLba<Impl: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastReadLba() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastWrittenLba<Impl: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastWrittenLba() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalSystemBuffer<Impl: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalSystemBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsedSystemBuffer<Impl: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsedSystemBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeSystemBuffer<Impl: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FreeSystemBuffer() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            StartLba: StartLba::<Impl, IMPL_OFFSET>,
            SectorCount: SectorCount::<Impl, IMPL_OFFSET>,
            LastReadLba: LastReadLba::<Impl, IMPL_OFFSET>,
            LastWrittenLba: LastWrittenLba::<Impl, IMPL_OFFSET>,
            TotalSystemBuffer: TotalSystemBuffer::<Impl, IMPL_OFFSET>,
            UsedSystemBuffer: UsedSystemBuffer::<Impl, IMPL_OFFSET>,
            FreeSystemBuffer: FreeSystemBuffer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWriteEngine2EventArgs as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IWriteSpeedDescriptor_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn MediaType(&mut self) -> ::windows::core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn RotationTypeIsPureCAV(&mut self) -> ::windows::core::Result<i16>;
    fn WriteSpeed(&mut self) -> ::windows::core::Result<i32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWriteSpeedDescriptor_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWriteSpeedDescriptor_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWriteSpeedDescriptor_Vtbl {
        unsafe extern "system" fn MediaType<Impl: IWriteSpeedDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaType() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotationTypeIsPureCAV<Impl: IWriteSpeedDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationTypeIsPureCAV() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteSpeed<Impl: IWriteSpeedDescriptor_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteSpeed() {
                ::core::result::Result::Ok(ok__) => {
                    *value = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            MediaType: MediaType::<Impl, IMPL_OFFSET>,
            RotationTypeIsPureCAV: RotationTypeIsPureCAV::<Impl, IMPL_OFFSET>,
            WriteSpeed: WriteSpeed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWriteSpeedDescriptor as ::windows::core::Interface>::IID
    }
}
