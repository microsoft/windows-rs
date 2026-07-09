#[inline]
pub unsafe fn CreateVssExpressWriterInternal() -> windows_core::Result<IVssExpressWriter> {
    windows_core::link!("vssapi.dll" "system" fn CreateVssExpressWriterInternal(ppwriter : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateVssExpressWriterInternal(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateWriter<P0>(pwriter: P0) -> windows_core::Result<IVssWriterImpl>
where
    P0: windows_core::Param<CVssWriter>,
{
    windows_core::link!("vssapi.dll" "system" fn CreateWriter(pwriter : *mut core::ffi::c_void, pwriterimpl : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateWriter(pwriter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn CreateWriterEx<P0>(pwriter: P0) -> windows_core::Result<IVssWriterImpl>
where
    P0: windows_core::Param<CVssWriterEx>,
{
    windows_core::link!("vssapi.dll" "system" fn CreateWriterEx(pwriter : *mut core::ffi::c_void, pwriterimpl : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        CreateWriterEx(pwriter.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
windows_core::imp::define_interface!(CVssWriter, CVssWriter_Vtbl);
impl CVssWriter {
    pub unsafe fn OnPrepareSnapshot(&self) -> bool {
        unsafe { (windows_core::Interface::vtable(self).OnPrepareSnapshot)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnFreeze(&self) -> bool {
        unsafe { (windows_core::Interface::vtable(self).OnFreeze)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnThaw(&self) -> bool {
        unsafe { (windows_core::Interface::vtable(self).OnThaw)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn OnAbort(&self) -> bool {
        unsafe { (windows_core::Interface::vtable(self).OnAbort)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct CVssWriter_Vtbl {
    pub OnPrepareSnapshot: unsafe extern "system" fn(*mut core::ffi::c_void) -> bool,
    pub OnFreeze: unsafe extern "system" fn(*mut core::ffi::c_void) -> bool,
    pub OnThaw: unsafe extern "system" fn(*mut core::ffi::c_void) -> bool,
    pub OnAbort: unsafe extern "system" fn(*mut core::ffi::c_void) -> bool,
}
pub trait CVssWriter_Impl {
    fn OnPrepareSnapshot(&self) -> bool;
    fn OnFreeze(&self) -> bool;
    fn OnThaw(&self) -> bool;
    fn OnAbort(&self) -> bool;
}
impl CVssWriter_Vtbl {
    pub const fn new<Identity: CVssWriter_Impl>() -> Self {
        unsafe extern "system" fn OnPrepareSnapshot<Identity: CVssWriter_Impl>(this: *mut core::ffi::c_void) -> bool {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                CVssWriter_Impl::OnPrepareSnapshot(this)
            }
        }
        unsafe extern "system" fn OnFreeze<Identity: CVssWriter_Impl>(this: *mut core::ffi::c_void) -> bool {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                CVssWriter_Impl::OnFreeze(this)
            }
        }
        unsafe extern "system" fn OnThaw<Identity: CVssWriter_Impl>(this: *mut core::ffi::c_void) -> bool {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                CVssWriter_Impl::OnThaw(this)
            }
        }
        unsafe extern "system" fn OnAbort<Identity: CVssWriter_Impl>(this: *mut core::ffi::c_void) -> bool {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                CVssWriter_Impl::OnAbort(this)
            }
        }
        Self { OnPrepareSnapshot: OnPrepareSnapshot::<Identity>, OnFreeze: OnFreeze::<Identity>, OnThaw: OnThaw::<Identity>, OnAbort: OnAbort::<Identity> }
    }
}
struct CVssWriter_ImplVtbl<T: CVssWriter_Impl>(core::marker::PhantomData<T>);
impl<T: CVssWriter_Impl> CVssWriter_ImplVtbl<T> {
    const VTABLE: CVssWriter_Vtbl = CVssWriter_Vtbl::new::<T>();
}
impl CVssWriter {
    pub fn new<'a, T: CVssWriter_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &CVssWriter_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(CVssWriterEx, CVssWriterEx_Vtbl);
impl core::ops::Deref for CVssWriterEx {
    type Target = CVssWriter;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(CVssWriterEx, CVssWriter);
#[repr(C)]
#[doc(hidden)]
pub struct CVssWriterEx_Vtbl {
    pub base__: CVssWriter_Vtbl,
}
pub trait CVssWriterEx_Impl: CVssWriter_Impl {}
impl CVssWriterEx_Vtbl {
    pub const fn new<Identity: CVssWriterEx_Impl>() -> Self {
        Self { base__: CVssWriter_Vtbl::new::<Identity>() }
    }
}
struct CVssWriterEx_ImplVtbl<T: CVssWriterEx_Impl>(core::marker::PhantomData<T>);
impl<T: CVssWriterEx_Impl> CVssWriterEx_ImplVtbl<T> {
    const VTABLE: CVssWriterEx_Vtbl = CVssWriterEx_Vtbl::new::<T>();
}
impl CVssWriterEx {
    pub fn new<'a, T: CVssWriterEx_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &CVssWriterEx_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(CVssWriterEx2, CVssWriterEx2_Vtbl);
impl core::ops::Deref for CVssWriterEx2 {
    type Target = CVssWriterEx;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(CVssWriterEx2, CVssWriter, CVssWriterEx);
#[repr(C)]
#[doc(hidden)]
pub struct CVssWriterEx2_Vtbl {
    pub base__: CVssWriterEx_Vtbl,
}
pub trait CVssWriterEx2_Impl: CVssWriterEx_Impl {}
impl CVssWriterEx2_Vtbl {
    pub const fn new<Identity: CVssWriterEx2_Impl>() -> Self {
        Self { base__: CVssWriterEx_Vtbl::new::<Identity>() }
    }
}
struct CVssWriterEx2_ImplVtbl<T: CVssWriterEx2_Impl>(core::marker::PhantomData<T>);
impl<T: CVssWriterEx2_Impl> CVssWriterEx2_ImplVtbl<T> {
    const VTABLE: CVssWriterEx2_Vtbl = CVssWriterEx2_Vtbl::new::<T>();
}
impl CVssWriterEx2 {
    pub fn new<'a, T: CVssWriterEx2_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &CVssWriterEx2_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(IVssComponent, IVssComponent_Vtbl, 0xd2c72c96_c121_4518_b627_e5a93d010ead);
windows_core::imp::interface_hierarchy!(IVssComponent, windows_core::IUnknown);
impl IVssComponent {
    pub unsafe fn GetLogicalPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLogicalPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetComponentType(&self) -> windows_core::Result<VSS_COMPONENT_TYPE> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetComponentType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetComponentName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetComponentName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetBackupSucceeded(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackupSucceeded)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAlternateLocationMappingCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAlternateLocationMappingCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAlternateLocationMapping(&self, imapping: u32) -> windows_core::Result<IVssWMFiledesc> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAlternateLocationMapping)(windows_core::Interface::as_raw(self), imapping, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetBackupMetadata<P0>(&self, wszdata: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBackupMetadata)(windows_core::Interface::as_raw(self), wszdata.param().abi()) }
    }
    pub unsafe fn GetBackupMetadata(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackupMetadata)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn AddPartialFile<P0, P1, P2, P3>(&self, wszpath: P0, wszfilename: P1, wszranges: P2, wszmetadata: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddPartialFile)(windows_core::Interface::as_raw(self), wszpath.param().abi(), wszfilename.param().abi(), wszranges.param().abi(), wszmetadata.param().abi()) }
    }
    pub unsafe fn GetPartialFileCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPartialFileCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPartialFile(&self, ipartialfile: u32, pbstrpath: *mut windows_core::BSTR, pbstrfilename: *mut windows_core::BSTR, pbstrrange: *mut windows_core::BSTR, pbstrmetadata: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPartialFile)(windows_core::Interface::as_raw(self), ipartialfile, core::mem::transmute(pbstrpath), core::mem::transmute(pbstrfilename), core::mem::transmute(pbstrrange), core::mem::transmute(pbstrmetadata)) }
    }
    pub unsafe fn IsSelectedForRestore(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSelectedForRestore)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAdditionalRestores(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAdditionalRestores)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetNewTargetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNewTargetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetNewTarget(&self, inewtarget: u32) -> windows_core::Result<IVssWMFiledesc> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNewTarget)(windows_core::Interface::as_raw(self), inewtarget, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddDirectedTarget<P0, P1, P2, P3, P4, P5>(&self, wszsourcepath: P0, wszsourcefilename: P1, wszsourcerangelist: P2, wszdestinationpath: P3, wszdestinationfilename: P4, wszdestinationrangelist: P5) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddDirectedTarget)(windows_core::Interface::as_raw(self), wszsourcepath.param().abi(), wszsourcefilename.param().abi(), wszsourcerangelist.param().abi(), wszdestinationpath.param().abi(), wszdestinationfilename.param().abi(), wszdestinationrangelist.param().abi()) }
    }
    pub unsafe fn GetDirectedTargetCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDirectedTargetCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetDirectedTarget(&self, idirectedtarget: u32, pbstrsourcepath: *mut windows_core::BSTR, pbstrsourcefilename: *mut windows_core::BSTR, pbstrsourcerangelist: *mut windows_core::BSTR, pbstrdestinationpath: *mut windows_core::BSTR, pbstrdestinationfilename: *mut windows_core::BSTR, pbstrdestinationrangelist: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDirectedTarget)(windows_core::Interface::as_raw(self), idirectedtarget, core::mem::transmute(pbstrsourcepath), core::mem::transmute(pbstrsourcefilename), core::mem::transmute(pbstrsourcerangelist), core::mem::transmute(pbstrdestinationpath), core::mem::transmute(pbstrdestinationfilename), core::mem::transmute(pbstrdestinationrangelist)) }
    }
    pub unsafe fn SetRestoreMetadata<P0>(&self, wszrestoremetadata: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRestoreMetadata)(windows_core::Interface::as_raw(self), wszrestoremetadata.param().abi()) }
    }
    pub unsafe fn GetRestoreMetadata(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRestoreMetadata)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetRestoreTarget(&self, target: VSS_RESTORE_TARGET) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetRestoreTarget)(windows_core::Interface::as_raw(self), target) }
    }
    pub unsafe fn GetRestoreTarget(&self) -> windows_core::Result<VSS_RESTORE_TARGET> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRestoreTarget)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetPreRestoreFailureMsg<P0>(&self, wszprerestorefailuremsg: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPreRestoreFailureMsg)(windows_core::Interface::as_raw(self), wszprerestorefailuremsg.param().abi()) }
    }
    pub unsafe fn GetPreRestoreFailureMsg(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPreRestoreFailureMsg)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetPostRestoreFailureMsg<P0>(&self, wszpostrestorefailuremsg: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPostRestoreFailureMsg)(windows_core::Interface::as_raw(self), wszpostrestorefailuremsg.param().abi()) }
    }
    pub unsafe fn GetPostRestoreFailureMsg(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPostRestoreFailureMsg)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetBackupStamp<P0>(&self, wszbackupstamp: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetBackupStamp)(windows_core::Interface::as_raw(self), wszbackupstamp.param().abi()) }
    }
    pub unsafe fn GetBackupStamp(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackupStamp)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetPreviousBackupStamp(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPreviousBackupStamp)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetBackupOptions(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackupOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetRestoreOptions(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRestoreOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetRestoreSubcomponentCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRestoreSubcomponentCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRestoreSubcomponent(&self, icomponent: u32, pbstrlogicalpath: *mut windows_core::BSTR, pbstrcomponentname: *mut windows_core::BSTR, pbrepair: *mut bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRestoreSubcomponent)(windows_core::Interface::as_raw(self), icomponent, core::mem::transmute(pbstrlogicalpath), core::mem::transmute(pbstrcomponentname), pbrepair as _) }
    }
    pub unsafe fn GetFileRestoreStatus(&self) -> windows_core::Result<VSS_FILE_RESTORE_STATUS> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFileRestoreStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_minwindef")]
    pub unsafe fn AddDifferencedFilesByLastModifyTime<P0, P1>(&self, wszpath: P0, wszfilespec: P1, brecursive: bool, ftlastmodifytime: super::minwindef::FILETIME) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddDifferencedFilesByLastModifyTime)(windows_core::Interface::as_raw(self), wszpath.param().abi(), wszfilespec.param().abi(), brecursive.into(), core::mem::transmute(ftlastmodifytime)) }
    }
    pub unsafe fn AddDifferencedFilesByLastModifyLSN<P0, P1>(&self, wszpath: P0, wszfilespec: P1, brecursive: bool, bstrlsnstring: &windows_core::BSTR) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddDifferencedFilesByLastModifyLSN)(windows_core::Interface::as_raw(self), wszpath.param().abi(), wszfilespec.param().abi(), brecursive.into(), core::mem::transmute_copy(bstrlsnstring)) }
    }
    pub unsafe fn GetDifferencedFilesCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDifferencedFilesCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_minwindef")]
    pub unsafe fn GetDifferencedFile(&self, idifferencedfile: u32, pbstrpath: *mut windows_core::BSTR, pbstrfilespec: *mut windows_core::BSTR, pbrecursive: *mut windows_core::BOOL, pbstrlsnstring: *mut windows_core::BSTR, pftlastmodifytime: *mut super::minwindef::FILETIME) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetDifferencedFile)(windows_core::Interface::as_raw(self), idifferencedfile, core::mem::transmute(pbstrpath), core::mem::transmute(pbstrfilespec), pbrecursive as _, core::mem::transmute(pbstrlsnstring), pftlastmodifytime as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssComponent_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetLogicalPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetComponentType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VSS_COMPONENT_TYPE) -> windows_core::HRESULT,
    pub GetComponentName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBackupSucceeded: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetAlternateLocationMappingCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetAlternateLocationMapping: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBackupMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetBackupMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddPartialFile: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetPartialFileCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetPartialFile: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSelectedForRestore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetAdditionalRestores: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetNewTargetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetNewTarget: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddDirectedTarget: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetDirectedTargetCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetDirectedTarget: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRestoreMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetRestoreMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetRestoreTarget: unsafe extern "system" fn(*mut core::ffi::c_void, VSS_RESTORE_TARGET) -> windows_core::HRESULT,
    pub GetRestoreTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VSS_RESTORE_TARGET) -> windows_core::HRESULT,
    pub SetPreRestoreFailureMsg: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetPreRestoreFailureMsg: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetPostRestoreFailureMsg: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetPostRestoreFailureMsg: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetBackupStamp: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetBackupStamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPreviousBackupStamp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBackupOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRestoreOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRestoreSubcomponentCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub GetRestoreSubcomponent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetFileRestoreStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut VSS_FILE_RESTORE_STATUS) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_minwindef")]
    pub AddDifferencedFilesByLastModifyTime: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::BOOL, super::minwindef::FILETIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_minwindef"))]
    AddDifferencedFilesByLastModifyTime: usize,
    pub AddDifferencedFilesByLastModifyLSN: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::BOOL, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetDifferencedFilesCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_minwindef")]
    pub GetDifferencedFile: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut windows_core::BOOL, *mut *mut core::ffi::c_void, *mut super::minwindef::FILETIME) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_minwindef"))]
    GetDifferencedFile: usize,
}
#[cfg(feature = "Win32_minwindef")]
pub trait IVssComponent_Impl: windows_core::IUnknownImpl {
    fn GetLogicalPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetComponentType(&self) -> windows_core::Result<VSS_COMPONENT_TYPE>;
    fn GetComponentName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetBackupSucceeded(&self) -> windows_core::Result<bool>;
    fn GetAlternateLocationMappingCount(&self) -> windows_core::Result<u32>;
    fn GetAlternateLocationMapping(&self, imapping: u32) -> windows_core::Result<IVssWMFiledesc>;
    fn SetBackupMetadata(&self, wszdata: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetBackupMetadata(&self) -> windows_core::Result<windows_core::BSTR>;
    fn AddPartialFile(&self, wszpath: &windows_core::PCWSTR, wszfilename: &windows_core::PCWSTR, wszranges: &windows_core::PCWSTR, wszmetadata: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPartialFileCount(&self) -> windows_core::Result<u32>;
    fn GetPartialFile(&self, ipartialfile: u32, pbstrpath: *mut windows_core::BSTR, pbstrfilename: *mut windows_core::BSTR, pbstrrange: *mut windows_core::BSTR, pbstrmetadata: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn IsSelectedForRestore(&self) -> windows_core::Result<bool>;
    fn GetAdditionalRestores(&self) -> windows_core::Result<bool>;
    fn GetNewTargetCount(&self) -> windows_core::Result<u32>;
    fn GetNewTarget(&self, inewtarget: u32) -> windows_core::Result<IVssWMFiledesc>;
    fn AddDirectedTarget(&self, wszsourcepath: &windows_core::PCWSTR, wszsourcefilename: &windows_core::PCWSTR, wszsourcerangelist: &windows_core::PCWSTR, wszdestinationpath: &windows_core::PCWSTR, wszdestinationfilename: &windows_core::PCWSTR, wszdestinationrangelist: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetDirectedTargetCount(&self) -> windows_core::Result<u32>;
    fn GetDirectedTarget(&self, idirectedtarget: u32, pbstrsourcepath: *mut windows_core::BSTR, pbstrsourcefilename: *mut windows_core::BSTR, pbstrsourcerangelist: *mut windows_core::BSTR, pbstrdestinationpath: *mut windows_core::BSTR, pbstrdestinationfilename: *mut windows_core::BSTR, pbstrdestinationrangelist: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn SetRestoreMetadata(&self, wszrestoremetadata: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetRestoreMetadata(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetRestoreTarget(&self, target: VSS_RESTORE_TARGET) -> windows_core::Result<()>;
    fn GetRestoreTarget(&self) -> windows_core::Result<VSS_RESTORE_TARGET>;
    fn SetPreRestoreFailureMsg(&self, wszprerestorefailuremsg: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPreRestoreFailureMsg(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetPostRestoreFailureMsg(&self, wszpostrestorefailuremsg: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPostRestoreFailureMsg(&self) -> windows_core::Result<windows_core::BSTR>;
    fn SetBackupStamp(&self, wszbackupstamp: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetBackupStamp(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPreviousBackupStamp(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetBackupOptions(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetRestoreOptions(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetRestoreSubcomponentCount(&self) -> windows_core::Result<u32>;
    fn GetRestoreSubcomponent(&self, icomponent: u32, pbstrlogicalpath: *mut windows_core::BSTR, pbstrcomponentname: *mut windows_core::BSTR, pbrepair: *mut bool) -> windows_core::Result<()>;
    fn GetFileRestoreStatus(&self) -> windows_core::Result<VSS_FILE_RESTORE_STATUS>;
    fn AddDifferencedFilesByLastModifyTime(&self, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, brecursive: windows_core::BOOL, ftlastmodifytime: &super::minwindef::FILETIME) -> windows_core::Result<()>;
    fn AddDifferencedFilesByLastModifyLSN(&self, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, brecursive: windows_core::BOOL, bstrlsnstring: &windows_core::BSTR) -> windows_core::Result<()>;
    fn GetDifferencedFilesCount(&self) -> windows_core::Result<u32>;
    fn GetDifferencedFile(&self, idifferencedfile: u32, pbstrpath: *mut windows_core::BSTR, pbstrfilespec: *mut windows_core::BSTR, pbrecursive: *mut windows_core::BOOL, pbstrlsnstring: *mut windows_core::BSTR, pftlastmodifytime: *mut super::minwindef::FILETIME) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_minwindef")]
impl IVssComponent_Vtbl {
    pub const fn new<Identity: IVssComponent_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetLogicalPath<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponent_Impl::GetLogicalPath(this) {
                    Ok(ok__) => {
                        pbstrpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetComponentType<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pct: *mut VSS_COMPONENT_TYPE) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponent_Impl::GetComponentType(this) {
                    Ok(ok__) => {
                        pct.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetComponentName<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponent_Impl::GetComponentName(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBackupSucceeded<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbsucceeded: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponent_Impl::GetBackupSucceeded(this) {
                    Ok(ok__) => {
                        pbsucceeded.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAlternateLocationMappingCount<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcmappings: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponent_Impl::GetAlternateLocationMappingCount(this) {
                    Ok(ok__) => {
                        pcmappings.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAlternateLocationMapping<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imapping: u32, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponent_Impl::GetAlternateLocationMapping(this, core::mem::transmute_copy(&imapping)) {
                    Ok(ok__) => {
                        ppfiledesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBackupMetadata<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszdata: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::SetBackupMetadata(this, core::mem::transmute(&wszdata)).into()
            }
        }
        unsafe extern "system" fn GetBackupMetadata<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponent_Impl::GetBackupMetadata(this) {
                    Ok(ok__) => {
                        pbstrdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddPartialFile<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, wszfilename: windows_core::PCWSTR, wszranges: windows_core::PCWSTR, wszmetadata: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::AddPartialFile(this, core::mem::transmute(&wszpath), core::mem::transmute(&wszfilename), core::mem::transmute(&wszranges), core::mem::transmute(&wszmetadata)).into()
            }
        }
        unsafe extern "system" fn GetPartialFileCount<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcpartialfiles: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponent_Impl::GetPartialFileCount(this) {
                    Ok(ok__) => {
                        pcpartialfiles.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPartialFile<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ipartialfile: u32, pbstrpath: *mut *mut core::ffi::c_void, pbstrfilename: *mut *mut core::ffi::c_void, pbstrrange: *mut *mut core::ffi::c_void, pbstrmetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetPartialFile(this, core::mem::transmute_copy(&ipartialfile), core::mem::transmute_copy(&pbstrpath), core::mem::transmute_copy(&pbstrfilename), core::mem::transmute_copy(&pbstrrange), core::mem::transmute_copy(&pbstrmetadata)).into()
            }
        }
        unsafe extern "system" fn IsSelectedForRestore<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbselectedforrestore: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponent_Impl::IsSelectedForRestore(this) {
                    Ok(ok__) => {
                        pbselectedforrestore.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAdditionalRestores<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbadditionalrestores: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponent_Impl::GetAdditionalRestores(this) {
                    Ok(ok__) => {
                        pbadditionalrestores.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNewTargetCount<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcnewtarget: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponent_Impl::GetNewTargetCount(this) {
                    Ok(ok__) => {
                        pcnewtarget.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNewTarget<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inewtarget: u32, ppfiledesc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponent_Impl::GetNewTarget(this, core::mem::transmute_copy(&inewtarget)) {
                    Ok(ok__) => {
                        ppfiledesc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddDirectedTarget<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszsourcepath: windows_core::PCWSTR, wszsourcefilename: windows_core::PCWSTR, wszsourcerangelist: windows_core::PCWSTR, wszdestinationpath: windows_core::PCWSTR, wszdestinationfilename: windows_core::PCWSTR, wszdestinationrangelist: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::AddDirectedTarget(this, core::mem::transmute(&wszsourcepath), core::mem::transmute(&wszsourcefilename), core::mem::transmute(&wszsourcerangelist), core::mem::transmute(&wszdestinationpath), core::mem::transmute(&wszdestinationfilename), core::mem::transmute(&wszdestinationrangelist)).into()
            }
        }
        unsafe extern "system" fn GetDirectedTargetCount<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdirectedtarget: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponent_Impl::GetDirectedTargetCount(this) {
                    Ok(ok__) => {
                        pcdirectedtarget.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDirectedTarget<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idirectedtarget: u32, pbstrsourcepath: *mut *mut core::ffi::c_void, pbstrsourcefilename: *mut *mut core::ffi::c_void, pbstrsourcerangelist: *mut *mut core::ffi::c_void, pbstrdestinationpath: *mut *mut core::ffi::c_void, pbstrdestinationfilename: *mut *mut core::ffi::c_void, pbstrdestinationrangelist: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetDirectedTarget(this, core::mem::transmute_copy(&idirectedtarget), core::mem::transmute_copy(&pbstrsourcepath), core::mem::transmute_copy(&pbstrsourcefilename), core::mem::transmute_copy(&pbstrsourcerangelist), core::mem::transmute_copy(&pbstrdestinationpath), core::mem::transmute_copy(&pbstrdestinationfilename), core::mem::transmute_copy(&pbstrdestinationrangelist)).into()
            }
        }
        unsafe extern "system" fn SetRestoreMetadata<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszrestoremetadata: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::SetRestoreMetadata(this, core::mem::transmute(&wszrestoremetadata)).into()
            }
        }
        unsafe extern "system" fn GetRestoreMetadata<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrrestoremetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponent_Impl::GetRestoreMetadata(this) {
                    Ok(ok__) => {
                        pbstrrestoremetadata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetRestoreTarget<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, target: VSS_RESTORE_TARGET) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::SetRestoreTarget(this, core::mem::transmute_copy(&target)).into()
            }
        }
        unsafe extern "system" fn GetRestoreTarget<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptarget: *mut VSS_RESTORE_TARGET) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponent_Impl::GetRestoreTarget(this) {
                    Ok(ok__) => {
                        ptarget.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPreRestoreFailureMsg<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszprerestorefailuremsg: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::SetPreRestoreFailureMsg(this, core::mem::transmute(&wszprerestorefailuremsg)).into()
            }
        }
        unsafe extern "system" fn GetPreRestoreFailureMsg<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrprerestorefailuremsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponent_Impl::GetPreRestoreFailureMsg(this) {
                    Ok(ok__) => {
                        pbstrprerestorefailuremsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetPostRestoreFailureMsg<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpostrestorefailuremsg: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::SetPostRestoreFailureMsg(this, core::mem::transmute(&wszpostrestorefailuremsg)).into()
            }
        }
        unsafe extern "system" fn GetPostRestoreFailureMsg<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpostrestorefailuremsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponent_Impl::GetPostRestoreFailureMsg(this) {
                    Ok(ok__) => {
                        pbstrpostrestorefailuremsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetBackupStamp<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszbackupstamp: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::SetBackupStamp(this, core::mem::transmute(&wszbackupstamp)).into()
            }
        }
        unsafe extern "system" fn GetBackupStamp<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbackupstamp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponent_Impl::GetBackupStamp(this) {
                    Ok(ok__) => {
                        pbstrbackupstamp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPreviousBackupStamp<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbackupstamp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponent_Impl::GetPreviousBackupStamp(this) {
                    Ok(ok__) => {
                        pbstrbackupstamp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBackupOptions<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrbackupoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponent_Impl::GetBackupOptions(this) {
                    Ok(ok__) => {
                        pbstrbackupoptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRestoreOptions<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrrestoreoptions: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponent_Impl::GetRestoreOptions(this) {
                    Ok(ok__) => {
                        pbstrrestoreoptions.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRestoreSubcomponentCount<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcrestoresubcomponent: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponent_Impl::GetRestoreSubcomponentCount(this) {
                    Ok(ok__) => {
                        pcrestoresubcomponent.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRestoreSubcomponent<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, icomponent: u32, pbstrlogicalpath: *mut *mut core::ffi::c_void, pbstrcomponentname: *mut *mut core::ffi::c_void, pbrepair: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetRestoreSubcomponent(this, core::mem::transmute_copy(&icomponent), core::mem::transmute_copy(&pbstrlogicalpath), core::mem::transmute_copy(&pbstrcomponentname), core::mem::transmute_copy(&pbrepair)).into()
            }
        }
        unsafe extern "system" fn GetFileRestoreStatus<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstatus: *mut VSS_FILE_RESTORE_STATUS) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponent_Impl::GetFileRestoreStatus(this) {
                    Ok(ok__) => {
                        pstatus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddDifferencedFilesByLastModifyTime<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: windows_core::BOOL, ftlastmodifytime: super::minwindef::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::AddDifferencedFilesByLastModifyTime(this, core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive), core::mem::transmute(&ftlastmodifytime)).into()
            }
        }
        unsafe extern "system" fn AddDifferencedFilesByLastModifyLSN<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: windows_core::BOOL, bstrlsnstring: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::AddDifferencedFilesByLastModifyLSN(this, core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive), core::mem::transmute(&bstrlsnstring)).into()
            }
        }
        unsafe extern "system" fn GetDifferencedFilesCount<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcdifferencedfiles: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponent_Impl::GetDifferencedFilesCount(this) {
                    Ok(ok__) => {
                        pcdifferencedfiles.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetDifferencedFile<Identity: IVssComponent_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idifferencedfile: u32, pbstrpath: *mut *mut core::ffi::c_void, pbstrfilespec: *mut *mut core::ffi::c_void, pbrecursive: *mut windows_core::BOOL, pbstrlsnstring: *mut *mut core::ffi::c_void, pftlastmodifytime: *mut super::minwindef::FILETIME) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponent_Impl::GetDifferencedFile(this, core::mem::transmute_copy(&idifferencedfile), core::mem::transmute_copy(&pbstrpath), core::mem::transmute_copy(&pbstrfilespec), core::mem::transmute_copy(&pbrecursive), core::mem::transmute_copy(&pbstrlsnstring), core::mem::transmute_copy(&pftlastmodifytime)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetLogicalPath: GetLogicalPath::<Identity, OFFSET>,
            GetComponentType: GetComponentType::<Identity, OFFSET>,
            GetComponentName: GetComponentName::<Identity, OFFSET>,
            GetBackupSucceeded: GetBackupSucceeded::<Identity, OFFSET>,
            GetAlternateLocationMappingCount: GetAlternateLocationMappingCount::<Identity, OFFSET>,
            GetAlternateLocationMapping: GetAlternateLocationMapping::<Identity, OFFSET>,
            SetBackupMetadata: SetBackupMetadata::<Identity, OFFSET>,
            GetBackupMetadata: GetBackupMetadata::<Identity, OFFSET>,
            AddPartialFile: AddPartialFile::<Identity, OFFSET>,
            GetPartialFileCount: GetPartialFileCount::<Identity, OFFSET>,
            GetPartialFile: GetPartialFile::<Identity, OFFSET>,
            IsSelectedForRestore: IsSelectedForRestore::<Identity, OFFSET>,
            GetAdditionalRestores: GetAdditionalRestores::<Identity, OFFSET>,
            GetNewTargetCount: GetNewTargetCount::<Identity, OFFSET>,
            GetNewTarget: GetNewTarget::<Identity, OFFSET>,
            AddDirectedTarget: AddDirectedTarget::<Identity, OFFSET>,
            GetDirectedTargetCount: GetDirectedTargetCount::<Identity, OFFSET>,
            GetDirectedTarget: GetDirectedTarget::<Identity, OFFSET>,
            SetRestoreMetadata: SetRestoreMetadata::<Identity, OFFSET>,
            GetRestoreMetadata: GetRestoreMetadata::<Identity, OFFSET>,
            SetRestoreTarget: SetRestoreTarget::<Identity, OFFSET>,
            GetRestoreTarget: GetRestoreTarget::<Identity, OFFSET>,
            SetPreRestoreFailureMsg: SetPreRestoreFailureMsg::<Identity, OFFSET>,
            GetPreRestoreFailureMsg: GetPreRestoreFailureMsg::<Identity, OFFSET>,
            SetPostRestoreFailureMsg: SetPostRestoreFailureMsg::<Identity, OFFSET>,
            GetPostRestoreFailureMsg: GetPostRestoreFailureMsg::<Identity, OFFSET>,
            SetBackupStamp: SetBackupStamp::<Identity, OFFSET>,
            GetBackupStamp: GetBackupStamp::<Identity, OFFSET>,
            GetPreviousBackupStamp: GetPreviousBackupStamp::<Identity, OFFSET>,
            GetBackupOptions: GetBackupOptions::<Identity, OFFSET>,
            GetRestoreOptions: GetRestoreOptions::<Identity, OFFSET>,
            GetRestoreSubcomponentCount: GetRestoreSubcomponentCount::<Identity, OFFSET>,
            GetRestoreSubcomponent: GetRestoreSubcomponent::<Identity, OFFSET>,
            GetFileRestoreStatus: GetFileRestoreStatus::<Identity, OFFSET>,
            AddDifferencedFilesByLastModifyTime: AddDifferencedFilesByLastModifyTime::<Identity, OFFSET>,
            AddDifferencedFilesByLastModifyLSN: AddDifferencedFilesByLastModifyLSN::<Identity, OFFSET>,
            GetDifferencedFilesCount: GetDifferencedFilesCount::<Identity, OFFSET>,
            GetDifferencedFile: GetDifferencedFile::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssComponent as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_minwindef")]
impl windows_core::RuntimeName for IVssComponent {}
windows_core::imp::define_interface!(IVssComponentEx, IVssComponentEx_Vtbl, 0x156c8b5e_f131_4bd7_9c97_d1923be7e1fa);
impl core::ops::Deref for IVssComponentEx {
    type Target = IVssComponent;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssComponentEx, windows_core::IUnknown, IVssComponent);
impl IVssComponentEx {
    pub unsafe fn SetPrepareForBackupFailureMsg<P0>(&self, wszfailuremsg: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPrepareForBackupFailureMsg)(windows_core::Interface::as_raw(self), wszfailuremsg.param().abi()) }
    }
    pub unsafe fn SetPostSnapshotFailureMsg<P0>(&self, wszfailuremsg: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetPostSnapshotFailureMsg)(windows_core::Interface::as_raw(self), wszfailuremsg.param().abi()) }
    }
    pub unsafe fn GetPrepareForBackupFailureMsg(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPrepareForBackupFailureMsg)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetPostSnapshotFailureMsg(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPostSnapshotFailureMsg)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetAuthoritativeRestore(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAuthoritativeRestore)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn GetRollForward(&self, prolltype: *mut super::vss::VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetRollForward)(windows_core::Interface::as_raw(self), prolltype as _, core::mem::transmute(pbstrpoint)) }
    }
    pub unsafe fn GetRestoreName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRestoreName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssComponentEx_Vtbl {
    pub base__: IVssComponent_Vtbl,
    pub SetPrepareForBackupFailureMsg: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetPostSnapshotFailureMsg: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub GetPrepareForBackupFailureMsg: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPostSnapshotFailureMsg: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetAuthoritativeRestore: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_vss")]
    pub GetRollForward: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::vss::VSS_ROLLFORWARD_TYPE, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    GetRollForward: usize,
    pub GetRestoreName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_vss"))]
pub trait IVssComponentEx_Impl: IVssComponent_Impl {
    fn SetPrepareForBackupFailureMsg(&self, wszfailuremsg: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetPostSnapshotFailureMsg(&self, wszfailuremsg: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetPrepareForBackupFailureMsg(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetPostSnapshotFailureMsg(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetAuthoritativeRestore(&self) -> windows_core::Result<bool>;
    fn GetRollForward(&self, prolltype: *mut super::vss::VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut windows_core::BSTR) -> windows_core::Result<()>;
    fn GetRestoreName(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_vss"))]
impl IVssComponentEx_Vtbl {
    pub const fn new<Identity: IVssComponentEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetPrepareForBackupFailureMsg<Identity: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszfailuremsg: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponentEx_Impl::SetPrepareForBackupFailureMsg(this, core::mem::transmute(&wszfailuremsg)).into()
            }
        }
        unsafe extern "system" fn SetPostSnapshotFailureMsg<Identity: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszfailuremsg: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponentEx_Impl::SetPostSnapshotFailureMsg(this, core::mem::transmute(&wszfailuremsg)).into()
            }
        }
        unsafe extern "system" fn GetPrepareForBackupFailureMsg<Identity: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfailuremsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponentEx_Impl::GetPrepareForBackupFailureMsg(this) {
                    Ok(ok__) => {
                        pbstrfailuremsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPostSnapshotFailureMsg<Identity: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfailuremsg: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponentEx_Impl::GetPostSnapshotFailureMsg(this) {
                    Ok(ok__) => {
                        pbstrfailuremsg.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAuthoritativeRestore<Identity: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbauth: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponentEx_Impl::GetAuthoritativeRestore(this) {
                    Ok(ok__) => {
                        pbauth.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRollForward<Identity: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, prolltype: *mut super::vss::VSS_ROLLFORWARD_TYPE, pbstrpoint: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponentEx_Impl::GetRollForward(this, core::mem::transmute_copy(&prolltype), core::mem::transmute_copy(&pbstrpoint)).into()
            }
        }
        unsafe extern "system" fn GetRestoreName<Identity: IVssComponentEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssComponentEx_Impl::GetRestoreName(this) {
                    Ok(ok__) => {
                        pbstrname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IVssComponent_Vtbl::new::<Identity, OFFSET>(),
            SetPrepareForBackupFailureMsg: SetPrepareForBackupFailureMsg::<Identity, OFFSET>,
            SetPostSnapshotFailureMsg: SetPostSnapshotFailureMsg::<Identity, OFFSET>,
            GetPrepareForBackupFailureMsg: GetPrepareForBackupFailureMsg::<Identity, OFFSET>,
            GetPostSnapshotFailureMsg: GetPostSnapshotFailureMsg::<Identity, OFFSET>,
            GetAuthoritativeRestore: GetAuthoritativeRestore::<Identity, OFFSET>,
            GetRollForward: GetRollForward::<Identity, OFFSET>,
            GetRestoreName: GetRestoreName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssComponentEx as windows_core::Interface>::IID || iid == &<IVssComponent as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_vss"))]
impl windows_core::RuntimeName for IVssComponentEx {}
windows_core::imp::define_interface!(IVssComponentEx2, IVssComponentEx2_Vtbl, 0x3b5be0f2_07a9_4e4b_bdd3_cfdc8e2c0d2d);
impl core::ops::Deref for IVssComponentEx2 {
    type Target = IVssComponentEx;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssComponentEx2, windows_core::IUnknown, IVssComponent, IVssComponentEx);
impl IVssComponentEx2 {
    pub unsafe fn SetFailure<P2>(&self, hr: windows_core::HRESULT, hrapplication: windows_core::HRESULT, wszapplicationmessage: P2, dwreserved: u32) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetFailure)(windows_core::Interface::as_raw(self), hr, hrapplication, wszapplicationmessage.param().abi(), dwreserved) }
    }
    pub unsafe fn GetFailure(&self, phr: *mut windows_core::HRESULT, phrapplication: *mut windows_core::HRESULT, pbstrapplicationmessage: *mut windows_core::BSTR, pdwreserved: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetFailure)(windows_core::Interface::as_raw(self), phr as _, phrapplication as _, core::mem::transmute(pbstrapplicationmessage), pdwreserved as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssComponentEx2_Vtbl {
    pub base__: IVssComponentEx_Vtbl,
    pub SetFailure: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, windows_core::HRESULT, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub GetFailure: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::HRESULT, *mut windows_core::HRESULT, *mut *mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_vss"))]
pub trait IVssComponentEx2_Impl: IVssComponentEx_Impl {
    fn SetFailure(&self, hr: windows_core::HRESULT, hrapplication: windows_core::HRESULT, wszapplicationmessage: &windows_core::PCWSTR, dwreserved: u32) -> windows_core::Result<()>;
    fn GetFailure(&self, phr: *mut windows_core::HRESULT, phrapplication: *mut windows_core::HRESULT, pbstrapplicationmessage: *mut windows_core::BSTR, pdwreserved: *mut u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_vss"))]
impl IVssComponentEx2_Vtbl {
    pub const fn new<Identity: IVssComponentEx2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetFailure<Identity: IVssComponentEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT, hrapplication: windows_core::HRESULT, wszapplicationmessage: windows_core::PCWSTR, dwreserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponentEx2_Impl::SetFailure(this, core::mem::transmute_copy(&hr), core::mem::transmute_copy(&hrapplication), core::mem::transmute(&wszapplicationmessage), core::mem::transmute_copy(&dwreserved)).into()
            }
        }
        unsafe extern "system" fn GetFailure<Identity: IVssComponentEx2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, phr: *mut windows_core::HRESULT, phrapplication: *mut windows_core::HRESULT, pbstrapplicationmessage: *mut *mut core::ffi::c_void, pdwreserved: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssComponentEx2_Impl::GetFailure(this, core::mem::transmute_copy(&phr), core::mem::transmute_copy(&phrapplication), core::mem::transmute_copy(&pbstrapplicationmessage), core::mem::transmute_copy(&pdwreserved)).into()
            }
        }
        Self { base__: IVssComponentEx_Vtbl::new::<Identity, OFFSET>(), SetFailure: SetFailure::<Identity, OFFSET>, GetFailure: GetFailure::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssComponentEx2 as windows_core::Interface>::IID || iid == &<IVssComponent as windows_core::Interface>::IID || iid == &<IVssComponentEx as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_minwindef", feature = "Win32_vss"))]
impl windows_core::RuntimeName for IVssComponentEx2 {}
windows_core::imp::define_interface!(IVssCreateExpressWriterMetadata, IVssCreateExpressWriterMetadata_Vtbl, 0x9c772e77_b26e_427f_92dd_c996f41ea5e3);
windows_core::imp::interface_hierarchy!(IVssCreateExpressWriterMetadata, windows_core::IUnknown);
impl IVssCreateExpressWriterMetadata {
    pub unsafe fn AddExcludeFiles<P0, P1>(&self, wszpath: P0, wszfilespec: P1, brecursive: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddExcludeFiles)(windows_core::Interface::as_raw(self), wszpath.param().abi(), wszfilespec.param().abi(), brecursive) }
    }
    pub unsafe fn AddComponent<P1, P2, P3>(&self, ct: VSS_COMPONENT_TYPE, wszlogicalpath: P1, wszcomponentname: P2, wszcaption: P3, pbicon: *const u8, cbicon: u32, brestoremetadata: bool, bnotifyonbackupcomplete: bool, bselectable: bool, bselectableforrestore: bool, dwcomponentflags: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddComponent)(windows_core::Interface::as_raw(self), ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), wszcaption.param().abi(), pbicon, cbicon, brestoremetadata, bnotifyonbackupcomplete, bselectable, bselectableforrestore, dwcomponentflags) }
    }
    pub unsafe fn AddFilesToFileGroup<P0, P1, P2, P3, P5>(&self, wszlogicalpath: P0, wszgroupname: P1, wszpath: P2, wszfilespec: P3, brecursive: bool, wszalternatelocation: P5, dwbackuptypemask: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddFilesToFileGroup)(windows_core::Interface::as_raw(self), wszlogicalpath.param().abi(), wszgroupname.param().abi(), wszpath.param().abi(), wszfilespec.param().abi(), brecursive, wszalternatelocation.param().abi(), dwbackuptypemask) }
    }
    pub unsafe fn SetRestoreMethod<P1, P2>(&self, method: VSS_RESTOREMETHOD_ENUM, wszservice: P1, wszuserprocedure: P2, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: bool) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRestoreMethod)(windows_core::Interface::as_raw(self), method, wszservice.param().abi(), wszuserprocedure.param().abi(), writerrestore, brebootrequired) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn AddComponentDependency<P0, P1, P3, P4>(&self, wszforlogicalpath: P0, wszforcomponentname: P1, onwriterid: super::vss::VSS_ID, wszonlogicalpath: P3, wszoncomponentname: P4) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddComponentDependency)(windows_core::Interface::as_raw(self), wszforlogicalpath.param().abi(), wszforcomponentname.param().abi(), core::mem::transmute(onwriterid), wszonlogicalpath.param().abi(), wszoncomponentname.param().abi()) }
    }
    pub unsafe fn SetBackupSchema(&self, dwschemamask: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBackupSchema)(windows_core::Interface::as_raw(self), dwschemamask) }
    }
    pub unsafe fn SaveAsXML(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SaveAsXML)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssCreateExpressWriterMetadata_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddExcludeFiles: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, bool) -> windows_core::HRESULT,
    pub AddComponent: unsafe extern "system" fn(*mut core::ffi::c_void, VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *const u8, u32, bool, bool, bool, bool, u32) -> windows_core::HRESULT,
    pub AddFilesToFileGroup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, bool, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub SetRestoreMethod: unsafe extern "system" fn(*mut core::ffi::c_void, VSS_RESTOREMETHOD_ENUM, windows_core::PCWSTR, windows_core::PCWSTR, VSS_WRITERRESTORE_ENUM, bool) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_vss")]
    pub AddComponentDependency: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, super::vss::VSS_ID, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    AddComponentDependency: usize,
    pub SetBackupSchema: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SaveAsXML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_vss")]
pub trait IVssCreateExpressWriterMetadata_Impl: windows_core::IUnknownImpl {
    fn AddExcludeFiles(&self, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, brecursive: bool) -> windows_core::Result<()>;
    fn AddComponent(&self, ct: VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, wszcaption: &windows_core::PCWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: bool, bnotifyonbackupcomplete: bool, bselectable: bool, bselectableforrestore: bool, dwcomponentflags: u32) -> windows_core::Result<()>;
    fn AddFilesToFileGroup(&self, wszlogicalpath: &windows_core::PCWSTR, wszgroupname: &windows_core::PCWSTR, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, brecursive: bool, wszalternatelocation: &windows_core::PCWSTR, dwbackuptypemask: u32) -> windows_core::Result<()>;
    fn SetRestoreMethod(&self, method: VSS_RESTOREMETHOD_ENUM, wszservice: &windows_core::PCWSTR, wszuserprocedure: &windows_core::PCWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: bool) -> windows_core::Result<()>;
    fn AddComponentDependency(&self, wszforlogicalpath: &windows_core::PCWSTR, wszforcomponentname: &windows_core::PCWSTR, onwriterid: &super::vss::VSS_ID, wszonlogicalpath: &windows_core::PCWSTR, wszoncomponentname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetBackupSchema(&self, dwschemamask: u32) -> windows_core::Result<()>;
    fn SaveAsXML(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_vss")]
impl IVssCreateExpressWriterMetadata_Vtbl {
    pub const fn new<Identity: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddExcludeFiles<Identity: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssCreateExpressWriterMetadata_Impl::AddExcludeFiles(this, core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive)).into()
            }
        }
        unsafe extern "system" fn AddComponent<Identity: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ct: VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszcaption: windows_core::PCWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: bool, bnotifyonbackupcomplete: bool, bselectable: bool, bselectableforrestore: bool, dwcomponentflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssCreateExpressWriterMetadata_Impl::AddComponent(this, core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute(&wszcaption), core::mem::transmute_copy(&pbicon), core::mem::transmute_copy(&cbicon), core::mem::transmute_copy(&brestoremetadata), core::mem::transmute_copy(&bnotifyonbackupcomplete), core::mem::transmute_copy(&bselectable), core::mem::transmute_copy(&bselectableforrestore), core::mem::transmute_copy(&dwcomponentflags)).into()
            }
        }
        unsafe extern "system" fn AddFilesToFileGroup<Identity: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszlogicalpath: windows_core::PCWSTR, wszgroupname: windows_core::PCWSTR, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: bool, wszalternatelocation: windows_core::PCWSTR, dwbackuptypemask: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssCreateExpressWriterMetadata_Impl::AddFilesToFileGroup(this, core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszgroupname), core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive), core::mem::transmute(&wszalternatelocation), core::mem::transmute_copy(&dwbackuptypemask)).into()
            }
        }
        unsafe extern "system" fn SetRestoreMethod<Identity: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, method: VSS_RESTOREMETHOD_ENUM, wszservice: windows_core::PCWSTR, wszuserprocedure: windows_core::PCWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssCreateExpressWriterMetadata_Impl::SetRestoreMethod(this, core::mem::transmute_copy(&method), core::mem::transmute(&wszservice), core::mem::transmute(&wszuserprocedure), core::mem::transmute_copy(&writerrestore), core::mem::transmute_copy(&brebootrequired)).into()
            }
        }
        unsafe extern "system" fn AddComponentDependency<Identity: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszforlogicalpath: windows_core::PCWSTR, wszforcomponentname: windows_core::PCWSTR, onwriterid: super::vss::VSS_ID, wszonlogicalpath: windows_core::PCWSTR, wszoncomponentname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssCreateExpressWriterMetadata_Impl::AddComponentDependency(this, core::mem::transmute(&wszforlogicalpath), core::mem::transmute(&wszforcomponentname), core::mem::transmute(&onwriterid), core::mem::transmute(&wszonlogicalpath), core::mem::transmute(&wszoncomponentname)).into()
            }
        }
        unsafe extern "system" fn SetBackupSchema<Identity: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwschemamask: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssCreateExpressWriterMetadata_Impl::SetBackupSchema(this, core::mem::transmute_copy(&dwschemamask)).into()
            }
        }
        unsafe extern "system" fn SaveAsXML<Identity: IVssCreateExpressWriterMetadata_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrxml: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssCreateExpressWriterMetadata_Impl::SaveAsXML(this) {
                    Ok(ok__) => {
                        pbstrxml.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddExcludeFiles: AddExcludeFiles::<Identity, OFFSET>,
            AddComponent: AddComponent::<Identity, OFFSET>,
            AddFilesToFileGroup: AddFilesToFileGroup::<Identity, OFFSET>,
            SetRestoreMethod: SetRestoreMethod::<Identity, OFFSET>,
            AddComponentDependency: AddComponentDependency::<Identity, OFFSET>,
            SetBackupSchema: SetBackupSchema::<Identity, OFFSET>,
            SaveAsXML: SaveAsXML::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssCreateExpressWriterMetadata as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_vss")]
impl windows_core::RuntimeName for IVssCreateExpressWriterMetadata {}
windows_core::imp::define_interface!(IVssCreateWriterMetadata, IVssCreateWriterMetadata_Vtbl);
impl IVssCreateWriterMetadata {
    pub unsafe fn AddIncludeFiles<P0, P1, P3>(&self, wszpath: P0, wszfilespec: P1, brecursive: bool, wszalternatelocation: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddIncludeFiles)(windows_core::Interface::as_raw(self), wszpath.param().abi(), wszfilespec.param().abi(), brecursive, wszalternatelocation.param().abi()) }
    }
    pub unsafe fn AddExcludeFiles<P0, P1>(&self, wszpath: P0, wszfilespec: P1, brecursive: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddExcludeFiles)(windows_core::Interface::as_raw(self), wszpath.param().abi(), wszfilespec.param().abi(), brecursive) }
    }
    pub unsafe fn AddComponent<P1, P2, P3>(&self, ct: VSS_COMPONENT_TYPE, wszlogicalpath: P1, wszcomponentname: P2, wszcaption: P3, pbicon: *const u8, cbicon: u32, brestoremetadata: bool, bnotifyonbackupcomplete: bool, bselectable: bool, bselectableforrestore: bool, dwcomponentflags: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddComponent)(windows_core::Interface::as_raw(self), ct, wszlogicalpath.param().abi(), wszcomponentname.param().abi(), wszcaption.param().abi(), pbicon, cbicon, brestoremetadata, bnotifyonbackupcomplete, bselectable, bselectableforrestore, dwcomponentflags) }
    }
    pub unsafe fn AddDatabaseFiles<P0, P1, P2, P3>(&self, wszlogicalpath: P0, wszdatabasename: P1, wszpath: P2, wszfilespec: P3, dwbackuptypemask: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddDatabaseFiles)(windows_core::Interface::as_raw(self), wszlogicalpath.param().abi(), wszdatabasename.param().abi(), wszpath.param().abi(), wszfilespec.param().abi(), dwbackuptypemask) }
    }
    pub unsafe fn AddDatabaseLogFiles<P0, P1, P2, P3>(&self, wszlogicalpath: P0, wszdatabasename: P1, wszpath: P2, wszfilespec: P3, dwbackuptypemask: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddDatabaseLogFiles)(windows_core::Interface::as_raw(self), wszlogicalpath.param().abi(), wszdatabasename.param().abi(), wszpath.param().abi(), wszfilespec.param().abi(), dwbackuptypemask) }
    }
    pub unsafe fn AddFilesToFileGroup<P0, P1, P2, P3, P5>(&self, wszlogicalpath: P0, wszgroupname: P1, wszpath: P2, wszfilespec: P3, brecursive: bool, wszalternatelocation: P5, dwbackuptypemask: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P5: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddFilesToFileGroup)(windows_core::Interface::as_raw(self), wszlogicalpath.param().abi(), wszgroupname.param().abi(), wszpath.param().abi(), wszfilespec.param().abi(), brecursive, wszalternatelocation.param().abi(), dwbackuptypemask) }
    }
    pub unsafe fn SetRestoreMethod<P1, P2>(&self, method: VSS_RESTOREMETHOD_ENUM, wszservice: P1, wszuserprocedure: P2, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: bool) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRestoreMethod)(windows_core::Interface::as_raw(self), method, wszservice.param().abi(), wszuserprocedure.param().abi(), writerrestore, brebootrequired) }
    }
    pub unsafe fn AddAlternateLocationMapping<P0, P1, P3>(&self, wszsourcepath: P0, wszsourcefilespec: P1, brecursive: bool, wszdestination: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddAlternateLocationMapping)(windows_core::Interface::as_raw(self), wszsourcepath.param().abi(), wszsourcefilespec.param().abi(), brecursive, wszdestination.param().abi()) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn AddComponentDependency<P0, P1, P3, P4>(&self, wszforlogicalpath: P0, wszforcomponentname: P1, onwriterid: super::vss::VSS_ID, wszonlogicalpath: P3, wszoncomponentname: P4) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
        P3: windows_core::Param<windows_core::PCWSTR>,
        P4: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddComponentDependency)(windows_core::Interface::as_raw(self), wszforlogicalpath.param().abi(), wszforcomponentname.param().abi(), core::mem::transmute(onwriterid), wszonlogicalpath.param().abi(), wszoncomponentname.param().abi()) }
    }
    pub unsafe fn SetBackupSchema(&self, dwschemamask: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetBackupSchema)(windows_core::Interface::as_raw(self), dwschemamask) }
    }
    #[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl"))]
    pub unsafe fn GetDocument(&self) -> windows_core::Result<super::msxml::IXMLDOMDocument> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDocument)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SaveAsXML(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SaveAsXML)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssCreateWriterMetadata_Vtbl {
    pub AddIncludeFiles: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, bool, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub AddExcludeFiles: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, bool) -> windows_core::HRESULT,
    pub AddComponent: unsafe extern "system" fn(*mut core::ffi::c_void, VSS_COMPONENT_TYPE, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, *const u8, u32, bool, bool, bool, bool, u32) -> windows_core::HRESULT,
    pub AddDatabaseFiles: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub AddDatabaseLogFiles: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub AddFilesToFileGroup: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, windows_core::PCWSTR, bool, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub SetRestoreMethod: unsafe extern "system" fn(*mut core::ffi::c_void, VSS_RESTOREMETHOD_ENUM, windows_core::PCWSTR, windows_core::PCWSTR, VSS_WRITERRESTORE_ENUM, bool) -> windows_core::HRESULT,
    pub AddAlternateLocationMapping: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, bool, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_vss")]
    pub AddComponentDependency: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, super::vss::VSS_ID, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    AddComponentDependency: usize,
    pub SetBackupSchema: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    #[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl"))]
    pub GetDocument: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "Win32_msxml", feature = "Win32_oaidl")))]
    GetDocument: usize,
    pub SaveAsXML: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_vss"))]
