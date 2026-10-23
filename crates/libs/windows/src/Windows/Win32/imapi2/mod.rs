#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(DDiscFormat2DataEvents, DDiscFormat2DataEvents_Vtbl, 0x2735413c_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for DDiscFormat2DataEvents {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(DDiscFormat2DataEvents, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl DDiscFormat2DataEvents {
    pub unsafe fn Update<P0, P1>(&self, object: P0, progress: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
        P1: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), object.param().abi(), progress.param().abi()) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct DDiscFormat2DataEvents_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait DDiscFormat2DataEvents_Impl: super::IDispatch_Impl {
    fn Update(&self, object: windows_core::Ref<super::IDispatch>, progress: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl DDiscFormat2DataEvents_Vtbl {
    pub const fn new<Identity: DDiscFormat2DataEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Update<Identity: DDiscFormat2DataEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, progress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                DDiscFormat2DataEvents_Impl::Update(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&progress)).into()
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DDiscFormat2DataEvents as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for DDiscFormat2DataEvents {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(DDiscFormat2EraseEvents, DDiscFormat2EraseEvents_Vtbl, 0x2735413a_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for DDiscFormat2EraseEvents {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(DDiscFormat2EraseEvents, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl DDiscFormat2EraseEvents {
    #[cfg(feature = "shared")]
    pub unsafe fn Update<P0>(&self, object: P0, elapsedseconds: super::LONG, estimatedtotalseconds: super::LONG) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), object.param().abi(), elapsedseconds, estimatedtotalseconds) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct DDiscFormat2EraseEvents_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Update: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait DDiscFormat2EraseEvents_Impl: super::IDispatch_Impl {
    fn Update(&self, object: windows_core::Ref<super::IDispatch>, elapsedseconds: super::LONG, estimatedtotalseconds: super::LONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl DDiscFormat2EraseEvents_Vtbl {
    pub const fn new<Identity: DDiscFormat2EraseEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Update<Identity: DDiscFormat2EraseEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, elapsedseconds: super::LONG, estimatedtotalseconds: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                DDiscFormat2EraseEvents_Impl::Update(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&elapsedseconds), core::mem::transmute_copy(&estimatedtotalseconds)).into()
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DDiscFormat2EraseEvents as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for DDiscFormat2EraseEvents {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(DDiscFormat2RawCDEvents, DDiscFormat2RawCDEvents_Vtbl, 0x27354142_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for DDiscFormat2RawCDEvents {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(DDiscFormat2RawCDEvents, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl DDiscFormat2RawCDEvents {
    pub unsafe fn Update<P0, P1>(&self, object: P0, progress: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
        P1: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), object.param().abi(), progress.param().abi()) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct DDiscFormat2RawCDEvents_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait DDiscFormat2RawCDEvents_Impl: super::IDispatch_Impl {
    fn Update(&self, object: windows_core::Ref<super::IDispatch>, progress: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl DDiscFormat2RawCDEvents_Vtbl {
    pub const fn new<Identity: DDiscFormat2RawCDEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Update<Identity: DDiscFormat2RawCDEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, progress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                DDiscFormat2RawCDEvents_Impl::Update(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&progress)).into()
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DDiscFormat2RawCDEvents as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for DDiscFormat2RawCDEvents {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(DDiscFormat2TrackAtOnceEvents, DDiscFormat2TrackAtOnceEvents_Vtbl, 0x2735413f_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for DDiscFormat2TrackAtOnceEvents {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(DDiscFormat2TrackAtOnceEvents, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl DDiscFormat2TrackAtOnceEvents {
    pub unsafe fn Update<P0, P1>(&self, object: P0, progress: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
        P1: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), object.param().abi(), progress.param().abi()) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct DDiscFormat2TrackAtOnceEvents_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait DDiscFormat2TrackAtOnceEvents_Impl: super::IDispatch_Impl {
    fn Update(&self, object: windows_core::Ref<super::IDispatch>, progress: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl DDiscFormat2TrackAtOnceEvents_Vtbl {
    pub const fn new<Identity: DDiscFormat2TrackAtOnceEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Update<Identity: DDiscFormat2TrackAtOnceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, progress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                DDiscFormat2TrackAtOnceEvents_Impl::Update(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&progress)).into()
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DDiscFormat2TrackAtOnceEvents as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for DDiscFormat2TrackAtOnceEvents {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(DDiscMaster2Events, DDiscMaster2Events_Vtbl, 0x27354131_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for DDiscMaster2Events {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(DDiscMaster2Events, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl DDiscMaster2Events {
    pub unsafe fn NotifyDeviceAdded<P0>(&self, object: P0, uniqueid: &windows_core::BSTR) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).NotifyDeviceAdded)(windows_core::Interface::as_raw(self), object.param().abi(), core::mem::transmute_copy(uniqueid)) }
    }
    pub unsafe fn NotifyDeviceRemoved<P0>(&self, object: P0, uniqueid: &windows_core::BSTR) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).NotifyDeviceRemoved)(windows_core::Interface::as_raw(self), object.param().abi(), core::mem::transmute_copy(uniqueid)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct DDiscMaster2Events_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub NotifyDeviceAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyDeviceRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait DDiscMaster2Events_Impl: super::IDispatch_Impl {
    fn NotifyDeviceAdded(&self, object: windows_core::Ref<super::IDispatch>, uniqueid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn NotifyDeviceRemoved(&self, object: windows_core::Ref<super::IDispatch>, uniqueid: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl DDiscMaster2Events_Vtbl {
    pub const fn new<Identity: DDiscMaster2Events_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn NotifyDeviceAdded<Identity: DDiscMaster2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, uniqueid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                DDiscMaster2Events_Impl::NotifyDeviceAdded(this, core::mem::transmute_copy(&object), core::mem::transmute(&uniqueid)).into()
            }
        }
        unsafe extern "system" fn NotifyDeviceRemoved<Identity: DDiscMaster2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, uniqueid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                DDiscMaster2Events_Impl::NotifyDeviceRemoved(this, core::mem::transmute_copy(&object), core::mem::transmute(&uniqueid)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            NotifyDeviceAdded: NotifyDeviceAdded::<Identity, OFFSET>,
            NotifyDeviceRemoved: NotifyDeviceRemoved::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DDiscMaster2Events as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for DDiscMaster2Events {}
pub const DISPID_DDISCFORMAT2DATAEVENTS_UPDATE: i32 = 512;
pub const DISPID_DDISCFORMAT2RAWCDEVENTS_UPDATE: i32 = 512;
pub const DISPID_DDISCFORMAT2TAOEVENTS_UPDATE: i32 = 512;
pub const DISPID_DDISCMASTER2EVENTS_DEVICEADDED: i32 = 256;
pub const DISPID_DDISCMASTER2EVENTS_DEVICEREMOVED: i32 = 257;
pub const DISPID_DWRITEENGINE2EVENTS_UPDATE: i32 = 256;
pub const DISPID_IBLOCKRANGELIST_BLOCKRANGES: i32 = 256;
pub const DISPID_IBLOCKRANGE_ENDLBA: i32 = 257;
pub const DISPID_IBLOCKRANGE_STARTLBA: i32 = 256;
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_CURRENTACTION: i32 = 771;
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ELAPSEDTIME: i32 = 768;
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ESTIMATEDREMAININGTIME: i32 = 769;
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ESTIMATEDTOTALTIME: i32 = 770;
pub const DISPID_IDISCFORMAT2DATA_BUFFERUNDERRUNFREEDISABLED: i32 = 257;
pub const DISPID_IDISCFORMAT2DATA_CANCELWRITE: i32 = 513;
pub const DISPID_IDISCFORMAT2DATA_CLIENTNAME: i32 = 272;
pub const DISPID_IDISCFORMAT2DATA_CURRENTMEDIASTATUS: i32 = 262;
pub const DISPID_IDISCFORMAT2DATA_CURRENTMEDIATYPE: i32 = 271;
pub const DISPID_IDISCFORMAT2DATA_CURRENTROTATIONTYPEISPURECAV: i32 = 276;
pub const DISPID_IDISCFORMAT2DATA_CURRENTWRITESPEED: i32 = 275;
pub const DISPID_IDISCFORMAT2DATA_DISABLEDVDCOMPATIBILITYMODE: i32 = 270;
pub const DISPID_IDISCFORMAT2DATA_FORCEMEDIATOBECLOSED: i32 = 269;
pub const DISPID_IDISCFORMAT2DATA_FORCEOVERWRITE: i32 = 279;
pub const DISPID_IDISCFORMAT2DATA_FREESECTORS: i32 = 265;
pub const DISPID_IDISCFORMAT2DATA_LASTSECTOROFPREVIOUSSESSION: i32 = 268;
pub const DISPID_IDISCFORMAT2DATA_MUTLISESSIONINTERFACES: i32 = 280;
pub const DISPID_IDISCFORMAT2DATA_NEXTWRITABLEADDRESS: i32 = 266;
pub const DISPID_IDISCFORMAT2DATA_POSTGAPALREADYINIMAGE: i32 = 260;
pub const DISPID_IDISCFORMAT2DATA_RECORDER: i32 = 256;
pub const DISPID_IDISCFORMAT2DATA_REQUESTEDROTATIONTYPEISPURECAV: i32 = 274;
pub const DISPID_IDISCFORMAT2DATA_REQUESTEDWRITESPEED: i32 = 273;
pub const DISPID_IDISCFORMAT2DATA_SETWRITESPEED: i32 = 514;
pub const DISPID_IDISCFORMAT2DATA_STARTSECTOROFPREVIOUSSESSION: i32 = 267;
pub const DISPID_IDISCFORMAT2DATA_SUPPORTEDWRITESPEEDDESCRIPTORS: i32 = 278;
pub const DISPID_IDISCFORMAT2DATA_SUPPORTEDWRITESPEEDS: i32 = 277;
pub const DISPID_IDISCFORMAT2DATA_TOTALSECTORS: i32 = 264;
pub const DISPID_IDISCFORMAT2DATA_WRITE: i32 = 512;
pub const DISPID_IDISCFORMAT2DATA_WRITEPROTECTSTATUS: i32 = 263;
pub const DISPID_IDISCFORMAT2ERASEEVENTS_UPDATE: i32 = 512;
pub const DISPID_IDISCFORMAT2ERASE_CLIENTNAME: i32 = 259;
pub const DISPID_IDISCFORMAT2ERASE_ERASEMEDIA: i32 = 513;
pub const DISPID_IDISCFORMAT2ERASE_FULLERASE: i32 = 257;
pub const DISPID_IDISCFORMAT2ERASE_MEDIATYPE: i32 = 258;
pub const DISPID_IDISCFORMAT2ERASE_RECORDER: i32 = 256;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_CURRENTACTION: i32 = 769;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_CURRENTTRACKNUMBER: i32 = 768;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ELAPSEDTIME: i32 = 768;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ESTIMATEDREMAININGTIME: i32 = 769;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ESTIMATEDTOTALTIME: i32 = 770;
pub const DISPID_IDISCFORMAT2RAWCD_BUFFERUNDERRUNFREEDISABLED: i32 = 258;
pub const DISPID_IDISCFORMAT2RAWCD_CANCELWRITE: i32 = 515;
pub const DISPID_IDISCFORMAT2RAWCD_CLIENTNAME: i32 = 266;
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTMEDIATYPE: i32 = 261;
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTROTATIONTYPEISPURECAV: i32 = 270;
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTWRITESPEED: i32 = 269;
pub const DISPID_IDISCFORMAT2RAWCD_LASTPOSSIBLESTARTOFLEADOUT: i32 = 260;
pub const DISPID_IDISCFORMAT2RAWCD_PREPAREMEDIA: i32 = 512;
pub const DISPID_IDISCFORMAT2RAWCD_RECORDER: i32 = 256;
pub const DISPID_IDISCFORMAT2RAWCD_RELEASEMEDIA: i32 = 516;
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDDATASECTORTYPE: i32 = 265;
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDROTATIONTYPEISPURECAV: i32 = 268;
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDWRITESPEED: i32 = 267;
pub const DISPID_IDISCFORMAT2RAWCD_SETWRITESPEED: i32 = 517;
pub const DISPID_IDISCFORMAT2RAWCD_STARTOFNEXTSESSION: i32 = 259;
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDDATASECTORTYPES: i32 = 264;
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDWRITESPEEDDESCRIPTORS: i32 = 272;
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDWRITESPEEDS: i32 = 271;
pub const DISPID_IDISCFORMAT2RAWCD_WRITEMEDIA: i32 = 513;
pub const DISPID_IDISCFORMAT2RAWCD_WRITEMEDIAWITHVALIDATION: i32 = 514;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_CURRENTACTION: i32 = 769;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_CURRENTTRACKNUMBER: i32 = 768;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ELAPSEDTIME: i32 = 770;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ESTIMATEDREMAININGTIME: i32 = 771;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ESTIMATEDTOTALTIME: i32 = 772;
pub const DISPID_IDISCFORMAT2TAO_ADDAUDIOTRACK: i32 = 513;
pub const DISPID_IDISCFORMAT2TAO_BUFFERUNDERRUNFREEDISABLED: i32 = 258;
pub const DISPID_IDISCFORMAT2TAO_CANCELADDTRACK: i32 = 514;
pub const DISPID_IDISCFORMAT2TAO_CLIENTNAME: i32 = 270;
pub const DISPID_IDISCFORMAT2TAO_CURRENTMEDIATYPE: i32 = 267;
pub const DISPID_IDISCFORMAT2TAO_CURRENTROTATIONTYPEISPURECAV: i32 = 274;
pub const DISPID_IDISCFORMAT2TAO_CURRENTWRITESPEED: i32 = 273;
pub const DISPID_IDISCFORMAT2TAO_DONOTFINALIZEMEDIA: i32 = 263;
pub const DISPID_IDISCFORMAT2TAO_EXPECTEDTABLEOFCONTENTS: i32 = 266;
pub const DISPID_IDISCFORMAT2TAO_FINISHMEDIA: i32 = 515;
pub const DISPID_IDISCFORMAT2TAO_FREESECTORSONMEDIA: i32 = 261;
pub const DISPID_IDISCFORMAT2TAO_NUMBEROFEXISTINGTRACKS: i32 = 259;
pub const DISPID_IDISCFORMAT2TAO_PREPAREMEDIA: i32 = 512;
pub const DISPID_IDISCFORMAT2TAO_RECORDER: i32 = 256;
pub const DISPID_IDISCFORMAT2TAO_REQUESTEDROTATIONTYPEISPURECAV: i32 = 272;
pub const DISPID_IDISCFORMAT2TAO_REQUESTEDWRITESPEED: i32 = 271;
pub const DISPID_IDISCFORMAT2TAO_SETWRITESPEED: i32 = 516;
pub const DISPID_IDISCFORMAT2TAO_SUPPORTEDWRITESPEEDDESCRIPTORS: i32 = 276;
pub const DISPID_IDISCFORMAT2TAO_SUPPORTEDWRITESPEEDS: i32 = 275;
pub const DISPID_IDISCFORMAT2TAO_TOTALSECTORSONMEDIA: i32 = 260;
pub const DISPID_IDISCFORMAT2TAO_USEDSECTORSONMEDIA: i32 = 262;
pub const DISPID_IDISCFORMAT2_MEDIAHEURISTICALLYBLANK: i32 = 1793;
pub const DISPID_IDISCFORMAT2_MEDIAPHYSICALLYBLANK: i32 = 1792;
pub const DISPID_IDISCFORMAT2_MEDIASUPPORTED: i32 = 2049;
pub const DISPID_IDISCFORMAT2_RECORDERSUPPORTED: i32 = 2048;
pub const DISPID_IDISCFORMAT2_SUPPORTEDMEDIATYPES: i32 = 1794;
pub const DISPID_IDISCRECORDER2_ACQUIREEXCLUSIVEACCESS: i32 = 258;
pub const DISPID_IDISCRECORDER2_ACTIVEDISCRECORDER: i32 = 0;
pub const DISPID_IDISCRECORDER2_CLOSETRAY: i32 = 257;
pub const DISPID_IDISCRECORDER2_CURRENTFEATUREPAGES: i32 = 521;
pub const DISPID_IDISCRECORDER2_CURRENTPROFILES: i32 = 523;
pub const DISPID_IDISCRECORDER2_DEVICECANLOADMEDIA: i32 = 518;
pub const DISPID_IDISCRECORDER2_DISABLEMCN: i32 = 260;
pub const DISPID_IDISCRECORDER2_EJECTMEDIA: i32 = 256;
pub const DISPID_IDISCRECORDER2_ENABLEMCN: i32 = 261;
pub const DISPID_IDISCRECORDER2_EXCLUSIVEACCESSOWNER: i32 = 525;
pub const DISPID_IDISCRECORDER2_INITIALIZEDISCRECORDER: i32 = 262;
pub const DISPID_IDISCRECORDER2_LEGACYDEVICENUMBER: i32 = 519;
pub const DISPID_IDISCRECORDER2_PRODUCTID: i32 = 514;
pub const DISPID_IDISCRECORDER2_PRODUCTREVISION: i32 = 515;
pub const DISPID_IDISCRECORDER2_RELEASEEXCLUSIVEACCESS: i32 = 259;
pub const DISPID_IDISCRECORDER2_SUPPORTEDFEATUREPAGES: i32 = 520;
pub const DISPID_IDISCRECORDER2_SUPPORTEDMODEPAGES: i32 = 524;
pub const DISPID_IDISCRECORDER2_SUPPORTEDPROFILES: i32 = 522;
pub const DISPID_IDISCRECORDER2_VENDORID: i32 = 513;
pub const DISPID_IDISCRECORDER2_VOLUMENAME: i32 = 516;
pub const DISPID_IDISCRECORDER2_VOLUMEPATHNAMES: i32 = 517;
pub const DISPID_IMULTISESSION_FIRSTDATASESSION: i32 = 512;
pub const DISPID_IMULTISESSION_FREESECTORS: i32 = 516;
pub const DISPID_IMULTISESSION_IMPORTRECORDER: i32 = 258;
pub const DISPID_IMULTISESSION_INUSE: i32 = 257;
pub const DISPID_IMULTISESSION_LASTSECTOROFPREVIOUSSESSION: i32 = 514;
pub const DISPID_IMULTISESSION_LASTWRITTENADDRESS: i32 = 518;
pub const DISPID_IMULTISESSION_NEXTWRITABLEADDRESS: i32 = 515;
pub const DISPID_IMULTISESSION_SECTORSONMEDIA: i32 = 519;
pub const DISPID_IMULTISESSION_STARTSECTOROFPREVIOUSSESSION: i32 = 513;
pub const DISPID_IMULTISESSION_SUPPORTEDONCURRENTMEDIA: i32 = 256;
pub const DISPID_IMULTISESSION_WRITEUNITSIZE: i32 = 517;
pub const DISPID_IRAWCDIMAGECREATOR_ADDSPECIALPREGAP: i32 = 514;
pub const DISPID_IRAWCDIMAGECREATOR_ADDSUBCODERWGENERATOR: i32 = 515;
pub const DISPID_IRAWCDIMAGECREATOR_ADDTRACK: i32 = 513;
pub const DISPID_IRAWCDIMAGECREATOR_CREATERESULTIMAGE: i32 = 512;
pub const DISPID_IRAWCDIMAGECREATOR_DISABLEGAPLESSAUDIO: i32 = 259;
pub const DISPID_IRAWCDIMAGECREATOR_EXPECTEDTABLEOFCONTENTS: i32 = 265;
pub const DISPID_IRAWCDIMAGECREATOR_MEDIACATALOGNUMBER: i32 = 260;
pub const DISPID_IRAWCDIMAGECREATOR_NUMBEROFEXISTINGTRACKS: i32 = 263;
pub const DISPID_IRAWCDIMAGECREATOR_RESULTINGIMAGETYPE: i32 = 256;
pub const DISPID_IRAWCDIMAGECREATOR_STARTINGTRACKNUMBER: i32 = 261;
pub const DISPID_IRAWCDIMAGECREATOR_STARTOFLEADOUT: i32 = 257;
pub const DISPID_IRAWCDIMAGECREATOR_STARTOFLEADOUTLIMIT: i32 = 258;
pub const DISPID_IRAWCDIMAGECREATOR_TRACKINFO: i32 = 262;
pub const DISPID_IRAWCDIMAGECREATOR_USEDSECTORSONDISC: i32 = 264;
pub const DISPID_IRAWCDTRACKINFO_AUDIOHASPREEMPHASIS: i32 = 262;
pub const DISPID_IRAWCDTRACKINFO_DIGITALAUDIOCOPYSETTING: i32 = 261;
pub const DISPID_IRAWCDTRACKINFO_ISRC: i32 = 260;
pub const DISPID_IRAWCDTRACKINFO_SECTORCOUNT: i32 = 257;
pub const DISPID_IRAWCDTRACKINFO_SECTORTYPE: i32 = 259;
pub const DISPID_IRAWCDTRACKINFO_STARTINGLBA: i32 = 256;
pub const DISPID_IRAWCDTRACKINFO_TRACKNUMBER: i32 = 258;
pub const DISPID_IWRITEENGINE2EVENTARGS_FREESYSTEMBUFFER: i32 = 264;
pub const DISPID_IWRITEENGINE2EVENTARGS_LASTREADLBA: i32 = 258;
pub const DISPID_IWRITEENGINE2EVENTARGS_LASTWRITTENLBA: i32 = 259;
pub const DISPID_IWRITEENGINE2EVENTARGS_SECTORCOUNT: i32 = 257;
pub const DISPID_IWRITEENGINE2EVENTARGS_STARTLBA: i32 = 256;
pub const DISPID_IWRITEENGINE2EVENTARGS_TOTALDEVICEBUFFER: i32 = 260;
pub const DISPID_IWRITEENGINE2EVENTARGS_TOTALSYSTEMBUFFER: i32 = 262;
pub const DISPID_IWRITEENGINE2EVENTARGS_USEDDEVICEBUFFER: i32 = 261;
pub const DISPID_IWRITEENGINE2EVENTARGS_USEDSYSTEMBUFFER: i32 = 263;
pub const DISPID_IWRITEENGINE2_BYTESPERSECTOR: i32 = 260;
pub const DISPID_IWRITEENGINE2_CANCELWRITE: i32 = 513;
pub const DISPID_IWRITEENGINE2_DISCRECORDER: i32 = 256;
pub const DISPID_IWRITEENGINE2_ENDINGSECTORSPERSECOND: i32 = 259;
pub const DISPID_IWRITEENGINE2_STARTINGSECTORSPERSECOND: i32 = 258;
pub const DISPID_IWRITEENGINE2_USESTREAMINGWRITE12: i32 = 257;
pub const DISPID_IWRITEENGINE2_WRITEINPROGRESS: i32 = 261;
pub const DISPID_IWRITEENGINE2_WRITESECTION: i32 = 512;
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(DWriteEngine2Events, DWriteEngine2Events_Vtbl, 0x27354137_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for DWriteEngine2Events {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(DWriteEngine2Events, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl DWriteEngine2Events {
    pub unsafe fn Update<P0, P1>(&self, object: P0, progress: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IDispatch>,
        P1: windows_core::Param<super::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), object.param().abi(), progress.param().abi()) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct DWriteEngine2Events_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait DWriteEngine2Events_Impl: super::IDispatch_Impl {
    fn Update(&self, object: windows_core::Ref<super::IDispatch>, progress: windows_core::Ref<super::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl DWriteEngine2Events_Vtbl {
    pub const fn new<Identity: DWriteEngine2Events_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Update<Identity: DWriteEngine2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, progress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                DWriteEngine2Events_Impl::Update(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&progress)).into()
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DWriteEngine2Events as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for DWriteEngine2Events {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IBlockRange, IBlockRange_Vtbl, 0xb507ca25_2204_11dd_966a_001aa01bbc58);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IBlockRange {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IBlockRange, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IBlockRange {
    #[cfg(feature = "shared")]
    pub unsafe fn get_StartLba(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StartLba)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_EndLba(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_EndLba)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IBlockRange_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub get_StartLba: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_StartLba: usize,
    #[cfg(feature = "shared")]
    pub get_EndLba: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_EndLba: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IBlockRange_Impl: super::IDispatch_Impl {
    fn get_StartLba(&self) -> windows_core::Result<super::LONG>;
    fn get_EndLba(&self) -> windows_core::Result<super::LONG>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IBlockRange_Vtbl {
    pub const fn new<Identity: IBlockRange_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_StartLba<Identity: IBlockRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBlockRange_Impl::get_StartLba(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_EndLba<Identity: IBlockRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBlockRange_Impl::get_EndLba(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_StartLba: get_StartLba::<Identity, OFFSET>,
            get_EndLba: get_EndLba::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBlockRange as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IBlockRange {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IBlockRangeList, IBlockRangeList_Vtbl, 0xb507ca26_2204_11dd_966a_001aa01bbc58);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IBlockRangeList {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IBlockRangeList, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IBlockRangeList {
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_BlockRanges(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BlockRanges)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IBlockRangeList_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_BlockRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_BlockRanges: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IBlockRangeList_Impl: super::IDispatch_Impl {
    fn get_BlockRanges(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IBlockRangeList_Vtbl {
    pub const fn new<Identity: IBlockRangeList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_BlockRanges<Identity: IBlockRangeList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBlockRangeList_Impl::get_BlockRanges(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(), get_BlockRanges: get_BlockRanges::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBlockRangeList as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IBlockRangeList {}
windows_core::imp::define_interface!(IBurnVerification, IBurnVerification_Vtbl, 0xd2ffd834_958b_426d_8470_2a13879c6a91);
windows_core::imp::interface_hierarchy!(IBurnVerification, windows_core::IUnknown);
impl IBurnVerification {
    pub unsafe fn put_BurnVerificationLevel(&self, value: IMAPI_BURN_VERIFICATION_LEVEL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_BurnVerificationLevel)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn get_BurnVerificationLevel(&self) -> windows_core::Result<IMAPI_BURN_VERIFICATION_LEVEL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BurnVerificationLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBurnVerification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub put_BurnVerificationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_BURN_VERIFICATION_LEVEL) -> windows_core::HRESULT,
    pub get_BurnVerificationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_BURN_VERIFICATION_LEVEL) -> windows_core::HRESULT,
}
pub trait IBurnVerification_Impl: windows_core::IUnknownImpl {
    fn put_BurnVerificationLevel(&self, value: IMAPI_BURN_VERIFICATION_LEVEL) -> windows_core::Result<()>;
    fn get_BurnVerificationLevel(&self) -> windows_core::Result<IMAPI_BURN_VERIFICATION_LEVEL>;
}
impl IBurnVerification_Vtbl {
    pub const fn new<Identity: IBurnVerification_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn put_BurnVerificationLevel<Identity: IBurnVerification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: IMAPI_BURN_VERIFICATION_LEVEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBurnVerification_Impl::put_BurnVerificationLevel(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_BurnVerificationLevel<Identity: IBurnVerification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_BURN_VERIFICATION_LEVEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBurnVerification_Impl::get_BurnVerificationLevel(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            put_BurnVerificationLevel: put_BurnVerificationLevel::<Identity, OFFSET>,
            get_BurnVerificationLevel: get_BurnVerificationLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBurnVerification as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IBurnVerification {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDiscFormat2, IDiscFormat2_Vtbl, 0x27354152_8f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDiscFormat2 {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDiscFormat2, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IDiscFormat2 {
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsRecorderSupported<P0>(&self, recorder: P0) -> windows_core::Result<super::VARIANT_BOOL>
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRecorderSupported)(windows_core::Interface::as_raw(self), recorder.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsCurrentMediaSupported<P0>(&self, recorder: P0) -> windows_core::Result<super::VARIANT_BOOL>
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsCurrentMediaSupported)(windows_core::Interface::as_raw(self), recorder.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_MediaPhysicallyBlank(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MediaPhysicallyBlank)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_MediaHeuristicallyBlank(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MediaHeuristicallyBlank)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_SupportedMediaTypes(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SupportedMediaTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "wtypes")]
    pub IsRecorderSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsRecorderSupported: usize,
    #[cfg(feature = "wtypes")]
    pub IsCurrentMediaSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsCurrentMediaSupported: usize,
    #[cfg(feature = "wtypes")]
    pub get_MediaPhysicallyBlank: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_MediaPhysicallyBlank: usize,
    #[cfg(feature = "wtypes")]
    pub get_MediaHeuristicallyBlank: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_MediaHeuristicallyBlank: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_SupportedMediaTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_SupportedMediaTypes: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDiscFormat2_Impl: super::IDispatch_Impl {
    fn IsRecorderSupported(&self, recorder: windows_core::Ref<IDiscRecorder2>) -> windows_core::Result<super::VARIANT_BOOL>;
    fn IsCurrentMediaSupported(&self, recorder: windows_core::Ref<IDiscRecorder2>) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_MediaPhysicallyBlank(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_MediaHeuristicallyBlank(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_SupportedMediaTypes(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDiscFormat2_Vtbl {
    pub const fn new<Identity: IDiscFormat2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsRecorderSupported<Identity: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, recorder: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2_Impl::IsRecorderSupported(this, core::mem::transmute_copy(&recorder)) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsCurrentMediaSupported<Identity: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, recorder: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2_Impl::IsCurrentMediaSupported(this, core::mem::transmute_copy(&recorder)) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_MediaPhysicallyBlank<Identity: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2_Impl::get_MediaPhysicallyBlank(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_MediaHeuristicallyBlank<Identity: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2_Impl::get_MediaHeuristicallyBlank(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SupportedMediaTypes<Identity: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2_Impl::get_SupportedMediaTypes(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            IsRecorderSupported: IsRecorderSupported::<Identity, OFFSET>,
            IsCurrentMediaSupported: IsCurrentMediaSupported::<Identity, OFFSET>,
            get_MediaPhysicallyBlank: get_MediaPhysicallyBlank::<Identity, OFFSET>,
            get_MediaHeuristicallyBlank: get_MediaHeuristicallyBlank::<Identity, OFFSET>,
            get_SupportedMediaTypes: get_SupportedMediaTypes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDiscFormat2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDiscFormat2Data, IDiscFormat2Data_Vtbl, 0x27354153_9f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDiscFormat2Data {
    type Target = IDiscFormat2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDiscFormat2Data, windows_core::IUnknown, super::IDispatch, IDiscFormat2);
#[cfg(feature = "oaidl")]
impl IDiscFormat2Data {
    pub unsafe fn put_Recorder<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_Recorder)(windows_core::Interface::as_raw(self), value.param().abi()) }
    }
    pub unsafe fn get_Recorder(&self) -> windows_core::Result<IDiscRecorder2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Recorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_BufferUnderrunFreeDisabled(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_BufferUnderrunFreeDisabled)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_BufferUnderrunFreeDisabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BufferUnderrunFreeDisabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_PostgapAlreadyInImage(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_PostgapAlreadyInImage)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_PostgapAlreadyInImage(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PostgapAlreadyInImage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentMediaStatus(&self) -> windows_core::Result<IMAPI_FORMAT2_DATA_MEDIA_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentMediaStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_WriteProtectStatus(&self) -> windows_core::Result<IMAPI_MEDIA_WRITE_PROTECT_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WriteProtectStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_TotalSectorsOnMedia(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TotalSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_FreeSectorsOnMedia(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FreeSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_NextWritableAddress(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NextWritableAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_StartAddressOfPreviousSession(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StartAddressOfPreviousSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_LastWrittenAddressOfPreviousSession(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LastWrittenAddressOfPreviousSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_ForceMediaToBeClosed(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ForceMediaToBeClosed)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_ForceMediaToBeClosed(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ForceMediaToBeClosed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_DisableConsumerDvdCompatibilityMode(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DisableConsumerDvdCompatibilityMode)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_DisableConsumerDvdCompatibilityMode(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DisableConsumerDvdCompatibilityMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentPhysicalMediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ClientName(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ClientName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn get_ClientName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ClientName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_RequestedWriteSpeed(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RequestedWriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_RequestedRotationTypeIsPureCAV(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RequestedRotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_CurrentWriteSpeed(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentWriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_CurrentRotationTypeIsPureCAV(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentRotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_SupportedWriteSpeeds(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SupportedWriteSpeeds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_SupportedWriteSpeedDescriptors(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SupportedWriteSpeedDescriptors)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_ForceOverwrite(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ForceOverwrite)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_ForceOverwrite(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ForceOverwrite)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_MultisessionInterfaces(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MultisessionInterfaces)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn Write<P0>(&self, data: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), data.param().abi()) }
    }
    pub unsafe fn CancelWrite(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CancelWrite)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn SetWriteSpeed(&self, requestedsectorspersecond: super::LONG, rotationtypeispurecav: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWriteSpeed)(windows_core::Interface::as_raw(self), requestedsectorspersecond, rotationtypeispurecav) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2Data_Vtbl {
    pub base__: IDiscFormat2_Vtbl,
    pub put_Recorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Recorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub put_BufferUnderrunFreeDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_BufferUnderrunFreeDisabled: usize,
    #[cfg(feature = "wtypes")]
    pub get_BufferUnderrunFreeDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_BufferUnderrunFreeDisabled: usize,
    #[cfg(feature = "wtypes")]
    pub put_PostgapAlreadyInImage: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_PostgapAlreadyInImage: usize,
    #[cfg(feature = "wtypes")]
    pub get_PostgapAlreadyInImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_PostgapAlreadyInImage: usize,
    pub get_CurrentMediaStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_FORMAT2_DATA_MEDIA_STATE) -> windows_core::HRESULT,
    pub get_WriteProtectStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_MEDIA_WRITE_PROTECT_STATE) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_TotalSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_TotalSectorsOnMedia: usize,
    #[cfg(feature = "shared")]
    pub get_FreeSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_FreeSectorsOnMedia: usize,
    #[cfg(feature = "shared")]
    pub get_NextWritableAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_NextWritableAddress: usize,
    #[cfg(feature = "shared")]
    pub get_StartAddressOfPreviousSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_StartAddressOfPreviousSession: usize,
    #[cfg(feature = "shared")]
    pub get_LastWrittenAddressOfPreviousSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_LastWrittenAddressOfPreviousSession: usize,
    #[cfg(feature = "wtypes")]
    pub put_ForceMediaToBeClosed: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_ForceMediaToBeClosed: usize,
    #[cfg(feature = "wtypes")]
    pub get_ForceMediaToBeClosed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_ForceMediaToBeClosed: usize,
    #[cfg(feature = "wtypes")]
    pub put_DisableConsumerDvdCompatibilityMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_DisableConsumerDvdCompatibilityMode: usize,
    #[cfg(feature = "wtypes")]
    pub get_DisableConsumerDvdCompatibilityMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_DisableConsumerDvdCompatibilityMode: usize,
    pub get_CurrentPhysicalMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT,
    pub put_ClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_RequestedWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_RequestedWriteSpeed: usize,
    #[cfg(feature = "wtypes")]
    pub get_RequestedRotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_RequestedRotationTypeIsPureCAV: usize,
    #[cfg(feature = "shared")]
    pub get_CurrentWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_CurrentWriteSpeed: usize,
    #[cfg(feature = "wtypes")]
    pub get_CurrentRotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_CurrentRotationTypeIsPureCAV: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_SupportedWriteSpeeds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_SupportedWriteSpeeds: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_SupportedWriteSpeedDescriptors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_SupportedWriteSpeedDescriptors: usize,
    #[cfg(feature = "wtypes")]
    pub put_ForceOverwrite: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_ForceOverwrite: usize,
    #[cfg(feature = "wtypes")]
    pub get_ForceOverwrite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_ForceOverwrite: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_MultisessionInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_MultisessionInterfaces: usize,
    #[cfg(feature = "objidlbase")]
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    Write: usize,
    pub CancelWrite: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub SetWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    SetWriteSpeed: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDiscFormat2Data_Impl: IDiscFormat2_Impl {
    fn put_Recorder(&self, value: windows_core::Ref<IDiscRecorder2>) -> windows_core::Result<()>;
    fn get_Recorder(&self) -> windows_core::Result<IDiscRecorder2>;
    fn put_BufferUnderrunFreeDisabled(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_BufferUnderrunFreeDisabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_PostgapAlreadyInImage(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_PostgapAlreadyInImage(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_CurrentMediaStatus(&self) -> windows_core::Result<IMAPI_FORMAT2_DATA_MEDIA_STATE>;
    fn get_WriteProtectStatus(&self) -> windows_core::Result<IMAPI_MEDIA_WRITE_PROTECT_STATE>;
    fn get_TotalSectorsOnMedia(&self) -> windows_core::Result<super::LONG>;
    fn get_FreeSectorsOnMedia(&self) -> windows_core::Result<super::LONG>;
    fn get_NextWritableAddress(&self) -> windows_core::Result<super::LONG>;
    fn get_StartAddressOfPreviousSession(&self) -> windows_core::Result<super::LONG>;
    fn get_LastWrittenAddressOfPreviousSession(&self) -> windows_core::Result<super::LONG>;
    fn put_ForceMediaToBeClosed(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_ForceMediaToBeClosed(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_DisableConsumerDvdCompatibilityMode(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_DisableConsumerDvdCompatibilityMode(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn put_ClientName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ClientName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_RequestedWriteSpeed(&self) -> windows_core::Result<super::LONG>;
    fn get_RequestedRotationTypeIsPureCAV(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_CurrentWriteSpeed(&self) -> windows_core::Result<super::LONG>;
    fn get_CurrentRotationTypeIsPureCAV(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_SupportedWriteSpeeds(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn get_SupportedWriteSpeedDescriptors(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn put_ForceOverwrite(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_ForceOverwrite(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_MultisessionInterfaces(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn Write(&self, data: windows_core::Ref<super::IStream>) -> windows_core::Result<()>;
    fn CancelWrite(&self) -> windows_core::Result<()>;
    fn SetWriteSpeed(&self, requestedsectorspersecond: super::LONG, rotationtypeispurecav: super::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDiscFormat2Data_Vtbl {
    pub const fn new<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn put_Recorder<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::put_Recorder(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_Recorder<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::get_Recorder(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_BufferUnderrunFreeDisabled<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::put_BufferUnderrunFreeDisabled(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_BufferUnderrunFreeDisabled<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::get_BufferUnderrunFreeDisabled(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_PostgapAlreadyInImage<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::put_PostgapAlreadyInImage(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_PostgapAlreadyInImage<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::get_PostgapAlreadyInImage(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentMediaStatus<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_FORMAT2_DATA_MEDIA_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::get_CurrentMediaStatus(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_WriteProtectStatus<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_MEDIA_WRITE_PROTECT_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::get_WriteProtectStatus(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_TotalSectorsOnMedia<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::get_TotalSectorsOnMedia(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_FreeSectorsOnMedia<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::get_FreeSectorsOnMedia(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_NextWritableAddress<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::get_NextWritableAddress(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_StartAddressOfPreviousSession<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::get_StartAddressOfPreviousSession(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_LastWrittenAddressOfPreviousSession<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::get_LastWrittenAddressOfPreviousSession(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ForceMediaToBeClosed<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::put_ForceMediaToBeClosed(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_ForceMediaToBeClosed<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::get_ForceMediaToBeClosed(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DisableConsumerDvdCompatibilityMode<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::put_DisableConsumerDvdCompatibilityMode(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_DisableConsumerDvdCompatibilityMode<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::get_DisableConsumerDvdCompatibilityMode(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentPhysicalMediaType<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::get_CurrentPhysicalMediaType(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ClientName<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::put_ClientName(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn get_ClientName<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::get_ClientName(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RequestedWriteSpeed<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::get_RequestedWriteSpeed(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RequestedRotationTypeIsPureCAV<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::get_RequestedRotationTypeIsPureCAV(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentWriteSpeed<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::get_CurrentWriteSpeed(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentRotationTypeIsPureCAV<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::get_CurrentRotationTypeIsPureCAV(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SupportedWriteSpeeds<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedspeeds: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::get_SupportedWriteSpeeds(this) {
                    Ok(ok__) => {
                        supportedspeeds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SupportedWriteSpeedDescriptors<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::get_SupportedWriteSpeedDescriptors(this) {
                    Ok(ok__) => {
                        supportedspeeddescriptors.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ForceOverwrite<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::put_ForceOverwrite(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_ForceOverwrite<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::get_ForceOverwrite(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_MultisessionInterfaces<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::get_MultisessionInterfaces(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Write<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::Write(this, core::mem::transmute_copy(&data)).into()
            }
        }
        unsafe extern "system" fn CancelWrite<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::CancelWrite(this).into()
            }
        }
        unsafe extern "system" fn SetWriteSpeed<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestedsectorspersecond: super::LONG, rotationtypeispurecav: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::SetWriteSpeed(this, core::mem::transmute_copy(&requestedsectorspersecond), core::mem::transmute_copy(&rotationtypeispurecav)).into()
            }
        }
        Self {
            base__: IDiscFormat2_Vtbl::new::<Identity, OFFSET>(),
            put_Recorder: put_Recorder::<Identity, OFFSET>,
            get_Recorder: get_Recorder::<Identity, OFFSET>,
            put_BufferUnderrunFreeDisabled: put_BufferUnderrunFreeDisabled::<Identity, OFFSET>,
            get_BufferUnderrunFreeDisabled: get_BufferUnderrunFreeDisabled::<Identity, OFFSET>,
            put_PostgapAlreadyInImage: put_PostgapAlreadyInImage::<Identity, OFFSET>,
            get_PostgapAlreadyInImage: get_PostgapAlreadyInImage::<Identity, OFFSET>,
            get_CurrentMediaStatus: get_CurrentMediaStatus::<Identity, OFFSET>,
            get_WriteProtectStatus: get_WriteProtectStatus::<Identity, OFFSET>,
            get_TotalSectorsOnMedia: get_TotalSectorsOnMedia::<Identity, OFFSET>,
            get_FreeSectorsOnMedia: get_FreeSectorsOnMedia::<Identity, OFFSET>,
            get_NextWritableAddress: get_NextWritableAddress::<Identity, OFFSET>,
            get_StartAddressOfPreviousSession: get_StartAddressOfPreviousSession::<Identity, OFFSET>,
            get_LastWrittenAddressOfPreviousSession: get_LastWrittenAddressOfPreviousSession::<Identity, OFFSET>,
            put_ForceMediaToBeClosed: put_ForceMediaToBeClosed::<Identity, OFFSET>,
            get_ForceMediaToBeClosed: get_ForceMediaToBeClosed::<Identity, OFFSET>,
            put_DisableConsumerDvdCompatibilityMode: put_DisableConsumerDvdCompatibilityMode::<Identity, OFFSET>,
            get_DisableConsumerDvdCompatibilityMode: get_DisableConsumerDvdCompatibilityMode::<Identity, OFFSET>,
            get_CurrentPhysicalMediaType: get_CurrentPhysicalMediaType::<Identity, OFFSET>,
            put_ClientName: put_ClientName::<Identity, OFFSET>,
            get_ClientName: get_ClientName::<Identity, OFFSET>,
            get_RequestedWriteSpeed: get_RequestedWriteSpeed::<Identity, OFFSET>,
            get_RequestedRotationTypeIsPureCAV: get_RequestedRotationTypeIsPureCAV::<Identity, OFFSET>,
            get_CurrentWriteSpeed: get_CurrentWriteSpeed::<Identity, OFFSET>,
            get_CurrentRotationTypeIsPureCAV: get_CurrentRotationTypeIsPureCAV::<Identity, OFFSET>,
            get_SupportedWriteSpeeds: get_SupportedWriteSpeeds::<Identity, OFFSET>,
            get_SupportedWriteSpeedDescriptors: get_SupportedWriteSpeedDescriptors::<Identity, OFFSET>,
            put_ForceOverwrite: put_ForceOverwrite::<Identity, OFFSET>,
            get_ForceOverwrite: get_ForceOverwrite::<Identity, OFFSET>,
            get_MultisessionInterfaces: get_MultisessionInterfaces::<Identity, OFFSET>,
            Write: Write::<Identity, OFFSET>,
            CancelWrite: CancelWrite::<Identity, OFFSET>,
            SetWriteSpeed: SetWriteSpeed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2Data as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IDiscFormat2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDiscFormat2Data {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDiscFormat2DataEventArgs, IDiscFormat2DataEventArgs_Vtbl, 0x2735413d_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDiscFormat2DataEventArgs {
    type Target = IWriteEngine2EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDiscFormat2DataEventArgs, windows_core::IUnknown, super::IDispatch, IWriteEngine2EventArgs);
#[cfg(feature = "oaidl")]
impl IDiscFormat2DataEventArgs {
    #[cfg(feature = "shared")]
    pub unsafe fn get_ElapsedTime(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ElapsedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_RemainingTime(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RemainingTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_TotalTime(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TotalTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentAction(&self) -> windows_core::Result<IMAPI_FORMAT2_DATA_WRITE_ACTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2DataEventArgs_Vtbl {
    pub base__: IWriteEngine2EventArgs_Vtbl,
    #[cfg(feature = "shared")]
    pub get_ElapsedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_ElapsedTime: usize,
    #[cfg(feature = "shared")]
    pub get_RemainingTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_RemainingTime: usize,
    #[cfg(feature = "shared")]
    pub get_TotalTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_TotalTime: usize,
    pub get_CurrentAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_FORMAT2_DATA_WRITE_ACTION) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDiscFormat2DataEventArgs_Impl: IWriteEngine2EventArgs_Impl {
    fn get_ElapsedTime(&self) -> windows_core::Result<super::LONG>;
    fn get_RemainingTime(&self) -> windows_core::Result<super::LONG>;
    fn get_TotalTime(&self) -> windows_core::Result<super::LONG>;
    fn get_CurrentAction(&self) -> windows_core::Result<IMAPI_FORMAT2_DATA_WRITE_ACTION>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDiscFormat2DataEventArgs_Vtbl {
    pub const fn new<Identity: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ElapsedTime<Identity: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2DataEventArgs_Impl::get_ElapsedTime(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RemainingTime<Identity: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2DataEventArgs_Impl::get_RemainingTime(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_TotalTime<Identity: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2DataEventArgs_Impl::get_TotalTime(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentAction<Identity: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_FORMAT2_DATA_WRITE_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2DataEventArgs_Impl::get_CurrentAction(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWriteEngine2EventArgs_Vtbl::new::<Identity, OFFSET>(),
            get_ElapsedTime: get_ElapsedTime::<Identity, OFFSET>,
            get_RemainingTime: get_RemainingTime::<Identity, OFFSET>,
            get_TotalTime: get_TotalTime::<Identity, OFFSET>,
            get_CurrentAction: get_CurrentAction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2DataEventArgs as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IWriteEngine2EventArgs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDiscFormat2DataEventArgs {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDiscFormat2Erase, IDiscFormat2Erase_Vtbl, 0x27354156_8f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDiscFormat2Erase {
    type Target = IDiscFormat2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDiscFormat2Erase, windows_core::IUnknown, super::IDispatch, IDiscFormat2);
#[cfg(feature = "oaidl")]
impl IDiscFormat2Erase {
    pub unsafe fn put_Recorder<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_Recorder)(windows_core::Interface::as_raw(self), value.param().abi()) }
    }
    pub unsafe fn get_Recorder(&self) -> windows_core::Result<IDiscRecorder2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Recorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_FullErase(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_FullErase)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_FullErase(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FullErase)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentPhysicalMediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ClientName(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ClientName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn get_ClientName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ClientName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn EraseMedia(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EraseMedia)(windows_core::Interface::as_raw(self)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2Erase_Vtbl {
    pub base__: IDiscFormat2_Vtbl,
    pub put_Recorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Recorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub put_FullErase: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_FullErase: usize,
    #[cfg(feature = "wtypes")]
    pub get_FullErase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_FullErase: usize,
    pub get_CurrentPhysicalMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT,
    pub put_ClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EraseMedia: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDiscFormat2Erase_Impl: IDiscFormat2_Impl {
    fn put_Recorder(&self, value: windows_core::Ref<IDiscRecorder2>) -> windows_core::Result<()>;
    fn get_Recorder(&self) -> windows_core::Result<IDiscRecorder2>;
    fn put_FullErase(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_FullErase(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn put_ClientName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ClientName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn EraseMedia(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDiscFormat2Erase_Vtbl {
    pub const fn new<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn put_Recorder<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Erase_Impl::put_Recorder(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_Recorder<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Erase_Impl::get_Recorder(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_FullErase<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Erase_Impl::put_FullErase(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_FullErase<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Erase_Impl::get_FullErase(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentPhysicalMediaType<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Erase_Impl::get_CurrentPhysicalMediaType(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ClientName<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Erase_Impl::put_ClientName(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn get_ClientName<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Erase_Impl::get_ClientName(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EraseMedia<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Erase_Impl::EraseMedia(this).into()
            }
        }
        Self {
            base__: IDiscFormat2_Vtbl::new::<Identity, OFFSET>(),
            put_Recorder: put_Recorder::<Identity, OFFSET>,
            get_Recorder: get_Recorder::<Identity, OFFSET>,
            put_FullErase: put_FullErase::<Identity, OFFSET>,
            get_FullErase: get_FullErase::<Identity, OFFSET>,
            get_CurrentPhysicalMediaType: get_CurrentPhysicalMediaType::<Identity, OFFSET>,
            put_ClientName: put_ClientName::<Identity, OFFSET>,
            get_ClientName: get_ClientName::<Identity, OFFSET>,
            EraseMedia: EraseMedia::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2Erase as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IDiscFormat2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDiscFormat2Erase {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDiscFormat2RawCD, IDiscFormat2RawCD_Vtbl, 0x27354155_8f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDiscFormat2RawCD {
    type Target = IDiscFormat2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDiscFormat2RawCD, windows_core::IUnknown, super::IDispatch, IDiscFormat2);
#[cfg(feature = "oaidl")]
impl IDiscFormat2RawCD {
    pub unsafe fn PrepareMedia(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PrepareMedia)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn WriteMedia<P0>(&self, data: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteMedia)(windows_core::Interface::as_raw(self), data.param().abi()) }
    }
    #[cfg(all(feature = "objidlbase", feature = "shared"))]
    pub unsafe fn WriteMedia2<P0>(&self, data: P0, streamleadinsectors: super::LONG) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteMedia2)(windows_core::Interface::as_raw(self), data.param().abi(), streamleadinsectors) }
    }
    pub unsafe fn CancelWrite(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CancelWrite)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ReleaseMedia(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseMedia)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn SetWriteSpeed(&self, requestedsectorspersecond: super::LONG, rotationtypeispurecav: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWriteSpeed)(windows_core::Interface::as_raw(self), requestedsectorspersecond, rotationtypeispurecav) }
    }
    pub unsafe fn put_Recorder<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_Recorder)(windows_core::Interface::as_raw(self), value.param().abi()) }
    }
    pub unsafe fn get_Recorder(&self) -> windows_core::Result<IDiscRecorder2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Recorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_BufferUnderrunFreeDisabled(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_BufferUnderrunFreeDisabled)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_BufferUnderrunFreeDisabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BufferUnderrunFreeDisabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_StartOfNextSession(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StartOfNextSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_LastPossibleStartOfLeadout(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LastPossibleStartOfLeadout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentPhysicalMediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_SupportedSectorTypes(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SupportedSectorTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_RequestedSectorType(&self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_RequestedSectorType)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn get_RequestedSectorType(&self) -> windows_core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RequestedSectorType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ClientName(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ClientName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn get_ClientName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ClientName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_RequestedWriteSpeed(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RequestedWriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_RequestedRotationTypeIsPureCAV(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RequestedRotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_CurrentWriteSpeed(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentWriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_CurrentRotationTypeIsPureCAV(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentRotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_SupportedWriteSpeeds(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SupportedWriteSpeeds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_SupportedWriteSpeedDescriptors(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SupportedWriteSpeedDescriptors)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2RawCD_Vtbl {
    pub base__: IDiscFormat2_Vtbl,
    pub PrepareMedia: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub WriteMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    WriteMedia: usize,
    #[cfg(all(feature = "objidlbase", feature = "shared"))]
    pub WriteMedia2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidlbase", feature = "shared")))]
    WriteMedia2: usize,
    pub CancelWrite: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseMedia: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub SetWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    SetWriteSpeed: usize,
    pub put_Recorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Recorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub put_BufferUnderrunFreeDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_BufferUnderrunFreeDisabled: usize,
    #[cfg(feature = "wtypes")]
    pub get_BufferUnderrunFreeDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_BufferUnderrunFreeDisabled: usize,
    #[cfg(feature = "shared")]
    pub get_StartOfNextSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_StartOfNextSession: usize,
    #[cfg(feature = "shared")]
    pub get_LastPossibleStartOfLeadout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_LastPossibleStartOfLeadout: usize,
    pub get_CurrentPhysicalMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_SupportedSectorTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_SupportedSectorTypes: usize,
    pub put_RequestedSectorType: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT,
    pub get_RequestedSectorType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT,
    pub put_ClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_RequestedWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_RequestedWriteSpeed: usize,
    #[cfg(feature = "wtypes")]
    pub get_RequestedRotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_RequestedRotationTypeIsPureCAV: usize,
    #[cfg(feature = "shared")]
    pub get_CurrentWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_CurrentWriteSpeed: usize,
    #[cfg(feature = "wtypes")]
    pub get_CurrentRotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_CurrentRotationTypeIsPureCAV: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_SupportedWriteSpeeds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_SupportedWriteSpeeds: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_SupportedWriteSpeedDescriptors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_SupportedWriteSpeedDescriptors: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDiscFormat2RawCD_Impl: IDiscFormat2_Impl {
    fn PrepareMedia(&self) -> windows_core::Result<()>;
    fn WriteMedia(&self, data: windows_core::Ref<super::IStream>) -> windows_core::Result<()>;
    fn WriteMedia2(&self, data: windows_core::Ref<super::IStream>, streamleadinsectors: super::LONG) -> windows_core::Result<()>;
    fn CancelWrite(&self) -> windows_core::Result<()>;
    fn ReleaseMedia(&self) -> windows_core::Result<()>;
    fn SetWriteSpeed(&self, requestedsectorspersecond: super::LONG, rotationtypeispurecav: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn put_Recorder(&self, value: windows_core::Ref<IDiscRecorder2>) -> windows_core::Result<()>;
    fn get_Recorder(&self) -> windows_core::Result<IDiscRecorder2>;
    fn put_BufferUnderrunFreeDisabled(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_BufferUnderrunFreeDisabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_StartOfNextSession(&self) -> windows_core::Result<super::LONG>;
    fn get_LastPossibleStartOfLeadout(&self) -> windows_core::Result<super::LONG>;
    fn get_CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn get_SupportedSectorTypes(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn put_RequestedSectorType(&self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::Result<()>;
    fn get_RequestedSectorType(&self) -> windows_core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE>;
    fn put_ClientName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ClientName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_RequestedWriteSpeed(&self) -> windows_core::Result<super::LONG>;
    fn get_RequestedRotationTypeIsPureCAV(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_CurrentWriteSpeed(&self) -> windows_core::Result<super::LONG>;
    fn get_CurrentRotationTypeIsPureCAV(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_SupportedWriteSpeeds(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn get_SupportedWriteSpeedDescriptors(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDiscFormat2RawCD_Vtbl {
    pub const fn new<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PrepareMedia<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2RawCD_Impl::PrepareMedia(this).into()
            }
        }
        unsafe extern "system" fn WriteMedia<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2RawCD_Impl::WriteMedia(this, core::mem::transmute_copy(&data)).into()
            }
        }
        unsafe extern "system" fn WriteMedia2<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void, streamleadinsectors: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2RawCD_Impl::WriteMedia2(this, core::mem::transmute_copy(&data), core::mem::transmute_copy(&streamleadinsectors)).into()
            }
        }
        unsafe extern "system" fn CancelWrite<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2RawCD_Impl::CancelWrite(this).into()
            }
        }
        unsafe extern "system" fn ReleaseMedia<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2RawCD_Impl::ReleaseMedia(this).into()
            }
        }
        unsafe extern "system" fn SetWriteSpeed<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestedsectorspersecond: super::LONG, rotationtypeispurecav: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2RawCD_Impl::SetWriteSpeed(this, core::mem::transmute_copy(&requestedsectorspersecond), core::mem::transmute_copy(&rotationtypeispurecav)).into()
            }
        }
        unsafe extern "system" fn put_Recorder<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2RawCD_Impl::put_Recorder(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_Recorder<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::get_Recorder(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_BufferUnderrunFreeDisabled<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2RawCD_Impl::put_BufferUnderrunFreeDisabled(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_BufferUnderrunFreeDisabled<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::get_BufferUnderrunFreeDisabled(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_StartOfNextSession<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::get_StartOfNextSession(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_LastPossibleStartOfLeadout<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::get_LastPossibleStartOfLeadout(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentPhysicalMediaType<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::get_CurrentPhysicalMediaType(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SupportedSectorTypes<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::get_SupportedSectorTypes(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_RequestedSectorType<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2RawCD_Impl::put_RequestedSectorType(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_RequestedSectorType<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::get_RequestedSectorType(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ClientName<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2RawCD_Impl::put_ClientName(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn get_ClientName<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::get_ClientName(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RequestedWriteSpeed<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::get_RequestedWriteSpeed(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RequestedRotationTypeIsPureCAV<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::get_RequestedRotationTypeIsPureCAV(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentWriteSpeed<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::get_CurrentWriteSpeed(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentRotationTypeIsPureCAV<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::get_CurrentRotationTypeIsPureCAV(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SupportedWriteSpeeds<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedspeeds: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::get_SupportedWriteSpeeds(this) {
                    Ok(ok__) => {
                        supportedspeeds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SupportedWriteSpeedDescriptors<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::get_SupportedWriteSpeedDescriptors(this) {
                    Ok(ok__) => {
                        supportedspeeddescriptors.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDiscFormat2_Vtbl::new::<Identity, OFFSET>(),
            PrepareMedia: PrepareMedia::<Identity, OFFSET>,
            WriteMedia: WriteMedia::<Identity, OFFSET>,
            WriteMedia2: WriteMedia2::<Identity, OFFSET>,
            CancelWrite: CancelWrite::<Identity, OFFSET>,
            ReleaseMedia: ReleaseMedia::<Identity, OFFSET>,
            SetWriteSpeed: SetWriteSpeed::<Identity, OFFSET>,
            put_Recorder: put_Recorder::<Identity, OFFSET>,
            get_Recorder: get_Recorder::<Identity, OFFSET>,
            put_BufferUnderrunFreeDisabled: put_BufferUnderrunFreeDisabled::<Identity, OFFSET>,
            get_BufferUnderrunFreeDisabled: get_BufferUnderrunFreeDisabled::<Identity, OFFSET>,
            get_StartOfNextSession: get_StartOfNextSession::<Identity, OFFSET>,
            get_LastPossibleStartOfLeadout: get_LastPossibleStartOfLeadout::<Identity, OFFSET>,
            get_CurrentPhysicalMediaType: get_CurrentPhysicalMediaType::<Identity, OFFSET>,
            get_SupportedSectorTypes: get_SupportedSectorTypes::<Identity, OFFSET>,
            put_RequestedSectorType: put_RequestedSectorType::<Identity, OFFSET>,
            get_RequestedSectorType: get_RequestedSectorType::<Identity, OFFSET>,
            put_ClientName: put_ClientName::<Identity, OFFSET>,
            get_ClientName: get_ClientName::<Identity, OFFSET>,
            get_RequestedWriteSpeed: get_RequestedWriteSpeed::<Identity, OFFSET>,
            get_RequestedRotationTypeIsPureCAV: get_RequestedRotationTypeIsPureCAV::<Identity, OFFSET>,
            get_CurrentWriteSpeed: get_CurrentWriteSpeed::<Identity, OFFSET>,
            get_CurrentRotationTypeIsPureCAV: get_CurrentRotationTypeIsPureCAV::<Identity, OFFSET>,
            get_SupportedWriteSpeeds: get_SupportedWriteSpeeds::<Identity, OFFSET>,
            get_SupportedWriteSpeedDescriptors: get_SupportedWriteSpeedDescriptors::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2RawCD as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IDiscFormat2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDiscFormat2RawCD {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDiscFormat2RawCDEventArgs, IDiscFormat2RawCDEventArgs_Vtbl, 0x27354143_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDiscFormat2RawCDEventArgs {
    type Target = IWriteEngine2EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDiscFormat2RawCDEventArgs, windows_core::IUnknown, super::IDispatch, IWriteEngine2EventArgs);
#[cfg(feature = "oaidl")]
impl IDiscFormat2RawCDEventArgs {
    pub unsafe fn get_CurrentAction(&self) -> windows_core::Result<IMAPI_FORMAT2_RAW_CD_WRITE_ACTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_ElapsedTime(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ElapsedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_RemainingTime(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RemainingTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2RawCDEventArgs_Vtbl {
    pub base__: IWriteEngine2EventArgs_Vtbl,
    pub get_CurrentAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_FORMAT2_RAW_CD_WRITE_ACTION) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_ElapsedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_ElapsedTime: usize,
    #[cfg(feature = "shared")]
    pub get_RemainingTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_RemainingTime: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDiscFormat2RawCDEventArgs_Impl: IWriteEngine2EventArgs_Impl {
    fn get_CurrentAction(&self) -> windows_core::Result<IMAPI_FORMAT2_RAW_CD_WRITE_ACTION>;
    fn get_ElapsedTime(&self) -> windows_core::Result<super::LONG>;
    fn get_RemainingTime(&self) -> windows_core::Result<super::LONG>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDiscFormat2RawCDEventArgs_Vtbl {
    pub const fn new<Identity: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_CurrentAction<Identity: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_WRITE_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCDEventArgs_Impl::get_CurrentAction(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ElapsedTime<Identity: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCDEventArgs_Impl::get_ElapsedTime(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RemainingTime<Identity: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCDEventArgs_Impl::get_RemainingTime(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWriteEngine2EventArgs_Vtbl::new::<Identity, OFFSET>(),
            get_CurrentAction: get_CurrentAction::<Identity, OFFSET>,
            get_ElapsedTime: get_ElapsedTime::<Identity, OFFSET>,
            get_RemainingTime: get_RemainingTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2RawCDEventArgs as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IWriteEngine2EventArgs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDiscFormat2RawCDEventArgs {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDiscFormat2TrackAtOnce, IDiscFormat2TrackAtOnce_Vtbl, 0x27354154_8f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDiscFormat2TrackAtOnce {
    type Target = IDiscFormat2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDiscFormat2TrackAtOnce, windows_core::IUnknown, super::IDispatch, IDiscFormat2);
#[cfg(feature = "oaidl")]
impl IDiscFormat2TrackAtOnce {
    pub unsafe fn PrepareMedia(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PrepareMedia)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn AddAudioTrack<P0>(&self, data: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddAudioTrack)(windows_core::Interface::as_raw(self), data.param().abi()) }
    }
    pub unsafe fn CancelAddTrack(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CancelAddTrack)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ReleaseMedia(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseMedia)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub unsafe fn SetWriteSpeed(&self, requestedsectorspersecond: super::LONG, rotationtypeispurecav: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWriteSpeed)(windows_core::Interface::as_raw(self), requestedsectorspersecond, rotationtypeispurecav) }
    }
    pub unsafe fn put_Recorder<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_Recorder)(windows_core::Interface::as_raw(self), value.param().abi()) }
    }
    pub unsafe fn get_Recorder(&self) -> windows_core::Result<IDiscRecorder2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Recorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_BufferUnderrunFreeDisabled(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_BufferUnderrunFreeDisabled)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_BufferUnderrunFreeDisabled(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BufferUnderrunFreeDisabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_NumberOfExistingTracks(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NumberOfExistingTracks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_TotalSectorsOnMedia(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TotalSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_FreeSectorsOnMedia(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FreeSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_UsedSectorsOnMedia(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UsedSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_DoNotFinalizeMedia(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DoNotFinalizeMedia)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_DoNotFinalizeMedia(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DoNotFinalizeMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_ExpectedTableOfContents(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ExpectedTableOfContents)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentPhysicalMediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ClientName(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ClientName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn get_ClientName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ClientName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_RequestedWriteSpeed(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RequestedWriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_RequestedRotationTypeIsPureCAV(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RequestedRotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_CurrentWriteSpeed(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentWriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_CurrentRotationTypeIsPureCAV(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentRotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_SupportedWriteSpeeds(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SupportedWriteSpeeds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_SupportedWriteSpeedDescriptors(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SupportedWriteSpeedDescriptors)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2TrackAtOnce_Vtbl {
    pub base__: IDiscFormat2_Vtbl,
    pub PrepareMedia: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub AddAudioTrack: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    AddAudioTrack: usize,
    pub CancelAddTrack: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseMedia: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "wtypes"))]
    pub SetWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "wtypes")))]
    SetWriteSpeed: usize,
    pub put_Recorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Recorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub put_BufferUnderrunFreeDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_BufferUnderrunFreeDisabled: usize,
    #[cfg(feature = "wtypes")]
    pub get_BufferUnderrunFreeDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_BufferUnderrunFreeDisabled: usize,
    #[cfg(feature = "shared")]
    pub get_NumberOfExistingTracks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_NumberOfExistingTracks: usize,
    #[cfg(feature = "shared")]
    pub get_TotalSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_TotalSectorsOnMedia: usize,
    #[cfg(feature = "shared")]
    pub get_FreeSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_FreeSectorsOnMedia: usize,
    #[cfg(feature = "shared")]
    pub get_UsedSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_UsedSectorsOnMedia: usize,
    #[cfg(feature = "wtypes")]
    pub put_DoNotFinalizeMedia: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_DoNotFinalizeMedia: usize,
    #[cfg(feature = "wtypes")]
    pub get_DoNotFinalizeMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_DoNotFinalizeMedia: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_ExpectedTableOfContents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_ExpectedTableOfContents: usize,
    pub get_CurrentPhysicalMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT,
    pub put_ClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_RequestedWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_RequestedWriteSpeed: usize,
    #[cfg(feature = "wtypes")]
    pub get_RequestedRotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_RequestedRotationTypeIsPureCAV: usize,
    #[cfg(feature = "shared")]
    pub get_CurrentWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_CurrentWriteSpeed: usize,
    #[cfg(feature = "wtypes")]
    pub get_CurrentRotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_CurrentRotationTypeIsPureCAV: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_SupportedWriteSpeeds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_SupportedWriteSpeeds: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_SupportedWriteSpeedDescriptors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_SupportedWriteSpeedDescriptors: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDiscFormat2TrackAtOnce_Impl: IDiscFormat2_Impl {
    fn PrepareMedia(&self) -> windows_core::Result<()>;
    fn AddAudioTrack(&self, data: windows_core::Ref<super::IStream>) -> windows_core::Result<()>;
    fn CancelAddTrack(&self) -> windows_core::Result<()>;
    fn ReleaseMedia(&self) -> windows_core::Result<()>;
    fn SetWriteSpeed(&self, requestedsectorspersecond: super::LONG, rotationtypeispurecav: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn put_Recorder(&self, value: windows_core::Ref<IDiscRecorder2>) -> windows_core::Result<()>;
    fn get_Recorder(&self) -> windows_core::Result<IDiscRecorder2>;
    fn put_BufferUnderrunFreeDisabled(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_BufferUnderrunFreeDisabled(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_NumberOfExistingTracks(&self) -> windows_core::Result<super::LONG>;
    fn get_TotalSectorsOnMedia(&self) -> windows_core::Result<super::LONG>;
    fn get_FreeSectorsOnMedia(&self) -> windows_core::Result<super::LONG>;
    fn get_UsedSectorsOnMedia(&self) -> windows_core::Result<super::LONG>;
    fn put_DoNotFinalizeMedia(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_DoNotFinalizeMedia(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_ExpectedTableOfContents(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn get_CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn put_ClientName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ClientName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_RequestedWriteSpeed(&self) -> windows_core::Result<super::LONG>;
    fn get_RequestedRotationTypeIsPureCAV(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_CurrentWriteSpeed(&self) -> windows_core::Result<super::LONG>;
    fn get_CurrentRotationTypeIsPureCAV(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_SupportedWriteSpeeds(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn get_SupportedWriteSpeedDescriptors(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDiscFormat2TrackAtOnce_Vtbl {
    pub const fn new<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PrepareMedia<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2TrackAtOnce_Impl::PrepareMedia(this).into()
            }
        }
        unsafe extern "system" fn AddAudioTrack<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2TrackAtOnce_Impl::AddAudioTrack(this, core::mem::transmute_copy(&data)).into()
            }
        }
        unsafe extern "system" fn CancelAddTrack<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2TrackAtOnce_Impl::CancelAddTrack(this).into()
            }
        }
        unsafe extern "system" fn ReleaseMedia<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2TrackAtOnce_Impl::ReleaseMedia(this).into()
            }
        }
        unsafe extern "system" fn SetWriteSpeed<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestedsectorspersecond: super::LONG, rotationtypeispurecav: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2TrackAtOnce_Impl::SetWriteSpeed(this, core::mem::transmute_copy(&requestedsectorspersecond), core::mem::transmute_copy(&rotationtypeispurecav)).into()
            }
        }
        unsafe extern "system" fn put_Recorder<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2TrackAtOnce_Impl::put_Recorder(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_Recorder<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::get_Recorder(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_BufferUnderrunFreeDisabled<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2TrackAtOnce_Impl::put_BufferUnderrunFreeDisabled(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_BufferUnderrunFreeDisabled<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::get_BufferUnderrunFreeDisabled(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_NumberOfExistingTracks<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::get_NumberOfExistingTracks(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_TotalSectorsOnMedia<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::get_TotalSectorsOnMedia(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_FreeSectorsOnMedia<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::get_FreeSectorsOnMedia(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_UsedSectorsOnMedia<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::get_UsedSectorsOnMedia(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DoNotFinalizeMedia<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2TrackAtOnce_Impl::put_DoNotFinalizeMedia(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_DoNotFinalizeMedia<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::get_DoNotFinalizeMedia(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ExpectedTableOfContents<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::get_ExpectedTableOfContents(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentPhysicalMediaType<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::get_CurrentPhysicalMediaType(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ClientName<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2TrackAtOnce_Impl::put_ClientName(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn get_ClientName<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::get_ClientName(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RequestedWriteSpeed<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::get_RequestedWriteSpeed(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RequestedRotationTypeIsPureCAV<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::get_RequestedRotationTypeIsPureCAV(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentWriteSpeed<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::get_CurrentWriteSpeed(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentRotationTypeIsPureCAV<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::get_CurrentRotationTypeIsPureCAV(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SupportedWriteSpeeds<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedspeeds: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::get_SupportedWriteSpeeds(this) {
                    Ok(ok__) => {
                        supportedspeeds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SupportedWriteSpeedDescriptors<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::get_SupportedWriteSpeedDescriptors(this) {
                    Ok(ok__) => {
                        supportedspeeddescriptors.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IDiscFormat2_Vtbl::new::<Identity, OFFSET>(),
            PrepareMedia: PrepareMedia::<Identity, OFFSET>,
            AddAudioTrack: AddAudioTrack::<Identity, OFFSET>,
            CancelAddTrack: CancelAddTrack::<Identity, OFFSET>,
            ReleaseMedia: ReleaseMedia::<Identity, OFFSET>,
            SetWriteSpeed: SetWriteSpeed::<Identity, OFFSET>,
            put_Recorder: put_Recorder::<Identity, OFFSET>,
            get_Recorder: get_Recorder::<Identity, OFFSET>,
            put_BufferUnderrunFreeDisabled: put_BufferUnderrunFreeDisabled::<Identity, OFFSET>,
            get_BufferUnderrunFreeDisabled: get_BufferUnderrunFreeDisabled::<Identity, OFFSET>,
            get_NumberOfExistingTracks: get_NumberOfExistingTracks::<Identity, OFFSET>,
            get_TotalSectorsOnMedia: get_TotalSectorsOnMedia::<Identity, OFFSET>,
            get_FreeSectorsOnMedia: get_FreeSectorsOnMedia::<Identity, OFFSET>,
            get_UsedSectorsOnMedia: get_UsedSectorsOnMedia::<Identity, OFFSET>,
            put_DoNotFinalizeMedia: put_DoNotFinalizeMedia::<Identity, OFFSET>,
            get_DoNotFinalizeMedia: get_DoNotFinalizeMedia::<Identity, OFFSET>,
            get_ExpectedTableOfContents: get_ExpectedTableOfContents::<Identity, OFFSET>,
            get_CurrentPhysicalMediaType: get_CurrentPhysicalMediaType::<Identity, OFFSET>,
            put_ClientName: put_ClientName::<Identity, OFFSET>,
            get_ClientName: get_ClientName::<Identity, OFFSET>,
            get_RequestedWriteSpeed: get_RequestedWriteSpeed::<Identity, OFFSET>,
            get_RequestedRotationTypeIsPureCAV: get_RequestedRotationTypeIsPureCAV::<Identity, OFFSET>,
            get_CurrentWriteSpeed: get_CurrentWriteSpeed::<Identity, OFFSET>,
            get_CurrentRotationTypeIsPureCAV: get_CurrentRotationTypeIsPureCAV::<Identity, OFFSET>,
            get_SupportedWriteSpeeds: get_SupportedWriteSpeeds::<Identity, OFFSET>,
            get_SupportedWriteSpeedDescriptors: get_SupportedWriteSpeedDescriptors::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2TrackAtOnce as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IDiscFormat2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDiscFormat2TrackAtOnce {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDiscFormat2TrackAtOnceEventArgs, IDiscFormat2TrackAtOnceEventArgs_Vtbl, 0x27354140_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDiscFormat2TrackAtOnceEventArgs {
    type Target = IWriteEngine2EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDiscFormat2TrackAtOnceEventArgs, windows_core::IUnknown, super::IDispatch, IWriteEngine2EventArgs);
#[cfg(feature = "oaidl")]
impl IDiscFormat2TrackAtOnceEventArgs {
    #[cfg(feature = "shared")]
    pub unsafe fn get_CurrentTrackNumber(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentTrackNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentAction(&self) -> windows_core::Result<IMAPI_FORMAT2_TAO_WRITE_ACTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_ElapsedTime(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ElapsedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_RemainingTime(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RemainingTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2TrackAtOnceEventArgs_Vtbl {
    pub base__: IWriteEngine2EventArgs_Vtbl,
    #[cfg(feature = "shared")]
    pub get_CurrentTrackNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_CurrentTrackNumber: usize,
    pub get_CurrentAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_FORMAT2_TAO_WRITE_ACTION) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_ElapsedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_ElapsedTime: usize,
    #[cfg(feature = "shared")]
    pub get_RemainingTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_RemainingTime: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDiscFormat2TrackAtOnceEventArgs_Impl: IWriteEngine2EventArgs_Impl {
    fn get_CurrentTrackNumber(&self) -> windows_core::Result<super::LONG>;
    fn get_CurrentAction(&self) -> windows_core::Result<IMAPI_FORMAT2_TAO_WRITE_ACTION>;
    fn get_ElapsedTime(&self) -> windows_core::Result<super::LONG>;
    fn get_RemainingTime(&self) -> windows_core::Result<super::LONG>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDiscFormat2TrackAtOnceEventArgs_Vtbl {
    pub const fn new<Identity: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_CurrentTrackNumber<Identity: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnceEventArgs_Impl::get_CurrentTrackNumber(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentAction<Identity: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_FORMAT2_TAO_WRITE_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnceEventArgs_Impl::get_CurrentAction(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ElapsedTime<Identity: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnceEventArgs_Impl::get_ElapsedTime(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RemainingTime<Identity: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnceEventArgs_Impl::get_RemainingTime(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IWriteEngine2EventArgs_Vtbl::new::<Identity, OFFSET>(),
            get_CurrentTrackNumber: get_CurrentTrackNumber::<Identity, OFFSET>,
            get_CurrentAction: get_CurrentAction::<Identity, OFFSET>,
            get_ElapsedTime: get_ElapsedTime::<Identity, OFFSET>,
            get_RemainingTime: get_RemainingTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2TrackAtOnceEventArgs as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IWriteEngine2EventArgs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDiscFormat2TrackAtOnceEventArgs {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDiscMaster2, IDiscMaster2_Vtbl, 0x27354130_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDiscMaster2 {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDiscMaster2, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IDiscMaster2 {
    pub unsafe fn get__NewEnum(&self) -> windows_core::Result<super::IEnumVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get__NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Item(&self, index: super::LONG) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Item)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Count(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsSupportedEnvironment(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsSupportedEnvironment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscMaster2_Vtbl {
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
    #[cfg(feature = "wtypes")]
    pub get_IsSupportedEnvironment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsSupportedEnvironment: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDiscMaster2_Impl: super::IDispatch_Impl {
    fn get__NewEnum(&self) -> windows_core::Result<super::IEnumVARIANT>;
    fn get_Item(&self, index: super::LONG) -> windows_core::Result<windows_core::BSTR>;
    fn get_Count(&self) -> windows_core::Result<super::LONG>;
    fn get_IsSupportedEnvironment(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDiscMaster2_Vtbl {
    pub const fn new<Identity: IDiscMaster2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get__NewEnum<Identity: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscMaster2_Impl::get__NewEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Item<Identity: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::LONG, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscMaster2_Impl::get_Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Count<Identity: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscMaster2_Impl::get_Count(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsSupportedEnvironment<Identity: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscMaster2_Impl::get_IsSupportedEnvironment(this) {
                    Ok(ok__) => {
                        value.write(ok__);
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
            get_IsSupportedEnvironment: get_IsSupportedEnvironment::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscMaster2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDiscMaster2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDiscRecorder2, IDiscRecorder2_Vtbl, 0x27354133_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDiscRecorder2 {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDiscRecorder2, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IDiscRecorder2 {
    pub unsafe fn EjectMedia(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EjectMedia)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CloseTray(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CloseTray)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn AcquireExclusiveAccess(&self, force: super::VARIANT_BOOL, __midl__idiscrecorder20000: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AcquireExclusiveAccess)(windows_core::Interface::as_raw(self), force, core::mem::transmute_copy(__midl__idiscrecorder20000)) }
    }
    pub unsafe fn ReleaseExclusiveAccess(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseExclusiveAccess)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn DisableMcn(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DisableMcn)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn EnableMcn(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EnableMcn)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn InitializeDiscRecorder(&self, recorderuniqueid: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InitializeDiscRecorder)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(recorderuniqueid)) }
    }
    pub unsafe fn get_ActiveDiscRecorder(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ActiveDiscRecorder)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_VendorId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_VendorId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_ProductId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ProductId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_ProductRevision(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ProductRevision)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_VolumeName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_VolumeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_VolumePathNames(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_VolumePathNames)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_DeviceCanLoadMedia(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DeviceCanLoadMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_LegacyDeviceNumber(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LegacyDeviceNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_SupportedFeaturePages(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SupportedFeaturePages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_CurrentFeaturePages(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentFeaturePages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_SupportedProfiles(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SupportedProfiles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_CurrentProfiles(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentProfiles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_SupportedModePages(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SupportedModePages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ExclusiveAccessOwner(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ExclusiveAccessOwner)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscRecorder2_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub EjectMedia: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CloseTray: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub AcquireExclusiveAccess: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    AcquireExclusiveAccess: usize,
    pub ReleaseExclusiveAccess: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableMcn: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableMcn: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitializeDiscRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ActiveDiscRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_VendorId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ProductId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ProductRevision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_VolumeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_VolumePathNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_VolumePathNames: usize,
    #[cfg(feature = "wtypes")]
    pub get_DeviceCanLoadMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_DeviceCanLoadMedia: usize,
    #[cfg(feature = "shared")]
    pub get_LegacyDeviceNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_LegacyDeviceNumber: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_SupportedFeaturePages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_SupportedFeaturePages: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_CurrentFeaturePages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_CurrentFeaturePages: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_SupportedProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_SupportedProfiles: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_CurrentProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_CurrentProfiles: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_SupportedModePages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_SupportedModePages: usize,
    pub get_ExclusiveAccessOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDiscRecorder2_Impl: super::IDispatch_Impl {
    fn EjectMedia(&self) -> windows_core::Result<()>;
    fn CloseTray(&self) -> windows_core::Result<()>;
    fn AcquireExclusiveAccess(&self, force: super::VARIANT_BOOL, __midl__idiscrecorder20000: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ReleaseExclusiveAccess(&self) -> windows_core::Result<()>;
    fn DisableMcn(&self) -> windows_core::Result<()>;
    fn EnableMcn(&self) -> windows_core::Result<()>;
    fn InitializeDiscRecorder(&self, recorderuniqueid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_ActiveDiscRecorder(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_VendorId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_ProductId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_ProductRevision(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_VolumeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_VolumePathNames(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn get_DeviceCanLoadMedia(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_LegacyDeviceNumber(&self) -> windows_core::Result<super::LONG>;
    fn get_SupportedFeaturePages(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn get_CurrentFeaturePages(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn get_SupportedProfiles(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn get_CurrentProfiles(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn get_SupportedModePages(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn get_ExclusiveAccessOwner(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDiscRecorder2_Vtbl {
    pub const fn new<Identity: IDiscRecorder2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn EjectMedia<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2_Impl::EjectMedia(this).into()
            }
        }
        unsafe extern "system" fn CloseTray<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2_Impl::CloseTray(this).into()
            }
        }
        unsafe extern "system" fn AcquireExclusiveAccess<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, force: super::VARIANT_BOOL, __midl__idiscrecorder20000: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2_Impl::AcquireExclusiveAccess(this, core::mem::transmute_copy(&force), core::mem::transmute(&__midl__idiscrecorder20000)).into()
            }
        }
        unsafe extern "system" fn ReleaseExclusiveAccess<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2_Impl::ReleaseExclusiveAccess(this).into()
            }
        }
        unsafe extern "system" fn DisableMcn<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2_Impl::DisableMcn(this).into()
            }
        }
        unsafe extern "system" fn EnableMcn<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2_Impl::EnableMcn(this).into()
            }
        }
        unsafe extern "system" fn InitializeDiscRecorder<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, recorderuniqueid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2_Impl::InitializeDiscRecorder(this, core::mem::transmute(&recorderuniqueid)).into()
            }
        }
        unsafe extern "system" fn get_ActiveDiscRecorder<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::get_ActiveDiscRecorder(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_VendorId<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::get_VendorId(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ProductId<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::get_ProductId(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ProductRevision<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::get_ProductRevision(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_VolumeName<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::get_VolumeName(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_VolumePathNames<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::get_VolumePathNames(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DeviceCanLoadMedia<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::get_DeviceCanLoadMedia(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_LegacyDeviceNumber<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, legacydevicenumber: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::get_LegacyDeviceNumber(this) {
                    Ok(ok__) => {
                        legacydevicenumber.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SupportedFeaturePages<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::get_SupportedFeaturePages(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentFeaturePages<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::get_CurrentFeaturePages(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SupportedProfiles<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::get_SupportedProfiles(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentProfiles<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::get_CurrentProfiles(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SupportedModePages<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::get_SupportedModePages(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ExclusiveAccessOwner<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::get_ExclusiveAccessOwner(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            EjectMedia: EjectMedia::<Identity, OFFSET>,
            CloseTray: CloseTray::<Identity, OFFSET>,
            AcquireExclusiveAccess: AcquireExclusiveAccess::<Identity, OFFSET>,
            ReleaseExclusiveAccess: ReleaseExclusiveAccess::<Identity, OFFSET>,
            DisableMcn: DisableMcn::<Identity, OFFSET>,
            EnableMcn: EnableMcn::<Identity, OFFSET>,
            InitializeDiscRecorder: InitializeDiscRecorder::<Identity, OFFSET>,
            get_ActiveDiscRecorder: get_ActiveDiscRecorder::<Identity, OFFSET>,
            get_VendorId: get_VendorId::<Identity, OFFSET>,
            get_ProductId: get_ProductId::<Identity, OFFSET>,
            get_ProductRevision: get_ProductRevision::<Identity, OFFSET>,
            get_VolumeName: get_VolumeName::<Identity, OFFSET>,
            get_VolumePathNames: get_VolumePathNames::<Identity, OFFSET>,
            get_DeviceCanLoadMedia: get_DeviceCanLoadMedia::<Identity, OFFSET>,
            get_LegacyDeviceNumber: get_LegacyDeviceNumber::<Identity, OFFSET>,
            get_SupportedFeaturePages: get_SupportedFeaturePages::<Identity, OFFSET>,
            get_CurrentFeaturePages: get_CurrentFeaturePages::<Identity, OFFSET>,
            get_SupportedProfiles: get_SupportedProfiles::<Identity, OFFSET>,
            get_CurrentProfiles: get_CurrentProfiles::<Identity, OFFSET>,
            get_SupportedModePages: get_SupportedModePages::<Identity, OFFSET>,
            get_ExclusiveAccessOwner: get_ExclusiveAccessOwner::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscRecorder2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDiscRecorder2 {}
windows_core::imp::define_interface!(IDiscRecorder2Ex, IDiscRecorder2Ex_Vtbl, 0x27354132_7f64_5b0f_8f00_5d77afbe261e);
windows_core::imp::interface_hierarchy!(IDiscRecorder2Ex, windows_core::IUnknown);
impl IDiscRecorder2Ex {
    #[cfg(feature = "shared")]
    pub unsafe fn SendCommandNoData(&self, cdb: &[super::BYTE], sensebuffer: *mut super::BYTE, timeout: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SendCommandNoData)(windows_core::Interface::as_raw(self), cdb.as_ptr(), cdb.len().try_into().unwrap(), sensebuffer as _, timeout) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SendCommandSendDataToDevice(&self, cdb: &[super::BYTE], sensebuffer: *mut super::BYTE, timeout: super::ULONG, buffer: &[super::BYTE]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SendCommandSendDataToDevice)(windows_core::Interface::as_raw(self), cdb.as_ptr(), cdb.len().try_into().unwrap(), sensebuffer as _, timeout, buffer.as_ptr(), buffer.len().try_into().unwrap()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SendCommandGetDataFromDevice(&self, cdb: &[super::BYTE], sensebuffer: *mut super::BYTE, timeout: super::ULONG, buffer: *mut super::BYTE, buffersize: ULONG_IMAPI2_NONZERO, bufferfetched: *mut ULONG_IMAPI2_NOT_NEGATIVE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SendCommandGetDataFromDevice)(windows_core::Interface::as_raw(self), cdb.as_ptr(), cdb.len().try_into().unwrap(), sensebuffer as _, timeout, buffer as _, buffersize, bufferfetched as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn ReadDvdStructure(&self, format: super::ULONG, address: super::ULONG, layer: super::ULONG, agid: super::ULONG, data: *mut *mut super::BYTE, count: *mut ULONG_IMAPI2_DVD_STRUCTURE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReadDvdStructure)(windows_core::Interface::as_raw(self), format, address, layer, agid, data as _, count as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SendDvdStructure(&self, format: super::ULONG, data: &[super::BYTE]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SendDvdStructure)(windows_core::Interface::as_raw(self), format, data.as_ptr(), data.len().try_into().unwrap()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetAdapterDescriptor(&self, data: *mut *mut super::BYTE, bytesize: *mut ULONG_IMAPI2_ADAPTER_DESCRIPTOR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterDescriptor)(windows_core::Interface::as_raw(self), data as _, bytesize as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDeviceDescriptor(&self, data: *mut *mut super::BYTE, bytesize: *mut ULONG_IMAPI2_DEVICE_DESCRIPTOR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceDescriptor)(windows_core::Interface::as_raw(self), data as _, bytesize as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetDiscInformation(&self, discinformation: *mut *mut super::BYTE, bytesize: *mut ULONG_IMAPI2_DISC_INFORMATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDiscInformation)(windows_core::Interface::as_raw(self), discinformation as _, bytesize as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetTrackInformation(&self, address: super::ULONG, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut super::BYTE, bytesize: *mut ULONG_IMAPI2_TRACK_INFORMATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTrackInformation)(windows_core::Interface::as_raw(self), address, addresstype, trackinformation as _, bytesize as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetFeaturePage(&self, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: super::BOOLEAN, featuredata: *mut *mut super::BYTE, bytesize: *mut ULONG_IMAPI2_FEATURE_PAGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFeaturePage)(windows_core::Interface::as_raw(self), requestedfeature, currentfeatureonly, featuredata as _, bytesize as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetModePage(&self, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut super::BYTE, bytesize: *mut ULONG_IMAPI2_MODE_PAGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetModePage)(windows_core::Interface::as_raw(self), requestedmodepage, requesttype, modepagedata as _, bytesize as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetModePage(&self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: &[super::BYTE]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetModePage)(windows_core::Interface::as_raw(self), requesttype, data.as_ptr(), data.len().try_into().unwrap()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSupportedFeaturePages(&self, currentfeatureonly: super::BOOLEAN, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut ULONG_IMAPI2_ALL_FEATURE_PAGES) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSupportedFeaturePages)(windows_core::Interface::as_raw(self), currentfeatureonly, featuredata as _, bytesize as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetSupportedProfiles(&self, currentonly: super::BOOLEAN, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut ULONG_IMAPI2_ALL_PROFILES) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSupportedProfiles)(windows_core::Interface::as_raw(self), currentonly, profiletypes as _, validprofiles as _) }
    }
    pub unsafe fn GetSupportedModePages(&self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut ULONG_IMAPI2_ALL_MODE_PAGES) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSupportedModePages)(windows_core::Interface::as_raw(self), requesttype, modepagetypes as _, validpages as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetByteAlignmentMask(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetByteAlignmentMask)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMaximumNonPageAlignedTransferSize(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaximumNonPageAlignedTransferSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetMaximumPageAlignedTransferSize(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaximumPageAlignedTransferSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDiscRecorder2Ex_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub SendCommandNoData: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::BYTE, super::ULONG, *mut super::BYTE, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SendCommandNoData: usize,
    #[cfg(feature = "shared")]
    pub SendCommandSendDataToDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::BYTE, super::ULONG, *mut super::BYTE, super::ULONG, *const super::BYTE, ULONG_IMAPI2_NONZERO) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SendCommandSendDataToDevice: usize,
    #[cfg(feature = "shared")]
    pub SendCommandGetDataFromDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::BYTE, super::ULONG, *mut super::BYTE, super::ULONG, *mut super::BYTE, ULONG_IMAPI2_NONZERO, *mut ULONG_IMAPI2_NOT_NEGATIVE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SendCommandGetDataFromDevice: usize,
    #[cfg(feature = "shared")]
    pub ReadDvdStructure: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, super::ULONG, super::ULONG, super::ULONG, *mut *mut super::BYTE, *mut ULONG_IMAPI2_DVD_STRUCTURE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ReadDvdStructure: usize,
    #[cfg(feature = "shared")]
    pub SendDvdStructure: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, *const super::BYTE, ULONG_IMAPI2_DVD_STRUCTURE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SendDvdStructure: usize,
    #[cfg(feature = "shared")]
    pub GetAdapterDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::BYTE, *mut ULONG_IMAPI2_ADAPTER_DESCRIPTOR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetAdapterDescriptor: usize,
    #[cfg(feature = "shared")]
    pub GetDeviceDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::BYTE, *mut ULONG_IMAPI2_DEVICE_DESCRIPTOR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDeviceDescriptor: usize,
    #[cfg(feature = "shared")]
    pub GetDiscInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::BYTE, *mut ULONG_IMAPI2_DISC_INFORMATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetDiscInformation: usize,
    #[cfg(feature = "shared")]
    pub GetTrackInformation: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG, IMAPI_READ_TRACK_ADDRESS_TYPE, *mut *mut super::BYTE, *mut ULONG_IMAPI2_TRACK_INFORMATION) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetTrackInformation: usize,
    #[cfg(feature = "shared")]
    pub GetFeaturePage: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_FEATURE_PAGE_TYPE, super::BOOLEAN, *mut *mut super::BYTE, *mut ULONG_IMAPI2_FEATURE_PAGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetFeaturePage: usize,
    #[cfg(feature = "shared")]
    pub GetModePage: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_MODE_PAGE_TYPE, IMAPI_MODE_PAGE_REQUEST_TYPE, *mut *mut super::BYTE, *mut ULONG_IMAPI2_MODE_PAGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetModePage: usize,
    #[cfg(feature = "shared")]
    pub SetModePage: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_MODE_PAGE_REQUEST_TYPE, *const super::BYTE, ULONG_IMAPI2_MODE_PAGE) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetModePage: usize,
    #[cfg(feature = "shared")]
    pub GetSupportedFeaturePages: unsafe extern "system" fn(*mut core::ffi::c_void, super::BOOLEAN, *mut *mut IMAPI_FEATURE_PAGE_TYPE, *mut ULONG_IMAPI2_ALL_FEATURE_PAGES) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSupportedFeaturePages: usize,
    #[cfg(feature = "shared")]
    pub GetSupportedProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, super::BOOLEAN, *mut *mut IMAPI_PROFILE_TYPE, *mut ULONG_IMAPI2_ALL_PROFILES) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetSupportedProfiles: usize,
    pub GetSupportedModePages: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_MODE_PAGE_REQUEST_TYPE, *mut *mut IMAPI_MODE_PAGE_TYPE, *mut ULONG_IMAPI2_ALL_MODE_PAGES) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetByteAlignmentMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetByteAlignmentMask: usize,
    #[cfg(feature = "shared")]
    pub GetMaximumNonPageAlignedTransferSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMaximumNonPageAlignedTransferSize: usize,
    #[cfg(feature = "shared")]
    pub GetMaximumPageAlignedTransferSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetMaximumPageAlignedTransferSize: usize,
}
#[cfg(feature = "shared")]
pub trait IDiscRecorder2Ex_Impl: windows_core::IUnknownImpl {
    fn SendCommandNoData(&self, cdb: *const super::BYTE, cdbsize: super::ULONG, sensebuffer: *mut super::BYTE, timeout: super::ULONG) -> windows_core::Result<()>;
    fn SendCommandSendDataToDevice(&self, cdb: *const super::BYTE, cdbsize: super::ULONG, sensebuffer: *mut super::BYTE, timeout: super::ULONG, buffer: *const super::BYTE, buffersize: ULONG_IMAPI2_NONZERO) -> windows_core::Result<()>;
    fn SendCommandGetDataFromDevice(&self, cdb: *const super::BYTE, cdbsize: super::ULONG, sensebuffer: *mut super::BYTE, timeout: super::ULONG, buffer: *mut super::BYTE, buffersize: ULONG_IMAPI2_NONZERO, bufferfetched: *mut ULONG_IMAPI2_NOT_NEGATIVE) -> windows_core::Result<()>;
    fn ReadDvdStructure(&self, format: super::ULONG, address: super::ULONG, layer: super::ULONG, agid: super::ULONG, data: *mut *mut super::BYTE, count: *mut ULONG_IMAPI2_DVD_STRUCTURE) -> windows_core::Result<()>;
    fn SendDvdStructure(&self, format: super::ULONG, data: *const super::BYTE, count: ULONG_IMAPI2_DVD_STRUCTURE) -> windows_core::Result<()>;
    fn GetAdapterDescriptor(&self, data: *mut *mut super::BYTE, bytesize: *mut ULONG_IMAPI2_ADAPTER_DESCRIPTOR) -> windows_core::Result<()>;
    fn GetDeviceDescriptor(&self, data: *mut *mut super::BYTE, bytesize: *mut ULONG_IMAPI2_DEVICE_DESCRIPTOR) -> windows_core::Result<()>;
    fn GetDiscInformation(&self, discinformation: *mut *mut super::BYTE, bytesize: *mut ULONG_IMAPI2_DISC_INFORMATION) -> windows_core::Result<()>;
    fn GetTrackInformation(&self, address: super::ULONG, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut super::BYTE, bytesize: *mut ULONG_IMAPI2_TRACK_INFORMATION) -> windows_core::Result<()>;
    fn GetFeaturePage(&self, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: super::BOOLEAN, featuredata: *mut *mut super::BYTE, bytesize: *mut ULONG_IMAPI2_FEATURE_PAGE) -> windows_core::Result<()>;
    fn GetModePage(&self, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut super::BYTE, bytesize: *mut ULONG_IMAPI2_MODE_PAGE) -> windows_core::Result<()>;
    fn SetModePage(&self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: *const super::BYTE, bytesize: ULONG_IMAPI2_MODE_PAGE) -> windows_core::Result<()>;
    fn GetSupportedFeaturePages(&self, currentfeatureonly: super::BOOLEAN, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut ULONG_IMAPI2_ALL_FEATURE_PAGES) -> windows_core::Result<()>;
    fn GetSupportedProfiles(&self, currentonly: super::BOOLEAN, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut ULONG_IMAPI2_ALL_PROFILES) -> windows_core::Result<()>;
    fn GetSupportedModePages(&self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut ULONG_IMAPI2_ALL_MODE_PAGES) -> windows_core::Result<()>;
    fn GetByteAlignmentMask(&self) -> windows_core::Result<super::ULONG>;
    fn GetMaximumNonPageAlignedTransferSize(&self) -> windows_core::Result<super::ULONG>;
    fn GetMaximumPageAlignedTransferSize(&self) -> windows_core::Result<super::ULONG>;
}
#[cfg(feature = "shared")]
impl IDiscRecorder2Ex_Vtbl {
    pub const fn new<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SendCommandNoData<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cdb: *const super::BYTE, cdbsize: super::ULONG, sensebuffer: *mut super::BYTE, timeout: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::SendCommandNoData(this, core::mem::transmute_copy(&cdb), core::mem::transmute_copy(&cdbsize), core::mem::transmute_copy(&sensebuffer), core::mem::transmute_copy(&timeout)).into()
            }
        }
        unsafe extern "system" fn SendCommandSendDataToDevice<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cdb: *const super::BYTE, cdbsize: super::ULONG, sensebuffer: *mut super::BYTE, timeout: super::ULONG, buffer: *const super::BYTE, buffersize: ULONG_IMAPI2_NONZERO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::SendCommandSendDataToDevice(this, core::mem::transmute_copy(&cdb), core::mem::transmute_copy(&cdbsize), core::mem::transmute_copy(&sensebuffer), core::mem::transmute_copy(&timeout), core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&buffersize)).into()
            }
        }
        unsafe extern "system" fn SendCommandGetDataFromDevice<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cdb: *const super::BYTE, cdbsize: super::ULONG, sensebuffer: *mut super::BYTE, timeout: super::ULONG, buffer: *mut super::BYTE, buffersize: ULONG_IMAPI2_NONZERO, bufferfetched: *mut ULONG_IMAPI2_NOT_NEGATIVE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::SendCommandGetDataFromDevice(this, core::mem::transmute_copy(&cdb), core::mem::transmute_copy(&cdbsize), core::mem::transmute_copy(&sensebuffer), core::mem::transmute_copy(&timeout), core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&buffersize), core::mem::transmute_copy(&bufferfetched)).into()
            }
        }
        unsafe extern "system" fn ReadDvdStructure<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: super::ULONG, address: super::ULONG, layer: super::ULONG, agid: super::ULONG, data: *mut *mut super::BYTE, count: *mut ULONG_IMAPI2_DVD_STRUCTURE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::ReadDvdStructure(this, core::mem::transmute_copy(&format), core::mem::transmute_copy(&address), core::mem::transmute_copy(&layer), core::mem::transmute_copy(&agid), core::mem::transmute_copy(&data), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn SendDvdStructure<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: super::ULONG, data: *const super::BYTE, count: ULONG_IMAPI2_DVD_STRUCTURE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::SendDvdStructure(this, core::mem::transmute_copy(&format), core::mem::transmute_copy(&data), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetAdapterDescriptor<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut *mut super::BYTE, bytesize: *mut ULONG_IMAPI2_ADAPTER_DESCRIPTOR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::GetAdapterDescriptor(this, core::mem::transmute_copy(&data), core::mem::transmute_copy(&bytesize)).into()
            }
        }
        unsafe extern "system" fn GetDeviceDescriptor<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut *mut super::BYTE, bytesize: *mut ULONG_IMAPI2_DEVICE_DESCRIPTOR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::GetDeviceDescriptor(this, core::mem::transmute_copy(&data), core::mem::transmute_copy(&bytesize)).into()
            }
        }
        unsafe extern "system" fn GetDiscInformation<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, discinformation: *mut *mut super::BYTE, bytesize: *mut ULONG_IMAPI2_DISC_INFORMATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::GetDiscInformation(this, core::mem::transmute_copy(&discinformation), core::mem::transmute_copy(&bytesize)).into()
            }
        }
        unsafe extern "system" fn GetTrackInformation<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, address: super::ULONG, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut super::BYTE, bytesize: *mut ULONG_IMAPI2_TRACK_INFORMATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::GetTrackInformation(this, core::mem::transmute_copy(&address), core::mem::transmute_copy(&addresstype), core::mem::transmute_copy(&trackinformation), core::mem::transmute_copy(&bytesize)).into()
            }
        }
        unsafe extern "system" fn GetFeaturePage<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: super::BOOLEAN, featuredata: *mut *mut super::BYTE, bytesize: *mut ULONG_IMAPI2_FEATURE_PAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::GetFeaturePage(this, core::mem::transmute_copy(&requestedfeature), core::mem::transmute_copy(&currentfeatureonly), core::mem::transmute_copy(&featuredata), core::mem::transmute_copy(&bytesize)).into()
            }
        }
        unsafe extern "system" fn GetModePage<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut super::BYTE, bytesize: *mut ULONG_IMAPI2_MODE_PAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::GetModePage(this, core::mem::transmute_copy(&requestedmodepage), core::mem::transmute_copy(&requesttype), core::mem::transmute_copy(&modepagedata), core::mem::transmute_copy(&bytesize)).into()
            }
        }
        unsafe extern "system" fn SetModePage<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: *const super::BYTE, bytesize: ULONG_IMAPI2_MODE_PAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::SetModePage(this, core::mem::transmute_copy(&requesttype), core::mem::transmute_copy(&data), core::mem::transmute_copy(&bytesize)).into()
            }
        }
        unsafe extern "system" fn GetSupportedFeaturePages<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentfeatureonly: super::BOOLEAN, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut ULONG_IMAPI2_ALL_FEATURE_PAGES) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::GetSupportedFeaturePages(this, core::mem::transmute_copy(&currentfeatureonly), core::mem::transmute_copy(&featuredata), core::mem::transmute_copy(&bytesize)).into()
            }
        }
        unsafe extern "system" fn GetSupportedProfiles<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentonly: super::BOOLEAN, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut ULONG_IMAPI2_ALL_PROFILES) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::GetSupportedProfiles(this, core::mem::transmute_copy(&currentonly), core::mem::transmute_copy(&profiletypes), core::mem::transmute_copy(&validprofiles)).into()
            }
        }
        unsafe extern "system" fn GetSupportedModePages<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut ULONG_IMAPI2_ALL_MODE_PAGES) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::GetSupportedModePages(this, core::mem::transmute_copy(&requesttype), core::mem::transmute_copy(&modepagetypes), core::mem::transmute_copy(&validpages)).into()
            }
        }
        unsafe extern "system" fn GetByteAlignmentMask<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2Ex_Impl::GetByteAlignmentMask(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaximumNonPageAlignedTransferSize<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2Ex_Impl::GetMaximumNonPageAlignedTransferSize(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetMaximumPageAlignedTransferSize<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2Ex_Impl::GetMaximumPageAlignedTransferSize(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SendCommandNoData: SendCommandNoData::<Identity, OFFSET>,
            SendCommandSendDataToDevice: SendCommandSendDataToDevice::<Identity, OFFSET>,
            SendCommandGetDataFromDevice: SendCommandGetDataFromDevice::<Identity, OFFSET>,
            ReadDvdStructure: ReadDvdStructure::<Identity, OFFSET>,
            SendDvdStructure: SendDvdStructure::<Identity, OFFSET>,
            GetAdapterDescriptor: GetAdapterDescriptor::<Identity, OFFSET>,
            GetDeviceDescriptor: GetDeviceDescriptor::<Identity, OFFSET>,
            GetDiscInformation: GetDiscInformation::<Identity, OFFSET>,
            GetTrackInformation: GetTrackInformation::<Identity, OFFSET>,
            GetFeaturePage: GetFeaturePage::<Identity, OFFSET>,
            GetModePage: GetModePage::<Identity, OFFSET>,
            SetModePage: SetModePage::<Identity, OFFSET>,
            GetSupportedFeaturePages: GetSupportedFeaturePages::<Identity, OFFSET>,
            GetSupportedProfiles: GetSupportedProfiles::<Identity, OFFSET>,
            GetSupportedModePages: GetSupportedModePages::<Identity, OFFSET>,
            GetByteAlignmentMask: GetByteAlignmentMask::<Identity, OFFSET>,
            GetMaximumNonPageAlignedTransferSize: GetMaximumNonPageAlignedTransferSize::<Identity, OFFSET>,
            GetMaximumPageAlignedTransferSize: GetMaximumPageAlignedTransferSize::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscRecorder2Ex as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IDiscRecorder2Ex {}
pub const IMAPI2_DEFAULT_COMMAND_TIMEOUT: i32 = 10;
pub const IMAPILib2_MajorVersion: i32 = 1;
pub const IMAPILib2_MinorVersion: i32 = 0;
pub const IMAPI_BURN_VERIFICATION_FULL: IMAPI_BURN_VERIFICATION_LEVEL = 2;
pub type IMAPI_BURN_VERIFICATION_LEVEL = i32;
pub const IMAPI_BURN_VERIFICATION_NONE: IMAPI_BURN_VERIFICATION_LEVEL = 0;
pub const IMAPI_BURN_VERIFICATION_QUICK: IMAPI_BURN_VERIFICATION_LEVEL = 1;
pub const IMAPI_CD_SECTOR_AUDIO: IMAPI_CD_SECTOR_TYPE = 0;
pub const IMAPI_CD_SECTOR_MODE1: IMAPI_CD_SECTOR_TYPE = 2;
pub const IMAPI_CD_SECTOR_MODE1RAW: IMAPI_CD_SECTOR_TYPE = 6;
pub const IMAPI_CD_SECTOR_MODE2FORM0: IMAPI_CD_SECTOR_TYPE = 3;
pub const IMAPI_CD_SECTOR_MODE2FORM0RAW: IMAPI_CD_SECTOR_TYPE = 7;
pub const IMAPI_CD_SECTOR_MODE2FORM1: IMAPI_CD_SECTOR_TYPE = 4;
pub const IMAPI_CD_SECTOR_MODE2FORM1RAW: IMAPI_CD_SECTOR_TYPE = 8;
pub const IMAPI_CD_SECTOR_MODE2FORM2: IMAPI_CD_SECTOR_TYPE = 5;
pub const IMAPI_CD_SECTOR_MODE2FORM2RAW: IMAPI_CD_SECTOR_TYPE = 9;
pub const IMAPI_CD_SECTOR_MODE_ZERO: IMAPI_CD_SECTOR_TYPE = 1;
pub type IMAPI_CD_SECTOR_TYPE = i32;
pub const IMAPI_CD_TRACK_DIGITAL_COPY_PERMITTED: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING = 0;
pub const IMAPI_CD_TRACK_DIGITAL_COPY_PROHIBITED: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING = 1;
pub const IMAPI_CD_TRACK_DIGITAL_COPY_SCMS: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING = 2;
pub type IMAPI_CD_TRACK_DIGITAL_COPY_SETTING = i32;
pub type IMAPI_FEATURE_PAGE_TYPE = i32;
pub const IMAPI_FEATURE_PAGE_TYPE_AACS: IMAPI_FEATURE_PAGE_TYPE = 269;
pub const IMAPI_FEATURE_PAGE_TYPE_BD_PSEUDO_OVERWRITE: IMAPI_FEATURE_PAGE_TYPE = 56;
pub const IMAPI_FEATURE_PAGE_TYPE_BD_READ: IMAPI_FEATURE_PAGE_TYPE = 64;
pub const IMAPI_FEATURE_PAGE_TYPE_BD_WRITE: IMAPI_FEATURE_PAGE_TYPE = 65;
pub const IMAPI_FEATURE_PAGE_TYPE_CDRW_CAV_WRITE: IMAPI_FEATURE_PAGE_TYPE = 39;
pub const IMAPI_FEATURE_PAGE_TYPE_CD_ANALOG_PLAY: IMAPI_FEATURE_PAGE_TYPE = 259;
pub const IMAPI_FEATURE_PAGE_TYPE_CD_MASTERING: IMAPI_FEATURE_PAGE_TYPE = 46;
pub const IMAPI_FEATURE_PAGE_TYPE_CD_MULTIREAD: IMAPI_FEATURE_PAGE_TYPE = 29;
pub const IMAPI_FEATURE_PAGE_TYPE_CD_READ: IMAPI_FEATURE_PAGE_TYPE = 30;
pub const IMAPI_FEATURE_PAGE_TYPE_CD_RW_MEDIA_WRITE_SUPPORT: IMAPI_FEATURE_PAGE_TYPE = 55;
pub const IMAPI_FEATURE_PAGE_TYPE_CD_TRACK_AT_ONCE: IMAPI_FEATURE_PAGE_TYPE = 45;
pub const IMAPI_FEATURE_PAGE_TYPE_CORE: IMAPI_FEATURE_PAGE_TYPE = 1;
pub const IMAPI_FEATURE_PAGE_TYPE_DISC_CONTROL_BLOCKS: IMAPI_FEATURE_PAGE_TYPE = 266;
pub const IMAPI_FEATURE_PAGE_TYPE_DOUBLE_DENSITY_CD_READ: IMAPI_FEATURE_PAGE_TYPE = 48;
pub const IMAPI_FEATURE_PAGE_TYPE_DOUBLE_DENSITY_CD_RW_WRITE: IMAPI_FEATURE_PAGE_TYPE = 50;
pub const IMAPI_FEATURE_PAGE_TYPE_DOUBLE_DENSITY_CD_R_WRITE: IMAPI_FEATURE_PAGE_TYPE = 49;
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_CPRM: IMAPI_FEATURE_PAGE_TYPE = 267;
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_CSS: IMAPI_FEATURE_PAGE_TYPE = 262;
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_DASH_WRITE: IMAPI_FEATURE_PAGE_TYPE = 47;
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_PLUS_R: IMAPI_FEATURE_PAGE_TYPE = 43;
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_PLUS_RW: IMAPI_FEATURE_PAGE_TYPE = 42;
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_PLUS_R_DUAL_LAYER: IMAPI_FEATURE_PAGE_TYPE = 59;
pub const IMAPI_FEATURE_PAGE_TYPE_DVD_READ: IMAPI_FEATURE_PAGE_TYPE = 31;
pub const IMAPI_FEATURE_PAGE_TYPE_EMBEDDED_CHANGER: IMAPI_FEATURE_PAGE_TYPE = 258;
pub const IMAPI_FEATURE_PAGE_TYPE_ENHANCED_DEFECT_REPORTING: IMAPI_FEATURE_PAGE_TYPE = 41;
pub const IMAPI_FEATURE_PAGE_TYPE_FIRMWARE_INFORMATION: IMAPI_FEATURE_PAGE_TYPE = 268;
pub const IMAPI_FEATURE_PAGE_TYPE_FORMATTABLE: IMAPI_FEATURE_PAGE_TYPE = 35;
pub const IMAPI_FEATURE_PAGE_TYPE_HARDWARE_DEFECT_MANAGEMENT: IMAPI_FEATURE_PAGE_TYPE = 36;
pub const IMAPI_FEATURE_PAGE_TYPE_HD_DVD_READ: IMAPI_FEATURE_PAGE_TYPE = 80;
pub const IMAPI_FEATURE_PAGE_TYPE_HD_DVD_WRITE: IMAPI_FEATURE_PAGE_TYPE = 81;
pub const IMAPI_FEATURE_PAGE_TYPE_INCREMENTAL_STREAMING_WRITABLE: IMAPI_FEATURE_PAGE_TYPE = 33;
pub const IMAPI_FEATURE_PAGE_TYPE_LAYER_JUMP_RECORDING: IMAPI_FEATURE_PAGE_TYPE = 51;
pub const IMAPI_FEATURE_PAGE_TYPE_LOGICAL_UNIT_SERIAL_NUMBER: IMAPI_FEATURE_PAGE_TYPE = 264;
pub const IMAPI_FEATURE_PAGE_TYPE_MEDIA_SERIAL_NUMBER: IMAPI_FEATURE_PAGE_TYPE = 265;
pub const IMAPI_FEATURE_PAGE_TYPE_MICROCODE_UPDATE: IMAPI_FEATURE_PAGE_TYPE = 260;
pub const IMAPI_FEATURE_PAGE_TYPE_MORPHING: IMAPI_FEATURE_PAGE_TYPE = 2;
pub const IMAPI_FEATURE_PAGE_TYPE_MRW: IMAPI_FEATURE_PAGE_TYPE = 40;
pub const IMAPI_FEATURE_PAGE_TYPE_POWER_MANAGEMENT: IMAPI_FEATURE_PAGE_TYPE = 256;
pub const IMAPI_FEATURE_PAGE_TYPE_PROFILE_LIST: IMAPI_FEATURE_PAGE_TYPE = 0;
pub const IMAPI_FEATURE_PAGE_TYPE_RANDOMLY_READABLE: IMAPI_FEATURE_PAGE_TYPE = 16;
pub const IMAPI_FEATURE_PAGE_TYPE_RANDOMLY_WRITABLE: IMAPI_FEATURE_PAGE_TYPE = 32;
pub const IMAPI_FEATURE_PAGE_TYPE_REAL_TIME_STREAMING: IMAPI_FEATURE_PAGE_TYPE = 263;
pub const IMAPI_FEATURE_PAGE_TYPE_REMOVABLE_MEDIUM: IMAPI_FEATURE_PAGE_TYPE = 3;
pub const IMAPI_FEATURE_PAGE_TYPE_RESTRICTED_OVERWRITE: IMAPI_FEATURE_PAGE_TYPE = 38;
pub const IMAPI_FEATURE_PAGE_TYPE_RIGID_RESTRICTED_OVERWRITE: IMAPI_FEATURE_PAGE_TYPE = 44;
pub const IMAPI_FEATURE_PAGE_TYPE_SECTOR_ERASABLE: IMAPI_FEATURE_PAGE_TYPE = 34;
pub const IMAPI_FEATURE_PAGE_TYPE_SMART: IMAPI_FEATURE_PAGE_TYPE = 257;
pub const IMAPI_FEATURE_PAGE_TYPE_TIMEOUT: IMAPI_FEATURE_PAGE_TYPE = 261;
pub const IMAPI_FEATURE_PAGE_TYPE_VCPS: IMAPI_FEATURE_PAGE_TYPE = 272;
pub const IMAPI_FEATURE_PAGE_TYPE_WRITE_ONCE: IMAPI_FEATURE_PAGE_TYPE = 37;
pub const IMAPI_FEATURE_PAGE_TYPE_WRITE_PROTECT: IMAPI_FEATURE_PAGE_TYPE = 4;
pub type IMAPI_FORMAT2_DATA_MEDIA_STATE = i32;
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_APPENDABLE: IMAPI_FORMAT2_DATA_MEDIA_STATE = 4;
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_BLANK: IMAPI_FORMAT2_DATA_MEDIA_STATE = 2;
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_DAMAGED: IMAPI_FORMAT2_DATA_MEDIA_STATE = 1024;
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_ERASE_REQUIRED: IMAPI_FORMAT2_DATA_MEDIA_STATE = 2048;
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_FINALIZED: IMAPI_FORMAT2_DATA_MEDIA_STATE = 16384;
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_FINAL_SESSION: IMAPI_FORMAT2_DATA_MEDIA_STATE = 8;
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_INFORMATIONAL_MASK: IMAPI_FORMAT2_DATA_MEDIA_STATE = 15;
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_NON_EMPTY_SESSION: IMAPI_FORMAT2_DATA_MEDIA_STATE = 4096;
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_OVERWRITE_ONLY: IMAPI_FORMAT2_DATA_MEDIA_STATE = 1;
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_RANDOMLY_WRITABLE: IMAPI_FORMAT2_DATA_MEDIA_STATE = 1;
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_UNKNOWN: IMAPI_FORMAT2_DATA_MEDIA_STATE = 0;
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_UNSUPPORTED_MASK: IMAPI_FORMAT2_DATA_MEDIA_STATE = 64512;
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_UNSUPPORTED_MEDIA: IMAPI_FORMAT2_DATA_MEDIA_STATE = 32768;
pub const IMAPI_FORMAT2_DATA_MEDIA_STATE_WRITE_PROTECTED: IMAPI_FORMAT2_DATA_MEDIA_STATE = 8192;
pub type IMAPI_FORMAT2_DATA_WRITE_ACTION = i32;
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_CALIBRATING_POWER: IMAPI_FORMAT2_DATA_WRITE_ACTION = 3;
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_COMPLETED: IMAPI_FORMAT2_DATA_WRITE_ACTION = 6;
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_FINALIZATION: IMAPI_FORMAT2_DATA_WRITE_ACTION = 5;
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_FORMATTING_MEDIA: IMAPI_FORMAT2_DATA_WRITE_ACTION = 1;
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_INITIALIZING_HARDWARE: IMAPI_FORMAT2_DATA_WRITE_ACTION = 2;
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_VALIDATING_MEDIA: IMAPI_FORMAT2_DATA_WRITE_ACTION = 0;
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_VERIFYING: IMAPI_FORMAT2_DATA_WRITE_ACTION = 7;
pub const IMAPI_FORMAT2_DATA_WRITE_ACTION_WRITING_DATA: IMAPI_FORMAT2_DATA_WRITE_ACTION = 4;
pub type IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = i32;
pub const IMAPI_FORMAT2_RAW_CD_SUBCODE_IS_COOKED: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = 2;
pub const IMAPI_FORMAT2_RAW_CD_SUBCODE_IS_RAW: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = 3;
pub const IMAPI_FORMAT2_RAW_CD_SUBCODE_PQ_ONLY: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = 1;
pub type IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = i32;
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_FINISHING: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = 3;
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_PREPARING: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = 1;
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_UNKNOWN: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = 0;
pub const IMAPI_FORMAT2_RAW_CD_WRITE_ACTION_WRITING: IMAPI_FORMAT2_RAW_CD_WRITE_ACTION = 2;
pub type IMAPI_FORMAT2_TAO_WRITE_ACTION = i32;
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_FINISHING: IMAPI_FORMAT2_TAO_WRITE_ACTION = 3;
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_PREPARING: IMAPI_FORMAT2_TAO_WRITE_ACTION = 1;
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_UNKNOWN: IMAPI_FORMAT2_TAO_WRITE_ACTION = 0;
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_VERIFYING: IMAPI_FORMAT2_TAO_WRITE_ACTION = 4;
pub const IMAPI_FORMAT2_TAO_WRITE_ACTION_WRITING: IMAPI_FORMAT2_TAO_WRITE_ACTION = 2;
pub type IMAPI_MEDIA_PHYSICAL_TYPE = i32;
pub const IMAPI_MEDIA_TYPE_BDR: IMAPI_MEDIA_PHYSICAL_TYPE = 18;
pub const IMAPI_MEDIA_TYPE_BDRE: IMAPI_MEDIA_PHYSICAL_TYPE = 19;
pub const IMAPI_MEDIA_TYPE_BDROM: IMAPI_MEDIA_PHYSICAL_TYPE = 17;
pub const IMAPI_MEDIA_TYPE_CDR: IMAPI_MEDIA_PHYSICAL_TYPE = 2;
pub const IMAPI_MEDIA_TYPE_CDROM: IMAPI_MEDIA_PHYSICAL_TYPE = 1;
pub const IMAPI_MEDIA_TYPE_CDRW: IMAPI_MEDIA_PHYSICAL_TYPE = 3;
pub const IMAPI_MEDIA_TYPE_DISK: IMAPI_MEDIA_PHYSICAL_TYPE = 12;
pub const IMAPI_MEDIA_TYPE_DVDDASHR: IMAPI_MEDIA_PHYSICAL_TYPE = 9;
pub const IMAPI_MEDIA_TYPE_DVDDASHRW: IMAPI_MEDIA_PHYSICAL_TYPE = 10;
pub const IMAPI_MEDIA_TYPE_DVDDASHR_DUALLAYER: IMAPI_MEDIA_PHYSICAL_TYPE = 11;
pub const IMAPI_MEDIA_TYPE_DVDPLUSR: IMAPI_MEDIA_PHYSICAL_TYPE = 6;
pub const IMAPI_MEDIA_TYPE_DVDPLUSRW: IMAPI_MEDIA_PHYSICAL_TYPE = 7;
pub const IMAPI_MEDIA_TYPE_DVDPLUSRW_DUALLAYER: IMAPI_MEDIA_PHYSICAL_TYPE = 13;
pub const IMAPI_MEDIA_TYPE_DVDPLUSR_DUALLAYER: IMAPI_MEDIA_PHYSICAL_TYPE = 8;
pub const IMAPI_MEDIA_TYPE_DVDRAM: IMAPI_MEDIA_PHYSICAL_TYPE = 5;
pub const IMAPI_MEDIA_TYPE_DVDROM: IMAPI_MEDIA_PHYSICAL_TYPE = 4;
pub const IMAPI_MEDIA_TYPE_HDDVDR: IMAPI_MEDIA_PHYSICAL_TYPE = 15;
pub const IMAPI_MEDIA_TYPE_HDDVDRAM: IMAPI_MEDIA_PHYSICAL_TYPE = 16;
pub const IMAPI_MEDIA_TYPE_HDDVDROM: IMAPI_MEDIA_PHYSICAL_TYPE = 14;
pub const IMAPI_MEDIA_TYPE_MAX: IMAPI_MEDIA_PHYSICAL_TYPE = 19;
pub const IMAPI_MEDIA_TYPE_UNKNOWN: IMAPI_MEDIA_PHYSICAL_TYPE = 0;
pub type IMAPI_MEDIA_WRITE_PROTECT_STATE = i32;
pub type IMAPI_MODE_PAGE_REQUEST_TYPE = i32;
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_CHANGEABLE_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = 1;
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_CURRENT_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = 0;
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_DEFAULT_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = 2;
pub const IMAPI_MODE_PAGE_REQUEST_TYPE_SAVED_VALUES: IMAPI_MODE_PAGE_REQUEST_TYPE = 3;
pub type IMAPI_MODE_PAGE_TYPE = i32;
pub const IMAPI_MODE_PAGE_TYPE_CACHING: IMAPI_MODE_PAGE_TYPE = 8;
pub const IMAPI_MODE_PAGE_TYPE_INFORMATIONAL_EXCEPTIONS: IMAPI_MODE_PAGE_TYPE = 28;
pub const IMAPI_MODE_PAGE_TYPE_LEGACY_CAPABILITIES: IMAPI_MODE_PAGE_TYPE = 42;
pub const IMAPI_MODE_PAGE_TYPE_MRW: IMAPI_MODE_PAGE_TYPE = 3;
pub const IMAPI_MODE_PAGE_TYPE_POWER_CONDITION: IMAPI_MODE_PAGE_TYPE = 26;
pub const IMAPI_MODE_PAGE_TYPE_READ_WRITE_ERROR_RECOVERY: IMAPI_MODE_PAGE_TYPE = 1;
pub const IMAPI_MODE_PAGE_TYPE_TIMEOUT_AND_PROTECT: IMAPI_MODE_PAGE_TYPE = 29;
pub const IMAPI_MODE_PAGE_TYPE_WRITE_PARAMETERS: IMAPI_MODE_PAGE_TYPE = 5;
pub type IMAPI_PROFILE_TYPE = i32;
pub const IMAPI_PROFILE_TYPE_AS_MO: IMAPI_PROFILE_TYPE = 5;
pub const IMAPI_PROFILE_TYPE_BD_REWRITABLE: IMAPI_PROFILE_TYPE = 67;
pub const IMAPI_PROFILE_TYPE_BD_ROM: IMAPI_PROFILE_TYPE = 64;
pub const IMAPI_PROFILE_TYPE_BD_R_RANDOM_RECORDING: IMAPI_PROFILE_TYPE = 66;
pub const IMAPI_PROFILE_TYPE_BD_R_SEQUENTIAL: IMAPI_PROFILE_TYPE = 65;
pub const IMAPI_PROFILE_TYPE_CDROM: IMAPI_PROFILE_TYPE = 8;
pub const IMAPI_PROFILE_TYPE_CD_RECORDABLE: IMAPI_PROFILE_TYPE = 9;
pub const IMAPI_PROFILE_TYPE_CD_REWRITABLE: IMAPI_PROFILE_TYPE = 10;
pub const IMAPI_PROFILE_TYPE_DDCDROM: IMAPI_PROFILE_TYPE = 32;
pub const IMAPI_PROFILE_TYPE_DDCD_RECORDABLE: IMAPI_PROFILE_TYPE = 33;
pub const IMAPI_PROFILE_TYPE_DDCD_REWRITABLE: IMAPI_PROFILE_TYPE = 34;
pub const IMAPI_PROFILE_TYPE_DVDROM: IMAPI_PROFILE_TYPE = 16;
pub const IMAPI_PROFILE_TYPE_DVD_DASH_RECORDABLE: IMAPI_PROFILE_TYPE = 17;
pub const IMAPI_PROFILE_TYPE_DVD_DASH_REWRITABLE: IMAPI_PROFILE_TYPE = 19;
pub const IMAPI_PROFILE_TYPE_DVD_DASH_RW_SEQUENTIAL: IMAPI_PROFILE_TYPE = 20;
pub const IMAPI_PROFILE_TYPE_DVD_DASH_R_DUAL_LAYER_JUMP: IMAPI_PROFILE_TYPE = 22;
pub const IMAPI_PROFILE_TYPE_DVD_DASH_R_DUAL_SEQUENTIAL: IMAPI_PROFILE_TYPE = 21;
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_R: IMAPI_PROFILE_TYPE = 27;
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_RW: IMAPI_PROFILE_TYPE = 26;
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_RW_DUAL: IMAPI_PROFILE_TYPE = 42;
pub const IMAPI_PROFILE_TYPE_DVD_PLUS_R_DUAL: IMAPI_PROFILE_TYPE = 43;
pub const IMAPI_PROFILE_TYPE_DVD_RAM: IMAPI_PROFILE_TYPE = 18;
pub const IMAPI_PROFILE_TYPE_HD_DVD_RAM: IMAPI_PROFILE_TYPE = 82;
pub const IMAPI_PROFILE_TYPE_HD_DVD_RECORDABLE: IMAPI_PROFILE_TYPE = 81;
pub const IMAPI_PROFILE_TYPE_HD_DVD_ROM: IMAPI_PROFILE_TYPE = 80;
pub const IMAPI_PROFILE_TYPE_INVALID: IMAPI_PROFILE_TYPE = 0;
pub const IMAPI_PROFILE_TYPE_MO_ERASABLE: IMAPI_PROFILE_TYPE = 3;
pub const IMAPI_PROFILE_TYPE_MO_WRITE_ONCE: IMAPI_PROFILE_TYPE = 4;
pub const IMAPI_PROFILE_TYPE_NON_REMOVABLE_DISK: IMAPI_PROFILE_TYPE = 1;
pub const IMAPI_PROFILE_TYPE_NON_STANDARD: IMAPI_PROFILE_TYPE = 65535;
pub const IMAPI_PROFILE_TYPE_REMOVABLE_DISK: IMAPI_PROFILE_TYPE = 2;
pub type IMAPI_READ_TRACK_ADDRESS_TYPE = i32;
pub const IMAPI_READ_TRACK_ADDRESS_TYPE_LBA: IMAPI_READ_TRACK_ADDRESS_TYPE = 0;
pub const IMAPI_READ_TRACK_ADDRESS_TYPE_SESSION: IMAPI_READ_TRACK_ADDRESS_TYPE = 2;
pub const IMAPI_READ_TRACK_ADDRESS_TYPE_TRACK: IMAPI_READ_TRACK_ADDRESS_TYPE = 1;
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_BD: i32 = 2195;
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_CD: i32 = 75;
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_DVD: i32 = 680;
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_HD_DVD: i32 = 4568;
pub const IMAPI_SECTOR_SIZE: i32 = 2048;
pub const IMAPI_WRITEPROTECTED_BY_CARTRIDGE: IMAPI_MEDIA_WRITE_PROTECT_STATE = 2;
pub const IMAPI_WRITEPROTECTED_BY_DISC_CONTROL_BLOCK: IMAPI_MEDIA_WRITE_PROTECT_STATE = 16;
pub const IMAPI_WRITEPROTECTED_BY_MEDIA_SPECIFIC_REASON: IMAPI_MEDIA_WRITE_PROTECT_STATE = 4;
pub const IMAPI_WRITEPROTECTED_BY_SOFTWARE_WRITE_PROTECT: IMAPI_MEDIA_WRITE_PROTECT_STATE = 8;
pub const IMAPI_WRITEPROTECTED_READ_ONLY_MEDIA: IMAPI_MEDIA_WRITE_PROTECT_STATE = 16384;
pub const IMAPI_WRITEPROTECTED_UNTIL_POWERDOWN: IMAPI_MEDIA_WRITE_PROTECT_STATE = 1;
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IMultisession, IMultisession_Vtbl, 0x27354150_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IMultisession {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IMultisession, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IMultisession {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsSupportedOnCurrentMediaState(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsSupportedOnCurrentMediaState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_InUse(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_InUse)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_InUse(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_InUse)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ImportRecorder(&self) -> windows_core::Result<IDiscRecorder2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ImportRecorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IMultisession_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_IsSupportedOnCurrentMediaState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsSupportedOnCurrentMediaState: usize,
    #[cfg(feature = "wtypes")]
    pub put_InUse: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_InUse: usize,
    #[cfg(feature = "wtypes")]
    pub get_InUse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_InUse: usize,
    pub get_ImportRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMultisession_Impl: super::IDispatch_Impl {
    fn get_IsSupportedOnCurrentMediaState(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_InUse(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_InUse(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_ImportRecorder(&self) -> windows_core::Result<IDiscRecorder2>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMultisession_Vtbl {
    pub const fn new<Identity: IMultisession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_IsSupportedOnCurrentMediaState<Identity: IMultisession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisession_Impl::get_IsSupportedOnCurrentMediaState(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_InUse<Identity: IMultisession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultisession_Impl::put_InUse(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_InUse<Identity: IMultisession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisession_Impl::get_InUse(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ImportRecorder<Identity: IMultisession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisession_Impl::get_ImportRecorder(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_IsSupportedOnCurrentMediaState: get_IsSupportedOnCurrentMediaState::<Identity, OFFSET>,
            put_InUse: put_InUse::<Identity, OFFSET>,
            get_InUse: get_InUse::<Identity, OFFSET>,
            get_ImportRecorder: get_ImportRecorder::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultisession as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMultisession {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IMultisessionRandomWrite, IMultisessionRandomWrite_Vtbl, 0xb507ca23_2204_11dd_966a_001aa01bbc58);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IMultisessionRandomWrite {
    type Target = IMultisession;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IMultisessionRandomWrite, windows_core::IUnknown, super::IDispatch, IMultisession);
#[cfg(feature = "oaidl")]
impl IMultisessionRandomWrite {
    #[cfg(feature = "shared")]
    pub unsafe fn get_WriteUnitSize(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WriteUnitSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_LastWrittenAddress(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LastWrittenAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_TotalSectorsOnMedia(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TotalSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IMultisessionRandomWrite_Vtbl {
    pub base__: IMultisession_Vtbl,
    #[cfg(feature = "shared")]
    pub get_WriteUnitSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_WriteUnitSize: usize,
    #[cfg(feature = "shared")]
    pub get_LastWrittenAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_LastWrittenAddress: usize,
    #[cfg(feature = "shared")]
    pub get_TotalSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_TotalSectorsOnMedia: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMultisessionRandomWrite_Impl: IMultisession_Impl {
    fn get_WriteUnitSize(&self) -> windows_core::Result<super::LONG>;
    fn get_LastWrittenAddress(&self) -> windows_core::Result<super::LONG>;
    fn get_TotalSectorsOnMedia(&self) -> windows_core::Result<super::LONG>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMultisessionRandomWrite_Vtbl {
    pub const fn new<Identity: IMultisessionRandomWrite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_WriteUnitSize<Identity: IMultisessionRandomWrite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionRandomWrite_Impl::get_WriteUnitSize(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_LastWrittenAddress<Identity: IMultisessionRandomWrite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionRandomWrite_Impl::get_LastWrittenAddress(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_TotalSectorsOnMedia<Identity: IMultisessionRandomWrite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionRandomWrite_Impl::get_TotalSectorsOnMedia(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IMultisession_Vtbl::new::<Identity, OFFSET>(),
            get_WriteUnitSize: get_WriteUnitSize::<Identity, OFFSET>,
            get_LastWrittenAddress: get_LastWrittenAddress::<Identity, OFFSET>,
            get_TotalSectorsOnMedia: get_TotalSectorsOnMedia::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultisessionRandomWrite as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IMultisession as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMultisessionRandomWrite {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IMultisessionSequential, IMultisessionSequential_Vtbl, 0x27354151_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IMultisessionSequential {
    type Target = IMultisession;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IMultisessionSequential, windows_core::IUnknown, super::IDispatch, IMultisession);
#[cfg(feature = "oaidl")]
impl IMultisessionSequential {
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_IsFirstDataSession(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsFirstDataSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_StartAddressOfPreviousSession(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StartAddressOfPreviousSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_LastWrittenAddressOfPreviousSession(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LastWrittenAddressOfPreviousSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_NextWritableAddress(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NextWritableAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_FreeSectorsOnMedia(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FreeSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IMultisessionSequential_Vtbl {
    pub base__: IMultisession_Vtbl,
    #[cfg(feature = "wtypes")]
    pub get_IsFirstDataSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_IsFirstDataSession: usize,
    #[cfg(feature = "shared")]
    pub get_StartAddressOfPreviousSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_StartAddressOfPreviousSession: usize,
    #[cfg(feature = "shared")]
    pub get_LastWrittenAddressOfPreviousSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_LastWrittenAddressOfPreviousSession: usize,
    #[cfg(feature = "shared")]
    pub get_NextWritableAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_NextWritableAddress: usize,
    #[cfg(feature = "shared")]
    pub get_FreeSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_FreeSectorsOnMedia: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMultisessionSequential_Impl: IMultisession_Impl {
    fn get_IsFirstDataSession(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_StartAddressOfPreviousSession(&self) -> windows_core::Result<super::LONG>;
    fn get_LastWrittenAddressOfPreviousSession(&self) -> windows_core::Result<super::LONG>;
    fn get_NextWritableAddress(&self) -> windows_core::Result<super::LONG>;
    fn get_FreeSectorsOnMedia(&self) -> windows_core::Result<super::LONG>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMultisessionSequential_Vtbl {
    pub const fn new<Identity: IMultisessionSequential_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_IsFirstDataSession<Identity: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionSequential_Impl::get_IsFirstDataSession(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_StartAddressOfPreviousSession<Identity: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionSequential_Impl::get_StartAddressOfPreviousSession(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_LastWrittenAddressOfPreviousSession<Identity: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionSequential_Impl::get_LastWrittenAddressOfPreviousSession(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_NextWritableAddress<Identity: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionSequential_Impl::get_NextWritableAddress(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_FreeSectorsOnMedia<Identity: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionSequential_Impl::get_FreeSectorsOnMedia(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IMultisession_Vtbl::new::<Identity, OFFSET>(),
            get_IsFirstDataSession: get_IsFirstDataSession::<Identity, OFFSET>,
            get_StartAddressOfPreviousSession: get_StartAddressOfPreviousSession::<Identity, OFFSET>,
            get_LastWrittenAddressOfPreviousSession: get_LastWrittenAddressOfPreviousSession::<Identity, OFFSET>,
            get_NextWritableAddress: get_NextWritableAddress::<Identity, OFFSET>,
            get_FreeSectorsOnMedia: get_FreeSectorsOnMedia::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultisessionSequential as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IMultisession as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMultisessionSequential {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IMultisessionSequential2, IMultisessionSequential2_Vtbl, 0xb507ca22_2204_11dd_966a_001aa01bbc58);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IMultisessionSequential2 {
    type Target = IMultisessionSequential;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IMultisessionSequential2, windows_core::IUnknown, super::IDispatch, IMultisession, IMultisessionSequential);
#[cfg(feature = "oaidl")]
impl IMultisessionSequential2 {
    #[cfg(feature = "shared")]
    pub unsafe fn get_WriteUnitSize(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WriteUnitSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IMultisessionSequential2_Vtbl {
    pub base__: IMultisessionSequential_Vtbl,
    #[cfg(feature = "shared")]
    pub get_WriteUnitSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_WriteUnitSize: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMultisessionSequential2_Impl: IMultisessionSequential_Impl {
    fn get_WriteUnitSize(&self) -> windows_core::Result<super::LONG>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMultisessionSequential2_Vtbl {
    pub const fn new<Identity: IMultisessionSequential2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_WriteUnitSize<Identity: IMultisessionSequential2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionSequential2_Impl::get_WriteUnitSize(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IMultisessionSequential_Vtbl::new::<Identity, OFFSET>(), get_WriteUnitSize: get_WriteUnitSize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultisessionSequential2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID || iid == &<IMultisession as windows_core::Interface>::IID || iid == &<IMultisessionSequential as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMultisessionSequential2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IRawCDImageCreator, IRawCDImageCreator_Vtbl, 0x25983550_9d65_49ce_b335_40630d901227);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IRawCDImageCreator {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IRawCDImageCreator, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IRawCDImageCreator {
    #[cfg(feature = "objidlbase")]
    pub unsafe fn CreateResultImage(&self) -> windows_core::Result<super::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateResultImage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "objidlbase", feature = "shared"))]
    pub unsafe fn AddTrack<P1>(&self, datatype: IMAPI_CD_SECTOR_TYPE, data: P1) -> windows_core::Result<super::LONG>
    where
        P1: windows_core::Param<super::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddTrack)(windows_core::Interface::as_raw(self), datatype, data.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn AddSpecialPregap<P0>(&self, data: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddSpecialPregap)(windows_core::Interface::as_raw(self), data.param().abi()) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn AddSubcodeRWGenerator<P0>(&self, subcode: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddSubcodeRWGenerator)(windows_core::Interface::as_raw(self), subcode.param().abi()) }
    }
    pub unsafe fn put_ResultingImageType(&self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ResultingImageType)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn get_ResultingImageType(&self) -> windows_core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ResultingImageType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_StartOfLeadout(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StartOfLeadout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_StartOfLeadoutLimit(&self, value: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_StartOfLeadoutLimit)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_StartOfLeadoutLimit(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StartOfLeadoutLimit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_DisableGaplessAudio(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DisableGaplessAudio)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_DisableGaplessAudio(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DisableGaplessAudio)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_MediaCatalogNumber(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_MediaCatalogNumber)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn get_MediaCatalogNumber(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MediaCatalogNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_StartingTrackNumber(&self, value: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_StartingTrackNumber)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_StartingTrackNumber(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StartingTrackNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_TrackInfo(&self, trackindex: super::LONG) -> windows_core::Result<IRawCDImageTrackInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TrackInfo)(windows_core::Interface::as_raw(self), trackindex, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_NumberOfExistingTracks(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NumberOfExistingTracks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_LastUsedUserSectorInImage(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LastUsedUserSectorInImage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_ExpectedTableOfContents(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ExpectedTableOfContents)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IRawCDImageCreator_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "objidlbase")]
    pub CreateResultImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    CreateResultImage: usize,
    #[cfg(all(feature = "objidlbase", feature = "shared"))]
    pub AddTrack: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_CD_SECTOR_TYPE, *mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidlbase", feature = "shared")))]
    AddTrack: usize,
    #[cfg(feature = "objidlbase")]
    pub AddSpecialPregap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    AddSpecialPregap: usize,
    #[cfg(feature = "objidlbase")]
    pub AddSubcodeRWGenerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    AddSubcodeRWGenerator: usize,
    pub put_ResultingImageType: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT,
    pub get_ResultingImageType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_StartOfLeadout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_StartOfLeadout: usize,
    #[cfg(feature = "shared")]
    pub put_StartOfLeadoutLimit: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_StartOfLeadoutLimit: usize,
    #[cfg(feature = "shared")]
    pub get_StartOfLeadoutLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_StartOfLeadoutLimit: usize,
    #[cfg(feature = "wtypes")]
    pub put_DisableGaplessAudio: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_DisableGaplessAudio: usize,
    #[cfg(feature = "wtypes")]
    pub get_DisableGaplessAudio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_DisableGaplessAudio: usize,
    pub put_MediaCatalogNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_MediaCatalogNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub put_StartingTrackNumber: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_StartingTrackNumber: usize,
    #[cfg(feature = "shared")]
    pub get_StartingTrackNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_StartingTrackNumber: usize,
    #[cfg(feature = "shared")]
    pub get_TrackInfo: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_TrackInfo: usize,
    #[cfg(feature = "shared")]
    pub get_NumberOfExistingTracks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_NumberOfExistingTracks: usize,
    #[cfg(feature = "shared")]
    pub get_LastUsedUserSectorInImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_LastUsedUserSectorInImage: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_ExpectedTableOfContents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_ExpectedTableOfContents: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IRawCDImageCreator_Impl: super::IDispatch_Impl {
    fn CreateResultImage(&self) -> windows_core::Result<super::IStream>;
    fn AddTrack(&self, datatype: IMAPI_CD_SECTOR_TYPE, data: windows_core::Ref<super::IStream>) -> windows_core::Result<super::LONG>;
    fn AddSpecialPregap(&self, data: windows_core::Ref<super::IStream>) -> windows_core::Result<()>;
    fn AddSubcodeRWGenerator(&self, subcode: windows_core::Ref<super::IStream>) -> windows_core::Result<()>;
    fn put_ResultingImageType(&self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::Result<()>;
    fn get_ResultingImageType(&self) -> windows_core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE>;
    fn get_StartOfLeadout(&self) -> windows_core::Result<super::LONG>;
    fn put_StartOfLeadoutLimit(&self, value: super::LONG) -> windows_core::Result<()>;
    fn get_StartOfLeadoutLimit(&self) -> windows_core::Result<super::LONG>;
    fn put_DisableGaplessAudio(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_DisableGaplessAudio(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_MediaCatalogNumber(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_MediaCatalogNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_StartingTrackNumber(&self, value: super::LONG) -> windows_core::Result<()>;
    fn get_StartingTrackNumber(&self) -> windows_core::Result<super::LONG>;
    fn get_TrackInfo(&self, trackindex: super::LONG) -> windows_core::Result<IRawCDImageTrackInfo>;
    fn get_NumberOfExistingTracks(&self) -> windows_core::Result<super::LONG>;
    fn get_LastUsedUserSectorInImage(&self) -> windows_core::Result<super::LONG>;
    fn get_ExpectedTableOfContents(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IRawCDImageCreator_Vtbl {
    pub const fn new<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateResultImage<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resultstream: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::CreateResultImage(this) {
                    Ok(ok__) => {
                        resultstream.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddTrack<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datatype: IMAPI_CD_SECTOR_TYPE, data: *mut core::ffi::c_void, trackindex: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::AddTrack(this, core::mem::transmute_copy(&datatype), core::mem::transmute_copy(&data)) {
                    Ok(ok__) => {
                        trackindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddSpecialPregap<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageCreator_Impl::AddSpecialPregap(this, core::mem::transmute_copy(&data)).into()
            }
        }
        unsafe extern "system" fn AddSubcodeRWGenerator<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, subcode: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageCreator_Impl::AddSubcodeRWGenerator(this, core::mem::transmute_copy(&subcode)).into()
            }
        }
        unsafe extern "system" fn put_ResultingImageType<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageCreator_Impl::put_ResultingImageType(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_ResultingImageType<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::get_ResultingImageType(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_StartOfLeadout<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::get_StartOfLeadout(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_StartOfLeadoutLimit<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageCreator_Impl::put_StartOfLeadoutLimit(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_StartOfLeadoutLimit<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::get_StartOfLeadoutLimit(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DisableGaplessAudio<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageCreator_Impl::put_DisableGaplessAudio(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_DisableGaplessAudio<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::get_DisableGaplessAudio(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_MediaCatalogNumber<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageCreator_Impl::put_MediaCatalogNumber(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn get_MediaCatalogNumber<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::get_MediaCatalogNumber(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_StartingTrackNumber<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageCreator_Impl::put_StartingTrackNumber(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_StartingTrackNumber<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::get_StartingTrackNumber(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_TrackInfo<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, trackindex: super::LONG, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::get_TrackInfo(this, core::mem::transmute_copy(&trackindex)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_NumberOfExistingTracks<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::get_NumberOfExistingTracks(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_LastUsedUserSectorInImage<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::get_LastUsedUserSectorInImage(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ExpectedTableOfContents<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::get_ExpectedTableOfContents(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CreateResultImage: CreateResultImage::<Identity, OFFSET>,
            AddTrack: AddTrack::<Identity, OFFSET>,
            AddSpecialPregap: AddSpecialPregap::<Identity, OFFSET>,
            AddSubcodeRWGenerator: AddSubcodeRWGenerator::<Identity, OFFSET>,
            put_ResultingImageType: put_ResultingImageType::<Identity, OFFSET>,
            get_ResultingImageType: get_ResultingImageType::<Identity, OFFSET>,
            get_StartOfLeadout: get_StartOfLeadout::<Identity, OFFSET>,
            put_StartOfLeadoutLimit: put_StartOfLeadoutLimit::<Identity, OFFSET>,
            get_StartOfLeadoutLimit: get_StartOfLeadoutLimit::<Identity, OFFSET>,
            put_DisableGaplessAudio: put_DisableGaplessAudio::<Identity, OFFSET>,
            get_DisableGaplessAudio: get_DisableGaplessAudio::<Identity, OFFSET>,
            put_MediaCatalogNumber: put_MediaCatalogNumber::<Identity, OFFSET>,
            get_MediaCatalogNumber: get_MediaCatalogNumber::<Identity, OFFSET>,
            put_StartingTrackNumber: put_StartingTrackNumber::<Identity, OFFSET>,
            get_StartingTrackNumber: get_StartingTrackNumber::<Identity, OFFSET>,
            get_TrackInfo: get_TrackInfo::<Identity, OFFSET>,
            get_NumberOfExistingTracks: get_NumberOfExistingTracks::<Identity, OFFSET>,
            get_LastUsedUserSectorInImage: get_LastUsedUserSectorInImage::<Identity, OFFSET>,
            get_ExpectedTableOfContents: get_ExpectedTableOfContents::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRawCDImageCreator as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IRawCDImageCreator {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IRawCDImageTrackInfo, IRawCDImageTrackInfo_Vtbl, 0x25983551_9d65_49ce_b335_40630d901227);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IRawCDImageTrackInfo {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IRawCDImageTrackInfo, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IRawCDImageTrackInfo {
    #[cfg(feature = "shared")]
    pub unsafe fn get_StartingLba(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StartingLba)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_SectorCount(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SectorCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_TrackNumber(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TrackNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_SectorType(&self) -> windows_core::Result<IMAPI_CD_SECTOR_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SectorType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ISRC(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ISRC)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn put_ISRC(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ISRC)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn get_DigitalAudioCopySetting(&self) -> windows_core::Result<IMAPI_CD_TRACK_DIGITAL_COPY_SETTING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DigitalAudioCopySetting)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_DigitalAudioCopySetting(&self, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_DigitalAudioCopySetting)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_AudioHasPreemphasis(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AudioHasPreemphasis)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_AudioHasPreemphasis(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AudioHasPreemphasis)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub unsafe fn get_TrackIndexes(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TrackIndexes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddTrackIndex(&self, lbaoffset: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddTrackIndex)(windows_core::Interface::as_raw(self), lbaoffset) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn ClearTrackIndex(&self, lbaoffset: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearTrackIndex)(windows_core::Interface::as_raw(self), lbaoffset) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IRawCDImageTrackInfo_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub get_StartingLba: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_StartingLba: usize,
    #[cfg(feature = "shared")]
    pub get_SectorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_SectorCount: usize,
    #[cfg(feature = "shared")]
    pub get_TrackNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_TrackNumber: usize,
    pub get_SectorType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_CD_SECTOR_TYPE) -> windows_core::HRESULT,
    pub get_ISRC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_ISRC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DigitalAudioCopySetting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> windows_core::HRESULT,
    pub put_DigitalAudioCopySetting: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_AudioHasPreemphasis: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_AudioHasPreemphasis: usize,
    #[cfg(feature = "wtypes")]
    pub put_AudioHasPreemphasis: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_AudioHasPreemphasis: usize,
    #[cfg(all(feature = "shared", feature = "winnt"))]
    pub get_TrackIndexes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "winnt")))]
    get_TrackIndexes: usize,
    #[cfg(feature = "shared")]
    pub AddTrackIndex: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddTrackIndex: usize,
    #[cfg(feature = "shared")]
    pub ClearTrackIndex: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    ClearTrackIndex: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IRawCDImageTrackInfo_Impl: super::IDispatch_Impl {
    fn get_StartingLba(&self) -> windows_core::Result<super::LONG>;
    fn get_SectorCount(&self) -> windows_core::Result<super::LONG>;
    fn get_TrackNumber(&self) -> windows_core::Result<super::LONG>;
    fn get_SectorType(&self) -> windows_core::Result<IMAPI_CD_SECTOR_TYPE>;
    fn get_ISRC(&self) -> windows_core::Result<windows_core::BSTR>;
    fn put_ISRC(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_DigitalAudioCopySetting(&self) -> windows_core::Result<IMAPI_CD_TRACK_DIGITAL_COPY_SETTING>;
    fn put_DigitalAudioCopySetting(&self, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> windows_core::Result<()>;
    fn get_AudioHasPreemphasis(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_AudioHasPreemphasis(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_TrackIndexes(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn AddTrackIndex(&self, lbaoffset: super::LONG) -> windows_core::Result<()>;
    fn ClearTrackIndex(&self, lbaoffset: super::LONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IRawCDImageTrackInfo_Vtbl {
    pub const fn new<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_StartingLba<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageTrackInfo_Impl::get_StartingLba(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SectorCount<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageTrackInfo_Impl::get_SectorCount(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_TrackNumber<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageTrackInfo_Impl::get_TrackNumber(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SectorType<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_CD_SECTOR_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageTrackInfo_Impl::get_SectorType(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ISRC<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageTrackInfo_Impl::get_ISRC(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ISRC<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageTrackInfo_Impl::put_ISRC(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn get_DigitalAudioCopySetting<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageTrackInfo_Impl::get_DigitalAudioCopySetting(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_DigitalAudioCopySetting<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageTrackInfo_Impl::put_DigitalAudioCopySetting(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_AudioHasPreemphasis<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageTrackInfo_Impl::get_AudioHasPreemphasis(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AudioHasPreemphasis<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageTrackInfo_Impl::put_AudioHasPreemphasis(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_TrackIndexes<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageTrackInfo_Impl::get_TrackIndexes(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddTrackIndex<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbaoffset: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageTrackInfo_Impl::AddTrackIndex(this, core::mem::transmute_copy(&lbaoffset)).into()
            }
        }
        unsafe extern "system" fn ClearTrackIndex<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbaoffset: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageTrackInfo_Impl::ClearTrackIndex(this, core::mem::transmute_copy(&lbaoffset)).into()
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_StartingLba: get_StartingLba::<Identity, OFFSET>,
            get_SectorCount: get_SectorCount::<Identity, OFFSET>,
            get_TrackNumber: get_TrackNumber::<Identity, OFFSET>,
            get_SectorType: get_SectorType::<Identity, OFFSET>,
            get_ISRC: get_ISRC::<Identity, OFFSET>,
            put_ISRC: put_ISRC::<Identity, OFFSET>,
            get_DigitalAudioCopySetting: get_DigitalAudioCopySetting::<Identity, OFFSET>,
            put_DigitalAudioCopySetting: put_DigitalAudioCopySetting::<Identity, OFFSET>,
            get_AudioHasPreemphasis: get_AudioHasPreemphasis::<Identity, OFFSET>,
            put_AudioHasPreemphasis: put_AudioHasPreemphasis::<Identity, OFFSET>,
            get_TrackIndexes: get_TrackIndexes::<Identity, OFFSET>,
            AddTrackIndex: AddTrackIndex::<Identity, OFFSET>,
            ClearTrackIndex: ClearTrackIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRawCDImageTrackInfo as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IRawCDImageTrackInfo {}
#[cfg(feature = "objidlbase")]
windows_core::imp::define_interface!(IStreamConcatenate, IStreamConcatenate_Vtbl, 0x27354146_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "objidlbase")]
impl core::ops::Deref for IStreamConcatenate {
    type Target = super::IStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "objidlbase")]
windows_core::imp::interface_hierarchy!(IStreamConcatenate, windows_core::IUnknown, super::ISequentialStream, super::IStream);
#[cfg(feature = "objidlbase")]
impl IStreamConcatenate {
    pub unsafe fn Initialize<P0, P1>(&self, stream1: P0, stream2: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IStream>,
        P1: windows_core::Param<super::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), stream1.param().abi(), stream2.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Initialize2(&self, streams: &[Option<super::IStream>]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Initialize2)(windows_core::Interface::as_raw(self), core::mem::transmute(streams.as_ptr()), streams.len().try_into().unwrap()) }
    }
    pub unsafe fn Append<P0>(&self, stream: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), stream.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Append2(&self, streams: &[Option<super::IStream>]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Append2)(windows_core::Interface::as_raw(self), core::mem::transmute(streams.as_ptr()), streams.len().try_into().unwrap()) }
    }
}
#[cfg(feature = "objidlbase")]
#[repr(C)]
#[doc(hidden)]
pub struct IStreamConcatenate_Vtbl {
    pub base__: super::IStream_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub Initialize2: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Initialize2: usize,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub Append2: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Append2: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypesbase"))]
pub trait IStreamConcatenate_Impl: super::IStream_Impl {
    fn Initialize(&self, stream1: windows_core::Ref<super::IStream>, stream2: windows_core::Ref<super::IStream>) -> windows_core::Result<()>;
    fn Initialize2(&self, streams: *const Option<super::IStream>, streamcount: super::ULONG) -> windows_core::Result<()>;
    fn Append(&self, stream: windows_core::Ref<super::IStream>) -> windows_core::Result<()>;
    fn Append2(&self, streams: *const Option<super::IStream>, streamcount: super::ULONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypesbase"))]
impl IStreamConcatenate_Vtbl {
    pub const fn new<Identity: IStreamConcatenate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream1: *mut core::ffi::c_void, stream2: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStreamConcatenate_Impl::Initialize(this, core::mem::transmute_copy(&stream1), core::mem::transmute_copy(&stream2)).into()
            }
        }
        unsafe extern "system" fn Initialize2<Identity: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streams: *const *mut core::ffi::c_void, streamcount: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStreamConcatenate_Impl::Initialize2(this, core::mem::transmute_copy(&streams), core::mem::transmute_copy(&streamcount)).into()
            }
        }
        unsafe extern "system" fn Append<Identity: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStreamConcatenate_Impl::Append(this, core::mem::transmute_copy(&stream)).into()
            }
        }
        unsafe extern "system" fn Append2<Identity: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streams: *const *mut core::ffi::c_void, streamcount: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStreamConcatenate_Impl::Append2(this, core::mem::transmute_copy(&streams), core::mem::transmute_copy(&streamcount)).into()
            }
        }
        Self {
            base__: super::IStream_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Initialize2: Initialize2::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
            Append2: Append2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStreamConcatenate as windows_core::Interface>::IID || iid == &<super::ISequentialStream as windows_core::Interface>::IID || iid == &<super::IStream as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IStreamConcatenate {}
#[cfg(feature = "objidlbase")]
windows_core::imp::define_interface!(IStreamInterleave, IStreamInterleave_Vtbl, 0x27354147_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "objidlbase")]
impl core::ops::Deref for IStreamInterleave {
    type Target = super::IStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "objidlbase")]
windows_core::imp::interface_hierarchy!(IStreamInterleave, windows_core::IUnknown, super::ISequentialStream, super::IStream);
#[cfg(feature = "objidlbase")]
impl IStreamInterleave {
    #[cfg(feature = "shared")]
    pub unsafe fn Initialize(&self, streams: *const Option<super::IStream>, interleavesizes: *const super::ULONG, streamcount: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), core::mem::transmute(streams), interleavesizes, streamcount) }
    }
}
#[cfg(feature = "objidlbase")]
#[repr(C)]
#[doc(hidden)]
pub struct IStreamInterleave_Vtbl {
    pub base__: super::IStream_Vtbl,
    #[cfg(feature = "shared")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, *const super::ULONG, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Initialize: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypesbase"))]
pub trait IStreamInterleave_Impl: super::IStream_Impl {
    fn Initialize(&self, streams: *const Option<super::IStream>, interleavesizes: *const super::ULONG, streamcount: super::ULONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypesbase"))]
impl IStreamInterleave_Vtbl {
    pub const fn new<Identity: IStreamInterleave_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IStreamInterleave_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streams: *const *mut core::ffi::c_void, interleavesizes: *const super::ULONG, streamcount: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStreamInterleave_Impl::Initialize(this, core::mem::transmute_copy(&streams), core::mem::transmute_copy(&interleavesizes), core::mem::transmute_copy(&streamcount)).into()
            }
        }
        Self { base__: super::IStream_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStreamInterleave as windows_core::Interface>::IID || iid == &<super::ISequentialStream as windows_core::Interface>::IID || iid == &<super::IStream as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IStreamInterleave {}
#[cfg(feature = "objidlbase")]
windows_core::imp::define_interface!(IStreamPseudoRandomBased, IStreamPseudoRandomBased_Vtbl, 0x27354145_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "objidlbase")]
impl core::ops::Deref for IStreamPseudoRandomBased {
    type Target = super::IStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "objidlbase")]
windows_core::imp::interface_hierarchy!(IStreamPseudoRandomBased, windows_core::IUnknown, super::ISequentialStream, super::IStream);
#[cfg(feature = "objidlbase")]
impl IStreamPseudoRandomBased {
    #[cfg(feature = "shared")]
    pub unsafe fn put_Seed(&self, value: super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Seed)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Seed(&self) -> windows_core::Result<super::ULONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Seed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_ExtendedSeed(&self, values: &[super::ULONG]) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ExtendedSeed)(windows_core::Interface::as_raw(self), values.as_ptr(), values.len().try_into().unwrap()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_ExtendedSeed(&self, values: *mut *mut super::ULONG, ecount: *mut super::ULONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_ExtendedSeed)(windows_core::Interface::as_raw(self), values as _, ecount as _) }
    }
}
#[cfg(feature = "objidlbase")]
#[repr(C)]
#[doc(hidden)]
pub struct IStreamPseudoRandomBased_Vtbl {
    pub base__: super::IStream_Vtbl,
    #[cfg(feature = "shared")]
    pub put_Seed: unsafe extern "system" fn(*mut core::ffi::c_void, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_Seed: usize,
    #[cfg(feature = "shared")]
    pub get_Seed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Seed: usize,
    #[cfg(feature = "shared")]
    pub put_ExtendedSeed: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::ULONG, super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_ExtendedSeed: usize,
    #[cfg(feature = "shared")]
    pub get_ExtendedSeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::ULONG, *mut super::ULONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_ExtendedSeed: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypesbase"))]
pub trait IStreamPseudoRandomBased_Impl: super::IStream_Impl {
    fn put_Seed(&self, value: super::ULONG) -> windows_core::Result<()>;
    fn get_Seed(&self) -> windows_core::Result<super::ULONG>;
    fn put_ExtendedSeed(&self, values: *const super::ULONG, ecount: super::ULONG) -> windows_core::Result<()>;
    fn get_ExtendedSeed(&self, values: *mut *mut super::ULONG, ecount: *mut super::ULONG) -> windows_core::Result<()>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypesbase"))]
impl IStreamPseudoRandomBased_Vtbl {
    pub const fn new<Identity: IStreamPseudoRandomBased_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn put_Seed<Identity: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStreamPseudoRandomBased_Impl::put_Seed(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_Seed<Identity: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStreamPseudoRandomBased_Impl::get_Seed(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ExtendedSeed<Identity: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, values: *const super::ULONG, ecount: super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStreamPseudoRandomBased_Impl::put_ExtendedSeed(this, core::mem::transmute_copy(&values), core::mem::transmute_copy(&ecount)).into()
            }
        }
        unsafe extern "system" fn get_ExtendedSeed<Identity: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, values: *mut *mut super::ULONG, ecount: *mut super::ULONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStreamPseudoRandomBased_Impl::get_ExtendedSeed(this, core::mem::transmute_copy(&values), core::mem::transmute_copy(&ecount)).into()
            }
        }
        Self {
            base__: super::IStream_Vtbl::new::<Identity, OFFSET>(),
            put_Seed: put_Seed::<Identity, OFFSET>,
            get_Seed: get_Seed::<Identity, OFFSET>,
            put_ExtendedSeed: put_ExtendedSeed::<Identity, OFFSET>,
            get_ExtendedSeed: get_ExtendedSeed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStreamPseudoRandomBased as windows_core::Interface>::IID || iid == &<super::ISequentialStream as windows_core::Interface>::IID || iid == &<super::IStream as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IStreamPseudoRandomBased {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWriteEngine2, IWriteEngine2_Vtbl, 0x27354135_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWriteEngine2 {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWriteEngine2, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWriteEngine2 {
    #[cfg(all(feature = "objidlbase", feature = "shared"))]
    pub unsafe fn WriteSection<P0>(&self, data: P0, startingblockaddress: super::LONG, numberofblocks: super::LONG) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteSection)(windows_core::Interface::as_raw(self), data.param().abi(), startingblockaddress, numberofblocks) }
    }
    pub unsafe fn CancelWrite(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CancelWrite)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn put_Recorder<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDiscRecorder2Ex>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_Recorder)(windows_core::Interface::as_raw(self), value.param().abi()) }
    }
    pub unsafe fn get_Recorder(&self) -> windows_core::Result<IDiscRecorder2Ex> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Recorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn put_UseStreamingWrite12(&self, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_UseStreamingWrite12)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_UseStreamingWrite12(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UseStreamingWrite12)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_StartingSectorsPerSecond(&self, value: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_StartingSectorsPerSecond)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_StartingSectorsPerSecond(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StartingSectorsPerSecond)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_EndingSectorsPerSecond(&self, value: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_EndingSectorsPerSecond)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_EndingSectorsPerSecond(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_EndingSectorsPerSecond)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_BytesPerSector(&self, value: super::LONG) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_BytesPerSector)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_BytesPerSector(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BytesPerSector)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_WriteInProgress(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WriteInProgress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWriteEngine2_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(all(feature = "objidlbase", feature = "shared"))]
    pub WriteSection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::LONG, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "objidlbase", feature = "shared")))]
    WriteSection: usize,
    pub CancelWrite: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_Recorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Recorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub put_UseStreamingWrite12: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    put_UseStreamingWrite12: usize,
    #[cfg(feature = "wtypes")]
    pub get_UseStreamingWrite12: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_UseStreamingWrite12: usize,
    #[cfg(feature = "shared")]
    pub put_StartingSectorsPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_StartingSectorsPerSecond: usize,
    #[cfg(feature = "shared")]
    pub get_StartingSectorsPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_StartingSectorsPerSecond: usize,
    #[cfg(feature = "shared")]
    pub put_EndingSectorsPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_EndingSectorsPerSecond: usize,
    #[cfg(feature = "shared")]
    pub get_EndingSectorsPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_EndingSectorsPerSecond: usize,
    #[cfg(feature = "shared")]
    pub put_BytesPerSector: unsafe extern "system" fn(*mut core::ffi::c_void, super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_BytesPerSector: usize,
    #[cfg(feature = "shared")]
    pub get_BytesPerSector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_BytesPerSector: usize,
    #[cfg(feature = "wtypes")]
    pub get_WriteInProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_WriteInProgress: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWriteEngine2_Impl: super::IDispatch_Impl {
    fn WriteSection(&self, data: windows_core::Ref<super::IStream>, startingblockaddress: super::LONG, numberofblocks: super::LONG) -> windows_core::Result<()>;
    fn CancelWrite(&self) -> windows_core::Result<()>;
    fn put_Recorder(&self, value: windows_core::Ref<IDiscRecorder2Ex>) -> windows_core::Result<()>;
    fn get_Recorder(&self) -> windows_core::Result<IDiscRecorder2Ex>;
    fn put_UseStreamingWrite12(&self, value: super::VARIANT_BOOL) -> windows_core::Result<()>;
    fn get_UseStreamingWrite12(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn put_StartingSectorsPerSecond(&self, value: super::LONG) -> windows_core::Result<()>;
    fn get_StartingSectorsPerSecond(&self) -> windows_core::Result<super::LONG>;
    fn put_EndingSectorsPerSecond(&self, value: super::LONG) -> windows_core::Result<()>;
    fn get_EndingSectorsPerSecond(&self) -> windows_core::Result<super::LONG>;
    fn put_BytesPerSector(&self, value: super::LONG) -> windows_core::Result<()>;
    fn get_BytesPerSector(&self) -> windows_core::Result<super::LONG>;
    fn get_WriteInProgress(&self) -> windows_core::Result<super::VARIANT_BOOL>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWriteEngine2_Vtbl {
    pub const fn new<Identity: IWriteEngine2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WriteSection<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void, startingblockaddress: super::LONG, numberofblocks: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWriteEngine2_Impl::WriteSection(this, core::mem::transmute_copy(&data), core::mem::transmute_copy(&startingblockaddress), core::mem::transmute_copy(&numberofblocks)).into()
            }
        }
        unsafe extern "system" fn CancelWrite<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWriteEngine2_Impl::CancelWrite(this).into()
            }
        }
        unsafe extern "system" fn put_Recorder<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWriteEngine2_Impl::put_Recorder(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_Recorder<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2_Impl::get_Recorder(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_UseStreamingWrite12<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWriteEngine2_Impl::put_UseStreamingWrite12(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_UseStreamingWrite12<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2_Impl::get_UseStreamingWrite12(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_StartingSectorsPerSecond<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWriteEngine2_Impl::put_StartingSectorsPerSecond(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_StartingSectorsPerSecond<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2_Impl::get_StartingSectorsPerSecond(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_EndingSectorsPerSecond<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWriteEngine2_Impl::put_EndingSectorsPerSecond(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_EndingSectorsPerSecond<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2_Impl::get_EndingSectorsPerSecond(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_BytesPerSector<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWriteEngine2_Impl::put_BytesPerSector(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_BytesPerSector<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2_Impl::get_BytesPerSector(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_WriteInProgress<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2_Impl::get_WriteInProgress(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            WriteSection: WriteSection::<Identity, OFFSET>,
            CancelWrite: CancelWrite::<Identity, OFFSET>,
            put_Recorder: put_Recorder::<Identity, OFFSET>,
            get_Recorder: get_Recorder::<Identity, OFFSET>,
            put_UseStreamingWrite12: put_UseStreamingWrite12::<Identity, OFFSET>,
            get_UseStreamingWrite12: get_UseStreamingWrite12::<Identity, OFFSET>,
            put_StartingSectorsPerSecond: put_StartingSectorsPerSecond::<Identity, OFFSET>,
            get_StartingSectorsPerSecond: get_StartingSectorsPerSecond::<Identity, OFFSET>,
            put_EndingSectorsPerSecond: put_EndingSectorsPerSecond::<Identity, OFFSET>,
            get_EndingSectorsPerSecond: get_EndingSectorsPerSecond::<Identity, OFFSET>,
            put_BytesPerSector: put_BytesPerSector::<Identity, OFFSET>,
            get_BytesPerSector: get_BytesPerSector::<Identity, OFFSET>,
            get_WriteInProgress: get_WriteInProgress::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWriteEngine2 as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "objidlbase", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWriteEngine2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWriteEngine2EventArgs, IWriteEngine2EventArgs_Vtbl, 0x27354136_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWriteEngine2EventArgs {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWriteEngine2EventArgs, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWriteEngine2EventArgs {
    #[cfg(feature = "shared")]
    pub unsafe fn get_StartLba(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StartLba)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_SectorCount(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SectorCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_LastReadLba(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LastReadLba)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_LastWrittenLba(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LastWrittenLba)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_TotalSystemBuffer(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TotalSystemBuffer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_UsedSystemBuffer(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_UsedSystemBuffer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_FreeSystemBuffer(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FreeSystemBuffer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWriteEngine2EventArgs_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    #[cfg(feature = "shared")]
    pub get_StartLba: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_StartLba: usize,
    #[cfg(feature = "shared")]
    pub get_SectorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_SectorCount: usize,
    #[cfg(feature = "shared")]
    pub get_LastReadLba: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_LastReadLba: usize,
    #[cfg(feature = "shared")]
    pub get_LastWrittenLba: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_LastWrittenLba: usize,
    #[cfg(feature = "shared")]
    pub get_TotalSystemBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_TotalSystemBuffer: usize,
    #[cfg(feature = "shared")]
    pub get_UsedSystemBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_UsedSystemBuffer: usize,
    #[cfg(feature = "shared")]
    pub get_FreeSystemBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_FreeSystemBuffer: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWriteEngine2EventArgs_Impl: super::IDispatch_Impl {
    fn get_StartLba(&self) -> windows_core::Result<super::LONG>;
    fn get_SectorCount(&self) -> windows_core::Result<super::LONG>;
    fn get_LastReadLba(&self) -> windows_core::Result<super::LONG>;
    fn get_LastWrittenLba(&self) -> windows_core::Result<super::LONG>;
    fn get_TotalSystemBuffer(&self) -> windows_core::Result<super::LONG>;
    fn get_UsedSystemBuffer(&self) -> windows_core::Result<super::LONG>;
    fn get_FreeSystemBuffer(&self) -> windows_core::Result<super::LONG>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWriteEngine2EventArgs_Vtbl {
    pub const fn new<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_StartLba<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2EventArgs_Impl::get_StartLba(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SectorCount<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2EventArgs_Impl::get_SectorCount(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_LastReadLba<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2EventArgs_Impl::get_LastReadLba(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_LastWrittenLba<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2EventArgs_Impl::get_LastWrittenLba(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_TotalSystemBuffer<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2EventArgs_Impl::get_TotalSystemBuffer(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_UsedSystemBuffer<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2EventArgs_Impl::get_UsedSystemBuffer(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_FreeSystemBuffer<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2EventArgs_Impl::get_FreeSystemBuffer(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_StartLba: get_StartLba::<Identity, OFFSET>,
            get_SectorCount: get_SectorCount::<Identity, OFFSET>,
            get_LastReadLba: get_LastReadLba::<Identity, OFFSET>,
            get_LastWrittenLba: get_LastWrittenLba::<Identity, OFFSET>,
            get_TotalSystemBuffer: get_TotalSystemBuffer::<Identity, OFFSET>,
            get_UsedSystemBuffer: get_UsedSystemBuffer::<Identity, OFFSET>,
            get_FreeSystemBuffer: get_FreeSystemBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWriteEngine2EventArgs as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWriteEngine2EventArgs {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWriteSpeedDescriptor, IWriteSpeedDescriptor_Vtbl, 0x27354144_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWriteSpeedDescriptor {
    type Target = super::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWriteSpeedDescriptor, windows_core::IUnknown, super::IDispatch);
#[cfg(feature = "oaidl")]
impl IWriteSpeedDescriptor {
    pub unsafe fn get_MediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_MediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn get_RotationTypeIsPureCAV(&self) -> windows_core::Result<super::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_WriteSpeed(&self) -> windows_core::Result<super::LONG> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWriteSpeedDescriptor_Vtbl {
    pub base__: super::IDispatch_Vtbl,
    pub get_MediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub get_RotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    get_RotationTypeIsPureCAV: usize,
    #[cfg(feature = "shared")]
    pub get_WriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LONG) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_WriteSpeed: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWriteSpeedDescriptor_Impl: super::IDispatch_Impl {
    fn get_MediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn get_RotationTypeIsPureCAV(&self) -> windows_core::Result<super::VARIANT_BOOL>;
    fn get_WriteSpeed(&self) -> windows_core::Result<super::LONG>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWriteSpeedDescriptor_Vtbl {
    pub const fn new<Identity: IWriteSpeedDescriptor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_MediaType<Identity: IWriteSpeedDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteSpeedDescriptor_Impl::get_MediaType(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RotationTypeIsPureCAV<Identity: IWriteSpeedDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteSpeedDescriptor_Impl::get_RotationTypeIsPureCAV(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_WriteSpeed<Identity: IWriteSpeedDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::LONG) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteSpeedDescriptor_Impl::get_WriteSpeed(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            get_MediaType: get_MediaType::<Identity, OFFSET>,
            get_RotationTypeIsPureCAV: get_RotationTypeIsPureCAV::<Identity, OFFSET>,
            get_WriteSpeed: get_WriteSpeed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWriteSpeedDescriptor as windows_core::Interface>::IID || iid == &<super::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWriteSpeedDescriptor {}
pub const MsftDiscFormat2Data: windows_core::GUID = windows_core::GUID::from_u128(0x2735412a_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftDiscFormat2Erase: windows_core::GUID = windows_core::GUID::from_u128(0x2735412b_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftDiscFormat2RawCD: windows_core::GUID = windows_core::GUID::from_u128(0x27354128_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftDiscFormat2TrackAtOnce: windows_core::GUID = windows_core::GUID::from_u128(0x27354129_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftDiscMaster2: windows_core::GUID = windows_core::GUID::from_u128(0x2735412e_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftDiscRecorder2: windows_core::GUID = windows_core::GUID::from_u128(0x2735412d_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftMultisessionRandomWrite: windows_core::GUID = windows_core::GUID::from_u128(0xb507ca24_2204_11dd_966a_001aa01bbc58);
pub const MsftMultisessionSequential: windows_core::GUID = windows_core::GUID::from_u128(0x27354122_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftRawCDImageCreator: windows_core::GUID = windows_core::GUID::from_u128(0x25983561_9d65_49ce_b335_40630d901227);
pub const MsftStreamConcatenate: windows_core::GUID = windows_core::GUID::from_u128(0x27354125_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftStreamInterleave: windows_core::GUID = windows_core::GUID::from_u128(0x27354124_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftStreamPrng001: windows_core::GUID = windows_core::GUID::from_u128(0x27354126_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftStreamZero: windows_core::GUID = windows_core::GUID::from_u128(0x27354127_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftWriteEngine2: windows_core::GUID = windows_core::GUID::from_u128(0x2735412c_7f64_5b0f_8f00_5d77afbe261e);
pub const MsftWriteSpeedDescriptor: windows_core::GUID = windows_core::GUID::from_u128(0x27354123_7f64_5b0f_8f00_5d77afbe261e);
pub type PIMAPI_BURN_VERIFICATION_LEVEL = *mut IMAPI_BURN_VERIFICATION_LEVEL;
pub type PIMAPI_CD_SECTOR_TYPE = *mut IMAPI_CD_SECTOR_TYPE;
pub type PIMAPI_CD_TRACK_DIGITAL_COPY_SETTING = *mut IMAPI_CD_TRACK_DIGITAL_COPY_SETTING;
pub type PIMAPI_FEATURE_PAGE_TYPE = *mut IMAPI_FEATURE_PAGE_TYPE;
pub type PIMAPI_FORMAT2_DATA_MEDIA_STATE = *mut IMAPI_FORMAT2_DATA_MEDIA_STATE;
pub type PIMAPI_FORMAT2_DATA_WRITE_ACTION = *mut IMAPI_FORMAT2_DATA_WRITE_ACTION;
pub type PIMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE = *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE;
pub type PIMAPI_FORMAT2_RAW_CD_WRITE_ACTION = *mut IMAPI_FORMAT2_RAW_CD_WRITE_ACTION;
pub type PIMAPI_FORMAT2_TAO_WRITE_ACTION = *mut IMAPI_FORMAT2_TAO_WRITE_ACTION;
pub type PIMAPI_MEDIA_PHYSICAL_TYPE = *mut IMAPI_MEDIA_PHYSICAL_TYPE;
pub type PIMAPI_MEDIA_WRITE_PROTECT_STATE = *mut IMAPI_MEDIA_WRITE_PROTECT_STATE;
pub type PIMAPI_MODE_PAGE_REQUEST_TYPE = *mut IMAPI_MODE_PAGE_REQUEST_TYPE;
pub type PIMAPI_MODE_PAGE_TYPE = *mut IMAPI_MODE_PAGE_TYPE;
pub type PIMAPI_PROFILE_TYPE = *mut IMAPI_PROFILE_TYPE;
pub type PIMAPI_READ_TRACK_ADDRESS_TYPE = *mut IMAPI_READ_TRACK_ADDRESS_TYPE;
pub type ULONG_IMAPI2_ADAPTER_DESCRIPTOR = u32;
pub type ULONG_IMAPI2_ALL_FEATURE_PAGES = u32;
pub type ULONG_IMAPI2_ALL_MODE_PAGES = u32;
pub type ULONG_IMAPI2_ALL_PROFILES = u32;
pub type ULONG_IMAPI2_DEVICE_DESCRIPTOR = u32;
pub type ULONG_IMAPI2_DISC_INFORMATION = u32;
pub type ULONG_IMAPI2_DVD_STRUCTURE = u32;
pub type ULONG_IMAPI2_FEATURE_PAGE = u32;
pub type ULONG_IMAPI2_MODE_PAGE = u32;
pub type ULONG_IMAPI2_NONZERO = u32;
pub type ULONG_IMAPI2_NOT_NEGATIVE = u32;
pub type ULONG_IMAPI2_TRACK_INFORMATION = u32;
