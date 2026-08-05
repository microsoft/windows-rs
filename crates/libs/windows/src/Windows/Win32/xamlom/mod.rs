#[inline]
pub unsafe fn InitializeXamlDiagnosticsEx<P0, P2, P3, P5>(endpointname: P0, pid: u32, wszdllxamldiagnostics: P2, wsztapdllname: P3, tapclsid: windows_core::GUID, wszinitializationdata: P5) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("windows.ui.xaml.dll" "C" fn InitializeXamlDiagnosticsEx(endpointname : windows_core::PCWSTR, pid : u32, wszdllxamldiagnostics : windows_core::PCWSTR, wsztapdllname : windows_core::PCWSTR, tapclsid : windows_core::GUID, wszinitializationdata : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { InitializeXamlDiagnosticsEx(endpointname.param().abi(), pid, wszdllxamldiagnostics.param().abi(), wsztapdllname.param().abi(), tapclsid, wszinitializationdata.param().abi()) }
}
pub const Add: VisualMutationType = 0;
pub const Animation: BaseValueSource = 12;
pub type BaseValueSource = i32;
pub const BaseValueSourceBuiltInStyle: BaseValueSource = 2;
pub const BaseValueSourceDefault: BaseValueSource = 1;
pub const BaseValueSourceLocal: BaseValueSource = 4;
pub const BaseValueSourceStyle: BaseValueSource = 3;
pub const BaseValueSourceUnknown: BaseValueSource = 0;
pub const BaseValueSourceVisualState: BaseValueSource = 14;
#[repr(C)]
#[cfg(feature = "dxgi")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BitmapDescription {
    pub Width: u32,
    pub Height: u32,
    pub Format: super::DXGI_FORMAT,
    pub AlphaMode: super::DXGI_ALPHA_MODE,
}
pub const Coercion: BaseValueSource = 13;
#[repr(C)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CollectionElementValue {
    pub Index: u32,
    pub ValueType: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub Value: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub MetadataBits: i64,
}
pub const DefaultStyleTrigger: BaseValueSource = 6;
pub const E_UNKNOWNTYPE: i32 = -2144665560;
#[repr(C)]
#[cfg(feature = "oaidl")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EnumType {
    pub Name: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub ValueInts: *mut super::SAFEARRAY,
    pub ValueStrings: *mut super::SAFEARRAY,
}
#[cfg(feature = "oaidl")]
impl Default for EnumType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ErrorInvalidResource: VisualElementState = 2;
pub const ErrorResolved: VisualElementState = 0;
pub const ErrorResourceNotFound: VisualElementState = 1;
windows_core::imp::define_interface!(IBitmapData, IBitmapData_Vtbl, 0xd1a34ef2_cad8_4635_a3d2_fcda8d3f3caf);
windows_core::imp::interface_hierarchy!(IBitmapData, windows_core::IUnknown);
impl IBitmapData {
    #[cfg(feature = "rpc")]
    pub unsafe fn CopyBytesTo(&self, sourceoffsetinbytes: u32, maxbytestocopy: u32, pvbytes: *mut super::byte, numberofbytescopied: *mut u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CopyBytesTo)(windows_core::Interface::as_raw(self), sourceoffsetinbytes, maxbytestocopy, pvbytes as _, numberofbytescopied as _) }
    }
    pub unsafe fn GetStride(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStride)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "dxgi")]
    pub unsafe fn GetBitmapDescription(&self) -> windows_core::Result<BitmapDescription> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBitmapDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "dxgi")]
    pub unsafe fn GetSourceBitmapDescription(&self) -> windows_core::Result<BitmapDescription> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSourceBitmapDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBitmapData_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "rpc")]
    pub CopyBytesTo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut super::byte, *mut u32) -> windows_core::HRESULT,
    #[cfg(not(feature = "rpc"))]
    CopyBytesTo: usize,
    pub GetStride: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "dxgi")]
    pub GetBitmapDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BitmapDescription) -> windows_core::HRESULT,
    #[cfg(not(feature = "dxgi"))]
    GetBitmapDescription: usize,
    #[cfg(feature = "dxgi")]
    pub GetSourceBitmapDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BitmapDescription) -> windows_core::HRESULT,
    #[cfg(not(feature = "dxgi"))]
    GetSourceBitmapDescription: usize,
}
#[cfg(all(feature = "dxgi", feature = "rpc"))]
pub trait IBitmapData_Impl: windows_core::IUnknownImpl {
    fn CopyBytesTo(&self, sourceoffsetinbytes: u32, maxbytestocopy: u32, pvbytes: *mut super::byte, numberofbytescopied: *mut u32) -> windows_core::Result<()>;
    fn GetStride(&self) -> windows_core::Result<u32>;
    fn GetBitmapDescription(&self) -> windows_core::Result<BitmapDescription>;
    fn GetSourceBitmapDescription(&self) -> windows_core::Result<BitmapDescription>;
}
#[cfg(all(feature = "dxgi", feature = "rpc"))]
impl IBitmapData_Vtbl {
    pub const fn new<Identity: IBitmapData_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CopyBytesTo<Identity: IBitmapData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourceoffsetinbytes: u32, maxbytestocopy: u32, pvbytes: *mut super::byte, numberofbytescopied: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBitmapData_Impl::CopyBytesTo(this, core::mem::transmute_copy(&sourceoffsetinbytes), core::mem::transmute_copy(&maxbytestocopy), core::mem::transmute_copy(&pvbytes), core::mem::transmute_copy(&numberofbytescopied)).into()
            }
        }
        unsafe extern "system" fn GetStride<Identity: IBitmapData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstride: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitmapData_Impl::GetStride(this) {
                    Ok(ok__) => {
                        pstride.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBitmapDescription<Identity: IBitmapData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitmapdescription: *mut BitmapDescription) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitmapData_Impl::GetBitmapDescription(this) {
                    Ok(ok__) => {
                        pbitmapdescription.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSourceBitmapDescription<Identity: IBitmapData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pbitmapdescription: *mut BitmapDescription) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBitmapData_Impl::GetSourceBitmapDescription(this) {
                    Ok(ok__) => {
                        pbitmapdescription.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CopyBytesTo: CopyBytesTo::<Identity, OFFSET>,
            GetStride: GetStride::<Identity, OFFSET>,
            GetBitmapDescription: GetBitmapDescription::<Identity, OFFSET>,
            GetSourceBitmapDescription: GetSourceBitmapDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBitmapData as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "dxgi", feature = "rpc"))]
impl windows_core::RuntimeName for IBitmapData {}
windows_core::imp::define_interface!(IVisualTreeService, IVisualTreeService_Vtbl, 0xa593b11a_d17f_48bb_8f66_83910731c8a5);
windows_core::imp::interface_hierarchy!(IVisualTreeService, windows_core::IUnknown);
impl IVisualTreeService {
    pub unsafe fn AdviseVisualTreeChange<P0>(&self, pcallback: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IVisualTreeServiceCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).AdviseVisualTreeChange)(windows_core::Interface::as_raw(self), pcallback.param().abi()) }
    }
    pub unsafe fn UnadviseVisualTreeChange<P0>(&self, pcallback: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IVisualTreeServiceCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnadviseVisualTreeChange)(windows_core::Interface::as_raw(self), pcallback.param().abi()) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetEnums(&self, pcount: *mut u32, ppenums: *mut *mut EnumType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetEnums)(windows_core::Interface::as_raw(self), pcount as _, ppenums as _) }
    }
    pub unsafe fn CreateInstance(&self, typename: &windows_core::BSTR, value: &windows_core::BSTR) -> windows_core::Result<InstanceHandle> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(typename), core::mem::transmute_copy(value), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPropertyValuesChain(&self, instancehandle: InstanceHandle, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyValuesChain)(windows_core::Interface::as_raw(self), instancehandle, psourcecount as _, pppropertysources as _, ppropertycount as _, pppropertyvalues as _) }
    }
    pub unsafe fn SetProperty(&self, instancehandle: InstanceHandle, value: InstanceHandle, propertyindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), instancehandle, value, propertyindex) }
    }
    pub unsafe fn ClearProperty(&self, instancehandle: InstanceHandle, propertyindex: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearProperty)(windows_core::Interface::as_raw(self), instancehandle, propertyindex) }
    }
    pub unsafe fn GetCollectionCount(&self, instancehandle: InstanceHandle) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCollectionCount)(windows_core::Interface::as_raw(self), instancehandle, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCollectionElements(&self, instancehandle: InstanceHandle, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCollectionElements)(windows_core::Interface::as_raw(self), instancehandle, startindex, pelementcount as _, ppelementvalues as _) }
    }
    pub unsafe fn AddChild(&self, parent: InstanceHandle, child: InstanceHandle, index: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddChild)(windows_core::Interface::as_raw(self), parent, child, index) }
    }
    pub unsafe fn RemoveChild(&self, parent: InstanceHandle, index: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveChild)(windows_core::Interface::as_raw(self), parent, index) }
    }
    pub unsafe fn ClearChildren(&self, parent: InstanceHandle) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearChildren)(windows_core::Interface::as_raw(self), parent) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AdviseVisualTreeChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnadviseVisualTreeChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub GetEnums: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut EnumType) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetEnums: usize,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut InstanceHandle) -> windows_core::HRESULT,
    pub GetPropertyValuesChain: unsafe extern "system" fn(*mut core::ffi::c_void, InstanceHandle, *mut u32, *mut *mut PropertyChainSource, *mut u32, *mut *mut PropertyChainValue) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, InstanceHandle, InstanceHandle, u32) -> windows_core::HRESULT,
    pub ClearProperty: unsafe extern "system" fn(*mut core::ffi::c_void, InstanceHandle, u32) -> windows_core::HRESULT,
    pub GetCollectionCount: unsafe extern "system" fn(*mut core::ffi::c_void, InstanceHandle, *mut u32) -> windows_core::HRESULT,
    pub GetCollectionElements: unsafe extern "system" fn(*mut core::ffi::c_void, InstanceHandle, u32, *mut u32, *mut *mut CollectionElementValue) -> windows_core::HRESULT,
    pub AddChild: unsafe extern "system" fn(*mut core::ffi::c_void, InstanceHandle, InstanceHandle, u32) -> windows_core::HRESULT,
    pub RemoveChild: unsafe extern "system" fn(*mut core::ffi::c_void, InstanceHandle, u32) -> windows_core::HRESULT,
    pub ClearChildren: unsafe extern "system" fn(*mut core::ffi::c_void, InstanceHandle) -> windows_core::HRESULT,
}
#[cfg(feature = "oaidl")]
pub trait IVisualTreeService_Impl: windows_core::IUnknownImpl {
    fn AdviseVisualTreeChange(&self, pcallback: windows_core::Ref<IVisualTreeServiceCallback>) -> windows_core::Result<()>;
    fn UnadviseVisualTreeChange(&self, pcallback: windows_core::Ref<IVisualTreeServiceCallback>) -> windows_core::Result<()>;
    fn GetEnums(&self, pcount: *mut u32, ppenums: *mut *mut EnumType) -> windows_core::Result<()>;
    fn CreateInstance(&self, typename: &windows_core::BSTR, value: &windows_core::BSTR) -> windows_core::Result<InstanceHandle>;
    fn GetPropertyValuesChain(&self, instancehandle: InstanceHandle, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> windows_core::Result<()>;
    fn SetProperty(&self, instancehandle: InstanceHandle, value: InstanceHandle, propertyindex: u32) -> windows_core::Result<()>;
    fn ClearProperty(&self, instancehandle: InstanceHandle, propertyindex: u32) -> windows_core::Result<()>;
    fn GetCollectionCount(&self, instancehandle: InstanceHandle) -> windows_core::Result<u32>;
    fn GetCollectionElements(&self, instancehandle: InstanceHandle, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> windows_core::Result<()>;
    fn AddChild(&self, parent: InstanceHandle, child: InstanceHandle, index: u32) -> windows_core::Result<()>;
    fn RemoveChild(&self, parent: InstanceHandle, index: u32) -> windows_core::Result<()>;
    fn ClearChildren(&self, parent: InstanceHandle) -> windows_core::Result<()>;
}
#[cfg(feature = "oaidl")]
impl IVisualTreeService_Vtbl {
    pub const fn new<Identity: IVisualTreeService_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdviseVisualTreeChange<Identity: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService_Impl::AdviseVisualTreeChange(this, core::mem::transmute_copy(&pcallback)).into()
            }
        }
        unsafe extern "system" fn UnadviseVisualTreeChange<Identity: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcallback: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService_Impl::UnadviseVisualTreeChange(this, core::mem::transmute_copy(&pcallback)).into()
            }
        }
        unsafe extern "system" fn GetEnums<Identity: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcount: *mut u32, ppenums: *mut *mut EnumType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService_Impl::GetEnums(this, core::mem::transmute_copy(&pcount), core::mem::transmute_copy(&ppenums)).into()
            }
        }
        unsafe extern "system" fn CreateInstance<Identity: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, typename: *mut core::ffi::c_void, value: *mut core::ffi::c_void, pinstancehandle: *mut InstanceHandle) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVisualTreeService_Impl::CreateInstance(this, core::mem::transmute(&typename), core::mem::transmute(&value)) {
                    Ok(ok__) => {
                        pinstancehandle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropertyValuesChain<Identity: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, instancehandle: InstanceHandle, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService_Impl::GetPropertyValuesChain(this, core::mem::transmute_copy(&instancehandle), core::mem::transmute_copy(&psourcecount), core::mem::transmute_copy(&pppropertysources), core::mem::transmute_copy(&ppropertycount), core::mem::transmute_copy(&pppropertyvalues)).into()
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, instancehandle: InstanceHandle, value: InstanceHandle, propertyindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService_Impl::SetProperty(this, core::mem::transmute_copy(&instancehandle), core::mem::transmute_copy(&value), core::mem::transmute_copy(&propertyindex)).into()
            }
        }
        unsafe extern "system" fn ClearProperty<Identity: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, instancehandle: InstanceHandle, propertyindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService_Impl::ClearProperty(this, core::mem::transmute_copy(&instancehandle), core::mem::transmute_copy(&propertyindex)).into()
            }
        }
        unsafe extern "system" fn GetCollectionCount<Identity: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, instancehandle: InstanceHandle, pcollectionsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVisualTreeService_Impl::GetCollectionCount(this, core::mem::transmute_copy(&instancehandle)) {
                    Ok(ok__) => {
                        pcollectionsize.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCollectionElements<Identity: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, instancehandle: InstanceHandle, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService_Impl::GetCollectionElements(this, core::mem::transmute_copy(&instancehandle), core::mem::transmute_copy(&startindex), core::mem::transmute_copy(&pelementcount), core::mem::transmute_copy(&ppelementvalues)).into()
            }
        }
        unsafe extern "system" fn AddChild<Identity: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parent: InstanceHandle, child: InstanceHandle, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService_Impl::AddChild(this, core::mem::transmute_copy(&parent), core::mem::transmute_copy(&child), core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn RemoveChild<Identity: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parent: InstanceHandle, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService_Impl::RemoveChild(this, core::mem::transmute_copy(&parent), core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn ClearChildren<Identity: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parent: InstanceHandle) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService_Impl::ClearChildren(this, core::mem::transmute_copy(&parent)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseVisualTreeChange: AdviseVisualTreeChange::<Identity, OFFSET>,
            UnadviseVisualTreeChange: UnadviseVisualTreeChange::<Identity, OFFSET>,
            GetEnums: GetEnums::<Identity, OFFSET>,
            CreateInstance: CreateInstance::<Identity, OFFSET>,
            GetPropertyValuesChain: GetPropertyValuesChain::<Identity, OFFSET>,
            SetProperty: SetProperty::<Identity, OFFSET>,
            ClearProperty: ClearProperty::<Identity, OFFSET>,
            GetCollectionCount: GetCollectionCount::<Identity, OFFSET>,
            GetCollectionElements: GetCollectionElements::<Identity, OFFSET>,
            AddChild: AddChild::<Identity, OFFSET>,
            RemoveChild: RemoveChild::<Identity, OFFSET>,
            ClearChildren: ClearChildren::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVisualTreeService as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IVisualTreeService {}
windows_core::imp::define_interface!(IVisualTreeService2, IVisualTreeService2_Vtbl, 0x130f5136_ec43_4f61_89c7_9801a36d2e95);
impl core::ops::Deref for IVisualTreeService2 {
    type Target = IVisualTreeService;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVisualTreeService2, windows_core::IUnknown, IVisualTreeService);
impl IVisualTreeService2 {
    pub unsafe fn GetPropertyIndex<P1>(&self, object: InstanceHandle, propertyname: P1) -> windows_core::Result<u32>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyIndex)(windows_core::Interface::as_raw(self), object, propertyname.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProperty(&self, object: InstanceHandle, propertyindex: u32) -> windows_core::Result<InstanceHandle> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), object, propertyindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReplaceResource(&self, resourcedictionary: InstanceHandle, key: InstanceHandle, newvalue: InstanceHandle) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ReplaceResource)(windows_core::Interface::as_raw(self), resourcedictionary, key, newvalue) }
    }
    pub unsafe fn RenderTargetBitmap(&self, handle: InstanceHandle, options: RenderTargetBitmapOptions, maxpixelwidth: u32, maxpixelheight: u32) -> windows_core::Result<IBitmapData> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RenderTargetBitmap)(windows_core::Interface::as_raw(self), handle, options, maxpixelwidth, maxpixelheight, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeService2_Vtbl {
    pub base__: IVisualTreeService_Vtbl,
    pub GetPropertyIndex: unsafe extern "system" fn(*mut core::ffi::c_void, InstanceHandle, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, InstanceHandle, u32, *mut InstanceHandle) -> windows_core::HRESULT,
    pub ReplaceResource: unsafe extern "system" fn(*mut core::ffi::c_void, InstanceHandle, InstanceHandle, InstanceHandle) -> windows_core::HRESULT,
    pub RenderTargetBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, InstanceHandle, RenderTargetBitmapOptions, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "oaidl")]
