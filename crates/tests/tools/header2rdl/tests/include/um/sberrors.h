//
// Copyright (c) 1998-2008  Microsoft Corporation. All Rights Reserved.
//
// Module Name: tscerror.h
//
// Abstract:
//
//      Session Broker TSV Internal Error Codes
//
// Author:
//
//      Mahadev Alladi
//
// Revision History:
//
//      1/24/2008 Created
//


#ifndef _SB_ERRORS_H_
#define _SB_ERRORS_H_

#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


///////////////////////////////////////////////////////////////////////////////
//
// Error codes
//

#define FACILITY_SB  0x813

#define MAKE_SBTSV_ERR(x) MAKE_HRESULT(SEVERITY_ERROR, FACILITY_SB, x)

///////////////////////////////////////////////////////////////////////////////
//
// Base error codes
//

// TSV SBerrors start at 1
#define MAKE_SB_ERR(x)                                  MAKE_SBTSV_ERR(x)

//
// No disconnected session found
//
#define E_SB_NO_DISCONNECTED_SESSION                    MAKE_SB_ERR(1)

//
// Unable to find a resource plugin
//
#define E_SB_NO_RESOURCE_PLUGIN                         MAKE_SB_ERR(2)

//
// Host not found
//
#define E_SB_TARGET_NOT_FOUND                           MAKE_SB_ERR(3)

//
// host environment not found
//
#define E_SB_ENVIRONMENT_NOT_FOUND                      MAKE_SB_ERR(4)

//
// Unable to find a filter plugin
//
#define E_SB_NO_FILTER_PLUGIN                           MAKE_SB_ERR(5)

//
// no hint data
//
#define E_SB_NO_HINT_DATA                               MAKE_SB_ERR(6)

//
// Load Balancing failed
//
#define E_SB_LOAD_BAL_FAILED                            MAKE_SB_ERR(7)

//
// Query Placement Failed
//
#define E_SB_QUERY_PLACEMENT_FAILED                     MAKE_SB_ERR(8)

//
// Create Placement failed
//
#define E_SB_CREATE_PLACEMENT_FAILED                    MAKE_SB_ERR(9)

//
// Orchestration failed
//
#define E_SB_ORCHESTRATION_FAILED                       MAKE_SB_ERR(10)

//
// Disconnected session check failed
//
#define E_SB_DISCONN_SESSION_CHECK_FAILED               MAKE_SB_ERR(11)

//
// Policy Plugin process request failed
//
#define E_SB_POLICY_PLUGIN_PROC_REQ_FAILED              MAKE_SB_ERR(12)

//
// CLSID of the Plugin (in the registry) is invalid or absent
//
#define E_SB_NO_PLUGIN_CLSID_REG                        MAKE_SB_ERR(13)

//
// Provider of the Plugin (in the registry) is invalid or absent
//
#define E_SB_NO_PLUGIN_PROVIDER_REG                     MAKE_SB_ERR(14)

//
// Plugin is disabled (from the registry) or IsEnabled=0
//
#define E_SB_PLUGIN_DISABLED_REG                        MAKE_SB_ERR(15)

//
// Plugin has implemented ITsSbResourcePlugin, but did not implement from
// ITsSbLoadBalancing, ITsSbPlacement, ITsSbOrchestration. All should be 
// implemented by a resource plugin
//
#define E_SB_BAD_RESOURCE_PLUGIN                        MAKE_SB_ERR(16)

//
// A Filter Plugin should implement atleast one of the following interfaces
// ITsSbLoadBalancing, ITsSbPlacement, ITsSbOrchestration. 
//
#define E_SB_BAD_FILTER_PLUGIN                          MAKE_SB_ERR(17)

//
// Session Broker Computers group is empty
//
#define E_SB_GROUP_EMPTY                                MAKE_SB_ERR(18)

//
// Session Broker Computers group does not exist
//
#define E_SB_GROUP_NOT_EXIST                            MAKE_SB_ERR(19)

//
// Target supplied by the plugin doesnt have IP address.
//
#define E_SB_NO_TARGET_IP_ADDRESS                       MAKE_SB_ERR(20)

//
// Failed to redirect to destination
//
#define E_SB_REDIRECT_TO_DESTINATION_FAILED             MAKE_SB_ERR(21)

