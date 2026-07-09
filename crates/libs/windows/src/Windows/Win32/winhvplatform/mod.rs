#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_winhvplatformdefs", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn WHvAcceptPartitionMigration(migrationhandle: super::winnt::HANDLE) -> windows_core::Result<super::winhvplatformdefs::WHV_PARTITION_HANDLE> {
    windows_core::link!("winhvplatform.dll" "system" fn WHvAcceptPartitionMigration(migrationhandle : super::winnt::HANDLE, partition : *mut super::winhvplatformdefs::WHV_PARTITION_HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WHvAcceptPartitionMigration(migrationhandle, &mut result__).map(|| result__)
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvAdviseGpaRange(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, gparanges: &[super::winhvplatformdefs::WHV_MEMORY_RANGE_ENTRY], advice: super::winhvplatformdefs::WHV_ADVISE_GPA_RANGE_CODE, advicebuffer: *const core::ffi::c_void, advicebuffersizeinbytes: u32) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvAdviseGpaRange(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, gparanges : *const super::winhvplatformdefs::WHV_MEMORY_RANGE_ENTRY, gparangescount : u32, advice : super::winhvplatformdefs::WHV_ADVISE_GPA_RANGE_CODE, advicebuffer : *const core::ffi::c_void, advicebuffersizeinbytes : u32) -> windows_core::HRESULT);
    unsafe { WHvAdviseGpaRange(partition, core::mem::transmute(gparanges.as_ptr()), gparanges.len().try_into().unwrap(), advice, advicebuffer, advicebuffersizeinbytes) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_winhvplatformdefs", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn WHvAllocateVpciResource(providerid: Option<*const windows_core::GUID>, flags: super::winhvplatformdefs::WHV_ALLOCATE_VPCI_RESOURCE_FLAGS, resourcedescriptor: Option<&[u8]>) -> windows_core::Result<super::winnt::HANDLE> {
    windows_core::link!("winhvplatform.dll" "system" fn WHvAllocateVpciResource(providerid : *const windows_core::GUID, flags : super::winhvplatformdefs::WHV_ALLOCATE_VPCI_RESOURCE_FLAGS, resourcedescriptor : *const core::ffi::c_void, resourcedescriptorsizeinbytes : u32, vpciresource : *mut super::winnt::HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WHvAllocateVpciResource(providerid.unwrap_or(core::mem::zeroed()) as _, flags, core::mem::transmute(resourcedescriptor.map_or(core::ptr::null(), |slice| slice.as_ptr())), resourcedescriptor.map_or(0, |slice| slice.len().try_into().unwrap()), &mut result__).map(|| result__)
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvCancelPartitionMigration(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "C" fn WHvCancelPartitionMigration(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE) -> windows_core::HRESULT);
    unsafe { WHvCancelPartitionMigration(partition) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvCancelRunVirtualProcessor(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex: u32, flags: u32) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvCancelRunVirtualProcessor(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex : u32, flags : u32) -> windows_core::HRESULT);
    unsafe { WHvCancelRunVirtualProcessor(partition, vpindex, flags) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvCompletePartitionMigration(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "C" fn WHvCompletePartitionMigration(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE) -> windows_core::HRESULT);
    unsafe { WHvCompletePartitionMigration(partition) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_winhvplatformdefs", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn WHvCreateNotificationPort(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, parameters: *const super::winhvplatformdefs::WHV_NOTIFICATION_PORT_PARAMETERS, eventhandle: super::winnt::HANDLE) -> windows_core::Result<super::winhvplatformdefs::WHV_NOTIFICATION_PORT_HANDLE> {
    windows_core::link!("winhvplatform.dll" "system" fn WHvCreateNotificationPort(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, parameters : *const super::winhvplatformdefs::WHV_NOTIFICATION_PORT_PARAMETERS, eventhandle : super::winnt::HANDLE, porthandle : *mut super::winhvplatformdefs::WHV_NOTIFICATION_PORT_HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WHvCreateNotificationPort(partition, parameters, eventhandle, &mut result__).map(|| result__)
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvCreatePartition() -> windows_core::Result<super::winhvplatformdefs::WHV_PARTITION_HANDLE> {
    windows_core::link!("winhvplatform.dll" "system" fn WHvCreatePartition(partition : *mut super::winhvplatformdefs::WHV_PARTITION_HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WHvCreatePartition(&mut result__).map(|| result__)
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_winhvplatformdefs", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn WHvCreateTrigger(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, parameters: *const super::winhvplatformdefs::WHV_TRIGGER_PARAMETERS, triggerhandle: *mut super::winhvplatformdefs::WHV_TRIGGER_HANDLE, eventhandle: *mut super::winnt::HANDLE) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvCreateTrigger(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, parameters : *const super::winhvplatformdefs::WHV_TRIGGER_PARAMETERS, triggerhandle : *mut super::winhvplatformdefs::WHV_TRIGGER_HANDLE, eventhandle : *mut super::winnt::HANDLE) -> windows_core::HRESULT);
    unsafe { WHvCreateTrigger(partition, parameters, triggerhandle as _, eventhandle as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvCreateVirtualProcessor(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex: u32, flags: u32) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvCreateVirtualProcessor(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex : u32, flags : u32) -> windows_core::HRESULT);
    unsafe { WHvCreateVirtualProcessor(partition, vpindex, flags) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvCreateVirtualProcessor2(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex: u32, properties: &[super::winhvplatformdefs::WHV_VIRTUAL_PROCESSOR_PROPERTY]) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvCreateVirtualProcessor2(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex : u32, properties : *const super::winhvplatformdefs::WHV_VIRTUAL_PROCESSOR_PROPERTY, propertycount : u32) -> windows_core::HRESULT);
    unsafe { WHvCreateVirtualProcessor2(partition, vpindex, core::mem::transmute(properties.as_ptr()), properties.len().try_into().unwrap()) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_winhvplatformdefs", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn WHvCreateVpciDevice(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid: u64, vpciresource: super::winnt::HANDLE, flags: super::winhvplatformdefs::WHV_CREATE_VPCI_DEVICE_FLAGS, notificationeventhandle: Option<super::winnt::HANDLE>) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvCreateVpciDevice(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid : u64, vpciresource : super::winnt::HANDLE, flags : super::winhvplatformdefs::WHV_CREATE_VPCI_DEVICE_FLAGS, notificationeventhandle : super::winnt::HANDLE) -> windows_core::HRESULT);
    unsafe { WHvCreateVpciDevice(partition, logicaldeviceid, vpciresource, flags, notificationeventhandle.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvDeleteNotificationPort(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, porthandle: super::winhvplatformdefs::WHV_NOTIFICATION_PORT_HANDLE) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvDeleteNotificationPort(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, porthandle : super::winhvplatformdefs::WHV_NOTIFICATION_PORT_HANDLE) -> windows_core::HRESULT);
    unsafe { WHvDeleteNotificationPort(partition, porthandle) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvDeletePartition(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvDeletePartition(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE) -> windows_core::HRESULT);
    unsafe { WHvDeletePartition(partition) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvDeleteTrigger(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, triggerhandle: super::winhvplatformdefs::WHV_TRIGGER_HANDLE) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvDeleteTrigger(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, triggerhandle : super::winhvplatformdefs::WHV_TRIGGER_HANDLE) -> windows_core::HRESULT);
    unsafe { WHvDeleteTrigger(partition, triggerhandle) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvDeleteVirtualProcessor(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex: u32) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvDeleteVirtualProcessor(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex : u32) -> windows_core::HRESULT);
    unsafe { WHvDeleteVirtualProcessor(partition, vpindex) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvDeleteVpciDevice(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid: u64) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvDeleteVpciDevice(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid : u64) -> windows_core::HRESULT);
    unsafe { WHvDeleteVpciDevice(partition, logicaldeviceid) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvGetCapability(capabilitycode: super::winhvplatformdefs::WHV_CAPABILITY_CODE, capabilitybuffer: *mut core::ffi::c_void, capabilitybuffersizeinbytes: u32, writtensizeinbytes: Option<*mut u32>) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvGetCapability(capabilitycode : super::winhvplatformdefs::WHV_CAPABILITY_CODE, capabilitybuffer : *mut core::ffi::c_void, capabilitybuffersizeinbytes : u32, writtensizeinbytes : *mut u32) -> windows_core::HRESULT);
    unsafe { WHvGetCapability(capabilitycode, capabilitybuffer as _, capabilitybuffersizeinbytes, writtensizeinbytes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvGetInterruptTargetVpSet(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, destination: u64, destinationmode: super::winhvplatformdefs::WHV_INTERRUPT_DESTINATION_MODE, targetvps: &mut [u32], targetvpcount: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvGetInterruptTargetVpSet(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, destination : u64, destinationmode : super::winhvplatformdefs::WHV_INTERRUPT_DESTINATION_MODE, targetvps : *mut u32, vpcount : u32, targetvpcount : *mut u32) -> windows_core::HRESULT);
    unsafe { WHvGetInterruptTargetVpSet(partition, destination, destinationmode, core::mem::transmute(targetvps.as_ptr()), targetvps.len().try_into().unwrap(), targetvpcount as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvGetPartitionCounters(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, counterset: super::winhvplatformdefs::WHV_PARTITION_COUNTER_SET, buffer: *mut core::ffi::c_void, buffersizeinbytes: u32, byteswritten: Option<*mut u32>) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvGetPartitionCounters(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, counterset : super::winhvplatformdefs::WHV_PARTITION_COUNTER_SET, buffer : *mut core::ffi::c_void, buffersizeinbytes : u32, byteswritten : *mut u32) -> windows_core::HRESULT);
    unsafe { WHvGetPartitionCounters(partition, counterset, buffer as _, buffersizeinbytes, byteswritten.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvGetPartitionProperty(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, propertycode: super::winhvplatformdefs::WHV_PARTITION_PROPERTY_CODE, propertybuffer: *mut core::ffi::c_void, propertybuffersizeinbytes: u32, writtensizeinbytes: Option<*mut u32>) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvGetPartitionProperty(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, propertycode : super::winhvplatformdefs::WHV_PARTITION_PROPERTY_CODE, propertybuffer : *mut core::ffi::c_void, propertybuffersizeinbytes : u32, writtensizeinbytes : *mut u32) -> windows_core::HRESULT);
    unsafe { WHvGetPartitionProperty(partition, propertycode, propertybuffer as _, propertybuffersizeinbytes, writtensizeinbytes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvGetVirtualProcessorCounters(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex: u32, counterset: super::winhvplatformdefs::WHV_PROCESSOR_COUNTER_SET, buffer: *mut core::ffi::c_void, buffersizeinbytes: u32, byteswritten: Option<*mut u32>) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvGetVirtualProcessorCounters(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex : u32, counterset : super::winhvplatformdefs::WHV_PROCESSOR_COUNTER_SET, buffer : *mut core::ffi::c_void, buffersizeinbytes : u32, byteswritten : *mut u32) -> windows_core::HRESULT);
    unsafe { WHvGetVirtualProcessorCounters(partition, vpindex, counterset, buffer as _, buffersizeinbytes, byteswritten.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvGetVirtualProcessorCpuidOutput(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex: u32, eax: u32, ecx: u32) -> windows_core::Result<super::winhvplatformdefs::WHV_CPUID_OUTPUT> {
    windows_core::link!("winhvplatform.dll" "system" fn WHvGetVirtualProcessorCpuidOutput(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex : u32, eax : u32, ecx : u32, cpuidoutput : *mut super::winhvplatformdefs::WHV_CPUID_OUTPUT) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WHvGetVirtualProcessorCpuidOutput(partition, vpindex, eax, ecx, &mut result__).map(|| result__)
    }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvGetVirtualProcessorInterruptControllerState(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex: u32, state: *mut core::ffi::c_void, statesize: u32, writtensize: Option<*mut u32>) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvGetVirtualProcessorInterruptControllerState(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex : u32, state : *mut core::ffi::c_void, statesize : u32, writtensize : *mut u32) -> windows_core::HRESULT);
    unsafe { WHvGetVirtualProcessorInterruptControllerState(partition, vpindex, state as _, statesize, writtensize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvGetVirtualProcessorInterruptControllerState2(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex: u32, state: *mut core::ffi::c_void, statesize: u32, writtensize: Option<*mut u32>) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvGetVirtualProcessorInterruptControllerState2(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex : u32, state : *mut core::ffi::c_void, statesize : u32, writtensize : *mut u32) -> windows_core::HRESULT);
    unsafe { WHvGetVirtualProcessorInterruptControllerState2(partition, vpindex, state as _, statesize, writtensize.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvGetVirtualProcessorRegisters(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex: u32, registernames: *const super::winhvplatformdefs::WHV_REGISTER_NAME, registercount: u32, registervalues: *mut super::winhvplatformdefs::WHV_REGISTER_VALUE) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvGetVirtualProcessorRegisters(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex : u32, registernames : *const super::winhvplatformdefs::WHV_REGISTER_NAME, registercount : u32, registervalues : *mut super::winhvplatformdefs::WHV_REGISTER_VALUE) -> windows_core::HRESULT);
    unsafe { WHvGetVirtualProcessorRegisters(partition, vpindex, registernames, registercount, registervalues as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvGetVirtualProcessorState(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex: u32, statetype: super::winhvplatformdefs::WHV_VIRTUAL_PROCESSOR_STATE_TYPE, buffer: *mut core::ffi::c_void, buffersizeinbytes: u32, byteswritten: Option<*mut u32>) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvGetVirtualProcessorState(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex : u32, statetype : super::winhvplatformdefs::WHV_VIRTUAL_PROCESSOR_STATE_TYPE, buffer : *mut core::ffi::c_void, buffersizeinbytes : u32, byteswritten : *mut u32) -> windows_core::HRESULT);
    unsafe { WHvGetVirtualProcessorState(partition, vpindex, statetype, buffer as _, buffersizeinbytes, byteswritten.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvGetVirtualProcessorXsaveState(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex: u32, buffer: *mut core::ffi::c_void, buffersizeinbytes: u32, byteswritten: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvGetVirtualProcessorXsaveState(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex : u32, buffer : *mut core::ffi::c_void, buffersizeinbytes : u32, byteswritten : *mut u32) -> windows_core::HRESULT);
    unsafe { WHvGetVirtualProcessorXsaveState(partition, vpindex, buffer as _, buffersizeinbytes, byteswritten as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvGetVpciDeviceInterruptTarget(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid: u64, index: u32, multimessagenumber: u32, target: *mut super::winhvplatformdefs::WHV_VPCI_INTERRUPT_TARGET, targetsizeinbytes: u32, byteswritten: Option<*mut u32>) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvGetVpciDeviceInterruptTarget(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid : u64, index : u32, multimessagenumber : u32, target : *mut super::winhvplatformdefs::WHV_VPCI_INTERRUPT_TARGET, targetsizeinbytes : u32, byteswritten : *mut u32) -> windows_core::HRESULT);
    unsafe { WHvGetVpciDeviceInterruptTarget(partition, logicaldeviceid, index, multimessagenumber, target as _, targetsizeinbytes, byteswritten.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvGetVpciDeviceNotification(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid: u64, notification: *mut super::winhvplatformdefs::WHV_VPCI_DEVICE_NOTIFICATION, notificationsizeinbytes: u32) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvGetVpciDeviceNotification(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid : u64, notification : *mut super::winhvplatformdefs::WHV_VPCI_DEVICE_NOTIFICATION, notificationsizeinbytes : u32) -> windows_core::HRESULT);
    unsafe { WHvGetVpciDeviceNotification(partition, logicaldeviceid, notification as _, notificationsizeinbytes) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvGetVpciDeviceProperty(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid: u64, propertycode: super::winhvplatformdefs::WHV_VPCI_DEVICE_PROPERTY_CODE, propertybuffer: *mut core::ffi::c_void, propertybuffersizeinbytes: u32, writtensizeinbytes: Option<*mut u32>) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvGetVpciDeviceProperty(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid : u64, propertycode : super::winhvplatformdefs::WHV_VPCI_DEVICE_PROPERTY_CODE, propertybuffer : *mut core::ffi::c_void, propertybuffersizeinbytes : u32, writtensizeinbytes : *mut u32) -> windows_core::HRESULT);
    unsafe { WHvGetVpciDeviceProperty(partition, logicaldeviceid, propertycode, propertybuffer as _, propertybuffersizeinbytes, writtensizeinbytes.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvMapGpaRange(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, sourceaddress: *const core::ffi::c_void, guestaddress: super::winhvplatformdefs::WHV_GUEST_PHYSICAL_ADDRESS, sizeinbytes: u64, flags: super::winhvplatformdefs::WHV_MAP_GPA_RANGE_FLAGS) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvMapGpaRange(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, sourceaddress : *const core::ffi::c_void, guestaddress : super::winhvplatformdefs::WHV_GUEST_PHYSICAL_ADDRESS, sizeinbytes : u64, flags : super::winhvplatformdefs::WHV_MAP_GPA_RANGE_FLAGS) -> windows_core::HRESULT);
    unsafe { WHvMapGpaRange(partition, sourceaddress, guestaddress, sizeinbytes, flags) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_winhvplatformdefs", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn WHvMapGpaRange2(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, process: super::winnt::HANDLE, sourceaddress: *const core::ffi::c_void, guestaddress: super::winhvplatformdefs::WHV_GUEST_PHYSICAL_ADDRESS, sizeinbytes: u64, flags: super::winhvplatformdefs::WHV_MAP_GPA_RANGE_FLAGS) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvMapGpaRange2(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, process : super::winnt::HANDLE, sourceaddress : *const core::ffi::c_void, guestaddress : super::winhvplatformdefs::WHV_GUEST_PHYSICAL_ADDRESS, sizeinbytes : u64, flags : super::winhvplatformdefs::WHV_MAP_GPA_RANGE_FLAGS) -> windows_core::HRESULT);
    unsafe { WHvMapGpaRange2(partition, process, sourceaddress, guestaddress, sizeinbytes, flags) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvMapVpciDeviceInterrupt(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid: u64, index: u32, messagecount: u32, target: *const super::winhvplatformdefs::WHV_VPCI_INTERRUPT_TARGET, msiaddress: *mut u64, msidata: *mut u32) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvMapVpciDeviceInterrupt(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid : u64, index : u32, messagecount : u32, target : *const super::winhvplatformdefs::WHV_VPCI_INTERRUPT_TARGET, msiaddress : *mut u64, msidata : *mut u32) -> windows_core::HRESULT);
    unsafe { WHvMapVpciDeviceInterrupt(partition, logicaldeviceid, index, messagecount, target, msiaddress as _, msidata as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvMapVpciDeviceMmioRanges(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid: u64, mappingcount: *mut u32, mappings: *mut *mut super::winhvplatformdefs::WHV_VPCI_MMIO_MAPPING) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvMapVpciDeviceMmioRanges(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid : u64, mappingcount : *mut u32, mappings : *mut *mut super::winhvplatformdefs::WHV_VPCI_MMIO_MAPPING) -> windows_core::HRESULT);
    unsafe { WHvMapVpciDeviceMmioRanges(partition, logicaldeviceid, mappingcount as _, mappings as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvPostVirtualProcessorSynicMessage(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex: u32, sintindex: u32, message: *const core::ffi::c_void, messagesizeinbytes: u32) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvPostVirtualProcessorSynicMessage(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex : u32, sintindex : u32, message : *const core::ffi::c_void, messagesizeinbytes : u32) -> windows_core::HRESULT);
    unsafe { WHvPostVirtualProcessorSynicMessage(partition, vpindex, sintindex, message, messagesizeinbytes) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvQueryGpaRangeDirtyBitmap(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, guestaddress: super::winhvplatformdefs::WHV_GUEST_PHYSICAL_ADDRESS, rangesizeinbytes: u64, bitmap: Option<*mut u64>, bitmapsizeinbytes: u32) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvQueryGpaRangeDirtyBitmap(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, guestaddress : super::winhvplatformdefs::WHV_GUEST_PHYSICAL_ADDRESS, rangesizeinbytes : u64, bitmap : *mut u64, bitmapsizeinbytes : u32) -> windows_core::HRESULT);
    unsafe { WHvQueryGpaRangeDirtyBitmap(partition, guestaddress, rangesizeinbytes, bitmap.unwrap_or(core::mem::zeroed()) as _, bitmapsizeinbytes) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvReadGpaRange(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex: u32, guestaddress: super::winhvplatformdefs::WHV_GUEST_PHYSICAL_ADDRESS, controls: super::winhvplatformdefs::WHV_ACCESS_GPA_CONTROLS, data: *mut core::ffi::c_void, datasizeinbytes: u32) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvReadGpaRange(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex : u32, guestaddress : super::winhvplatformdefs::WHV_GUEST_PHYSICAL_ADDRESS, controls : super::winhvplatformdefs::WHV_ACCESS_GPA_CONTROLS, data : *mut core::ffi::c_void, datasizeinbytes : u32) -> windows_core::HRESULT);
    unsafe { WHvReadGpaRange(partition, vpindex, guestaddress, core::mem::transmute(controls), data as _, datasizeinbytes) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvReadVpciDeviceRegister(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid: u64, register: *const super::winhvplatformdefs::WHV_VPCI_DEVICE_REGISTER, data: *mut core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvReadVpciDeviceRegister(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid : u64, register : *const super::winhvplatformdefs::WHV_VPCI_DEVICE_REGISTER, data : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WHvReadVpciDeviceRegister(partition, logicaldeviceid, register, data as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_winhvplatformdefs", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn WHvRegisterPartitionDoorbellEvent(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, matchdata: *const super::winhvplatformdefs::WHV_DOORBELL_MATCH_DATA, eventhandle: super::winnt::HANDLE) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvRegisterPartitionDoorbellEvent(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, matchdata : *const super::winhvplatformdefs::WHV_DOORBELL_MATCH_DATA, eventhandle : super::winnt::HANDLE) -> windows_core::HRESULT);
    unsafe { WHvRegisterPartitionDoorbellEvent(partition, matchdata, eventhandle) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvRequestInterrupt(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, interrupt: *const super::winhvplatformdefs::WHV_INTERRUPT_CONTROL, interruptcontrolsize: u32) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvRequestInterrupt(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, interrupt : *const super::winhvplatformdefs::WHV_INTERRUPT_CONTROL, interruptcontrolsize : u32) -> windows_core::HRESULT);
    unsafe { WHvRequestInterrupt(partition, interrupt, interruptcontrolsize) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvRequestVpciDeviceInterrupt(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid: u64, msiaddress: u64, msidata: u32) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvRequestVpciDeviceInterrupt(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid : u64, msiaddress : u64, msidata : u32) -> windows_core::HRESULT);
    unsafe { WHvRequestVpciDeviceInterrupt(partition, logicaldeviceid, msiaddress, msidata) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvResetPartition(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvResetPartition(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE) -> windows_core::HRESULT);
    unsafe { WHvResetPartition(partition) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvResumePartitionTime(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvResumePartitionTime(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE) -> windows_core::HRESULT);
    unsafe { WHvResumePartitionTime(partition) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvRetargetVpciDeviceInterrupt(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid: u64, msiaddress: u64, msidata: u32, target: *const super::winhvplatformdefs::WHV_VPCI_INTERRUPT_TARGET) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvRetargetVpciDeviceInterrupt(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid : u64, msiaddress : u64, msidata : u32, target : *const super::winhvplatformdefs::WHV_VPCI_INTERRUPT_TARGET) -> windows_core::HRESULT);
    unsafe { WHvRetargetVpciDeviceInterrupt(partition, logicaldeviceid, msiaddress, msidata, target) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvRunVirtualProcessor(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex: u32, exitcontext: *mut core::ffi::c_void, exitcontextsizeinbytes: u32) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvRunVirtualProcessor(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex : u32, exitcontext : *mut core::ffi::c_void, exitcontextsizeinbytes : u32) -> windows_core::HRESULT);
    unsafe { WHvRunVirtualProcessor(partition, vpindex, exitcontext as _, exitcontextsizeinbytes) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvSetNotificationPortProperty(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, porthandle: super::winhvplatformdefs::WHV_NOTIFICATION_PORT_HANDLE, propertycode: super::winhvplatformdefs::WHV_NOTIFICATION_PORT_PROPERTY_CODE, propertyvalue: super::winhvplatformdefs::WHV_NOTIFICATION_PORT_PROPERTY) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvSetNotificationPortProperty(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, porthandle : super::winhvplatformdefs::WHV_NOTIFICATION_PORT_HANDLE, propertycode : super::winhvplatformdefs::WHV_NOTIFICATION_PORT_PROPERTY_CODE, propertyvalue : super::winhvplatformdefs::WHV_NOTIFICATION_PORT_PROPERTY) -> windows_core::HRESULT);
    unsafe { WHvSetNotificationPortProperty(partition, porthandle, propertycode, propertyvalue) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvSetPartitionProperty(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, propertycode: super::winhvplatformdefs::WHV_PARTITION_PROPERTY_CODE, propertybuffer: *const core::ffi::c_void, propertybuffersizeinbytes: u32) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvSetPartitionProperty(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, propertycode : super::winhvplatformdefs::WHV_PARTITION_PROPERTY_CODE, propertybuffer : *const core::ffi::c_void, propertybuffersizeinbytes : u32) -> windows_core::HRESULT);
    unsafe { WHvSetPartitionProperty(partition, propertycode, propertybuffer, propertybuffersizeinbytes) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvSetVirtualProcessorInterruptControllerState(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex: u32, state: *const core::ffi::c_void, statesize: u32) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvSetVirtualProcessorInterruptControllerState(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex : u32, state : *const core::ffi::c_void, statesize : u32) -> windows_core::HRESULT);
    unsafe { WHvSetVirtualProcessorInterruptControllerState(partition, vpindex, state, statesize) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvSetVirtualProcessorInterruptControllerState2(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex: u32, state: *const core::ffi::c_void, statesize: u32) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvSetVirtualProcessorInterruptControllerState2(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex : u32, state : *const core::ffi::c_void, statesize : u32) -> windows_core::HRESULT);
    unsafe { WHvSetVirtualProcessorInterruptControllerState2(partition, vpindex, state, statesize) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvSetVirtualProcessorRegisters(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex: u32, registernames: *const super::winhvplatformdefs::WHV_REGISTER_NAME, registercount: u32, registervalues: *const super::winhvplatformdefs::WHV_REGISTER_VALUE) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvSetVirtualProcessorRegisters(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex : u32, registernames : *const super::winhvplatformdefs::WHV_REGISTER_NAME, registercount : u32, registervalues : *const super::winhvplatformdefs::WHV_REGISTER_VALUE) -> windows_core::HRESULT);
    unsafe { WHvSetVirtualProcessorRegisters(partition, vpindex, registernames, registercount, registervalues) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvSetVirtualProcessorState(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex: u32, statetype: super::winhvplatformdefs::WHV_VIRTUAL_PROCESSOR_STATE_TYPE, buffer: *const core::ffi::c_void, buffersizeinbytes: u32) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvSetVirtualProcessorState(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex : u32, statetype : super::winhvplatformdefs::WHV_VIRTUAL_PROCESSOR_STATE_TYPE, buffer : *const core::ffi::c_void, buffersizeinbytes : u32) -> windows_core::HRESULT);
    unsafe { WHvSetVirtualProcessorState(partition, vpindex, statetype, buffer, buffersizeinbytes) }
}
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvSetVirtualProcessorXsaveState(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex: u32, buffer: *const core::ffi::c_void, buffersizeinbytes: u32) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvSetVirtualProcessorXsaveState(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex : u32, buffer : *const core::ffi::c_void, buffersizeinbytes : u32) -> windows_core::HRESULT);
    unsafe { WHvSetVirtualProcessorXsaveState(partition, vpindex, buffer, buffersizeinbytes) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_winhvplatformdefs", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn WHvSetVpciDevicePowerState(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid: u64, powerstate: super::winnt::DEVICE_POWER_STATE) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvSetVpciDevicePowerState(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid : u64, powerstate : super::winnt::DEVICE_POWER_STATE) -> windows_core::HRESULT);
    unsafe { WHvSetVpciDevicePowerState(partition, logicaldeviceid, powerstate) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvSetupPartition(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvSetupPartition(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE) -> windows_core::HRESULT);
    unsafe { WHvSetupPartition(partition) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvSignalVirtualProcessorSynicEvent(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, synicevent: super::winhvplatformdefs::WHV_SYNIC_EVENT_PARAMETERS, newlysignaled: Option<*mut windows_core::BOOL>) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvSignalVirtualProcessorSynicEvent(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, synicevent : super::winhvplatformdefs::WHV_SYNIC_EVENT_PARAMETERS, newlysignaled : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { WHvSignalVirtualProcessorSynicEvent(partition, core::mem::transmute(synicevent), newlysignaled.unwrap_or(core::mem::zeroed()) as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "Win32_winhvplatformdefs", feature = "Win32_winnt"))]
#[inline]
pub unsafe fn WHvStartPartitionMigration(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE) -> windows_core::Result<super::winnt::HANDLE> {
    windows_core::link!("winhvplatform.dll" "system" fn WHvStartPartitionMigration(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, migrationhandle : *mut super::winnt::HANDLE) -> windows_core::HRESULT);
    unsafe {
        let mut result__ = core::mem::zeroed();
        WHvStartPartitionMigration(partition, &mut result__).map(|| result__)
    }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvSuspendPartitionTime(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvSuspendPartitionTime(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE) -> windows_core::HRESULT);
    unsafe { WHvSuspendPartitionTime(partition) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvTranslateGva(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex: u32, gva: super::winhvplatformdefs::WHV_GUEST_VIRTUAL_ADDRESS, translateflags: super::winhvplatformdefs::WHV_TRANSLATE_GVA_FLAGS, translationresult: *mut super::winhvplatformdefs::WHV_TRANSLATE_GVA_RESULT, gpa: *mut super::winhvplatformdefs::WHV_GUEST_PHYSICAL_ADDRESS) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvTranslateGva(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex : u32, gva : super::winhvplatformdefs::WHV_GUEST_VIRTUAL_ADDRESS, translateflags : super::winhvplatformdefs::WHV_TRANSLATE_GVA_FLAGS, translationresult : *mut super::winhvplatformdefs::WHV_TRANSLATE_GVA_RESULT, gpa : *mut super::winhvplatformdefs::WHV_GUEST_PHYSICAL_ADDRESS) -> windows_core::HRESULT);
    unsafe { WHvTranslateGva(partition, vpindex, gva, translateflags, translationresult as _, gpa as _) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvUnmapGpaRange(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, guestaddress: super::winhvplatformdefs::WHV_GUEST_PHYSICAL_ADDRESS, sizeinbytes: u64) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvUnmapGpaRange(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, guestaddress : super::winhvplatformdefs::WHV_GUEST_PHYSICAL_ADDRESS, sizeinbytes : u64) -> windows_core::HRESULT);
    unsafe { WHvUnmapGpaRange(partition, guestaddress, sizeinbytes) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvUnmapVpciDeviceInterrupt(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid: u64, index: u32) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvUnmapVpciDeviceInterrupt(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid : u64, index : u32) -> windows_core::HRESULT);
    unsafe { WHvUnmapVpciDeviceInterrupt(partition, logicaldeviceid, index) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvUnmapVpciDeviceMmioRanges(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid: u64) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvUnmapVpciDeviceMmioRanges(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid : u64) -> windows_core::HRESULT);
    unsafe { WHvUnmapVpciDeviceMmioRanges(partition, logicaldeviceid) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvUnregisterPartitionDoorbellEvent(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, matchdata: *const super::winhvplatformdefs::WHV_DOORBELL_MATCH_DATA) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvUnregisterPartitionDoorbellEvent(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, matchdata : *const super::winhvplatformdefs::WHV_DOORBELL_MATCH_DATA) -> windows_core::HRESULT);
    unsafe { WHvUnregisterPartitionDoorbellEvent(partition, matchdata) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvUpdateTriggerParameters(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, parameters: *const super::winhvplatformdefs::WHV_TRIGGER_PARAMETERS, triggerhandle: super::winhvplatformdefs::WHV_TRIGGER_HANDLE) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvUpdateTriggerParameters(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, parameters : *const super::winhvplatformdefs::WHV_TRIGGER_PARAMETERS, triggerhandle : super::winhvplatformdefs::WHV_TRIGGER_HANDLE) -> windows_core::HRESULT);
    unsafe { WHvUpdateTriggerParameters(partition, parameters, triggerhandle) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvWriteGpaRange(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex: u32, guestaddress: super::winhvplatformdefs::WHV_GUEST_PHYSICAL_ADDRESS, controls: super::winhvplatformdefs::WHV_ACCESS_GPA_CONTROLS, data: *const core::ffi::c_void, datasizeinbytes: u32) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvWriteGpaRange(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, vpindex : u32, guestaddress : super::winhvplatformdefs::WHV_GUEST_PHYSICAL_ADDRESS, controls : super::winhvplatformdefs::WHV_ACCESS_GPA_CONTROLS, data : *const core::ffi::c_void, datasizeinbytes : u32) -> windows_core::HRESULT);
    unsafe { WHvWriteGpaRange(partition, vpindex, guestaddress, core::mem::transmute(controls), data, datasizeinbytes) }
}
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "Win32_winhvplatformdefs")]
#[inline]
pub unsafe fn WHvWriteVpciDeviceRegister(partition: super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid: u64, register: *const super::winhvplatformdefs::WHV_VPCI_DEVICE_REGISTER, data: *const core::ffi::c_void) -> windows_core::HRESULT {
    windows_core::link!("winhvplatform.dll" "system" fn WHvWriteVpciDeviceRegister(partition : super::winhvplatformdefs::WHV_PARTITION_HANDLE, logicaldeviceid : u64, register : *const super::winhvplatformdefs::WHV_VPCI_DEVICE_REGISTER, data : *const core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { WHvWriteVpciDeviceRegister(partition, logicaldeviceid, register, data) }
}
