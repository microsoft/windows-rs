#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DDiscFormat2DataEventsImpl: Sized + IDispatchImpl {
    fn Update();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DDiscFormat2DataEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DDiscFormat2DataEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DDiscFormat2DataEventsVtbl {
        unsafe extern "system" fn Update<Impl: DDiscFormat2DataEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, progress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Update: Update::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DDiscFormat2DataEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DDiscFormat2EraseEventsImpl: Sized + IDispatchImpl {
    fn Update();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DDiscFormat2EraseEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DDiscFormat2EraseEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DDiscFormat2EraseEventsVtbl {
        unsafe extern "system" fn Update<Impl: DDiscFormat2EraseEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, elapsedseconds: i32, estimatedtotalseconds: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Update: Update::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DDiscFormat2EraseEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DDiscFormat2RawCDEventsImpl: Sized + IDispatchImpl {
    fn Update();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DDiscFormat2RawCDEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DDiscFormat2RawCDEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DDiscFormat2RawCDEventsVtbl {
        unsafe extern "system" fn Update<Impl: DDiscFormat2RawCDEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, progress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Update: Update::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DDiscFormat2RawCDEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DDiscFormat2TrackAtOnceEventsImpl: Sized + IDispatchImpl {
    fn Update();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DDiscFormat2TrackAtOnceEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DDiscFormat2TrackAtOnceEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DDiscFormat2TrackAtOnceEventsVtbl {
        unsafe extern "system" fn Update<Impl: DDiscFormat2TrackAtOnceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, progress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Update: Update::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DDiscFormat2TrackAtOnceEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DDiscMaster2EventsImpl: Sized + IDispatchImpl {
    fn NotifyDeviceAdded();
    fn NotifyDeviceRemoved();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DDiscMaster2EventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DDiscMaster2EventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DDiscMaster2EventsVtbl {
        unsafe extern "system" fn NotifyDeviceAdded<Impl: DDiscMaster2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, uniqueid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyDeviceRemoved<Impl: DDiscMaster2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, uniqueid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            NotifyDeviceAdded: NotifyDeviceAdded::<Impl, IMPL_OFFSET>,
            NotifyDeviceRemoved: NotifyDeviceRemoved::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DDiscMaster2Events as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DFileSystemImageEventsImpl: Sized + IDispatchImpl {
    fn Update();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DFileSystemImageEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DFileSystemImageEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DFileSystemImageEventsVtbl {
        unsafe extern "system" fn Update<Impl: DFileSystemImageEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, currentfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, copiedsectors: i32, totalsectors: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Update: Update::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DFileSystemImageEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DFileSystemImageImportEventsImpl: Sized + IDispatchImpl {
    fn UpdateImport();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DFileSystemImageImportEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DFileSystemImageImportEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DFileSystemImageImportEventsVtbl {
        unsafe extern "system" fn UpdateImport<Impl: DFileSystemImageImportEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, filesystem: FsiFileSystems, currentitem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, importeddirectoryitems: i32, totaldirectoryitems: i32, importedfileitems: i32, totalfileitems: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), UpdateImport: UpdateImport::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DFileSystemImageImportEvents as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait DWriteEngine2EventsImpl: Sized + IDispatchImpl {
    fn Update();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl DWriteEngine2EventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DWriteEngine2EventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> DWriteEngine2EventsVtbl {
        unsafe extern "system" fn Update<Impl: DWriteEngine2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, progress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Update: Update::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<DWriteEngine2Events as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IBlockRangeImpl: Sized + IDispatchImpl {
    fn StartLba();
    fn EndLba();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IBlockRangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBlockRangeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBlockRangeVtbl {
        unsafe extern "system" fn StartLba<Impl: IBlockRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndLba<Impl: IBlockRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            StartLba: StartLba::<Impl, IMPL_OFFSET>,
            EndLba: EndLba::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBlockRange as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IBlockRangeListImpl: Sized + IDispatchImpl {
    fn BlockRanges();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IBlockRangeListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBlockRangeListImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBlockRangeListVtbl {
        unsafe extern "system" fn BlockRanges<Impl: IBlockRangeListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), BlockRanges: BlockRanges::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBlockRangeList as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IBootOptionsImpl: Sized + IDispatchImpl {
    fn BootImage();
    fn Manufacturer();
    fn SetManufacturer();
    fn PlatformId();
    fn SetPlatformId();
    fn Emulation();
    fn SetEmulation();
    fn ImageSize();
    fn AssignBootImage();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IBootOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBootOptionsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBootOptionsVtbl {
        unsafe extern "system" fn BootImage<Impl: IBootOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Manufacturer<Impl: IBootOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetManufacturer<Impl: IBootOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PlatformId<Impl: IBootOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut PlatformId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPlatformId<Impl: IBootOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: PlatformId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Emulation<Impl: IBootOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut EmulationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEmulation<Impl: IBootOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: EmulationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ImageSize<Impl: IBootOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AssignBootImage<Impl: IBootOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IBurnVerificationImpl: Sized {
    fn SetBurnVerificationLevel();
    fn BurnVerificationLevel();
}
impl IBurnVerificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBurnVerificationImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IBurnVerificationVtbl {
        unsafe extern "system" fn SetBurnVerificationLevel<Impl: IBurnVerificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BurnVerificationLevel<Impl: IBurnVerificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IDiscFormat2Impl: Sized + IDispatchImpl {
    fn IsRecorderSupported();
    fn IsCurrentMediaSupported();
    fn MediaPhysicallyBlank();
    fn MediaHeuristicallyBlank();
    fn SupportedMediaTypes();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscFormat2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscFormat2Vtbl {
        unsafe extern "system" fn IsRecorderSupported<Impl: IDiscFormat2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recorder: ::windows::core::RawPtr, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsCurrentMediaSupported<Impl: IDiscFormat2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recorder: ::windows::core::RawPtr, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MediaPhysicallyBlank<Impl: IDiscFormat2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MediaHeuristicallyBlank<Impl: IDiscFormat2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SupportedMediaTypes<Impl: IDiscFormat2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IDiscFormat2DataImpl: Sized + IDispatchImpl + IDiscFormat2Impl {
    fn SetRecorder();
    fn Recorder();
    fn SetBufferUnderrunFreeDisabled();
    fn BufferUnderrunFreeDisabled();
    fn SetPostgapAlreadyInImage();
    fn PostgapAlreadyInImage();
    fn CurrentMediaStatus();
    fn WriteProtectStatus();
    fn TotalSectorsOnMedia();
    fn FreeSectorsOnMedia();
    fn NextWritableAddress();
    fn StartAddressOfPreviousSession();
    fn LastWrittenAddressOfPreviousSession();
    fn SetForceMediaToBeClosed();
    fn ForceMediaToBeClosed();
    fn SetDisableConsumerDvdCompatibilityMode();
    fn DisableConsumerDvdCompatibilityMode();
    fn CurrentPhysicalMediaType();
    fn SetClientName();
    fn ClientName();
    fn RequestedWriteSpeed();
    fn RequestedRotationTypeIsPureCAV();
    fn CurrentWriteSpeed();
    fn CurrentRotationTypeIsPureCAV();
    fn SupportedWriteSpeeds();
    fn SupportedWriteSpeedDescriptors();
    fn SetForceOverwrite();
    fn ForceOverwrite();
    fn MultisessionInterfaces();
    fn Write();
    fn CancelWrite();
    fn SetWriteSpeed();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscFormat2DataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2DataImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscFormat2DataVtbl {
        unsafe extern "system" fn SetRecorder<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Recorder<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPostgapAlreadyInImage<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn PostgapAlreadyInImage<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentMediaStatus<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_DATA_MEDIA_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteProtectStatus<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_WRITE_PROTECT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NextWritableAddress<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartAddressOfPreviousSession<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastWrittenAddressOfPreviousSession<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetForceMediaToBeClosed<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ForceMediaToBeClosed<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisableConsumerDvdCompatibilityMode<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisableConsumerDvdCompatibilityMode<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClientName<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClientName<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestedWriteSpeed<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentWriteSpeed<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetForceOverwrite<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ForceOverwrite<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MultisessionInterfaces<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Write<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelWrite<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWriteSpeed<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDiscFormat2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IDiscFormat2DataEventArgsImpl: Sized + IDispatchImpl + IWriteEngine2EventArgsImpl {
    fn ElapsedTime();
    fn RemainingTime();
    fn TotalTime();
    fn CurrentAction();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscFormat2DataEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2DataEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscFormat2DataEventArgsVtbl {
        unsafe extern "system" fn ElapsedTime<Impl: IDiscFormat2DataEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemainingTime<Impl: IDiscFormat2DataEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TotalTime<Impl: IDiscFormat2DataEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentAction<Impl: IDiscFormat2DataEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_DATA_WRITE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IWriteEngine2EventArgsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IDiscFormat2EraseImpl: Sized + IDispatchImpl + IDiscFormat2Impl {
    fn SetRecorder();
    fn Recorder();
    fn SetFullErase();
    fn FullErase();
    fn CurrentPhysicalMediaType();
    fn SetClientName();
    fn ClientName();
    fn EraseMedia();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscFormat2EraseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2EraseImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscFormat2EraseVtbl {
        unsafe extern "system" fn SetRecorder<Impl: IDiscFormat2EraseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Recorder<Impl: IDiscFormat2EraseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFullErase<Impl: IDiscFormat2EraseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FullErase<Impl: IDiscFormat2EraseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Impl: IDiscFormat2EraseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClientName<Impl: IDiscFormat2EraseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClientName<Impl: IDiscFormat2EraseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EraseMedia<Impl: IDiscFormat2EraseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDiscFormat2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IDiscFormat2RawCDImpl: Sized + IDispatchImpl + IDiscFormat2Impl {
    fn PrepareMedia();
    fn WriteMedia();
    fn WriteMedia2();
    fn CancelWrite();
    fn ReleaseMedia();
    fn SetWriteSpeed();
    fn SetRecorder();
    fn Recorder();
    fn SetBufferUnderrunFreeDisabled();
    fn BufferUnderrunFreeDisabled();
    fn StartOfNextSession();
    fn LastPossibleStartOfLeadout();
    fn CurrentPhysicalMediaType();
    fn SupportedSectorTypes();
    fn SetRequestedSectorType();
    fn RequestedSectorType();
    fn SetClientName();
    fn ClientName();
    fn RequestedWriteSpeed();
    fn RequestedRotationTypeIsPureCAV();
    fn CurrentWriteSpeed();
    fn CurrentRotationTypeIsPureCAV();
    fn SupportedWriteSpeeds();
    fn SupportedWriteSpeedDescriptors();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscFormat2RawCDVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCDImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscFormat2RawCDVtbl {
        unsafe extern "system" fn PrepareMedia<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteMedia<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteMedia2<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, streamleadinsectors: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelWrite<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseMedia<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWriteSpeed<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRecorder<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Recorder<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartOfNextSession<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastPossibleStartOfLeadout<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SupportedSectorTypes<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRequestedSectorType<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestedSectorType<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClientName<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClientName<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestedWriteSpeed<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentWriteSpeed<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDiscFormat2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IDiscFormat2RawCDEventArgsImpl: Sized + IDispatchImpl + IWriteEngine2EventArgsImpl {
    fn CurrentAction();
    fn ElapsedTime();
    fn RemainingTime();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscFormat2RawCDEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCDEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscFormat2RawCDEventArgsVtbl {
        unsafe extern "system" fn CurrentAction<Impl: IDiscFormat2RawCDEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_WRITE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ElapsedTime<Impl: IDiscFormat2RawCDEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemainingTime<Impl: IDiscFormat2RawCDEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IWriteEngine2EventArgsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IDiscFormat2TrackAtOnceImpl: Sized + IDispatchImpl + IDiscFormat2Impl {
    fn PrepareMedia();
    fn AddAudioTrack();
    fn CancelAddTrack();
    fn ReleaseMedia();
    fn SetWriteSpeed();
    fn SetRecorder();
    fn Recorder();
    fn SetBufferUnderrunFreeDisabled();
    fn BufferUnderrunFreeDisabled();
    fn NumberOfExistingTracks();
    fn TotalSectorsOnMedia();
    fn FreeSectorsOnMedia();
    fn UsedSectorsOnMedia();
    fn SetDoNotFinalizeMedia();
    fn DoNotFinalizeMedia();
    fn ExpectedTableOfContents();
    fn CurrentPhysicalMediaType();
    fn SetClientName();
    fn ClientName();
    fn RequestedWriteSpeed();
    fn RequestedRotationTypeIsPureCAV();
    fn CurrentWriteSpeed();
    fn CurrentRotationTypeIsPureCAV();
    fn SupportedWriteSpeeds();
    fn SupportedWriteSpeedDescriptors();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscFormat2TrackAtOnceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnceImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscFormat2TrackAtOnceVtbl {
        unsafe extern "system" fn PrepareMedia<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddAudioTrack<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelAddTrack<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseMedia<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWriteSpeed<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRecorder<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Recorder<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NumberOfExistingTracks<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UsedSectorsOnMedia<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDoNotFinalizeMedia<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DoNotFinalizeMedia<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExpectedTableOfContents<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetClientName<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClientName<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestedWriteSpeed<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentWriteSpeed<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDiscFormat2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IDiscFormat2TrackAtOnceEventArgsImpl: Sized + IDispatchImpl + IWriteEngine2EventArgsImpl {
    fn CurrentTrackNumber();
    fn CurrentAction();
    fn ElapsedTime();
    fn RemainingTime();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscFormat2TrackAtOnceEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnceEventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscFormat2TrackAtOnceEventArgsVtbl {
        unsafe extern "system" fn CurrentTrackNumber<Impl: IDiscFormat2TrackAtOnceEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentAction<Impl: IDiscFormat2TrackAtOnceEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_TAO_WRITE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ElapsedTime<Impl: IDiscFormat2TrackAtOnceEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemainingTime<Impl: IDiscFormat2TrackAtOnceEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IWriteEngine2EventArgsVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IDiscMasterImpl: Sized {
    fn Open();
    fn EnumDiscMasterFormats();
    fn GetActiveDiscMasterFormat();
    fn SetActiveDiscMasterFormat();
    fn EnumDiscRecorders();
    fn GetActiveDiscRecorder();
    fn SetActiveDiscRecorder();
    fn ClearFormatContent();
    fn ProgressAdvise();
    fn ProgressUnadvise();
    fn RecordDisc();
    fn Close();
}
impl IDiscMasterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMasterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscMasterVtbl {
        unsafe extern "system" fn Open<Impl: IDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumDiscMasterFormats<Impl: IDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetActiveDiscMasterFormat<Impl: IDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetActiveDiscMasterFormat<Impl: IDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumDiscRecorders<Impl: IDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetActiveDiscRecorder<Impl: IDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprecorder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetActiveDiscRecorder<Impl: IDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precorder: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearFormatContent<Impl: IDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProgressAdvise<Impl: IDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevents: ::windows::core::RawPtr, pvcookie: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProgressUnadvise<Impl: IDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcookie: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RecordDisc<Impl: IDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsimulate: u8, bejectafterburn: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IDiscMaster2Impl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn IsSupportedEnvironment();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscMaster2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMaster2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscMaster2Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: IDiscMaster2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IDiscMaster2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IDiscMaster2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsSupportedEnvironment<Impl: IDiscMaster2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IDiscMasterProgressEventsImpl: Sized {
    fn QueryCancel();
    fn NotifyPnPActivity();
    fn NotifyAddProgress();
    fn NotifyBlockProgress();
    fn NotifyTrackProgress();
    fn NotifyPreparingBurn();
    fn NotifyClosingDisc();
    fn NotifyBurnComplete();
    fn NotifyEraseComplete();
}
impl IDiscMasterProgressEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMasterProgressEventsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscMasterProgressEventsVtbl {
        unsafe extern "system" fn QueryCancel<Impl: IDiscMasterProgressEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcancel: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyPnPActivity<Impl: IDiscMasterProgressEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyAddProgress<Impl: IDiscMasterProgressEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncompletedsteps: i32, ntotalsteps: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyBlockProgress<Impl: IDiscMasterProgressEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncompleted: i32, ntotal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyTrackProgress<Impl: IDiscMasterProgressEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncurrenttrack: i32, ntotaltracks: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyPreparingBurn<Impl: IDiscMasterProgressEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nestimatedseconds: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyClosingDisc<Impl: IDiscMasterProgressEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nestimatedseconds: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyBurnComplete<Impl: IDiscMasterProgressEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NotifyEraseComplete<Impl: IDiscMasterProgressEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IDiscRecorderImpl: Sized {
    fn Init();
    fn GetRecorderGUID();
    fn GetRecorderType();
    fn GetDisplayNames();
    fn GetBasePnPID();
    fn GetPath();
    fn GetRecorderProperties();
    fn SetRecorderProperties();
    fn GetRecorderState();
    fn OpenExclusive();
    fn QueryMediaType();
    fn QueryMediaInfo();
    fn Eject();
    fn Erase();
    fn Close();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
impl IDiscRecorderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscRecorderVtbl {
        unsafe extern "system" fn Init<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbyuniqueid: *const u8, nulidsize: u32, nuldrivenumber: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRecorderGUID<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbyuniqueid: *mut u8, ulbuffersize: u32, pulreturnsizerequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRecorderType<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ftypecode: *mut RECORDER_TYPES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDisplayNames<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrvendorid: *mut super::super::Foundation::BSTR, pbstrproductid: *mut super::super::Foundation::BSTR, pbstrrevision: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetBasePnPID<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbasepnpid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetPath<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRecorderProperties<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRecorderProperties<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetRecorderState<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puldevstateflags: *mut DISC_RECORDER_STATE_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn OpenExclusive<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryMediaType<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmediatype: *mut MEDIA_TYPES, fmediaflags: *mut MEDIA_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn QueryMediaInfo<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsessions: *mut u8, pblasttrack: *mut u8, ulstartaddress: *mut u32, ulnextwritable: *mut u32, ulfreeblocks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Eject<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Erase<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfullerase: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Close<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IDiscRecorder2Impl: Sized + IDispatchImpl {
    fn EjectMedia();
    fn CloseTray();
    fn AcquireExclusiveAccess();
    fn ReleaseExclusiveAccess();
    fn DisableMcn();
    fn EnableMcn();
    fn InitializeDiscRecorder();
    fn ActiveDiscRecorder();
    fn VendorId();
    fn ProductId();
    fn ProductRevision();
    fn VolumeName();
    fn VolumePathNames();
    fn DeviceCanLoadMedia();
    fn LegacyDeviceNumber();
    fn SupportedFeaturePages();
    fn CurrentFeaturePages();
    fn SupportedProfiles();
    fn CurrentProfiles();
    fn SupportedModePages();
    fn ExclusiveAccessOwner();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IDiscRecorder2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscRecorder2Vtbl {
        unsafe extern "system" fn EjectMedia<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CloseTray<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AcquireExclusiveAccess<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, force: i16, __midl__idiscrecorder20000: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReleaseExclusiveAccess<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisableMcn<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnableMcn<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InitializeDiscRecorder<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recorderuniqueid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ActiveDiscRecorder<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VendorId<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProductId<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProductRevision<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VolumeName<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VolumePathNames<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DeviceCanLoadMedia<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LegacyDeviceNumber<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, legacydevicenumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SupportedFeaturePages<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentFeaturePages<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SupportedProfiles<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CurrentProfiles<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SupportedModePages<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExclusiveAccessOwner<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IDiscRecorder2ExImpl: Sized {
    fn SendCommandNoData();
    fn SendCommandSendDataToDevice();
    fn SendCommandGetDataFromDevice();
    fn ReadDvdStructure();
    fn SendDvdStructure();
    fn GetAdapterDescriptor();
    fn GetDeviceDescriptor();
    fn GetDiscInformation();
    fn GetTrackInformation();
    fn GetFeaturePage();
    fn GetModePage();
    fn SetModePage();
    fn GetSupportedFeaturePages();
    fn GetSupportedProfiles();
    fn GetSupportedModePages();
    fn GetByteAlignmentMask();
    fn GetMaximumNonPageAlignedTransferSize();
    fn GetMaximumPageAlignedTransferSize();
}
#[cfg(feature = "Win32_Foundation")]
impl IDiscRecorder2ExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2ExImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDiscRecorder2ExVtbl {
        unsafe extern "system" fn SendCommandNoData<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendCommandSendDataToDevice<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *const u8, buffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendCommandGetDataFromDevice<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *mut u8, buffersize: u32, bufferfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ReadDvdStructure<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: u32, address: u32, layer: u32, agid: u32, data: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SendDvdStructure<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: u32, data: *const u8, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAdapterDescriptor<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDeviceDescriptor<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDiscInformation<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTrackInformation<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: u32, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetFeaturePage<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetModePage<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetModePage<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: *const u8, bytesize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSupportedFeaturePages<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSupportedProfiles<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentonly: super::super::Foundation::BOOLEAN, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetSupportedModePages<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetByteAlignmentMask<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaximumNonPageAlignedTransferSize<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetMaximumPageAlignedTransferSize<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IEnumDiscMasterFormatsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumDiscMasterFormatsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDiscMasterFormatsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumDiscMasterFormatsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumDiscMasterFormatsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cformats: u32, lpiidformatid: *mut ::windows::core::GUID, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumDiscMasterFormatsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cformats: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumDiscMasterFormatsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumDiscMasterFormatsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IEnumDiscRecordersImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl IEnumDiscRecordersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDiscRecordersImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumDiscRecordersVtbl {
        unsafe extern "system" fn Next<Impl: IEnumDiscRecordersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crecorders: u32, pprecorder: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumDiscRecordersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crecorders: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumDiscRecordersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumDiscRecordersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IEnumFsiItemsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumFsiItemsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumFsiItemsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumFsiItemsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumFsiItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumFsiItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumFsiItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumFsiItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IEnumProgressItemsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
#[cfg(feature = "Win32_System_Com")]
impl IEnumProgressItemsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumProgressItemsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IEnumProgressItemsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumProgressItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Skip<Impl: IEnumProgressItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Reset<Impl: IEnumProgressItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Clone<Impl: IEnumProgressItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IFileSystemImageImpl: Sized + IDispatchImpl {
    fn Root();
    fn SessionStartBlock();
    fn SetSessionStartBlock();
    fn FreeMediaBlocks();
    fn SetFreeMediaBlocks();
    fn SetMaxMediaBlocksFromDevice();
    fn UsedBlocks();
    fn VolumeName();
    fn SetVolumeName();
    fn ImportedVolumeName();
    fn BootImageOptions();
    fn SetBootImageOptions();
    fn FileCount();
    fn DirectoryCount();
    fn WorkingDirectory();
    fn SetWorkingDirectory();
    fn ChangePoint();
    fn StrictFileSystemCompliance();
    fn SetStrictFileSystemCompliance();
    fn UseRestrictedCharacterSet();
    fn SetUseRestrictedCharacterSet();
    fn FileSystemsToCreate();
    fn SetFileSystemsToCreate();
    fn FileSystemsSupported();
    fn SetUDFRevision();
    fn UDFRevision();
    fn UDFRevisionsSupported();
    fn ChooseImageDefaults();
    fn ChooseImageDefaultsForMediaType();
    fn SetISO9660InterchangeLevel();
    fn ISO9660InterchangeLevel();
    fn ISO9660InterchangeLevelsSupported();
    fn CreateResultImage();
    fn Exists();
    fn CalculateDiscIdentifier();
    fn IdentifyFileSystemsOnDisc();
    fn GetDefaultFileSystemForImport();
    fn ImportFileSystem();
    fn ImportSpecificFileSystem();
    fn RollbackToChangePoint();
    fn LockInChangePoint();
    fn CreateDirectoryItem();
    fn CreateFileItem();
    fn VolumeNameUDF();
    fn VolumeNameJoliet();
    fn VolumeNameISO9660();
    fn StageFiles();
    fn SetStageFiles();
    fn MultisessionInterfaces();
    fn SetMultisessionInterfaces();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFileSystemImageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImageImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileSystemImageVtbl {
        unsafe extern "system" fn Root<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SessionStartBlock<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetSessionStartBlock<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FreeMediaBlocks<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFreeMediaBlocks<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMaxMediaBlocksFromDevice<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discrecorder: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UsedBlocks<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VolumeName<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetVolumeName<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ImportedVolumeName<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BootImageOptions<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBootImageOptions<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FileCount<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DirectoryCount<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WorkingDirectory<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetWorkingDirectory<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ChangePoint<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StrictFileSystemCompliance<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStrictFileSystemCompliance<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UseRestrictedCharacterSet<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUseRestrictedCharacterSet<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FileSystemsToCreate<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetFileSystemsToCreate<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FileSystemsSupported<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUDFRevision<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UDFRevision<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UDFRevisionsSupported<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ChooseImageDefaults<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discrecorder: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ChooseImageDefaultsForMediaType<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetISO9660InterchangeLevel<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ISO9660InterchangeLevel<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ISO9660InterchangeLevelsSupported<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateResultImage<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Exists<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fullpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, itemtype: *mut FsiItemType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CalculateDiscIdentifier<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discidentifier: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IdentifyFileSystemsOnDisc<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discrecorder: ::windows::core::RawPtr, filesystems: *mut FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDefaultFileSystemForImport<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesystems: FsiFileSystems, importdefault: *mut FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ImportFileSystem<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, importedfilesystem: *mut FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ImportSpecificFileSystem<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesystemtouse: FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RollbackToChangePoint<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changepoint: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LockInChangePoint<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateDirectoryItem<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateFileItem<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VolumeNameUDF<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VolumeNameJoliet<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn VolumeNameISO9660<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StageFiles<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStageFiles<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MultisessionInterfaces<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMultisessionInterfaces<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IFileSystemImage2Impl: Sized + IDispatchImpl + IFileSystemImageImpl {
    fn BootImageOptionsArray();
    fn SetBootImageOptionsArray();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFileSystemImage2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileSystemImage2Vtbl {
        unsafe extern "system" fn BootImageOptionsArray<Impl: IFileSystemImage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBootImageOptionsArray<Impl: IFileSystemImage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IFileSystemImageVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BootImageOptionsArray: BootImageOptionsArray::<Impl, IMPL_OFFSET>,
            SetBootImageOptionsArray: SetBootImageOptionsArray::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSystemImage2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFileSystemImage3Impl: Sized + IDispatchImpl + IFileSystemImageImpl + IFileSystemImage2Impl {
    fn CreateRedundantUdfMetadataFiles();
    fn SetCreateRedundantUdfMetadataFiles();
    fn ProbeSpecificFileSystem();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFileSystemImage3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage3Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileSystemImage3Vtbl {
        unsafe extern "system" fn CreateRedundantUdfMetadataFiles<Impl: IFileSystemImage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCreateRedundantUdfMetadataFiles<Impl: IFileSystemImage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProbeSpecificFileSystem<Impl: IFileSystemImage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesystemtoprobe: FsiFileSystems, isappendable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IFileSystemImage2Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IFileSystemImageResultImpl: Sized + IDispatchImpl {
    fn ImageStream();
    fn ProgressItems();
    fn TotalBlocks();
    fn BlockSize();
    fn DiscId();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFileSystemImageResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImageResultImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileSystemImageResultVtbl {
        unsafe extern "system" fn ImageStream<Impl: IFileSystemImageResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProgressItems<Impl: IFileSystemImageResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TotalBlocks<Impl: IFileSystemImageResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BlockSize<Impl: IFileSystemImageResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DiscId<Impl: IFileSystemImageResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IFileSystemImageResult2Impl: Sized + IDispatchImpl + IFileSystemImageResultImpl {
    fn ModifiedBlocks();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFileSystemImageResult2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImageResult2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFileSystemImageResult2Vtbl {
        unsafe extern "system" fn ModifiedBlocks<Impl: IFileSystemImageResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IFileSystemImageResultVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), ModifiedBlocks: ModifiedBlocks::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFileSystemImageResult2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsiDirectoryItemImpl: Sized + IDispatchImpl + IFsiItemImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn EnumFsiItems();
    fn AddDirectory();
    fn AddFile();
    fn AddTree();
    fn Add();
    fn Remove();
    fn RemoveTree();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsiDirectoryItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsiDirectoryItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsiDirectoryItemVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFsiDirectoryItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IFsiDirectoryItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IFsiDirectoryItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumFsiItems<Impl: IFsiDirectoryItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddDirectory<Impl: IFsiDirectoryItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddFile<Impl: IFsiDirectoryItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filedata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddTree<Impl: IFsiDirectoryItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcedirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, includebasedirectory: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Add<Impl: IFsiDirectoryItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Remove<Impl: IFsiDirectoryItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveTree<Impl: IFsiDirectoryItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IFsiItemVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IFsiDirectoryItem2Impl: Sized + IDispatchImpl + IFsiItemImpl + IFsiDirectoryItemImpl {
    fn AddTreeWithNamedStreams();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsiDirectoryItem2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsiDirectoryItem2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsiDirectoryItem2Vtbl {
        unsafe extern "system" fn AddTreeWithNamedStreams<Impl: IFsiDirectoryItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcedirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, includebasedirectory: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IFsiDirectoryItemVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddTreeWithNamedStreams: AddTreeWithNamedStreams::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFsiDirectoryItem2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IFsiFileItemImpl: Sized + IDispatchImpl + IFsiItemImpl {
    fn DataSize();
    fn DataSize32BitLow();
    fn DataSize32BitHigh();
    fn Data();
    fn SetData();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsiFileItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsiFileItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsiFileItemVtbl {
        unsafe extern "system" fn DataSize<Impl: IFsiFileItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DataSize32BitLow<Impl: IFsiFileItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DataSize32BitHigh<Impl: IFsiFileItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Data<Impl: IFsiFileItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetData<Impl: IFsiFileItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IFsiItemVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IFsiFileItem2Impl: Sized + IDispatchImpl + IFsiItemImpl + IFsiFileItemImpl {
    fn FsiNamedStreams();
    fn IsNamedStream();
    fn AddStream();
    fn RemoveStream();
    fn IsRealTime();
    fn SetIsRealTime();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsiFileItem2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsiFileItem2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsiFileItem2Vtbl {
        unsafe extern "system" fn FsiNamedStreams<Impl: IFsiFileItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsNamedStream<Impl: IFsiFileItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddStream<Impl: IFsiFileItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, streamdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RemoveStream<Impl: IFsiFileItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsRealTime<Impl: IFsiFileItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIsRealTime<Impl: IFsiFileItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IFsiFileItemVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IFsiItemImpl: Sized + IDispatchImpl {
    fn Name();
    fn FullPath();
    fn CreationTime();
    fn SetCreationTime();
    fn LastAccessedTime();
    fn SetLastAccessedTime();
    fn LastModifiedTime();
    fn SetLastModifiedTime();
    fn IsHidden();
    fn SetIsHidden();
    fn FileSystemName();
    fn FileSystemPath();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsiItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsiItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsiItemVtbl {
        unsafe extern "system" fn Name<Impl: IFsiItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FullPath<Impl: IFsiItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreationTime<Impl: IFsiItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetCreationTime<Impl: IFsiItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastAccessedTime<Impl: IFsiItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLastAccessedTime<Impl: IFsiItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastModifiedTime<Impl: IFsiItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetLastModifiedTime<Impl: IFsiItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn IsHidden<Impl: IFsiItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetIsHidden<Impl: IFsiItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FileSystemName<Impl: IFsiItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesystem: FsiFileSystems, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FileSystemPath<Impl: IFsiItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesystem: FsiFileSystems, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IFsiNamedStreamsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn EnumNamedStreams();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IFsiNamedStreamsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsiNamedStreamsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFsiNamedStreamsVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFsiNamedStreamsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IFsiNamedStreamsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IFsiNamedStreamsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumNamedStreams<Impl: IFsiNamedStreamsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IIsoImageManagerImpl: Sized + IDispatchImpl {
    fn Path();
    fn Stream();
    fn SetPath();
    fn SetStream();
    fn Validate();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IIsoImageManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIsoImageManagerImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IIsoImageManagerVtbl {
        unsafe extern "system" fn Path<Impl: IIsoImageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Stream<Impl: IIsoImageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetPath<Impl: IIsoImageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStream<Impl: IIsoImageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Validate<Impl: IIsoImageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IJolietDiscMasterImpl: Sized {
    fn GetTotalDataBlocks();
    fn GetUsedDataBlocks();
    fn GetDataBlockSize();
    fn AddData();
    fn GetJolietProperties();
    fn SetJolietProperties();
}
#[cfg(feature = "Win32_System_Com_StructuredStorage")]
impl IJolietDiscMasterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJolietDiscMasterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IJolietDiscMasterVtbl {
        unsafe extern "system" fn GetTotalDataBlocks<Impl: IJolietDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUsedDataBlocks<Impl: IJolietDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetDataBlockSize<Impl: IJolietDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblockbytes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddData<Impl: IJolietDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstorage: ::windows::core::RawPtr, lfileoverwrite: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetJolietProperties<Impl: IJolietDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetJolietProperties<Impl: IJolietDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IMultisessionImpl: Sized + IDispatchImpl {
    fn IsSupportedOnCurrentMediaState();
    fn SetInUse();
    fn InUse();
    fn ImportRecorder();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMultisessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultisessionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultisessionVtbl {
        unsafe extern "system" fn IsSupportedOnCurrentMediaState<Impl: IMultisessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetInUse<Impl: IMultisessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn InUse<Impl: IMultisessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ImportRecorder<Impl: IMultisessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMultisessionRandomWriteImpl: Sized + IDispatchImpl + IMultisessionImpl {
    fn WriteUnitSize();
    fn LastWrittenAddress();
    fn TotalSectorsOnMedia();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMultisessionRandomWriteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultisessionRandomWriteImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultisessionRandomWriteVtbl {
        unsafe extern "system" fn WriteUnitSize<Impl: IMultisessionRandomWriteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastWrittenAddress<Impl: IMultisessionRandomWriteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Impl: IMultisessionRandomWriteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMultisessionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMultisessionSequentialImpl: Sized + IDispatchImpl + IMultisessionImpl {
    fn IsFirstDataSession();
    fn StartAddressOfPreviousSession();
    fn LastWrittenAddressOfPreviousSession();
    fn NextWritableAddress();
    fn FreeSectorsOnMedia();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMultisessionSequentialVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultisessionSequentialImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultisessionSequentialVtbl {
        unsafe extern "system" fn IsFirstDataSession<Impl: IMultisessionSequentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartAddressOfPreviousSession<Impl: IMultisessionSequentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastWrittenAddressOfPreviousSession<Impl: IMultisessionSequentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NextWritableAddress<Impl: IMultisessionSequentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Impl: IMultisessionSequentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IMultisessionVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IMultisessionSequential2Impl: Sized + IDispatchImpl + IMultisessionImpl + IMultisessionSequentialImpl {
    fn WriteUnitSize();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IMultisessionSequential2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultisessionSequential2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMultisessionSequential2Vtbl {
        unsafe extern "system" fn WriteUnitSize<Impl: IMultisessionSequential2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IMultisessionSequentialVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), WriteUnitSize: WriteUnitSize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMultisessionSequential2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IProgressItemImpl: Sized + IDispatchImpl {
    fn Description();
    fn FirstBlock();
    fn LastBlock();
    fn BlockCount();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IProgressItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProgressItemImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProgressItemVtbl {
        unsafe extern "system" fn Description<Impl: IProgressItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FirstBlock<Impl: IProgressItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, block: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastBlock<Impl: IProgressItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, block: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BlockCount<Impl: IProgressItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blocks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IProgressItemsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn ProgressItemFromBlock();
    fn ProgressItemFromDescription();
    fn EnumProgressItems();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IProgressItemsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProgressItemsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IProgressItemsVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IProgressItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Item<Impl: IProgressItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Count<Impl: IProgressItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProgressItemFromBlock<Impl: IProgressItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, block: u32, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ProgressItemFromDescription<Impl: IProgressItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EnumProgressItems<Impl: IProgressItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRawCDImageCreatorImpl: Sized + IDispatchImpl {
    fn CreateResultImage();
    fn AddTrack();
    fn AddSpecialPregap();
    fn AddSubcodeRWGenerator();
    fn SetResultingImageType();
    fn ResultingImageType();
    fn StartOfLeadout();
    fn SetStartOfLeadoutLimit();
    fn StartOfLeadoutLimit();
    fn SetDisableGaplessAudio();
    fn DisableGaplessAudio();
    fn SetMediaCatalogNumber();
    fn MediaCatalogNumber();
    fn SetStartingTrackNumber();
    fn StartingTrackNumber();
    fn TrackInfo();
    fn NumberOfExistingTracks();
    fn LastUsedUserSectorInImage();
    fn ExpectedTableOfContents();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRawCDImageCreatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageCreatorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawCDImageCreatorVtbl {
        unsafe extern "system" fn CreateResultImage<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddTrack<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatype: IMAPI_CD_SECTOR_TYPE, data: ::windows::core::RawPtr, trackindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddSpecialPregap<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddSubcodeRWGenerator<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subcode: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetResultingImageType<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ResultingImageType<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartOfLeadout<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStartOfLeadoutLimit<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartOfLeadoutLimit<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDisableGaplessAudio<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DisableGaplessAudio<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetMediaCatalogNumber<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn MediaCatalogNumber<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStartingTrackNumber<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartingTrackNumber<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TrackInfo<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trackindex: i32, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn NumberOfExistingTracks<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastUsedUserSectorInImage<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExpectedTableOfContents<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRawCDImageTrackInfoImpl: Sized + IDispatchImpl {
    fn StartingLba();
    fn SectorCount();
    fn TrackNumber();
    fn SectorType();
    fn ISRC();
    fn SetISRC();
    fn DigitalAudioCopySetting();
    fn SetDigitalAudioCopySetting();
    fn AudioHasPreemphasis();
    fn SetAudioHasPreemphasis();
    fn TrackIndexes();
    fn AddTrackIndex();
    fn ClearTrackIndex();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IRawCDImageTrackInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageTrackInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRawCDImageTrackInfoVtbl {
        unsafe extern "system" fn StartingLba<Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SectorCount<Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TrackNumber<Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SectorType<Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_CD_SECTOR_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ISRC<Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetISRC<Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn DigitalAudioCopySetting<Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetDigitalAudioCopySetting<Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AudioHasPreemphasis<Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetAudioHasPreemphasis<Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TrackIndexes<Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddTrackIndex<Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbaoffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ClearTrackIndex<Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbaoffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IRedbookDiscMasterImpl: Sized {
    fn GetTotalAudioTracks();
    fn GetTotalAudioBlocks();
    fn GetUsedAudioBlocks();
    fn GetAvailableAudioTrackBlocks();
    fn GetAudioBlockSize();
    fn CreateAudioTrack();
    fn AddAudioTrackBlocks();
    fn CloseAudioTrack();
}
impl IRedbookDiscMasterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRedbookDiscMasterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IRedbookDiscMasterVtbl {
        unsafe extern "system" fn GetTotalAudioTracks<Impl: IRedbookDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pntracks: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetTotalAudioBlocks<Impl: IRedbookDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetUsedAudioBlocks<Impl: IRedbookDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAvailableAudioTrackBlocks<Impl: IRedbookDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn GetAudioBlockSize<Impl: IRedbookDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblockbytes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CreateAudioTrack<Impl: IRedbookDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nblocks: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn AddAudioTrackBlocks<Impl: IRedbookDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pby: *const u8, cb: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CloseAudioTrack<Impl: IRedbookDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
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
pub trait IStreamConcatenateImpl: Sized + ISequentialStreamImpl + IStreamImpl {
    fn Initialize();
    fn Initialize2();
    fn Append();
    fn Append2();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IStreamConcatenateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamConcatenateImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamConcatenateVtbl {
        unsafe extern "system" fn Initialize<Impl: IStreamConcatenateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream1: ::windows::core::RawPtr, stream2: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Initialize2<Impl: IStreamConcatenateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streams: *const ::windows::core::RawPtr, streamcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Append<Impl: IStreamConcatenateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Append2<Impl: IStreamConcatenateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streams: *const ::windows::core::RawPtr, streamcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IStreamVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IStreamInterleaveImpl: Sized + ISequentialStreamImpl + IStreamImpl {
    fn Initialize();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IStreamInterleaveVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamInterleaveImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamInterleaveVtbl {
        unsafe extern "system" fn Initialize<Impl: IStreamInterleaveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streams: *const ::windows::core::RawPtr, interleavesizes: *const u32, streamcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self { base: IStreamVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), Initialize: Initialize::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IStreamInterleave as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
pub trait IStreamPseudoRandomBasedImpl: Sized + ISequentialStreamImpl + IStreamImpl {
    fn SetSeed();
    fn Seed();
    fn SetExtendedSeed();
    fn ExtendedSeed();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
impl IStreamPseudoRandomBasedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamPseudoRandomBasedImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IStreamPseudoRandomBasedVtbl {
        unsafe extern "system" fn SetSeed<Impl: IStreamPseudoRandomBasedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Seed<Impl: IStreamPseudoRandomBasedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetExtendedSeed<Impl: IStreamPseudoRandomBasedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: *const u32, ecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn ExtendedSeed<Impl: IStreamPseudoRandomBasedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: *mut *mut u32, ecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IStreamVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWriteEngine2Impl: Sized + IDispatchImpl {
    fn WriteSection();
    fn CancelWrite();
    fn SetRecorder();
    fn Recorder();
    fn SetUseStreamingWrite12();
    fn UseStreamingWrite12();
    fn SetStartingSectorsPerSecond();
    fn StartingSectorsPerSecond();
    fn SetEndingSectorsPerSecond();
    fn EndingSectorsPerSecond();
    fn SetBytesPerSector();
    fn BytesPerSector();
    fn WriteInProgress();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWriteEngine2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWriteEngine2Vtbl {
        unsafe extern "system" fn WriteSection<Impl: IWriteEngine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, startingblockaddress: i32, numberofblocks: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn CancelWrite<Impl: IWriteEngine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetRecorder<Impl: IWriteEngine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn Recorder<Impl: IWriteEngine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetUseStreamingWrite12<Impl: IWriteEngine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UseStreamingWrite12<Impl: IWriteEngine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetStartingSectorsPerSecond<Impl: IWriteEngine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn StartingSectorsPerSecond<Impl: IWriteEngine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetEndingSectorsPerSecond<Impl: IWriteEngine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn EndingSectorsPerSecond<Impl: IWriteEngine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SetBytesPerSector<Impl: IWriteEngine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BytesPerSector<Impl: IWriteEngine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteInProgress<Impl: IWriteEngine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWriteEngine2EventArgsImpl: Sized + IDispatchImpl {
    fn StartLba();
    fn SectorCount();
    fn LastReadLba();
    fn LastWrittenLba();
    fn TotalSystemBuffer();
    fn UsedSystemBuffer();
    fn FreeSystemBuffer();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWriteEngine2EventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2EventArgsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWriteEngine2EventArgsVtbl {
        unsafe extern "system" fn StartLba<Impl: IWriteEngine2EventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SectorCount<Impl: IWriteEngine2EventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastReadLba<Impl: IWriteEngine2EventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn LastWrittenLba<Impl: IWriteEngine2EventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn TotalSystemBuffer<Impl: IWriteEngine2EventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn UsedSystemBuffer<Impl: IWriteEngine2EventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn FreeSystemBuffer<Impl: IWriteEngine2EventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
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
pub trait IWriteSpeedDescriptorImpl: Sized + IDispatchImpl {
    fn MediaType();
    fn RotationTypeIsPureCAV();
    fn WriteSpeed();
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IWriteSpeedDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWriteSpeedDescriptorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IWriteSpeedDescriptorVtbl {
        unsafe extern "system" fn MediaType<Impl: IWriteSpeedDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn RotationTypeIsPureCAV<Impl: IWriteSpeedDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn WriteSpeed<Impl: IWriteSpeedDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: IDispatchVtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            MediaType: MediaType::<Impl, IMPL_OFFSET>,
            RotationTypeIsPureCAV: RotationTypeIsPureCAV::<Impl, IMPL_OFFSET>,
            WriteSpeed: WriteSpeed::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IWriteSpeedDescriptor as ::windows::core::Interface>::IID
    }
}