pub trait IVisualTreeService2_Impl: IVisualTreeService_Impl {
    fn GetPropertyIndex(&self, object: InstanceHandle, propertyname: &windows_core::PCWSTR) -> windows_core::Result<u32>;
    fn GetProperty(&self, object: InstanceHandle, propertyindex: u32) -> windows_core::Result<InstanceHandle>;
    fn ReplaceResource(&self, resourcedictionary: InstanceHandle, key: InstanceHandle, newvalue: InstanceHandle) -> windows_core::Result<()>;
    fn RenderTargetBitmap(&self, handle: InstanceHandle, options: RenderTargetBitmapOptions, maxpixelwidth: u32, maxpixelheight: u32) -> windows_core::Result<IBitmapData>;
}
#[cfg(feature = "oaidl")]
impl IVisualTreeService2_Vtbl {
    pub const fn new<Identity: IVisualTreeService2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPropertyIndex<Identity: IVisualTreeService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: InstanceHandle, propertyname: windows_core::PCWSTR, ppropertyindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVisualTreeService2_Impl::GetPropertyIndex(this, core::mem::transmute_copy(&object), core::mem::transmute(&propertyname)) {
                    Ok(ok__) => {
                        ppropertyindex.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IVisualTreeService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: InstanceHandle, propertyindex: u32, pvalue: *mut InstanceHandle) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVisualTreeService2_Impl::GetProperty(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&propertyindex)) {
                    Ok(ok__) => {
                        pvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReplaceResource<Identity: IVisualTreeService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcedictionary: InstanceHandle, key: InstanceHandle, newvalue: InstanceHandle) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService2_Impl::ReplaceResource(this, core::mem::transmute_copy(&resourcedictionary), core::mem::transmute_copy(&key), core::mem::transmute_copy(&newvalue)).into()
            }
        }
        unsafe extern "system" fn RenderTargetBitmap<Identity: IVisualTreeService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: InstanceHandle, options: RenderTargetBitmapOptions, maxpixelwidth: u32, maxpixelheight: u32, ppbitmapdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVisualTreeService2_Impl::RenderTargetBitmap(this, core::mem::transmute_copy(&handle), core::mem::transmute_copy(&options), core::mem::transmute_copy(&maxpixelwidth), core::mem::transmute_copy(&maxpixelheight)) {
                    Ok(ok__) => {
                        ppbitmapdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IVisualTreeService_Vtbl::new::<Identity, OFFSET>(),
            GetPropertyIndex: GetPropertyIndex::<Identity, OFFSET>,
            GetProperty: GetProperty::<Identity, OFFSET>,
            ReplaceResource: ReplaceResource::<Identity, OFFSET>,
            RenderTargetBitmap: RenderTargetBitmap::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVisualTreeService2 as windows_core::Interface>::IID || iid == &<IVisualTreeService as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IVisualTreeService2 {}
windows_core::imp::define_interface!(IVisualTreeService3, IVisualTreeService3_Vtbl, 0x0e79c6e0_85a0_4be8_b41a_655cf1fd19bd);
impl core::ops::Deref for IVisualTreeService3 {
    type Target = IVisualTreeService2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVisualTreeService3, windows_core::IUnknown, IVisualTreeService, IVisualTreeService2);
impl IVisualTreeService3 {
    pub unsafe fn ResolveResource<P1>(&self, resourcecontext: InstanceHandle, resourcename: P1, resourcetype: ResourceType, propertyindex: u32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ResolveResource)(windows_core::Interface::as_raw(self), resourcecontext, resourcename.param().abi(), resourcetype, propertyindex) }
    }
    pub unsafe fn GetDictionaryItem<P1>(&self, dictionaryhandle: InstanceHandle, resourcename: P1, resourceisimplicitstyle: bool) -> windows_core::Result<InstanceHandle>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDictionaryItem)(windows_core::Interface::as_raw(self), dictionaryhandle, resourcename.param().abi(), resourceisimplicitstyle.into(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AddDictionaryItem(&self, dictionaryhandle: InstanceHandle, resourcekey: InstanceHandle, resourcehandle: InstanceHandle) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddDictionaryItem)(windows_core::Interface::as_raw(self), dictionaryhandle, resourcekey, resourcehandle) }
    }
    pub unsafe fn RemoveDictionaryItem(&self, dictionaryhandle: InstanceHandle, resourcekey: InstanceHandle) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveDictionaryItem)(windows_core::Interface::as_raw(self), dictionaryhandle, resourcekey) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeService3_Vtbl {
    pub base__: IVisualTreeService2_Vtbl,
    pub ResolveResource: unsafe extern "system" fn(*mut core::ffi::c_void, InstanceHandle, windows_core::PCWSTR, ResourceType, u32) -> windows_core::HRESULT,
    pub GetDictionaryItem: unsafe extern "system" fn(*mut core::ffi::c_void, InstanceHandle, windows_core::PCWSTR, windows_core::BOOL, *mut InstanceHandle) -> windows_core::HRESULT,
    pub AddDictionaryItem: unsafe extern "system" fn(*mut core::ffi::c_void, InstanceHandle, InstanceHandle, InstanceHandle) -> windows_core::HRESULT,
    pub RemoveDictionaryItem: unsafe extern "system" fn(*mut core::ffi::c_void, InstanceHandle, InstanceHandle) -> windows_core::HRESULT,
}
#[cfg(feature = "oaidl")]
pub trait IVisualTreeService3_Impl: IVisualTreeService2_Impl {
    fn ResolveResource(&self, resourcecontext: InstanceHandle, resourcename: &windows_core::PCWSTR, resourcetype: ResourceType, propertyindex: u32) -> windows_core::Result<()>;
    fn GetDictionaryItem(&self, dictionaryhandle: InstanceHandle, resourcename: &windows_core::PCWSTR, resourceisimplicitstyle: windows_core::BOOL) -> windows_core::Result<InstanceHandle>;
    fn AddDictionaryItem(&self, dictionaryhandle: InstanceHandle, resourcekey: InstanceHandle, resourcehandle: InstanceHandle) -> windows_core::Result<()>;
    fn RemoveDictionaryItem(&self, dictionaryhandle: InstanceHandle, resourcekey: InstanceHandle) -> windows_core::Result<()>;
}
#[cfg(feature = "oaidl")]
impl IVisualTreeService3_Vtbl {
    pub const fn new<Identity: IVisualTreeService3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ResolveResource<Identity: IVisualTreeService3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcecontext: InstanceHandle, resourcename: windows_core::PCWSTR, resourcetype: ResourceType, propertyindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService3_Impl::ResolveResource(this, core::mem::transmute_copy(&resourcecontext), core::mem::transmute(&resourcename), core::mem::transmute_copy(&resourcetype), core::mem::transmute_copy(&propertyindex)).into()
            }
        }
        unsafe extern "system" fn GetDictionaryItem<Identity: IVisualTreeService3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionaryhandle: InstanceHandle, resourcename: windows_core::PCWSTR, resourceisimplicitstyle: windows_core::BOOL, resourcehandle: *mut InstanceHandle) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVisualTreeService3_Impl::GetDictionaryItem(this, core::mem::transmute_copy(&dictionaryhandle), core::mem::transmute(&resourcename), core::mem::transmute_copy(&resourceisimplicitstyle)) {
                    Ok(ok__) => {
                        resourcehandle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddDictionaryItem<Identity: IVisualTreeService3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionaryhandle: InstanceHandle, resourcekey: InstanceHandle, resourcehandle: InstanceHandle) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService3_Impl::AddDictionaryItem(this, core::mem::transmute_copy(&dictionaryhandle), core::mem::transmute_copy(&resourcekey), core::mem::transmute_copy(&resourcehandle)).into()
            }
        }
        unsafe extern "system" fn RemoveDictionaryItem<Identity: IVisualTreeService3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionaryhandle: InstanceHandle, resourcekey: InstanceHandle) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService3_Impl::RemoveDictionaryItem(this, core::mem::transmute_copy(&dictionaryhandle), core::mem::transmute_copy(&resourcekey)).into()
            }
        }
        Self {
            base__: IVisualTreeService2_Vtbl::new::<Identity, OFFSET>(),
            ResolveResource: ResolveResource::<Identity, OFFSET>,
            GetDictionaryItem: GetDictionaryItem::<Identity, OFFSET>,
            AddDictionaryItem: AddDictionaryItem::<Identity, OFFSET>,
            RemoveDictionaryItem: RemoveDictionaryItem::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVisualTreeService3 as windows_core::Interface>::IID || iid == &<IVisualTreeService as windows_core::Interface>::IID || iid == &<IVisualTreeService2 as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IVisualTreeService3 {}
windows_core::imp::define_interface!(IVisualTreeServiceCallback, IVisualTreeServiceCallback_Vtbl, 0xaa7a8931_80e4_4fec_8f3b_553f87b4966e);
windows_core::imp::interface_hierarchy!(IVisualTreeServiceCallback, windows_core::IUnknown);
impl IVisualTreeServiceCallback {
    pub unsafe fn OnVisualTreeChange(&self, relation: ParentChildRelation, element: &VisualElement, mutationtype: VisualMutationType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).OnVisualTreeChange)(windows_core::Interface::as_raw(self), relation, core::mem::transmute_copy(element), mutationtype) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeServiceCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnVisualTreeChange: unsafe extern "system" fn(*mut core::ffi::c_void, ParentChildRelation, VisualElement, VisualMutationType) -> windows_core::HRESULT,
}
pub trait IVisualTreeServiceCallback_Impl: windows_core::IUnknownImpl {
    fn OnVisualTreeChange(&self, relation: &ParentChildRelation, element: &VisualElement, mutationtype: VisualMutationType) -> windows_core::Result<()>;
}
impl IVisualTreeServiceCallback_Vtbl {
    pub const fn new<Identity: IVisualTreeServiceCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnVisualTreeChange<Identity: IVisualTreeServiceCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, relation: ParentChildRelation, element: VisualElement, mutationtype: VisualMutationType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeServiceCallback_Impl::OnVisualTreeChange(this, core::mem::transmute(&relation), core::mem::transmute(&element), core::mem::transmute_copy(&mutationtype)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), OnVisualTreeChange: OnVisualTreeChange::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVisualTreeServiceCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVisualTreeServiceCallback {}
windows_core::imp::define_interface!(IVisualTreeServiceCallback2, IVisualTreeServiceCallback2_Vtbl, 0xbad9eb88_ae77_4397_b948_5fa2db0a19ea);
impl core::ops::Deref for IVisualTreeServiceCallback2 {
    type Target = IVisualTreeServiceCallback;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IVisualTreeServiceCallback2, windows_core::IUnknown, IVisualTreeServiceCallback);
impl IVisualTreeServiceCallback2 {
    pub unsafe fn OnElementStateChanged<P2>(&self, element: InstanceHandle, elementstate: VisualElementState, context: P2) -> windows_core::HRESULT
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnElementStateChanged)(windows_core::Interface::as_raw(self), element, elementstate, context.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeServiceCallback2_Vtbl {
    pub base__: IVisualTreeServiceCallback_Vtbl,
    pub OnElementStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, InstanceHandle, VisualElementState, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IVisualTreeServiceCallback2_Impl: IVisualTreeServiceCallback_Impl {
    fn OnElementStateChanged(&self, element: InstanceHandle, elementstate: VisualElementState, context: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IVisualTreeServiceCallback2_Vtbl {
    pub const fn new<Identity: IVisualTreeServiceCallback2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnElementStateChanged<Identity: IVisualTreeServiceCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: InstanceHandle, elementstate: VisualElementState, context: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeServiceCallback2_Impl::OnElementStateChanged(this, core::mem::transmute_copy(&element), core::mem::transmute_copy(&elementstate), core::mem::transmute(&context)).into()
            }
        }
        Self { base__: IVisualTreeServiceCallback_Vtbl::new::<Identity, OFFSET>(), OnElementStateChanged: OnElementStateChanged::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVisualTreeServiceCallback2 as windows_core::Interface>::IID || iid == &<IVisualTreeServiceCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVisualTreeServiceCallback2 {}
windows_core::imp::define_interface!(IXamlDiagnostics, IXamlDiagnostics_Vtbl, 0x18c9e2b6_3f43_4116_9f2b_ff935d7770d2);
windows_core::imp::interface_hierarchy!(IXamlDiagnostics, windows_core::IUnknown);
impl IXamlDiagnostics {
    pub unsafe fn GetDispatcher(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDispatcher)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetUiLayer(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUiLayer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetApplication(&self) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetApplication)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetIInspectableFromHandle(&self, instancehandle: InstanceHandle) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIInspectableFromHandle)(windows_core::Interface::as_raw(self), instancehandle, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetHandleFromIInspectable<P0>(&self, pinstance: P0) -> windows_core::Result<InstanceHandle>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHandleFromIInspectable)(windows_core::Interface::as_raw(self), pinstance.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn HitTest(&self, rect: super::RECT, pcount: *mut u32, ppinstancehandles: *mut *mut InstanceHandle) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).HitTest)(windows_core::Interface::as_raw(self), rect, pcount as _, ppinstancehandles as _) }
    }
    pub unsafe fn RegisterInstance<P0>(&self, pinstance: P0) -> windows_core::Result<InstanceHandle>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterInstance)(windows_core::Interface::as_raw(self), pinstance.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetInitializationData(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetInitializationData)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IXamlDiagnostics_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetDispatcher: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetUiLayer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetApplication: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetIInspectableFromHandle: unsafe extern "system" fn(*mut core::ffi::c_void, InstanceHandle, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHandleFromIInspectable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut InstanceHandle) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub HitTest: unsafe extern "system" fn(*mut core::ffi::c_void, super::RECT, *mut u32, *mut *mut InstanceHandle) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    HitTest: usize,
    pub RegisterInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut InstanceHandle) -> windows_core::HRESULT,
    pub GetInitializationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "windef")]
