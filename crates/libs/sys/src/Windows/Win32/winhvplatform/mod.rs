#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "winhvplatformdefs", feature = "winnt"))]
windows_link::link!("winhvplatform.dll" "system" fn WHvAcceptPartitionMigration(migrationhandle : super::HANDLE, partition : *mut super::WHV_PARTITION_HANDLE) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvAdviseGpaRange(partition : super::WHV_PARTITION_HANDLE, gparanges : *const super::WHV_MEMORY_RANGE_ENTRY, gparangescount : u32, advice : super::WHV_ADVISE_GPA_RANGE_CODE, advicebuffer : *const core::ffi::c_void, advicebuffersizeinbytes : u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "winhvplatformdefs", feature = "winnt"))]
windows_link::link!("winhvplatform.dll" "system" fn WHvAllocateVpciResource(providerid : *const windows_sys::core::GUID, flags : super::WHV_ALLOCATE_VPCI_RESOURCE_FLAGS, resourcedescriptor : *const core::ffi::c_void, resourcedescriptorsizeinbytes : u32, vpciresource : *mut super::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "C" fn WHvCancelPartitionMigration(partition : super::WHV_PARTITION_HANDLE) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvCancelRunVirtualProcessor(partition : super::WHV_PARTITION_HANDLE, vpindex : u32, flags : u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "C" fn WHvCompletePartitionMigration(partition : super::WHV_PARTITION_HANDLE) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "winhvplatformdefs", feature = "winnt"))]
windows_link::link!("winhvplatform.dll" "system" fn WHvCreateNotificationPort(partition : super::WHV_PARTITION_HANDLE, parameters : *const super::WHV_NOTIFICATION_PORT_PARAMETERS, eventhandle : super::HANDLE, porthandle : *mut super::WHV_NOTIFICATION_PORT_HANDLE) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvCreatePartition(partition : *mut super::WHV_PARTITION_HANDLE) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "winhvplatformdefs", feature = "winnt"))]
windows_link::link!("winhvplatform.dll" "system" fn WHvCreateTrigger(partition : super::WHV_PARTITION_HANDLE, parameters : *const super::WHV_TRIGGER_PARAMETERS, triggerhandle : *mut super::WHV_TRIGGER_HANDLE, eventhandle : *mut super::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvCreateVirtualProcessor(partition : super::WHV_PARTITION_HANDLE, vpindex : u32, flags : u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvCreateVirtualProcessor2(partition : super::WHV_PARTITION_HANDLE, vpindex : u32, properties : *const super::WHV_VIRTUAL_PROCESSOR_PROPERTY, propertycount : u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "winhvplatformdefs", feature = "winnt"))]
windows_link::link!("winhvplatform.dll" "system" fn WHvCreateVpciDevice(partition : super::WHV_PARTITION_HANDLE, logicaldeviceid : u64, vpciresource : super::HANDLE, flags : super::WHV_CREATE_VPCI_DEVICE_FLAGS, notificationeventhandle : super::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvDeleteNotificationPort(partition : super::WHV_PARTITION_HANDLE, porthandle : super::WHV_NOTIFICATION_PORT_HANDLE) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvDeletePartition(partition : super::WHV_PARTITION_HANDLE) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvDeleteTrigger(partition : super::WHV_PARTITION_HANDLE, triggerhandle : super::WHV_TRIGGER_HANDLE) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvDeleteVirtualProcessor(partition : super::WHV_PARTITION_HANDLE, vpindex : u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvDeleteVpciDevice(partition : super::WHV_PARTITION_HANDLE, logicaldeviceid : u64) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvGetCapability(capabilitycode : super::WHV_CAPABILITY_CODE, capabilitybuffer : *mut core::ffi::c_void, capabilitybuffersizeinbytes : u32, writtensizeinbytes : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvGetInterruptTargetVpSet(partition : super::WHV_PARTITION_HANDLE, destination : u64, destinationmode : super::WHV_INTERRUPT_DESTINATION_MODE, targetvps : *mut u32, vpcount : u32, targetvpcount : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvGetPartitionCounters(partition : super::WHV_PARTITION_HANDLE, counterset : super::WHV_PARTITION_COUNTER_SET, buffer : *mut core::ffi::c_void, buffersizeinbytes : u32, byteswritten : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvGetPartitionProperty(partition : super::WHV_PARTITION_HANDLE, propertycode : super::WHV_PARTITION_PROPERTY_CODE, propertybuffer : *mut core::ffi::c_void, propertybuffersizeinbytes : u32, writtensizeinbytes : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvGetVirtualProcessorCounters(partition : super::WHV_PARTITION_HANDLE, vpindex : u32, counterset : super::WHV_PROCESSOR_COUNTER_SET, buffer : *mut core::ffi::c_void, buffersizeinbytes : u32, byteswritten : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvGetVirtualProcessorCpuidOutput(partition : super::WHV_PARTITION_HANDLE, vpindex : u32, eax : u32, ecx : u32, cpuidoutput : *mut super::WHV_CPUID_OUTPUT) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvGetVirtualProcessorInterruptControllerState(partition : super::WHV_PARTITION_HANDLE, vpindex : u32, state : *mut core::ffi::c_void, statesize : u32, writtensize : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvGetVirtualProcessorInterruptControllerState2(partition : super::WHV_PARTITION_HANDLE, vpindex : u32, state : *mut core::ffi::c_void, statesize : u32, writtensize : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvGetVirtualProcessorRegisters(partition : super::WHV_PARTITION_HANDLE, vpindex : u32, registernames : *const super::WHV_REGISTER_NAME, registercount : u32, registervalues : *mut super::WHV_REGISTER_VALUE) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvGetVirtualProcessorState(partition : super::WHV_PARTITION_HANDLE, vpindex : u32, statetype : super::WHV_VIRTUAL_PROCESSOR_STATE_TYPE, buffer : *mut core::ffi::c_void, buffersizeinbytes : u32, byteswritten : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvGetVirtualProcessorXsaveState(partition : super::WHV_PARTITION_HANDLE, vpindex : u32, buffer : *mut core::ffi::c_void, buffersizeinbytes : u32, byteswritten : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvGetVpciDeviceInterruptTarget(partition : super::WHV_PARTITION_HANDLE, logicaldeviceid : u64, index : u32, multimessagenumber : u32, target : *mut super::WHV_VPCI_INTERRUPT_TARGET, targetsizeinbytes : u32, byteswritten : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvGetVpciDeviceNotification(partition : super::WHV_PARTITION_HANDLE, logicaldeviceid : u64, notification : *mut super::WHV_VPCI_DEVICE_NOTIFICATION, notificationsizeinbytes : u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvGetVpciDeviceProperty(partition : super::WHV_PARTITION_HANDLE, logicaldeviceid : u64, propertycode : super::WHV_VPCI_DEVICE_PROPERTY_CODE, propertybuffer : *mut core::ffi::c_void, propertybuffersizeinbytes : u32, writtensizeinbytes : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvMapGpaRange(partition : super::WHV_PARTITION_HANDLE, sourceaddress : *const core::ffi::c_void, guestaddress : super::WHV_GUEST_PHYSICAL_ADDRESS, sizeinbytes : u64, flags : super::WHV_MAP_GPA_RANGE_FLAGS) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "winhvplatformdefs", feature = "winnt"))]
windows_link::link!("winhvplatform.dll" "system" fn WHvMapGpaRange2(partition : super::WHV_PARTITION_HANDLE, process : super::HANDLE, sourceaddress : *const core::ffi::c_void, guestaddress : super::WHV_GUEST_PHYSICAL_ADDRESS, sizeinbytes : u64, flags : super::WHV_MAP_GPA_RANGE_FLAGS) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvMapVpciDeviceInterrupt(partition : super::WHV_PARTITION_HANDLE, logicaldeviceid : u64, index : u32, messagecount : u32, target : *const super::WHV_VPCI_INTERRUPT_TARGET, msiaddress : *mut u64, msidata : *mut u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvMapVpciDeviceMmioRanges(partition : super::WHV_PARTITION_HANDLE, logicaldeviceid : u64, mappingcount : *mut u32, mappings : *mut *mut super::WHV_VPCI_MMIO_MAPPING) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvPostVirtualProcessorSynicMessage(partition : super::WHV_PARTITION_HANDLE, vpindex : u32, sintindex : u32, message : *const core::ffi::c_void, messagesizeinbytes : u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvQueryGpaRangeDirtyBitmap(partition : super::WHV_PARTITION_HANDLE, guestaddress : super::WHV_GUEST_PHYSICAL_ADDRESS, rangesizeinbytes : u64, bitmap : *mut u64, bitmapsizeinbytes : u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvReadGpaRange(partition : super::WHV_PARTITION_HANDLE, vpindex : u32, guestaddress : super::WHV_GUEST_PHYSICAL_ADDRESS, controls : super::WHV_ACCESS_GPA_CONTROLS, data : *mut core::ffi::c_void, datasizeinbytes : u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvReadVpciDeviceRegister(partition : super::WHV_PARTITION_HANDLE, logicaldeviceid : u64, register : *const super::WHV_VPCI_DEVICE_REGISTER, data : *mut core::ffi::c_void) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "winhvplatformdefs", feature = "winnt"))]
windows_link::link!("winhvplatform.dll" "system" fn WHvRegisterPartitionDoorbellEvent(partition : super::WHV_PARTITION_HANDLE, matchdata : *const super::WHV_DOORBELL_MATCH_DATA, eventhandle : super::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvRequestInterrupt(partition : super::WHV_PARTITION_HANDLE, interrupt : *const super::WHV_INTERRUPT_CONTROL, interruptcontrolsize : u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvRequestVpciDeviceInterrupt(partition : super::WHV_PARTITION_HANDLE, logicaldeviceid : u64, msiaddress : u64, msidata : u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvResetPartition(partition : super::WHV_PARTITION_HANDLE) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvResumePartitionTime(partition : super::WHV_PARTITION_HANDLE) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvRetargetVpciDeviceInterrupt(partition : super::WHV_PARTITION_HANDLE, logicaldeviceid : u64, msiaddress : u64, msidata : u32, target : *const super::WHV_VPCI_INTERRUPT_TARGET) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvRunVirtualProcessor(partition : super::WHV_PARTITION_HANDLE, vpindex : u32, exitcontext : *mut core::ffi::c_void, exitcontextsizeinbytes : u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvSetNotificationPortProperty(partition : super::WHV_PARTITION_HANDLE, porthandle : super::WHV_NOTIFICATION_PORT_HANDLE, propertycode : super::WHV_NOTIFICATION_PORT_PROPERTY_CODE, propertyvalue : super::WHV_NOTIFICATION_PORT_PROPERTY) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvSetPartitionProperty(partition : super::WHV_PARTITION_HANDLE, propertycode : super::WHV_PARTITION_PROPERTY_CODE, propertybuffer : *const core::ffi::c_void, propertybuffersizeinbytes : u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvSetVirtualProcessorInterruptControllerState(partition : super::WHV_PARTITION_HANDLE, vpindex : u32, state : *const core::ffi::c_void, statesize : u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvSetVirtualProcessorInterruptControllerState2(partition : super::WHV_PARTITION_HANDLE, vpindex : u32, state : *const core::ffi::c_void, statesize : u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvSetVirtualProcessorRegisters(partition : super::WHV_PARTITION_HANDLE, vpindex : u32, registernames : *const super::WHV_REGISTER_NAME, registercount : u32, registervalues : *const super::WHV_REGISTER_VALUE) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvSetVirtualProcessorState(partition : super::WHV_PARTITION_HANDLE, vpindex : u32, statetype : super::WHV_VIRTUAL_PROCESSOR_STATE_TYPE, buffer : *const core::ffi::c_void, buffersizeinbytes : u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvSetVirtualProcessorXsaveState(partition : super::WHV_PARTITION_HANDLE, vpindex : u32, buffer : *const core::ffi::c_void, buffersizeinbytes : u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "winhvplatformdefs", feature = "winnt"))]
windows_link::link!("winhvplatform.dll" "system" fn WHvSetVpciDevicePowerState(partition : super::WHV_PARTITION_HANDLE, logicaldeviceid : u64, powerstate : super::DEVICE_POWER_STATE) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvSetupPartition(partition : super::WHV_PARTITION_HANDLE) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvSignalVirtualProcessorSynicEvent(partition : super::WHV_PARTITION_HANDLE, synicevent : super::WHV_SYNIC_EVENT_PARAMETERS, newlysignaled : *mut windows_sys::core::BOOL) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(all(feature = "winhvplatformdefs", feature = "winnt"))]
windows_link::link!("winhvplatform.dll" "system" fn WHvStartPartitionMigration(partition : super::WHV_PARTITION_HANDLE, migrationhandle : *mut super::HANDLE) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvSuspendPartitionTime(partition : super::WHV_PARTITION_HANDLE) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvTranslateGva(partition : super::WHV_PARTITION_HANDLE, vpindex : u32, gva : super::WHV_GUEST_VIRTUAL_ADDRESS, translateflags : super::WHV_TRANSLATE_GVA_FLAGS, translationresult : *mut super::WHV_TRANSLATE_GVA_RESULT, gpa : *mut super::WHV_GUEST_PHYSICAL_ADDRESS) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvUnmapGpaRange(partition : super::WHV_PARTITION_HANDLE, guestaddress : super::WHV_GUEST_PHYSICAL_ADDRESS, sizeinbytes : u64) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvUnmapVpciDeviceInterrupt(partition : super::WHV_PARTITION_HANDLE, logicaldeviceid : u64, index : u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvUnmapVpciDeviceMmioRanges(partition : super::WHV_PARTITION_HANDLE, logicaldeviceid : u64) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvUnregisterPartitionDoorbellEvent(partition : super::WHV_PARTITION_HANDLE, matchdata : *const super::WHV_DOORBELL_MATCH_DATA) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvUpdateTriggerParameters(partition : super::WHV_PARTITION_HANDLE, parameters : *const super::WHV_TRIGGER_PARAMETERS, triggerhandle : super::WHV_TRIGGER_HANDLE) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvWriteGpaRange(partition : super::WHV_PARTITION_HANDLE, vpindex : u32, guestaddress : super::WHV_GUEST_PHYSICAL_ADDRESS, controls : super::WHV_ACCESS_GPA_CONTROLS, data : *const core::ffi::c_void, datasizeinbytes : u32) -> windows_sys::core::HRESULT);
#[cfg(any(target_arch = "aarch64", target_arch = "arm64ec", target_arch = "x86_64"))]
#[cfg(feature = "winhvplatformdefs")]
windows_link::link!("winhvplatform.dll" "system" fn WHvWriteVpciDeviceRegister(partition : super::WHV_PARTITION_HANDLE, logicaldeviceid : u64, register : *const super::WHV_VPCI_DEVICE_REGISTER, data : *const core::ffi::c_void) -> windows_sys::core::HRESULT);