//
// Failed to wake VM (specific to VM plugin)
//
#define E_SB_VM_WAKE_FAILED                             MAKE_SB_ERR(22)

//
// Failed to boot a VM (specific to VM plugin)
//
#define E_SB_VM_BOOT_FAILED                             MAKE_SB_ERR(23)

//
// Target supplied by the plugin doesnt have IP address.
//
#define E_SB_FARM_NOT_FOUND                             MAKE_SB_ERR(24)

//
// Session Broker Service is stopping.
//
#define E_SB_SERVICE_STOPPING                           MAKE_SB_ERR(25)

//
// Target is not in the same Farm as expected
//
#define E_SB_TARGET_IN_DIFFERENT_FARM                   MAKE_SB_ERR(26)

//
// Target and Target Type don't match
//
#define E_SB_TARGET_TYPE_MISMATCH                       MAKE_SB_ERR(27)

//
// The format of the TSV URL is incorrect
//
#define E_SB_WRONG_TSV_URL_FORMAT                       MAKE_SB_ERR(28)

//
// Unidentified Target Type
//
#define E_SB_UNIDENTIFIED_TARGET_TYPE                   MAKE_SB_ERR(29)

//
// Error in publishing service returned data
//
#define E_SB_WRONG_USER_DATA_FROM_PUB_SERVICE           MAKE_SB_ERR(30)

//
// The tsv URL is not present
//
#define E_SB_TSV_URL_NOT_PRESENT                        MAKE_SB_ERR(31)

//
// Unknown Resource Plugin Provider
//
#define E_SB_UNKNOWN_RESOURCE_PLUGIN                    MAKE_SB_ERR(32)

//
// Unknown Central Publishing Plugin Guid
//
#define E_SB_UNKNOWN_CP_PLUGIN                          MAKE_SB_ERR(33)

//
// NULL Central Publishing Plugin Guid
//
#define E_SB_NULL_CP_PLUGIN                             MAKE_SB_ERR(34)

//
// State of a resource (target/session) is unchanged 
// but broker still received a OnStateChange call.
//
#define E_SB_RESOURCE_STATE_UNCHANGED                   MAKE_SB_ERR(35)

//
// Load balancing was attempted on a target that
// is not available for that.
//
#define E_SB_TARGET_LB_NOT_AVAILABLE                    MAKE_SB_ERR(36)

//
// Target state cannot be idle when there are
// existing sessions associated with it.
//
#define E_SB_TARGET_IN_USE                              MAKE_SB_ERR(37)

//
// There was a synchronization conflict in the database while saving the object.
//
#define E_SB_SYNCH_CONFLICT                             MAKE_SB_ERR(38)

//
// The call to broker WMI failed.
//
#define E_SB_BROKER_WMI_FAIL                            MAKE_SB_ERR(39)




///////////////////////////////////
// VM Provisioning errors//////////
///////////////////////////////////

#define SB_PROV_ERR_BASE    0x100


///////////////////////////////
//Provisioning errors at broker
///////////////////////////////

//
// Broker failed to delete M/C account for the VM at Active directory
//      Probable causes:
//          - Broker m/c account does not have required permission on the requested OU
//          - AD DC unreachable
//      Event log gives more detailed info about the failure
//
#define E_SB_PROV_DELETE_VM_AD_ACCOUNT_FAILED           MAKE_SBTSV_ERR( SB_PROV_ERR_BASE + 1 )

//
// Broker failed to find the specified VMHost name in the list of currently connected VMHosts
//      Probable causes:
//          - Incorrect hyperV host name
//          - specified hyperV host is not joined to VM host
//          - specified hyperV host is not running/unreachable/VMHA service not running
//      Event log gives more detailed info about the failure
//
#define E_SB_PROV_INVALID_HYPERV_HOST_NAME              MAKE_SBTSV_ERR( SB_PROV_ERR_BASE + 2 )

//
// RPC Call from Broker to HyperV host failed
//      Probable causes:
//          - network connectivity with the hyperV host is lost
//          - hypreV host/VMHA service crashed
//      Event log gives more detailed info about the failure
//
#define E_SB_PROV_HYPERV_HOST_RPC_FAILED                MAKE_SBTSV_ERR( SB_PROV_ERR_BASE + 3 )