pub trait IXamlDiagnostics_Impl: windows_core::IUnknownImpl {
    fn GetDispatcher(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn GetUiLayer(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn GetApplication(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn GetIInspectableFromHandle(&self, instancehandle: InstanceHandle) -> windows_core::Result<windows_core::IInspectable>;
    fn GetHandleFromIInspectable(&self, pinstance: windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<InstanceHandle>;
    fn HitTest(&self, rect: &super::RECT, pcount: *mut u32, ppinstancehandles: *mut *mut InstanceHandle) -> windows_core::Result<()>;
    fn RegisterInstance(&self, pinstance: windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<InstanceHandle>;
    fn GetInitializationData(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "windef")]
impl IXamlDiagnostics_Vtbl {
    pub const fn new<Identity: IXamlDiagnostics_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetDispatcher<Identity: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppdispatcher: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlDiagnostics_Impl::GetDispatcher(this) {
                    Ok(ok__) => {
                        ppdispatcher.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetUiLayer<Identity: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pplayer: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlDiagnostics_Impl::GetUiLayer(this) {
                    Ok(ok__) => {
                        pplayer.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetApplication<Identity: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppapplication: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlDiagnostics_Impl::GetApplication(this) {
                    Ok(ok__) => {
                        ppapplication.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIInspectableFromHandle<Identity: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, instancehandle: InstanceHandle, ppinstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlDiagnostics_Impl::GetIInspectableFromHandle(this, core::mem::transmute_copy(&instancehandle)) {
                    Ok(ok__) => {
                        ppinstance.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetHandleFromIInspectable<Identity: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinstance: *mut core::ffi::c_void, phandle: *mut InstanceHandle) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlDiagnostics_Impl::GetHandleFromIInspectable(this, core::mem::transmute_copy(&pinstance)) {
                    Ok(ok__) => {
                        phandle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HitTest<Identity: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rect: super::RECT, pcount: *mut u32, ppinstancehandles: *mut *mut InstanceHandle) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXamlDiagnostics_Impl::HitTest(this, core::mem::transmute(&rect), core::mem::transmute_copy(&pcount), core::mem::transmute_copy(&ppinstancehandles)).into()
            }
        }
        unsafe extern "system" fn RegisterInstance<Identity: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinstance: *mut core::ffi::c_void, pinstancehandle: *mut InstanceHandle) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlDiagnostics_Impl::RegisterInstance(this, core::mem::transmute_copy(&pinstance)) {
                    Ok(ok__) => {
                        pinstancehandle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetInitializationData<Identity: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinitializationdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlDiagnostics_Impl::GetInitializationData(this) {
                    Ok(ok__) => {
                        pinitializationdata.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetDispatcher: GetDispatcher::<Identity, OFFSET>,
            GetUiLayer: GetUiLayer::<Identity, OFFSET>,
            GetApplication: GetApplication::<Identity, OFFSET>,
            GetIInspectableFromHandle: GetIInspectableFromHandle::<Identity, OFFSET>,
            GetHandleFromIInspectable: GetHandleFromIInspectable::<Identity, OFFSET>,
            HitTest: HitTest::<Identity, OFFSET>,
            RegisterInstance: RegisterInstance::<Identity, OFFSET>,
            GetInitializationData: GetInitializationData::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IXamlDiagnostics as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IXamlDiagnostics {}
pub const ImplicitStyleReference: BaseValueSource = 9;
pub const Inherited: BaseValueSource = 5;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct InstanceHandle(pub u64);
pub const IsPropertyReadOnly: MetadataBit = 2;
pub const IsValueBindingExpression: MetadataBit = 16;
pub const IsValueCollection: MetadataBit = 4;
pub const IsValueCollectionReadOnly: MetadataBit = 8;
pub const IsValueHandle: MetadataBit = 1;
pub const IsValueHandleAndEvaluatedValue: MetadataBit = 64;
pub const IsValueNull: MetadataBit = 32;
pub type MetadataBit = i32;
pub const None: MetadataBit = 0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ParentChildRelation {
    pub Parent: InstanceHandle,
    pub Child: InstanceHandle,
    pub ChildIndex: u32,
}
pub const ParentTemplate: BaseValueSource = 10;
pub const ParentTemplateTrigger: BaseValueSource = 11;
#[repr(C)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PropertyChainSource {
    pub Handle: InstanceHandle,
    pub TargetType: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub Name: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub Source: BaseValueSource,
    pub SrcInfo: SourceInfo,
}
#[repr(C)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct PropertyChainValue {
    pub Index: u32,
    pub Type: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub DeclaringType: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub ValueType: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub ItemType: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub Value: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub Overridden: windows_core::BOOL,
    pub MetadataBits: i64,
    pub PropertyName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub PropertyChainIndex: u32,
}
pub const Remove: VisualMutationType = 1;
pub const RenderTarget: RenderTargetBitmapOptions = 0;
pub const RenderTargetAndChildren: RenderTargetBitmapOptions = 1;
pub type RenderTargetBitmapOptions = i32;
pub type ResourceType = i32;
pub const ResourceTypeStatic: ResourceType = 0;
pub const ResourceTypeTheme: ResourceType = 1;
#[repr(C)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SourceInfo {
    pub FileName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub LineNumber: u32,
    pub ColumnNumber: u32,
    pub CharPosition: u32,
    pub Hash: core::mem::ManuallyDrop<windows_core::BSTR>,
}
pub const StyleTrigger: BaseValueSource = 8;
pub const TemplateTrigger: BaseValueSource = 7;
#[repr(C)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct VisualElement {
    pub Handle: InstanceHandle,
    pub SrcInfo: SourceInfo,
    pub Type: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub Name: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub NumChildren: u32,
}
pub type VisualElementState = i32;
pub type VisualMutationType = i32;