pub trait IVssCreateWriterMetadata_Impl {
    fn AddIncludeFiles(&self, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, brecursive: bool, wszalternatelocation: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddExcludeFiles(&self, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, brecursive: bool) -> windows_core::Result<()>;
    fn AddComponent(&self, ct: VSS_COMPONENT_TYPE, wszlogicalpath: &windows_core::PCWSTR, wszcomponentname: &windows_core::PCWSTR, wszcaption: &windows_core::PCWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: bool, bnotifyonbackupcomplete: bool, bselectable: bool, bselectableforrestore: bool, dwcomponentflags: u32) -> windows_core::Result<()>;
    fn AddDatabaseFiles(&self, wszlogicalpath: &windows_core::PCWSTR, wszdatabasename: &windows_core::PCWSTR, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, dwbackuptypemask: u32) -> windows_core::Result<()>;
    fn AddDatabaseLogFiles(&self, wszlogicalpath: &windows_core::PCWSTR, wszdatabasename: &windows_core::PCWSTR, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, dwbackuptypemask: u32) -> windows_core::Result<()>;
    fn AddFilesToFileGroup(&self, wszlogicalpath: &windows_core::PCWSTR, wszgroupname: &windows_core::PCWSTR, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, brecursive: bool, wszalternatelocation: &windows_core::PCWSTR, dwbackuptypemask: u32) -> windows_core::Result<()>;
    fn SetRestoreMethod(&self, method: VSS_RESTOREMETHOD_ENUM, wszservice: &windows_core::PCWSTR, wszuserprocedure: &windows_core::PCWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: bool) -> windows_core::Result<()>;
    fn AddAlternateLocationMapping(&self, wszsourcepath: &windows_core::PCWSTR, wszsourcefilespec: &windows_core::PCWSTR, brecursive: bool, wszdestination: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn AddComponentDependency(&self, wszforlogicalpath: &windows_core::PCWSTR, wszforcomponentname: &windows_core::PCWSTR, onwriterid: &super::vss::VSS_ID, wszonlogicalpath: &windows_core::PCWSTR, wszoncomponentname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetBackupSchema(&self, dwschemamask: u32) -> windows_core::Result<()>;
    fn GetDocument(&self) -> windows_core::Result<super::msxml::IXMLDOMDocument>;
    fn SaveAsXML(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_vss"))]
impl IVssCreateWriterMetadata_Vtbl {
    pub const fn new<Identity: IVssCreateWriterMetadata_Impl>() -> Self {
        unsafe extern "system" fn AddIncludeFiles<Identity: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: bool, wszalternatelocation: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IVssCreateWriterMetadata_Impl::AddIncludeFiles(this, core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive), core::mem::transmute(&wszalternatelocation)).into()
            }
        }
        unsafe extern "system" fn AddExcludeFiles<Identity: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: bool) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IVssCreateWriterMetadata_Impl::AddExcludeFiles(this, core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive)).into()
            }
        }
        unsafe extern "system" fn AddComponent<Identity: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, ct: VSS_COMPONENT_TYPE, wszlogicalpath: windows_core::PCWSTR, wszcomponentname: windows_core::PCWSTR, wszcaption: windows_core::PCWSTR, pbicon: *const u8, cbicon: u32, brestoremetadata: bool, bnotifyonbackupcomplete: bool, bselectable: bool, bselectableforrestore: bool, dwcomponentflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IVssCreateWriterMetadata_Impl::AddComponent(this, core::mem::transmute_copy(&ct), core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszcomponentname), core::mem::transmute(&wszcaption), core::mem::transmute_copy(&pbicon), core::mem::transmute_copy(&cbicon), core::mem::transmute_copy(&brestoremetadata), core::mem::transmute_copy(&bnotifyonbackupcomplete), core::mem::transmute_copy(&bselectable), core::mem::transmute_copy(&bselectableforrestore), core::mem::transmute_copy(&dwcomponentflags)).into()
            }
        }
        unsafe extern "system" fn AddDatabaseFiles<Identity: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, wszlogicalpath: windows_core::PCWSTR, wszdatabasename: windows_core::PCWSTR, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, dwbackuptypemask: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IVssCreateWriterMetadata_Impl::AddDatabaseFiles(this, core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszdatabasename), core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&dwbackuptypemask)).into()
            }
        }
        unsafe extern "system" fn AddDatabaseLogFiles<Identity: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, wszlogicalpath: windows_core::PCWSTR, wszdatabasename: windows_core::PCWSTR, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, dwbackuptypemask: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IVssCreateWriterMetadata_Impl::AddDatabaseLogFiles(this, core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszdatabasename), core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&dwbackuptypemask)).into()
            }
        }
        unsafe extern "system" fn AddFilesToFileGroup<Identity: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, wszlogicalpath: windows_core::PCWSTR, wszgroupname: windows_core::PCWSTR, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: bool, wszalternatelocation: windows_core::PCWSTR, dwbackuptypemask: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IVssCreateWriterMetadata_Impl::AddFilesToFileGroup(this, core::mem::transmute(&wszlogicalpath), core::mem::transmute(&wszgroupname), core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive), core::mem::transmute(&wszalternatelocation), core::mem::transmute_copy(&dwbackuptypemask)).into()
            }
        }
        unsafe extern "system" fn SetRestoreMethod<Identity: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, method: VSS_RESTOREMETHOD_ENUM, wszservice: windows_core::PCWSTR, wszuserprocedure: windows_core::PCWSTR, writerrestore: VSS_WRITERRESTORE_ENUM, brebootrequired: bool) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IVssCreateWriterMetadata_Impl::SetRestoreMethod(this, core::mem::transmute_copy(&method), core::mem::transmute(&wszservice), core::mem::transmute(&wszuserprocedure), core::mem::transmute_copy(&writerrestore), core::mem::transmute_copy(&brebootrequired)).into()
            }
        }
        unsafe extern "system" fn AddAlternateLocationMapping<Identity: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, wszsourcepath: windows_core::PCWSTR, wszsourcefilespec: windows_core::PCWSTR, brecursive: bool, wszdestination: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IVssCreateWriterMetadata_Impl::AddAlternateLocationMapping(this, core::mem::transmute(&wszsourcepath), core::mem::transmute(&wszsourcefilespec), core::mem::transmute_copy(&brecursive), core::mem::transmute(&wszdestination)).into()
            }
        }
        unsafe extern "system" fn AddComponentDependency<Identity: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, wszforlogicalpath: windows_core::PCWSTR, wszforcomponentname: windows_core::PCWSTR, onwriterid: super::vss::VSS_ID, wszonlogicalpath: windows_core::PCWSTR, wszoncomponentname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IVssCreateWriterMetadata_Impl::AddComponentDependency(this, core::mem::transmute(&wszforlogicalpath), core::mem::transmute(&wszforcomponentname), core::mem::transmute(&onwriterid), core::mem::transmute(&wszonlogicalpath), core::mem::transmute(&wszoncomponentname)).into()
            }
        }
        unsafe extern "system" fn SetBackupSchema<Identity: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, dwschemamask: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IVssCreateWriterMetadata_Impl::SetBackupSchema(this, core::mem::transmute_copy(&dwschemamask)).into()
            }
        }
        unsafe extern "system" fn GetDocument<Identity: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, pdoc: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match IVssCreateWriterMetadata_Impl::GetDocument(this) {
                    Ok(ok__) => {
                        pdoc.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SaveAsXML<Identity: IVssCreateWriterMetadata_Impl>(this: *mut core::ffi::c_void, pbstrxml: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match IVssCreateWriterMetadata_Impl::SaveAsXML(this) {
                    Ok(ok__) => {
                        pbstrxml.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            AddIncludeFiles: AddIncludeFiles::<Identity>,
            AddExcludeFiles: AddExcludeFiles::<Identity>,
            AddComponent: AddComponent::<Identity>,
            AddDatabaseFiles: AddDatabaseFiles::<Identity>,
            AddDatabaseLogFiles: AddDatabaseLogFiles::<Identity>,
            AddFilesToFileGroup: AddFilesToFileGroup::<Identity>,
            SetRestoreMethod: SetRestoreMethod::<Identity>,
            AddAlternateLocationMapping: AddAlternateLocationMapping::<Identity>,
            AddComponentDependency: AddComponentDependency::<Identity>,
            SetBackupSchema: SetBackupSchema::<Identity>,
            GetDocument: GetDocument::<Identity>,
            SaveAsXML: SaveAsXML::<Identity>,
        }
    }
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_vss"))]
struct IVssCreateWriterMetadata_ImplVtbl<T: IVssCreateWriterMetadata_Impl>(core::marker::PhantomData<T>);
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_vss"))]
impl<T: IVssCreateWriterMetadata_Impl> IVssCreateWriterMetadata_ImplVtbl<T> {
    const VTABLE: IVssCreateWriterMetadata_Vtbl = IVssCreateWriterMetadata_Vtbl::new::<T>();
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_vss"))]
impl IVssCreateWriterMetadata {
    pub fn new<'a, T: IVssCreateWriterMetadata_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IVssCreateWriterMetadata_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(IVssCreateWriterMetadataEx, IVssCreateWriterMetadataEx_Vtbl);
impl core::ops::Deref for IVssCreateWriterMetadataEx {
    type Target = IVssCreateWriterMetadata;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVssCreateWriterMetadataEx, IVssCreateWriterMetadata);
impl IVssCreateWriterMetadataEx {
    pub unsafe fn AddExcludeFilesFromSnapshot<P0, P1>(&self, wszpath: P0, wszfilespec: P1, brecursive: bool) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddExcludeFilesFromSnapshot)(windows_core::Interface::as_raw(self), wszpath.param().abi(), wszfilespec.param().abi(), brecursive) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssCreateWriterMetadataEx_Vtbl {
    pub base__: IVssCreateWriterMetadata_Vtbl,
    pub AddExcludeFilesFromSnapshot: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, bool) -> windows_core::HRESULT,
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_vss"))]
pub trait IVssCreateWriterMetadataEx_Impl: IVssCreateWriterMetadata_Impl {
    fn AddExcludeFilesFromSnapshot(&self, wszpath: &windows_core::PCWSTR, wszfilespec: &windows_core::PCWSTR, brecursive: bool) -> windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_vss"))]
impl IVssCreateWriterMetadataEx_Vtbl {
    pub const fn new<Identity: IVssCreateWriterMetadataEx_Impl>() -> Self {
        unsafe extern "system" fn AddExcludeFilesFromSnapshot<Identity: IVssCreateWriterMetadataEx_Impl>(this: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR, wszfilespec: windows_core::PCWSTR, brecursive: bool) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IVssCreateWriterMetadataEx_Impl::AddExcludeFilesFromSnapshot(this, core::mem::transmute(&wszpath), core::mem::transmute(&wszfilespec), core::mem::transmute_copy(&brecursive)).into()
            }
        }
        Self { base__: IVssCreateWriterMetadata_Vtbl::new::<Identity>(), AddExcludeFilesFromSnapshot: AddExcludeFilesFromSnapshot::<Identity> }
    }
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_vss"))]
struct IVssCreateWriterMetadataEx_ImplVtbl<T: IVssCreateWriterMetadataEx_Impl>(core::marker::PhantomData<T>);
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_vss"))]
impl<T: IVssCreateWriterMetadataEx_Impl> IVssCreateWriterMetadataEx_ImplVtbl<T> {
    const VTABLE: IVssCreateWriterMetadataEx_Vtbl = IVssCreateWriterMetadataEx_Vtbl::new::<T>();
}
#[cfg(all(feature = "Win32_msxml", feature = "Win32_oaidl", feature = "Win32_vss"))]
impl IVssCreateWriterMetadataEx {
    pub fn new<'a, T: IVssCreateWriterMetadataEx_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IVssCreateWriterMetadataEx_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(IVssExpressWriter, IVssExpressWriter_Vtbl, 0xe33affdc_59c7_47b1_97d5_4266598f6235);
windows_core::imp::interface_hierarchy!(IVssExpressWriter, windows_core::IUnknown);
impl IVssExpressWriter {
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn CreateMetadata<P1>(&self, writerid: super::vss::VSS_ID, writername: P1, usagetype: VSS_USAGE_TYPE, versionmajor: u32, versionminor: u32, reserved: u32) -> windows_core::Result<IVssCreateExpressWriterMetadata>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateMetadata)(windows_core::Interface::as_raw(self), core::mem::transmute(writerid), writername.param().abi(), usagetype, versionmajor, versionminor, reserved, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn LoadMetadata<P0>(&self, metadata: P0, reserved: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).LoadMetadata)(windows_core::Interface::as_raw(self), metadata.param().abi(), reserved) }
    }
    pub unsafe fn Register(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Register)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn Unregister(&self, writerid: super::vss::VSS_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unregister)(windows_core::Interface::as_raw(self), core::mem::transmute(writerid)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssExpressWriter_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_vss")]
    pub CreateMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, windows_core::PCWSTR, VSS_USAGE_TYPE, u32, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    CreateMetadata: usize,
    pub LoadMetadata: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32) -> windows_core::HRESULT,
    pub Register: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_vss")]
    pub Unregister: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    Unregister: usize,
}
#[cfg(feature = "Win32_vss")]
pub trait IVssExpressWriter_Impl: windows_core::IUnknownImpl {
    fn CreateMetadata(&self, writerid: &super::vss::VSS_ID, writername: &windows_core::PCWSTR, usagetype: VSS_USAGE_TYPE, versionmajor: u32, versionminor: u32, reserved: u32) -> windows_core::Result<IVssCreateExpressWriterMetadata>;
    fn LoadMetadata(&self, metadata: &windows_core::PCWSTR, reserved: u32) -> windows_core::Result<()>;
    fn Register(&self) -> windows_core::Result<()>;
    fn Unregister(&self, writerid: &super::vss::VSS_ID) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_vss")]
impl IVssExpressWriter_Vtbl {
    pub const fn new<Identity: IVssExpressWriter_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateMetadata<Identity: IVssExpressWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::vss::VSS_ID, writername: windows_core::PCWSTR, usagetype: VSS_USAGE_TYPE, versionmajor: u32, versionminor: u32, reserved: u32, ppmetadata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssExpressWriter_Impl::CreateMetadata(this, core::mem::transmute(&writerid), core::mem::transmute(&writername), core::mem::transmute_copy(&usagetype), core::mem::transmute_copy(&versionmajor), core::mem::transmute_copy(&versionminor), core::mem::transmute_copy(&reserved)) {
                    Ok(ok__) => {
                        ppmetadata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LoadMetadata<Identity: IVssExpressWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, metadata: windows_core::PCWSTR, reserved: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssExpressWriter_Impl::LoadMetadata(this, core::mem::transmute(&metadata), core::mem::transmute_copy(&reserved)).into()
            }
        }
        unsafe extern "system" fn Register<Identity: IVssExpressWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssExpressWriter_Impl::Register(this).into()
            }
        }
        unsafe extern "system" fn Unregister<Identity: IVssExpressWriter_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::vss::VSS_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssExpressWriter_Impl::Unregister(this, core::mem::transmute(&writerid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateMetadata: CreateMetadata::<Identity, OFFSET>,
            LoadMetadata: LoadMetadata::<Identity, OFFSET>,
            Register: Register::<Identity, OFFSET>,
            Unregister: Unregister::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssExpressWriter as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_vss")]
impl windows_core::RuntimeName for IVssExpressWriter {}
windows_core::imp::define_interface!(IVssWMDependency, IVssWMDependency_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IVssWMDependency, windows_core::IUnknown);
impl IVssWMDependency {
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn GetWriterId(&self) -> windows_core::Result<super::vss::VSS_ID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWriterId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetLogicalPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLogicalPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetComponentName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetComponentName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssWMDependency_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_vss")]
    pub GetWriterId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::vss::VSS_ID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    GetWriterId: usize,
    pub GetLogicalPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetComponentName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_vss")]
pub trait IVssWMDependency_Impl: windows_core::IUnknownImpl {
    fn GetWriterId(&self) -> windows_core::Result<super::vss::VSS_ID>;
    fn GetLogicalPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetComponentName(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "Win32_vss")]
impl IVssWMDependency_Vtbl {
    pub const fn new<Identity: IVssWMDependency_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetWriterId<Identity: IVssWMDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pwriterid: *mut super::vss::VSS_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssWMDependency_Impl::GetWriterId(this) {
                    Ok(ok__) => {
                        pwriterid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLogicalPath<Identity: IVssWMDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrlogicalpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssWMDependency_Impl::GetLogicalPath(this) {
                    Ok(ok__) => {
                        pbstrlogicalpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetComponentName<Identity: IVssWMDependency_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrcomponentname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssWMDependency_Impl::GetComponentName(this) {
                    Ok(ok__) => {
                        pbstrcomponentname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetWriterId: GetWriterId::<Identity, OFFSET>,
            GetLogicalPath: GetLogicalPath::<Identity, OFFSET>,
            GetComponentName: GetComponentName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssWMDependency as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_vss")]
impl windows_core::RuntimeName for IVssWMDependency {}
windows_core::imp::define_interface!(IVssWMFiledesc, IVssWMFiledesc_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IVssWMFiledesc, windows_core::IUnknown);
impl IVssWMFiledesc {
    pub unsafe fn GetPath(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPath)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetFilespec(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFilespec)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetRecursive(&self) -> windows_core::Result<bool> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRecursive)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetAlternateLocation(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAlternateLocation)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetBackupTypeMask(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBackupTypeMask)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssWMFiledesc_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetPath: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFilespec: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRecursive: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub GetAlternateLocation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetBackupTypeMask: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
pub trait IVssWMFiledesc_Impl: windows_core::IUnknownImpl {
    fn GetPath(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetFilespec(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetRecursive(&self) -> windows_core::Result<bool>;
    fn GetAlternateLocation(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetBackupTypeMask(&self) -> windows_core::Result<u32>;
}
impl IVssWMFiledesc_Vtbl {
    pub const fn new<Identity: IVssWMFiledesc_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPath<Identity: IVssWMFiledesc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrpath: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssWMFiledesc_Impl::GetPath(this) {
                    Ok(ok__) => {
                        pbstrpath.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFilespec<Identity: IVssWMFiledesc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstrfilespec: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssWMFiledesc_Impl::GetFilespec(this) {
                    Ok(ok__) => {
                        pbstrfilespec.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRecursive<Identity: IVssWMFiledesc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbrecursive: *mut bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssWMFiledesc_Impl::GetRecursive(this) {
                    Ok(ok__) => {
                        pbrecursive.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAlternateLocation<Identity: IVssWMFiledesc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbstralternatelocation: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssWMFiledesc_Impl::GetAlternateLocation(this) {
                    Ok(ok__) => {
                        pbstralternatelocation.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBackupTypeMask<Identity: IVssWMFiledesc_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwtypemask: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssWMFiledesc_Impl::GetBackupTypeMask(this) {
                    Ok(ok__) => {
                        pdwtypemask.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetPath: GetPath::<Identity, OFFSET>,
            GetFilespec: GetFilespec::<Identity, OFFSET>,
            GetRecursive: GetRecursive::<Identity, OFFSET>,
            GetAlternateLocation: GetAlternateLocation::<Identity, OFFSET>,
            GetBackupTypeMask: GetBackupTypeMask::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssWMFiledesc as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVssWMFiledesc {}
windows_core::imp::define_interface!(IVssWriterComponents, IVssWriterComponents_Vtbl);
impl IVssWriterComponents {
    pub unsafe fn GetComponentCount(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetComponentCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn GetWriterInfo(&self, pidinstance: *mut super::vss::VSS_ID, pidwriter: *mut super::vss::VSS_ID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetWriterInfo)(windows_core::Interface::as_raw(self), pidinstance as _, pidwriter as _) }
    }
    pub unsafe fn GetComponent(&self, icomponent: u32) -> windows_core::Result<IVssComponent> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetComponent)(windows_core::Interface::as_raw(self), icomponent, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssWriterComponents_Vtbl {
    pub GetComponentCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_vss")]
    pub GetWriterInfo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::vss::VSS_ID, *mut super::vss::VSS_ID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    GetWriterInfo: usize,
    pub GetComponent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_vss")]
pub trait IVssWriterComponents_Impl {
    fn GetComponentCount(&self) -> windows_core::Result<u32>;
    fn GetWriterInfo(&self, pidinstance: *mut super::vss::VSS_ID, pidwriter: *mut super::vss::VSS_ID) -> windows_core::Result<()>;
    fn GetComponent(&self, icomponent: u32) -> windows_core::Result<IVssComponent>;
}
#[cfg(feature = "Win32_vss")]
impl IVssWriterComponents_Vtbl {
    pub const fn new<Identity: IVssWriterComponents_Impl>() -> Self {
        unsafe extern "system" fn GetComponentCount<Identity: IVssWriterComponents_Impl>(this: *mut core::ffi::c_void, pccomponents: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match IVssWriterComponents_Impl::GetComponentCount(this) {
                    Ok(ok__) => {
                        pccomponents.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetWriterInfo<Identity: IVssWriterComponents_Impl>(this: *mut core::ffi::c_void, pidinstance: *mut super::vss::VSS_ID, pidwriter: *mut super::vss::VSS_ID) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IVssWriterComponents_Impl::GetWriterInfo(this, core::mem::transmute_copy(&pidinstance), core::mem::transmute_copy(&pidwriter)).into()
            }
        }
        unsafe extern "system" fn GetComponent<Identity: IVssWriterComponents_Impl>(this: *mut core::ffi::c_void, icomponent: u32, ppcomponent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                match IVssWriterComponents_Impl::GetComponent(this, core::mem::transmute_copy(&icomponent)) {
                    Ok(ok__) => {
                        ppcomponent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { GetComponentCount: GetComponentCount::<Identity>, GetWriterInfo: GetWriterInfo::<Identity>, GetComponent: GetComponent::<Identity> }
    }
}
#[cfg(feature = "Win32_vss")]
struct IVssWriterComponents_ImplVtbl<T: IVssWriterComponents_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "Win32_vss")]
impl<T: IVssWriterComponents_Impl> IVssWriterComponents_ImplVtbl<T> {
    const VTABLE: IVssWriterComponents_Vtbl = IVssWriterComponents_Vtbl::new::<T>();
}
#[cfg(feature = "Win32_vss")]
impl IVssWriterComponents {
    pub fn new<'a, T: IVssWriterComponents_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IVssWriterComponents_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(IVssWriterImpl, IVssWriterImpl_Vtbl, 0);
windows_core::imp::interface_hierarchy!(IVssWriterImpl, windows_core::IUnknown);
impl IVssWriterImpl {
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn Initialize<P1, P2>(&self, writerid: super::vss::VSS_ID, wszwritername: P1, wszwriterinstancename: P2, dwmajorversion: u32, dwminorversion: u32, ut: VSS_USAGE_TYPE, st: VSS_SOURCE_TYPE, nlevel: super::vss::VSS_APPLICATION_LEVEL, dwtimeout: u32, aws: VSS_ALTERNATE_WRITER_STATE, biothrottlingonly: bool) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).Initialize)(windows_core::Interface::as_raw(self), core::mem::transmute(writerid), wszwritername.param().abi(), wszwriterinstancename.param().abi(), dwmajorversion, dwminorversion, ut, st, nlevel, dwtimeout, aws, biothrottlingonly) }
    }
    pub unsafe fn Subscribe(&self, dwsubscribetimeout: u32, dweventflags: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Subscribe)(windows_core::Interface::as_raw(self), dwsubscribetimeout, dweventflags) }
    }
    pub unsafe fn Unsubscribe(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Unsubscribe)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Uninitialize(&self) {
        unsafe {
            (windows_core::Interface::vtable(self).Uninitialize)(windows_core::Interface::as_raw(self));
        }
    }
    pub unsafe fn GetCurrentVolumeArray(&self) -> *mut windows_core::PCWSTR {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentVolumeArray)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetCurrentVolumeCount(&self) -> u32 {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentVolumeCount)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn GetSnapshotDeviceName<P0>(&self, wszoriginalvolume: P0) -> windows_core::Result<windows_core::PCWSTR>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSnapshotDeviceName)(windows_core::Interface::as_raw(self), wszoriginalvolume.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn GetCurrentSnapshotSetId(&self) -> super::vss::VSS_ID {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentSnapshotSetId)(windows_core::Interface::as_raw(self), &mut result__);
            result__
        }
    }
    pub unsafe fn GetContext(&self) -> i32 {
        unsafe { (windows_core::Interface::vtable(self).GetContext)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn GetCurrentLevel(&self) -> super::vss::VSS_APPLICATION_LEVEL {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentLevel)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsPathAffected<P0>(&self, wszpath: P0) -> bool
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).IsPathAffected)(windows_core::Interface::as_raw(self), wszpath.param().abi()) }
    }
    pub unsafe fn IsBootableSystemStateBackedUp(&self) -> bool {
        unsafe { (windows_core::Interface::vtable(self).IsBootableSystemStateBackedUp)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AreComponentsSelected(&self) -> bool {
        unsafe { (windows_core::Interface::vtable(self).AreComponentsSelected)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn GetBackupType(&self) -> super::vss::VSS_BACKUP_TYPE {
        unsafe { (windows_core::Interface::vtable(self).GetBackupType)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn GetRestoreType(&self) -> super::vss::VSS_RESTORE_TYPE {
        unsafe { (windows_core::Interface::vtable(self).GetRestoreType)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetWriterFailure(&self, hr: windows_core::HRESULT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWriterFailure)(windows_core::Interface::as_raw(self), hr) }
    }
    pub unsafe fn IsPartialFileSupportEnabled(&self) -> bool {
        unsafe { (windows_core::Interface::vtable(self).IsPartialFileSupportEnabled)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn InstallAlternateWriter(&self, idwriter: super::vss::VSS_ID, clsid: windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InstallAlternateWriter)(windows_core::Interface::as_raw(self), core::mem::transmute(idwriter), core::mem::transmute(clsid)) }
    }
    #[cfg(feature = "Win32_vsbackup")]
    pub unsafe fn GetIdentityInformation(&self) -> Option<super::vsbackup::IVssExamineWriterMetadata> {
        unsafe { (windows_core::Interface::vtable(self).GetIdentityInformation)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetWriterFailureEx<P2>(&self, hr: windows_core::HRESULT, hrapplication: windows_core::HRESULT, wszapplicationmessage: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetWriterFailureEx)(windows_core::Interface::as_raw(self), hr, hrapplication, wszapplicationmessage.param().abi()) }
    }
    #[cfg(feature = "Win32_vss")]
    pub unsafe fn GetSessionId(&self) -> windows_core::Result<super::vss::VSS_ID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSessionId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsWriterShuttingDown(&self) -> bool {
        unsafe { (windows_core::Interface::vtable(self).IsWriterShuttingDown)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVssWriterImpl_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_vss")]
    pub Initialize: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, windows_core::PCWSTR, windows_core::PCWSTR, u32, u32, VSS_USAGE_TYPE, VSS_SOURCE_TYPE, super::vss::VSS_APPLICATION_LEVEL, u32, VSS_ALTERNATE_WRITER_STATE, bool) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    Initialize: usize,
    pub Subscribe: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32) -> windows_core::HRESULT,
    pub Unsubscribe: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Uninitialize: unsafe extern "system" fn(*mut core::ffi::c_void),
    pub GetCurrentVolumeArray: unsafe extern "system" fn(*mut core::ffi::c_void) -> *mut windows_core::PCWSTR,
    pub GetCurrentVolumeCount: unsafe extern "system" fn(*mut core::ffi::c_void) -> u32,
    pub GetSnapshotDeviceName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_vss")]
    pub GetCurrentSnapshotSetId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::vss::VSS_ID),
    #[cfg(not(feature = "Win32_vss"))]
    GetCurrentSnapshotSetId: usize,
    pub GetContext: unsafe extern "system" fn(*mut core::ffi::c_void) -> i32,
    #[cfg(feature = "Win32_vss")]
    pub GetCurrentLevel: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::vss::VSS_APPLICATION_LEVEL,
    #[cfg(not(feature = "Win32_vss"))]
    GetCurrentLevel: usize,
    pub IsPathAffected: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> bool,
    pub IsBootableSystemStateBackedUp: unsafe extern "system" fn(*mut core::ffi::c_void) -> bool,
    pub AreComponentsSelected: unsafe extern "system" fn(*mut core::ffi::c_void) -> bool,
    #[cfg(feature = "Win32_vss")]
    pub GetBackupType: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::vss::VSS_BACKUP_TYPE,
    #[cfg(not(feature = "Win32_vss"))]
    GetBackupType: usize,
    #[cfg(feature = "Win32_vss")]
    pub GetRestoreType: unsafe extern "system" fn(*mut core::ffi::c_void) -> super::vss::VSS_RESTORE_TYPE,
    #[cfg(not(feature = "Win32_vss"))]
    GetRestoreType: usize,
    pub SetWriterFailure: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT) -> windows_core::HRESULT,
    pub IsPartialFileSupportEnabled: unsafe extern "system" fn(*mut core::ffi::c_void) -> bool,
    #[cfg(feature = "Win32_vss")]
    pub InstallAlternateWriter: unsafe extern "system" fn(*mut core::ffi::c_void, super::vss::VSS_ID, windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    InstallAlternateWriter: usize,
    #[cfg(feature = "Win32_vsbackup")]
    pub GetIdentityInformation: unsafe extern "system" fn(*mut core::ffi::c_void) -> Option<super::vsbackup::IVssExamineWriterMetadata>,
    #[cfg(not(feature = "Win32_vsbackup"))]
    GetIdentityInformation: usize,
    pub SetWriterFailureEx: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::HRESULT, windows_core::HRESULT, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_vss")]
    pub GetSessionId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::vss::VSS_ID) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_vss"))]
    GetSessionId: usize,
    pub IsWriterShuttingDown: unsafe extern "system" fn(*mut core::ffi::c_void) -> bool,
}
#[cfg(all(feature = "Win32_vsbackup", feature = "Win32_vss"))]
pub trait IVssWriterImpl_Impl: windows_core::IUnknownImpl {
    fn Initialize(&self, writerid: &super::vss::VSS_ID, wszwritername: &windows_core::PCWSTR, wszwriterinstancename: &windows_core::PCWSTR, dwmajorversion: u32, dwminorversion: u32, ut: VSS_USAGE_TYPE, st: VSS_SOURCE_TYPE, nlevel: super::vss::VSS_APPLICATION_LEVEL, dwtimeout: u32, aws: VSS_ALTERNATE_WRITER_STATE, biothrottlingonly: bool) -> windows_core::Result<()>;
    fn Subscribe(&self, dwsubscribetimeout: u32, dweventflags: u32) -> windows_core::Result<()>;
    fn Unsubscribe(&self) -> windows_core::Result<()>;
    fn Uninitialize(&self);
    fn GetCurrentVolumeArray(&self) -> *mut windows_core::PCWSTR;
    fn GetCurrentVolumeCount(&self) -> u32;
    fn GetSnapshotDeviceName(&self, wszoriginalvolume: &windows_core::PCWSTR) -> windows_core::Result<windows_core::PCWSTR>;
    fn GetCurrentSnapshotSetId(&self) -> super::vss::VSS_ID;
    fn GetContext(&self) -> i32;
    fn GetCurrentLevel(&self) -> super::vss::VSS_APPLICATION_LEVEL;
    fn IsPathAffected(&self, wszpath: &windows_core::PCWSTR) -> bool;
    fn IsBootableSystemStateBackedUp(&self) -> bool;
    fn AreComponentsSelected(&self) -> bool;
    fn GetBackupType(&self) -> super::vss::VSS_BACKUP_TYPE;
    fn GetRestoreType(&self) -> super::vss::VSS_RESTORE_TYPE;
    fn SetWriterFailure(&self, hr: windows_core::HRESULT) -> windows_core::Result<()>;
    fn IsPartialFileSupportEnabled(&self) -> bool;
    fn InstallAlternateWriter(&self, idwriter: &super::vss::VSS_ID, clsid: &windows_core::GUID) -> windows_core::Result<()>;
    fn GetIdentityInformation(&self) -> Option<super::vsbackup::IVssExamineWriterMetadata>;
    fn SetWriterFailureEx(&self, hr: windows_core::HRESULT, hrapplication: windows_core::HRESULT, wszapplicationmessage: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetSessionId(&self) -> windows_core::Result<super::vss::VSS_ID>;
    fn IsWriterShuttingDown(&self) -> bool;
}
#[cfg(all(feature = "Win32_vsbackup", feature = "Win32_vss"))]
impl IVssWriterImpl_Vtbl {
    pub const fn new<Identity: IVssWriterImpl_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Initialize<Identity: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, writerid: super::vss::VSS_ID, wszwritername: windows_core::PCWSTR, wszwriterinstancename: windows_core::PCWSTR, dwmajorversion: u32, dwminorversion: u32, ut: VSS_USAGE_TYPE, st: VSS_SOURCE_TYPE, nlevel: super::vss::VSS_APPLICATION_LEVEL, dwtimeout: u32, aws: VSS_ALTERNATE_WRITER_STATE, biothrottlingonly: bool) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWriterImpl_Impl::Initialize(this, core::mem::transmute(&writerid), core::mem::transmute(&wszwritername), core::mem::transmute(&wszwriterinstancename), core::mem::transmute_copy(&dwmajorversion), core::mem::transmute_copy(&dwminorversion), core::mem::transmute_copy(&ut), core::mem::transmute_copy(&st), core::mem::transmute_copy(&nlevel), core::mem::transmute_copy(&dwtimeout), core::mem::transmute_copy(&aws), core::mem::transmute_copy(&biothrottlingonly)).into()
            }
        }
        unsafe extern "system" fn Subscribe<Identity: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dwsubscribetimeout: u32, dweventflags: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWriterImpl_Impl::Subscribe(this, core::mem::transmute_copy(&dwsubscribetimeout), core::mem::transmute_copy(&dweventflags)).into()
            }
        }
        unsafe extern "system" fn Unsubscribe<Identity: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWriterImpl_Impl::Unsubscribe(this).into()
            }
        }
        unsafe extern "system" fn Uninitialize<Identity: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWriterImpl_Impl::Uninitialize(this);
            }
        }
        unsafe extern "system" fn GetCurrentVolumeArray<Identity: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> *mut windows_core::PCWSTR {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWriterImpl_Impl::GetCurrentVolumeArray(this)
            }
        }
        unsafe extern "system" fn GetCurrentVolumeCount<Identity: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> u32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWriterImpl_Impl::GetCurrentVolumeCount(this)
            }
        }
        unsafe extern "system" fn GetSnapshotDeviceName<Identity: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszoriginalvolume: windows_core::PCWSTR, ppwszsnapshotdevice: *mut windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssWriterImpl_Impl::GetSnapshotDeviceName(this, core::mem::transmute(&wszoriginalvolume)) {
                    Ok(ok__) => {
                        ppwszsnapshotdevice.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentSnapshotSetId<Identity: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, result__: *mut super::vss::VSS_ID) {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                *result__ = IVssWriterImpl_Impl::GetCurrentSnapshotSetId(this);
            }
        }
        unsafe extern "system" fn GetContext<Identity: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> i32 {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWriterImpl_Impl::GetContext(this)
            }
        }
        unsafe extern "system" fn GetCurrentLevel<Identity: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::vss::VSS_APPLICATION_LEVEL {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWriterImpl_Impl::GetCurrentLevel(this)
            }
        }
        unsafe extern "system" fn IsPathAffected<Identity: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, wszpath: windows_core::PCWSTR) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWriterImpl_Impl::IsPathAffected(this, core::mem::transmute(&wszpath))
            }
        }
        unsafe extern "system" fn IsBootableSystemStateBackedUp<Identity: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWriterImpl_Impl::IsBootableSystemStateBackedUp(this)
            }
        }
        unsafe extern "system" fn AreComponentsSelected<Identity: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWriterImpl_Impl::AreComponentsSelected(this)
            }
        }
        unsafe extern "system" fn GetBackupType<Identity: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::vss::VSS_BACKUP_TYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWriterImpl_Impl::GetBackupType(this)
            }
        }
        unsafe extern "system" fn GetRestoreType<Identity: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> super::vss::VSS_RESTORE_TYPE {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWriterImpl_Impl::GetRestoreType(this)
            }
        }
        unsafe extern "system" fn SetWriterFailure<Identity: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWriterImpl_Impl::SetWriterFailure(this, core::mem::transmute_copy(&hr)).into()
            }
        }
        unsafe extern "system" fn IsPartialFileSupportEnabled<Identity: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWriterImpl_Impl::IsPartialFileSupportEnabled(this)
            }
        }
        unsafe extern "system" fn InstallAlternateWriter<Identity: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idwriter: super::vss::VSS_ID, clsid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWriterImpl_Impl::InstallAlternateWriter(this, core::mem::transmute(&idwriter), core::mem::transmute(&clsid)).into()
            }
        }
        unsafe extern "system" fn GetIdentityInformation<Identity: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> Option<super::vsbackup::IVssExamineWriterMetadata> {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWriterImpl_Impl::GetIdentityInformation(this)
            }
        }
        unsafe extern "system" fn SetWriterFailureEx<Identity: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hr: windows_core::HRESULT, hrapplication: windows_core::HRESULT, wszapplicationmessage: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWriterImpl_Impl::SetWriterFailureEx(this, core::mem::transmute_copy(&hr), core::mem::transmute_copy(&hrapplication), core::mem::transmute(&wszapplicationmessage)).into()
            }
        }
        unsafe extern "system" fn GetSessionId<Identity: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idsession: *mut super::vss::VSS_ID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVssWriterImpl_Impl::GetSessionId(this) {
                    Ok(ok__) => {
                        idsession.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsWriterShuttingDown<Identity: IVssWriterImpl_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> bool {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVssWriterImpl_Impl::IsWriterShuttingDown(this)
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Initialize: Initialize::<Identity, OFFSET>,
            Subscribe: Subscribe::<Identity, OFFSET>,
            Unsubscribe: Unsubscribe::<Identity, OFFSET>,
            Uninitialize: Uninitialize::<Identity, OFFSET>,
            GetCurrentVolumeArray: GetCurrentVolumeArray::<Identity, OFFSET>,
            GetCurrentVolumeCount: GetCurrentVolumeCount::<Identity, OFFSET>,
            GetSnapshotDeviceName: GetSnapshotDeviceName::<Identity, OFFSET>,
            GetCurrentSnapshotSetId: GetCurrentSnapshotSetId::<Identity, OFFSET>,
            GetContext: GetContext::<Identity, OFFSET>,
            GetCurrentLevel: GetCurrentLevel::<Identity, OFFSET>,
            IsPathAffected: IsPathAffected::<Identity, OFFSET>,
            IsBootableSystemStateBackedUp: IsBootableSystemStateBackedUp::<Identity, OFFSET>,
            AreComponentsSelected: AreComponentsSelected::<Identity, OFFSET>,
            GetBackupType: GetBackupType::<Identity, OFFSET>,
            GetRestoreType: GetRestoreType::<Identity, OFFSET>,
            SetWriterFailure: SetWriterFailure::<Identity, OFFSET>,
            IsPartialFileSupportEnabled: IsPartialFileSupportEnabled::<Identity, OFFSET>,
            InstallAlternateWriter: InstallAlternateWriter::<Identity, OFFSET>,
            GetIdentityInformation: GetIdentityInformation::<Identity, OFFSET>,
            SetWriterFailureEx: SetWriterFailureEx::<Identity, OFFSET>,
            GetSessionId: GetSessionId::<Identity, OFFSET>,
            IsWriterShuttingDown: IsWriterShuttingDown::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVssWriterImpl as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_vsbackup", feature = "Win32_vss"))]
impl windows_core::RuntimeName for IVssWriterImpl {}
pub type VSS_ALTERNATE_WRITER_STATE = i32;
pub const VSS_AWS_ALTERNATE_WRITER_EXISTS: VSS_ALTERNATE_WRITER_STATE = 2;
pub const VSS_AWS_NO_ALTERNATE_WRITER: VSS_ALTERNATE_WRITER_STATE = 1;
pub const VSS_AWS_THIS_IS_ALTERNATE_WRITER: VSS_ALTERNATE_WRITER_STATE = 3;
pub const VSS_AWS_UNDEFINED: VSS_ALTERNATE_WRITER_STATE = 0;
pub const VSS_CF_APP_ROLLBACK_RECOVERY: VSS_COMPONENT_FLAGS = 2;
pub const VSS_CF_BACKUP_RECOVERY: VSS_COMPONENT_FLAGS = 1;
pub const VSS_CF_NOT_SYSTEM_STATE: VSS_COMPONENT_FLAGS = 4;
pub type VSS_COMPONENT_FLAGS = i32;
pub type VSS_COMPONENT_TYPE = i32;
pub const VSS_CT_DATABASE: VSS_COMPONENT_TYPE = 1;
pub const VSS_CT_FILEGROUP: VSS_COMPONENT_TYPE = 2;
pub const VSS_CT_UNDEFINED: VSS_COMPONENT_TYPE = 0;
pub type VSS_FILE_RESTORE_STATUS = i32;
pub type VSS_RESTOREMETHOD_ENUM = i32;
pub type VSS_RESTORE_TARGET = i32;
pub const VSS_RME_CUSTOM: VSS_RESTOREMETHOD_ENUM = 7;
pub const VSS_RME_RESTORE_AT_REBOOT: VSS_RESTOREMETHOD_ENUM = 5;
pub const VSS_RME_RESTORE_AT_REBOOT_IF_CANNOT_REPLACE: VSS_RESTOREMETHOD_ENUM = 6;
pub const VSS_RME_RESTORE_IF_CAN_REPLACE: VSS_RESTOREMETHOD_ENUM = 2;
pub const VSS_RME_RESTORE_IF_NOT_THERE: VSS_RESTOREMETHOD_ENUM = 1;
pub const VSS_RME_RESTORE_STOP_START: VSS_RESTOREMETHOD_ENUM = 8;
pub const VSS_RME_RESTORE_TO_ALTERNATE_LOCATION: VSS_RESTOREMETHOD_ENUM = 4;
pub const VSS_RME_STOP_RESTORE_START: VSS_RESTOREMETHOD_ENUM = 3;
pub const VSS_RME_UNDEFINED: VSS_RESTOREMETHOD_ENUM = 0;
pub const VSS_RS_ALL: VSS_FILE_RESTORE_STATUS = 2;
pub const VSS_RS_FAILED: VSS_FILE_RESTORE_STATUS = 3;
pub const VSS_RS_NONE: VSS_FILE_RESTORE_STATUS = 1;
pub const VSS_RS_UNDEFINED: VSS_FILE_RESTORE_STATUS = 0;
pub const VSS_RT_ALTERNATE: VSS_RESTORE_TARGET = 2;
pub const VSS_RT_DIRECTED: VSS_RESTORE_TARGET = 3;
pub const VSS_RT_ORIGINAL: VSS_RESTORE_TARGET = 1;
pub const VSS_RT_ORIGINAL_LOCATION: VSS_RESTORE_TARGET = 4;
pub const VSS_RT_UNDEFINED: VSS_RESTORE_TARGET = 0;
pub const VSS_SM_ALL_FLAGS: VSS_SUBSCRIBE_MASK = -1;
pub const VSS_SM_BACKUP_EVENTS_FLAG: VSS_SUBSCRIBE_MASK = 2;
pub const VSS_SM_IO_THROTTLING_FLAG: VSS_SUBSCRIBE_MASK = 8;
pub const VSS_SM_POST_SNAPSHOT_FLAG: VSS_SUBSCRIBE_MASK = 1;
pub const VSS_SM_RESTORE_EVENTS_FLAG: VSS_SUBSCRIBE_MASK = 4;
pub type VSS_SOURCE_TYPE = i32;
pub const VSS_ST_NONTRANSACTEDDB: VSS_SOURCE_TYPE = 2;
pub const VSS_ST_OTHER: VSS_SOURCE_TYPE = 3;
pub const VSS_ST_TRANSACTEDDB: VSS_SOURCE_TYPE = 1;
pub const VSS_ST_UNDEFINED: VSS_SOURCE_TYPE = 0;
pub type VSS_SUBSCRIBE_MASK = i32;
pub type VSS_USAGE_TYPE = i32;
pub const VSS_UT_BOOTABLESYSTEMSTATE: VSS_USAGE_TYPE = 1;
pub const VSS_UT_OTHER: VSS_USAGE_TYPE = 4;
pub const VSS_UT_SYSTEMSERVICE: VSS_USAGE_TYPE = 2;
pub const VSS_UT_UNDEFINED: VSS_USAGE_TYPE = 0;
pub const VSS_UT_USERDATA: VSS_USAGE_TYPE = 3;
pub const VSS_WRE_ALWAYS: VSS_WRITERRESTORE_ENUM = 3;
pub const VSS_WRE_IF_REPLACE_FAILS: VSS_WRITERRESTORE_ENUM = 2;
pub const VSS_WRE_NEVER: VSS_WRITERRESTORE_ENUM = 1;
pub const VSS_WRE_UNDEFINED: VSS_WRITERRESTORE_ENUM = 0;
pub type VSS_WRITERRESTORE_ENUM = i32;
