#[cfg(feature = "restrictederrorinfo")]
#[inline]
pub unsafe fn GetRestrictedErrorInfo(pprestrictederrorinfo: *mut Option<super::IRestrictedErrorInfo>) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn GetRestrictedErrorInfo(pprestrictederrorinfo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { GetRestrictedErrorInfo(core::mem::transmute(pprestrictederrorinfo)) }
}
#[inline]
pub unsafe fn IsErrorPropagationEnabled() -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn IsErrorPropagationEnabled() -> windows_core::BOOL);
    unsafe { IsErrorPropagationEnabled() }
}
#[cfg(all(feature = "guiddef", feature = "minwindef"))]
#[inline]
pub unsafe fn MetaDataGetDispenser(rclsid: *const super::IID, riid: *const super::IID, ppv: *mut super::LPVOID) -> windows_core::HRESULT {
    windows_core::link!("rometadata.dll" "system" fn MetaDataGetDispenser(rclsid : *const super::IID, riid : *const super::IID, ppv : *mut super::LPVOID) -> windows_core::HRESULT);
    unsafe { MetaDataGetDispenser(rclsid, riid, ppv as _) }
}
#[inline]
pub unsafe fn RoActivateInstance(activatableclassid: &windows_core::HSTRING) -> windows_core::Result<windows_core::IInspectable> {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoActivateInstance(activatableclassid : *mut core::ffi::c_void, instance : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoActivateInstance(core::mem::transmute_copy(activatableclassid), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
#[cfg(feature = "guiddef")]
#[inline]
pub unsafe fn RoGetActivationFactory<T>(activatableclassid: &windows_core::HSTRING) -> windows_core::Result<T>
where
    T: windows_core::Interface,
{
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoGetActivationFactory(activatableclassid : *mut core::ffi::c_void, iid : *const super::IID, factory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    let mut result__ = core::ptr::null_mut();
    unsafe { RoGetActivationFactory(core::mem::transmute_copy(activatableclassid), &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn RoGetApartmentIdentifier(apartmentidentifier: *mut super::UINT64) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoGetApartmentIdentifier(apartmentidentifier : *mut super::UINT64) -> windows_core::HRESULT);
    unsafe { RoGetApartmentIdentifier(apartmentidentifier as _) }
}
#[cfg(feature = "objidlbase")]
#[inline]
pub unsafe fn RoGetBufferMarshaler(buffermarshaler: *mut Option<super::IMarshal>) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-robuffer-l1-1-0.dll" "system" fn RoGetBufferMarshaler(buffermarshaler : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RoGetBufferMarshaler(core::mem::transmute(buffermarshaler)) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn RoGetErrorReportingFlags(pflags: *mut super::UINT32) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoGetErrorReportingFlags(pflags : *mut super::UINT32) -> windows_core::HRESULT);
    unsafe { RoGetErrorReportingFlags(pflags as _) }
}
#[cfg(feature = "restrictederrorinfo")]
#[inline]
pub unsafe fn RoGetMatchingRestrictedErrorInfo(hrin: windows_core::HRESULT) -> windows_core::Result<super::IRestrictedErrorInfo> {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoGetMatchingRestrictedErrorInfo(hrin : windows_core::HRESULT, pprestrictederrorinfo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        RoGetMatchingRestrictedErrorInfo(hrin, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
    }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn RoGetServerActivatableClasses(servername: &windows_core::HSTRING, activatableclassids: *mut *mut windows_core::HSTRING, count: *mut super::DWORD) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-registration-l1-1-0.dll" "system" fn RoGetServerActivatableClasses(servername : *mut core::ffi::c_void, activatableclassids : *mut *mut *mut core::ffi::c_void, count : *mut super::DWORD) -> windows_core::HRESULT);
    unsafe { RoGetServerActivatableClasses(core::mem::transmute_copy(servername), activatableclassids as _, count as _) }
}
#[inline]
pub unsafe fn RoInitialize(inittype: RO_INIT_TYPE) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoInitialize(inittype : RO_INIT_TYPE) -> windows_core::HRESULT);
    unsafe { RoInitialize(inittype) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RoInspectCapturedStackBackTrace(targeterrorinfoaddress: super::UINT_PTR, machine: super::USHORT, readmemorycallback: PINSPECT_MEMORY_CALLBACK, context: Option<super::PVOID>, framecount: *mut super::UINT32, targetbacktraceaddress: *mut super::UINT_PTR) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoInspectCapturedStackBackTrace(targeterrorinfoaddress : super::UINT_PTR, machine : super::USHORT, readmemorycallback : PINSPECT_MEMORY_CALLBACK, context : super::PVOID, framecount : *mut super::UINT32, targetbacktraceaddress : *mut super::UINT_PTR) -> windows_core::HRESULT);
    unsafe { RoInspectCapturedStackBackTrace(targeterrorinfoaddress, machine, readmemorycallback, context.unwrap_or(core::mem::zeroed()) as _, framecount as _, targetbacktraceaddress as _) }
}
#[cfg(all(feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn RoInspectThreadErrorInfo(targettebaddress: super::UINT_PTR, machine: super::USHORT, readmemorycallback: PINSPECT_MEMORY_CALLBACK, context: Option<super::PVOID>, targeterrorinfoaddress: *mut super::UINT_PTR) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoInspectThreadErrorInfo(targettebaddress : super::UINT_PTR, machine : super::USHORT, readmemorycallback : PINSPECT_MEMORY_CALLBACK, context : super::PVOID, targeterrorinfoaddress : *mut super::UINT_PTR) -> windows_core::HRESULT);
    unsafe { RoInspectThreadErrorInfo(targettebaddress, machine, readmemorycallback, context.unwrap_or(core::mem::zeroed()) as _, targeterrorinfoaddress as _) }
}
#[inline]
pub unsafe fn RoOriginateError(error: windows_core::HRESULT, message: &windows_core::HSTRING) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoOriginateError(error : windows_core::HRESULT, message : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { RoOriginateError(error, core::mem::transmute_copy(message)) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn RoOriginateErrorW(error: windows_core::HRESULT, cchmax: super::UINT, message: windows_core::PCWSTR) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoOriginateErrorW(error : windows_core::HRESULT, cchmax : super::UINT, message : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { RoOriginateErrorW(error, cchmax, message) }
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
#[cfg(all(feature = "activation", feature = "shared"))]
#[inline]
pub unsafe fn RoRegisterActivationFactories(activatableclassids: *const windows_core::HSTRING, activationfactorycallbacks: *const PFNGETACTIVATIONFACTORY, count: super::UINT32, cookie: *mut RO_REGISTRATION_COOKIE) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoRegisterActivationFactories(activatableclassids : *const *mut core::ffi::c_void, activationfactorycallbacks : *const PFNGETACTIVATIONFACTORY, count : super::UINT32, cookie : *mut RO_REGISTRATION_COOKIE) -> windows_core::HRESULT);
    unsafe { RoRegisterActivationFactories(core::mem::transmute(activatableclassids), activationfactorycallbacks, count, cookie as _) }
}
#[cfg(all(feature = "objidl", feature = "shared"))]
#[inline]
pub unsafe fn RoRegisterForApartmentShutdown<P0>(callbackobject: P0, apartmentidentifier: *mut super::UINT64, regcookie: *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IApartmentShutdown>,
{
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoRegisterForApartmentShutdown(callbackobject : *mut core::ffi::c_void, apartmentidentifier : *mut super::UINT64, regcookie : *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE) -> windows_core::HRESULT);
    unsafe { RoRegisterForApartmentShutdown(callbackobject.param().abi(), apartmentidentifier as _, regcookie as _) }
}
#[cfg(feature = "restrictederrorinfo")]
#[inline]
pub unsafe fn RoReportFailedDelegate<P0, P1>(punkdelegate: P0, prestrictederrorinfo: P1) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::IUnknown>,
    P1: windows_core::Param<super::IRestrictedErrorInfo>,
{
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoReportFailedDelegate(punkdelegate : *mut core::ffi::c_void, prestrictederrorinfo : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RoReportFailedDelegate(punkdelegate.param().abi(), prestrictederrorinfo.param().abi()) }
}
#[cfg(feature = "restrictederrorinfo")]
#[inline]
pub unsafe fn RoReportUnhandledError<P0>(prestrictederrorinfo: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IRestrictedErrorInfo>,
{
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-1.dll" "system" fn RoReportUnhandledError(prestrictederrorinfo : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RoReportUnhandledError(prestrictederrorinfo.param().abi()) }
}
#[cfg(feature = "restrictederrorinfo")]
#[inline]
pub unsafe fn RoResolveRestrictedErrorInfoReference<P0>(reference: P0, pprestrictederrorinfo: *mut Option<super::IRestrictedErrorInfo>) -> windows_core::HRESULT
where
    P0: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoResolveRestrictedErrorInfoReference(reference : windows_core::PCWSTR, pprestrictederrorinfo : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { RoResolveRestrictedErrorInfoReference(reference.param().abi(), core::mem::transmute(pprestrictederrorinfo)) }
}
#[inline]
pub unsafe fn RoRevokeActivationFactories(cookie: RO_REGISTRATION_COOKIE) {
    windows_core::link!("api-ms-win-core-winrt-l1-1-0.dll" "system" fn RoRevokeActivationFactories(cookie : RO_REGISTRATION_COOKIE));
    unsafe { RoRevokeActivationFactories(cookie) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn RoSetErrorReportingFlags(flags: super::UINT32) -> windows_core::HRESULT {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoSetErrorReportingFlags(flags : super::UINT32) -> windows_core::HRESULT);
    unsafe { RoSetErrorReportingFlags(flags) }
}
#[inline]
pub unsafe fn RoTransformError(olderror: windows_core::HRESULT, newerror: windows_core::HRESULT, message: &windows_core::HSTRING) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoTransformError(olderror : windows_core::HRESULT, newerror : windows_core::HRESULT, message : *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { RoTransformError(olderror, newerror, core::mem::transmute_copy(message)) }
}
#[cfg(feature = "shared")]
#[inline]
pub unsafe fn RoTransformErrorW(olderror: windows_core::HRESULT, newerror: windows_core::HRESULT, cchmax: super::UINT, message: windows_core::PCWSTR) -> windows_core::BOOL {
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn RoTransformErrorW(olderror : windows_core::HRESULT, newerror : windows_core::HRESULT, cchmax : super::UINT, message : windows_core::PCWSTR) -> windows_core::BOOL);
    unsafe { RoTransformErrorW(olderror, newerror, cchmax, message) }
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
#[cfg(feature = "restrictederrorinfo")]
#[inline]
pub unsafe fn SetRestrictedErrorInfo<P0>(prestrictederrorinfo: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IRestrictedErrorInfo>,
{
    windows_core::link!("api-ms-win-core-winrt-error-l1-1-0.dll" "system" fn SetRestrictedErrorInfo(prestrictederrorinfo : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { SetRestrictedErrorInfo(prestrictederrorinfo.param().abi()) }
}
pub type APARTMENT_SHUTDOWN_REGISTRATION_COOKIE = *mut APARTMENT_SHUTDOWN_REGISTRATION_COOKIE__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct APARTMENT_SHUTDOWN_REGISTRATION_COOKIE__ {
    pub unused: i32,
}
#[cfg(feature = "shared")]
pub const EXCEPTION_RO_ORIGINATEERROR: super::DWORD = 1074266625;
#[cfg(feature = "shared")]
pub const EXCEPTION_RO_TRANSFORMERROR: super::DWORD = 1074266626;
windows_core::imp::define_interface!(IBufferByteAccess, IBufferByteAccess_Vtbl, 0x905a0fef_bc53_11df_8c49_001e4fc686da);
windows_core::imp::interface_hierarchy!(IBufferByteAccess, windows_core::IUnknown);
impl IBufferByteAccess {
    #[cfg(feature = "rpc")]
    pub unsafe fn Buffer(&self, value: *mut *mut super::byte) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Buffer)(windows_core::Interface::as_raw(self), value as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IBufferByteAccess_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "rpc")]
    pub Buffer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::byte) -> windows_core::HRESULT,
    #[cfg(not(feature = "rpc"))]
    Buffer: usize,
}
#[cfg(feature = "rpc")]
pub trait IBufferByteAccess_Impl: windows_core::IUnknownImpl {
    fn Buffer(&self, value: *mut *mut super::byte) -> windows_core::Result<()>;
}
#[cfg(feature = "rpc")]
impl IBufferByteAccess_Vtbl {
    pub const fn new<Identity: IBufferByteAccess_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Buffer<Identity: IBufferByteAccess_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: *mut *mut super::byte) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IBufferByteAccess_Impl::Buffer(this, core::mem::transmute_copy(&value)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Buffer: Buffer::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IBufferByteAccess as windows_core::Interface>::IID
    }
}
#[cfg(feature = "rpc")]
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
        let this = windows_core::imp::box_into_raw(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(this)) }
    }
}
windows_core::imp::define_interface!(IRoSimpleMetaDataBuilder, IRoSimpleMetaDataBuilder_Vtbl);
impl IRoSimpleMetaDataBuilder {
    pub unsafe fn SetWinRtInterface(&self, iid: windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWinRtInterface)(windows_core::Interface::as_raw(self), iid) }
    }
    pub unsafe fn SetDelegate(&self, iid: windows_core::GUID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDelegate)(windows_core::Interface::as_raw(self), iid) }
    }
    pub unsafe fn SetInterfaceGroupSimpleDefault<P0, P1>(&self, name: P0, defaultinterfacename: P1, defaultinterfaceiid: Option<*const windows_core::GUID>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInterfaceGroupSimpleDefault)(windows_core::Interface::as_raw(self), name.param().abi(), defaultinterfacename.param().abi(), defaultinterfaceiid.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetInterfaceGroupParameterizedDefault<P0>(&self, name: P0, defaultinterfacenameelements: &[windows_core::PCWSTR]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetInterfaceGroupParameterizedDefault)(windows_core::Interface::as_raw(self), name.param().abi(), defaultinterfacenameelements.len().try_into().unwrap(), defaultinterfacenameelements.as_ptr()) }
    }
    pub unsafe fn SetRuntimeClassSimpleDefault<P0, P1>(&self, name: P0, defaultinterfacename: P1, defaultinterfaceiid: Option<*const windows_core::GUID>) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRuntimeClassSimpleDefault)(windows_core::Interface::as_raw(self), name.param().abi(), defaultinterfacename.param().abi(), defaultinterfaceiid.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetRuntimeClassParameterizedDefault<P0>(&self, name: P0, defaultinterfacenameelements: &[windows_core::PCWSTR]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetRuntimeClassParameterizedDefault)(windows_core::Interface::as_raw(self), name.param().abi(), defaultinterfacenameelements.len().try_into().unwrap(), defaultinterfacenameelements.as_ptr()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetStruct<P0>(&self, name: P0, fieldtypenames: &[windows_core::PCWSTR]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetStruct)(windows_core::Interface::as_raw(self), name.param().abi(), fieldtypenames.len().try_into().unwrap(), fieldtypenames.as_ptr()) }
    }
    pub unsafe fn SetEnum<P0, P1>(&self, name: P0, basetype: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
        P1: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetEnum)(windows_core::Interface::as_raw(self), name.param().abi(), basetype.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetParameterizedInterface(&self, piid: windows_core::GUID, numargs: super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetParameterizedInterface)(windows_core::Interface::as_raw(self), piid, numargs) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn SetParameterizedDelegate(&self, piid: windows_core::GUID, numargs: super::UINT32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetParameterizedDelegate)(windows_core::Interface::as_raw(self), piid, numargs) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRoSimpleMetaDataBuilder_Vtbl {
    pub SetWinRtInterface: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub SetDelegate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID) -> windows_core::HRESULT,
    pub SetInterfaceGroupSimpleDefault: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetInterfaceGroupParameterizedDefault: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::UINT32, *const windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetInterfaceGroupParameterizedDefault: usize,
    pub SetRuntimeClassSimpleDefault: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR, *const windows_core::GUID) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetRuntimeClassParameterizedDefault: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::UINT32, *const windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetRuntimeClassParameterizedDefault: usize,
    #[cfg(feature = "shared")]
    pub SetStruct: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, super::UINT32, *const windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetStruct: usize,
    pub SetEnum: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub SetParameterizedInterface: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetParameterizedInterface: usize,
    #[cfg(feature = "shared")]
    pub SetParameterizedDelegate: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::GUID, super::UINT32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    SetParameterizedDelegate: usize,
}
#[cfg(feature = "shared")]
pub trait IRoSimpleMetaDataBuilder_Impl {
    fn SetWinRtInterface(&self, iid: &windows_core::GUID) -> windows_core::Result<()>;
    fn SetDelegate(&self, iid: &windows_core::GUID) -> windows_core::Result<()>;
    fn SetInterfaceGroupSimpleDefault(&self, name: &windows_core::PCWSTR, defaultinterfacename: &windows_core::PCWSTR, defaultinterfaceiid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetInterfaceGroupParameterizedDefault(&self, name: &windows_core::PCWSTR, elementcount: super::UINT32, defaultinterfacenameelements: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetRuntimeClassSimpleDefault(&self, name: &windows_core::PCWSTR, defaultinterfacename: &windows_core::PCWSTR, defaultinterfaceiid: *const windows_core::GUID) -> windows_core::Result<()>;
    fn SetRuntimeClassParameterizedDefault(&self, name: &windows_core::PCWSTR, elementcount: super::UINT32, defaultinterfacenameelements: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetStruct(&self, name: &windows_core::PCWSTR, numfields: super::UINT32, fieldtypenames: *const windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetEnum(&self, name: &windows_core::PCWSTR, basetype: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetParameterizedInterface(&self, piid: &windows_core::GUID, numargs: super::UINT32) -> windows_core::Result<()>;
    fn SetParameterizedDelegate(&self, piid: &windows_core::GUID, numargs: super::UINT32) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
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
        unsafe extern "system" fn SetInterfaceGroupParameterizedDefault<Identity: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, elementcount: super::UINT32, defaultinterfacenameelements: *const windows_core::PCWSTR) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetRuntimeClassParameterizedDefault<Identity: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, elementcount: super::UINT32, defaultinterfacenameelements: *const windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IRoSimpleMetaDataBuilder_Impl::SetRuntimeClassParameterizedDefault(this, core::mem::transmute(&name), core::mem::transmute_copy(&elementcount), core::mem::transmute_copy(&defaultinterfacenameelements)).into()
            }
        }
        unsafe extern "system" fn SetStruct<Identity: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, numfields: super::UINT32, fieldtypenames: *const windows_core::PCWSTR) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetParameterizedInterface<Identity: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, piid: windows_core::GUID, numargs: super::UINT32) -> windows_core::HRESULT {
            unsafe {
                let this = (this as *mut *mut core::ffi::c_void) as *const windows_core::ScopedHeap;
                let this = &*((*this).this as *const Identity);
                IRoSimpleMetaDataBuilder_Impl::SetParameterizedInterface(this, core::mem::transmute(&piid), core::mem::transmute_copy(&numargs)).into()
            }
        }
        unsafe extern "system" fn SetParameterizedDelegate<Identity: IRoSimpleMetaDataBuilder_Impl>(this: *mut core::ffi::c_void, piid: windows_core::GUID, numargs: super::UINT32) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
struct IRoSimpleMetaDataBuilder_ImplVtbl<T: IRoSimpleMetaDataBuilder_Impl>(core::marker::PhantomData<T>);
#[cfg(feature = "shared")]
impl<T: IRoSimpleMetaDataBuilder_Impl> IRoSimpleMetaDataBuilder_ImplVtbl<T> {
    const VTABLE: IRoSimpleMetaDataBuilder_Vtbl = IRoSimpleMetaDataBuilder_Vtbl::new::<T>();
}
#[cfg(feature = "shared")]
impl IRoSimpleMetaDataBuilder {
    pub fn new<'a, T: IRoSimpleMetaDataBuilder_Impl>(this: &'a T) -> windows_core::ScopedInterface<'a, Self> {
        let this = windows_core::ScopedHeap { vtable: &IRoSimpleMetaDataBuilder_ImplVtbl::<T>::VTABLE as *const _ as *const _, this: this as *const _ as *const _ };
        let this = windows_core::imp::box_into_raw(windows_core::imp::box_new(this));
        unsafe { windows_core::ScopedInterface::new(core::mem::transmute(this)) }
    }
}
pub const MAX_ERROR_MESSAGE_CHARS: i32 = 512;
#[cfg(feature = "activation")]
pub type PFNGETACTIVATIONFACTORY = Option<unsafe extern "system" fn(param0: windows_core::Ref<windows_core::HSTRING>, param1: windows_core::OutRef<super::IActivationFactory>) -> windows_core::HRESULT>;
#[cfg(feature = "shared")]
pub type PINSPECT_MEMORY_CALLBACK = Option<unsafe extern "system" fn(context: *const core::ffi::c_void, readaddress: super::UINT_PTR, length: super::UINT32, buffer: *mut super::BYTE) -> windows_core::HRESULT>;
pub type ROPARAMIIDHANDLE = *mut ROPARAMIIDHANDLE__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ROPARAMIIDHANDLE__ {
    pub unused: i32,
}
pub type RO_ERROR_REPORTING_FLAGS = i32;
pub const RO_ERROR_REPORTING_FORCEEXCEPTIONS: RO_ERROR_REPORTING_FLAGS = 2;
pub const RO_ERROR_REPORTING_NONE: RO_ERROR_REPORTING_FLAGS = 0;
pub const RO_ERROR_REPORTING_SUPPRESSEXCEPTIONS: RO_ERROR_REPORTING_FLAGS = 1;
pub const RO_ERROR_REPORTING_SUPPRESSSETERRORINFO: RO_ERROR_REPORTING_FLAGS = 8;
pub const RO_ERROR_REPORTING_USESETERRORINFO: RO_ERROR_REPORTING_FLAGS = 4;
pub const RO_INIT_MULTITHREADED: RO_INIT_TYPE = 1;
pub const RO_INIT_SINGLETHREADED: RO_INIT_TYPE = 0;
pub type RO_INIT_TYPE = i32;
pub type RO_REGISTRATION_COOKIE = *mut _RO_REGISTRATION_COOKIE;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct _RO_REGISTRATION_COOKIE(pub u8);