//
// The VM specified in the CREATE pool request already exists in the deployment
//
#define E_SB_PROV_VM_ALREADY_EXIST                      MAKE_SBTSV_ERR( SB_PROV_ERR_BASE + 4 )

//
// The VM name specified in the CREATE pool request is longer then supported Netbios name
//
#define E_SB_PROV_VM_NAME_TOO_LONG                      MAKE_SBTSV_ERR( SB_PROV_ERR_BASE + 5 )

//
// The provisioning operation was cancelled because 
//      - provisioning op. for some other VM in the Job request failed and StopOnError was specified.
//      - Job was cancelled by user
//
#define E_SB_PROV_OPERATION_CANCELLED                   MAKE_SBTSV_ERR( SB_PROV_ERR_BASE + 6 )

//
// No HyperV host is connected to the broker
//
#define E_SB_PROV_NO_HYPERV_HOST_AVAILABLE              MAKE_SBTSV_ERR( SB_PROV_ERR_BASE + 7 )

//
// the VM specified in the selectedVMs list in the provisioning job is not present in broker Target database
//
#define E_SB_PROV_SELECTED_VM_NOT_FOUND                 MAKE_SBTSV_ERR( SB_PROV_ERR_BASE + 8 )

//TODO: added error code Operation aborted to distinguish between user cancelled action and 
// provisioning operation aborted when stop on error was specified.

//
// The Wmi object (win32_RdPublishedFarm) does not exist for the pool name in the Provisioning request
//
#define E_SB_PROV_POOL_NOT_FOUND                        MAKE_SBTSV_ERR( SB_PROV_ERR_BASE + 9 )

//
// The specified unattend XML is not supported
//
#define E_SB_PROV_UNATTEND_XML_NOT_SUPPORTED            MAKE_SBTSV_ERR( SB_PROV_ERR_BASE + 10 )

//
// The specified unattend XML is not supported
//
#define E_SB_PROV_INVALID_UNATTEND_XML                  MAKE_SBTSV_ERR( SB_PROV_ERR_BASE + 11 )

//
// The AD machine account already exists for the specified VM name
//
#define E_SB_PROV_VM_AD_ACCOUNT_ALREADY_EXISTS          MAKE_SBTSV_ERR( SB_PROV_ERR_BASE + 12 )

//
// Broker failed to create M/C account for the VM at Active Directory.
//      Probable causes:
//          - Broker m/c account does not have required permission on the requested OU
//          - AD DC unreachable
//          - Machine account already exists in a different OU
//      Event log gives more detailed info about the failure
//
#define E_SB_PROV_CREATE_VM_AD_ACCOUNT_FAILED           MAKE_SBTSV_ERR( SB_PROV_ERR_BASE + 13 )

////////////////////////////////
//provisioning errors at VMHA
///////////////////////////////

#define SB_PROV_VMHA_ERR_BASE    0x200

//
// VMHost Agent failed to access the base VM location
//      Probable causes:
//          - Invalid Base VM Location
//          - No read/write permission to VMHA service/VMHA Machine account on Base VM location
//          - if BaseVmLocation is on SMB, then there could N/W connectivity issues
//      Event log gives more detailed info about the failure
//
#define E_SB_PROV_INVALID_BASE_VM_PATH                  MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 0 )

//
// VMHost Agent failed to access the Local Gold Cache location
//      Probable causes:
//          - Invalid Local gold cache Location
//          - No read/write permission to VMHA service/VMHA Machine account
//          - Non local gold cache location
//      Event log gives more detailed info about the failure
//
#define E_SB_PROV_INVALID_CACHE_PATH                    MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 1 )


//
// VMHost Agent failed to access the VM creation path
//      Probable causes:
//          - Invalid Local vm creation path
//          - No read/write permission to VMHA service/VMHA Machine account
//          - Non local VM creation path
//      Event log gives more detailed info about the failure
//
#define E_SB_PROV_INVALID_VM_CREATION_PATH              MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 2 )

