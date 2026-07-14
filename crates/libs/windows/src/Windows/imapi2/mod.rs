#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(DDiscFormat2DataEvents, DDiscFormat2DataEvents_Vtbl, 0x2735413c_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for DDiscFormat2DataEvents {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(DDiscFormat2DataEvents, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl DDiscFormat2DataEvents {
    pub unsafe fn Update<P0, P1>(&self, object: P0, progress: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oaidl::IDispatch>,
        P1: windows_core::Param<super::oaidl::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), object.param().abi(), progress.param().abi()) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct DDiscFormat2DataEvents_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait DDiscFormat2DataEvents_Impl: super::oaidl::IDispatch_Impl {
    fn Update(&self, object: windows_core::Ref<super::oaidl::IDispatch>, progress: windows_core::Ref<super::oaidl::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl DDiscFormat2DataEvents_Vtbl {
    pub const fn new<Identity: DDiscFormat2DataEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Update<Identity: DDiscFormat2DataEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, progress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                DDiscFormat2DataEvents_Impl::Update(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&progress)).into()
            }
        }
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DDiscFormat2DataEvents as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for DDiscFormat2DataEvents {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(DDiscFormat2EraseEvents, DDiscFormat2EraseEvents_Vtbl, 0x2735413a_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for DDiscFormat2EraseEvents {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(DDiscFormat2EraseEvents, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl DDiscFormat2EraseEvents {
    pub unsafe fn Update<P0>(&self, object: P0, elapsedseconds: i32, estimatedtotalseconds: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oaidl::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), object.param().abi(), elapsedseconds, estimatedtotalseconds) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct DDiscFormat2EraseEvents_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait DDiscFormat2EraseEvents_Impl: super::oaidl::IDispatch_Impl {
    fn Update(&self, object: windows_core::Ref<super::oaidl::IDispatch>, elapsedseconds: i32, estimatedtotalseconds: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl DDiscFormat2EraseEvents_Vtbl {
    pub const fn new<Identity: DDiscFormat2EraseEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Update<Identity: DDiscFormat2EraseEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, elapsedseconds: i32, estimatedtotalseconds: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                DDiscFormat2EraseEvents_Impl::Update(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&elapsedseconds), core::mem::transmute_copy(&estimatedtotalseconds)).into()
            }
        }
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DDiscFormat2EraseEvents as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for DDiscFormat2EraseEvents {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(DDiscFormat2RawCDEvents, DDiscFormat2RawCDEvents_Vtbl, 0x27354142_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for DDiscFormat2RawCDEvents {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(DDiscFormat2RawCDEvents, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl DDiscFormat2RawCDEvents {
    pub unsafe fn Update<P0, P1>(&self, object: P0, progress: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oaidl::IDispatch>,
        P1: windows_core::Param<super::oaidl::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), object.param().abi(), progress.param().abi()) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct DDiscFormat2RawCDEvents_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait DDiscFormat2RawCDEvents_Impl: super::oaidl::IDispatch_Impl {
    fn Update(&self, object: windows_core::Ref<super::oaidl::IDispatch>, progress: windows_core::Ref<super::oaidl::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl DDiscFormat2RawCDEvents_Vtbl {
    pub const fn new<Identity: DDiscFormat2RawCDEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Update<Identity: DDiscFormat2RawCDEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, progress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                DDiscFormat2RawCDEvents_Impl::Update(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&progress)).into()
            }
        }
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DDiscFormat2RawCDEvents as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for DDiscFormat2RawCDEvents {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(DDiscFormat2TrackAtOnceEvents, DDiscFormat2TrackAtOnceEvents_Vtbl, 0x2735413f_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for DDiscFormat2TrackAtOnceEvents {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(DDiscFormat2TrackAtOnceEvents, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl DDiscFormat2TrackAtOnceEvents {
    pub unsafe fn Update<P0, P1>(&self, object: P0, progress: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oaidl::IDispatch>,
        P1: windows_core::Param<super::oaidl::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), object.param().abi(), progress.param().abi()) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct DDiscFormat2TrackAtOnceEvents_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait DDiscFormat2TrackAtOnceEvents_Impl: super::oaidl::IDispatch_Impl {
    fn Update(&self, object: windows_core::Ref<super::oaidl::IDispatch>, progress: windows_core::Ref<super::oaidl::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl DDiscFormat2TrackAtOnceEvents_Vtbl {
    pub const fn new<Identity: DDiscFormat2TrackAtOnceEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Update<Identity: DDiscFormat2TrackAtOnceEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, progress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                DDiscFormat2TrackAtOnceEvents_Impl::Update(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&progress)).into()
            }
        }
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DDiscFormat2TrackAtOnceEvents as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for DDiscFormat2TrackAtOnceEvents {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(DDiscMaster2Events, DDiscMaster2Events_Vtbl, 0x27354131_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for DDiscMaster2Events {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(DDiscMaster2Events, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl DDiscMaster2Events {
    pub unsafe fn NotifyDeviceAdded<P0>(&self, object: P0, uniqueid: &windows_core::BSTR) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oaidl::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).NotifyDeviceAdded)(windows_core::Interface::as_raw(self), object.param().abi(), core::mem::transmute_copy(uniqueid)) }
    }
    pub unsafe fn NotifyDeviceRemoved<P0>(&self, object: P0, uniqueid: &windows_core::BSTR) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oaidl::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).NotifyDeviceRemoved)(windows_core::Interface::as_raw(self), object.param().abi(), core::mem::transmute_copy(uniqueid)) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct DDiscMaster2Events_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub NotifyDeviceAdded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NotifyDeviceRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait DDiscMaster2Events_Impl: super::oaidl::IDispatch_Impl {
    fn NotifyDeviceAdded(&self, object: windows_core::Ref<super::oaidl::IDispatch>, uniqueid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn NotifyDeviceRemoved(&self, object: windows_core::Ref<super::oaidl::IDispatch>, uniqueid: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            NotifyDeviceAdded: NotifyDeviceAdded::<Identity, OFFSET>,
            NotifyDeviceRemoved: NotifyDeviceRemoved::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DDiscMaster2Events as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for DDiscMaster2Events {}
pub const DISPID_DDISCFORMAT2DATAEVENTS_UPDATE: u32 = 512;
pub const DISPID_DDISCFORMAT2RAWCDEVENTS_UPDATE: u32 = 512;
pub const DISPID_DDISCFORMAT2TAOEVENTS_UPDATE: u32 = 512;
pub const DISPID_DDISCMASTER2EVENTS_DEVICEADDED: u32 = 256;
pub const DISPID_DDISCMASTER2EVENTS_DEVICEREMOVED: u32 = 257;
pub const DISPID_DWRITEENGINE2EVENTS_UPDATE: u32 = 256;
pub const DISPID_IBLOCKRANGELIST_BLOCKRANGES: u32 = 256;
pub const DISPID_IBLOCKRANGE_ENDLBA: u32 = 257;
pub const DISPID_IBLOCKRANGE_STARTLBA: u32 = 256;
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_CURRENTACTION: u32 = 771;
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ELAPSEDTIME: u32 = 768;
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 769;
pub const DISPID_IDISCFORMAT2DATAEVENTARGS_ESTIMATEDTOTALTIME: u32 = 770;
pub const DISPID_IDISCFORMAT2DATA_BUFFERUNDERRUNFREEDISABLED: u32 = 257;
pub const DISPID_IDISCFORMAT2DATA_CANCELWRITE: u32 = 513;
pub const DISPID_IDISCFORMAT2DATA_CLIENTNAME: u32 = 272;
pub const DISPID_IDISCFORMAT2DATA_CURRENTMEDIASTATUS: u32 = 262;
pub const DISPID_IDISCFORMAT2DATA_CURRENTMEDIATYPE: u32 = 271;
pub const DISPID_IDISCFORMAT2DATA_CURRENTROTATIONTYPEISPURECAV: u32 = 276;
pub const DISPID_IDISCFORMAT2DATA_CURRENTWRITESPEED: u32 = 275;
pub const DISPID_IDISCFORMAT2DATA_DISABLEDVDCOMPATIBILITYMODE: u32 = 270;
pub const DISPID_IDISCFORMAT2DATA_FORCEMEDIATOBECLOSED: u32 = 269;
pub const DISPID_IDISCFORMAT2DATA_FORCEOVERWRITE: u32 = 279;
pub const DISPID_IDISCFORMAT2DATA_FREESECTORS: u32 = 265;
pub const DISPID_IDISCFORMAT2DATA_LASTSECTOROFPREVIOUSSESSION: u32 = 268;
pub const DISPID_IDISCFORMAT2DATA_MUTLISESSIONINTERFACES: u32 = 280;
pub const DISPID_IDISCFORMAT2DATA_NEXTWRITABLEADDRESS: u32 = 266;
pub const DISPID_IDISCFORMAT2DATA_POSTGAPALREADYINIMAGE: u32 = 260;
pub const DISPID_IDISCFORMAT2DATA_RECORDER: u32 = 256;
pub const DISPID_IDISCFORMAT2DATA_REQUESTEDROTATIONTYPEISPURECAV: u32 = 274;
pub const DISPID_IDISCFORMAT2DATA_REQUESTEDWRITESPEED: u32 = 273;
pub const DISPID_IDISCFORMAT2DATA_SETWRITESPEED: u32 = 514;
pub const DISPID_IDISCFORMAT2DATA_STARTSECTOROFPREVIOUSSESSION: u32 = 267;
pub const DISPID_IDISCFORMAT2DATA_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 278;
pub const DISPID_IDISCFORMAT2DATA_SUPPORTEDWRITESPEEDS: u32 = 277;
pub const DISPID_IDISCFORMAT2DATA_TOTALSECTORS: u32 = 264;
pub const DISPID_IDISCFORMAT2DATA_WRITE: u32 = 512;
pub const DISPID_IDISCFORMAT2DATA_WRITEPROTECTSTATUS: u32 = 263;
pub const DISPID_IDISCFORMAT2ERASEEVENTS_UPDATE: u32 = 512;
pub const DISPID_IDISCFORMAT2ERASE_CLIENTNAME: u32 = 259;
pub const DISPID_IDISCFORMAT2ERASE_ERASEMEDIA: u32 = 513;
pub const DISPID_IDISCFORMAT2ERASE_FULLERASE: u32 = 257;
pub const DISPID_IDISCFORMAT2ERASE_MEDIATYPE: u32 = 258;
pub const DISPID_IDISCFORMAT2ERASE_RECORDER: u32 = 256;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_CURRENTACTION: u32 = 769;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_CURRENTTRACKNUMBER: u32 = 768;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ELAPSEDTIME: u32 = 768;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 769;
pub const DISPID_IDISCFORMAT2RAWCDEVENTARGS_ESTIMATEDTOTALTIME: u32 = 770;
pub const DISPID_IDISCFORMAT2RAWCD_BUFFERUNDERRUNFREEDISABLED: u32 = 258;
pub const DISPID_IDISCFORMAT2RAWCD_CANCELWRITE: u32 = 515;
pub const DISPID_IDISCFORMAT2RAWCD_CLIENTNAME: u32 = 266;
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTMEDIATYPE: u32 = 261;
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTROTATIONTYPEISPURECAV: u32 = 270;
pub const DISPID_IDISCFORMAT2RAWCD_CURRENTWRITESPEED: u32 = 269;
pub const DISPID_IDISCFORMAT2RAWCD_LASTPOSSIBLESTARTOFLEADOUT: u32 = 260;
pub const DISPID_IDISCFORMAT2RAWCD_PREPAREMEDIA: u32 = 512;
pub const DISPID_IDISCFORMAT2RAWCD_RECORDER: u32 = 256;
pub const DISPID_IDISCFORMAT2RAWCD_RELEASEMEDIA: u32 = 516;
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDDATASECTORTYPE: u32 = 265;
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDROTATIONTYPEISPURECAV: u32 = 268;
pub const DISPID_IDISCFORMAT2RAWCD_REQUESTEDWRITESPEED: u32 = 267;
pub const DISPID_IDISCFORMAT2RAWCD_SETWRITESPEED: u32 = 517;
pub const DISPID_IDISCFORMAT2RAWCD_STARTOFNEXTSESSION: u32 = 259;
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDDATASECTORTYPES: u32 = 264;
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 272;
pub const DISPID_IDISCFORMAT2RAWCD_SUPPORTEDWRITESPEEDS: u32 = 271;
pub const DISPID_IDISCFORMAT2RAWCD_WRITEMEDIA: u32 = 513;
pub const DISPID_IDISCFORMAT2RAWCD_WRITEMEDIAWITHVALIDATION: u32 = 514;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_CURRENTACTION: u32 = 769;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_CURRENTTRACKNUMBER: u32 = 768;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ELAPSEDTIME: u32 = 770;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ESTIMATEDREMAININGTIME: u32 = 771;
pub const DISPID_IDISCFORMAT2TAOEVENTARGS_ESTIMATEDTOTALTIME: u32 = 772;
pub const DISPID_IDISCFORMAT2TAO_ADDAUDIOTRACK: u32 = 513;
pub const DISPID_IDISCFORMAT2TAO_BUFFERUNDERRUNFREEDISABLED: u32 = 258;
pub const DISPID_IDISCFORMAT2TAO_CANCELADDTRACK: u32 = 514;
pub const DISPID_IDISCFORMAT2TAO_CLIENTNAME: u32 = 270;
pub const DISPID_IDISCFORMAT2TAO_CURRENTMEDIATYPE: u32 = 267;
pub const DISPID_IDISCFORMAT2TAO_CURRENTROTATIONTYPEISPURECAV: u32 = 274;
pub const DISPID_IDISCFORMAT2TAO_CURRENTWRITESPEED: u32 = 273;
pub const DISPID_IDISCFORMAT2TAO_DONOTFINALIZEMEDIA: u32 = 263;
pub const DISPID_IDISCFORMAT2TAO_EXPECTEDTABLEOFCONTENTS: u32 = 266;
pub const DISPID_IDISCFORMAT2TAO_FINISHMEDIA: u32 = 515;
pub const DISPID_IDISCFORMAT2TAO_FREESECTORSONMEDIA: u32 = 261;
pub const DISPID_IDISCFORMAT2TAO_NUMBEROFEXISTINGTRACKS: u32 = 259;
pub const DISPID_IDISCFORMAT2TAO_PREPAREMEDIA: u32 = 512;
pub const DISPID_IDISCFORMAT2TAO_RECORDER: u32 = 256;
pub const DISPID_IDISCFORMAT2TAO_REQUESTEDROTATIONTYPEISPURECAV: u32 = 272;
pub const DISPID_IDISCFORMAT2TAO_REQUESTEDWRITESPEED: u32 = 271;
pub const DISPID_IDISCFORMAT2TAO_SETWRITESPEED: u32 = 516;
pub const DISPID_IDISCFORMAT2TAO_SUPPORTEDWRITESPEEDDESCRIPTORS: u32 = 276;
pub const DISPID_IDISCFORMAT2TAO_SUPPORTEDWRITESPEEDS: u32 = 275;
pub const DISPID_IDISCFORMAT2TAO_TOTALSECTORSONMEDIA: u32 = 260;
pub const DISPID_IDISCFORMAT2TAO_USEDSECTORSONMEDIA: u32 = 262;
pub const DISPID_IDISCFORMAT2_MEDIAHEURISTICALLYBLANK: u32 = 1793;
pub const DISPID_IDISCFORMAT2_MEDIAPHYSICALLYBLANK: u32 = 1792;
pub const DISPID_IDISCFORMAT2_MEDIASUPPORTED: u32 = 2049;
pub const DISPID_IDISCFORMAT2_RECORDERSUPPORTED: u32 = 2048;
pub const DISPID_IDISCFORMAT2_SUPPORTEDMEDIATYPES: u32 = 1794;
pub const DISPID_IDISCRECORDER2_ACQUIREEXCLUSIVEACCESS: u32 = 258;
pub const DISPID_IDISCRECORDER2_ACTIVEDISCRECORDER: u32 = 0;
pub const DISPID_IDISCRECORDER2_CLOSETRAY: u32 = 257;
pub const DISPID_IDISCRECORDER2_CURRENTFEATUREPAGES: u32 = 521;
pub const DISPID_IDISCRECORDER2_CURRENTPROFILES: u32 = 523;
pub const DISPID_IDISCRECORDER2_DEVICECANLOADMEDIA: u32 = 518;
pub const DISPID_IDISCRECORDER2_DISABLEMCN: u32 = 260;
pub const DISPID_IDISCRECORDER2_EJECTMEDIA: u32 = 256;
pub const DISPID_IDISCRECORDER2_ENABLEMCN: u32 = 261;
pub const DISPID_IDISCRECORDER2_EXCLUSIVEACCESSOWNER: u32 = 525;
pub const DISPID_IDISCRECORDER2_INITIALIZEDISCRECORDER: u32 = 262;
pub const DISPID_IDISCRECORDER2_LEGACYDEVICENUMBER: u32 = 519;
pub const DISPID_IDISCRECORDER2_PRODUCTID: u32 = 514;
pub const DISPID_IDISCRECORDER2_PRODUCTREVISION: u32 = 515;
pub const DISPID_IDISCRECORDER2_RELEASEEXCLUSIVEACCESS: u32 = 259;
pub const DISPID_IDISCRECORDER2_SUPPORTEDFEATUREPAGES: u32 = 520;
pub const DISPID_IDISCRECORDER2_SUPPORTEDMODEPAGES: u32 = 524;
pub const DISPID_IDISCRECORDER2_SUPPORTEDPROFILES: u32 = 522;
pub const DISPID_IDISCRECORDER2_VENDORID: u32 = 513;
pub const DISPID_IDISCRECORDER2_VOLUMENAME: u32 = 516;
pub const DISPID_IDISCRECORDER2_VOLUMEPATHNAMES: u32 = 517;
pub const DISPID_IMULTISESSION_FIRSTDATASESSION: u32 = 512;
pub const DISPID_IMULTISESSION_FREESECTORS: u32 = 516;
pub const DISPID_IMULTISESSION_IMPORTRECORDER: u32 = 258;
pub const DISPID_IMULTISESSION_INUSE: u32 = 257;
pub const DISPID_IMULTISESSION_LASTSECTOROFPREVIOUSSESSION: u32 = 514;
pub const DISPID_IMULTISESSION_LASTWRITTENADDRESS: u32 = 518;
pub const DISPID_IMULTISESSION_NEXTWRITABLEADDRESS: u32 = 515;
pub const DISPID_IMULTISESSION_SECTORSONMEDIA: u32 = 519;
pub const DISPID_IMULTISESSION_STARTSECTOROFPREVIOUSSESSION: u32 = 513;
pub const DISPID_IMULTISESSION_SUPPORTEDONCURRENTMEDIA: u32 = 256;
pub const DISPID_IMULTISESSION_WRITEUNITSIZE: u32 = 517;
pub const DISPID_IRAWCDIMAGECREATOR_ADDSPECIALPREGAP: u32 = 514;
pub const DISPID_IRAWCDIMAGECREATOR_ADDSUBCODERWGENERATOR: u32 = 515;
pub const DISPID_IRAWCDIMAGECREATOR_ADDTRACK: u32 = 513;
pub const DISPID_IRAWCDIMAGECREATOR_CREATERESULTIMAGE: u32 = 512;
pub const DISPID_IRAWCDIMAGECREATOR_DISABLEGAPLESSAUDIO: u32 = 259;
pub const DISPID_IRAWCDIMAGECREATOR_EXPECTEDTABLEOFCONTENTS: u32 = 265;
pub const DISPID_IRAWCDIMAGECREATOR_MEDIACATALOGNUMBER: u32 = 260;
pub const DISPID_IRAWCDIMAGECREATOR_NUMBEROFEXISTINGTRACKS: u32 = 263;
pub const DISPID_IRAWCDIMAGECREATOR_RESULTINGIMAGETYPE: u32 = 256;
pub const DISPID_IRAWCDIMAGECREATOR_STARTINGTRACKNUMBER: u32 = 261;
pub const DISPID_IRAWCDIMAGECREATOR_STARTOFLEADOUT: u32 = 257;
pub const DISPID_IRAWCDIMAGECREATOR_STARTOFLEADOUTLIMIT: u32 = 258;
pub const DISPID_IRAWCDIMAGECREATOR_TRACKINFO: u32 = 262;
pub const DISPID_IRAWCDIMAGECREATOR_USEDSECTORSONDISC: u32 = 264;
pub const DISPID_IRAWCDTRACKINFO_AUDIOHASPREEMPHASIS: u32 = 262;
pub const DISPID_IRAWCDTRACKINFO_DIGITALAUDIOCOPYSETTING: u32 = 261;
pub const DISPID_IRAWCDTRACKINFO_ISRC: u32 = 260;
pub const DISPID_IRAWCDTRACKINFO_SECTORCOUNT: u32 = 257;
pub const DISPID_IRAWCDTRACKINFO_SECTORTYPE: u32 = 259;
pub const DISPID_IRAWCDTRACKINFO_STARTINGLBA: u32 = 256;
pub const DISPID_IRAWCDTRACKINFO_TRACKNUMBER: u32 = 258;
pub const DISPID_IWRITEENGINE2EVENTARGS_FREESYSTEMBUFFER: u32 = 264;
pub const DISPID_IWRITEENGINE2EVENTARGS_LASTREADLBA: u32 = 258;
pub const DISPID_IWRITEENGINE2EVENTARGS_LASTWRITTENLBA: u32 = 259;
pub const DISPID_IWRITEENGINE2EVENTARGS_SECTORCOUNT: u32 = 257;
pub const DISPID_IWRITEENGINE2EVENTARGS_STARTLBA: u32 = 256;
pub const DISPID_IWRITEENGINE2EVENTARGS_TOTALDEVICEBUFFER: u32 = 260;
pub const DISPID_IWRITEENGINE2EVENTARGS_TOTALSYSTEMBUFFER: u32 = 262;
pub const DISPID_IWRITEENGINE2EVENTARGS_USEDDEVICEBUFFER: u32 = 261;
pub const DISPID_IWRITEENGINE2EVENTARGS_USEDSYSTEMBUFFER: u32 = 263;
pub const DISPID_IWRITEENGINE2_BYTESPERSECTOR: u32 = 260;
pub const DISPID_IWRITEENGINE2_CANCELWRITE: u32 = 513;
pub const DISPID_IWRITEENGINE2_DISCRECORDER: u32 = 256;
pub const DISPID_IWRITEENGINE2_ENDINGSECTORSPERSECOND: u32 = 259;
pub const DISPID_IWRITEENGINE2_STARTINGSECTORSPERSECOND: u32 = 258;
pub const DISPID_IWRITEENGINE2_USESTREAMINGWRITE12: u32 = 257;
pub const DISPID_IWRITEENGINE2_WRITEINPROGRESS: u32 = 261;
pub const DISPID_IWRITEENGINE2_WRITESECTION: u32 = 512;
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(DWriteEngine2Events, DWriteEngine2Events_Vtbl, 0x27354137_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for DWriteEngine2Events {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(DWriteEngine2Events, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl DWriteEngine2Events {
    pub unsafe fn Update<P0, P1>(&self, object: P0, progress: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::oaidl::IDispatch>,
        P1: windows_core::Param<super::oaidl::IDispatch>,
    {
        unsafe { (windows_core::Interface::vtable(self).Update)(windows_core::Interface::as_raw(self), object.param().abi(), progress.param().abi()) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct DWriteEngine2Events_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait DWriteEngine2Events_Impl: super::oaidl::IDispatch_Impl {
    fn Update(&self, object: windows_core::Ref<super::oaidl::IDispatch>, progress: windows_core::Ref<super::oaidl::IDispatch>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl DWriteEngine2Events_Vtbl {
    pub const fn new<Identity: DWriteEngine2Events_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Update<Identity: DWriteEngine2Events_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: *mut core::ffi::c_void, progress: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                DWriteEngine2Events_Impl::Update(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&progress)).into()
            }
        }
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(), Update: Update::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<DWriteEngine2Events as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for DWriteEngine2Events {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IBlockRange, IBlockRange_Vtbl, 0xb507ca25_2204_11dd_966a_001aa01bbc58);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IBlockRange {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IBlockRange, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IBlockRange {
    pub unsafe fn StartLba(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartLba)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn EndLba(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndLba)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IBlockRange_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub StartLba: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub EndLba: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IBlockRange_Impl: super::oaidl::IDispatch_Impl {
    fn StartLba(&self) -> windows_core::Result<i32>;
    fn EndLba(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IBlockRange_Vtbl {
    pub const fn new<Identity: IBlockRange_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartLba<Identity: IBlockRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBlockRange_Impl::StartLba(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn EndLba<Identity: IBlockRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBlockRange_Impl::EndLba(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(), StartLba: StartLba::<Identity, OFFSET>, EndLba: EndLba::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBlockRange as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IBlockRange {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IBlockRangeList, IBlockRangeList_Vtbl, 0xb507ca26_2204_11dd_966a_001aa01bbc58);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IBlockRangeList {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IBlockRangeList, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IBlockRangeList {
    pub unsafe fn BlockRanges(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BlockRanges)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IBlockRangeList_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub BlockRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IBlockRangeList_Impl: super::oaidl::IDispatch_Impl {
    fn BlockRanges(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IBlockRangeList_Vtbl {
    pub const fn new<Identity: IBlockRangeList_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BlockRanges<Identity: IBlockRangeList_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBlockRangeList_Impl::BlockRanges(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(), BlockRanges: BlockRanges::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBlockRangeList as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IBlockRangeList {}
windows_core::imp::define_interface!(IBurnVerification, IBurnVerification_Vtbl, 0xd2ffd834_958b_426d_8470_2a13879c6a91);
windows_core::imp::interface_hierarchy!(IBurnVerification, windows_core::IUnknown);
impl IBurnVerification {
    pub unsafe fn SetBurnVerificationLevel(&self, value: IMAPI_BURN_VERIFICATION_LEVEL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBurnVerificationLevel)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn BurnVerificationLevel(&self) -> windows_core::Result<IMAPI_BURN_VERIFICATION_LEVEL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BurnVerificationLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBurnVerification_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetBurnVerificationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_BURN_VERIFICATION_LEVEL) -> windows_core::HRESULT,
    pub BurnVerificationLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_BURN_VERIFICATION_LEVEL) -> windows_core::HRESULT,
}
pub trait IBurnVerification_Impl: windows_core::IUnknownImpl {
    fn SetBurnVerificationLevel(&self, value: IMAPI_BURN_VERIFICATION_LEVEL) -> windows_core::Result<()>;
    fn BurnVerificationLevel(&self) -> windows_core::Result<IMAPI_BURN_VERIFICATION_LEVEL>;
}
impl IBurnVerification_Vtbl {
    pub const fn new<Identity: IBurnVerification_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetBurnVerificationLevel<Identity: IBurnVerification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: IMAPI_BURN_VERIFICATION_LEVEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBurnVerification_Impl::SetBurnVerificationLevel(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn BurnVerificationLevel<Identity: IBurnVerification_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_BURN_VERIFICATION_LEVEL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBurnVerification_Impl::BurnVerificationLevel(this) {
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
            SetBurnVerificationLevel: SetBurnVerificationLevel::<Identity, OFFSET>,
            BurnVerificationLevel: BurnVerificationLevel::<Identity, OFFSET>,
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
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDiscFormat2, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IDiscFormat2 {
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsRecorderSupported<P0>(&self, recorder: P0) -> windows_core::Result<super::wtypes::VARIANT_BOOL>
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsRecorderSupported)(windows_core::Interface::as_raw(self), recorder.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsCurrentMediaSupported<P0>(&self, recorder: P0) -> windows_core::Result<super::wtypes::VARIANT_BOOL>
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsCurrentMediaSupported)(windows_core::Interface::as_raw(self), recorder.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn MediaPhysicallyBlank(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MediaPhysicallyBlank)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn MediaHeuristicallyBlank(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MediaHeuristicallyBlank)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SupportedMediaTypes(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedMediaTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(feature = "wtypes")]
    pub IsRecorderSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsRecorderSupported: usize,
    #[cfg(feature = "wtypes")]
    pub IsCurrentMediaSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsCurrentMediaSupported: usize,
    #[cfg(feature = "wtypes")]
    pub MediaPhysicallyBlank: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    MediaPhysicallyBlank: usize,
    #[cfg(feature = "wtypes")]
    pub MediaHeuristicallyBlank: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    MediaHeuristicallyBlank: usize,
    pub SupportedMediaTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDiscFormat2_Impl: super::oaidl::IDispatch_Impl {
    fn IsRecorderSupported(&self, recorder: windows_core::Ref<IDiscRecorder2>) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn IsCurrentMediaSupported(&self, recorder: windows_core::Ref<IDiscRecorder2>) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn MediaPhysicallyBlank(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn MediaHeuristicallyBlank(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SupportedMediaTypes(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDiscFormat2_Vtbl {
    pub const fn new<Identity: IDiscFormat2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsRecorderSupported<Identity: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, recorder: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
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
        unsafe extern "system" fn IsCurrentMediaSupported<Identity: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, recorder: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
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
        unsafe extern "system" fn MediaPhysicallyBlank<Identity: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2_Impl::MediaPhysicallyBlank(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MediaHeuristicallyBlank<Identity: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2_Impl::MediaHeuristicallyBlank(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportedMediaTypes<Identity: IDiscFormat2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2_Impl::SupportedMediaTypes(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            IsRecorderSupported: IsRecorderSupported::<Identity, OFFSET>,
            IsCurrentMediaSupported: IsCurrentMediaSupported::<Identity, OFFSET>,
            MediaPhysicallyBlank: MediaPhysicallyBlank::<Identity, OFFSET>,
            MediaHeuristicallyBlank: MediaHeuristicallyBlank::<Identity, OFFSET>,
            SupportedMediaTypes: SupportedMediaTypes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
windows_core::imp::interface_hierarchy!(IDiscFormat2Data, windows_core::IUnknown, super::oaidl::IDispatch, IDiscFormat2);
#[cfg(feature = "oaidl")]
impl IDiscFormat2Data {
    pub unsafe fn SetRecorder<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRecorder)(windows_core::Interface::as_raw(self), value.param().abi()) }
    }
    pub unsafe fn Recorder(&self) -> windows_core::Result<IDiscRecorder2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Recorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetBufferUnderrunFreeDisabled(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBufferUnderrunFreeDisabled)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn BufferUnderrunFreeDisabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BufferUnderrunFreeDisabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetPostgapAlreadyInImage(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetPostgapAlreadyInImage)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn PostgapAlreadyInImage(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PostgapAlreadyInImage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentMediaStatus(&self) -> windows_core::Result<IMAPI_FORMAT2_DATA_MEDIA_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentMediaStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn WriteProtectStatus(&self) -> windows_core::Result<IMAPI_MEDIA_WRITE_PROTECT_STATE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WriteProtectStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TotalSectorsOnMedia(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TotalSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FreeSectorsOnMedia(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FreeSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn NextWritableAddress(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NextWritableAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StartAddressOfPreviousSession(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartAddressOfPreviousSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LastWrittenAddressOfPreviousSession(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastWrittenAddressOfPreviousSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetForceMediaToBeClosed(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetForceMediaToBeClosed)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn ForceMediaToBeClosed(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ForceMediaToBeClosed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetDisableConsumerDvdCompatibilityMode(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDisableConsumerDvdCompatibilityMode)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn DisableConsumerDvdCompatibilityMode(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisableConsumerDvdCompatibilityMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentPhysicalMediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetClientName(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClientName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn ClientName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClientName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RequestedWriteSpeed(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestedWriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn RequestedRotationTypeIsPureCAV(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestedRotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentWriteSpeed(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentWriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn CurrentRotationTypeIsPureCAV(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentRotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SupportedWriteSpeeds(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedWriteSpeeds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SupportedWriteSpeedDescriptors(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedWriteSpeedDescriptors)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetForceOverwrite(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetForceOverwrite)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn ForceOverwrite(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ForceOverwrite)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MultisessionInterfaces(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MultisessionInterfaces)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn Write<P0>(&self, data: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Write)(windows_core::Interface::as_raw(self), data.param().abi()) }
    }
    pub unsafe fn CancelWrite(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CancelWrite)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetWriteSpeed(&self, requestedsectorspersecond: i32, rotationtypeispurecav: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWriteSpeed)(windows_core::Interface::as_raw(self), requestedsectorspersecond, rotationtypeispurecav) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2Data_Vtbl {
    pub base__: IDiscFormat2_Vtbl,
    pub SetRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Recorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub SetBufferUnderrunFreeDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetBufferUnderrunFreeDisabled: usize,
    #[cfg(feature = "wtypes")]
    pub BufferUnderrunFreeDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    BufferUnderrunFreeDisabled: usize,
    #[cfg(feature = "wtypes")]
    pub SetPostgapAlreadyInImage: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetPostgapAlreadyInImage: usize,
    #[cfg(feature = "wtypes")]
    pub PostgapAlreadyInImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    PostgapAlreadyInImage: usize,
    pub CurrentMediaStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_FORMAT2_DATA_MEDIA_STATE) -> windows_core::HRESULT,
    pub WriteProtectStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_MEDIA_WRITE_PROTECT_STATE) -> windows_core::HRESULT,
    pub TotalSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub FreeSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NextWritableAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StartAddressOfPreviousSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastWrittenAddressOfPreviousSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub SetForceMediaToBeClosed: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetForceMediaToBeClosed: usize,
    #[cfg(feature = "wtypes")]
    pub ForceMediaToBeClosed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    ForceMediaToBeClosed: usize,
    #[cfg(feature = "wtypes")]
    pub SetDisableConsumerDvdCompatibilityMode: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetDisableConsumerDvdCompatibilityMode: usize,
    #[cfg(feature = "wtypes")]
    pub DisableConsumerDvdCompatibilityMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    DisableConsumerDvdCompatibilityMode: usize,
    pub CurrentPhysicalMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT,
    pub SetClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestedWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub RequestedRotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    RequestedRotationTypeIsPureCAV: usize,
    pub CurrentWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub CurrentRotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    CurrentRotationTypeIsPureCAV: usize,
    pub SupportedWriteSpeeds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub SupportedWriteSpeedDescriptors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub SetForceOverwrite: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetForceOverwrite: usize,
    #[cfg(feature = "wtypes")]
    pub ForceOverwrite: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    ForceOverwrite: usize,
    pub MultisessionInterfaces: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(feature = "objidlbase")]
    pub Write: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    Write: usize,
    pub CancelWrite: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub SetWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetWriteSpeed: usize,
}
#[cfg(all(feature = "oaidl", feature = "objidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDiscFormat2Data_Impl: IDiscFormat2_Impl {
    fn SetRecorder(&self, value: windows_core::Ref<IDiscRecorder2>) -> windows_core::Result<()>;
    fn Recorder(&self) -> windows_core::Result<IDiscRecorder2>;
    fn SetBufferUnderrunFreeDisabled(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn BufferUnderrunFreeDisabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetPostgapAlreadyInImage(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn PostgapAlreadyInImage(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn CurrentMediaStatus(&self) -> windows_core::Result<IMAPI_FORMAT2_DATA_MEDIA_STATE>;
    fn WriteProtectStatus(&self) -> windows_core::Result<IMAPI_MEDIA_WRITE_PROTECT_STATE>;
    fn TotalSectorsOnMedia(&self) -> windows_core::Result<i32>;
    fn FreeSectorsOnMedia(&self) -> windows_core::Result<i32>;
    fn NextWritableAddress(&self) -> windows_core::Result<i32>;
    fn StartAddressOfPreviousSession(&self) -> windows_core::Result<i32>;
    fn LastWrittenAddressOfPreviousSession(&self) -> windows_core::Result<i32>;
    fn SetForceMediaToBeClosed(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ForceMediaToBeClosed(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetDisableConsumerDvdCompatibilityMode(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DisableConsumerDvdCompatibilityMode(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SetClientName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ClientName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RequestedWriteSpeed(&self) -> windows_core::Result<i32>;
    fn RequestedRotationTypeIsPureCAV(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn CurrentWriteSpeed(&self) -> windows_core::Result<i32>;
    fn CurrentRotationTypeIsPureCAV(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SupportedWriteSpeeds(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn SupportedWriteSpeedDescriptors(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn SetForceOverwrite(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn ForceOverwrite(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn MultisessionInterfaces(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn Write(&self, data: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<()>;
    fn CancelWrite(&self) -> windows_core::Result<()>;
    fn SetWriteSpeed(&self, requestedsectorspersecond: i32, rotationtypeispurecav: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "objidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDiscFormat2Data_Vtbl {
    pub const fn new<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetRecorder<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::SetRecorder(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Recorder<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::Recorder(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::SetBufferUnderrunFreeDisabled(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::BufferUnderrunFreeDisabled(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPostgapAlreadyInImage<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::SetPostgapAlreadyInImage(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn PostgapAlreadyInImage<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::PostgapAlreadyInImage(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentMediaStatus<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_FORMAT2_DATA_MEDIA_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::CurrentMediaStatus(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WriteProtectStatus<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_MEDIA_WRITE_PROTECT_STATE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::WriteProtectStatus(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::TotalSectorsOnMedia(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::FreeSectorsOnMedia(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NextWritableAddress<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::NextWritableAddress(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartAddressOfPreviousSession<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::StartAddressOfPreviousSession(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastWrittenAddressOfPreviousSession<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::LastWrittenAddressOfPreviousSession(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetForceMediaToBeClosed<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::SetForceMediaToBeClosed(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn ForceMediaToBeClosed<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::ForceMediaToBeClosed(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisableConsumerDvdCompatibilityMode<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::SetDisableConsumerDvdCompatibilityMode(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn DisableConsumerDvdCompatibilityMode<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::DisableConsumerDvdCompatibilityMode(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::CurrentPhysicalMediaType(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClientName<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::SetClientName(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn ClientName<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::ClientName(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestedWriteSpeed<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::RequestedWriteSpeed(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::RequestedRotationTypeIsPureCAV(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentWriteSpeed<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::CurrentWriteSpeed(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::CurrentRotationTypeIsPureCAV(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedspeeds: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::SupportedWriteSpeeds(this) {
                    Ok(ok__) => {
                        supportedspeeds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::SupportedWriteSpeedDescriptors(this) {
                    Ok(ok__) => {
                        supportedspeeddescriptors.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetForceOverwrite<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::SetForceOverwrite(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn ForceOverwrite<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::ForceOverwrite(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MultisessionInterfaces<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Data_Impl::MultisessionInterfaces(this) {
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
        unsafe extern "system" fn SetWriteSpeed<Identity: IDiscFormat2Data_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Data_Impl::SetWriteSpeed(this, core::mem::transmute_copy(&requestedsectorspersecond), core::mem::transmute_copy(&rotationtypeispurecav)).into()
            }
        }
        Self {
            base__: IDiscFormat2_Vtbl::new::<Identity, OFFSET>(),
            SetRecorder: SetRecorder::<Identity, OFFSET>,
            Recorder: Recorder::<Identity, OFFSET>,
            SetBufferUnderrunFreeDisabled: SetBufferUnderrunFreeDisabled::<Identity, OFFSET>,
            BufferUnderrunFreeDisabled: BufferUnderrunFreeDisabled::<Identity, OFFSET>,
            SetPostgapAlreadyInImage: SetPostgapAlreadyInImage::<Identity, OFFSET>,
            PostgapAlreadyInImage: PostgapAlreadyInImage::<Identity, OFFSET>,
            CurrentMediaStatus: CurrentMediaStatus::<Identity, OFFSET>,
            WriteProtectStatus: WriteProtectStatus::<Identity, OFFSET>,
            TotalSectorsOnMedia: TotalSectorsOnMedia::<Identity, OFFSET>,
            FreeSectorsOnMedia: FreeSectorsOnMedia::<Identity, OFFSET>,
            NextWritableAddress: NextWritableAddress::<Identity, OFFSET>,
            StartAddressOfPreviousSession: StartAddressOfPreviousSession::<Identity, OFFSET>,
            LastWrittenAddressOfPreviousSession: LastWrittenAddressOfPreviousSession::<Identity, OFFSET>,
            SetForceMediaToBeClosed: SetForceMediaToBeClosed::<Identity, OFFSET>,
            ForceMediaToBeClosed: ForceMediaToBeClosed::<Identity, OFFSET>,
            SetDisableConsumerDvdCompatibilityMode: SetDisableConsumerDvdCompatibilityMode::<Identity, OFFSET>,
            DisableConsumerDvdCompatibilityMode: DisableConsumerDvdCompatibilityMode::<Identity, OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Identity, OFFSET>,
            SetClientName: SetClientName::<Identity, OFFSET>,
            ClientName: ClientName::<Identity, OFFSET>,
            RequestedWriteSpeed: RequestedWriteSpeed::<Identity, OFFSET>,
            RequestedRotationTypeIsPureCAV: RequestedRotationTypeIsPureCAV::<Identity, OFFSET>,
            CurrentWriteSpeed: CurrentWriteSpeed::<Identity, OFFSET>,
            CurrentRotationTypeIsPureCAV: CurrentRotationTypeIsPureCAV::<Identity, OFFSET>,
            SupportedWriteSpeeds: SupportedWriteSpeeds::<Identity, OFFSET>,
            SupportedWriteSpeedDescriptors: SupportedWriteSpeedDescriptors::<Identity, OFFSET>,
            SetForceOverwrite: SetForceOverwrite::<Identity, OFFSET>,
            ForceOverwrite: ForceOverwrite::<Identity, OFFSET>,
            MultisessionInterfaces: MultisessionInterfaces::<Identity, OFFSET>,
            Write: Write::<Identity, OFFSET>,
            CancelWrite: CancelWrite::<Identity, OFFSET>,
            SetWriteSpeed: SetWriteSpeed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2Data as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IDiscFormat2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "objidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
windows_core::imp::interface_hierarchy!(IDiscFormat2DataEventArgs, windows_core::IUnknown, super::oaidl::IDispatch, IWriteEngine2EventArgs);
#[cfg(feature = "oaidl")]
impl IDiscFormat2DataEventArgs {
    pub unsafe fn ElapsedTime(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElapsedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemainingTime(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemainingTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TotalTime(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TotalTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentAction(&self) -> windows_core::Result<IMAPI_FORMAT2_DATA_WRITE_ACTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2DataEventArgs_Vtbl {
    pub base__: IWriteEngine2EventArgs_Vtbl,
    pub ElapsedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RemainingTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TotalTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_FORMAT2_DATA_WRITE_ACTION) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDiscFormat2DataEventArgs_Impl: IWriteEngine2EventArgs_Impl {
    fn ElapsedTime(&self) -> windows_core::Result<i32>;
    fn RemainingTime(&self) -> windows_core::Result<i32>;
    fn TotalTime(&self) -> windows_core::Result<i32>;
    fn CurrentAction(&self) -> windows_core::Result<IMAPI_FORMAT2_DATA_WRITE_ACTION>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDiscFormat2DataEventArgs_Vtbl {
    pub const fn new<Identity: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ElapsedTime<Identity: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2DataEventArgs_Impl::ElapsedTime(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemainingTime<Identity: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2DataEventArgs_Impl::RemainingTime(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TotalTime<Identity: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2DataEventArgs_Impl::TotalTime(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentAction<Identity: IDiscFormat2DataEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_FORMAT2_DATA_WRITE_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2DataEventArgs_Impl::CurrentAction(this) {
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
            ElapsedTime: ElapsedTime::<Identity, OFFSET>,
            RemainingTime: RemainingTime::<Identity, OFFSET>,
            TotalTime: TotalTime::<Identity, OFFSET>,
            CurrentAction: CurrentAction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2DataEventArgs as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IWriteEngine2EventArgs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
windows_core::imp::interface_hierarchy!(IDiscFormat2Erase, windows_core::IUnknown, super::oaidl::IDispatch, IDiscFormat2);
#[cfg(feature = "oaidl")]
impl IDiscFormat2Erase {
    pub unsafe fn SetRecorder<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRecorder)(windows_core::Interface::as_raw(self), value.param().abi()) }
    }
    pub unsafe fn Recorder(&self) -> windows_core::Result<IDiscRecorder2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Recorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetFullErase(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFullErase)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn FullErase(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FullErase)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentPhysicalMediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetClientName(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClientName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn ClientName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClientName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
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
    pub SetRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Recorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub SetFullErase: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetFullErase: usize,
    #[cfg(feature = "wtypes")]
    pub FullErase: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    FullErase: usize,
    pub CurrentPhysicalMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT,
    pub SetClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EraseMedia: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDiscFormat2Erase_Impl: IDiscFormat2_Impl {
    fn SetRecorder(&self, value: windows_core::Ref<IDiscRecorder2>) -> windows_core::Result<()>;
    fn Recorder(&self) -> windows_core::Result<IDiscRecorder2>;
    fn SetFullErase(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn FullErase(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SetClientName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ClientName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn EraseMedia(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDiscFormat2Erase_Vtbl {
    pub const fn new<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetRecorder<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Erase_Impl::SetRecorder(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Recorder<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Erase_Impl::Recorder(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFullErase<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Erase_Impl::SetFullErase(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn FullErase<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Erase_Impl::FullErase(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Erase_Impl::CurrentPhysicalMediaType(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClientName<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2Erase_Impl::SetClientName(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn ClientName<Identity: IDiscFormat2Erase_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2Erase_Impl::ClientName(this) {
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
            SetRecorder: SetRecorder::<Identity, OFFSET>,
            Recorder: Recorder::<Identity, OFFSET>,
            SetFullErase: SetFullErase::<Identity, OFFSET>,
            FullErase: FullErase::<Identity, OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Identity, OFFSET>,
            SetClientName: SetClientName::<Identity, OFFSET>,
            ClientName: ClientName::<Identity, OFFSET>,
            EraseMedia: EraseMedia::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2Erase as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IDiscFormat2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
windows_core::imp::interface_hierarchy!(IDiscFormat2RawCD, windows_core::IUnknown, super::oaidl::IDispatch, IDiscFormat2);
#[cfg(feature = "oaidl")]
impl IDiscFormat2RawCD {
    pub unsafe fn PrepareMedia(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PrepareMedia)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn WriteMedia<P0>(&self, data: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteMedia)(windows_core::Interface::as_raw(self), data.param().abi()) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn WriteMedia2<P0>(&self, data: P0, streamleadinsectors: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteMedia2)(windows_core::Interface::as_raw(self), data.param().abi(), streamleadinsectors) }
    }
    pub unsafe fn CancelWrite(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CancelWrite)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ReleaseMedia(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseMedia)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetWriteSpeed(&self, requestedsectorspersecond: i32, rotationtypeispurecav: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWriteSpeed)(windows_core::Interface::as_raw(self), requestedsectorspersecond, rotationtypeispurecav) }
    }
    pub unsafe fn SetRecorder<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRecorder)(windows_core::Interface::as_raw(self), value.param().abi()) }
    }
    pub unsafe fn Recorder(&self) -> windows_core::Result<IDiscRecorder2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Recorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetBufferUnderrunFreeDisabled(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBufferUnderrunFreeDisabled)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn BufferUnderrunFreeDisabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BufferUnderrunFreeDisabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StartOfNextSession(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartOfNextSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LastPossibleStartOfLeadout(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastPossibleStartOfLeadout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentPhysicalMediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SupportedSectorTypes(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedSectorTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetRequestedSectorType(&self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRequestedSectorType)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn RequestedSectorType(&self) -> windows_core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestedSectorType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetClientName(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClientName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn ClientName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClientName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RequestedWriteSpeed(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestedWriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn RequestedRotationTypeIsPureCAV(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestedRotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentWriteSpeed(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentWriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn CurrentRotationTypeIsPureCAV(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentRotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SupportedWriteSpeeds(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedWriteSpeeds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SupportedWriteSpeedDescriptors(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedWriteSpeedDescriptors)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    #[cfg(feature = "objidlbase")]
    pub WriteMedia2: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    WriteMedia2: usize,
    pub CancelWrite: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReleaseMedia: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub SetWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetWriteSpeed: usize,
    pub SetRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Recorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub SetBufferUnderrunFreeDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetBufferUnderrunFreeDisabled: usize,
    #[cfg(feature = "wtypes")]
    pub BufferUnderrunFreeDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    BufferUnderrunFreeDisabled: usize,
    pub StartOfNextSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastPossibleStartOfLeadout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentPhysicalMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT,
    pub SupportedSectorTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub SetRequestedSectorType: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT,
    pub RequestedSectorType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT,
    pub SetClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestedWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub RequestedRotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    RequestedRotationTypeIsPureCAV: usize,
    pub CurrentWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub CurrentRotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    CurrentRotationTypeIsPureCAV: usize,
    pub SupportedWriteSpeeds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub SupportedWriteSpeedDescriptors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "objidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDiscFormat2RawCD_Impl: IDiscFormat2_Impl {
    fn PrepareMedia(&self) -> windows_core::Result<()>;
    fn WriteMedia(&self, data: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<()>;
    fn WriteMedia2(&self, data: windows_core::Ref<super::objidlbase::IStream>, streamleadinsectors: i32) -> windows_core::Result<()>;
    fn CancelWrite(&self) -> windows_core::Result<()>;
    fn ReleaseMedia(&self) -> windows_core::Result<()>;
    fn SetWriteSpeed(&self, requestedsectorspersecond: i32, rotationtypeispurecav: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetRecorder(&self, value: windows_core::Ref<IDiscRecorder2>) -> windows_core::Result<()>;
    fn Recorder(&self) -> windows_core::Result<IDiscRecorder2>;
    fn SetBufferUnderrunFreeDisabled(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn BufferUnderrunFreeDisabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn StartOfNextSession(&self) -> windows_core::Result<i32>;
    fn LastPossibleStartOfLeadout(&self) -> windows_core::Result<i32>;
    fn CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SupportedSectorTypes(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn SetRequestedSectorType(&self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::Result<()>;
    fn RequestedSectorType(&self) -> windows_core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE>;
    fn SetClientName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ClientName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RequestedWriteSpeed(&self) -> windows_core::Result<i32>;
    fn RequestedRotationTypeIsPureCAV(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn CurrentWriteSpeed(&self) -> windows_core::Result<i32>;
    fn CurrentRotationTypeIsPureCAV(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SupportedWriteSpeeds(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn SupportedWriteSpeedDescriptors(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
}
#[cfg(all(feature = "oaidl", feature = "objidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
        unsafe extern "system" fn WriteMedia2<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void, streamleadinsectors: i32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetWriteSpeed<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2RawCD_Impl::SetWriteSpeed(this, core::mem::transmute_copy(&requestedsectorspersecond), core::mem::transmute_copy(&rotationtypeispurecav)).into()
            }
        }
        unsafe extern "system" fn SetRecorder<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2RawCD_Impl::SetRecorder(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Recorder<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::Recorder(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2RawCD_Impl::SetBufferUnderrunFreeDisabled(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::BufferUnderrunFreeDisabled(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartOfNextSession<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::StartOfNextSession(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastPossibleStartOfLeadout<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::LastPossibleStartOfLeadout(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::CurrentPhysicalMediaType(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportedSectorTypes<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::SupportedSectorTypes(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRequestedSectorType<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2RawCD_Impl::SetRequestedSectorType(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn RequestedSectorType<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::RequestedSectorType(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClientName<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2RawCD_Impl::SetClientName(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn ClientName<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::ClientName(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestedWriteSpeed<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::RequestedWriteSpeed(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::RequestedRotationTypeIsPureCAV(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentWriteSpeed<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::CurrentWriteSpeed(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::CurrentRotationTypeIsPureCAV(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedspeeds: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::SupportedWriteSpeeds(this) {
                    Ok(ok__) => {
                        supportedspeeds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Identity: IDiscFormat2RawCD_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCD_Impl::SupportedWriteSpeedDescriptors(this) {
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
            SetRecorder: SetRecorder::<Identity, OFFSET>,
            Recorder: Recorder::<Identity, OFFSET>,
            SetBufferUnderrunFreeDisabled: SetBufferUnderrunFreeDisabled::<Identity, OFFSET>,
            BufferUnderrunFreeDisabled: BufferUnderrunFreeDisabled::<Identity, OFFSET>,
            StartOfNextSession: StartOfNextSession::<Identity, OFFSET>,
            LastPossibleStartOfLeadout: LastPossibleStartOfLeadout::<Identity, OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Identity, OFFSET>,
            SupportedSectorTypes: SupportedSectorTypes::<Identity, OFFSET>,
            SetRequestedSectorType: SetRequestedSectorType::<Identity, OFFSET>,
            RequestedSectorType: RequestedSectorType::<Identity, OFFSET>,
            SetClientName: SetClientName::<Identity, OFFSET>,
            ClientName: ClientName::<Identity, OFFSET>,
            RequestedWriteSpeed: RequestedWriteSpeed::<Identity, OFFSET>,
            RequestedRotationTypeIsPureCAV: RequestedRotationTypeIsPureCAV::<Identity, OFFSET>,
            CurrentWriteSpeed: CurrentWriteSpeed::<Identity, OFFSET>,
            CurrentRotationTypeIsPureCAV: CurrentRotationTypeIsPureCAV::<Identity, OFFSET>,
            SupportedWriteSpeeds: SupportedWriteSpeeds::<Identity, OFFSET>,
            SupportedWriteSpeedDescriptors: SupportedWriteSpeedDescriptors::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2RawCD as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IDiscFormat2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "objidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
windows_core::imp::interface_hierarchy!(IDiscFormat2RawCDEventArgs, windows_core::IUnknown, super::oaidl::IDispatch, IWriteEngine2EventArgs);
#[cfg(feature = "oaidl")]
impl IDiscFormat2RawCDEventArgs {
    pub unsafe fn CurrentAction(&self) -> windows_core::Result<IMAPI_FORMAT2_RAW_CD_WRITE_ACTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ElapsedTime(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElapsedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemainingTime(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemainingTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2RawCDEventArgs_Vtbl {
    pub base__: IWriteEngine2EventArgs_Vtbl,
    pub CurrentAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_FORMAT2_RAW_CD_WRITE_ACTION) -> windows_core::HRESULT,
    pub ElapsedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RemainingTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDiscFormat2RawCDEventArgs_Impl: IWriteEngine2EventArgs_Impl {
    fn CurrentAction(&self) -> windows_core::Result<IMAPI_FORMAT2_RAW_CD_WRITE_ACTION>;
    fn ElapsedTime(&self) -> windows_core::Result<i32>;
    fn RemainingTime(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDiscFormat2RawCDEventArgs_Vtbl {
    pub const fn new<Identity: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentAction<Identity: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_WRITE_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCDEventArgs_Impl::CurrentAction(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ElapsedTime<Identity: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCDEventArgs_Impl::ElapsedTime(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemainingTime<Identity: IDiscFormat2RawCDEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2RawCDEventArgs_Impl::RemainingTime(this) {
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
            CurrentAction: CurrentAction::<Identity, OFFSET>,
            ElapsedTime: ElapsedTime::<Identity, OFFSET>,
            RemainingTime: RemainingTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2RawCDEventArgs as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IWriteEngine2EventArgs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
windows_core::imp::interface_hierarchy!(IDiscFormat2TrackAtOnce, windows_core::IUnknown, super::oaidl::IDispatch, IDiscFormat2);
#[cfg(feature = "oaidl")]
impl IDiscFormat2TrackAtOnce {
    pub unsafe fn PrepareMedia(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).PrepareMedia)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn AddAudioTrack<P0>(&self, data: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddAudioTrack)(windows_core::Interface::as_raw(self), data.param().abi()) }
    }
    pub unsafe fn CancelAddTrack(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CancelAddTrack)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ReleaseMedia(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReleaseMedia)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetWriteSpeed(&self, requestedsectorspersecond: i32, rotationtypeispurecav: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWriteSpeed)(windows_core::Interface::as_raw(self), requestedsectorspersecond, rotationtypeispurecav) }
    }
    pub unsafe fn SetRecorder<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDiscRecorder2>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRecorder)(windows_core::Interface::as_raw(self), value.param().abi()) }
    }
    pub unsafe fn Recorder(&self) -> windows_core::Result<IDiscRecorder2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Recorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetBufferUnderrunFreeDisabled(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBufferUnderrunFreeDisabled)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn BufferUnderrunFreeDisabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BufferUnderrunFreeDisabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn NumberOfExistingTracks(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NumberOfExistingTracks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TotalSectorsOnMedia(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TotalSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FreeSectorsOnMedia(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FreeSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UsedSectorsOnMedia(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UsedSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetDoNotFinalizeMedia(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDoNotFinalizeMedia)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn DoNotFinalizeMedia(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DoNotFinalizeMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ExpectedTableOfContents(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExpectedTableOfContents)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentPhysicalMediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetClientName(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetClientName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn ClientName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClientName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn RequestedWriteSpeed(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestedWriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn RequestedRotationTypeIsPureCAV(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RequestedRotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentWriteSpeed(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentWriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn CurrentRotationTypeIsPureCAV(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentRotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SupportedWriteSpeeds(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedWriteSpeeds)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SupportedWriteSpeedDescriptors(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedWriteSpeedDescriptors)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    #[cfg(feature = "wtypes")]
    pub SetWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetWriteSpeed: usize,
    pub SetRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Recorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub SetBufferUnderrunFreeDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetBufferUnderrunFreeDisabled: usize,
    #[cfg(feature = "wtypes")]
    pub BufferUnderrunFreeDisabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    BufferUnderrunFreeDisabled: usize,
    pub NumberOfExistingTracks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TotalSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub FreeSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub UsedSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub SetDoNotFinalizeMedia: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetDoNotFinalizeMedia: usize,
    #[cfg(feature = "wtypes")]
    pub DoNotFinalizeMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    DoNotFinalizeMedia: usize,
    pub ExpectedTableOfContents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub CurrentPhysicalMediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT,
    pub SetClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClientName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RequestedWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub RequestedRotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    RequestedRotationTypeIsPureCAV: usize,
    pub CurrentWriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub CurrentRotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    CurrentRotationTypeIsPureCAV: usize,
    pub SupportedWriteSpeeds: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub SupportedWriteSpeedDescriptors: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "objidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDiscFormat2TrackAtOnce_Impl: IDiscFormat2_Impl {
    fn PrepareMedia(&self) -> windows_core::Result<()>;
    fn AddAudioTrack(&self, data: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<()>;
    fn CancelAddTrack(&self) -> windows_core::Result<()>;
    fn ReleaseMedia(&self) -> windows_core::Result<()>;
    fn SetWriteSpeed(&self, requestedsectorspersecond: i32, rotationtypeispurecav: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn SetRecorder(&self, value: windows_core::Ref<IDiscRecorder2>) -> windows_core::Result<()>;
    fn Recorder(&self) -> windows_core::Result<IDiscRecorder2>;
    fn SetBufferUnderrunFreeDisabled(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn BufferUnderrunFreeDisabled(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn NumberOfExistingTracks(&self) -> windows_core::Result<i32>;
    fn TotalSectorsOnMedia(&self) -> windows_core::Result<i32>;
    fn FreeSectorsOnMedia(&self) -> windows_core::Result<i32>;
    fn UsedSectorsOnMedia(&self) -> windows_core::Result<i32>;
    fn SetDoNotFinalizeMedia(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DoNotFinalizeMedia(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn ExpectedTableOfContents(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn CurrentPhysicalMediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn SetClientName(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ClientName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn RequestedWriteSpeed(&self) -> windows_core::Result<i32>;
    fn RequestedRotationTypeIsPureCAV(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn CurrentWriteSpeed(&self) -> windows_core::Result<i32>;
    fn CurrentRotationTypeIsPureCAV(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SupportedWriteSpeeds(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn SupportedWriteSpeedDescriptors(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
}
#[cfg(all(feature = "oaidl", feature = "objidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
        unsafe extern "system" fn SetWriteSpeed<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestedsectorspersecond: i32, rotationtypeispurecav: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2TrackAtOnce_Impl::SetWriteSpeed(this, core::mem::transmute_copy(&requestedsectorspersecond), core::mem::transmute_copy(&rotationtypeispurecav)).into()
            }
        }
        unsafe extern "system" fn SetRecorder<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2TrackAtOnce_Impl::SetRecorder(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Recorder<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::Recorder(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBufferUnderrunFreeDisabled<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2TrackAtOnce_Impl::SetBufferUnderrunFreeDisabled(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn BufferUnderrunFreeDisabled<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::BufferUnderrunFreeDisabled(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NumberOfExistingTracks<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::NumberOfExistingTracks(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::TotalSectorsOnMedia(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::FreeSectorsOnMedia(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UsedSectorsOnMedia<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::UsedSectorsOnMedia(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDoNotFinalizeMedia<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2TrackAtOnce_Impl::SetDoNotFinalizeMedia(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn DoNotFinalizeMedia<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::DoNotFinalizeMedia(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExpectedTableOfContents<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::ExpectedTableOfContents(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentPhysicalMediaType<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::CurrentPhysicalMediaType(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClientName<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscFormat2TrackAtOnce_Impl::SetClientName(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn ClientName<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::ClientName(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestedWriteSpeed<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::RequestedWriteSpeed(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RequestedRotationTypeIsPureCAV<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::RequestedRotationTypeIsPureCAV(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentWriteSpeed<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::CurrentWriteSpeed(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentRotationTypeIsPureCAV<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::CurrentRotationTypeIsPureCAV(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportedWriteSpeeds<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedspeeds: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::SupportedWriteSpeeds(this) {
                    Ok(ok__) => {
                        supportedspeeds.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportedWriteSpeedDescriptors<Identity: IDiscFormat2TrackAtOnce_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedspeeddescriptors: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnce_Impl::SupportedWriteSpeedDescriptors(this) {
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
            SetRecorder: SetRecorder::<Identity, OFFSET>,
            Recorder: Recorder::<Identity, OFFSET>,
            SetBufferUnderrunFreeDisabled: SetBufferUnderrunFreeDisabled::<Identity, OFFSET>,
            BufferUnderrunFreeDisabled: BufferUnderrunFreeDisabled::<Identity, OFFSET>,
            NumberOfExistingTracks: NumberOfExistingTracks::<Identity, OFFSET>,
            TotalSectorsOnMedia: TotalSectorsOnMedia::<Identity, OFFSET>,
            FreeSectorsOnMedia: FreeSectorsOnMedia::<Identity, OFFSET>,
            UsedSectorsOnMedia: UsedSectorsOnMedia::<Identity, OFFSET>,
            SetDoNotFinalizeMedia: SetDoNotFinalizeMedia::<Identity, OFFSET>,
            DoNotFinalizeMedia: DoNotFinalizeMedia::<Identity, OFFSET>,
            ExpectedTableOfContents: ExpectedTableOfContents::<Identity, OFFSET>,
            CurrentPhysicalMediaType: CurrentPhysicalMediaType::<Identity, OFFSET>,
            SetClientName: SetClientName::<Identity, OFFSET>,
            ClientName: ClientName::<Identity, OFFSET>,
            RequestedWriteSpeed: RequestedWriteSpeed::<Identity, OFFSET>,
            RequestedRotationTypeIsPureCAV: RequestedRotationTypeIsPureCAV::<Identity, OFFSET>,
            CurrentWriteSpeed: CurrentWriteSpeed::<Identity, OFFSET>,
            CurrentRotationTypeIsPureCAV: CurrentRotationTypeIsPureCAV::<Identity, OFFSET>,
            SupportedWriteSpeeds: SupportedWriteSpeeds::<Identity, OFFSET>,
            SupportedWriteSpeedDescriptors: SupportedWriteSpeedDescriptors::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2TrackAtOnce as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IDiscFormat2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "objidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
windows_core::imp::interface_hierarchy!(IDiscFormat2TrackAtOnceEventArgs, windows_core::IUnknown, super::oaidl::IDispatch, IWriteEngine2EventArgs);
#[cfg(feature = "oaidl")]
impl IDiscFormat2TrackAtOnceEventArgs {
    pub unsafe fn CurrentTrackNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentTrackNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentAction(&self) -> windows_core::Result<IMAPI_FORMAT2_TAO_WRITE_ACTION> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ElapsedTime(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElapsedTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RemainingTime(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RemainingTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscFormat2TrackAtOnceEventArgs_Vtbl {
    pub base__: IWriteEngine2EventArgs_Vtbl,
    pub CurrentTrackNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_FORMAT2_TAO_WRITE_ACTION) -> windows_core::HRESULT,
    pub ElapsedTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RemainingTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDiscFormat2TrackAtOnceEventArgs_Impl: IWriteEngine2EventArgs_Impl {
    fn CurrentTrackNumber(&self) -> windows_core::Result<i32>;
    fn CurrentAction(&self) -> windows_core::Result<IMAPI_FORMAT2_TAO_WRITE_ACTION>;
    fn ElapsedTime(&self) -> windows_core::Result<i32>;
    fn RemainingTime(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDiscFormat2TrackAtOnceEventArgs_Vtbl {
    pub const fn new<Identity: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentTrackNumber<Identity: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnceEventArgs_Impl::CurrentTrackNumber(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentAction<Identity: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_FORMAT2_TAO_WRITE_ACTION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnceEventArgs_Impl::CurrentAction(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ElapsedTime<Identity: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnceEventArgs_Impl::ElapsedTime(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RemainingTime<Identity: IDiscFormat2TrackAtOnceEventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscFormat2TrackAtOnceEventArgs_Impl::RemainingTime(this) {
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
            CurrentTrackNumber: CurrentTrackNumber::<Identity, OFFSET>,
            CurrentAction: CurrentAction::<Identity, OFFSET>,
            ElapsedTime: ElapsedTime::<Identity, OFFSET>,
            RemainingTime: RemainingTime::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscFormat2TrackAtOnceEventArgs as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IWriteEngine2EventArgs as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDiscFormat2TrackAtOnceEventArgs {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDiscMaster2, IDiscMaster2_Vtbl, 0x27354130_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDiscMaster2 {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDiscMaster2, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IDiscMaster2 {
    pub unsafe fn _NewEnum(&self) -> windows_core::Result<super::oaidl::IEnumVARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self)._NewEnum)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Item(&self, index: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Item)(windows_core::Interface::as_raw(self), index, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Count(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsSupportedEnvironment(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSupportedEnvironment)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscMaster2_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub _NewEnum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Item: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub IsSupportedEnvironment: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsSupportedEnvironment: usize,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDiscMaster2_Impl: super::oaidl::IDispatch_Impl {
    fn _NewEnum(&self) -> windows_core::Result<super::oaidl::IEnumVARIANT>;
    fn Item(&self, index: i32) -> windows_core::Result<windows_core::BSTR>;
    fn Count(&self) -> windows_core::Result<i32>;
    fn IsSupportedEnvironment(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IDiscMaster2_Vtbl {
    pub const fn new<Identity: IDiscMaster2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn _NewEnum<Identity: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunk: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscMaster2_Impl::_NewEnum(this) {
                    Ok(ok__) => {
                        ppunk.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Item<Identity: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscMaster2_Impl::Item(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Count<Identity: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscMaster2_Impl::Count(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsSupportedEnvironment<Identity: IDiscMaster2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscMaster2_Impl::IsSupportedEnvironment(this) {
                    Ok(ok__) => {
                        value.write(ok__);
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
            IsSupportedEnvironment: IsSupportedEnvironment::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscMaster2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDiscMaster2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IDiscRecorder2, IDiscRecorder2_Vtbl, 0x27354133_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IDiscRecorder2 {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IDiscRecorder2, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IDiscRecorder2 {
    pub unsafe fn EjectMedia(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).EjectMedia)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn CloseTray(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CloseTray)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn AcquireExclusiveAccess(&self, force: super::wtypes::VARIANT_BOOL, param1: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AcquireExclusiveAccess)(windows_core::Interface::as_raw(self), force, core::mem::transmute_copy(param1)) }
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
    pub unsafe fn ActiveDiscRecorder(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ActiveDiscRecorder)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn VendorId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VendorId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ProductId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProductId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ProductRevision(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProductRevision)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn VolumeName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VolumeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn VolumePathNames(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VolumePathNames)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn DeviceCanLoadMedia(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DeviceCanLoadMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LegacyDeviceNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LegacyDeviceNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SupportedFeaturePages(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedFeaturePages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentFeaturePages(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentFeaturePages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SupportedProfiles(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedProfiles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentProfiles(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentProfiles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SupportedModePages(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedModePages)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ExclusiveAccessOwner(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExclusiveAccessOwner)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IDiscRecorder2_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub EjectMedia: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CloseTray: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub AcquireExclusiveAccess: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    AcquireExclusiveAccess: usize,
    pub ReleaseExclusiveAccess: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DisableMcn: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub EnableMcn: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub InitializeDiscRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ActiveDiscRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VendorId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProductId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProductRevision: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VolumeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub VolumePathNames: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub DeviceCanLoadMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    DeviceCanLoadMedia: usize,
    pub LegacyDeviceNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SupportedFeaturePages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub CurrentFeaturePages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub SupportedProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub CurrentProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub SupportedModePages: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub ExclusiveAccessOwner: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IDiscRecorder2_Impl: super::oaidl::IDispatch_Impl {
    fn EjectMedia(&self) -> windows_core::Result<()>;
    fn CloseTray(&self) -> windows_core::Result<()>;
    fn AcquireExclusiveAccess(&self, force: super::wtypes::VARIANT_BOOL, param1: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ReleaseExclusiveAccess(&self) -> windows_core::Result<()>;
    fn DisableMcn(&self) -> windows_core::Result<()>;
    fn EnableMcn(&self) -> windows_core::Result<()>;
    fn InitializeDiscRecorder(&self, recorderuniqueid: &windows_core::BSTR) -> windows_core::Result<()>;
    fn ActiveDiscRecorder(&self) -> windows_core::Result<windows_core::BSTR>;
    fn VendorId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ProductId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ProductRevision(&self) -> windows_core::Result<windows_core::BSTR>;
    fn VolumeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn VolumePathNames(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn DeviceCanLoadMedia(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn LegacyDeviceNumber(&self) -> windows_core::Result<i32>;
    fn SupportedFeaturePages(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn CurrentFeaturePages(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn SupportedProfiles(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn CurrentProfiles(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn SupportedModePages(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn ExclusiveAccessOwner(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
        unsafe extern "system" fn AcquireExclusiveAccess<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, force: super::wtypes::VARIANT_BOOL, param1: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2_Impl::AcquireExclusiveAccess(this, core::mem::transmute_copy(&force), core::mem::transmute(&param1)).into()
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
        unsafe extern "system" fn ActiveDiscRecorder<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::ActiveDiscRecorder(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VendorId<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::VendorId(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProductId<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::ProductId(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProductRevision<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::ProductRevision(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VolumeName<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::VolumeName(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VolumePathNames<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::VolumePathNames(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DeviceCanLoadMedia<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::DeviceCanLoadMedia(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LegacyDeviceNumber<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, legacydevicenumber: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::LegacyDeviceNumber(this) {
                    Ok(ok__) => {
                        legacydevicenumber.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportedFeaturePages<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::SupportedFeaturePages(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentFeaturePages<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::CurrentFeaturePages(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportedProfiles<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::SupportedProfiles(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentProfiles<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::CurrentProfiles(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportedModePages<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::SupportedModePages(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExclusiveAccessOwner<Identity: IDiscRecorder2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDiscRecorder2_Impl::ExclusiveAccessOwner(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            EjectMedia: EjectMedia::<Identity, OFFSET>,
            CloseTray: CloseTray::<Identity, OFFSET>,
            AcquireExclusiveAccess: AcquireExclusiveAccess::<Identity, OFFSET>,
            ReleaseExclusiveAccess: ReleaseExclusiveAccess::<Identity, OFFSET>,
            DisableMcn: DisableMcn::<Identity, OFFSET>,
            EnableMcn: EnableMcn::<Identity, OFFSET>,
            InitializeDiscRecorder: InitializeDiscRecorder::<Identity, OFFSET>,
            ActiveDiscRecorder: ActiveDiscRecorder::<Identity, OFFSET>,
            VendorId: VendorId::<Identity, OFFSET>,
            ProductId: ProductId::<Identity, OFFSET>,
            ProductRevision: ProductRevision::<Identity, OFFSET>,
            VolumeName: VolumeName::<Identity, OFFSET>,
            VolumePathNames: VolumePathNames::<Identity, OFFSET>,
            DeviceCanLoadMedia: DeviceCanLoadMedia::<Identity, OFFSET>,
            LegacyDeviceNumber: LegacyDeviceNumber::<Identity, OFFSET>,
            SupportedFeaturePages: SupportedFeaturePages::<Identity, OFFSET>,
            CurrentFeaturePages: CurrentFeaturePages::<Identity, OFFSET>,
            SupportedProfiles: SupportedProfiles::<Identity, OFFSET>,
            CurrentProfiles: CurrentProfiles::<Identity, OFFSET>,
            SupportedModePages: SupportedModePages::<Identity, OFFSET>,
            ExclusiveAccessOwner: ExclusiveAccessOwner::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDiscRecorder2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IDiscRecorder2 {}
windows_core::imp::define_interface!(IDiscRecorder2Ex, IDiscRecorder2Ex_Vtbl, 0x27354132_7f64_5b0f_8f00_5d77afbe261e);
windows_core::imp::interface_hierarchy!(IDiscRecorder2Ex, windows_core::IUnknown);
impl IDiscRecorder2Ex {
    pub unsafe fn SendCommandNoData(&self, cdb: *const u8, cdbsize: u32, sensebuffer: &mut [u8; 18], timeout: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SendCommandNoData)(windows_core::Interface::as_raw(self), cdb, cdbsize, sensebuffer.as_mut_ptr(), timeout) }
    }
    pub unsafe fn SendCommandSendDataToDevice(&self, cdb: *const u8, cdbsize: u32, sensebuffer: &mut [u8; 18], timeout: u32, buffer: *const u8, buffersize: ULONG_IMAPI2_NONZERO) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SendCommandSendDataToDevice)(windows_core::Interface::as_raw(self), cdb, cdbsize, sensebuffer.as_mut_ptr(), timeout, buffer, buffersize) }
    }
    pub unsafe fn SendCommandGetDataFromDevice(&self, cdb: *const u8, cdbsize: u32, sensebuffer: &mut [u8; 18], timeout: u32, buffer: *mut u8, buffersize: ULONG_IMAPI2_NONZERO, bufferfetched: *mut ULONG_IMAPI2_NOT_NEGATIVE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SendCommandGetDataFromDevice)(windows_core::Interface::as_raw(self), cdb, cdbsize, sensebuffer.as_mut_ptr(), timeout, buffer as _, buffersize, bufferfetched as _) }
    }
    pub unsafe fn ReadDvdStructure(&self, format: u32, address: u32, layer: u32, agid: u32, data: *mut *mut u8, count: *mut ULONG_IMAPI2_DVD_STRUCTURE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReadDvdStructure)(windows_core::Interface::as_raw(self), format, address, layer, agid, data as _, count as _) }
    }
    pub unsafe fn SendDvdStructure(&self, format: u32, data: *const u8, count: ULONG_IMAPI2_DVD_STRUCTURE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SendDvdStructure)(windows_core::Interface::as_raw(self), format, data, count) }
    }
    pub unsafe fn GetAdapterDescriptor(&self, data: *mut *mut u8, bytesize: *mut ULONG_IMAPI2_ADAPTER_DESCRIPTOR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetAdapterDescriptor)(windows_core::Interface::as_raw(self), data as _, bytesize as _) }
    }
    pub unsafe fn GetDeviceDescriptor(&self, data: *mut *mut u8, bytesize: *mut ULONG_IMAPI2_DEVICE_DESCRIPTOR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDeviceDescriptor)(windows_core::Interface::as_raw(self), data as _, bytesize as _) }
    }
    pub unsafe fn GetDiscInformation(&self, discinformation: *mut *mut u8, bytesize: *mut ULONG_IMAPI2_DISC_INFORMATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDiscInformation)(windows_core::Interface::as_raw(self), discinformation as _, bytesize as _) }
    }
    pub unsafe fn GetTrackInformation(&self, address: u32, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut u8, bytesize: *mut ULONG_IMAPI2_TRACK_INFORMATION) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetTrackInformation)(windows_core::Interface::as_raw(self), address, addresstype, trackinformation as _, bytesize as _) }
    }
    pub unsafe fn GetFeaturePage(&self, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: bool, featuredata: *mut *mut u8, bytesize: *mut ULONG_IMAPI2_FEATURE_PAGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFeaturePage)(windows_core::Interface::as_raw(self), requestedfeature, currentfeatureonly, featuredata as _, bytesize as _) }
    }
    pub unsafe fn GetModePage(&self, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut u8, bytesize: *mut ULONG_IMAPI2_MODE_PAGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetModePage)(windows_core::Interface::as_raw(self), requestedmodepage, requesttype, modepagedata as _, bytesize as _) }
    }
    pub unsafe fn SetModePage(&self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: *const u8, bytesize: ULONG_IMAPI2_MODE_PAGE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetModePage)(windows_core::Interface::as_raw(self), requesttype, data, bytesize) }
    }
    pub unsafe fn GetSupportedFeaturePages(&self, currentfeatureonly: bool, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut ULONG_IMAPI2_ALL_FEATURE_PAGES) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSupportedFeaturePages)(windows_core::Interface::as_raw(self), currentfeatureonly, featuredata as _, bytesize as _) }
    }
    pub unsafe fn GetSupportedProfiles(&self, currentonly: bool, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut ULONG_IMAPI2_ALL_PROFILES) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSupportedProfiles)(windows_core::Interface::as_raw(self), currentonly, profiletypes as _, validprofiles as _) }
    }
    pub unsafe fn GetSupportedModePages(&self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut ULONG_IMAPI2_ALL_MODE_PAGES) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetSupportedModePages)(windows_core::Interface::as_raw(self), requesttype, modepagetypes as _, validpages as _) }
    }
    pub unsafe fn GetByteAlignmentMask(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetByteAlignmentMask)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMaximumNonPageAlignedTransferSize(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMaximumNonPageAlignedTransferSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetMaximumPageAlignedTransferSize(&self) -> windows_core::Result<u32> {
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
    pub SendCommandNoData: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut u8, u32) -> windows_core::HRESULT,
    pub SendCommandSendDataToDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut u8, u32, *const u8, ULONG_IMAPI2_NONZERO) -> windows_core::HRESULT,
    pub SendCommandGetDataFromDevice: unsafe extern "system" fn(*mut core::ffi::c_void, *const u8, u32, *mut u8, u32, *mut u8, ULONG_IMAPI2_NONZERO, *mut ULONG_IMAPI2_NOT_NEGATIVE) -> windows_core::HRESULT,
    pub ReadDvdStructure: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, u32, u32, *mut *mut u8, *mut ULONG_IMAPI2_DVD_STRUCTURE) -> windows_core::HRESULT,
    pub SendDvdStructure: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const u8, ULONG_IMAPI2_DVD_STRUCTURE) -> windows_core::HRESULT,
    pub GetAdapterDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut ULONG_IMAPI2_ADAPTER_DESCRIPTOR) -> windows_core::HRESULT,
    pub GetDeviceDescriptor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut ULONG_IMAPI2_DEVICE_DESCRIPTOR) -> windows_core::HRESULT,
    pub GetDiscInformation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u8, *mut ULONG_IMAPI2_DISC_INFORMATION) -> windows_core::HRESULT,
    pub GetTrackInformation: unsafe extern "system" fn(*mut core::ffi::c_void, u32, IMAPI_READ_TRACK_ADDRESS_TYPE, *mut *mut u8, *mut ULONG_IMAPI2_TRACK_INFORMATION) -> windows_core::HRESULT,
    pub GetFeaturePage: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_FEATURE_PAGE_TYPE, bool, *mut *mut u8, *mut ULONG_IMAPI2_FEATURE_PAGE) -> windows_core::HRESULT,
    pub GetModePage: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_MODE_PAGE_TYPE, IMAPI_MODE_PAGE_REQUEST_TYPE, *mut *mut u8, *mut ULONG_IMAPI2_MODE_PAGE) -> windows_core::HRESULT,
    pub SetModePage: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_MODE_PAGE_REQUEST_TYPE, *const u8, ULONG_IMAPI2_MODE_PAGE) -> windows_core::HRESULT,
    pub GetSupportedFeaturePages: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut IMAPI_FEATURE_PAGE_TYPE, *mut ULONG_IMAPI2_ALL_FEATURE_PAGES) -> windows_core::HRESULT,
    pub GetSupportedProfiles: unsafe extern "system" fn(*mut core::ffi::c_void, bool, *mut *mut IMAPI_PROFILE_TYPE, *mut ULONG_IMAPI2_ALL_PROFILES) -> windows_core::HRESULT,
    pub GetSupportedModePages: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_MODE_PAGE_REQUEST_TYPE, *mut *mut IMAPI_MODE_PAGE_TYPE, *mut ULONG_IMAPI2_ALL_MODE_PAGES) -> windows_core::HRESULT,
    pub GetByteAlignmentMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMaximumNonPageAlignedTransferSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetMaximumPageAlignedTransferSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IDiscRecorder2Ex_Impl: windows_core::IUnknownImpl {
    fn SendCommandNoData(&self, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32) -> windows_core::Result<()>;
    fn SendCommandSendDataToDevice(&self, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *const u8, buffersize: ULONG_IMAPI2_NONZERO) -> windows_core::Result<()>;
    fn SendCommandGetDataFromDevice(&self, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *mut u8, buffersize: ULONG_IMAPI2_NONZERO, bufferfetched: *mut ULONG_IMAPI2_NOT_NEGATIVE) -> windows_core::Result<()>;
    fn ReadDvdStructure(&self, format: u32, address: u32, layer: u32, agid: u32, data: *mut *mut u8, count: *mut ULONG_IMAPI2_DVD_STRUCTURE) -> windows_core::Result<()>;
    fn SendDvdStructure(&self, format: u32, data: *const u8, count: ULONG_IMAPI2_DVD_STRUCTURE) -> windows_core::Result<()>;
    fn GetAdapterDescriptor(&self, data: *mut *mut u8, bytesize: *mut ULONG_IMAPI2_ADAPTER_DESCRIPTOR) -> windows_core::Result<()>;
    fn GetDeviceDescriptor(&self, data: *mut *mut u8, bytesize: *mut ULONG_IMAPI2_DEVICE_DESCRIPTOR) -> windows_core::Result<()>;
    fn GetDiscInformation(&self, discinformation: *mut *mut u8, bytesize: *mut ULONG_IMAPI2_DISC_INFORMATION) -> windows_core::Result<()>;
    fn GetTrackInformation(&self, address: u32, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut u8, bytesize: *mut ULONG_IMAPI2_TRACK_INFORMATION) -> windows_core::Result<()>;
    fn GetFeaturePage(&self, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: bool, featuredata: *mut *mut u8, bytesize: *mut ULONG_IMAPI2_FEATURE_PAGE) -> windows_core::Result<()>;
    fn GetModePage(&self, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut u8, bytesize: *mut ULONG_IMAPI2_MODE_PAGE) -> windows_core::Result<()>;
    fn SetModePage(&self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: *const u8, bytesize: ULONG_IMAPI2_MODE_PAGE) -> windows_core::Result<()>;
    fn GetSupportedFeaturePages(&self, currentfeatureonly: bool, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut ULONG_IMAPI2_ALL_FEATURE_PAGES) -> windows_core::Result<()>;
    fn GetSupportedProfiles(&self, currentonly: bool, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut ULONG_IMAPI2_ALL_PROFILES) -> windows_core::Result<()>;
    fn GetSupportedModePages(&self, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagetypes: *mut *mut IMAPI_MODE_PAGE_TYPE, validpages: *mut ULONG_IMAPI2_ALL_MODE_PAGES) -> windows_core::Result<()>;
    fn GetByteAlignmentMask(&self) -> windows_core::Result<u32>;
    fn GetMaximumNonPageAlignedTransferSize(&self) -> windows_core::Result<u32>;
    fn GetMaximumPageAlignedTransferSize(&self) -> windows_core::Result<u32>;
}
impl IDiscRecorder2Ex_Vtbl {
    pub const fn new<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SendCommandNoData<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::SendCommandNoData(this, core::mem::transmute_copy(&cdb), core::mem::transmute_copy(&cdbsize), core::mem::transmute_copy(&sensebuffer), core::mem::transmute_copy(&timeout)).into()
            }
        }
        unsafe extern "system" fn SendCommandSendDataToDevice<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *const u8, buffersize: ULONG_IMAPI2_NONZERO) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::SendCommandSendDataToDevice(this, core::mem::transmute_copy(&cdb), core::mem::transmute_copy(&cdbsize), core::mem::transmute_copy(&sensebuffer), core::mem::transmute_copy(&timeout), core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&buffersize)).into()
            }
        }
        unsafe extern "system" fn SendCommandGetDataFromDevice<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cdb: *const u8, cdbsize: u32, sensebuffer: *mut u8, timeout: u32, buffer: *mut u8, buffersize: ULONG_IMAPI2_NONZERO, bufferfetched: *mut ULONG_IMAPI2_NOT_NEGATIVE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::SendCommandGetDataFromDevice(this, core::mem::transmute_copy(&cdb), core::mem::transmute_copy(&cdbsize), core::mem::transmute_copy(&sensebuffer), core::mem::transmute_copy(&timeout), core::mem::transmute_copy(&buffer), core::mem::transmute_copy(&buffersize), core::mem::transmute_copy(&bufferfetched)).into()
            }
        }
        unsafe extern "system" fn ReadDvdStructure<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: u32, address: u32, layer: u32, agid: u32, data: *mut *mut u8, count: *mut ULONG_IMAPI2_DVD_STRUCTURE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::ReadDvdStructure(this, core::mem::transmute_copy(&format), core::mem::transmute_copy(&address), core::mem::transmute_copy(&layer), core::mem::transmute_copy(&agid), core::mem::transmute_copy(&data), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn SendDvdStructure<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, format: u32, data: *const u8, count: ULONG_IMAPI2_DVD_STRUCTURE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::SendDvdStructure(this, core::mem::transmute_copy(&format), core::mem::transmute_copy(&data), core::mem::transmute_copy(&count)).into()
            }
        }
        unsafe extern "system" fn GetAdapterDescriptor<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut *mut u8, bytesize: *mut ULONG_IMAPI2_ADAPTER_DESCRIPTOR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::GetAdapterDescriptor(this, core::mem::transmute_copy(&data), core::mem::transmute_copy(&bytesize)).into()
            }
        }
        unsafe extern "system" fn GetDeviceDescriptor<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut *mut u8, bytesize: *mut ULONG_IMAPI2_DEVICE_DESCRIPTOR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::GetDeviceDescriptor(this, core::mem::transmute_copy(&data), core::mem::transmute_copy(&bytesize)).into()
            }
        }
        unsafe extern "system" fn GetDiscInformation<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, discinformation: *mut *mut u8, bytesize: *mut ULONG_IMAPI2_DISC_INFORMATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::GetDiscInformation(this, core::mem::transmute_copy(&discinformation), core::mem::transmute_copy(&bytesize)).into()
            }
        }
        unsafe extern "system" fn GetTrackInformation<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, address: u32, addresstype: IMAPI_READ_TRACK_ADDRESS_TYPE, trackinformation: *mut *mut u8, bytesize: *mut ULONG_IMAPI2_TRACK_INFORMATION) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::GetTrackInformation(this, core::mem::transmute_copy(&address), core::mem::transmute_copy(&addresstype), core::mem::transmute_copy(&trackinformation), core::mem::transmute_copy(&bytesize)).into()
            }
        }
        unsafe extern "system" fn GetFeaturePage<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestedfeature: IMAPI_FEATURE_PAGE_TYPE, currentfeatureonly: bool, featuredata: *mut *mut u8, bytesize: *mut ULONG_IMAPI2_FEATURE_PAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::GetFeaturePage(this, core::mem::transmute_copy(&requestedfeature), core::mem::transmute_copy(&currentfeatureonly), core::mem::transmute_copy(&featuredata), core::mem::transmute_copy(&bytesize)).into()
            }
        }
        unsafe extern "system" fn GetModePage<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requestedmodepage: IMAPI_MODE_PAGE_TYPE, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, modepagedata: *mut *mut u8, bytesize: *mut ULONG_IMAPI2_MODE_PAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::GetModePage(this, core::mem::transmute_copy(&requestedmodepage), core::mem::transmute_copy(&requesttype), core::mem::transmute_copy(&modepagedata), core::mem::transmute_copy(&bytesize)).into()
            }
        }
        unsafe extern "system" fn SetModePage<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, requesttype: IMAPI_MODE_PAGE_REQUEST_TYPE, data: *const u8, bytesize: ULONG_IMAPI2_MODE_PAGE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::SetModePage(this, core::mem::transmute_copy(&requesttype), core::mem::transmute_copy(&data), core::mem::transmute_copy(&bytesize)).into()
            }
        }
        unsafe extern "system" fn GetSupportedFeaturePages<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentfeatureonly: bool, featuredata: *mut *mut IMAPI_FEATURE_PAGE_TYPE, bytesize: *mut ULONG_IMAPI2_ALL_FEATURE_PAGES) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDiscRecorder2Ex_Impl::GetSupportedFeaturePages(this, core::mem::transmute_copy(&currentfeatureonly), core::mem::transmute_copy(&featuredata), core::mem::transmute_copy(&bytesize)).into()
            }
        }
        unsafe extern "system" fn GetSupportedProfiles<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, currentonly: bool, profiletypes: *mut *mut IMAPI_PROFILE_TYPE, validprofiles: *mut ULONG_IMAPI2_ALL_PROFILES) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetByteAlignmentMask<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetMaximumNonPageAlignedTransferSize<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetMaximumPageAlignedTransferSize<Identity: IDiscRecorder2Ex_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u32) -> windows_core::HRESULT {
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
impl windows_core::RuntimeName for IDiscRecorder2Ex {}
pub const IMAPI2_DEFAULT_COMMAND_TIMEOUT: u32 = 10;
pub const IMAPILib2_MajorVersion: u32 = 1;
pub const IMAPILib2_MinorVersion: u32 = 0;
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
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_BD: u32 = 2195;
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_CD: u32 = 75;
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_DVD: u32 = 680;
pub const IMAPI_SECTORS_PER_SECOND_AT_1X_HD_DVD: u32 = 4568;
pub const IMAPI_SECTOR_SIZE: u32 = 2048;
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
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IMultisession, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IMultisession {
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsSupportedOnCurrentMediaState(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSupportedOnCurrentMediaState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetInUse(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetInUse)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn InUse(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).InUse)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ImportRecorder(&self) -> windows_core::Result<IDiscRecorder2> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImportRecorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IMultisession_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(feature = "wtypes")]
    pub IsSupportedOnCurrentMediaState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsSupportedOnCurrentMediaState: usize,
    #[cfg(feature = "wtypes")]
    pub SetInUse: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetInUse: usize,
    #[cfg(feature = "wtypes")]
    pub InUse: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    InUse: usize,
    pub ImportRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMultisession_Impl: super::oaidl::IDispatch_Impl {
    fn IsSupportedOnCurrentMediaState(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetInUse(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn InUse(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn ImportRecorder(&self) -> windows_core::Result<IDiscRecorder2>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMultisession_Vtbl {
    pub const fn new<Identity: IMultisession_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsSupportedOnCurrentMediaState<Identity: IMultisession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisession_Impl::IsSupportedOnCurrentMediaState(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetInUse<Identity: IMultisession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultisession_Impl::SetInUse(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn InUse<Identity: IMultisession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisession_Impl::InUse(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ImportRecorder<Identity: IMultisession_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisession_Impl::ImportRecorder(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            IsSupportedOnCurrentMediaState: IsSupportedOnCurrentMediaState::<Identity, OFFSET>,
            SetInUse: SetInUse::<Identity, OFFSET>,
            InUse: InUse::<Identity, OFFSET>,
            ImportRecorder: ImportRecorder::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultisession as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
windows_core::imp::interface_hierarchy!(IMultisessionRandomWrite, windows_core::IUnknown, super::oaidl::IDispatch, IMultisession);
#[cfg(feature = "oaidl")]
impl IMultisessionRandomWrite {
    pub unsafe fn WriteUnitSize(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WriteUnitSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LastWrittenAddress(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastWrittenAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TotalSectorsOnMedia(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TotalSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IMultisessionRandomWrite_Vtbl {
    pub base__: IMultisession_Vtbl,
    pub WriteUnitSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastWrittenAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TotalSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMultisessionRandomWrite_Impl: IMultisession_Impl {
    fn WriteUnitSize(&self) -> windows_core::Result<i32>;
    fn LastWrittenAddress(&self) -> windows_core::Result<i32>;
    fn TotalSectorsOnMedia(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMultisessionRandomWrite_Vtbl {
    pub const fn new<Identity: IMultisessionRandomWrite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WriteUnitSize<Identity: IMultisessionRandomWrite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionRandomWrite_Impl::WriteUnitSize(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastWrittenAddress<Identity: IMultisessionRandomWrite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionRandomWrite_Impl::LastWrittenAddress(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TotalSectorsOnMedia<Identity: IMultisessionRandomWrite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionRandomWrite_Impl::TotalSectorsOnMedia(this) {
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
            WriteUnitSize: WriteUnitSize::<Identity, OFFSET>,
            LastWrittenAddress: LastWrittenAddress::<Identity, OFFSET>,
            TotalSectorsOnMedia: TotalSectorsOnMedia::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultisessionRandomWrite as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IMultisession as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
windows_core::imp::interface_hierarchy!(IMultisessionSequential, windows_core::IUnknown, super::oaidl::IDispatch, IMultisession);
#[cfg(feature = "oaidl")]
impl IMultisessionSequential {
    #[cfg(feature = "wtypes")]
    pub unsafe fn IsFirstDataSession(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsFirstDataSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StartAddressOfPreviousSession(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartAddressOfPreviousSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LastWrittenAddressOfPreviousSession(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastWrittenAddressOfPreviousSession)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn NextWritableAddress(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NextWritableAddress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FreeSectorsOnMedia(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FreeSectorsOnMedia)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IMultisessionSequential_Vtbl {
    pub base__: IMultisession_Vtbl,
    #[cfg(feature = "wtypes")]
    pub IsFirstDataSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    IsFirstDataSession: usize,
    pub StartAddressOfPreviousSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastWrittenAddressOfPreviousSession: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub NextWritableAddress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub FreeSectorsOnMedia: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMultisessionSequential_Impl: IMultisession_Impl {
    fn IsFirstDataSession(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn StartAddressOfPreviousSession(&self) -> windows_core::Result<i32>;
    fn LastWrittenAddressOfPreviousSession(&self) -> windows_core::Result<i32>;
    fn NextWritableAddress(&self) -> windows_core::Result<i32>;
    fn FreeSectorsOnMedia(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMultisessionSequential_Vtbl {
    pub const fn new<Identity: IMultisessionSequential_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsFirstDataSession<Identity: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionSequential_Impl::IsFirstDataSession(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartAddressOfPreviousSession<Identity: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionSequential_Impl::StartAddressOfPreviousSession(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastWrittenAddressOfPreviousSession<Identity: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionSequential_Impl::LastWrittenAddressOfPreviousSession(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NextWritableAddress<Identity: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionSequential_Impl::NextWritableAddress(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FreeSectorsOnMedia<Identity: IMultisessionSequential_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionSequential_Impl::FreeSectorsOnMedia(this) {
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
            IsFirstDataSession: IsFirstDataSession::<Identity, OFFSET>,
            StartAddressOfPreviousSession: StartAddressOfPreviousSession::<Identity, OFFSET>,
            LastWrittenAddressOfPreviousSession: LastWrittenAddressOfPreviousSession::<Identity, OFFSET>,
            NextWritableAddress: NextWritableAddress::<Identity, OFFSET>,
            FreeSectorsOnMedia: FreeSectorsOnMedia::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultisessionSequential as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IMultisession as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
windows_core::imp::interface_hierarchy!(IMultisessionSequential2, windows_core::IUnknown, super::oaidl::IDispatch, IMultisession, IMultisessionSequential);
#[cfg(feature = "oaidl")]
impl IMultisessionSequential2 {
    pub unsafe fn WriteUnitSize(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WriteUnitSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IMultisessionSequential2_Vtbl {
    pub base__: IMultisessionSequential_Vtbl,
    pub WriteUnitSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IMultisessionSequential2_Impl: IMultisessionSequential_Impl {
    fn WriteUnitSize(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IMultisessionSequential2_Vtbl {
    pub const fn new<Identity: IMultisessionSequential2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WriteUnitSize<Identity: IMultisessionSequential2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultisessionSequential2_Impl::WriteUnitSize(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IMultisessionSequential_Vtbl::new::<Identity, OFFSET>(), WriteUnitSize: WriteUnitSize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultisessionSequential2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID || iid == &<IMultisession as windows_core::Interface>::IID || iid == &<IMultisessionSequential as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IMultisessionSequential2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IRawCDImageCreator, IRawCDImageCreator_Vtbl, 0x25983550_9d65_49ce_b335_40630d901227);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IRawCDImageCreator {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IRawCDImageCreator, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IRawCDImageCreator {
    #[cfg(feature = "objidlbase")]
    pub unsafe fn CreateResultImage(&self) -> windows_core::Result<super::objidlbase::IStream> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateResultImage)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn AddTrack<P1>(&self, datatype: IMAPI_CD_SECTOR_TYPE, data: P1) -> windows_core::Result<i32>
    where
        P1: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AddTrack)(windows_core::Interface::as_raw(self), datatype, data.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn AddSpecialPregap<P0>(&self, data: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddSpecialPregap)(windows_core::Interface::as_raw(self), data.param().abi()) }
    }
    #[cfg(feature = "objidlbase")]
    pub unsafe fn AddSubcodeRWGenerator<P0>(&self, subcode: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddSubcodeRWGenerator)(windows_core::Interface::as_raw(self), subcode.param().abi()) }
    }
    pub unsafe fn SetResultingImageType(&self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetResultingImageType)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn ResultingImageType(&self) -> windows_core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ResultingImageType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StartOfLeadout(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartOfLeadout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStartOfLeadoutLimit(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStartOfLeadoutLimit)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn StartOfLeadoutLimit(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartOfLeadoutLimit)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetDisableGaplessAudio(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDisableGaplessAudio)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn DisableGaplessAudio(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DisableGaplessAudio)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetMediaCatalogNumber(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetMediaCatalogNumber)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn MediaCatalogNumber(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MediaCatalogNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetStartingTrackNumber(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStartingTrackNumber)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn StartingTrackNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartingTrackNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TrackInfo(&self, trackindex: i32) -> windows_core::Result<IRawCDImageTrackInfo> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TrackInfo)(windows_core::Interface::as_raw(self), trackindex, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn NumberOfExistingTracks(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NumberOfExistingTracks)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LastUsedUserSectorInImage(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastUsedUserSectorInImage)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ExpectedTableOfContents(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExpectedTableOfContents)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IRawCDImageCreator_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(feature = "objidlbase")]
    pub CreateResultImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    CreateResultImage: usize,
    #[cfg(feature = "objidlbase")]
    pub AddTrack: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_CD_SECTOR_TYPE, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    AddTrack: usize,
    #[cfg(feature = "objidlbase")]
    pub AddSpecialPregap: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    AddSpecialPregap: usize,
    #[cfg(feature = "objidlbase")]
    pub AddSubcodeRWGenerator: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    AddSubcodeRWGenerator: usize,
    pub SetResultingImageType: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT,
    pub ResultingImageType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT,
    pub StartOfLeadout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetStartOfLeadoutLimit: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub StartOfLeadoutLimit: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub SetDisableGaplessAudio: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetDisableGaplessAudio: usize,
    #[cfg(feature = "wtypes")]
    pub DisableGaplessAudio: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    DisableGaplessAudio: usize,
    pub SetMediaCatalogNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MediaCatalogNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetStartingTrackNumber: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub StartingTrackNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TrackInfo: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NumberOfExistingTracks: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastUsedUserSectorInImage: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ExpectedTableOfContents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "objidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IRawCDImageCreator_Impl: super::oaidl::IDispatch_Impl {
    fn CreateResultImage(&self) -> windows_core::Result<super::objidlbase::IStream>;
    fn AddTrack(&self, datatype: IMAPI_CD_SECTOR_TYPE, data: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<i32>;
    fn AddSpecialPregap(&self, data: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<()>;
    fn AddSubcodeRWGenerator(&self, subcode: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<()>;
    fn SetResultingImageType(&self, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::Result<()>;
    fn ResultingImageType(&self) -> windows_core::Result<IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE>;
    fn StartOfLeadout(&self) -> windows_core::Result<i32>;
    fn SetStartOfLeadoutLimit(&self, value: i32) -> windows_core::Result<()>;
    fn StartOfLeadoutLimit(&self) -> windows_core::Result<i32>;
    fn SetDisableGaplessAudio(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn DisableGaplessAudio(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetMediaCatalogNumber(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn MediaCatalogNumber(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetStartingTrackNumber(&self, value: i32) -> windows_core::Result<()>;
    fn StartingTrackNumber(&self) -> windows_core::Result<i32>;
    fn TrackInfo(&self, trackindex: i32) -> windows_core::Result<IRawCDImageTrackInfo>;
    fn NumberOfExistingTracks(&self) -> windows_core::Result<i32>;
    fn LastUsedUserSectorInImage(&self) -> windows_core::Result<i32>;
    fn ExpectedTableOfContents(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
}
#[cfg(all(feature = "oaidl", feature = "objidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
        unsafe extern "system" fn AddTrack<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, datatype: IMAPI_CD_SECTOR_TYPE, data: *mut core::ffi::c_void, trackindex: *mut i32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetResultingImageType<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageCreator_Impl::SetResultingImageType(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn ResultingImageType<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_FORMAT2_RAW_CD_DATA_SECTOR_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::ResultingImageType(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StartOfLeadout<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::StartOfLeadout(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStartOfLeadoutLimit<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageCreator_Impl::SetStartOfLeadoutLimit(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn StartOfLeadoutLimit<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::StartOfLeadoutLimit(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDisableGaplessAudio<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageCreator_Impl::SetDisableGaplessAudio(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn DisableGaplessAudio<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::DisableGaplessAudio(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetMediaCatalogNumber<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageCreator_Impl::SetMediaCatalogNumber(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn MediaCatalogNumber<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::MediaCatalogNumber(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStartingTrackNumber<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageCreator_Impl::SetStartingTrackNumber(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn StartingTrackNumber<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::StartingTrackNumber(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TrackInfo<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, trackindex: i32, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::TrackInfo(this, core::mem::transmute_copy(&trackindex)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NumberOfExistingTracks<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::NumberOfExistingTracks(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastUsedUserSectorInImage<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::LastUsedUserSectorInImage(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExpectedTableOfContents<Identity: IRawCDImageCreator_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageCreator_Impl::ExpectedTableOfContents(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            CreateResultImage: CreateResultImage::<Identity, OFFSET>,
            AddTrack: AddTrack::<Identity, OFFSET>,
            AddSpecialPregap: AddSpecialPregap::<Identity, OFFSET>,
            AddSubcodeRWGenerator: AddSubcodeRWGenerator::<Identity, OFFSET>,
            SetResultingImageType: SetResultingImageType::<Identity, OFFSET>,
            ResultingImageType: ResultingImageType::<Identity, OFFSET>,
            StartOfLeadout: StartOfLeadout::<Identity, OFFSET>,
            SetStartOfLeadoutLimit: SetStartOfLeadoutLimit::<Identity, OFFSET>,
            StartOfLeadoutLimit: StartOfLeadoutLimit::<Identity, OFFSET>,
            SetDisableGaplessAudio: SetDisableGaplessAudio::<Identity, OFFSET>,
            DisableGaplessAudio: DisableGaplessAudio::<Identity, OFFSET>,
            SetMediaCatalogNumber: SetMediaCatalogNumber::<Identity, OFFSET>,
            MediaCatalogNumber: MediaCatalogNumber::<Identity, OFFSET>,
            SetStartingTrackNumber: SetStartingTrackNumber::<Identity, OFFSET>,
            StartingTrackNumber: StartingTrackNumber::<Identity, OFFSET>,
            TrackInfo: TrackInfo::<Identity, OFFSET>,
            NumberOfExistingTracks: NumberOfExistingTracks::<Identity, OFFSET>,
            LastUsedUserSectorInImage: LastUsedUserSectorInImage::<Identity, OFFSET>,
            ExpectedTableOfContents: ExpectedTableOfContents::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRawCDImageCreator as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "objidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IRawCDImageCreator {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IRawCDImageTrackInfo, IRawCDImageTrackInfo_Vtbl, 0x25983551_9d65_49ce_b335_40630d901227);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IRawCDImageTrackInfo {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IRawCDImageTrackInfo, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IRawCDImageTrackInfo {
    pub unsafe fn StartingLba(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartingLba)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SectorCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SectorCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TrackNumber(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TrackNumber)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SectorType(&self) -> windows_core::Result<IMAPI_CD_SECTOR_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SectorType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ISRC(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ISRC)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetISRC(&self, value: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetISRC)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value)) }
    }
    pub unsafe fn DigitalAudioCopySetting(&self) -> windows_core::Result<IMAPI_CD_TRACK_DIGITAL_COPY_SETTING> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DigitalAudioCopySetting)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetDigitalAudioCopySetting(&self, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDigitalAudioCopySetting)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn AudioHasPreemphasis(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AudioHasPreemphasis)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetAudioHasPreemphasis(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAudioHasPreemphasis)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn TrackIndexes(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TrackIndexes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AddTrackIndex(&self, lbaoffset: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddTrackIndex)(windows_core::Interface::as_raw(self), lbaoffset) }
    }
    pub unsafe fn ClearTrackIndex(&self, lbaoffset: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearTrackIndex)(windows_core::Interface::as_raw(self), lbaoffset) }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IRawCDImageTrackInfo_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub StartingLba: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SectorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TrackNumber: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SectorType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_CD_SECTOR_TYPE) -> windows_core::HRESULT,
    pub ISRC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetISRC: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DigitalAudioCopySetting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> windows_core::HRESULT,
    pub SetDigitalAudioCopySetting: unsafe extern "system" fn(*mut core::ffi::c_void, IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub AudioHasPreemphasis: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    AudioHasPreemphasis: usize,
    #[cfg(feature = "wtypes")]
    pub SetAudioHasPreemphasis: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetAudioHasPreemphasis: usize,
    pub TrackIndexes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    pub AddTrackIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub ClearTrackIndex: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IRawCDImageTrackInfo_Impl: super::oaidl::IDispatch_Impl {
    fn StartingLba(&self) -> windows_core::Result<i32>;
    fn SectorCount(&self) -> windows_core::Result<i32>;
    fn TrackNumber(&self) -> windows_core::Result<i32>;
    fn SectorType(&self) -> windows_core::Result<IMAPI_CD_SECTOR_TYPE>;
    fn ISRC(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetISRC(&self, value: &windows_core::BSTR) -> windows_core::Result<()>;
    fn DigitalAudioCopySetting(&self) -> windows_core::Result<IMAPI_CD_TRACK_DIGITAL_COPY_SETTING>;
    fn SetDigitalAudioCopySetting(&self, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> windows_core::Result<()>;
    fn AudioHasPreemphasis(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetAudioHasPreemphasis(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn TrackIndexes(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn AddTrackIndex(&self, lbaoffset: i32) -> windows_core::Result<()>;
    fn ClearTrackIndex(&self, lbaoffset: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IRawCDImageTrackInfo_Vtbl {
    pub const fn new<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartingLba<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageTrackInfo_Impl::StartingLba(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SectorCount<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageTrackInfo_Impl::SectorCount(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TrackNumber<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageTrackInfo_Impl::TrackNumber(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SectorType<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_CD_SECTOR_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageTrackInfo_Impl::SectorType(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ISRC<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageTrackInfo_Impl::ISRC(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetISRC<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageTrackInfo_Impl::SetISRC(this, core::mem::transmute(&value)).into()
            }
        }
        unsafe extern "system" fn DigitalAudioCopySetting<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageTrackInfo_Impl::DigitalAudioCopySetting(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetDigitalAudioCopySetting<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: IMAPI_CD_TRACK_DIGITAL_COPY_SETTING) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageTrackInfo_Impl::SetDigitalAudioCopySetting(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn AudioHasPreemphasis<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageTrackInfo_Impl::AudioHasPreemphasis(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAudioHasPreemphasis<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageTrackInfo_Impl::SetAudioHasPreemphasis(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn TrackIndexes<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawCDImageTrackInfo_Impl::TrackIndexes(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddTrackIndex<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbaoffset: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageTrackInfo_Impl::AddTrackIndex(this, core::mem::transmute_copy(&lbaoffset)).into()
            }
        }
        unsafe extern "system" fn ClearTrackIndex<Identity: IRawCDImageTrackInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, lbaoffset: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawCDImageTrackInfo_Impl::ClearTrackIndex(this, core::mem::transmute_copy(&lbaoffset)).into()
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            StartingLba: StartingLba::<Identity, OFFSET>,
            SectorCount: SectorCount::<Identity, OFFSET>,
            TrackNumber: TrackNumber::<Identity, OFFSET>,
            SectorType: SectorType::<Identity, OFFSET>,
            ISRC: ISRC::<Identity, OFFSET>,
            SetISRC: SetISRC::<Identity, OFFSET>,
            DigitalAudioCopySetting: DigitalAudioCopySetting::<Identity, OFFSET>,
            SetDigitalAudioCopySetting: SetDigitalAudioCopySetting::<Identity, OFFSET>,
            AudioHasPreemphasis: AudioHasPreemphasis::<Identity, OFFSET>,
            SetAudioHasPreemphasis: SetAudioHasPreemphasis::<Identity, OFFSET>,
            TrackIndexes: TrackIndexes::<Identity, OFFSET>,
            AddTrackIndex: AddTrackIndex::<Identity, OFFSET>,
            ClearTrackIndex: ClearTrackIndex::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRawCDImageTrackInfo as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IRawCDImageTrackInfo {}
#[cfg(feature = "objidlbase")]
windows_core::imp::define_interface!(IStreamConcatenate, IStreamConcatenate_Vtbl, 0x27354146_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "objidlbase")]
impl core::ops::Deref for IStreamConcatenate {
    type Target = super::objidlbase::IStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "objidlbase")]
windows_core::imp::interface_hierarchy!(IStreamConcatenate, windows_core::IUnknown, super::objidlbase::ISequentialStream, super::objidlbase::IStream);
#[cfg(feature = "objidlbase")]
impl IStreamConcatenate {
    pub unsafe fn Initialize<P0, P1>(&self, stream1: P0, stream2: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
        P1: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), stream1.param().abi(), stream2.param().abi()) }
    }
    pub unsafe fn Initialize2(&self, streams: *const Option<super::objidlbase::IStream>, streamcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Initialize2)(windows_core::Interface::as_raw(self), core::mem::transmute(streams), streamcount) }
    }
    pub unsafe fn Append<P0>(&self, stream: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).Append)(windows_core::Interface::as_raw(self), stream.param().abi()) }
    }
    pub unsafe fn Append2(&self, streams: *const Option<super::objidlbase::IStream>, streamcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Append2)(windows_core::Interface::as_raw(self), core::mem::transmute(streams), streamcount) }
    }
}
#[cfg(feature = "objidlbase")]
#[repr(C)]
#[doc(hidden)]
pub struct IStreamConcatenate_Vtbl {
    pub base__: super::objidlbase::IStream_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Initialize2: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub Append: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Append2: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
pub trait IStreamConcatenate_Impl: super::objidlbase::IStream_Impl {
    fn Initialize(&self, stream1: windows_core::Ref<super::objidlbase::IStream>, stream2: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<()>;
    fn Initialize2(&self, streams: *const Option<super::objidlbase::IStream>, streamcount: u32) -> windows_core::Result<()>;
    fn Append(&self, stream: windows_core::Ref<super::objidlbase::IStream>) -> windows_core::Result<()>;
    fn Append2(&self, streams: *const Option<super::objidlbase::IStream>, streamcount: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
impl IStreamConcatenate_Vtbl {
    pub const fn new<Identity: IStreamConcatenate_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, stream1: *mut core::ffi::c_void, stream2: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStreamConcatenate_Impl::Initialize(this, core::mem::transmute_copy(&stream1), core::mem::transmute_copy(&stream2)).into()
            }
        }
        unsafe extern "system" fn Initialize2<Identity: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streams: *const *mut core::ffi::c_void, streamcount: u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Append2<Identity: IStreamConcatenate_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streams: *const *mut core::ffi::c_void, streamcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStreamConcatenate_Impl::Append2(this, core::mem::transmute_copy(&streams), core::mem::transmute_copy(&streamcount)).into()
            }
        }
        Self {
            base__: super::objidlbase::IStream_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Initialize2: Initialize2::<Identity, OFFSET>,
            Append: Append::<Identity, OFFSET>,
            Append2: Append2::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStreamConcatenate as windows_core::Interface>::IID || iid == &<super::objidlbase::ISequentialStream as windows_core::Interface>::IID || iid == &<super::objidlbase::IStream as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
impl windows_core::RuntimeName for IStreamConcatenate {}
#[cfg(feature = "objidlbase")]
windows_core::imp::define_interface!(IStreamInterleave, IStreamInterleave_Vtbl, 0x27354147_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "objidlbase")]
impl core::ops::Deref for IStreamInterleave {
    type Target = super::objidlbase::IStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "objidlbase")]
windows_core::imp::interface_hierarchy!(IStreamInterleave, windows_core::IUnknown, super::objidlbase::ISequentialStream, super::objidlbase::IStream);
#[cfg(feature = "objidlbase")]
impl IStreamInterleave {
    pub unsafe fn Initialize(&self, streams: *const Option<super::objidlbase::IStream>, interleavesizes: *const u32, streamcount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), core::mem::transmute(streams), interleavesizes, streamcount) }
    }
}
#[cfg(feature = "objidlbase")]
#[repr(C)]
#[doc(hidden)]
pub struct IStreamInterleave_Vtbl {
    pub base__: super::objidlbase::IStream_Vtbl,
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, *const u32, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
pub trait IStreamInterleave_Impl: super::objidlbase::IStream_Impl {
    fn Initialize(&self, streams: *const Option<super::objidlbase::IStream>, interleavesizes: *const u32, streamcount: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
impl IStreamInterleave_Vtbl {
    pub const fn new<Identity: IStreamInterleave_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IStreamInterleave_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, streams: *const *mut core::ffi::c_void, interleavesizes: *const u32, streamcount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStreamInterleave_Impl::Initialize(this, core::mem::transmute_copy(&streams), core::mem::transmute_copy(&interleavesizes), core::mem::transmute_copy(&streamcount)).into()
            }
        }
        Self { base__: super::objidlbase::IStream_Vtbl::new::<Identity, OFFSET>(), Initialize: Initialize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStreamInterleave as windows_core::Interface>::IID || iid == &<super::objidlbase::ISequentialStream as windows_core::Interface>::IID || iid == &<super::objidlbase::IStream as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
impl windows_core::RuntimeName for IStreamInterleave {}
#[cfg(feature = "objidlbase")]
windows_core::imp::define_interface!(IStreamPseudoRandomBased, IStreamPseudoRandomBased_Vtbl, 0x27354145_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "objidlbase")]
impl core::ops::Deref for IStreamPseudoRandomBased {
    type Target = super::objidlbase::IStream;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "objidlbase")]
windows_core::imp::interface_hierarchy!(IStreamPseudoRandomBased, windows_core::IUnknown, super::objidlbase::ISequentialStream, super::objidlbase::IStream);
#[cfg(feature = "objidlbase")]
impl IStreamPseudoRandomBased {
    pub unsafe fn put_Seed(&self, value: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_Seed)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn get_Seed(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Seed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ExtendedSeed(&self, values: *const u32, ecount: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ExtendedSeed)(windows_core::Interface::as_raw(self), values, ecount) }
    }
    pub unsafe fn get_ExtendedSeed(&self, values: *mut *mut u32, ecount: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).get_ExtendedSeed)(windows_core::Interface::as_raw(self), values as _, ecount as _) }
    }
}
#[cfg(feature = "objidlbase")]
#[repr(C)]
#[doc(hidden)]
pub struct IStreamPseudoRandomBased_Vtbl {
    pub base__: super::objidlbase::IStream_Vtbl,
    pub put_Seed: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub get_Seed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub put_ExtendedSeed: unsafe extern "system" fn(*mut core::ffi::c_void, *const u32, u32) -> windows_core::HRESULT,
    pub get_ExtendedSeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut u32, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
pub trait IStreamPseudoRandomBased_Impl: super::objidlbase::IStream_Impl {
    fn put_Seed(&self, value: u32) -> windows_core::Result<()>;
    fn get_Seed(&self) -> windows_core::Result<u32>;
    fn put_ExtendedSeed(&self, values: *const u32, ecount: u32) -> windows_core::Result<()>;
    fn get_ExtendedSeed(&self, values: *mut *mut u32, ecount: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
impl IStreamPseudoRandomBased_Vtbl {
    pub const fn new<Identity: IStreamPseudoRandomBased_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn put_Seed<Identity: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStreamPseudoRandomBased_Impl::put_Seed(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn get_Seed<Identity: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut u32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn put_ExtendedSeed<Identity: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, values: *const u32, ecount: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStreamPseudoRandomBased_Impl::put_ExtendedSeed(this, core::mem::transmute_copy(&values), core::mem::transmute_copy(&ecount)).into()
            }
        }
        unsafe extern "system" fn get_ExtendedSeed<Identity: IStreamPseudoRandomBased_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, values: *mut *mut u32, ecount: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IStreamPseudoRandomBased_Impl::get_ExtendedSeed(this, core::mem::transmute_copy(&values), core::mem::transmute_copy(&ecount)).into()
            }
        }
        Self {
            base__: super::objidlbase::IStream_Vtbl::new::<Identity, OFFSET>(),
            put_Seed: put_Seed::<Identity, OFFSET>,
            get_Seed: get_Seed::<Identity, OFFSET>,
            put_ExtendedSeed: put_ExtendedSeed::<Identity, OFFSET>,
            get_ExtendedSeed: get_ExtendedSeed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStreamPseudoRandomBased as windows_core::Interface>::IID || iid == &<super::objidlbase::ISequentialStream as windows_core::Interface>::IID || iid == &<super::objidlbase::IStream as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "objidlbase"))]
impl windows_core::RuntimeName for IStreamPseudoRandomBased {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWriteEngine2, IWriteEngine2_Vtbl, 0x27354135_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWriteEngine2 {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWriteEngine2, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IWriteEngine2 {
    #[cfg(feature = "objidlbase")]
    pub unsafe fn WriteSection<P0>(&self, data: P0, startingblockaddress: i32, numberofblocks: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<super::objidlbase::IStream>,
    {
        unsafe { (windows_core::Interface::vtable(self).WriteSection)(windows_core::Interface::as_raw(self), data.param().abi(), startingblockaddress, numberofblocks) }
    }
    pub unsafe fn CancelWrite(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CancelWrite)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetRecorder<P0>(&self, value: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IDiscRecorder2Ex>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRecorder)(windows_core::Interface::as_raw(self), value.param().abi()) }
    }
    pub unsafe fn Recorder(&self) -> windows_core::Result<IDiscRecorder2Ex> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Recorder)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn SetUseStreamingWrite12(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetUseStreamingWrite12)(windows_core::Interface::as_raw(self), value) }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn UseStreamingWrite12(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UseStreamingWrite12)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetStartingSectorsPerSecond(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetStartingSectorsPerSecond)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn StartingSectorsPerSecond(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartingSectorsPerSecond)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetEndingSectorsPerSecond(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetEndingSectorsPerSecond)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn EndingSectorsPerSecond(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).EndingSectorsPerSecond)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetBytesPerSector(&self, value: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBytesPerSector)(windows_core::Interface::as_raw(self), value) }
    }
    pub unsafe fn BytesPerSector(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BytesPerSector)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn WriteInProgress(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WriteInProgress)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWriteEngine2_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    #[cfg(feature = "objidlbase")]
    pub WriteSection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "objidlbase"))]
    WriteSection: usize,
    pub CancelWrite: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRecorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Recorder: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub SetUseStreamingWrite12: unsafe extern "system" fn(*mut core::ffi::c_void, super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    SetUseStreamingWrite12: usize,
    #[cfg(feature = "wtypes")]
    pub UseStreamingWrite12: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    UseStreamingWrite12: usize,
    pub SetStartingSectorsPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub StartingSectorsPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetEndingSectorsPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub EndingSectorsPerSecond: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SetBytesPerSector: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub BytesPerSector: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub WriteInProgress: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    WriteInProgress: usize,
}
#[cfg(all(feature = "oaidl", feature = "objidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWriteEngine2_Impl: super::oaidl::IDispatch_Impl {
    fn WriteSection(&self, data: windows_core::Ref<super::objidlbase::IStream>, startingblockaddress: i32, numberofblocks: i32) -> windows_core::Result<()>;
    fn CancelWrite(&self) -> windows_core::Result<()>;
    fn SetRecorder(&self, value: windows_core::Ref<IDiscRecorder2Ex>) -> windows_core::Result<()>;
    fn Recorder(&self) -> windows_core::Result<IDiscRecorder2Ex>;
    fn SetUseStreamingWrite12(&self, value: super::wtypes::VARIANT_BOOL) -> windows_core::Result<()>;
    fn UseStreamingWrite12(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn SetStartingSectorsPerSecond(&self, value: i32) -> windows_core::Result<()>;
    fn StartingSectorsPerSecond(&self) -> windows_core::Result<i32>;
    fn SetEndingSectorsPerSecond(&self, value: i32) -> windows_core::Result<()>;
    fn EndingSectorsPerSecond(&self) -> windows_core::Result<i32>;
    fn SetBytesPerSector(&self, value: i32) -> windows_core::Result<()>;
    fn BytesPerSector(&self) -> windows_core::Result<i32>;
    fn WriteInProgress(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
}
#[cfg(all(feature = "oaidl", feature = "objidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWriteEngine2_Vtbl {
    pub const fn new<Identity: IWriteEngine2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn WriteSection<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, data: *mut core::ffi::c_void, startingblockaddress: i32, numberofblocks: i32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetRecorder<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWriteEngine2_Impl::SetRecorder(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn Recorder<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2_Impl::Recorder(this) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetUseStreamingWrite12<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWriteEngine2_Impl::SetUseStreamingWrite12(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn UseStreamingWrite12<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2_Impl::UseStreamingWrite12(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetStartingSectorsPerSecond<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWriteEngine2_Impl::SetStartingSectorsPerSecond(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn StartingSectorsPerSecond<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2_Impl::StartingSectorsPerSecond(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetEndingSectorsPerSecond<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWriteEngine2_Impl::SetEndingSectorsPerSecond(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn EndingSectorsPerSecond<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2_Impl::EndingSectorsPerSecond(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBytesPerSector<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWriteEngine2_Impl::SetBytesPerSector(this, core::mem::transmute_copy(&value)).into()
            }
        }
        unsafe extern "system" fn BytesPerSector<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2_Impl::BytesPerSector(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WriteInProgress<Identity: IWriteEngine2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2_Impl::WriteInProgress(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            WriteSection: WriteSection::<Identity, OFFSET>,
            CancelWrite: CancelWrite::<Identity, OFFSET>,
            SetRecorder: SetRecorder::<Identity, OFFSET>,
            Recorder: Recorder::<Identity, OFFSET>,
            SetUseStreamingWrite12: SetUseStreamingWrite12::<Identity, OFFSET>,
            UseStreamingWrite12: UseStreamingWrite12::<Identity, OFFSET>,
            SetStartingSectorsPerSecond: SetStartingSectorsPerSecond::<Identity, OFFSET>,
            StartingSectorsPerSecond: StartingSectorsPerSecond::<Identity, OFFSET>,
            SetEndingSectorsPerSecond: SetEndingSectorsPerSecond::<Identity, OFFSET>,
            EndingSectorsPerSecond: EndingSectorsPerSecond::<Identity, OFFSET>,
            SetBytesPerSector: SetBytesPerSector::<Identity, OFFSET>,
            BytesPerSector: BytesPerSector::<Identity, OFFSET>,
            WriteInProgress: WriteInProgress::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWriteEngine2 as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "objidlbase", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWriteEngine2 {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWriteEngine2EventArgs, IWriteEngine2EventArgs_Vtbl, 0x27354136_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWriteEngine2EventArgs {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWriteEngine2EventArgs, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IWriteEngine2EventArgs {
    pub unsafe fn StartLba(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StartLba)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SectorCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SectorCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LastReadLba(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastReadLba)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LastWrittenLba(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastWrittenLba)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn TotalSystemBuffer(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TotalSystemBuffer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UsedSystemBuffer(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).UsedSystemBuffer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FreeSystemBuffer(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FreeSystemBuffer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWriteEngine2EventArgs_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub StartLba: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub SectorCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastReadLba: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub LastWrittenLba: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub TotalSystemBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub UsedSystemBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub FreeSystemBuffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWriteEngine2EventArgs_Impl: super::oaidl::IDispatch_Impl {
    fn StartLba(&self) -> windows_core::Result<i32>;
    fn SectorCount(&self) -> windows_core::Result<i32>;
    fn LastReadLba(&self) -> windows_core::Result<i32>;
    fn LastWrittenLba(&self) -> windows_core::Result<i32>;
    fn TotalSystemBuffer(&self) -> windows_core::Result<i32>;
    fn UsedSystemBuffer(&self) -> windows_core::Result<i32>;
    fn FreeSystemBuffer(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWriteEngine2EventArgs_Vtbl {
    pub const fn new<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartLba<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2EventArgs_Impl::StartLba(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SectorCount<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2EventArgs_Impl::SectorCount(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastReadLba<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2EventArgs_Impl::LastReadLba(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastWrittenLba<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2EventArgs_Impl::LastWrittenLba(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TotalSystemBuffer<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2EventArgs_Impl::TotalSystemBuffer(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UsedSystemBuffer<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2EventArgs_Impl::UsedSystemBuffer(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FreeSystemBuffer<Identity: IWriteEngine2EventArgs_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteEngine2EventArgs_Impl::FreeSystemBuffer(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            StartLba: StartLba::<Identity, OFFSET>,
            SectorCount: SectorCount::<Identity, OFFSET>,
            LastReadLba: LastReadLba::<Identity, OFFSET>,
            LastWrittenLba: LastWrittenLba::<Identity, OFFSET>,
            TotalSystemBuffer: TotalSystemBuffer::<Identity, OFFSET>,
            UsedSystemBuffer: UsedSystemBuffer::<Identity, OFFSET>,
            FreeSystemBuffer: FreeSystemBuffer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWriteEngine2EventArgs as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IWriteEngine2EventArgs {}
#[cfg(feature = "oaidl")]
windows_core::imp::define_interface!(IWriteSpeedDescriptor, IWriteSpeedDescriptor_Vtbl, 0x27354144_7f64_5b0f_8f00_5d77afbe261e);
#[cfg(feature = "oaidl")]
impl core::ops::Deref for IWriteSpeedDescriptor {
    type Target = super::oaidl::IDispatch;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
#[cfg(feature = "oaidl")]
windows_core::imp::interface_hierarchy!(IWriteSpeedDescriptor, windows_core::IUnknown, super::oaidl::IDispatch);
#[cfg(feature = "oaidl")]
impl IWriteSpeedDescriptor {
    pub unsafe fn MediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MediaType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "wtypes")]
    pub unsafe fn RotationTypeIsPureCAV(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RotationTypeIsPureCAV)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn WriteSpeed(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WriteSpeed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[cfg(feature = "oaidl")]
#[repr(C)]
#[doc(hidden)]
pub struct IWriteSpeedDescriptor_Vtbl {
    pub base__: super::oaidl::IDispatch_Vtbl,
    pub MediaType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT,
    #[cfg(feature = "wtypes")]
    pub RotationTypeIsPureCAV: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "wtypes"))]
    RotationTypeIsPureCAV: usize,
    pub WriteSpeed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IWriteSpeedDescriptor_Impl: super::oaidl::IDispatch_Impl {
    fn MediaType(&self) -> windows_core::Result<IMAPI_MEDIA_PHYSICAL_TYPE>;
    fn RotationTypeIsPureCAV(&self) -> windows_core::Result<super::wtypes::VARIANT_BOOL>;
    fn WriteSpeed(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IWriteSpeedDescriptor_Vtbl {
    pub const fn new<Identity: IWriteSpeedDescriptor_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn MediaType<Identity: IWriteSpeedDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut IMAPI_MEDIA_PHYSICAL_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteSpeedDescriptor_Impl::MediaType(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RotationTypeIsPureCAV<Identity: IWriteSpeedDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut super::wtypes::VARIANT_BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteSpeedDescriptor_Impl::RotationTypeIsPureCAV(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WriteSpeed<Identity: IWriteSpeedDescriptor_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWriteSpeedDescriptor_Impl::WriteSpeed(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: super::oaidl::IDispatch_Vtbl::new::<Identity, OFFSET>(),
            MediaType: MediaType::<Identity, OFFSET>,
            RotationTypeIsPureCAV: RotationTypeIsPureCAV::<Identity, OFFSET>,
            WriteSpeed: WriteSpeed::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWriteSpeedDescriptor as windows_core::Interface>::IID || iid == &<super::oaidl::IDispatch as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ULONG_IMAPI2_ADAPTER_DESCRIPTOR(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ULONG_IMAPI2_ALL_FEATURE_PAGES(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ULONG_IMAPI2_ALL_MODE_PAGES(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ULONG_IMAPI2_ALL_PROFILES(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ULONG_IMAPI2_DEVICE_DESCRIPTOR(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ULONG_IMAPI2_DISC_INFORMATION(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ULONG_IMAPI2_DVD_STRUCTURE(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ULONG_IMAPI2_FEATURE_PAGE(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ULONG_IMAPI2_MODE_PAGE(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ULONG_IMAPI2_NONZERO(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ULONG_IMAPI2_NOT_NEGATIVE(pub u32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ULONG_IMAPI2_TRACK_INFORMATION(pub u32);
