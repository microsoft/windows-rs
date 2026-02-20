#[inline]
pub unsafe fn InitializeXamlDiagnostic<P0, P2, P3>(endpointname: P0, pid: u32, wszdllxamldiagnostics: P2, wsztapdllname: P3, tapclsid: windows_core::GUID) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("windows.ui.xaml.dll" "system" fn InitializeXamlDiagnostic(endpointname : windows_core::PCWSTR, pid : u32, wszdllxamldiagnostics : windows_core::PCWSTR, wsztapdllname : windows_core::PCWSTR, tapclsid : windows_core::GUID) -> windows_core::HRESULT);
    unsafe { InitializeXamlDiagnostic(endpointname.param().abi(), pid, wszdllxamldiagnostics.param().abi(), wsztapdllname.param().abi(), core::mem::transmute(tapclsid)).ok() }
}
#[inline]
pub unsafe fn InitializeXamlDiagnosticsEx<P0, P2, P3, P5>(endpointname: P0, pid: u32, wszdllxamldiagnostics: P2, wsztapdllname: P3, tapclsid: windows_core::GUID, wszinitializationdata: P5) -> windows_core::Result<()>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
    P2: windows_core::Param<windows_core::PCWSTR>,
    P3: windows_core::Param<windows_core::PCWSTR>,
    P5: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("windows.ui.xaml.dll" "system" fn InitializeXamlDiagnosticsEx(endpointname : windows_core::PCWSTR, pid : u32, wszdllxamldiagnostics : windows_core::PCWSTR, wsztapdllname : windows_core::PCWSTR, tapclsid : windows_core::GUID, wszinitializationdata : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { InitializeXamlDiagnosticsEx(endpointname.param().abi(), pid, wszdllxamldiagnostics.param().abi(), wsztapdllname.param().abi(), core::mem::transmute(tapclsid), wszinitializationdata.param().abi()).ok() }
}
pub const Add: VisualMutationType = VisualMutationType(0i32);
pub const Animation: BaseValueSource = BaseValueSource(12i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct BaseValueSource(pub i32);
pub const BaseValueSourceBuiltInStyle: BaseValueSource = BaseValueSource(2i32);
pub const BaseValueSourceDefault: BaseValueSource = BaseValueSource(1i32);
pub const BaseValueSourceLocal: BaseValueSource = BaseValueSource(4i32);
pub const BaseValueSourceStyle: BaseValueSource = BaseValueSource(3i32);
pub const BaseValueSourceUnknown: BaseValueSource = BaseValueSource(0i32);
pub const BaseValueSourceVisualState: BaseValueSource = BaseValueSource(14i32);
#[repr(C)]
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BitmapDescription {
    pub Width: u32,
    pub Height: u32,
    pub Format: super::super::super::Graphics::Dxgi::Common::DXGI_FORMAT,
    pub AlphaMode: super::super::super::Graphics::Dxgi::Common::DXGI_ALPHA_MODE,
}
pub const Coercion: BaseValueSource = BaseValueSource(13i32);
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CollectionElementValue {
    pub Index: u32,
    pub ValueType: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub Value: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub MetadataBits: i64,
}
pub const DefaultStyleTrigger: BaseValueSource = BaseValueSource(6i32);
pub const E_UNKNOWNTYPE: windows_core::HRESULT = windows_core::HRESULT(0x802B0028_u32 as _);
#[repr(C)]
#[cfg(feature = "Win32_System_Com")]
#[derive(Clone, Debug, PartialEq)]
pub struct EnumType {
    pub Name: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub ValueInts: *mut super::super::super::System::Com::SAFEARRAY,
    pub ValueStrings: *mut super::super::super::System::Com::SAFEARRAY,
}
#[cfg(feature = "Win32_System_Com")]
impl Default for EnumType {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const ErrorInvalidResource: VisualElementState = VisualElementState(2i32);
pub const ErrorResolved: VisualElementState = VisualElementState(0i32);
pub const ErrorResourceNotFound: VisualElementState = VisualElementState(1i32);
windows_core::imp::define_interface!(IBitmapData, IBitmapData_Vtbl, 0xd1a34ef2_cad8_4635_a3d2_fcda8d3f3caf);
windows_core::imp::interface_hierarchy!(IBitmapData, windows_core::IUnknown);
impl IBitmapData {
    pub unsafe fn CopyBytesTo(&self, sourceoffsetinbytes: u32, pvbytes: &mut [u8], numberofbytescopied: *mut u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).CopyBytesTo)(windows_core::Interface::as_raw(self), sourceoffsetinbytes, pvbytes.len().try_into().unwrap(), core::mem::transmute(pvbytes.as_ptr()), numberofbytescopied as _).ok() }
    }
    pub unsafe fn GetStride(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetStride)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub unsafe fn GetBitmapDescription(&self) -> windows_core::Result<BitmapDescription> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBitmapDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
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
    pub CopyBytesTo: unsafe extern "system" fn(*mut core::ffi::c_void, u32, u32, *mut u8, *mut u32) -> windows_core::HRESULT,
    pub GetStride: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetBitmapDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BitmapDescription) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetBitmapDescription: usize,
    #[cfg(feature = "Win32_Graphics_Dxgi_Common")]
    pub GetSourceBitmapDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut BitmapDescription) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_Graphics_Dxgi_Common"))]
    GetSourceBitmapDescription: usize,
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
pub trait IBitmapData_Impl: windows_core::IUnknownImpl {
    fn CopyBytesTo(&self, sourceoffsetinbytes: u32, maxbytestocopy: u32, pvbytes: *mut u8, numberofbytescopied: *mut u32) -> windows_core::Result<()>;
    fn GetStride(&self) -> windows_core::Result<u32>;
    fn GetBitmapDescription(&self) -> windows_core::Result<BitmapDescription>;
    fn GetSourceBitmapDescription(&self) -> windows_core::Result<BitmapDescription>;
}
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl IBitmapData_Vtbl {
    pub const fn new<Identity: IBitmapData_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CopyBytesTo<Identity: IBitmapData_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sourceoffsetinbytes: u32, maxbytestocopy: u32, pvbytes: *mut u8, numberofbytescopied: *mut u32) -> windows_core::HRESULT {
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
                        pstride.write(core::mem::transmute(ok__));
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
                        pbitmapdescription.write(core::mem::transmute(ok__));
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
                        pbitmapdescription.write(core::mem::transmute(ok__));
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
#[cfg(feature = "Win32_Graphics_Dxgi_Common")]
impl windows_core::RuntimeName for IBitmapData {}
windows_core::imp::define_interface!(IVisualTreeService, IVisualTreeService_Vtbl, 0xa593b11a_d17f_48bb_8f66_83910731c8a5);
windows_core::imp::interface_hierarchy!(IVisualTreeService, windows_core::IUnknown);
impl IVisualTreeService {
    pub unsafe fn AdviseVisualTreeChange<P0>(&self, pcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IVisualTreeServiceCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).AdviseVisualTreeChange)(windows_core::Interface::as_raw(self), pcallback.param().abi()).ok() }
    }
    pub unsafe fn UnadviseVisualTreeChange<P0>(&self, pcallback: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<IVisualTreeServiceCallback>,
    {
        unsafe { (windows_core::Interface::vtable(self).UnadviseVisualTreeChange)(windows_core::Interface::as_raw(self), pcallback.param().abi()).ok() }
    }
    #[cfg(feature = "Win32_System_Com")]
    pub unsafe fn GetEnums(&self, pcount: *mut u32, ppenums: *mut *mut EnumType) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetEnums)(windows_core::Interface::as_raw(self), pcount as _, ppenums as _).ok() }
    }
    pub unsafe fn CreateInstance(&self, typename: &windows_core::BSTR, value: &windows_core::BSTR) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateInstance)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(typename), core::mem::transmute_copy(value), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPropertyValuesChain(&self, instancehandle: u64, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetPropertyValuesChain)(windows_core::Interface::as_raw(self), instancehandle, psourcecount as _, pppropertysources as _, ppropertycount as _, pppropertyvalues as _).ok() }
    }
    pub unsafe fn SetProperty(&self, instancehandle: u64, value: u64, propertyindex: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).SetProperty)(windows_core::Interface::as_raw(self), instancehandle, value, propertyindex).ok() }
    }
    pub unsafe fn ClearProperty(&self, instancehandle: u64, propertyindex: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ClearProperty)(windows_core::Interface::as_raw(self), instancehandle, propertyindex).ok() }
    }
    pub unsafe fn GetCollectionCount(&self, instancehandle: u64) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCollectionCount)(windows_core::Interface::as_raw(self), instancehandle, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCollectionElements(&self, instancehandle: u64, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).GetCollectionElements)(windows_core::Interface::as_raw(self), instancehandle, startindex, pelementcount as _, ppelementvalues as _).ok() }
    }
    pub unsafe fn AddChild(&self, parent: u64, child: u64, index: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddChild)(windows_core::Interface::as_raw(self), parent, child, index).ok() }
    }
    pub unsafe fn RemoveChild(&self, parent: u64, index: u32) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveChild)(windows_core::Interface::as_raw(self), parent, index).ok() }
    }
    pub unsafe fn ClearChildren(&self, parent: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ClearChildren)(windows_core::Interface::as_raw(self), parent).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeService_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AdviseVisualTreeChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UnadviseVisualTreeChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "Win32_System_Com")]
    pub GetEnums: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut EnumType) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_System_Com"))]
    GetEnums: usize,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetPropertyValuesChain: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut u32, *mut *mut PropertyChainSource, *mut u32, *mut *mut PropertyChainValue) -> windows_core::HRESULT,
    pub SetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, u32) -> windows_core::HRESULT,
    pub ClearProperty: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u32) -> windows_core::HRESULT,
    pub GetCollectionCount: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut u32) -> windows_core::HRESULT,
    pub GetCollectionElements: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u32, *mut u32, *mut *mut CollectionElementValue) -> windows_core::HRESULT,
    pub AddChild: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, u32) -> windows_core::HRESULT,
    pub RemoveChild: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u32) -> windows_core::HRESULT,
    pub ClearChildren: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVisualTreeService_Impl: windows_core::IUnknownImpl {
    fn AdviseVisualTreeChange(&self, pcallback: windows_core::Ref<IVisualTreeServiceCallback>) -> windows_core::Result<()>;
    fn UnadviseVisualTreeChange(&self, pcallback: windows_core::Ref<IVisualTreeServiceCallback>) -> windows_core::Result<()>;
    fn GetEnums(&self, pcount: *mut u32, ppenums: *mut *mut EnumType) -> windows_core::Result<()>;
    fn CreateInstance(&self, typename: &windows_core::BSTR, value: &windows_core::BSTR) -> windows_core::Result<u64>;
    fn GetPropertyValuesChain(&self, instancehandle: u64, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> windows_core::Result<()>;
    fn SetProperty(&self, instancehandle: u64, value: u64, propertyindex: u32) -> windows_core::Result<()>;
    fn ClearProperty(&self, instancehandle: u64, propertyindex: u32) -> windows_core::Result<()>;
    fn GetCollectionCount(&self, instancehandle: u64) -> windows_core::Result<u32>;
    fn GetCollectionElements(&self, instancehandle: u64, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> windows_core::Result<()>;
    fn AddChild(&self, parent: u64, child: u64, index: u32) -> windows_core::Result<()>;
    fn RemoveChild(&self, parent: u64, index: u32) -> windows_core::Result<()>;
    fn ClearChildren(&self, parent: u64) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
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
        unsafe extern "system" fn CreateInstance<Identity: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, typename: *mut core::ffi::c_void, value: *mut core::ffi::c_void, pinstancehandle: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVisualTreeService_Impl::CreateInstance(this, core::mem::transmute(&typename), core::mem::transmute(&value)) {
                    Ok(ok__) => {
                        pinstancehandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropertyValuesChain<Identity: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, instancehandle: u64, psourcecount: *mut u32, pppropertysources: *mut *mut PropertyChainSource, ppropertycount: *mut u32, pppropertyvalues: *mut *mut PropertyChainValue) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService_Impl::GetPropertyValuesChain(this, core::mem::transmute_copy(&instancehandle), core::mem::transmute_copy(&psourcecount), core::mem::transmute_copy(&pppropertysources), core::mem::transmute_copy(&ppropertycount), core::mem::transmute_copy(&pppropertyvalues)).into()
            }
        }
        unsafe extern "system" fn SetProperty<Identity: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, instancehandle: u64, value: u64, propertyindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService_Impl::SetProperty(this, core::mem::transmute_copy(&instancehandle), core::mem::transmute_copy(&value), core::mem::transmute_copy(&propertyindex)).into()
            }
        }
        unsafe extern "system" fn ClearProperty<Identity: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, instancehandle: u64, propertyindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService_Impl::ClearProperty(this, core::mem::transmute_copy(&instancehandle), core::mem::transmute_copy(&propertyindex)).into()
            }
        }
        unsafe extern "system" fn GetCollectionCount<Identity: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, instancehandle: u64, pcollectionsize: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVisualTreeService_Impl::GetCollectionCount(this, core::mem::transmute_copy(&instancehandle)) {
                    Ok(ok__) => {
                        pcollectionsize.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCollectionElements<Identity: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, instancehandle: u64, startindex: u32, pelementcount: *mut u32, ppelementvalues: *mut *mut CollectionElementValue) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService_Impl::GetCollectionElements(this, core::mem::transmute_copy(&instancehandle), core::mem::transmute_copy(&startindex), core::mem::transmute_copy(&pelementcount), core::mem::transmute_copy(&ppelementvalues)).into()
            }
        }
        unsafe extern "system" fn AddChild<Identity: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parent: u64, child: u64, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService_Impl::AddChild(this, core::mem::transmute_copy(&parent), core::mem::transmute_copy(&child), core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn RemoveChild<Identity: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parent: u64, index: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService_Impl::RemoveChild(this, core::mem::transmute_copy(&parent), core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn ClearChildren<Identity: IVisualTreeService_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parent: u64) -> windows_core::HRESULT {
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
#[cfg(feature = "Win32_System_Com")]
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
    pub unsafe fn GetPropertyIndex<P1>(&self, object: u64, propertyname: P1) -> windows_core::Result<u32>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyIndex)(windows_core::Interface::as_raw(self), object, propertyname.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetProperty(&self, object: u64, propertyindex: u32) -> windows_core::Result<u64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), object, propertyindex, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReplaceResource(&self, resourcedictionary: u64, key: u64, newvalue: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).ReplaceResource)(windows_core::Interface::as_raw(self), resourcedictionary, key, newvalue).ok() }
    }
    pub unsafe fn RenderTargetBitmap(&self, handle: u64, options: RenderTargetBitmapOptions, maxpixelwidth: u32, maxpixelheight: u32) -> windows_core::Result<IBitmapData> {
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
    pub GetPropertyIndex: unsafe extern "system" fn(*mut core::ffi::c_void, u64, windows_core::PCWSTR, *mut u32) -> windows_core::HRESULT,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u32, *mut u64) -> windows_core::HRESULT,
    pub ReplaceResource: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, u64) -> windows_core::HRESULT,
    pub RenderTargetBitmap: unsafe extern "system" fn(*mut core::ffi::c_void, u64, RenderTargetBitmapOptions, u32, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVisualTreeService2_Impl: IVisualTreeService_Impl {
    fn GetPropertyIndex(&self, object: u64, propertyname: &windows_core::PCWSTR) -> windows_core::Result<u32>;
    fn GetProperty(&self, object: u64, propertyindex: u32) -> windows_core::Result<u64>;
    fn ReplaceResource(&self, resourcedictionary: u64, key: u64, newvalue: u64) -> windows_core::Result<()>;
    fn RenderTargetBitmap(&self, handle: u64, options: RenderTargetBitmapOptions, maxpixelwidth: u32, maxpixelheight: u32) -> windows_core::Result<IBitmapData>;
}
#[cfg(feature = "Win32_System_Com")]
impl IVisualTreeService2_Vtbl {
    pub const fn new<Identity: IVisualTreeService2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetPropertyIndex<Identity: IVisualTreeService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: u64, propertyname: windows_core::PCWSTR, ppropertyindex: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVisualTreeService2_Impl::GetPropertyIndex(this, core::mem::transmute_copy(&object), core::mem::transmute(&propertyname)) {
                    Ok(ok__) => {
                        ppropertyindex.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetProperty<Identity: IVisualTreeService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, object: u64, propertyindex: u32, pvalue: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVisualTreeService2_Impl::GetProperty(this, core::mem::transmute_copy(&object), core::mem::transmute_copy(&propertyindex)) {
                    Ok(ok__) => {
                        pvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReplaceResource<Identity: IVisualTreeService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcedictionary: u64, key: u64, newvalue: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService2_Impl::ReplaceResource(this, core::mem::transmute_copy(&resourcedictionary), core::mem::transmute_copy(&key), core::mem::transmute_copy(&newvalue)).into()
            }
        }
        unsafe extern "system" fn RenderTargetBitmap<Identity: IVisualTreeService2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: u64, options: RenderTargetBitmapOptions, maxpixelwidth: u32, maxpixelheight: u32, ppbitmapdata: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(feature = "Win32_System_Com")]
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
    pub unsafe fn ResolveResource<P1>(&self, resourcecontext: u64, resourcename: P1, resourcetype: ResourceType, propertyindex: u32) -> windows_core::Result<()>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).ResolveResource)(windows_core::Interface::as_raw(self), resourcecontext, resourcename.param().abi(), resourcetype, propertyindex).ok() }
    }
    pub unsafe fn GetDictionaryItem<P1>(&self, dictionaryhandle: u64, resourcename: P1, resourceisimplicitstyle: bool) -> windows_core::Result<u64>
    where
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetDictionaryItem)(windows_core::Interface::as_raw(self), dictionaryhandle, resourcename.param().abi(), resourceisimplicitstyle.into(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AddDictionaryItem(&self, dictionaryhandle: u64, resourcekey: u64, resourcehandle: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).AddDictionaryItem)(windows_core::Interface::as_raw(self), dictionaryhandle, resourcekey, resourcehandle).ok() }
    }
    pub unsafe fn RemoveDictionaryItem(&self, dictionaryhandle: u64, resourcekey: u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).RemoveDictionaryItem)(windows_core::Interface::as_raw(self), dictionaryhandle, resourcekey).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeService3_Vtbl {
    pub base__: IVisualTreeService2_Vtbl,
    pub ResolveResource: unsafe extern "system" fn(*mut core::ffi::c_void, u64, windows_core::PCWSTR, ResourceType, u32) -> windows_core::HRESULT,
    pub GetDictionaryItem: unsafe extern "system" fn(*mut core::ffi::c_void, u64, windows_core::PCWSTR, windows_core::BOOL, *mut u64) -> windows_core::HRESULT,
    pub AddDictionaryItem: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64, u64) -> windows_core::HRESULT,
    pub RemoveDictionaryItem: unsafe extern "system" fn(*mut core::ffi::c_void, u64, u64) -> windows_core::HRESULT,
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVisualTreeService3_Impl: IVisualTreeService2_Impl {
    fn ResolveResource(&self, resourcecontext: u64, resourcename: &windows_core::PCWSTR, resourcetype: ResourceType, propertyindex: u32) -> windows_core::Result<()>;
    fn GetDictionaryItem(&self, dictionaryhandle: u64, resourcename: &windows_core::PCWSTR, resourceisimplicitstyle: windows_core::BOOL) -> windows_core::Result<u64>;
    fn AddDictionaryItem(&self, dictionaryhandle: u64, resourcekey: u64, resourcehandle: u64) -> windows_core::Result<()>;
    fn RemoveDictionaryItem(&self, dictionaryhandle: u64, resourcekey: u64) -> windows_core::Result<()>;
}
#[cfg(feature = "Win32_System_Com")]
impl IVisualTreeService3_Vtbl {
    pub const fn new<Identity: IVisualTreeService3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ResolveResource<Identity: IVisualTreeService3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, resourcecontext: u64, resourcename: windows_core::PCWSTR, resourcetype: ResourceType, propertyindex: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService3_Impl::ResolveResource(this, core::mem::transmute_copy(&resourcecontext), core::mem::transmute(&resourcename), core::mem::transmute_copy(&resourcetype), core::mem::transmute_copy(&propertyindex)).into()
            }
        }
        unsafe extern "system" fn GetDictionaryItem<Identity: IVisualTreeService3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionaryhandle: u64, resourcename: windows_core::PCWSTR, resourceisimplicitstyle: windows_core::BOOL, resourcehandle: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IVisualTreeService3_Impl::GetDictionaryItem(this, core::mem::transmute_copy(&dictionaryhandle), core::mem::transmute(&resourcename), core::mem::transmute_copy(&resourceisimplicitstyle)) {
                    Ok(ok__) => {
                        resourcehandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddDictionaryItem<Identity: IVisualTreeService3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionaryhandle: u64, resourcekey: u64, resourcehandle: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVisualTreeService3_Impl::AddDictionaryItem(this, core::mem::transmute_copy(&dictionaryhandle), core::mem::transmute_copy(&resourcekey), core::mem::transmute_copy(&resourcehandle)).into()
            }
        }
        unsafe extern "system" fn RemoveDictionaryItem<Identity: IVisualTreeService3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dictionaryhandle: u64, resourcekey: u64) -> windows_core::HRESULT {
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
#[cfg(feature = "Win32_System_Com")]
impl windows_core::RuntimeName for IVisualTreeService3 {}
windows_core::imp::define_interface!(IVisualTreeServiceCallback, IVisualTreeServiceCallback_Vtbl, 0xaa7a8931_80e4_4fec_8f3b_553f87b4966e);
windows_core::imp::interface_hierarchy!(IVisualTreeServiceCallback, windows_core::IUnknown);
impl IVisualTreeServiceCallback {
    pub unsafe fn OnVisualTreeChange(&self, relation: ParentChildRelation, element: &VisualElement, mutationtype: VisualMutationType) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).OnVisualTreeChange)(windows_core::Interface::as_raw(self), core::mem::transmute(relation), core::mem::transmute_copy(element), mutationtype).ok() }
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
    pub unsafe fn OnElementStateChanged<P2>(&self, element: u64, elementstate: VisualElementState, context: P2) -> windows_core::Result<()>
    where
        P2: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnElementStateChanged)(windows_core::Interface::as_raw(self), element, elementstate, context.param().abi()).ok() }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVisualTreeServiceCallback2_Vtbl {
    pub base__: IVisualTreeServiceCallback_Vtbl,
    pub OnElementStateChanged: unsafe extern "system" fn(*mut core::ffi::c_void, u64, VisualElementState, windows_core::PCWSTR) -> windows_core::HRESULT,
}
pub trait IVisualTreeServiceCallback2_Impl: IVisualTreeServiceCallback_Impl {
    fn OnElementStateChanged(&self, element: u64, elementstate: VisualElementState, context: &windows_core::PCWSTR) -> windows_core::Result<()>;
}
impl IVisualTreeServiceCallback2_Vtbl {
    pub const fn new<Identity: IVisualTreeServiceCallback2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnElementStateChanged<Identity: IVisualTreeServiceCallback2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: u64, elementstate: VisualElementState, context: windows_core::PCWSTR) -> windows_core::HRESULT {
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
    pub unsafe fn GetIInspectableFromHandle(&self, instancehandle: u64) -> windows_core::Result<windows_core::IInspectable> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIInspectableFromHandle)(windows_core::Interface::as_raw(self), instancehandle, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetHandleFromIInspectable<P0>(&self, pinstance: P0) -> windows_core::Result<u64>
    where
        P0: windows_core::Param<windows_core::IInspectable>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetHandleFromIInspectable)(windows_core::Interface::as_raw(self), pinstance.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn HitTest(&self, rect: super::super::super::Foundation::RECT, pcount: *mut u32, ppinstancehandles: *mut *mut u64) -> windows_core::Result<()> {
        unsafe { (windows_core::Interface::vtable(self).HitTest)(windows_core::Interface::as_raw(self), core::mem::transmute(rect), pcount as _, ppinstancehandles as _).ok() }
    }
    pub unsafe fn RegisterInstance<P0>(&self, pinstance: P0) -> windows_core::Result<u64>
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
    pub GetIInspectableFromHandle: unsafe extern "system" fn(*mut core::ffi::c_void, u64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetHandleFromIInspectable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub HitTest: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::RECT, *mut u32, *mut *mut u64) -> windows_core::HRESULT,
    pub RegisterInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub GetInitializationData: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IXamlDiagnostics_Impl: windows_core::IUnknownImpl {
    fn GetDispatcher(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn GetUiLayer(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn GetApplication(&self) -> windows_core::Result<windows_core::IInspectable>;
    fn GetIInspectableFromHandle(&self, instancehandle: u64) -> windows_core::Result<windows_core::IInspectable>;
    fn GetHandleFromIInspectable(&self, pinstance: windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<u64>;
    fn HitTest(&self, rect: &super::super::super::Foundation::RECT, pcount: *mut u32, ppinstancehandles: *mut *mut u64) -> windows_core::Result<()>;
    fn RegisterInstance(&self, pinstance: windows_core::Ref<windows_core::IInspectable>) -> windows_core::Result<u64>;
    fn GetInitializationData(&self) -> windows_core::Result<windows_core::BSTR>;
}
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
        unsafe extern "system" fn GetIInspectableFromHandle<Identity: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, instancehandle: u64, ppinstance: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetHandleFromIInspectable<Identity: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinstance: *mut core::ffi::c_void, phandle: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlDiagnostics_Impl::GetHandleFromIInspectable(this, core::mem::transmute_copy(&pinstance)) {
                    Ok(ok__) => {
                        phandle.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HitTest<Identity: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rect: super::super::super::Foundation::RECT, pcount: *mut u32, ppinstancehandles: *mut *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IXamlDiagnostics_Impl::HitTest(this, core::mem::transmute(&rect), core::mem::transmute_copy(&pcount), core::mem::transmute_copy(&ppinstancehandles)).into()
            }
        }
        unsafe extern "system" fn RegisterInstance<Identity: IXamlDiagnostics_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pinstance: *mut core::ffi::c_void, pinstancehandle: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IXamlDiagnostics_Impl::RegisterInstance(this, core::mem::transmute_copy(&pinstance)) {
                    Ok(ok__) => {
                        pinstancehandle.write(core::mem::transmute(ok__));
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
impl windows_core::RuntimeName for IXamlDiagnostics {}
pub const ImplicitStyleReference: BaseValueSource = BaseValueSource(9i32);
pub const Inherited: BaseValueSource = BaseValueSource(5i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MetadataBit(pub i32);
impl MetadataBit {
    pub const None: Self = Self(0i32);
    pub const IsValueHandle: Self = Self(1i32);
    pub const IsPropertyReadOnly: Self = Self(2i32);
    pub const IsValueCollection: Self = Self(4i32);
    pub const IsValueCollectionReadOnly: Self = Self(8i32);
    pub const IsValueBindingExpression: Self = Self(16i32);
    pub const IsValueNull: Self = Self(32i32);
    pub const IsValueHandleAndEvaluatedValue: Self = Self(64i32);
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ParentChildRelation {
    pub Parent: u64,
    pub Child: u64,
    pub ChildIndex: u32,
}
pub const ParentTemplate: BaseValueSource = BaseValueSource(10i32);
pub const ParentTemplateTrigger: BaseValueSource = BaseValueSource(11i32);
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PropertyChainSource {
    pub Handle: u64,
    pub TargetType: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub Name: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub Source: BaseValueSource,
    pub SrcInfo: SourceInfo,
}
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
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
pub const Remove: VisualMutationType = VisualMutationType(1i32);
pub const RenderTarget: RenderTargetBitmapOptions = RenderTargetBitmapOptions(0i32);
pub const RenderTargetAndChildren: RenderTargetBitmapOptions = RenderTargetBitmapOptions(1i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RenderTargetBitmapOptions(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ResourceType(pub i32);
pub const ResourceTypeStatic: ResourceType = ResourceType(0i32);
pub const ResourceTypeTheme: ResourceType = ResourceType(1i32);
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SourceInfo {
    pub FileName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub LineNumber: u32,
    pub ColumnNumber: u32,
    pub CharPosition: u32,
    pub Hash: core::mem::ManuallyDrop<windows_core::BSTR>,
}
pub const StyleTrigger: BaseValueSource = BaseValueSource(8i32);
pub const TemplateTrigger: BaseValueSource = BaseValueSource(7i32);
#[repr(C)]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VisualElement {
    pub Handle: u64,
    pub SrcInfo: SourceInfo,
    pub Type: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub Name: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub NumChildren: u32,
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VisualElementState(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VisualMutationType(pub i32);