//
// VMHost Agent failed to access the SMB
//      Probable causes:
//          - Invalid SMB path
//          - No read/write permission to VMHA service/VMHA Machine account
//          - local Path
//          - Network connectivity issues to SMB share
//      Event log gives more detailed info about the failure
//
#define E_SB_PROV_INVALID_SMB_SHARE_PATH                MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 3 )

//
// VMHost Agent failed to cache the Gold VM from Base VM location to LocalGOldCache
//      Probable causes:
//          - N/w connectivity issues
//          - Disk ran out of space
//      Event log gives more detailed info about the failure
//
#define E_SB_PROV_MASTER_VHD_CACHING_FAILED             MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 4 )

//
// VMHost Agent failed to create the diff disk
//      Probable causes:
//          - Gold VHD is inaccessbile
//          - HyperV specific error
//      VMHA and hyperV Event log gives more detailed info about the failure
//
#define E_SB_PROV_CREATE_DIFF_VHD_FAILED                MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 5 )

//
// TODO: Applicable when we move to V2 namespace
//
#define E_SB_PROV_CREATE_CLONE_VHD_FAILED               MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 6 )

//
// VMHost Agent failed to start the VM
//      Probable causes:
//          - System low on resources (RAM, Procs)
//          - HyperV specific error
//      VMHA and hyperV Event log gives more detailed info about the failure
//
#define E_SB_PROV_VM_START_FAIL                         MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 7 )

//
// VMHost Agent failed to turn off the VM
//      Probable causes:
//          - Invalid VM state
//          - HyperV specific failure
//      VMHA and hyperV Event log gives more detailed info about the failure
//
#define E_SB_PROV_VM_TURN_OFF_FAIL                      MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 8 )

//
// VMHost Agent failed to take a snap shot of the temp pool VM
//      Probable causes:
//          - HyperV specific failure
//      VMHA and hyperV Event log gives more detailed info about the failure
//
#define E_SB_PROV_VM_SNAPSHOT_FAIL                      MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 9)

//
// VMHost Agent failed to detect BOOT event from VM, VM failed to boot.
//      Probable causes:
//          - invalid unattend file
//          - unexpected failure during sysprep specialization
//      Event log gives more detailed info about the failure
//
#define E_SB_PROV_VM_BOOT_FAIL                          MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 10)

//
// VMHost Agent failed to migrate the VM storage to SMB
//      Probable causes:
//          - SMB ran out of space
//          - network connectivity issues
//          - hyperv specific failure
//      VMHA and hyperV Event Event log gives more detailed info about the failure
//
#define E_SB_PROV_MIGRATE_STORAGE_FAILED                MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 11 )

//
// VMHost Agent failed to import from BASE VM Location
//      Probable causes:
//          - Invalid import files in BaseVmLocation
//          - hyperV configuration does not match the host from where gold was exported (e.g. External Virtual N/W switch name)
//          - hyperV server does not have required resources
//      VMHA and hyperV Event log gives more detailed info about the failure
//
#define E_SB_PROV_IMPORT_FAILED                         MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 12 )

//
// VMHA service failed to attach the VHD to the VM
//      Probable causes:
//          - IDE controller not present
//          - VM in invalid state
//      VMHA and hyperV Event log gives more detailed info about the failure
//
#define E_SB_PROV_CONNECT_VHD_FAILED                    MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 13 )

//
// VMHost Agent failed to disconnect VHD from VM
//      Probable causes:
//          - Invalid vm state, e.g. VM running
//          - hyperV specific failure
//      VMHA and hyperV Event log gives more detailed info about the failure
//
#define E_SB_PROV_DISCONNECT_VHD_FAILED                 MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 14 )

//
// VMHost Agent failed to
//      Probable causes:
//          - Failed to mount the VHD
//          - Invalid gold image
//          - 
//      VMHA and hyperV Event log gives more detailed info about the failure
//
#define E_SB_PROV_SETUP_VHD_FAILED                      MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 15)

//
// VMHost Agent failed to
//      Probable causes:
//          - VM not in turned off state, e.g. VM running
//          - 
//      VMHA and hyperV Event log gives more detailed info about the failure
//
#define E_SB_PROV_GEN_QUERY_OFFVM_FAILURE              MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 16)

