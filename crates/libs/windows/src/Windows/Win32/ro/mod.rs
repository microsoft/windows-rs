#[cfg(feature = "Win32_restrictederrorinfo")]
#[inline]
pub unsafe fn GetRestrictedErrorInfo() -> windows_core::Result<super::restrictederrorinfo::IRestrictedErrorInfo> {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn GetRestrictedErrorInfo(pprestrictederrorinfo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        GetRestrictedErrorInfo(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn IsErrorPropagationEnabled() -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn IsErrorPropagationEnabled() -> windows_core::BOOL);
    unsafe { IsErrorPropagationEnabled() }
}
#[inline]
pub unsafe fn MetaDataGetDispenser(rclsid: *const windows_core::GUID, riid: *const windows_core::GUID, ppv: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("rometadata.dll" "system" fn MetaDataGetDispenser(rclsid : *const windows_core::GUID, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MetaDataGetDispenser(rclsid, riid, ppv as _) }
}
#[inline]
pub unsafe fn RoActivateInstance(activatableclassid: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoActivateInstance(activatableclassid : *mut core::ffi::c_void, instance : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoActivateInstance(core::mem::transmute_copy(activatableclassid), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn RoCaptureErrorContext(hr: windows_core::HRESULT) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoCaptureErrorContext(hr : windows_core::HRESULT) -> windows_core::HRESULT);
    unsafe { RoCaptureErrorContext(hr) }
}
#[inline]
pub unsafe fn RoClearError() {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoClearError());
    unsafe { RoClearError() }
}
#[inline]
pub unsafe fn RoFailFastWithErrorContext(hrerror: windows_core::HRESULT) {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoFailFastWithErrorContext(hrerror : windows_core::HRESULT));
    unsafe { RoFailFastWithErrorContext(hrerror) }
}
#[inline]
pub unsafe fn RoFreeParameterizedTypeExtra(extra: ROPARAMIIDHANDLE) {
    windows_core::link!("api-ms-win-core-winrt-roparameterizediid-l1-1-0.dll" "system" fn RoFreeParameterizedTypeExtra(extra : ROPARAMIIDHANDLE));
    unsafe { RoFreeParameterizedTypeExtra(extra) }
}
#[inline]
pub unsafe fn RoGetActivationFactory<T>(activatableclassid: &windows_core::HSTRING) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoGetActivationFactory(activatableclassid : *mut core::ffi::c_void, iid : *const windows_core::GUID, factory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { RoGetActivationFactory(core::mem::transmute_copy(activatableclassid), &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
}
#[inline]
pub unsafe fn RoGetApartmentIdentifier() -> windows_core::Result<u64> {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoGetApartmentIdentifier(apartmentidentifier : *mut u64) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoGetApartmentIdentifier(&mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_objidlbase")]
#[inline]
pub unsafe fn RoGetBufferMarshaler() -> windows_core::Result<super::objidlbase::IMarshal> {
    windows_core::link!("api-ms-win-core-winrt-robuffer-l1-1-0.dll" "system" fn RoGetBufferMarshaler(buffermarshaler : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoGetBufferMarshaler(&mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn RoGetErrorReportingFlags() -> windows_core::Result<u32> {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoGetErrorReportingFlags(pflags : *mut u32) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoGetErrorReportingFlags(&mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_restrictederrorinfo")]
#[inline]
pub unsafe fn RoGetMatchingRestrictedErrorInfo(hrin: windows_core::HRESULT) -> windows_core::Result<super::restrictederrorinfo::IRestrictedErrorInfo> {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoGetMatchingRestrictedErrorInfo(hrin : windows_core::HRESULT, pprestrictederrorinfo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoGetMatchingRestrictedErrorInfo(hrin, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn RoGetParameterizedTypeInstanceIID<P2>(nameelements: &[windows_core::PCWSTR], metadatalocator: P2, iid: *mut windows_core::GUID, pextra: *mut ROPARAMIIDHANDLE) -> windows_core::HRESULT
where
    P2: windows_core::Param<IRoMetaDataLocator>,
{
    windows_core::link!("api-ms-win-core-winrt-roparameterizediid-l1-1-0.dll" "system" fn RoGetParameterizedTypeInstanceIID(nameelementcount : u32, nameelements : *const windows_core::PCWSTR, metadatalocator : *mut core::ffi::c_void, iid : *mut windows_core::GUID, pextra : *mut ROPARAMIIDHANDLE) -> windows_core::HRESULT);
    unsafe { RoGetParameterizedTypeInstanceIID(nameelements.len().try_into().unwrap(), core::mem::transmute(nameelements.as_ptr()), metadatalocator.param().abi(), iid as _, pextra as _) }
}
#[inline]
pub unsafe fn RoInitialize(inittype: RO_INIT_TYPE) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoInitialize(inittype : RO_INIT_TYPE) -> windows_core::HRESULT);
    unsafe { RoInitialize(inittype) }
}
#[inline]
pub unsafe fn RoInspectCapturedStackBackTrace(targeterrorinfoaddress: usize, machine: u16, readmemorycallback: PINSPECT_MEMORY_CALLBACK, context: Option<*const core::ffi::c_void>, framecount: *mut u32, targetbacktraceaddress: *mut usize) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoInspectCapturedStackBackTrace(targeterrorinfoaddress : usize, machine : u16, readmemorycallback : PINSPECT_MEMORY_CALLBACK, context : *const core::ffi::c_void, framecount : *mut u32, targetbacktraceaddress : *mut usize) -> windows_core::HRESULT);
    unsafe { RoInspectCapturedStackBackTrace(targeterrorinfoaddress, machine, readmemorycallback, context.unwrap_or(core::mem::zeroed()) as _, framecount as _, targetbacktraceaddress as _) }
}
#[inline]
pub unsafe fn RoInspectThreadErrorInfo(targettebaddress: usize, machine: u16, readmemorycallback: PINSPECT_MEMORY_CALLBACK, context: Option<*const core::ffi::c_void>) -> windows_core::Result<usize> {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoInspectThreadErrorInfo(targettebaddress : usize, machine : u16, readmemorycallback : PINSPECT_MEMORY_CALLBACK, context : *const core::ffi::c_void, targeterrorinfoaddress : *mut usize) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoInspectThreadErrorInfo(targettebaddress, machine, readmemorycallback, context.unwrap_or(core::mem::zeroed()) as _, &mut result__).map(|| result__)
    }
}
#[inline]
pub unsafe fn RoOriginateError(error: windows_core::HRESULT, message: &windows_core::HSTRING) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoOriginateError(error : windows_core::HRESULT, message : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { RoOriginateError(error, core::mem::transmute_copy(message)) }
}
#[inline]
pub unsafe fn RoOriginateErrorW<P2>(error: windows_core::HRESULT, cchmax: u32, message: P2) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoOriginateErrorW(error : windows_core::HRESULT, cchmax : u32, message : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { RoOriginateErrorW(error, cchmax, message.param().abi()) }
}
#[inline]
pub unsafe fn RoOriginateLanguageException<P2>(error: windows_core::HRESULT, message: &windows_core::HSTRING, languageexception: P2) -> windows_core::BOOL
where
    P2: windows_core::Param<windows_core::IUnknown>,
{
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoOriginateLanguageException(error : windows_core::HRESULT, message : *mut core::ffi::c_void, languageexception : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { RoOriginateLanguageException(error, core::mem::transmute_copy(message), languageexception.param().abi()) }
}
#[inline]
pub unsafe fn RoParameterizedTypeExtraGetTypeSignature(extra: ROPARAMIIDHANDLE) -> windows_core::PCSTR {
    windows_core::link!("api-ms-win-core-winrt-roparameterizediid-l1-1-0.dll" "system" fn RoParameterizedTypeExtraGetTypeSignature(extra : ROPARAMIIDHANDLE) -> windows_core::PCSTR);
    unsafe { RoParameterizedTypeExtraGetTypeSignature(extra) }
}
#[cfg(feature = "Win32_activation")]
#[inline]
pub unsafe fn RoRegisterActivationFactories(activatableclassids: *const windows_core::HSTRING, activationfactorycallbacks: *const PFNGETACTIVATIONFACTORY, count: u32) -> windows_core::Result<RO_REGISTRATION_COOKIE> {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoRegisterActivationFactories(activatableclassids : *const *mut core::ffi::c_void, activationfactorycallbacks : *const PFNGETACTIVATIONFACTORY, count : u32, cookie : *mut RO_REGISTRATION_COOKIE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoRegisterActivationFactories(core::mem::transmute(activatableclassids), activationfactorycallbacks, count, &mut result__).map(|| result__)
    }
}
#[cfg(feature = "Win32_objidl")]
#[inline]
pub unsafe fn RoRegisterForApartmentShutdown<P0>(callbackobject: P0, apartmentidentifier: *mut u64, regcookie: *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::objidl::IApartmentShutdown>,
{
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoRegisterForApartmentShutdown(callbackobject : *mut core::ffi::c_void, apartmentidentifier : *mut u64, regcookie : *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> windows_core::HRESULT);
    unsafe { RoRegisterForApartmentShutdown(callbackobject.param().abi(), apartmentidentifier as _, regcookie as _) }
}
#[cfg(feature = "Win32_restrictederrorinfo")]
#[inline]
pub unsafe fn RoReportFailedDelegate<P0, P1>(punkdelegate: P0, prestrictederrorinfo: P1) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<super::restrictederrorinfo::IRestrictedErrorInfo>,
{
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoReportFailedDelegate(punkdelegate : *mut core::ffi::c_void, prestrictederrorinfo : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RoReportFailedDelegate(punkdelegate.param().abi(), prestrictederrorinfo.param().abi()) }
}
#[cfg(feature = "Win32_restrictederrorinfo")]
#[inline]
pub unsafe fn RoReportUnhandledError<P0>(prestrictederrorinfo: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::restrictederrorinfo::IRestrictedErrorInfo>,
{
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoReportUnhandledError(prestrictederrorinfo : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RoReportUnhandledError(prestrictederrorinfo.param().abi()) }
}
#[cfg(feature = "Win32_restrictederrorinfo")]
#[inline]
pub unsafe fn RoResolveRestrictedErrorInfoReference<P0>(reference: P0) -> windows_core::Result<super::restrictederrorinfo::IRestrictedErrorInfo>
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoResolveRestrictedErrorInfoReference(reference : windows_core::PCWSTR, pprestrictederrorinfo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoResolveRestrictedErrorInfoReference(reference.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
    }
}
#[inline]
pub unsafe fn RoRevokeActivationFactories(cookie: *const _RO_REGISTRATION_COOKIE) {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoRevokeActivationFactories(cookie : *const _RO_REGISTRATION_COOKIE));
    unsafe { RoRevokeActivationFactories(cookie) }
}
#[inline]
pub unsafe fn RoSetErrorReportingFlags(flags: u32) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoSetErrorReportingFlags(flags : u32) -> windows_core::HRESULT);
    unsafe { RoSetErrorReportingFlags(flags) }
}
#[inline]
pub unsafe fn RoTransformError(olderror: windows_core::HRESULT, newerror: windows_core::HRESULT, message: &windows_core::HSTRING) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoTransformError(olderror : windows_core::HRESULT, newerror : windows_core::HRESULT, message : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { RoTransformError(olderror, newerror, core::mem::transmute_copy(message)) }
}
#[inline]
pub unsafe fn RoTransformErrorW<P3>(olderror: windows_core::HRESULT, newerror: windows_core::HRESULT, cchmax: u32, message: P3) -> windows_core::BOOL
where
    P3: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoTransformErrorW(olderror : windows_core::HRESULT, newerror : windows_core::HRESULT, cchmax : u32, message : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { RoTransformErrorW(olderror, newerror, cchmax, message.param().abi()) }
}
#[inline]
pub unsafe fn RoUninitialize() {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoUninitialize());
    unsafe { RoUninitialize() }
}
#[inline]
pub unsafe fn RoUnregisterForApartmentShutdown(regcookie: APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoUnregisterForApartmentShutdown(regcookie : APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> windows_core::HRESULT);
    unsafe { RoUnregisterForApartmentShutdown(regcookie) }
}
#[cfg(feature = "Win32_restrictederrorinfo")]
#[inline]
pub unsafe fn SetRestrictedErrorInfo<P0>(prestrictederrorinfo: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::restrictederrorinfo::IRestrictedErrorInfo>,
{
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn SetRestrictedErrorInfo(prestrictederrorinfo : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SetRestrictedErrorInfo(prestrictederrorinfo.param().abi()) }
}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct APARTMENT_SHUTDOWN_REGISTRATION_COOKIE(pub *mut core::ffi::c_void);
impl APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for APARTMENT_SHUTDOWN_REGISTRATION_COOKIE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub const EXCEPTION_RO_ORIGINATEERROR: u32 = 1074266625;
pub const EXCEPTION_RO_TRANSFORMERROR: u32 = 1074266626;
pub const ForceExceptions: RoErrorReportingFlags = 2;
windows_core::imp::define_interface!(IBufferByteAccess, IBufferByteAccess_Vtbl, 0x905a0fef_bc53_11df_8c49_001e4fc686da);
windows_core::imp::interface_hierarchy!(IBufferByteAccess, windows_core::IUnknown);
impl IBufferByteAccess {
    #[cfg(feature = "Win32_rpc")]
    pub unsafe fn Buffer(&self) -> windows_core::Result<*mut super::rpc::byte> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Buffer)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferByteAccess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "Win32_rpc")]
    pub Buffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::rpc::byte) -> windows_core::HRESULT,
    #[cfg(not(feature = "Win32_rpc"))]
    Buffer: usize,
}
#[cfg(feature = "Win32_rpc")]
pub trait IBufferByteAccess_Impl: windows_core::IUnknownImpl {
    fn Buffer(&self) -> windows_core::Result<*mut super::rpc::byte>;
}
#[cfg(feature = "Win32_rpc")]
impl IBufferByteAccess_Vtbl {
    pub const fn new<Identity: IBufferByteAccess_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Buffer<Identity: IBufferByteAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::rpc::byte) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IBufferByteAccess_Impl::Buffer(this) {
                    Ok(ok__) => {
                        value.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Buffer: Buffer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBufferByteAccess as windows_core::Interface>::IID
    }
}
#[cfg(feature = "Win32_rpc")]
impl windows_core::RuntimeName for IBufferByteAccess {}
windows_core::imp::define_interface!(IRoMetaDataLocator, IRoMetaDataLocator_Vtbl);
impl IRoMetaDataLocator {
    pub unsafe fn Locate<P0, P1>(&self, nameelement: P0, metadatadestination: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<IRoSimpleMetaDataBuilder>,
    {
        unsafe { (windows_core::Interface::vtable(self).Locate)(windows_core::Interface::as_raw(self), nameelement.param().abi(), metadatadestination.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoMetaDataLocator_Vtbl {
    pub Locate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRoMetaDataLocator_Impl {
    fn Locate(&self, nameelement: &windows_core::PCWSTR, metadatadestination: windows_core::Ref<IRoSimpleMetaDataBuilder>) -> windows_core::Result<()>;
}
impl IRoMetaDataLocator_Vtbl {
    pub const fn new<Identity: IRoMetaDataLocator_Impl>() -> Self {
        unsafe extern "system" fn Locate<Identity: IRoMetaDataLocator_Impl>(this: *mut core::ffi::c_void, nameelement: windows_core::PCWSTR, metadatadestination: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IRoMetaDataLocator_Impl::Locate(this, core::mem::transmute(&nameelement), core::mem::transmute_copy(&metadatadestination)).into()
            }
        }
        Self { Locate: Locate::<Identity> }
    }
}
struct IRoMetaDataLocator_ImplVtbl<T: IRoMetaDataLocator_Impl>(core::marker::PhantomData<T>);
impl<T: IRoMetaDataLocator_Impl> IRoMetaDataLocator_ImplVtbl<T> {
    const VTABLE: IRoMetaDataLocator_Vtbl = IRoMetaDataLocator_Vtbl::new::<T>();
}
impl IRoMetaDataLocator {
    pub fn new<'a, T: IRoMetaDataLocator_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IRoMetaDataLocator_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
windows_core::imp::define_interface!(IRoSimpleMetaDataBuilder, IRoSimpleMetaDataBuilder_Vtbl);
impl IRoSimpleMetaDataBuilder {
    pub unsafe fn SetWinRtInterface(&self, iid: windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWinRtInterface)(windows_core::Interface::as_raw(self), core::mem::transmute(iid)) }
    }
    pub unsafe fn SetDelegate(&self, iid: windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDelegate)(windows_core::Interface::as_raw(self), core::mem::transmute(iid)) }
    }
    pub unsafe fn SetInterfaceGroupSimpleDefault<P0, P1>(&self, name: P0, defaultinterfacename: P1, defaultinterfaceiid: Option<*const windows_core::GUID>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInterfaceGroupSimpleDefault)(windows_core::Interface::as_raw(self), name.param().abi(), defaultinterfacename.param().abi(), defaultinterfaceiid.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetInterfaceGroupParameterizedDefault<P0>(&self, name: P0, defaultinterfacenameelements: &[windows_core::PCWSTR]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInterfaceGroupParameterizedDefault)(windows_core::Interface::as_raw(self), name.param().abi(), defaultinterfacenameelements.len().try_into().unwrap(), core::mem::transmute(defaultinterfacenameelements.as_ptr())) }
    }
    pub unsafe fn SetRuntimeClassSimpleDefault<P0, P1>(&self, name: P0, defaultinterfacename: P1, defaultinterfaceiid: Option<*const windows_core::GUID>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRuntimeClassSimpleDefault)(windows_core::Interface::as_raw(self), name.param().abi(), defaultinterfacename.param().abi(), defaultinterfaceiid.unwrap_or(core::mem::zeroed()) as _) }
    }
    pub unsafe fn SetRuntimeClassParameterizedDefault<P0>(&self, name: P0, defaultinterfacenameelements: &[windows_core::PCWSTR]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRuntimeClassParameterizedDefault)(windows_core::Interface::as_raw(self), name.param().abi(), defaultinterfacenameelements.len().try_into().unwrap(), core::mem::transmute(defaultinterfacenameelements.as_ptr())) }
    }
    pub unsafe fn SetStruct<P0>(&self, name: P0, fieldtypenames: &[windows_core::PCWSTR]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStruct)(windows_core::Interface::as_raw(self), name.param().abi(), fieldtypenames.len().try_into().unwrap(), core::mem::transmute(fieldtypenames.as_ptr())) }
    }
    pub unsafe fn SetEnum<P0, P1>(&self, name: P0, basetype: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetEnum)(windows_core::Interface::as_raw(self), name.param().abi(), basetype.param().abi()) }
    }
    pub unsafe fn SetParameterizedInterface(&self, piid: windows_core::GUID, numargs: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetParameterizedInterface)(windows_core::Interface::as_raw(self), core::mem::transmute(piid), numargs) }
    }
    pub unsafe fn SetParameterizedDelegate(&self, piid: windows_core::GUID, numargs: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetParameterizedDelegate)(windows_core::Interface::as_raw(self), core::mem::transmute(piid), numargs) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoSimpleMetaDataBuilder_Vtbl {
    pub SetWinRtInterface: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub SetDelegate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub SetInterfaceGroupSimpleDefault: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetInterfaceGroupParameterizedDefault: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *const windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetRuntimeClassSimpleDefault: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
    pub SetRuntimeClassParameterizedDefault: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *const windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetStruct: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, u32, *const windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetEnum: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetParameterizedInterface: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32) -> windows_core::HRESULT,
    pub SetParameterizedDelegate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, u32) -> windows_core::HRESULT,
}
pub trait IRoSimpleMetaDataBuilder_Impl {
    fn SetWinRtInterface(&self, iid: &windows_core::GUID) -> windows_core::Result<()>;
    fn SetDelegate(&self, iid: &windows_core::GUID) -> windows_core::Result<()>;
    fn SetInterfaceGroupSimpleDefault(&self, name: &windows_core::PCWSTR, defaultinterfacename: &windows_core::PCWSTR, defaultinterfaceiid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetInterfaceGroupParameterizedDefault(&self, name: &windows_core::PCWSTR, elementcount: u32, defaultinterfacenameelements: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetRuntimeClassSimpleDefault(&self, name: &windows_core::PCWSTR, defaultinterfacename: &windows_core::PCWSTR, defaultinterfaceiid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetRuntimeClassParameterizedDefault(&self, name: &windows_core::PCWSTR, elementcount: u32, defaultinterfacenameelements: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetStruct(&self, name: &windows_core::PCWSTR, numfields: u32, fieldtypenames: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetEnum(&self, name: &windows_core::PCWSTR, basetype: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetParameterizedInterface(&self, piid: &windows_core::GUID, numargs: u32) -> windows_core::Result<()>;
    fn SetParameterizedDelegate(&self, piid: &windows_core::GUID, numargs: u32) -> windows_core::Result<()>;
}
impl IRoSimpleMetaDataBuilder_Vtbl {
    pub const fn new<Identity: IRoSimpleMetaDataBuilder_Impl>() -> Self {
        unsafe extern "system" fn SetWinRtInterface<Identity: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, iid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IRoSimpleMetaDataBuilder_Impl::SetWinRtInterface(this, core::mem::transmute(&iid)).into()
            }
        }
        unsafe extern "system" fn SetDelegate<Identity: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, iid: windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IRoSimpleMetaDataBuilder_Impl::SetDelegate(this, core::mem::transmute(&iid)).into()
            }
        }
        unsafe extern "system" fn SetInterfaceGroupSimpleDefault<Identity: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, defaultinterfacename: windows_core::PCWSTR, defaultinterfaceiid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IRoSimpleMetaDataBuilder_Impl::SetInterfaceGroupSimpleDefault(this, core::mem::transmute(&name), core::mem::transmute(&defaultinterfacename), core::mem::transmute_copy(&defaultinterfaceiid)).into()
            }
        }
        unsafe extern "system" fn SetInterfaceGroupParameterizedDefault<Identity: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, elementcount: u32, defaultinterfacenameelements: *const windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IRoSimpleMetaDataBuilder_Impl::SetInterfaceGroupParameterizedDefault(this, core::mem::transmute(&name), core::mem::transmute_copy(&elementcount), core::mem::transmute_copy(&defaultinterfacenameelements)).into()
            }
        }
        unsafe extern "system" fn SetRuntimeClassSimpleDefault<Identity: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, defaultinterfacename: windows_core::PCWSTR, defaultinterfaceiid: *const windows_core::GUID) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IRoSimpleMetaDataBuilder_Impl::SetRuntimeClassSimpleDefault(this, core::mem::transmute(&name), core::mem::transmute(&defaultinterfacename), core::mem::transmute_copy(&defaultinterfaceiid)).into()
            }
        }
        unsafe extern "system" fn SetRuntimeClassParameterizedDefault<Identity: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, elementcount: u32, defaultinterfacenameelements: *const windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IRoSimpleMetaDataBuilder_Impl::SetRuntimeClassParameterizedDefault(this, core::mem::transmute(&name), core::mem::transmute_copy(&elementcount), core::mem::transmute_copy(&defaultinterfacenameelements)).into()
            }
        }
        unsafe extern "system" fn SetStruct<Identity: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, numfields: u32, fieldtypenames: *const windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IRoSimpleMetaDataBuilder_Impl::SetStruct(this, core::mem::transmute(&name), core::mem::transmute_copy(&numfields), core::mem::transmute_copy(&fieldtypenames)).into()
            }
        }
        unsafe extern "system" fn SetEnum<Identity: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, basetype: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IRoSimpleMetaDataBuilder_Impl::SetEnum(this, core::mem::transmute(&name), core::mem::transmute(&basetype)).into()
            }
        }
        unsafe extern "system" fn SetParameterizedInterface<Identity: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, piid: windows_core::GUID, numargs: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IRoSimpleMetaDataBuilder_Impl::SetParameterizedInterface(this, core::mem::transmute(&piid), core::mem::transmute_copy(&numargs)).into()
            }
        }
        unsafe extern "system" fn SetParameterizedDelegate<Identity: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, piid: windows_core::GUID, numargs: u32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IRoSimpleMetaDataBuilder_Impl::SetParameterizedDelegate(this, core::mem::transmute(&piid), core::mem::transmute_copy(&numargs)).into()
            }
        }
        Self {
            SetWinRtInterface: SetWinRtInterface::<Identity>,
            SetDelegate: SetDelegate::<Identity>,
            SetInterfaceGroupSimpleDefault: SetInterfaceGroupSimpleDefault::<Identity>,
            SetInterfaceGroupParameterizedDefault: SetInterfaceGroupParameterizedDefault::<Identity>,
            SetRuntimeClassSimpleDefault: SetRuntimeClassSimpleDefault::<Identity>,
            SetRuntimeClassParameterizedDefault: SetRuntimeClassParameterizedDefault::<Identity>,
            SetStruct: SetStruct::<Identity>,
            SetEnum: SetEnum::<Identity>,
            SetParameterizedInterface: SetParameterizedInterface::<Identity>,
            SetParameterizedDelegate: SetParameterizedDelegate::<Identity>,
        }
    }
}
struct IRoSimpleMetaDataBuilder_ImplVtbl<T: IRoSimpleMetaDataBuilder_Impl>(core::marker::PhantomData<T>);
impl<T: IRoSimpleMetaDataBuilder_Impl> IRoSimpleMetaDataBuilder_ImplVtbl<T> {
    const VTABLE: IRoSimpleMetaDataBuilder_Vtbl = IRoSimpleMetaDataBuilder_Vtbl::new::<T>();
}
impl IRoSimpleMetaDataBuilder {
    pub fn new<'a, T: IRoSimpleMetaDataBuilder_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IRoSimpleMetaDataBuilder_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = core::mem::ManuallyDrop::new(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(&this.vtable)) }
    }
}
pub const MAX_ERROR_MESSAGE_CHARS: u32 = 512;
pub const None: RoErrorReportingFlags = 0;
#[cfg(feature = "Win32_activation")]
pub type PFNGETACTIVATIONFACTORY = Option<unsafe extern "system" fn(param0: windows_core::Ref<windows_core::HSTRING>, param1: windows_core::OutRef<super::activation::IActivationFactory>) -> windows_core::HRESULT>;
pub type PINSPECT_MEMORY_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, readaddress: usize, length: u32, buffer: *mut u8) -> windows_core::HRESULT>;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ROPARAMIIDHANDLE(pub *mut core::ffi::c_void);
impl ROPARAMIIDHANDLE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for ROPARAMIIDHANDLE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RO_ERROR_REPORTING_FLAGS = u32;
pub const RO_ERROR_REPORTING_FORCEEXCEPTIONS: RO_ERROR_REPORTING_FLAGS = 2;
pub const RO_ERROR_REPORTING_NONE: RO_ERROR_REPORTING_FLAGS = 0;
pub const RO_ERROR_REPORTING_SUPPRESSEXCEPTIONS: RO_ERROR_REPORTING_FLAGS = 1;
pub const RO_ERROR_REPORTING_SUPPRESSSETERRORINFO: RO_ERROR_REPORTING_FLAGS = 8;
pub const RO_ERROR_REPORTING_USESETERRORINFO: RO_ERROR_REPORTING_FLAGS = 4;
pub const RO_INIT_MULTITHREADED: RO_INIT_TYPE = 1;
pub const RO_INIT_SINGLETHREADED: RO_INIT_TYPE = 0;
pub type RO_INIT_TYPE = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RO_REGISTRATION_COOKIE(pub *mut _RO_REGISTRATION_COOKIE);
impl RO_REGISTRATION_COOKIE {
    pub fn is_invalid(&self) -> bool {
        self.0.is_null()
    }
}
impl Default for RO_REGISTRATION_COOKIE {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type RoErrorReportingFlags = u32;
pub const SuppressExceptions: RoErrorReportingFlags = 1;
pub const SuppressSetErrorInfo: RoErrorReportingFlags = 8;
pub const UseSetErrorInfo: RoErrorReportingFlags = 4;
#[repr(C, align(1))]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct _RO_REGISTRATION_COOKIE(pub u8);
