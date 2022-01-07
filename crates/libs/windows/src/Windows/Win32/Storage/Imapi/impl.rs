#[cfg(feature = "Win32_System_Com")]
pub trait DDiscFormat2DataEventsImpl: Sized + IDispatchImpl {
    fn Update();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for DDiscFormat2DataEvents {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.DDiscFormat2DataEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl DDiscFormat2DataEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DDiscFormat2DataEventsImpl, const OFFSET: isize>() -> DDiscFormat2DataEventsVtbl {
        unsafe extern "system" fn Update<Impl: DDiscFormat2DataEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, progress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Update(&*(&object as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), &*(&progress as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<DDiscFormat2DataEvents>, ::windows::core::GetTrustLevel, Update::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait DDiscFormat2EraseEventsImpl: Sized + IDispatchImpl {
    fn Update();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for DDiscFormat2EraseEvents {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.DDiscFormat2EraseEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl DDiscFormat2EraseEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DDiscFormat2EraseEventsImpl, const OFFSET: isize>() -> DDiscFormat2EraseEventsVtbl {
        unsafe extern "system" fn Update<Impl: DDiscFormat2EraseEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, elapsedseconds: i32, estimatedtotalseconds: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Update(&*(&object as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), elapsedseconds, estimatedtotalseconds) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<DDiscFormat2EraseEvents>, ::windows::core::GetTrustLevel, Update::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait DDiscFormat2RawCDEventsImpl: Sized + IDispatchImpl {
    fn Update();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for DDiscFormat2RawCDEvents {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.DDiscFormat2RawCDEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl DDiscFormat2RawCDEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DDiscFormat2RawCDEventsImpl, const OFFSET: isize>() -> DDiscFormat2RawCDEventsVtbl {
        unsafe extern "system" fn Update<Impl: DDiscFormat2RawCDEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, progress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Update(&*(&object as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), &*(&progress as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<DDiscFormat2RawCDEvents>, ::windows::core::GetTrustLevel, Update::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait DDiscFormat2TrackAtOnceEventsImpl: Sized + IDispatchImpl {
    fn Update();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for DDiscFormat2TrackAtOnceEvents {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.DDiscFormat2TrackAtOnceEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl DDiscFormat2TrackAtOnceEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DDiscFormat2TrackAtOnceEventsImpl, const OFFSET: isize>() -> DDiscFormat2TrackAtOnceEventsVtbl {
        unsafe extern "system" fn Update<Impl: DDiscFormat2TrackAtOnceEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, progress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Update(&*(&object as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), &*(&progress as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<DDiscFormat2TrackAtOnceEvents>, ::windows::core::GetTrustLevel, Update::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait DDiscMaster2EventsImpl: Sized + IDispatchImpl {
    fn NotifyDeviceAdded();
    fn NotifyDeviceRemoved();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for DDiscMaster2Events {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.DDiscMaster2Events";
}
#[cfg(feature = "Win32_System_Com")]
impl DDiscMaster2EventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DDiscMaster2EventsImpl, const OFFSET: isize>() -> DDiscMaster2EventsVtbl {
        unsafe extern "system" fn NotifyDeviceAdded<Impl: DDiscMaster2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, uniqueid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyDeviceAdded(&*(&object as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), &*(&uniqueid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyDeviceRemoved<Impl: DDiscMaster2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, uniqueid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyDeviceRemoved(&*(&object as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), &*(&uniqueid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<DDiscMaster2Events>, ::windows::core::GetTrustLevel, NotifyDeviceAdded::<Impl, OFFSET>, NotifyDeviceRemoved::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait DFileSystemImageEventsImpl: Sized + IDispatchImpl {
    fn Update();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for DFileSystemImageEvents {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.DFileSystemImageEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl DFileSystemImageEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DFileSystemImageEventsImpl, const OFFSET: isize>() -> DFileSystemImageEventsVtbl {
        unsafe extern "system" fn Update<Impl: DFileSystemImageEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, currentfile: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, copiedsectors: i32, totalsectors: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Update(&*(&object as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), &*(&currentfile as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), copiedsectors, totalsectors) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<DFileSystemImageEvents>, ::windows::core::GetTrustLevel, Update::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait DFileSystemImageImportEventsImpl: Sized + IDispatchImpl {
    fn UpdateImport();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for DFileSystemImageImportEvents {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.DFileSystemImageImportEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl DFileSystemImageImportEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DFileSystemImageImportEventsImpl, const OFFSET: isize>() -> DFileSystemImageImportEventsVtbl {
        unsafe extern "system" fn UpdateImport<Impl: DFileSystemImageImportEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, filesystem: FsiFileSystems, currentitem: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, importeddirectoryitems: i32, totaldirectoryitems: i32, importedfileitems: i32, totalfileitems: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateImport(&*(&object as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), filesystem, &*(&currentitem as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), importeddirectoryitems, totaldirectoryitems, importedfileitems, totalfileitems) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<DFileSystemImageImportEvents>, ::windows::core::GetTrustLevel, UpdateImport::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait DWriteEngine2EventsImpl: Sized + IDispatchImpl {
    fn Update();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for DWriteEngine2Events {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.DWriteEngine2Events";
}
#[cfg(feature = "Win32_System_Com")]
impl DWriteEngine2EventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: DWriteEngine2EventsImpl, const OFFSET: isize>() -> DWriteEngine2EventsVtbl {
        unsafe extern "system" fn Update<Impl: DWriteEngine2EventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, object: ::windows::core::RawPtr, progress: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Update(&*(&object as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), &*(&progress as *const <super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<DWriteEngine2Events>, ::windows::core::GetTrustLevel, Update::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBlockRangeImpl: Sized + IDispatchImpl {
    fn StartLba();
    fn EndLba();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IBlockRange {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IBlockRange";
}
#[cfg(feature = "Win32_System_Com")]
impl IBlockRangeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBlockRangeImpl, const OFFSET: isize>() -> IBlockRangeVtbl {
        unsafe extern "system" fn StartLba<Impl: IBlockRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartLba(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndLba<Impl: IBlockRangeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndLba(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBlockRange>, ::windows::core::GetTrustLevel, StartLba::<Impl, OFFSET>, EndLba::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IBlockRangeListImpl: Sized + IDispatchImpl {
    fn BlockRanges();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IBlockRangeList {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IBlockRangeList";
}
#[cfg(feature = "Win32_System_Com")]
impl IBlockRangeListVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBlockRangeListImpl, const OFFSET: isize>() -> IBlockRangeListVtbl {
        unsafe extern "system" fn BlockRanges<Impl: IBlockRangeListImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BlockRanges(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBlockRangeList>, ::windows::core::GetTrustLevel, BlockRanges::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IBootOptions {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IBootOptions";
}
#[cfg(feature = "Win32_System_Com")]
impl IBootOptionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBootOptionsImpl, const OFFSET: isize>() -> IBootOptionsVtbl {
        unsafe extern "system" fn BootImage<Impl: IBootOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BootImage(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Manufacturer<Impl: IBootOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Manufacturer(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetManufacturer<Impl: IBootOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetManufacturer(&*(&newval as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PlatformId<Impl: IBootOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut PlatformId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PlatformId(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPlatformId<Impl: IBootOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: PlatformId) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPlatformId(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Emulation<Impl: IBootOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut EmulationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Emulation(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEmulation<Impl: IBootOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: EmulationType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEmulation(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImageSize<Impl: IBootOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageSize(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AssignBootImage<Impl: IBootOptionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AssignBootImage(&*(&newval as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IBootOptions>,
            ::windows::core::GetTrustLevel,
            BootImage::<Impl, OFFSET>,
            Manufacturer::<Impl, OFFSET>,
            SetManufacturer::<Impl, OFFSET>,
            PlatformId::<Impl, OFFSET>,
            SetPlatformId::<Impl, OFFSET>,
            Emulation::<Impl, OFFSET>,
            SetEmulation::<Impl, OFFSET>,
            ImageSize::<Impl, OFFSET>,
            AssignBootImage::<Impl, OFFSET>,
        )
    }
}
pub trait IBurnVerificationImpl: Sized {
    fn SetBurnVerificationLevel();
    fn BurnVerificationLevel();
}
impl ::windows::core::RuntimeName for IBurnVerification {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IBurnVerification";
}
impl IBurnVerificationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBurnVerificationImpl, const OFFSET: isize>() -> IBurnVerificationVtbl {
        unsafe extern "system" fn SetBurnVerificationLevel<Impl: IBurnVerificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBurnVerificationLevel(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BurnVerificationLevel<Impl: IBurnVerificationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_BURN_VERIFICATION_LEVEL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BurnVerificationLevel(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IBurnVerification>, ::windows::core::GetTrustLevel, SetBurnVerificationLevel::<Impl, OFFSET>, BurnVerificationLevel::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDiscFormat2Impl: Sized + IDispatchImpl {
    fn IsRecorderSupported();
    fn IsCurrentMediaSupported();
    fn MediaPhysicallyBlank();
    fn MediaHeuristicallyBlank();
    fn SupportedMediaTypes();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDiscFormat2 {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IDiscFormat2";
}
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2Impl, const OFFSET: isize>() -> IDiscFormat2Vtbl {
        unsafe extern "system" fn IsRecorderSupported<Impl: IDiscFormat2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recorder: ::windows::core::RawPtr, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRecorderSupported(&*(&recorder as *const <IDiscRecorder2 as ::windows::core::Abi>::Abi as *const <IDiscRecorder2 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCurrentMediaSupported<Impl: IDiscFormat2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recorder: ::windows::core::RawPtr, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCurrentMediaSupported(&*(&recorder as *const <IDiscRecorder2 as ::windows::core::Abi>::Abi as *const <IDiscRecorder2 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaPhysicallyBlank<Impl: IDiscFormat2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaPhysicallyBlank(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaHeuristicallyBlank<Impl: IDiscFormat2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaHeuristicallyBlank(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedMediaTypes<Impl: IDiscFormat2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedMediaTypes(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDiscFormat2>, ::windows::core::GetTrustLevel, IsRecorderSupported::<Impl, OFFSET>, IsCurrentMediaSupported::<Impl, OFFSET>, MediaPhysicallyBlank::<Impl, OFFSET>, MediaHeuristicallyBlank::<Impl, OFFSET>, SupportedMediaTypes::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDiscFormat2DataImpl: Sized + IDiscFormat2Impl + IDispatchImpl {
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDiscFormat2Data {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IDiscFormat2Data";
}
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2DataVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2DataImpl, const OFFSET: isize>() -> IDiscFormat2DataVtbl {
        unsafe extern "system" fn SetRecorder<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRecorder(&*(&value as *const <IDiscRecorder2 as ::windows::core::Abi>::Abi as *const <IDiscRecorder2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recorder<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recorder(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBufferUnderrunFreeDisabled(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferUnderrunFreeDisabled(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPostgapAlreadyInImage<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPostgapAlreadyInImage(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PostgapAlreadyInImage<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PostgapAlreadyInImage(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentMediaStatus<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_DATA_MEDIA_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentMediaStatus(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteProtectStatus<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_WRITE_PROTECT_STATE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteProtectStatus(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalSectorsOnMedia(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FreeSectorsOnMedia(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextWritableAddress<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextWritableAddress(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAddressOfPreviousSession<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartAddressOfPreviousSession(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastWrittenAddressOfPreviousSession<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastWrittenAddressOfPreviousSession(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceMediaToBeClosed<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetForceMediaToBeClosed(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForceMediaToBeClosed<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForceMediaToBeClosed(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisableConsumerDvdCompatibilityMode<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDisableConsumerDvdCompatibilityMode(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableConsumerDvdCompatibilityMode<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableConsumerDvdCompatibilityMode(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPhysicalMediaType(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientName<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetClientName(&*(&value as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientName<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientName(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedWriteSpeed<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedWriteSpeed(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedRotationTypeIsPureCAV(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentWriteSpeed<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentWriteSpeed(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentRotationTypeIsPureCAV(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedWriteSpeeds(::core::mem::transmute_copy(&supportedspeeds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedWriteSpeedDescriptors(::core::mem::transmute_copy(&supportedspeeddescriptors)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetForceOverwrite<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetForceOverwrite(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ForceOverwrite<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ForceOverwrite(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MultisessionInterfaces<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MultisessionInterfaces(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Write<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Write(&*(&data as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelWrite<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelWrite() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWriteSpeed<Impl: IDiscFormat2DataImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWriteSpeed(requestedsectorspersecond, rotationtypeispurecav) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDiscFormat2Data>,
            ::windows::core::GetTrustLevel,
            SetRecorder::<Impl, OFFSET>,
            Recorder::<Impl, OFFSET>,
            SetBufferUnderrunFreeDisabled::<Impl, OFFSET>,
            BufferUnderrunFreeDisabled::<Impl, OFFSET>,
            SetPostgapAlreadyInImage::<Impl, OFFSET>,
            PostgapAlreadyInImage::<Impl, OFFSET>,
            CurrentMediaStatus::<Impl, OFFSET>,
            WriteProtectStatus::<Impl, OFFSET>,
            TotalSectorsOnMedia::<Impl, OFFSET>,
            FreeSectorsOnMedia::<Impl, OFFSET>,
            NextWritableAddress::<Impl, OFFSET>,
            StartAddressOfPreviousSession::<Impl, OFFSET>,
            LastWrittenAddressOfPreviousSession::<Impl, OFFSET>,
            SetForceMediaToBeClosed::<Impl, OFFSET>,
            ForceMediaToBeClosed::<Impl, OFFSET>,
            SetDisableConsumerDvdCompatibilityMode::<Impl, OFFSET>,
            DisableConsumerDvdCompatibilityMode::<Impl, OFFSET>,
            CurrentPhysicalMediaType::<Impl, OFFSET>,
            SetClientName::<Impl, OFFSET>,
            ClientName::<Impl, OFFSET>,
            RequestedWriteSpeed::<Impl, OFFSET>,
            RequestedRotationTypeIsPureCAV::<Impl, OFFSET>,
            CurrentWriteSpeed::<Impl, OFFSET>,
            CurrentRotationTypeIsPureCAV::<Impl, OFFSET>,
            SupportedWriteSpeeds::<Impl, OFFSET>,
            SupportedWriteSpeedDescriptors::<Impl, OFFSET>,
            SetForceOverwrite::<Impl, OFFSET>,
            ForceOverwrite::<Impl, OFFSET>,
            MultisessionInterfaces::<Impl, OFFSET>,
            Write::<Impl, OFFSET>,
            CancelWrite::<Impl, OFFSET>,
            SetWriteSpeed::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDiscFormat2DataEventArgsImpl: Sized + IWriteEngine2EventArgsImpl + IDispatchImpl {
    fn ElapsedTime();
    fn RemainingTime();
    fn TotalTime();
    fn CurrentAction();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDiscFormat2DataEventArgs {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IDiscFormat2DataEventArgs";
}
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2DataEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2DataEventArgsImpl, const OFFSET: isize>() -> IDiscFormat2DataEventArgsVtbl {
        unsafe extern "system" fn ElapsedTime<Impl: IDiscFormat2DataEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElapsedTime(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemainingTime<Impl: IDiscFormat2DataEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemainingTime(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalTime<Impl: IDiscFormat2DataEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalTime(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAction<Impl: IDiscFormat2DataEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_DATA_WRITE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAction(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDiscFormat2DataEventArgs>, ::windows::core::GetTrustLevel, ElapsedTime::<Impl, OFFSET>, RemainingTime::<Impl, OFFSET>, TotalTime::<Impl, OFFSET>, CurrentAction::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDiscFormat2EraseImpl: Sized + IDiscFormat2Impl + IDispatchImpl {
    fn SetRecorder();
    fn Recorder();
    fn SetFullErase();
    fn FullErase();
    fn CurrentPhysicalMediaType();
    fn SetClientName();
    fn ClientName();
    fn EraseMedia();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDiscFormat2Erase {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IDiscFormat2Erase";
}
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2EraseVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2EraseImpl, const OFFSET: isize>() -> IDiscFormat2EraseVtbl {
        unsafe extern "system" fn SetRecorder<Impl: IDiscFormat2EraseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRecorder(&*(&value as *const <IDiscRecorder2 as ::windows::core::Abi>::Abi as *const <IDiscRecorder2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recorder<Impl: IDiscFormat2EraseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recorder(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFullErase<Impl: IDiscFormat2EraseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFullErase(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FullErase<Impl: IDiscFormat2EraseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FullErase(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Impl: IDiscFormat2EraseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPhysicalMediaType(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientName<Impl: IDiscFormat2EraseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetClientName(&*(&value as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientName<Impl: IDiscFormat2EraseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientName(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EraseMedia<Impl: IDiscFormat2EraseImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EraseMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDiscFormat2Erase>,
            ::windows::core::GetTrustLevel,
            SetRecorder::<Impl, OFFSET>,
            Recorder::<Impl, OFFSET>,
            SetFullErase::<Impl, OFFSET>,
            FullErase::<Impl, OFFSET>,
            CurrentPhysicalMediaType::<Impl, OFFSET>,
            SetClientName::<Impl, OFFSET>,
            ClientName::<Impl, OFFSET>,
            EraseMedia::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDiscFormat2RawCDImpl: Sized + IDiscFormat2Impl + IDispatchImpl {
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDiscFormat2RawCD {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IDiscFormat2RawCD";
}
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2RawCDVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>() -> IDiscFormat2RawCDVtbl {
        unsafe extern "system" fn PrepareMedia<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrepareMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteMedia<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteMedia(&*(&data as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteMedia2<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, streamleadinsectors: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteMedia2(&*(&data as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), streamleadinsectors) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelWrite<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelWrite() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseMedia<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWriteSpeed<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWriteSpeed(requestedsectorspersecond, rotationtypeispurecav) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecorder<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRecorder(&*(&value as *const <IDiscRecorder2 as ::windows::core::Abi>::Abi as *const <IDiscRecorder2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recorder<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recorder(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBufferUnderrunFreeDisabled(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferUnderrunFreeDisabled(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartOfNextSession<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartOfNextSession(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastPossibleStartOfLeadout<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastPossibleStartOfLeadout(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPhysicalMediaType(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedSectorTypes<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedSectorTypes(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestedSectorType<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRequestedSectorType(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedSectorType<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedSectorType(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientName<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetClientName(&*(&value as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientName<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientName(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedWriteSpeed<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedWriteSpeed(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedRotationTypeIsPureCAV(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentWriteSpeed<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentWriteSpeed(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentRotationTypeIsPureCAV(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedWriteSpeeds(::core::mem::transmute_copy(&supportedspeeds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Impl: IDiscFormat2RawCDImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedWriteSpeedDescriptors(::core::mem::transmute_copy(&supportedspeeddescriptors)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDiscFormat2RawCD>,
            ::windows::core::GetTrustLevel,
            PrepareMedia::<Impl, OFFSET>,
            WriteMedia::<Impl, OFFSET>,
            WriteMedia2::<Impl, OFFSET>,
            CancelWrite::<Impl, OFFSET>,
            ReleaseMedia::<Impl, OFFSET>,
            SetWriteSpeed::<Impl, OFFSET>,
            SetRecorder::<Impl, OFFSET>,
            Recorder::<Impl, OFFSET>,
            SetBufferUnderrunFreeDisabled::<Impl, OFFSET>,
            BufferUnderrunFreeDisabled::<Impl, OFFSET>,
            StartOfNextSession::<Impl, OFFSET>,
            LastPossibleStartOfLeadout::<Impl, OFFSET>,
            CurrentPhysicalMediaType::<Impl, OFFSET>,
            SupportedSectorTypes::<Impl, OFFSET>,
            SetRequestedSectorType::<Impl, OFFSET>,
            RequestedSectorType::<Impl, OFFSET>,
            SetClientName::<Impl, OFFSET>,
            ClientName::<Impl, OFFSET>,
            RequestedWriteSpeed::<Impl, OFFSET>,
            RequestedRotationTypeIsPureCAV::<Impl, OFFSET>,
            CurrentWriteSpeed::<Impl, OFFSET>,
            CurrentRotationTypeIsPureCAV::<Impl, OFFSET>,
            SupportedWriteSpeeds::<Impl, OFFSET>,
            SupportedWriteSpeedDescriptors::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDiscFormat2RawCDEventArgsImpl: Sized + IWriteEngine2EventArgsImpl + IDispatchImpl {
    fn CurrentAction();
    fn ElapsedTime();
    fn RemainingTime();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDiscFormat2RawCDEventArgs {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IDiscFormat2RawCDEventArgs";
}
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2RawCDEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2RawCDEventArgsImpl, const OFFSET: isize>() -> IDiscFormat2RawCDEventArgsVtbl {
        unsafe extern "system" fn CurrentAction<Impl: IDiscFormat2RawCDEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_WRITE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAction(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElapsedTime<Impl: IDiscFormat2RawCDEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElapsedTime(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemainingTime<Impl: IDiscFormat2RawCDEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemainingTime(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDiscFormat2RawCDEventArgs>, ::windows::core::GetTrustLevel, CurrentAction::<Impl, OFFSET>, ElapsedTime::<Impl, OFFSET>, RemainingTime::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDiscFormat2TrackAtOnceImpl: Sized + IDiscFormat2Impl + IDispatchImpl {
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDiscFormat2TrackAtOnce {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IDiscFormat2TrackAtOnce";
}
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2TrackAtOnceVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>() -> IDiscFormat2TrackAtOnceVtbl {
        unsafe extern "system" fn PrepareMedia<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PrepareMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAudioTrack<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddAudioTrack(&*(&data as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelAddTrack<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelAddTrack() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseMedia<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWriteSpeed<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWriteSpeed(requestedsectorspersecond, rotationtypeispurecav) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecorder<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRecorder(&*(&value as *const <IDiscRecorder2 as ::windows::core::Abi>::Abi as *const <IDiscRecorder2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recorder<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recorder(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBufferUnderrunFreeDisabled(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BufferUnderrunFreeDisabled(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfExistingTracks<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfExistingTracks(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalSectorsOnMedia(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FreeSectorsOnMedia(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsedSectorsOnMedia<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsedSectorsOnMedia(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDoNotFinalizeMedia<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDoNotFinalizeMedia(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DoNotFinalizeMedia<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DoNotFinalizeMedia(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpectedTableOfContents<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpectedTableOfContents(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentPhysicalMediaType(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetClientName<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetClientName(&*(&value as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClientName<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClientName(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedWriteSpeed<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedWriteSpeed(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RequestedRotationTypeIsPureCAV(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentWriteSpeed<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentWriteSpeed(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentRotationTypeIsPureCAV(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeds: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedWriteSpeeds(::core::mem::transmute_copy(&supportedspeeds)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Impl: IDiscFormat2TrackAtOnceImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedWriteSpeedDescriptors(::core::mem::transmute_copy(&supportedspeeddescriptors)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDiscFormat2TrackAtOnce>,
            ::windows::core::GetTrustLevel,
            PrepareMedia::<Impl, OFFSET>,
            AddAudioTrack::<Impl, OFFSET>,
            CancelAddTrack::<Impl, OFFSET>,
            ReleaseMedia::<Impl, OFFSET>,
            SetWriteSpeed::<Impl, OFFSET>,
            SetRecorder::<Impl, OFFSET>,
            Recorder::<Impl, OFFSET>,
            SetBufferUnderrunFreeDisabled::<Impl, OFFSET>,
            BufferUnderrunFreeDisabled::<Impl, OFFSET>,
            NumberOfExistingTracks::<Impl, OFFSET>,
            TotalSectorsOnMedia::<Impl, OFFSET>,
            FreeSectorsOnMedia::<Impl, OFFSET>,
            UsedSectorsOnMedia::<Impl, OFFSET>,
            SetDoNotFinalizeMedia::<Impl, OFFSET>,
            DoNotFinalizeMedia::<Impl, OFFSET>,
            ExpectedTableOfContents::<Impl, OFFSET>,
            CurrentPhysicalMediaType::<Impl, OFFSET>,
            SetClientName::<Impl, OFFSET>,
            ClientName::<Impl, OFFSET>,
            RequestedWriteSpeed::<Impl, OFFSET>,
            RequestedRotationTypeIsPureCAV::<Impl, OFFSET>,
            CurrentWriteSpeed::<Impl, OFFSET>,
            CurrentRotationTypeIsPureCAV::<Impl, OFFSET>,
            SupportedWriteSpeeds::<Impl, OFFSET>,
            SupportedWriteSpeedDescriptors::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDiscFormat2TrackAtOnceEventArgsImpl: Sized + IWriteEngine2EventArgsImpl + IDispatchImpl {
    fn CurrentTrackNumber();
    fn CurrentAction();
    fn ElapsedTime();
    fn RemainingTime();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDiscFormat2TrackAtOnceEventArgs {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IDiscFormat2TrackAtOnceEventArgs";
}
#[cfg(feature = "Win32_System_Com")]
impl IDiscFormat2TrackAtOnceEventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscFormat2TrackAtOnceEventArgsImpl, const OFFSET: isize>() -> IDiscFormat2TrackAtOnceEventArgsVtbl {
        unsafe extern "system" fn CurrentTrackNumber<Impl: IDiscFormat2TrackAtOnceEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentTrackNumber(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentAction<Impl: IDiscFormat2TrackAtOnceEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_TAO_WRITE_ACTION) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentAction(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ElapsedTime<Impl: IDiscFormat2TrackAtOnceEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ElapsedTime(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemainingTime<Impl: IDiscFormat2TrackAtOnceEventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemainingTime(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDiscFormat2TrackAtOnceEventArgs>, ::windows::core::GetTrustLevel, CurrentTrackNumber::<Impl, OFFSET>, CurrentAction::<Impl, OFFSET>, ElapsedTime::<Impl, OFFSET>, RemainingTime::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for IDiscMaster {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IDiscMaster";
}
impl IDiscMasterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMasterImpl, const OFFSET: isize>() -> IDiscMasterVtbl {
        unsafe extern "system" fn Open<Impl: IDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Open() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDiscMasterFormats<Impl: IDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDiscMasterFormats(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveDiscMasterFormat<Impl: IDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpiid: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveDiscMasterFormat(::core::mem::transmute_copy(&lpiid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveDiscMasterFormat<Impl: IDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, riid: *const ::windows::core::GUID, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetActiveDiscMasterFormat(&*(&riid as *const <::windows::core::GUID as ::windows::core::Abi>::Abi as *const <::windows::core::GUID as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumDiscRecorders<Impl: IDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumDiscRecorders(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetActiveDiscRecorder<Impl: IDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprecorder: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetActiveDiscRecorder(::core::mem::transmute_copy(&pprecorder)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetActiveDiscRecorder<Impl: IDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, precorder: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetActiveDiscRecorder(&*(&precorder as *const <IDiscRecorder as ::windows::core::Abi>::Abi as *const <IDiscRecorder as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearFormatContent<Impl: IDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearFormatContent() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgressAdvise<Impl: IDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pevents: ::windows::core::RawPtr, pvcookie: *mut usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProgressAdvise(&*(&pevents as *const <IDiscMasterProgressEvents as ::windows::core::Abi>::Abi as *const <IDiscMasterProgressEvents as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvcookie)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgressUnadvise<Impl: IDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vcookie: usize) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProgressUnadvise(vcookie) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecordDisc<Impl: IDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bsimulate: u8, bejectafterburn: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RecordDisc(bsimulate, bejectafterburn) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDiscMaster>,
            ::windows::core::GetTrustLevel,
            Open::<Impl, OFFSET>,
            EnumDiscMasterFormats::<Impl, OFFSET>,
            GetActiveDiscMasterFormat::<Impl, OFFSET>,
            SetActiveDiscMasterFormat::<Impl, OFFSET>,
            EnumDiscRecorders::<Impl, OFFSET>,
            GetActiveDiscRecorder::<Impl, OFFSET>,
            SetActiveDiscRecorder::<Impl, OFFSET>,
            ClearFormatContent::<Impl, OFFSET>,
            ProgressAdvise::<Impl, OFFSET>,
            ProgressUnadvise::<Impl, OFFSET>,
            RecordDisc::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IDiscMaster2Impl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn IsSupportedEnvironment();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDiscMaster2 {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IDiscMaster2";
}
#[cfg(feature = "Win32_System_Com")]
impl IDiscMaster2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMaster2Impl, const OFFSET: isize>() -> IDiscMaster2Vtbl {
        unsafe extern "system" fn _NewEnum<Impl: IDiscMaster2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppunk: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IDiscMaster2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IDiscMaster2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsSupportedEnvironment<Impl: IDiscMaster2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupportedEnvironment(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDiscMaster2>, ::windows::core::GetTrustLevel, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, IsSupportedEnvironment::<Impl, OFFSET>)
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
impl ::windows::core::RuntimeName for IDiscMasterProgressEvents {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IDiscMasterProgressEvents";
}
impl IDiscMasterProgressEventsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscMasterProgressEventsImpl, const OFFSET: isize>() -> IDiscMasterProgressEventsVtbl {
        unsafe extern "system" fn QueryCancel<Impl: IDiscMasterProgressEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbcancel: *mut u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryCancel(::core::mem::transmute_copy(&pbcancel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyPnPActivity<Impl: IDiscMasterProgressEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyPnPActivity() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyAddProgress<Impl: IDiscMasterProgressEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncompletedsteps: i32, ntotalsteps: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyAddProgress(ncompletedsteps, ntotalsteps) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyBlockProgress<Impl: IDiscMasterProgressEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncompleted: i32, ntotal: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyBlockProgress(ncompleted, ntotal) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyTrackProgress<Impl: IDiscMasterProgressEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ncurrenttrack: i32, ntotaltracks: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyTrackProgress(ncurrenttrack, ntotaltracks) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyPreparingBurn<Impl: IDiscMasterProgressEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nestimatedseconds: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyPreparingBurn(nestimatedseconds) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyClosingDisc<Impl: IDiscMasterProgressEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nestimatedseconds: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyClosingDisc(nestimatedseconds) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyBurnComplete<Impl: IDiscMasterProgressEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyBurnComplete(status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NotifyEraseComplete<Impl: IDiscMasterProgressEventsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NotifyEraseComplete(status) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDiscMasterProgressEvents>,
            ::windows::core::GetTrustLevel,
            QueryCancel::<Impl, OFFSET>,
            NotifyPnPActivity::<Impl, OFFSET>,
            NotifyAddProgress::<Impl, OFFSET>,
            NotifyBlockProgress::<Impl, OFFSET>,
            NotifyTrackProgress::<Impl, OFFSET>,
            NotifyPreparingBurn::<Impl, OFFSET>,
            NotifyClosingDisc::<Impl, OFFSET>,
            NotifyBurnComplete::<Impl, OFFSET>,
            NotifyEraseComplete::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IDiscRecorder {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IDiscRecorder";
}
impl IDiscRecorderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorderImpl, const OFFSET: isize>() -> IDiscRecorderVtbl {
        unsafe extern "system" fn Init<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbyuniqueid: *const u8, nulidsize: u32, nuldrivenumber: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Init(pbyuniqueid, nulidsize, nuldrivenumber) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecorderGUID<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbyuniqueid: *mut u8, ulbuffersize: u32, pulreturnsizerequired: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecorderGUID(pbyuniqueid, ulbuffersize, ::core::mem::transmute_copy(&pulreturnsizerequired)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecorderType<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ftypecode: *mut RECORDER_TYPES) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecorderType(::core::mem::transmute_copy(&ftypecode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDisplayNames<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrvendorid: *mut super::super::Foundation::BSTR, pbstrproductid: *mut super::super::Foundation::BSTR, pbstrrevision: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDisplayNames(
                &*(&pbstrvendorid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pbstrproductid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pbstrrevision as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBasePnPID<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbasepnpid: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBasePnPID(::core::mem::transmute_copy(&pbstrbasepnpid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetPath<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrpath: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPath(::core::mem::transmute_copy(&pbstrpath)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecorderProperties<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecorderProperties(::core::mem::transmute_copy(&pppropstg)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecorderProperties<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRecorderProperties(&*(&ppropstg as *const <super::super::System::Com::StructuredStorage::IPropertyStorage as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::IPropertyStorage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRecorderState<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, puldevstateflags: *mut DISC_RECORDER_STATE_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRecorderState(::core::mem::transmute_copy(&puldevstateflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenExclusive<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenExclusive() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryMediaType<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fmediatype: *mut MEDIA_TYPES, fmediaflags: *mut MEDIA_FLAGS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryMediaType(::core::mem::transmute_copy(&fmediatype), ::core::mem::transmute_copy(&fmediaflags)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn QueryMediaInfo<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsessions: *mut u8, pblasttrack: *mut u8, ulstartaddress: *mut u32, ulnextwritable: *mut u32, ulfreeblocks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).QueryMediaInfo(::core::mem::transmute_copy(&pbsessions), ::core::mem::transmute_copy(&pblasttrack), ::core::mem::transmute_copy(&ulstartaddress), ::core::mem::transmute_copy(&ulnextwritable), ::core::mem::transmute_copy(&ulfreeblocks)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Eject<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Eject() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Erase<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bfullerase: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Erase(bfullerase) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Close<Impl: IDiscRecorderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Close() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDiscRecorder>,
            ::windows::core::GetTrustLevel,
            Init::<Impl, OFFSET>,
            GetRecorderGUID::<Impl, OFFSET>,
            GetRecorderType::<Impl, OFFSET>,
            GetDisplayNames::<Impl, OFFSET>,
            GetBasePnPID::<Impl, OFFSET>,
            GetPath::<Impl, OFFSET>,
            GetRecorderProperties::<Impl, OFFSET>,
            SetRecorderProperties::<Impl, OFFSET>,
            GetRecorderState::<Impl, OFFSET>,
            OpenExclusive::<Impl, OFFSET>,
            QueryMediaType::<Impl, OFFSET>,
            QueryMediaInfo::<Impl, OFFSET>,
            Eject::<Impl, OFFSET>,
            Erase::<Impl, OFFSET>,
            Close::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IDiscRecorder2 {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IDiscRecorder2";
}
#[cfg(feature = "Win32_System_Com")]
impl IDiscRecorder2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2Impl, const OFFSET: isize>() -> IDiscRecorder2Vtbl {
        unsafe extern "system" fn EjectMedia<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EjectMedia() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseTray<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CloseTray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AcquireExclusiveAccess<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, force: i16, __midl__idiscrecorder20000: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AcquireExclusiveAccess(force, &*(&__midl__idiscrecorder20000 as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReleaseExclusiveAccess<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReleaseExclusiveAccess() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableMcn<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableMcn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableMcn<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnableMcn() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeDiscRecorder<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, recorderuniqueid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeDiscRecorder(&*(&recorderuniqueid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActiveDiscRecorder<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActiveDiscRecorder(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VendorId<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VendorId(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductId<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductId(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProductRevision<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProductRevision(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeName<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VolumeName(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumePathNames<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VolumePathNames(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeviceCanLoadMedia<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeviceCanLoadMedia(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LegacyDeviceNumber<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, legacydevicenumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LegacyDeviceNumber(::core::mem::transmute_copy(&legacydevicenumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedFeaturePages<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedFeaturePages(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentFeaturePages<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentFeaturePages(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedProfiles<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedProfiles(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CurrentProfiles<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CurrentProfiles(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SupportedModePages<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SupportedModePages(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExclusiveAccessOwner<Impl: IDiscRecorder2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExclusiveAccessOwner(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDiscRecorder2>,
            ::windows::core::GetTrustLevel,
            EjectMedia::<Impl, OFFSET>,
            CloseTray::<Impl, OFFSET>,
            AcquireExclusiveAccess::<Impl, OFFSET>,
            ReleaseExclusiveAccess::<Impl, OFFSET>,
            DisableMcn::<Impl, OFFSET>,
            EnableMcn::<Impl, OFFSET>,
            InitializeDiscRecorder::<Impl, OFFSET>,
            ActiveDiscRecorder::<Impl, OFFSET>,
            VendorId::<Impl, OFFSET>,
            ProductId::<Impl, OFFSET>,
            ProductRevision::<Impl, OFFSET>,
            VolumeName::<Impl, OFFSET>,
            VolumePathNames::<Impl, OFFSET>,
            DeviceCanLoadMedia::<Impl, OFFSET>,
            LegacyDeviceNumber::<Impl, OFFSET>,
            SupportedFeaturePages::<Impl, OFFSET>,
            CurrentFeaturePages::<Impl, OFFSET>,
            SupportedProfiles::<Impl, OFFSET>,
            CurrentProfiles::<Impl, OFFSET>,
            SupportedModePages::<Impl, OFFSET>,
            ExclusiveAccessOwner::<Impl, OFFSET>,
        )
    }
}
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
impl ::windows::core::RuntimeName for IDiscRecorder2Ex {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IDiscRecorder2Ex";
}
impl IDiscRecorder2ExVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDiscRecorder2ExImpl, const OFFSET: isize>() -> IDiscRecorder2ExVtbl {
        unsafe extern "system" fn SendCommandNoData<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendCommandNoData(cdb, cdbsize, ::core::mem::transmute_copy(&sensebuffer), timeout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendCommandSendDataToDevice<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *const u8, buffersize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendCommandSendDataToDevice(cdb, cdbsize, ::core::mem::transmute_copy(&sensebuffer), timeout, buffer, buffersize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendCommandGetDataFromDevice<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *mut u8, buffersize: u32, bufferfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendCommandGetDataFromDevice(cdb, cdbsize, ::core::mem::transmute_copy(&sensebuffer), timeout, ::core::mem::transmute_copy(&buffer), buffersize, ::core::mem::transmute_copy(&bufferfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ReadDvdStructure<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: u32, address: u32, layer: u32, agid: u32, data: *mut *mut u8, count: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ReadDvdStructure(format, address, layer, agid, ::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SendDvdStructure<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, format: u32, data: *const u8, count: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SendDvdStructure(format, data, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAdapterDescriptor<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAdapterDescriptor(::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&bytesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDeviceDescriptor<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDeviceDescriptor(::core::mem::transmute_copy(&data), ::core::mem::transmute_copy(&bytesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDiscInformation<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDiscInformation(::core::mem::transmute_copy(&discinformation), ::core::mem::transmute_copy(&bytesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTrackInformation<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, address: u32, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTrackInformation(address, addresstype, ::core::mem::transmute_copy(&trackinformation), ::core::mem::transmute_copy(&bytesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetFeaturePage<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetFeaturePage(requestedfeature, &*(&currentfeatureonly as *const <super::super::Foundation::BOOLEAN as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOLEAN as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&featuredata), ::core::mem::transmute_copy(&bytesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetModePage<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut u8, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetModePage(requestedmodepage, requesttype, ::core::mem::transmute_copy(&modepagedata), ::core::mem::transmute_copy(&bytesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetModePage<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: *const u8, bytesize: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetModePage(requesttype, data, bytesize) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedFeaturePages<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentfeatureonly: super::super::Foundation::BOOLEAN, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedFeaturePages(&*(&currentfeatureonly as *const <super::super::Foundation::BOOLEAN as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOLEAN as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&featuredata), ::core::mem::transmute_copy(&bytesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedProfiles<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, currentonly: super::super::Foundation::BOOLEAN, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedProfiles(&*(&currentonly as *const <super::super::Foundation::BOOLEAN as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOLEAN as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&profiletypes), ::core::mem::transmute_copy(&validprofiles)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSupportedModePages<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSupportedModePages(requesttype, ::core::mem::transmute_copy(&modepagetypes), ::core::mem::transmute_copy(&validpages)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetByteAlignmentMask<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetByteAlignmentMask(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaximumNonPageAlignedTransferSize<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaximumNonPageAlignedTransferSize(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMaximumPageAlignedTransferSize<Impl: IDiscRecorder2ExImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMaximumPageAlignedTransferSize(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IDiscRecorder2Ex>,
            ::windows::core::GetTrustLevel,
            SendCommandNoData::<Impl, OFFSET>,
            SendCommandSendDataToDevice::<Impl, OFFSET>,
            SendCommandGetDataFromDevice::<Impl, OFFSET>,
            ReadDvdStructure::<Impl, OFFSET>,
            SendDvdStructure::<Impl, OFFSET>,
            GetAdapterDescriptor::<Impl, OFFSET>,
            GetDeviceDescriptor::<Impl, OFFSET>,
            GetDiscInformation::<Impl, OFFSET>,
            GetTrackInformation::<Impl, OFFSET>,
            GetFeaturePage::<Impl, OFFSET>,
            GetModePage::<Impl, OFFSET>,
            SetModePage::<Impl, OFFSET>,
            GetSupportedFeaturePages::<Impl, OFFSET>,
            GetSupportedProfiles::<Impl, OFFSET>,
            GetSupportedModePages::<Impl, OFFSET>,
            GetByteAlignmentMask::<Impl, OFFSET>,
            GetMaximumNonPageAlignedTransferSize::<Impl, OFFSET>,
            GetMaximumPageAlignedTransferSize::<Impl, OFFSET>,
        )
    }
}
pub trait IEnumDiscMasterFormatsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumDiscMasterFormats {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IEnumDiscMasterFormats";
}
impl IEnumDiscMasterFormatsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDiscMasterFormatsImpl, const OFFSET: isize>() -> IEnumDiscMasterFormatsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumDiscMasterFormatsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cformats: u32, lpiidformatid: *mut ::windows::core::GUID, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(cformats, ::core::mem::transmute_copy(&lpiidformatid), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumDiscMasterFormatsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, cformats: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(cformats) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumDiscMasterFormatsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumDiscMasterFormatsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumDiscMasterFormats>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumDiscRecordersImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumDiscRecorders {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IEnumDiscRecorders";
}
impl IEnumDiscRecordersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumDiscRecordersImpl, const OFFSET: isize>() -> IEnumDiscRecordersVtbl {
        unsafe extern "system" fn Next<Impl: IEnumDiscRecordersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crecorders: u32, pprecorder: *mut ::windows::core::RawPtr, pcfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(crecorders, ::core::mem::transmute_copy(&pprecorder), ::core::mem::transmute_copy(&pcfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumDiscRecordersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, crecorders: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(crecorders) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumDiscRecordersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumDiscRecordersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumDiscRecorders>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumFsiItemsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumFsiItems {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IEnumFsiItems";
}
impl IEnumFsiItemsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumFsiItemsImpl, const OFFSET: isize>() -> IEnumFsiItemsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumFsiItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumFsiItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumFsiItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumFsiItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumFsiItems>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
pub trait IEnumProgressItemsImpl: Sized {
    fn Next();
    fn Skip();
    fn Reset();
    fn Clone();
}
impl ::windows::core::RuntimeName for IEnumProgressItems {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IEnumProgressItems";
}
impl IEnumProgressItemsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IEnumProgressItemsImpl, const OFFSET: isize>() -> IEnumProgressItemsVtbl {
        unsafe extern "system" fn Next<Impl: IEnumProgressItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut ::windows::core::RawPtr, pceltfetched: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Next(celt, ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Skip<Impl: IEnumProgressItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Skip(celt) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Reset<Impl: IEnumProgressItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Clone<Impl: IEnumProgressItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Clone(::core::mem::transmute_copy(&ppenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IEnumProgressItems>, ::windows::core::GetTrustLevel, Next::<Impl, OFFSET>, Skip::<Impl, OFFSET>, Reset::<Impl, OFFSET>, Clone::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFileSystemImage {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IFileSystemImage";
}
#[cfg(feature = "Win32_System_Com")]
impl IFileSystemImageVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImageImpl, const OFFSET: isize>() -> IFileSystemImageVtbl {
        unsafe extern "system" fn Root<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Root(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SessionStartBlock<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SessionStartBlock(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSessionStartBlock<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSessionStartBlock(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeMediaBlocks<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FreeMediaBlocks(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFreeMediaBlocks<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFreeMediaBlocks(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxMediaBlocksFromDevice<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discrecorder: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxMediaBlocksFromDevice(&*(&discrecorder as *const <IDiscRecorder2 as ::windows::core::Abi>::Abi as *const <IDiscRecorder2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsedBlocks<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsedBlocks(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeName<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VolumeName(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVolumeName<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVolumeName(&*(&newval as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportedVolumeName<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportedVolumeName(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BootImageOptions<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BootImageOptions(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBootImageOptions<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBootImageOptions(&*(&newval as *const <IBootOptions as ::windows::core::Abi>::Abi as *const <IBootOptions as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileCount<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileCount(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DirectoryCount<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DirectoryCount(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WorkingDirectory<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WorkingDirectory(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetWorkingDirectory<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetWorkingDirectory(&*(&newval as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChangePoint<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChangePoint(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StrictFileSystemCompliance<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StrictFileSystemCompliance(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStrictFileSystemCompliance<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStrictFileSystemCompliance(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseRestrictedCharacterSet<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseRestrictedCharacterSet(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseRestrictedCharacterSet<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUseRestrictedCharacterSet(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileSystemsToCreate<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileSystemsToCreate(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetFileSystemsToCreate<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetFileSystemsToCreate(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileSystemsSupported<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileSystemsSupported(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUDFRevision<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUDFRevision(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UDFRevision<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UDFRevision(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UDFRevisionsSupported<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UDFRevisionsSupported(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChooseImageDefaults<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discrecorder: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChooseImageDefaults(&*(&discrecorder as *const <IDiscRecorder2 as ::windows::core::Abi>::Abi as *const <IDiscRecorder2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ChooseImageDefaultsForMediaType<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ChooseImageDefaultsForMediaType(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetISO9660InterchangeLevel<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetISO9660InterchangeLevel(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ISO9660InterchangeLevel<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ISO9660InterchangeLevel(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ISO9660InterchangeLevelsSupported<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ISO9660InterchangeLevelsSupported(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateResultImage<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateResultImage(::core::mem::transmute_copy(&resultstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Exists<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fullpath: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, itemtype: *mut FsiItemType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Exists(&*(&fullpath as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&itemtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CalculateDiscIdentifier<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discidentifier: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CalculateDiscIdentifier(::core::mem::transmute_copy(&discidentifier)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IdentifyFileSystemsOnDisc<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, discrecorder: ::windows::core::RawPtr, filesystems: *mut FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IdentifyFileSystemsOnDisc(&*(&discrecorder as *const <IDiscRecorder2 as ::windows::core::Abi>::Abi as *const <IDiscRecorder2 as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&filesystems)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDefaultFileSystemForImport<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesystems: FsiFileSystems, importdefault: *mut FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDefaultFileSystemForImport(filesystems, ::core::mem::transmute_copy(&importdefault)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportFileSystem<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, importedfilesystem: *mut FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportFileSystem(::core::mem::transmute_copy(&importedfilesystem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportSpecificFileSystem<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesystemtouse: FsiFileSystems) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportSpecificFileSystem(filesystemtouse) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RollbackToChangePoint<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, changepoint: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RollbackToChangePoint(changepoint) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LockInChangePoint<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LockInChangePoint() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateDirectoryItem<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateDirectoryItem(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&newitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFileItem<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, newitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFileItem(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&newitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeNameUDF<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VolumeNameUDF(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeNameJoliet<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VolumeNameJoliet(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn VolumeNameISO9660<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).VolumeNameISO9660(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StageFiles<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StageFiles(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStageFiles<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStageFiles(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MultisessionInterfaces<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MultisessionInterfaces(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMultisessionInterfaces<Impl: IFileSystemImageImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMultisessionInterfaces(&*(&newval as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFileSystemImage>,
            ::windows::core::GetTrustLevel,
            Root::<Impl, OFFSET>,
            SessionStartBlock::<Impl, OFFSET>,
            SetSessionStartBlock::<Impl, OFFSET>,
            FreeMediaBlocks::<Impl, OFFSET>,
            SetFreeMediaBlocks::<Impl, OFFSET>,
            SetMaxMediaBlocksFromDevice::<Impl, OFFSET>,
            UsedBlocks::<Impl, OFFSET>,
            VolumeName::<Impl, OFFSET>,
            SetVolumeName::<Impl, OFFSET>,
            ImportedVolumeName::<Impl, OFFSET>,
            BootImageOptions::<Impl, OFFSET>,
            SetBootImageOptions::<Impl, OFFSET>,
            FileCount::<Impl, OFFSET>,
            DirectoryCount::<Impl, OFFSET>,
            WorkingDirectory::<Impl, OFFSET>,
            SetWorkingDirectory::<Impl, OFFSET>,
            ChangePoint::<Impl, OFFSET>,
            StrictFileSystemCompliance::<Impl, OFFSET>,
            SetStrictFileSystemCompliance::<Impl, OFFSET>,
            UseRestrictedCharacterSet::<Impl, OFFSET>,
            SetUseRestrictedCharacterSet::<Impl, OFFSET>,
            FileSystemsToCreate::<Impl, OFFSET>,
            SetFileSystemsToCreate::<Impl, OFFSET>,
            FileSystemsSupported::<Impl, OFFSET>,
            SetUDFRevision::<Impl, OFFSET>,
            UDFRevision::<Impl, OFFSET>,
            UDFRevisionsSupported::<Impl, OFFSET>,
            ChooseImageDefaults::<Impl, OFFSET>,
            ChooseImageDefaultsForMediaType::<Impl, OFFSET>,
            SetISO9660InterchangeLevel::<Impl, OFFSET>,
            ISO9660InterchangeLevel::<Impl, OFFSET>,
            ISO9660InterchangeLevelsSupported::<Impl, OFFSET>,
            CreateResultImage::<Impl, OFFSET>,
            Exists::<Impl, OFFSET>,
            CalculateDiscIdentifier::<Impl, OFFSET>,
            IdentifyFileSystemsOnDisc::<Impl, OFFSET>,
            GetDefaultFileSystemForImport::<Impl, OFFSET>,
            ImportFileSystem::<Impl, OFFSET>,
            ImportSpecificFileSystem::<Impl, OFFSET>,
            RollbackToChangePoint::<Impl, OFFSET>,
            LockInChangePoint::<Impl, OFFSET>,
            CreateDirectoryItem::<Impl, OFFSET>,
            CreateFileItem::<Impl, OFFSET>,
            VolumeNameUDF::<Impl, OFFSET>,
            VolumeNameJoliet::<Impl, OFFSET>,
            VolumeNameISO9660::<Impl, OFFSET>,
            StageFiles::<Impl, OFFSET>,
            SetStageFiles::<Impl, OFFSET>,
            MultisessionInterfaces::<Impl, OFFSET>,
            SetMultisessionInterfaces::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFileSystemImage2Impl: Sized + IFileSystemImageImpl + IDispatchImpl {
    fn BootImageOptionsArray();
    fn SetBootImageOptionsArray();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFileSystemImage2 {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IFileSystemImage2";
}
#[cfg(feature = "Win32_System_Com")]
impl IFileSystemImage2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage2Impl, const OFFSET: isize>() -> IFileSystemImage2Vtbl {
        unsafe extern "system" fn BootImageOptionsArray<Impl: IFileSystemImage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BootImageOptionsArray(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBootImageOptionsArray<Impl: IFileSystemImage2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: *const super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBootImageOptionsArray(&*(&newval as *const <super::super::System::Com::SAFEARRAY as ::windows::core::Abi>::Abi as *const <super::super::System::Com::SAFEARRAY as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileSystemImage2>, ::windows::core::GetTrustLevel, BootImageOptionsArray::<Impl, OFFSET>, SetBootImageOptionsArray::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFileSystemImage3Impl: Sized + IFileSystemImage2Impl + IFileSystemImageImpl + IDispatchImpl {
    fn CreateRedundantUdfMetadataFiles();
    fn SetCreateRedundantUdfMetadataFiles();
    fn ProbeSpecificFileSystem();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFileSystemImage3 {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IFileSystemImage3";
}
#[cfg(feature = "Win32_System_Com")]
impl IFileSystemImage3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImage3Impl, const OFFSET: isize>() -> IFileSystemImage3Vtbl {
        unsafe extern "system" fn CreateRedundantUdfMetadataFiles<Impl: IFileSystemImage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRedundantUdfMetadataFiles(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreateRedundantUdfMetadataFiles<Impl: IFileSystemImage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCreateRedundantUdfMetadataFiles(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProbeSpecificFileSystem<Impl: IFileSystemImage3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesystemtoprobe: FsiFileSystems, isappendable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProbeSpecificFileSystem(filesystemtoprobe, ::core::mem::transmute_copy(&isappendable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileSystemImage3>, ::windows::core::GetTrustLevel, CreateRedundantUdfMetadataFiles::<Impl, OFFSET>, SetCreateRedundantUdfMetadataFiles::<Impl, OFFSET>, ProbeSpecificFileSystem::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFileSystemImageResultImpl: Sized + IDispatchImpl {
    fn ImageStream();
    fn ProgressItems();
    fn TotalBlocks();
    fn BlockSize();
    fn DiscId();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFileSystemImageResult {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IFileSystemImageResult";
}
#[cfg(feature = "Win32_System_Com")]
impl IFileSystemImageResultVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImageResultImpl, const OFFSET: isize>() -> IFileSystemImageResultVtbl {
        unsafe extern "system" fn ImageStream<Impl: IFileSystemImageResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImageStream(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgressItems<Impl: IFileSystemImageResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProgressItems(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalBlocks<Impl: IFileSystemImageResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalBlocks(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BlockSize<Impl: IFileSystemImageResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BlockSize(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DiscId<Impl: IFileSystemImageResultImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DiscId(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileSystemImageResult>, ::windows::core::GetTrustLevel, ImageStream::<Impl, OFFSET>, ProgressItems::<Impl, OFFSET>, TotalBlocks::<Impl, OFFSET>, BlockSize::<Impl, OFFSET>, DiscId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFileSystemImageResult2Impl: Sized + IFileSystemImageResultImpl + IDispatchImpl {
    fn ModifiedBlocks();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFileSystemImageResult2 {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IFileSystemImageResult2";
}
#[cfg(feature = "Win32_System_Com")]
impl IFileSystemImageResult2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFileSystemImageResult2Impl, const OFFSET: isize>() -> IFileSystemImageResult2Vtbl {
        unsafe extern "system" fn ModifiedBlocks<Impl: IFileSystemImageResult2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ModifiedBlocks(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFileSystemImageResult2>, ::windows::core::GetTrustLevel, ModifiedBlocks::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsiDirectoryItemImpl: Sized + IFsiItemImpl + IDispatchImpl {
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFsiDirectoryItem {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IFsiDirectoryItem";
}
#[cfg(feature = "Win32_System_Com")]
impl IFsiDirectoryItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsiDirectoryItemImpl, const OFFSET: isize>() -> IFsiDirectoryItemVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFsiDirectoryItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&newenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFsiDirectoryItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFsiDirectoryItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumFsiItems<Impl: IFsiDirectoryItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumFsiItems(::core::mem::transmute_copy(&newenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDirectory<Impl: IFsiDirectoryItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddDirectory(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddFile<Impl: IFsiDirectoryItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, filedata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddFile(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&filedata as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTree<Impl: IFsiDirectoryItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcedirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, includebasedirectory: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddTree(&*(&sourcedirectory as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), includebasedirectory) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Impl: IFsiDirectoryItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Add(&*(&item as *const <IFsiItem as ::windows::core::Abi>::Abi as *const <IFsiItem as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IFsiDirectoryItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveTree<Impl: IFsiDirectoryItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, path: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveTree(&*(&path as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFsiDirectoryItem>,
            ::windows::core::GetTrustLevel,
            _NewEnum::<Impl, OFFSET>,
            Item::<Impl, OFFSET>,
            Count::<Impl, OFFSET>,
            EnumFsiItems::<Impl, OFFSET>,
            AddDirectory::<Impl, OFFSET>,
            AddFile::<Impl, OFFSET>,
            AddTree::<Impl, OFFSET>,
            Add::<Impl, OFFSET>,
            Remove::<Impl, OFFSET>,
            RemoveTree::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsiDirectoryItem2Impl: Sized + IFsiDirectoryItemImpl + IFsiItemImpl + IDispatchImpl {
    fn AddTreeWithNamedStreams();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFsiDirectoryItem2 {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IFsiDirectoryItem2";
}
#[cfg(feature = "Win32_System_Com")]
impl IFsiDirectoryItem2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsiDirectoryItem2Impl, const OFFSET: isize>() -> IFsiDirectoryItem2Vtbl {
        unsafe extern "system" fn AddTreeWithNamedStreams<Impl: IFsiDirectoryItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sourcedirectory: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, includebasedirectory: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddTreeWithNamedStreams(&*(&sourcedirectory as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), includebasedirectory) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFsiDirectoryItem2>, ::windows::core::GetTrustLevel, AddTreeWithNamedStreams::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsiFileItemImpl: Sized + IFsiItemImpl + IDispatchImpl {
    fn DataSize();
    fn DataSize32BitLow();
    fn DataSize32BitHigh();
    fn Data();
    fn SetData();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFsiFileItem {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IFsiFileItem";
}
#[cfg(feature = "Win32_System_Com")]
impl IFsiFileItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsiFileItemImpl, const OFFSET: isize>() -> IFsiFileItemVtbl {
        unsafe extern "system" fn DataSize<Impl: IFsiFileItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataSize(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataSize32BitLow<Impl: IFsiFileItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataSize32BitLow(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DataSize32BitHigh<Impl: IFsiFileItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataSize32BitHigh(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Data<Impl: IFsiFileItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Data(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetData<Impl: IFsiFileItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetData(&*(&newval as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFsiFileItem>, ::windows::core::GetTrustLevel, DataSize::<Impl, OFFSET>, DataSize32BitLow::<Impl, OFFSET>, DataSize32BitHigh::<Impl, OFFSET>, Data::<Impl, OFFSET>, SetData::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsiFileItem2Impl: Sized + IFsiFileItemImpl + IFsiItemImpl + IDispatchImpl {
    fn FsiNamedStreams();
    fn IsNamedStream();
    fn AddStream();
    fn RemoveStream();
    fn IsRealTime();
    fn SetIsRealTime();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFsiFileItem2 {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IFsiFileItem2";
}
#[cfg(feature = "Win32_System_Com")]
impl IFsiFileItem2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsiFileItem2Impl, const OFFSET: isize>() -> IFsiFileItem2Vtbl {
        unsafe extern "system" fn FsiNamedStreams<Impl: IFsiFileItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streams: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FsiNamedStreams(::core::mem::transmute_copy(&streams)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsNamedStream<Impl: IFsiFileItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsNamedStream(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStream<Impl: IFsiFileItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, streamdata: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddStream(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&streamdata as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveStream<Impl: IFsiFileItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveStream(&*(&name as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRealTime<Impl: IFsiFileItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRealTime(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRealTime<Impl: IFsiFileItem2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIsRealTime(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFsiFileItem2>, ::windows::core::GetTrustLevel, FsiNamedStreams::<Impl, OFFSET>, IsNamedStream::<Impl, OFFSET>, AddStream::<Impl, OFFSET>, RemoveStream::<Impl, OFFSET>, IsRealTime::<Impl, OFFSET>, SetIsRealTime::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFsiItem {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IFsiItem";
}
#[cfg(feature = "Win32_System_Com")]
impl IFsiItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsiItemImpl, const OFFSET: isize>() -> IFsiItemVtbl {
        unsafe extern "system" fn Name<Impl: IFsiItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FullPath<Impl: IFsiItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FullPath(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreationTime<Impl: IFsiItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreationTime(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCreationTime<Impl: IFsiItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetCreationTime(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastAccessedTime<Impl: IFsiItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastAccessedTime(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastAccessedTime<Impl: IFsiItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLastAccessedTime(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastModifiedTime<Impl: IFsiItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastModifiedTime(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLastModifiedTime<Impl: IFsiItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLastModifiedTime(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsHidden<Impl: IFsiItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsHidden(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsHidden<Impl: IFsiItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newval: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIsHidden(newval) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileSystemName<Impl: IFsiItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesystem: FsiFileSystems, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileSystemName(filesystem, ::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FileSystemPath<Impl: IFsiItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, filesystem: FsiFileSystems, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FileSystemPath(filesystem, ::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IFsiItem>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, OFFSET>,
            FullPath::<Impl, OFFSET>,
            CreationTime::<Impl, OFFSET>,
            SetCreationTime::<Impl, OFFSET>,
            LastAccessedTime::<Impl, OFFSET>,
            SetLastAccessedTime::<Impl, OFFSET>,
            LastModifiedTime::<Impl, OFFSET>,
            SetLastModifiedTime::<Impl, OFFSET>,
            IsHidden::<Impl, OFFSET>,
            SetIsHidden::<Impl, OFFSET>,
            FileSystemName::<Impl, OFFSET>,
            FileSystemPath::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFsiNamedStreamsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn EnumNamedStreams();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IFsiNamedStreams {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IFsiNamedStreams";
}
#[cfg(feature = "Win32_System_Com")]
impl IFsiNamedStreamsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFsiNamedStreamsImpl, const OFFSET: isize>() -> IFsiNamedStreamsVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IFsiNamedStreamsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&newenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IFsiNamedStreamsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IFsiNamedStreamsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumNamedStreams<Impl: IFsiNamedStreamsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumNamedStreams(::core::mem::transmute_copy(&newenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IFsiNamedStreams>, ::windows::core::GetTrustLevel, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, EnumNamedStreams::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IIsoImageManagerImpl: Sized + IDispatchImpl {
    fn Path();
    fn Stream();
    fn SetPath();
    fn SetStream();
    fn Validate();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IIsoImageManager {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IIsoImageManager";
}
#[cfg(feature = "Win32_System_Com")]
impl IIsoImageManagerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IIsoImageManagerImpl, const OFFSET: isize>() -> IIsoImageManagerVtbl {
        unsafe extern "system" fn Path<Impl: IIsoImageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pval: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Path(::core::mem::transmute_copy(&pval)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Stream<Impl: IIsoImageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Stream(::core::mem::transmute_copy(&data)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPath<Impl: IIsoImageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, val: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetPath(&*(&val as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStream<Impl: IIsoImageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStream(&*(&data as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Validate<Impl: IIsoImageManagerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Validate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IIsoImageManager>, ::windows::core::GetTrustLevel, Path::<Impl, OFFSET>, Stream::<Impl, OFFSET>, SetPath::<Impl, OFFSET>, SetStream::<Impl, OFFSET>, Validate::<Impl, OFFSET>)
    }
}
pub trait IJolietDiscMasterImpl: Sized {
    fn GetTotalDataBlocks();
    fn GetUsedDataBlocks();
    fn GetDataBlockSize();
    fn AddData();
    fn GetJolietProperties();
    fn SetJolietProperties();
}
impl ::windows::core::RuntimeName for IJolietDiscMaster {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IJolietDiscMaster";
}
impl IJolietDiscMasterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IJolietDiscMasterImpl, const OFFSET: isize>() -> IJolietDiscMasterVtbl {
        unsafe extern "system" fn GetTotalDataBlocks<Impl: IJolietDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTotalDataBlocks(::core::mem::transmute_copy(&pnblocks)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUsedDataBlocks<Impl: IJolietDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUsedDataBlocks(::core::mem::transmute_copy(&pnblocks)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataBlockSize<Impl: IJolietDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblockbytes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataBlockSize(::core::mem::transmute_copy(&pnblockbytes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddData<Impl: IJolietDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstorage: ::windows::core::RawPtr, lfileoverwrite: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddData(&*(&pstorage as *const <super::super::System::Com::StructuredStorage::IStorage as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::IStorage as ::windows::core::DefaultType>::DefaultType), lfileoverwrite) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetJolietProperties<Impl: IJolietDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pppropstg: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetJolietProperties(::core::mem::transmute_copy(&pppropstg)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJolietProperties<Impl: IJolietDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppropstg: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetJolietProperties(&*(&ppropstg as *const <super::super::System::Com::StructuredStorage::IPropertyStorage as ::windows::core::Abi>::Abi as *const <super::super::System::Com::StructuredStorage::IPropertyStorage as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IJolietDiscMaster>, ::windows::core::GetTrustLevel, GetTotalDataBlocks::<Impl, OFFSET>, GetUsedDataBlocks::<Impl, OFFSET>, GetDataBlockSize::<Impl, OFFSET>, AddData::<Impl, OFFSET>, GetJolietProperties::<Impl, OFFSET>, SetJolietProperties::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMultisessionImpl: Sized + IDispatchImpl {
    fn IsSupportedOnCurrentMediaState();
    fn SetInUse();
    fn InUse();
    fn ImportRecorder();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMultisession {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IMultisession";
}
#[cfg(feature = "Win32_System_Com")]
impl IMultisessionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultisessionImpl, const OFFSET: isize>() -> IMultisessionVtbl {
        unsafe extern "system" fn IsSupportedOnCurrentMediaState<Impl: IMultisessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsSupportedOnCurrentMediaState(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetInUse<Impl: IMultisessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetInUse(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InUse<Impl: IMultisessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InUse(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ImportRecorder<Impl: IMultisessionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ImportRecorder(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMultisession>, ::windows::core::GetTrustLevel, IsSupportedOnCurrentMediaState::<Impl, OFFSET>, SetInUse::<Impl, OFFSET>, InUse::<Impl, OFFSET>, ImportRecorder::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMultisessionRandomWriteImpl: Sized + IMultisessionImpl + IDispatchImpl {
    fn WriteUnitSize();
    fn LastWrittenAddress();
    fn TotalSectorsOnMedia();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMultisessionRandomWrite {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IMultisessionRandomWrite";
}
#[cfg(feature = "Win32_System_Com")]
impl IMultisessionRandomWriteVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultisessionRandomWriteImpl, const OFFSET: isize>() -> IMultisessionRandomWriteVtbl {
        unsafe extern "system" fn WriteUnitSize<Impl: IMultisessionRandomWriteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteUnitSize(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastWrittenAddress<Impl: IMultisessionRandomWriteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastWrittenAddress(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Impl: IMultisessionRandomWriteImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalSectorsOnMedia(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMultisessionRandomWrite>, ::windows::core::GetTrustLevel, WriteUnitSize::<Impl, OFFSET>, LastWrittenAddress::<Impl, OFFSET>, TotalSectorsOnMedia::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMultisessionSequentialImpl: Sized + IMultisessionImpl + IDispatchImpl {
    fn IsFirstDataSession();
    fn StartAddressOfPreviousSession();
    fn LastWrittenAddressOfPreviousSession();
    fn NextWritableAddress();
    fn FreeSectorsOnMedia();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMultisessionSequential {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IMultisessionSequential";
}
#[cfg(feature = "Win32_System_Com")]
impl IMultisessionSequentialVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultisessionSequentialImpl, const OFFSET: isize>() -> IMultisessionSequentialVtbl {
        unsafe extern "system" fn IsFirstDataSession<Impl: IMultisessionSequentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFirstDataSession(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartAddressOfPreviousSession<Impl: IMultisessionSequentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartAddressOfPreviousSession(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastWrittenAddressOfPreviousSession<Impl: IMultisessionSequentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastWrittenAddressOfPreviousSession(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NextWritableAddress<Impl: IMultisessionSequentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NextWritableAddress(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Impl: IMultisessionSequentialImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FreeSectorsOnMedia(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMultisessionSequential>, ::windows::core::GetTrustLevel, IsFirstDataSession::<Impl, OFFSET>, StartAddressOfPreviousSession::<Impl, OFFSET>, LastWrittenAddressOfPreviousSession::<Impl, OFFSET>, NextWritableAddress::<Impl, OFFSET>, FreeSectorsOnMedia::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMultisessionSequential2Impl: Sized + IMultisessionSequentialImpl + IMultisessionImpl + IDispatchImpl {
    fn WriteUnitSize();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMultisessionSequential2 {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IMultisessionSequential2";
}
#[cfg(feature = "Win32_System_Com")]
impl IMultisessionSequential2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMultisessionSequential2Impl, const OFFSET: isize>() -> IMultisessionSequential2Vtbl {
        unsafe extern "system" fn WriteUnitSize<Impl: IMultisessionSequential2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteUnitSize(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMultisessionSequential2>, ::windows::core::GetTrustLevel, WriteUnitSize::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProgressItemImpl: Sized + IDispatchImpl {
    fn Description();
    fn FirstBlock();
    fn LastBlock();
    fn BlockCount();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IProgressItem {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IProgressItem";
}
#[cfg(feature = "Win32_System_Com")]
impl IProgressItemVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProgressItemImpl, const OFFSET: isize>() -> IProgressItemVtbl {
        unsafe extern "system" fn Description<Impl: IProgressItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, desc: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&desc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FirstBlock<Impl: IProgressItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, block: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FirstBlock(::core::mem::transmute_copy(&block)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastBlock<Impl: IProgressItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, block: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastBlock(::core::mem::transmute_copy(&block)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BlockCount<Impl: IProgressItemImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, blocks: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BlockCount(::core::mem::transmute_copy(&blocks)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProgressItem>, ::windows::core::GetTrustLevel, Description::<Impl, OFFSET>, FirstBlock::<Impl, OFFSET>, LastBlock::<Impl, OFFSET>, BlockCount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IProgressItemsImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn Count();
    fn ProgressItemFromBlock();
    fn ProgressItemFromDescription();
    fn EnumProgressItems();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IProgressItems {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IProgressItems";
}
#[cfg(feature = "Win32_System_Com")]
impl IProgressItemsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IProgressItemsImpl, const OFFSET: isize>() -> IProgressItemsVtbl {
        unsafe extern "system" fn _NewEnum<Impl: IProgressItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&newenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Impl: IProgressItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IProgressItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgressItemFromBlock<Impl: IProgressItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, block: u32, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProgressItemFromBlock(block, ::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProgressItemFromDescription<Impl: IProgressItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, description: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProgressItemFromDescription(&*(&description as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumProgressItems<Impl: IProgressItemsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newenum: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EnumProgressItems(::core::mem::transmute_copy(&newenum)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IProgressItems>, ::windows::core::GetTrustLevel, _NewEnum::<Impl, OFFSET>, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, ProgressItemFromBlock::<Impl, OFFSET>, ProgressItemFromDescription::<Impl, OFFSET>, EnumProgressItems::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRawCDImageCreator {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IRawCDImageCreator";
}
#[cfg(feature = "Win32_System_Com")]
impl IRawCDImageCreatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageCreatorImpl, const OFFSET: isize>() -> IRawCDImageCreatorVtbl {
        unsafe extern "system" fn CreateResultImage<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, resultstream: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateResultImage(::core::mem::transmute_copy(&resultstream)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTrack<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, datatype: IMAPI_CD_SECTOR_TYPE, data: ::windows::core::RawPtr, trackindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddTrack(datatype, &*(&data as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&trackindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSpecialPregap<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddSpecialPregap(&*(&data as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddSubcodeRWGenerator<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, subcode: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddSubcodeRWGenerator(&*(&subcode as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetResultingImageType<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetResultingImageType(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResultingImageType<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ResultingImageType(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartOfLeadout<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartOfLeadout(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartOfLeadoutLimit<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStartOfLeadoutLimit(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartOfLeadoutLimit<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartOfLeadoutLimit(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDisableGaplessAudio<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDisableGaplessAudio(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DisableGaplessAudio<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DisableGaplessAudio(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMediaCatalogNumber<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMediaCatalogNumber(&*(&value as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MediaCatalogNumber<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaCatalogNumber(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartingTrackNumber<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStartingTrackNumber(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartingTrackNumber<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartingTrackNumber(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrackInfo<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, trackindex: i32, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrackInfo(trackindex, ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NumberOfExistingTracks<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NumberOfExistingTracks(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastUsedUserSectorInImage<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastUsedUserSectorInImage(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExpectedTableOfContents<Impl: IRawCDImageCreatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExpectedTableOfContents(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IRawCDImageCreator>,
            ::windows::core::GetTrustLevel,
            CreateResultImage::<Impl, OFFSET>,
            AddTrack::<Impl, OFFSET>,
            AddSpecialPregap::<Impl, OFFSET>,
            AddSubcodeRWGenerator::<Impl, OFFSET>,
            SetResultingImageType::<Impl, OFFSET>,
            ResultingImageType::<Impl, OFFSET>,
            StartOfLeadout::<Impl, OFFSET>,
            SetStartOfLeadoutLimit::<Impl, OFFSET>,
            StartOfLeadoutLimit::<Impl, OFFSET>,
            SetDisableGaplessAudio::<Impl, OFFSET>,
            DisableGaplessAudio::<Impl, OFFSET>,
            SetMediaCatalogNumber::<Impl, OFFSET>,
            MediaCatalogNumber::<Impl, OFFSET>,
            SetStartingTrackNumber::<Impl, OFFSET>,
            StartingTrackNumber::<Impl, OFFSET>,
            TrackInfo::<Impl, OFFSET>,
            NumberOfExistingTracks::<Impl, OFFSET>,
            LastUsedUserSectorInImage::<Impl, OFFSET>,
            ExpectedTableOfContents::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IRawCDImageTrackInfo {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IRawCDImageTrackInfo";
}
#[cfg(feature = "Win32_System_Com")]
impl IRawCDImageTrackInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>() -> IRawCDImageTrackInfoVtbl {
        unsafe extern "system" fn StartingLba<Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartingLba(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SectorCount<Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SectorCount(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrackNumber<Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrackNumber(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SectorType<Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_CD_SECTOR_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SectorType(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ISRC<Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ISRC(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetISRC<Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetISRC(&*(&value as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DigitalAudioCopySetting<Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DigitalAudioCopySetting(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDigitalAudioCopySetting<Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDigitalAudioCopySetting(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AudioHasPreemphasis<Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AudioHasPreemphasis(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAudioHasPreemphasis<Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAudioHasPreemphasis(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TrackIndexes<Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TrackIndexes(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTrackIndex<Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbaoffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddTrackIndex(lbaoffset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ClearTrackIndex<Impl: IRawCDImageTrackInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lbaoffset: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ClearTrackIndex(lbaoffset) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IRawCDImageTrackInfo>,
            ::windows::core::GetTrustLevel,
            StartingLba::<Impl, OFFSET>,
            SectorCount::<Impl, OFFSET>,
            TrackNumber::<Impl, OFFSET>,
            SectorType::<Impl, OFFSET>,
            ISRC::<Impl, OFFSET>,
            SetISRC::<Impl, OFFSET>,
            DigitalAudioCopySetting::<Impl, OFFSET>,
            SetDigitalAudioCopySetting::<Impl, OFFSET>,
            AudioHasPreemphasis::<Impl, OFFSET>,
            SetAudioHasPreemphasis::<Impl, OFFSET>,
            TrackIndexes::<Impl, OFFSET>,
            AddTrackIndex::<Impl, OFFSET>,
            ClearTrackIndex::<Impl, OFFSET>,
        )
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
impl ::windows::core::RuntimeName for IRedbookDiscMaster {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IRedbookDiscMaster";
}
impl IRedbookDiscMasterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IRedbookDiscMasterImpl, const OFFSET: isize>() -> IRedbookDiscMasterVtbl {
        unsafe extern "system" fn GetTotalAudioTracks<Impl: IRedbookDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pntracks: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTotalAudioTracks(::core::mem::transmute_copy(&pntracks)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTotalAudioBlocks<Impl: IRedbookDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTotalAudioBlocks(::core::mem::transmute_copy(&pnblocks)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUsedAudioBlocks<Impl: IRedbookDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetUsedAudioBlocks(::core::mem::transmute_copy(&pnblocks)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAvailableAudioTrackBlocks<Impl: IRedbookDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblocks: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAvailableAudioTrackBlocks(::core::mem::transmute_copy(&pnblocks)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAudioBlockSize<Impl: IRedbookDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnblockbytes: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAudioBlockSize(::core::mem::transmute_copy(&pnblockbytes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateAudioTrack<Impl: IRedbookDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, nblocks: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateAudioTrack(nblocks) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAudioTrackBlocks<Impl: IRedbookDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pby: *const u8, cb: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddAudioTrackBlocks(pby, cb) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseAudioTrack<Impl: IRedbookDiscMasterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CloseAudioTrack() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IRedbookDiscMaster>,
            ::windows::core::GetTrustLevel,
            GetTotalAudioTracks::<Impl, OFFSET>,
            GetTotalAudioBlocks::<Impl, OFFSET>,
            GetUsedAudioBlocks::<Impl, OFFSET>,
            GetAvailableAudioTrackBlocks::<Impl, OFFSET>,
            GetAudioBlockSize::<Impl, OFFSET>,
            CreateAudioTrack::<Impl, OFFSET>,
            AddAudioTrackBlocks::<Impl, OFFSET>,
            CloseAudioTrack::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStreamConcatenateImpl: Sized + IStreamImpl + ISequentialStreamImpl {
    fn Initialize();
    fn Initialize2();
    fn Append();
    fn Append2();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IStreamConcatenate {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IStreamConcatenate";
}
#[cfg(feature = "Win32_System_Com")]
impl IStreamConcatenateVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamConcatenateImpl, const OFFSET: isize>() -> IStreamConcatenateVtbl {
        unsafe extern "system" fn Initialize<Impl: IStreamConcatenateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream1: ::windows::core::RawPtr, stream2: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&stream1 as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), &*(&stream2 as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize2<Impl: IStreamConcatenateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streams: *const ::windows::core::RawPtr, streamcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize2(&*(&streams as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), streamcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append<Impl: IStreamConcatenateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, stream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Append(&*(&stream as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Append2<Impl: IStreamConcatenateImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streams: *const ::windows::core::RawPtr, streamcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Append2(&*(&streams as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), streamcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStreamConcatenate>, ::windows::core::GetTrustLevel, Initialize::<Impl, OFFSET>, Initialize2::<Impl, OFFSET>, Append::<Impl, OFFSET>, Append2::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStreamInterleaveImpl: Sized + IStreamImpl + ISequentialStreamImpl {
    fn Initialize();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IStreamInterleave {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IStreamInterleave";
}
#[cfg(feature = "Win32_System_Com")]
impl IStreamInterleaveVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamInterleaveImpl, const OFFSET: isize>() -> IStreamInterleaveVtbl {
        unsafe extern "system" fn Initialize<Impl: IStreamInterleaveImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, streams: *const ::windows::core::RawPtr, interleavesizes: *const u32, streamcount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(&*(&streams as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), interleavesizes, streamcount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStreamInterleave>, ::windows::core::GetTrustLevel, Initialize::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IStreamPseudoRandomBasedImpl: Sized + IStreamImpl + ISequentialStreamImpl {
    fn SetSeed();
    fn Seed();
    fn SetExtendedSeed();
    fn ExtendedSeed();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IStreamPseudoRandomBased {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IStreamPseudoRandomBased";
}
#[cfg(feature = "Win32_System_Com")]
impl IStreamPseudoRandomBasedVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IStreamPseudoRandomBasedImpl, const OFFSET: isize>() -> IStreamPseudoRandomBasedVtbl {
        unsafe extern "system" fn SetSeed<Impl: IStreamPseudoRandomBasedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetSeed(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Seed<Impl: IStreamPseudoRandomBasedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Seed(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetExtendedSeed<Impl: IStreamPseudoRandomBasedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: *const u32, ecount: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetExtendedSeed(values, ecount) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ExtendedSeed<Impl: IStreamPseudoRandomBasedImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, values: *mut *mut u32, ecount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ExtendedSeed(::core::mem::transmute_copy(&values), ::core::mem::transmute_copy(&ecount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IStreamPseudoRandomBased>, ::windows::core::GetTrustLevel, SetSeed::<Impl, OFFSET>, Seed::<Impl, OFFSET>, SetExtendedSeed::<Impl, OFFSET>, ExtendedSeed::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
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
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWriteEngine2 {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IWriteEngine2";
}
#[cfg(feature = "Win32_System_Com")]
impl IWriteEngine2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2Impl, const OFFSET: isize>() -> IWriteEngine2Vtbl {
        unsafe extern "system" fn WriteSection<Impl: IWriteEngine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, data: ::windows::core::RawPtr, startingblockaddress: i32, numberofblocks: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteSection(&*(&data as *const <super::super::System::Com::IStream as ::windows::core::Abi>::Abi as *const <super::super::System::Com::IStream as ::windows::core::DefaultType>::DefaultType), startingblockaddress, numberofblocks) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CancelWrite<Impl: IWriteEngine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CancelWrite() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRecorder<Impl: IWriteEngine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRecorder(&*(&value as *const <IDiscRecorder2Ex as ::windows::core::Abi>::Abi as *const <IDiscRecorder2Ex as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Recorder<Impl: IWriteEngine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Recorder(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetUseStreamingWrite12<Impl: IWriteEngine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetUseStreamingWrite12(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UseStreamingWrite12<Impl: IWriteEngine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UseStreamingWrite12(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetStartingSectorsPerSecond<Impl: IWriteEngine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetStartingSectorsPerSecond(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn StartingSectorsPerSecond<Impl: IWriteEngine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartingSectorsPerSecond(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetEndingSectorsPerSecond<Impl: IWriteEngine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetEndingSectorsPerSecond(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EndingSectorsPerSecond<Impl: IWriteEngine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).EndingSectorsPerSecond(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBytesPerSector<Impl: IWriteEngine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBytesPerSector(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BytesPerSector<Impl: IWriteEngine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BytesPerSector(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteInProgress<Impl: IWriteEngine2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteInProgress(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWriteEngine2>,
            ::windows::core::GetTrustLevel,
            WriteSection::<Impl, OFFSET>,
            CancelWrite::<Impl, OFFSET>,
            SetRecorder::<Impl, OFFSET>,
            Recorder::<Impl, OFFSET>,
            SetUseStreamingWrite12::<Impl, OFFSET>,
            UseStreamingWrite12::<Impl, OFFSET>,
            SetStartingSectorsPerSecond::<Impl, OFFSET>,
            StartingSectorsPerSecond::<Impl, OFFSET>,
            SetEndingSectorsPerSecond::<Impl, OFFSET>,
            EndingSectorsPerSecond::<Impl, OFFSET>,
            SetBytesPerSector::<Impl, OFFSET>,
            BytesPerSector::<Impl, OFFSET>,
            WriteInProgress::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWriteEngine2EventArgsImpl: Sized + IDispatchImpl {
    fn StartLba();
    fn SectorCount();
    fn LastReadLba();
    fn LastWrittenLba();
    fn TotalSystemBuffer();
    fn UsedSystemBuffer();
    fn FreeSystemBuffer();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWriteEngine2EventArgs {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IWriteEngine2EventArgs";
}
#[cfg(feature = "Win32_System_Com")]
impl IWriteEngine2EventArgsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWriteEngine2EventArgsImpl, const OFFSET: isize>() -> IWriteEngine2EventArgsVtbl {
        unsafe extern "system" fn StartLba<Impl: IWriteEngine2EventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).StartLba(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SectorCount<Impl: IWriteEngine2EventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SectorCount(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastReadLba<Impl: IWriteEngine2EventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastReadLba(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LastWrittenLba<Impl: IWriteEngine2EventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LastWrittenLba(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TotalSystemBuffer<Impl: IWriteEngine2EventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TotalSystemBuffer(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UsedSystemBuffer<Impl: IWriteEngine2EventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UsedSystemBuffer(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FreeSystemBuffer<Impl: IWriteEngine2EventArgsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FreeSystemBuffer(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IWriteEngine2EventArgs>,
            ::windows::core::GetTrustLevel,
            StartLba::<Impl, OFFSET>,
            SectorCount::<Impl, OFFSET>,
            LastReadLba::<Impl, OFFSET>,
            LastWrittenLba::<Impl, OFFSET>,
            TotalSystemBuffer::<Impl, OFFSET>,
            UsedSystemBuffer::<Impl, OFFSET>,
            FreeSystemBuffer::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IWriteSpeedDescriptorImpl: Sized + IDispatchImpl {
    fn MediaType();
    fn RotationTypeIsPureCAV();
    fn WriteSpeed();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IWriteSpeedDescriptor {
    const NAME: &'static str = "Windows.Win32.Storage.Imapi.IWriteSpeedDescriptor";
}
#[cfg(feature = "Win32_System_Com")]
impl IWriteSpeedDescriptorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IWriteSpeedDescriptorImpl, const OFFSET: isize>() -> IWriteSpeedDescriptorVtbl {
        unsafe extern "system" fn MediaType<Impl: IWriteSpeedDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MediaType(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RotationTypeIsPureCAV<Impl: IWriteSpeedDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RotationTypeIsPureCAV(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WriteSpeed<Impl: IWriteSpeedDescriptorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WriteSpeed(::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IWriteSpeedDescriptor>, ::windows::core::GetTrustLevel, MediaType::<Impl, OFFSET>, RotationTypeIsPureCAV::<Impl, OFFSET>, WriteSpeed::<Impl, OFFSET>)
    }
}