//
// VMHost Agent failed to
//      Probable causes:
//          - VM not in turned off state, e.g. VM running
//          - 
//      VMHA and hyperV Event log gives more detailed info about the failure
//
#define E_SB_PROV_VM_NOT_TURNED_OFF                      MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 17)


//
// VMHost Agent failed to
//      Probable causes:
//          - Multiple VHDs are attached to the VM, we support only one
//          - 
//      VMHA and hyperV Event log gives more detailed info about the failure
//
#define E_SB_PROV_MULTIPLE_VHD_DISKS                     MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 18)

//
// VMHost Agent failed to
//      Probable causes:
//          - No VHD found for this VM
//
//      VMHA and hyperV Event log gives more detailed info about the failure
//
#define E_SB_PROV_NO_VHD_FOUND                          MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 19)


//
// VMHost Agent failed to
//      Probable causes:
//          - Multiple WINDOWS folders in a single VHD
//          - 
//      VMHA and hyperV Event log gives more detailed info about the failure
//
#define E_SB_PROV_MULTIPLE_WINDOWS_FOLDERS              MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 20)

//
// VMHost Agent failed to
//      Probable causes:
//          - Gold VM is not in generalized state
//          - 
//
#define E_SB_PROV_GOLDVM_NOT_GENERALIZED               MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 21)

//
// VMHost Agent failed to
//      Probable causes:
//          - Gold VM does not have the latest IC package
//          - 
//
#define E_SB_PROV_GOLDVM_IC_UPGRADE_REQUIRED           MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 22)

//
// VMHost Agent failed to verify gold VM export share
//      Probable causes:
//          - Import operation failed on gold cache during initial validation
//          - VmHost there gold exited and the Vmhost which returned this error are not configured identically
//          - The export share is corrupted
//
#define E_SB_PROV_GOLDVM_INVALID_EXPORT_SHARE           MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 23)

//
// VMHost Agent did not find any network adapter attached to the VM
//
#define E_SB_PROV_NO_NETWORK_ADAPTER_ATTACHED           MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 24)

//
// VMHost Agent did not find the Network connected to the network adapter of the VM
//
#define E_SB_PROV_NETWORK_NOT_CONNECTED                 MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 25)

//
// VMHost Agent failed to add a VM to a Farm, because the VM does not have a SCSI controller. Please configure
// a SCSI controller in the gold image of the Farm, re-create the VM and re-try adding it to the Farm.
//
#define E_VM_NEEDS_SCSI_CONTROLLER                      MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 26)

//
// VMHost Agent failed to provision VM because there is not enough RAM assigned to Gold VM
// If Dynamic Memory is enabled, then the Max RAM must be set to at least 1024MB,
// otherwise, Startup RAM must be at least 1024GB
//
#define E_SB_PROV_GOLDVM_NOT_ENOUGH_RAM                 MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 27)

//
// ServerSKUs are not supported as the GuestOS.
//
#define E_SB_PROV_SERVER_SKU_UNSUPPORTED                MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 28)

//
// Timed out waiting for SysPrep specialize to be complete
//
#define E_SB_PROV_VM_SYSPREP_SPECIALIZE_INCOMPLETE      MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 29 )

//
// Sysprep specialize is complete, not an error in most cases
//
#define E_SB_PROV_VM_SYSPREP_SPECIALIZE_COMPLETE       MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 30 )

//
// Failed to successfully detect VM's domain-joinedness
//
#define E_SB_PROV_VM_FAILED_DOMAIN_CHECK               MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 31 )

//
// Failed to communicate with the guest
//
#define E_SB_PROV_VM_FAILED_GUEST_COMMUNICATION        MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 32 )

//
// VM is not in fully deployable state
//
#define E_SB_PROV_VM_NOT_DEPLOYABLE_STATE              MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 33 )

//
// VM is in an invalid state for provisioning operation
//
#define E_SB_PROV_INVALID_VM_STATE                     MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 34 )

//
// VM is in an invalid state for provisioning operation
//
#define E_SB_PROV_VM_GENERATION_NOT_SUPPORTED          MAKE_SBTSV_ERR( SB_PROV_VMHA_ERR_BASE + 35 )

//
// Unknown error occurred
//
#define E_SB_UNKNOWN                                    MAKE_SB_ERR(0x500)


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif //_SB_ERRORS_H_
