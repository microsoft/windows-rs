#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsAttachLayerStorageFilter<P0, P1>(layerpath: P0, layerdata: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computestorage.dll""system" fn HcsAttachLayerStorageFilter ( layerpath : :: windows::core::PCWSTR , layerdata : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsAttachLayerStorageFilter(layerpath.into_param().abi(), layerdata.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCancelOperation<P0>(operation: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HCS_OPERATION>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsCancelOperation ( operation : HCS_OPERATION ) -> :: windows::core::HRESULT );
    HcsCancelOperation(operation.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCloseComputeSystem<P0>(computesystem: P0)
where
    P0: ::windows::core::IntoParam<HCS_SYSTEM>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsCloseComputeSystem ( computesystem : HCS_SYSTEM ) -> ( ) );
    HcsCloseComputeSystem(computesystem.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCloseOperation<P0>(operation: P0)
where
    P0: ::windows::core::IntoParam<HCS_OPERATION>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsCloseOperation ( operation : HCS_OPERATION ) -> ( ) );
    HcsCloseOperation(operation.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCloseProcess<P0>(process: P0)
where
    P0: ::windows::core::IntoParam<HCS_PROCESS>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsCloseProcess ( process : HCS_PROCESS ) -> ( ) );
    HcsCloseProcess(process.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCrashComputeSystem<P0, P1, P2>(computesystem: P0, operation: P1, options: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HCS_SYSTEM>,
    P1: ::windows::core::IntoParam<HCS_OPERATION>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsCrashComputeSystem ( computesystem : HCS_SYSTEM , operation : HCS_OPERATION , options : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsCrashComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn HcsCreateComputeSystem<P0, P1, P2>(id: P0, configuration: P1, operation: P2, securitydescriptor: ::core::option::Option<*const super::super::Security::SECURITY_DESCRIPTOR>) -> ::windows::core::Result<HCS_SYSTEM>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<HCS_OPERATION>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsCreateComputeSystem ( id : :: windows::core::PCWSTR , configuration : :: windows::core::PCWSTR , operation : HCS_OPERATION , securitydescriptor : *const super::super::Security:: SECURITY_DESCRIPTOR , computesystem : *mut HCS_SYSTEM ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<HCS_SYSTEM>();
    HcsCreateComputeSystem(id.into_param().abi(), configuration.into_param().abi(), operation.into_param().abi(), ::core::mem::transmute(securitydescriptor.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCreateComputeSystemInNamespace<P0, P1, P2, P3>(idnamespace: P0, id: P1, configuration: P2, operation: P3, options: ::core::option::Option<*const HCS_CREATE_OPTIONS>) -> ::windows::core::Result<HCS_SYSTEM>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<HCS_OPERATION>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsCreateComputeSystemInNamespace ( idnamespace : :: windows::core::PCWSTR , id : :: windows::core::PCWSTR , configuration : :: windows::core::PCWSTR , operation : HCS_OPERATION , options : *const HCS_CREATE_OPTIONS , computesystem : *mut HCS_SYSTEM ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<HCS_SYSTEM>();
    HcsCreateComputeSystemInNamespace(idnamespace.into_param().abi(), id.into_param().abi(), configuration.into_param().abi(), operation.into_param().abi(), ::core::mem::transmute(options.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCreateEmptyGuestStateFile<P0>(gueststatefilepath: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsCreateEmptyGuestStateFile ( gueststatefilepath : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsCreateEmptyGuestStateFile(gueststatefilepath.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCreateEmptyRuntimeStateFile<P0>(runtimestatefilepath: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsCreateEmptyRuntimeStateFile ( runtimestatefilepath : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsCreateEmptyRuntimeStateFile(runtimestatefilepath.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsCreateOperation(context: ::core::option::Option<*const ::core::ffi::c_void>, callback: HCS_OPERATION_COMPLETION) -> HCS_OPERATION {
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsCreateOperation ( context : *const ::core::ffi::c_void , callback : HCS_OPERATION_COMPLETION ) -> HCS_OPERATION );
    HcsCreateOperation(::core::mem::transmute(context.unwrap_or(::std::ptr::null())), callback)
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
#[inline]
pub unsafe fn HcsCreateProcess<P0, P1, P2>(computesystem: P0, processparameters: P1, operation: P2, securitydescriptor: ::core::option::Option<*const super::super::Security::SECURITY_DESCRIPTOR>) -> ::windows::core::Result<HCS_PROCESS>
where
    P0: ::windows::core::IntoParam<HCS_SYSTEM>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<HCS_OPERATION>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsCreateProcess ( computesystem : HCS_SYSTEM , processparameters : :: windows::core::PCWSTR , operation : HCS_OPERATION , securitydescriptor : *const super::super::Security:: SECURITY_DESCRIPTOR , process : *mut HCS_PROCESS ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<HCS_PROCESS>();
    HcsCreateProcess(computesystem.into_param().abi(), processparameters.into_param().abi(), operation.into_param().abi(), ::core::mem::transmute(securitydescriptor.unwrap_or(::std::ptr::null())), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsDestroyLayer<P0>(layerpath: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computestorage.dll""system" fn HcsDestroyLayer ( layerpath : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsDestroyLayer(layerpath.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsDetachLayerStorageFilter<P0>(layerpath: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computestorage.dll""system" fn HcsDetachLayerStorageFilter ( layerpath : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsDetachLayerStorageFilter(layerpath.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsEnumerateComputeSystems<P0, P1>(query: P0, operation: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<HCS_OPERATION>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsEnumerateComputeSystems ( query : :: windows::core::PCWSTR , operation : HCS_OPERATION ) -> :: windows::core::HRESULT );
    HcsEnumerateComputeSystems(query.into_param().abi(), operation.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsEnumerateComputeSystemsInNamespace<P0, P1, P2>(idnamespace: P0, query: P1, operation: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<HCS_OPERATION>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsEnumerateComputeSystemsInNamespace ( idnamespace : :: windows::core::PCWSTR , query : :: windows::core::PCWSTR , operation : HCS_OPERATION ) -> :: windows::core::HRESULT );
    HcsEnumerateComputeSystemsInNamespace(idnamespace.into_param().abi(), query.into_param().abi(), operation.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsExportLayer<P0, P1, P2, P3>(layerpath: P0, exportfolderpath: P1, layerdata: P2, options: P3) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computestorage.dll""system" fn HcsExportLayer ( layerpath : :: windows::core::PCWSTR , exportfolderpath : :: windows::core::PCWSTR , layerdata : :: windows::core::PCWSTR , options : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsExportLayer(layerpath.into_param().abi(), exportfolderpath.into_param().abi(), layerdata.into_param().abi(), options.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsExportLegacyWritableLayer<P0, P1, P2, P3>(writablelayermountpath: P0, writablelayerfolderpath: P1, exportfolderpath: P2, layerdata: P3) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computestorage.dll""system" fn HcsExportLegacyWritableLayer ( writablelayermountpath : :: windows::core::PCWSTR , writablelayerfolderpath : :: windows::core::PCWSTR , exportfolderpath : :: windows::core::PCWSTR , layerdata : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsExportLegacyWritableLayer(writablelayermountpath.into_param().abi(), writablelayerfolderpath.into_param().abi(), exportfolderpath.into_param().abi(), layerdata.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsFormatWritableLayerVhd<P0>(vhdhandle: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "computestorage.dll""system" fn HcsFormatWritableLayerVhd ( vhdhandle : super::super::Foundation:: HANDLE ) -> :: windows::core::HRESULT );
    HcsFormatWritableLayerVhd(vhdhandle.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetComputeSystemFromOperation<P0>(operation: P0) -> HCS_SYSTEM
where
    P0: ::windows::core::IntoParam<HCS_OPERATION>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsGetComputeSystemFromOperation ( operation : HCS_OPERATION ) -> HCS_SYSTEM );
    HcsGetComputeSystemFromOperation(operation.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetComputeSystemProperties<P0, P1, P2>(computesystem: P0, operation: P1, propertyquery: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HCS_SYSTEM>,
    P1: ::windows::core::IntoParam<HCS_OPERATION>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsGetComputeSystemProperties ( computesystem : HCS_SYSTEM , operation : HCS_OPERATION , propertyquery : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsGetComputeSystemProperties(computesystem.into_param().abi(), operation.into_param().abi(), propertyquery.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsGetLayerVhdMountPath<P0>(vhdhandle: P0) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "computestorage.dll""system" fn HcsGetLayerVhdMountPath ( vhdhandle : super::super::Foundation:: HANDLE , mountpath : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    HcsGetLayerVhdMountPath(vhdhandle.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetOperationContext<P0>(operation: P0) -> *mut ::core::ffi::c_void
where
    P0: ::windows::core::IntoParam<HCS_OPERATION>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsGetOperationContext ( operation : HCS_OPERATION ) -> *mut ::core::ffi::c_void );
    HcsGetOperationContext(operation.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetOperationId<P0>(operation: P0) -> u64
where
    P0: ::windows::core::IntoParam<HCS_OPERATION>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsGetOperationId ( operation : HCS_OPERATION ) -> u64 );
    HcsGetOperationId(operation.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetOperationResult<P0>(operation: P0, resultdocument: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HCS_OPERATION>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsGetOperationResult ( operation : HCS_OPERATION , resultdocument : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcsGetOperationResult(operation.into_param().abi(), ::core::mem::transmute(resultdocument.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsGetOperationResultAndProcessInfo<P0>(operation: P0, processinformation: ::core::option::Option<*mut HCS_PROCESS_INFORMATION>, resultdocument: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HCS_OPERATION>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsGetOperationResultAndProcessInfo ( operation : HCS_OPERATION , processinformation : *mut HCS_PROCESS_INFORMATION , resultdocument : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcsGetOperationResultAndProcessInfo(operation.into_param().abi(), ::core::mem::transmute(processinformation.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(resultdocument.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetOperationType<P0>(operation: P0) -> HCS_OPERATION_TYPE
where
    P0: ::windows::core::IntoParam<HCS_OPERATION>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsGetOperationType ( operation : HCS_OPERATION ) -> HCS_OPERATION_TYPE );
    HcsGetOperationType(operation.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetProcessFromOperation<P0>(operation: P0) -> HCS_PROCESS
where
    P0: ::windows::core::IntoParam<HCS_OPERATION>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsGetProcessFromOperation ( operation : HCS_OPERATION ) -> HCS_PROCESS );
    HcsGetProcessFromOperation(operation.into_param().abi())
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetProcessInfo<P0, P1>(process: P0, operation: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HCS_PROCESS>,
    P1: ::windows::core::IntoParam<HCS_OPERATION>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsGetProcessInfo ( process : HCS_PROCESS , operation : HCS_OPERATION ) -> :: windows::core::HRESULT );
    HcsGetProcessInfo(process.into_param().abi(), operation.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetProcessProperties<P0, P1, P2>(process: P0, operation: P1, propertyquery: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HCS_PROCESS>,
    P1: ::windows::core::IntoParam<HCS_OPERATION>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsGetProcessProperties ( process : HCS_PROCESS , operation : HCS_OPERATION , propertyquery : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsGetProcessProperties(process.into_param().abi(), operation.into_param().abi(), propertyquery.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetProcessorCompatibilityFromSavedState<P0>(runtimefilename: P0, processorfeaturesstring: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsGetProcessorCompatibilityFromSavedState ( runtimefilename : :: windows::core::PCWSTR , processorfeaturesstring : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcsGetProcessorCompatibilityFromSavedState(runtimefilename.into_param().abi(), ::core::mem::transmute(processorfeaturesstring.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGetServiceProperties<P0>(propertyquery: P0) -> ::windows::core::Result<::windows::core::PWSTR>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsGetServiceProperties ( propertyquery : :: windows::core::PCWSTR , result : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<::windows::core::PWSTR>();
    HcsGetServiceProperties(propertyquery.into_param().abi(), &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGrantVmAccess<P0, P1>(vmid: P0, filepath: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsGrantVmAccess ( vmid : :: windows::core::PCWSTR , filepath : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsGrantVmAccess(vmid.into_param().abi(), filepath.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsGrantVmGroupAccess<P0>(filepath: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsGrantVmGroupAccess ( filepath : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsGrantVmGroupAccess(filepath.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsImportLayer<P0, P1, P2>(layerpath: P0, sourcefolderpath: P1, layerdata: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computestorage.dll""system" fn HcsImportLayer ( layerpath : :: windows::core::PCWSTR , sourcefolderpath : :: windows::core::PCWSTR , layerdata : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsImportLayer(layerpath.into_param().abi(), sourcefolderpath.into_param().abi(), layerdata.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsInitializeLegacyWritableLayer<P0, P1, P2, P3>(writablelayermountpath: P0, writablelayerfolderpath: P1, layerdata: P2, options: P3) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computestorage.dll""system" fn HcsInitializeLegacyWritableLayer ( writablelayermountpath : :: windows::core::PCWSTR , writablelayerfolderpath : :: windows::core::PCWSTR , layerdata : :: windows::core::PCWSTR , options : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsInitializeLegacyWritableLayer(writablelayermountpath.into_param().abi(), writablelayerfolderpath.into_param().abi(), layerdata.into_param().abi(), options.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsInitializeWritableLayer<P0, P1, P2>(writablelayerpath: P0, layerdata: P1, options: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computestorage.dll""system" fn HcsInitializeWritableLayer ( writablelayerpath : :: windows::core::PCWSTR , layerdata : :: windows::core::PCWSTR , options : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsInitializeWritableLayer(writablelayerpath.into_param().abi(), layerdata.into_param().abi(), options.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsModifyComputeSystem<P0, P1, P2, P3>(computesystem: P0, operation: P1, configuration: P2, identity: P3) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HCS_SYSTEM>,
    P1: ::windows::core::IntoParam<HCS_OPERATION>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P3: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsModifyComputeSystem ( computesystem : HCS_SYSTEM , operation : HCS_OPERATION , configuration : :: windows::core::PCWSTR , identity : super::super::Foundation:: HANDLE ) -> :: windows::core::HRESULT );
    HcsModifyComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), configuration.into_param().abi(), identity.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsModifyProcess<P0, P1, P2>(process: P0, operation: P1, settings: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HCS_PROCESS>,
    P1: ::windows::core::IntoParam<HCS_OPERATION>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsModifyProcess ( process : HCS_PROCESS , operation : HCS_OPERATION , settings : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsModifyProcess(process.into_param().abi(), operation.into_param().abi(), settings.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsModifyServiceSettings<P0>(settings: P0, result: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsModifyServiceSettings ( settings : :: windows::core::PCWSTR , result : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcsModifyServiceSettings(settings.into_param().abi(), ::core::mem::transmute(result.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsOpenComputeSystem<P0>(id: P0, requestedaccess: u32) -> ::windows::core::Result<HCS_SYSTEM>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsOpenComputeSystem ( id : :: windows::core::PCWSTR , requestedaccess : u32 , computesystem : *mut HCS_SYSTEM ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<HCS_SYSTEM>();
    HcsOpenComputeSystem(id.into_param().abi(), requestedaccess, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsOpenComputeSystemInNamespace<P0, P1>(idnamespace: P0, id: P1, requestedaccess: u32) -> ::windows::core::Result<HCS_SYSTEM>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsOpenComputeSystemInNamespace ( idnamespace : :: windows::core::PCWSTR , id : :: windows::core::PCWSTR , requestedaccess : u32 , computesystem : *mut HCS_SYSTEM ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<HCS_SYSTEM>();
    HcsOpenComputeSystemInNamespace(idnamespace.into_param().abi(), id.into_param().abi(), requestedaccess, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsOpenProcess<P0>(computesystem: P0, processid: u32, requestedaccess: u32) -> ::windows::core::Result<HCS_PROCESS>
where
    P0: ::windows::core::IntoParam<HCS_SYSTEM>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsOpenProcess ( computesystem : HCS_SYSTEM , processid : u32 , requestedaccess : u32 , process : *mut HCS_PROCESS ) -> :: windows::core::HRESULT );
    let mut result__ = ::windows::core::zeroed::<HCS_PROCESS>();
    HcsOpenProcess(computesystem.into_param().abi(), processid, requestedaccess, &mut result__).from_abi(result__)
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsPauseComputeSystem<P0, P1, P2>(computesystem: P0, operation: P1, options: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HCS_SYSTEM>,
    P1: ::windows::core::IntoParam<HCS_OPERATION>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsPauseComputeSystem ( computesystem : HCS_SYSTEM , operation : HCS_OPERATION , options : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsPauseComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsResumeComputeSystem<P0, P1, P2>(computesystem: P0, operation: P1, options: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HCS_SYSTEM>,
    P1: ::windows::core::IntoParam<HCS_OPERATION>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsResumeComputeSystem ( computesystem : HCS_SYSTEM , operation : HCS_OPERATION , options : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsResumeComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsRevokeVmAccess<P0, P1>(vmid: P0, filepath: P1) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsRevokeVmAccess ( vmid : :: windows::core::PCWSTR , filepath : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsRevokeVmAccess(vmid.into_param().abi(), filepath.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsRevokeVmGroupAccess<P0>(filepath: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsRevokeVmGroupAccess ( filepath : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsRevokeVmGroupAccess(filepath.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsSaveComputeSystem<P0, P1, P2>(computesystem: P0, operation: P1, options: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HCS_SYSTEM>,
    P1: ::windows::core::IntoParam<HCS_OPERATION>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsSaveComputeSystem ( computesystem : HCS_SYSTEM , operation : HCS_OPERATION , options : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsSaveComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsSetComputeSystemCallback<P0>(computesystem: P0, callbackoptions: HCS_EVENT_OPTIONS, context: ::core::option::Option<*const ::core::ffi::c_void>, callback: HCS_EVENT_CALLBACK) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HCS_SYSTEM>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsSetComputeSystemCallback ( computesystem : HCS_SYSTEM , callbackoptions : HCS_EVENT_OPTIONS , context : *const ::core::ffi::c_void , callback : HCS_EVENT_CALLBACK ) -> :: windows::core::HRESULT );
    HcsSetComputeSystemCallback(computesystem.into_param().abi(), callbackoptions, ::core::mem::transmute(context.unwrap_or(::std::ptr::null())), callback).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsSetOperationCallback<P0>(operation: P0, context: ::core::option::Option<*const ::core::ffi::c_void>, callback: HCS_OPERATION_COMPLETION) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HCS_OPERATION>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsSetOperationCallback ( operation : HCS_OPERATION , context : *const ::core::ffi::c_void , callback : HCS_OPERATION_COMPLETION ) -> :: windows::core::HRESULT );
    HcsSetOperationCallback(operation.into_param().abi(), ::core::mem::transmute(context.unwrap_or(::std::ptr::null())), callback).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsSetOperationContext<P0>(operation: P0, context: ::core::option::Option<*const ::core::ffi::c_void>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HCS_OPERATION>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsSetOperationContext ( operation : HCS_OPERATION , context : *const ::core::ffi::c_void ) -> :: windows::core::HRESULT );
    HcsSetOperationContext(operation.into_param().abi(), ::core::mem::transmute(context.unwrap_or(::std::ptr::null()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsSetProcessCallback<P0>(process: P0, callbackoptions: HCS_EVENT_OPTIONS, context: *const ::core::ffi::c_void, callback: HCS_EVENT_CALLBACK) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HCS_PROCESS>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsSetProcessCallback ( process : HCS_PROCESS , callbackoptions : HCS_EVENT_OPTIONS , context : *const ::core::ffi::c_void , callback : HCS_EVENT_CALLBACK ) -> :: windows::core::HRESULT );
    HcsSetProcessCallback(process.into_param().abi(), callbackoptions, context, callback).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsSetupBaseOSLayer<P0, P1, P2>(layerpath: P0, vhdhandle: P1, options: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<super::super::Foundation::HANDLE>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computestorage.dll""system" fn HcsSetupBaseOSLayer ( layerpath : :: windows::core::PCWSTR , vhdhandle : super::super::Foundation:: HANDLE , options : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsSetupBaseOSLayer(layerpath.into_param().abi(), vhdhandle.into_param().abi(), options.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsSetupBaseOSVolume<P0, P1, P2>(layerpath: P0, volumepath: P1, options: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P1: ::windows::core::IntoParam<::windows::core::PCWSTR>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computestorage.dll""system" fn HcsSetupBaseOSVolume ( layerpath : :: windows::core::PCWSTR , volumepath : :: windows::core::PCWSTR , options : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsSetupBaseOSVolume(layerpath.into_param().abi(), volumepath.into_param().abi(), options.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsShutDownComputeSystem<P0, P1, P2>(computesystem: P0, operation: P1, options: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HCS_SYSTEM>,
    P1: ::windows::core::IntoParam<HCS_OPERATION>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsShutDownComputeSystem ( computesystem : HCS_SYSTEM , operation : HCS_OPERATION , options : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsShutDownComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsSignalProcess<P0, P1, P2>(process: P0, operation: P1, options: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HCS_PROCESS>,
    P1: ::windows::core::IntoParam<HCS_OPERATION>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsSignalProcess ( process : HCS_PROCESS , operation : HCS_OPERATION , options : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsSignalProcess(process.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsStartComputeSystem<P0, P1, P2>(computesystem: P0, operation: P1, options: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HCS_SYSTEM>,
    P1: ::windows::core::IntoParam<HCS_OPERATION>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsStartComputeSystem ( computesystem : HCS_SYSTEM , operation : HCS_OPERATION , options : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsStartComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsSubmitWerReport<P0>(settings: P0) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsSubmitWerReport ( settings : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsSubmitWerReport(settings.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsTerminateComputeSystem<P0, P1, P2>(computesystem: P0, operation: P1, options: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HCS_SYSTEM>,
    P1: ::windows::core::IntoParam<HCS_OPERATION>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsTerminateComputeSystem ( computesystem : HCS_SYSTEM , operation : HCS_OPERATION , options : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsTerminateComputeSystem(computesystem.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsTerminateProcess<P0, P1, P2>(process: P0, operation: P1, options: P2) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HCS_PROCESS>,
    P1: ::windows::core::IntoParam<HCS_OPERATION>,
    P2: ::windows::core::IntoParam<::windows::core::PCWSTR>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsTerminateProcess ( process : HCS_PROCESS , operation : HCS_OPERATION , options : :: windows::core::PCWSTR ) -> :: windows::core::HRESULT );
    HcsTerminateProcess(process.into_param().abi(), operation.into_param().abi(), options.into_param().abi()).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsWaitForComputeSystemExit<P0>(computesystem: P0, timeoutms: u32, result: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HCS_SYSTEM>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsWaitForComputeSystemExit ( computesystem : HCS_SYSTEM , timeoutms : u32 , result : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcsWaitForComputeSystemExit(computesystem.into_param().abi(), timeoutms, ::core::mem::transmute(result.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsWaitForOperationResult<P0>(operation: P0, timeoutms: u32, resultdocument: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HCS_OPERATION>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsWaitForOperationResult ( operation : HCS_OPERATION , timeoutms : u32 , resultdocument : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcsWaitForOperationResult(operation.into_param().abi(), timeoutms, ::core::mem::transmute(resultdocument.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
#[inline]
pub unsafe fn HcsWaitForOperationResultAndProcessInfo<P0>(operation: P0, timeoutms: u32, processinformation: ::core::option::Option<*mut HCS_PROCESS_INFORMATION>, resultdocument: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HCS_OPERATION>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsWaitForOperationResultAndProcessInfo ( operation : HCS_OPERATION , timeoutms : u32 , processinformation : *mut HCS_PROCESS_INFORMATION , resultdocument : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcsWaitForOperationResultAndProcessInfo(operation.into_param().abi(), timeoutms, ::core::mem::transmute(processinformation.unwrap_or(::std::ptr::null_mut())), ::core::mem::transmute(resultdocument.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[inline]
pub unsafe fn HcsWaitForProcessExit<P0>(computesystem: P0, timeoutms: u32, result: ::core::option::Option<*mut ::windows::core::PWSTR>) -> ::windows::core::Result<()>
where
    P0: ::windows::core::IntoParam<HCS_PROCESS>,
{
    ::windows::imp::link ! ( "computecore.dll""system" fn HcsWaitForProcessExit ( computesystem : HCS_PROCESS , timeoutms : u32 , result : *mut :: windows::core::PWSTR ) -> :: windows::core::HRESULT );
    HcsWaitForProcessExit(computesystem.into_param().abi(), timeoutms, ::core::mem::transmute(result.unwrap_or(::std::ptr::null_mut()))).ok()
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HCS_CREATE_OPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsCreateOptions_1: HCS_CREATE_OPTIONS = HCS_CREATE_OPTIONS(65536i32);
impl ::core::marker::Copy for HCS_CREATE_OPTIONS {}
impl ::core::clone::Clone for HCS_CREATE_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HCS_CREATE_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for HCS_CREATE_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HCS_CREATE_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCS_CREATE_OPTIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HCS_EVENT_OPTIONS(pub i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventOptionNone: HCS_EVENT_OPTIONS = HCS_EVENT_OPTIONS(0i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventOptionEnableOperationCallbacks: HCS_EVENT_OPTIONS = HCS_EVENT_OPTIONS(1i32);
impl ::core::marker::Copy for HCS_EVENT_OPTIONS {}
impl ::core::clone::Clone for HCS_EVENT_OPTIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HCS_EVENT_OPTIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for HCS_EVENT_OPTIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HCS_EVENT_OPTIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCS_EVENT_OPTIONS").field(&self.0).finish()
    }
}
impl HCS_EVENT_OPTIONS {
    pub const fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}
impl ::core::ops::BitOr for HCS_EVENT_OPTIONS {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }
}
impl ::core::ops::BitAnd for HCS_EVENT_OPTIONS {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        Self(self.0 & other.0)
    }
}
impl ::core::ops::BitOrAssign for HCS_EVENT_OPTIONS {
    fn bitor_assign(&mut self, other: Self) {
        self.0.bitor_assign(other.0)
    }
}
impl ::core::ops::BitAndAssign for HCS_EVENT_OPTIONS {
    fn bitand_assign(&mut self, other: Self) {
        self.0.bitand_assign(other.0)
    }
}
impl ::core::ops::Not for HCS_EVENT_OPTIONS {
    type Output = Self;
    fn not(self) -> Self {
        Self(self.0.not())
    }
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HCS_EVENT_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventInvalid: HCS_EVENT_TYPE = HCS_EVENT_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventSystemExited: HCS_EVENT_TYPE = HCS_EVENT_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventSystemCrashInitiated: HCS_EVENT_TYPE = HCS_EVENT_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventSystemCrashReport: HCS_EVENT_TYPE = HCS_EVENT_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventSystemRdpEnhancedModeStateChanged: HCS_EVENT_TYPE = HCS_EVENT_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventSystemSiloJobCreated: HCS_EVENT_TYPE = HCS_EVENT_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventSystemGuestConnectionClosed: HCS_EVENT_TYPE = HCS_EVENT_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventProcessExited: HCS_EVENT_TYPE = HCS_EVENT_TYPE(65536i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventOperationCallback: HCS_EVENT_TYPE = HCS_EVENT_TYPE(16777216i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsEventServiceDisconnect: HCS_EVENT_TYPE = HCS_EVENT_TYPE(33554432i32);
impl ::core::marker::Copy for HCS_EVENT_TYPE {}
impl ::core::clone::Clone for HCS_EVENT_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HCS_EVENT_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for HCS_EVENT_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HCS_EVENT_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCS_EVENT_TYPE").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HCS_NOTIFICATIONS(pub i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationInvalid: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(0i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemExited: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(1i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemCreateCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(2i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemStartCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(3i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemPauseCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(4i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemResumeCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(5i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemCrashReport: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(6i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemSiloJobCreated: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(7i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemSaveCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(8i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemRdpEnhancedModeStateChanged: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(9i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemShutdownFailed: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(10i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemShutdownCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(10i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemGetPropertiesCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(11i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemModifyCompleted: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(12i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemCrashInitiated: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(13i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemGuestConnectionClosed: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(14i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemOperationCompletion: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(15i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationSystemPassThru: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(16i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationProcessExited: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(65536i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationServiceDisconnect: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(16777216i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationFlagsReserved: HCS_NOTIFICATIONS = HCS_NOTIFICATIONS(-268435456i32);
impl ::core::marker::Copy for HCS_NOTIFICATIONS {}
impl ::core::clone::Clone for HCS_NOTIFICATIONS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HCS_NOTIFICATIONS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for HCS_NOTIFICATIONS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HCS_NOTIFICATIONS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCS_NOTIFICATIONS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HCS_NOTIFICATION_FLAGS(pub i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationFlagSuccess: HCS_NOTIFICATION_FLAGS = HCS_NOTIFICATION_FLAGS(0i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsNotificationFlagFailure: HCS_NOTIFICATION_FLAGS = HCS_NOTIFICATION_FLAGS(-2147483648i32);
impl ::core::marker::Copy for HCS_NOTIFICATION_FLAGS {}
impl ::core::clone::Clone for HCS_NOTIFICATION_FLAGS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HCS_NOTIFICATION_FLAGS {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for HCS_NOTIFICATION_FLAGS {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HCS_NOTIFICATION_FLAGS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCS_NOTIFICATION_FLAGS").field(&self.0).finish()
    }
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HCS_OPERATION_TYPE(pub i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeNone: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(-1i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeEnumerate: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(0i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeCreate: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(1i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeStart: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(2i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeShutdown: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(3i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypePause: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(4i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeResume: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(5i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeSave: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(6i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeTerminate: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(7i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeModify: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(8i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeGetProperties: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(9i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeCreateProcess: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(10i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeSignalProcess: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(11i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeGetProcessInfo: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(12i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeGetProcessProperties: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(13i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeModifyProcess: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(14i32);
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub const HcsOperationTypeCrash: HCS_OPERATION_TYPE = HCS_OPERATION_TYPE(15i32);
impl ::core::marker::Copy for HCS_OPERATION_TYPE {}
impl ::core::clone::Clone for HCS_OPERATION_TYPE {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::default::Default for HCS_OPERATION_TYPE {
    fn default() -> Self {
        Self(0)
    }
}
impl ::windows::core::TypeKind for HCS_OPERATION_TYPE {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::fmt::Debug for HCS_OPERATION_TYPE {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCS_OPERATION_TYPE").field(&self.0).finish()
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`, `\"Win32_Security\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
pub struct HCS_CREATE_OPTIONS_1 {
    pub Version: HCS_CREATE_OPTIONS,
    pub UserToken: super::super::Foundation::HANDLE,
    pub SecurityDescriptor: *mut super::super::Security::SECURITY_DESCRIPTOR,
    pub CallbackOptions: HCS_EVENT_OPTIONS,
    pub CallbackContext: *mut ::core::ffi::c_void,
    pub Callback: HCS_EVENT_CALLBACK,
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::marker::Copy for HCS_CREATE_OPTIONS_1 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::clone::Clone for HCS_CREATE_OPTIONS_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::fmt::Debug for HCS_CREATE_OPTIONS_1 {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HCS_CREATE_OPTIONS_1").field("Version", &self.Version).field("UserToken", &self.UserToken).field("SecurityDescriptor", &self.SecurityDescriptor).field("CallbackOptions", &self.CallbackOptions).field("CallbackContext", &self.CallbackContext).finish()
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::windows::core::TypeKind for HCS_CREATE_OPTIONS_1 {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
impl ::core::default::Default for HCS_CREATE_OPTIONS_1 {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub struct HCS_EVENT {
    pub Type: HCS_EVENT_TYPE,
    pub EventData: ::windows::core::PCWSTR,
    pub Operation: HCS_OPERATION,
}
impl ::core::marker::Copy for HCS_EVENT {}
impl ::core::clone::Clone for HCS_EVENT {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::fmt::Debug for HCS_EVENT {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HCS_EVENT").field("Type", &self.Type).field("EventData", &self.EventData).field("Operation", &self.Operation).finish()
    }
}
impl ::windows::core::TypeKind for HCS_EVENT {
    type TypeKind = ::windows::core::CopyType;
}
impl ::core::cmp::PartialEq for HCS_EVENT {
    fn eq(&self, other: &Self) -> bool {
        self.Type == other.Type && self.EventData == other.EventData && self.Operation == other.Operation
    }
}
impl ::core::cmp::Eq for HCS_EVENT {}
impl ::core::default::Default for HCS_EVENT {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HCS_OPERATION(pub isize);
impl HCS_OPERATION {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HCS_OPERATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HCS_OPERATION {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HCS_OPERATION {}
impl ::core::fmt::Debug for HCS_OPERATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCS_OPERATION").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HCS_OPERATION {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HCS_PROCESS(pub isize);
impl HCS_PROCESS {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HCS_PROCESS {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HCS_PROCESS {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HCS_PROCESS {}
impl ::core::fmt::Debug for HCS_PROCESS {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCS_PROCESS").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HCS_PROCESS {
    type TypeKind = ::windows::core::CopyType;
}
#[repr(C)]
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`, `\"Win32_Foundation\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub struct HCS_PROCESS_INFORMATION {
    pub ProcessId: u32,
    pub Reserved: u32,
    pub StdInput: super::super::Foundation::HANDLE,
    pub StdOutput: super::super::Foundation::HANDLE,
    pub StdError: super::super::Foundation::HANDLE,
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::marker::Copy for HCS_PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::clone::Clone for HCS_PROCESS_INFORMATION {
    fn clone(&self) -> Self {
        *self
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::fmt::Debug for HCS_PROCESS_INFORMATION {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("HCS_PROCESS_INFORMATION").field("ProcessId", &self.ProcessId).field("Reserved", &self.Reserved).field("StdInput", &self.StdInput).field("StdOutput", &self.StdOutput).field("StdError", &self.StdError).finish()
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows::core::TypeKind for HCS_PROCESS_INFORMATION {
    type TypeKind = ::windows::core::CopyType;
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::PartialEq for HCS_PROCESS_INFORMATION {
    fn eq(&self, other: &Self) -> bool {
        self.ProcessId == other.ProcessId && self.Reserved == other.Reserved && self.StdInput == other.StdInput && self.StdOutput == other.StdOutput && self.StdError == other.StdError
    }
}
#[cfg(feature = "Win32_Foundation")]
impl ::core::cmp::Eq for HCS_PROCESS_INFORMATION {}
#[cfg(feature = "Win32_Foundation")]
impl ::core::default::Default for HCS_PROCESS_INFORMATION {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
#[repr(transparent)]
#[derive(::core::cmp::PartialEq, ::core::cmp::Eq)]
pub struct HCS_SYSTEM(pub isize);
impl HCS_SYSTEM {
    pub fn is_invalid(&self) -> bool {
        self.0 == -1 || self.0 == 0
    }
}
impl ::core::default::Default for HCS_SYSTEM {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
impl ::core::clone::Clone for HCS_SYSTEM {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::core::marker::Copy for HCS_SYSTEM {}
impl ::core::fmt::Debug for HCS_SYSTEM {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("HCS_SYSTEM").field(&self.0).finish()
    }
}
impl ::windows::core::TypeKind for HCS_SYSTEM {
    type TypeKind = ::windows::core::CopyType;
}
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub type HCS_EVENT_CALLBACK = ::core::option::Option<unsafe extern "system" fn(event: *const HCS_EVENT, context: *const ::core::ffi::c_void) -> ()>;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub type HCS_NOTIFICATION_CALLBACK = ::core::option::Option<unsafe extern "system" fn(notificationtype: u32, context: *const ::core::ffi::c_void, notificationstatus: ::windows::core::HRESULT, notificationdata: ::windows::core::PCWSTR) -> ()>;
#[doc = "*Required features: `\"Win32_System_HostComputeSystem\"`*"]
pub type HCS_OPERATION_COMPLETION = ::core::option::Option<unsafe extern "system" fn(operation: HCS_OPERATION, context: *const ::core::ffi::c_void) -> ()>;
#[cfg(feature = "implement")]
::core::include!("impl.rs");
