/*** infstr.h - SetupAPI INF string definitions
 *
 *  This module contains public registry string definitions.
 *
 *  Copyright (c) Microsoft Corporation.  All rights reserved.
 *
 *  MODIFICATION HISTORY
 */


#ifndef _INC_INFSTR
#define _INC_INFSTR

#if _MSC_VER > 1000
#pragma once
#endif
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


//***************************************************************************
//
// Misc. key strings used by Setup Info. File
//
//***************************************************************************

#define MAX_KEY_LEN             100

#define SZ_KEY_OPTIONDESC            TEXT("OptionDesc")
#define SZ_KEY_LDIDOEM               TEXT("LdidOEM")
#define SZ_KEY_SRCDISKFILES          TEXT("SourceDisksFiles")
#define SZ_KEY_SRCDISKNAMES          TEXT("SourceDisksNames")
#define SZ_KEY_STRINGS               TEXT("Strings")
#define SZ_KEY_DESTDIRS              TEXT("DestinationDirs")
#define SZ_KEY_LAYOUT_FILE           TEXT("LayoutFile")
#define SZ_KEY_DEFDESTDIR            TEXT("DefaultDestDir")
#define SZ_KEY_LFN_SECTION           TEXT("VarLDID.LFN")
#define SZ_KEY_SFN_SECTION           TEXT("VarLDID.SFN")

#define SZ_KEY_UPDATEINIS            TEXT("UpdateInis")
#define SZ_KEY_UPDATEINIFIELDS       TEXT("UpdateIniFields")
#define SZ_KEY_INI2REG               TEXT("Ini2Reg")
#define SZ_KEY_COPYFILES             TEXT("CopyFiles")
#define SZ_KEY_RENFILES              TEXT("RenFiles")
#define SZ_KEY_DELFILES              TEXT("DelFiles")
#define SZ_KEY_ADDREG                TEXT("AddReg")
#define SZ_KEY_ADDREGNOCLOBBER       TEXT("AddRegNoClobber")
#define SZ_KEY_DELREG                TEXT("DelReg")
#define SZ_KEY_BITREG                TEXT("BitReg")
#define SZ_KEY_COPYINF               TEXT("CopyINF")
#define SZ_KEY_LOGCONFIG             TEXT("LogConfig")
#define SZ_KEY_ADDSERVICE            TEXT("AddService")
#define SZ_KEY_DELSERVICE            TEXT("DelService")
#define SZ_KEY_ADDTRIGGER            TEXT("AddTrigger")
#define SZ_KEY_FAILUREACTIONS        TEXT("FailureActions")
#define SZ_KEY_ADDINTERFACE          TEXT("AddInterface")
#define SZ_KEY_ADDIME                TEXT("AddIme")
#define SZ_KEY_DELIME                TEXT("DelIme")
#define SZ_KEY_REGSVR                TEXT("RegisterDlls")
#define SZ_KEY_UNREGSVR              TEXT("UnregisterDlls")
#define SZ_KEY_PROFILEITEMS          TEXT("ProfileItems")
#define SZ_KEY_MODULES               TEXT("Modules")
#define SZ_KEY_DEFAULTOPTION         TEXT("DefaultOption")
#define SZ_KEY_LISTOPTIONS           TEXT("ListOptions")
#define SZ_KEY_CLEANONLY             TEXT("CleanOnly")
#define SZ_KEY_UPGRADEONLY           TEXT("UpgradeOnly")
#define SZ_KEY_EXCLUDEID             TEXT("ExcludeId")
#define SZ_KEY_ADDPOWERSETTING       TEXT("AddPowerSetting")
#define SZ_KEY_ADDPROP               TEXT("AddProperty")
#define SZ_KEY_DELPROP               TEXT("DelProperty")
#define SZ_KEY_FEATURESCORE          TEXT("FeatureScore")
#define SZ_KEY_ADDEVENTPROVIDER      TEXT("AddEventProvider")
#define SZ_KEY_ADDCOMSERVER          TEXT("AddComServer")
#define SZ_KEY_ADDCOMCLASS           TEXT("AddComClass")
#define SZ_KEY_ADDCHANNEL            TEXT("AddChannel")
#define SZ_KEY_IMPORTCHANNEL         TEXT("ImportChannel")
#define SZ_KEY_ADDAUTOLOGGER         TEXT("AddAutoLogger")
#define SZ_KEY_UPDATEAUTOLOGGER      TEXT("UpdateAutoLogger")
#define SZ_KEY_ADDAUTOLOGGERPROVIDER TEXT("AddAutoLoggerProvider")
#define SZ_KEY_ADDFILTER             TEXT("AddFilter")
#define SZ_KEY_FILTERLEVEL           TEXT("FilterLevel")
#define SZ_KEY_FILTERPOSITION        TEXT("FilterPosition")
#define SZ_KEY_ADDCOMPONENT          TEXT("AddComponent")

// These are here for compatability with SetupX
// They are registry keys, and are not used by SetupAPI
#define SZ_KEY_PHASE1           TEXT("Phase1")
#define SZ_KEY_HARDWARE         TEXT("Hardware")

// Foll. char is used to enclose a STRING KEY -- A key enclosed by this char
// should be in the [Strings] section of the INF file.
#define CH_STRINGKEY            TEXT('%')

// Foll. char is used to specify that what follows it is a file name
// rather than a section with files in the Copy= file of a Generic
// Install_Section.
//
#define CH_FILESPECIFIER        TEXT('@')



/*** Strings that will be used in the PnP INF files to specify
 *   LogConfig information, etc. This will be used to update the
 *   registry appropriately.
 */

#define INFSTR_KEY_CONFIGPRIORITY       TEXT("ConfigPriority")

// Foll. is length of buffer for the strings like HARDWIRED, etc.
#define MAX_PRIORITYSTR_LEN     16

/*** Foll. are strings that can be used for ConfigPriority=
 */
#define INFSTR_CFGPRI_HARDWIRED         TEXT("HARDWIRED")
#define INFSTR_CFGPRI_DESIRED           TEXT("DESIRED")
#define INFSTR_CFGPRI_NORMAL            TEXT("NORMAL")
#define INFSTR_CFGPRI_SUBOPTIMAL        TEXT("SUBOPTIMAL")
#define INFSTR_CFGPRI_DISABLED          TEXT("DISABLED")
#define INFSTR_CFGPRI_RESTART           TEXT("RESTART")
#define INFSTR_CFGPRI_REBOOT            TEXT("REBOOT")
#define INFSTR_CFGPRI_POWEROFF          TEXT("POWEROFF")
#define INFSTR_CFGPRI_HARDRECONFIG      TEXT("HARDRECONFIG")
#define INFSTR_CFGPRI_FORCECONFIG       TEXT("FORCECONFIG")

#define INFSTR_CFGTYPE_BASIC            TEXT("BASIC")
#define INFSTR_CFGTYPE_FORCED           TEXT("FORCED")
#define INFSTR_CFGTYPE_OVERRIDE         TEXT("OVERRIDE")


#define INFSTR_KEY_MEMCONFIG            TEXT("MemConfig")
#define INFSTR_KEY_MEMLARGECONFIG       TEXT("MemLargeConfig")
#define INFSTR_KEY_IOCONFIG             TEXT("IOConfig")
#define INFSTR_KEY_IRQCONFIG            TEXT("IRQConfig")
#define INFSTR_KEY_DMACONFIG            TEXT("DMAConfig")
#define INFSTR_KEY_PCCARDCONFIG         TEXT("PcCardConfig")
#define INFSTR_KEY_MFCARDCONFIG         TEXT("MfCardConfig")

//
//  Used to install a class installer
//
#define INFSTR_SECT_CLASS_INSTALL       TEXT("ClassInstall")
#define INFSTR_SECT_CLASS_INSTALL_32    TEXT("ClassInstall32")

//
// Used to install a primitive INF
//
#define INFSTR_SECT_DEFAULT_INSTALL     TEXT("DefaultInstall")
#define INFSTR_SECT_DEFAULT_UNINSTALL   TEXT("DefaultUninstall")

//
//  Used to install an interface class
//
#define INFSTR_SECT_INTERFACE_INSTALL_32 TEXT("InterfaceInstall32")

//  General information about the contents/origins of the .INF.
#define INFSTR_SECT_VERSION             TEXT("Version")

//  Provider name under [version] section
#define INFSTR_KEY_PROVIDER             TEXT("Provider")

// Signature under [version] section indicates a Win95-style device INF
#define INFSTR_KEY_SIGNATURE            TEXT("Signature")

//  Dependent driver subset under [Version] section
#define INFSTR_KEY_DRIVERSET             TEXT("DriverSet")

//  [Version]
//  Specifies what the hardware class of any devices contained in this .INF.
#define MAX_INF_FLAG                    20
#define INFSTR_KEY_HARDWARE_CLASS       TEXT("Class")
#define INFSTR_KEY_HARDWARE_CLASSGUID   TEXT("ClassGUID")
#define INFSTR_KEY_NOSETUPINF           TEXT("NoSetupInf")
#define INFSTR_KEY_FROMINET             TEXT("FromINet")
#define INFSTR_KEY_CATALOGFILE          TEXT("CatalogFile")
#define INFSTR_KEY_PNPLOCKDOWN          TEXT("PnpLockDown")
#define INFSTR_KEY_EXTENSIONID          TEXT("ExtensionId")

//
//  Manufacturer section name
//
#define INFSTR_SECT_MFG                 TEXT("Manufacturer")

#define INFSTR_SECT_TARGETCOMPUTERS     TEXT("TargetComputers")

#define INFSTR_SECT_EXTENSIONCONTRACTS  TEXT("ExtensionContracts")

//
//  Specifies the hardware class of this device.
//
#define INFSTR_KEY_CLASS                TEXT("Class")
#define INFSTR_KEY_CLASSGUID            TEXT("ClassGUID")

//
//  Used by (Setup)DiInstallDevice to know that need to reboot or restart after
//  installing the device.
//
#define INFSTR_RESTART                  TEXT("Restart")
#define INFSTR_REBOOT                   TEXT("Reboot")

//
// Used by SetupDiInstallDevice to specify the service parameters passed
// to the Service Control Manager to create/modify a service.
//
#define INFSTR_KEY_DISPLAYNAME          TEXT("DisplayName")
#define INFSTR_KEY_SERVICETYPE          TEXT("ServiceType")
#define INFSTR_KEY_STARTTYPE            TEXT("StartType")
#define INFSTR_KEY_ERRORCONTROL         TEXT("ErrorControl")
#define INFSTR_KEY_SERVICEBINARY        TEXT("ServiceBinary")
#define INFSTR_KEY_LOADORDERGROUP       TEXT("LoadOrderGroup")
#define INFSTR_KEY_DEPENDENCIES         TEXT("Dependencies")
#define INFSTR_KEY_REQUIREDPRIVILEGES   TEXT("RequiredPrivileges")
#define INFSTR_KEY_STARTNAME            TEXT("StartName")
#define INFSTR_KEY_SECURITY             TEXT("Security")
#define INFSTR_KEY_DESCRIPTION          TEXT("Description")
#define INFSTR_KEY_SERVICESIDTYPE       TEXT("ServiceSidType")
#define INFSTR_KEY_DELAYEDAUTOSTART     TEXT("DelayedAutoStart")
#define INFSTR_KEY_BOOTFLAGS            TEXT("BootFlags")

//
// Used for Triggers within a Service section.
//
#define INFSTR_KEY_TRIGGER_TYPE         TEXT("TriggerType")
#define INFSTR_KEY_ACTION               TEXT("Action")
#define INFSTR_KEY_SUB_TYPE             TEXT("SubType")
#define INFSTR_KEY_DATA_ITEM            TEXT("DataItem")

//
// Used for FailureActions within a Service section
//
#define INFSTR_KEY_RESET_PERIOD         TEXT("ResetPeriod")
#define INFSTR_KEY_NON_CRASH_FAILURES   TEXT("NonCrashFailures")
#define INFSTR_KEY_FAILURE_ACTION       TEXT("Action")

//
// Used in event provider section
//
#define INFSTR_KEY_PROVIDER_NAME        TEXT("ProviderName")
#define INFSTR_KEY_RESOURCE_FILE        TEXT("ResourceFile")
#define INFSTR_KEY_MESSAGE_FILE         TEXT("MessageFile")
#define INFSTR_KEY_PARAMETER_FILE       TEXT("ParameterFile")

//
// Used in COM server section
//
#define INFSTR_KEY_COM_SERVER_TYPE            TEXT("ServerType")
#define INFSTR_KEY_COM_SERVER_BINARY          TEXT("ServerBinary")
#define INFSTR_KEY_COM_SERVER_BINARY_WOW64    TEXT("ServerBinaryWow64")
#define INFSTR_KEY_COM_SERVER_ADD_COM_CLASS   TEXT("AddComClass")

//
// Used in COM class section
//
#define INFSTR_KEY_COM_CLASS_THREADING_MODEL TEXT("ThreadingModel")
#define INFSTR_KEY_COM_CLASS_DESCRIPTION     TEXT("Description")

//
// Used in components section
//
#define INFSTR_KEY_COMPONENTIDS         TEXT("ComponentIds")

//
// Used for channel attributes in event channel section
//
#define INFSTR_KEY_CHANNEL_ACCESS       TEXT("Access")
#define INFSTR_KEY_CHANNEL_ISOLATION    TEXT("Isolation")
#define INFSTR_KEY_CHANNEL_ENABLED      TEXT("Enabled")
#define INFSTR_KEY_CHANNEL_VALUE        TEXT("Value")

//
// Used for logging attributes in event channel section
//
#define INFSTR_KEY_LOGGING_MAXSIZE      TEXT("LoggingMaxSize")
#define INFSTR_KEY_LOGGING_RETENTION    TEXT("LoggingRetention")
#define INFSTR_KEY_LOGGING_AUTOBACKUP   TEXT("LoggingAutoBackup")

//
// Used in event autologger section
//
#define INFSTR_KEY_START                TEXT("Start")
#define INFSTR_KEY_BUFFER_SIZE          TEXT("BufferSize")
#define INFSTR_KEY_CLOCK_TYPE           TEXT("ClockType")
#define INFSTR_KEY_DISABLE_REALTIME_PERSISTENCE      TEXT("DisableRealtimePersistence")
#define INFSTR_KEY_FILE_NAME            TEXT("FileName")
#define INFSTR_KEY_FILE_MAX             TEXT("FileMax")
#define INFSTR_KEY_FLUSH_TIMER          TEXT("FlushTimer")
#define INFSTR_KEY_LOG_FILE_MODE        TEXT("LogFileMode")
#define INFSTR_KEY_MAX_FILE_SIZE        TEXT("MaxFileSize")
#define INFSTR_KEY_MAXIMUM_BUFFERS      TEXT("MaximumBuffers")
#define INFSTR_KEY_MINIMUM_BUFFERS      TEXT("MinimumBuffers")

//
// Used in event autologger provider section
//
#define INFSTR_KEY_ENABLED              TEXT("Enabled")
#define INFSTR_KEY_ENABLE_FLAGS         TEXT("EnableFlags")
#define INFSTR_KEY_ENABLE_LEVEL         TEXT("EnableLevel")
#define INFSTR_KEY_ENABLE_PROPERTY      TEXT("EnableProperty")
#define INFSTR_KEY_MATCH_ANY_KEYWORD    TEXT("MatchAnyKeyword")
#define INFSTR_KEY_MATCH_ALL_KEYWORD    TEXT("MatchAllKeyword")

// The following are the characters to parse IORange and MemRange fields.
#define CH_SIZE_DELIM                   TEXT('@')
#define CH_MINMAX_SEP                   TEXT('-')
#define CH_ALIGNMASK_BEGIN              TEXT('%')
#define CH_TRAIL_BEGIN                  TEXT('(')
#define CH_TRAIL_SEP                    TEXT(':')
#define CH_TRAIL_END                    TEXT(')')


// The following is char to parse IRQ and DMA attr from the numbers!
#define CH_ATTR_DELIM                   TEXT(':')

// The following is for Windows 9x System Detection
#define INFSTR_SECT_DETMODULES          TEXT("Det.Modules")
#define INFSTR_SECT_DETCLASSINFO        TEXT("Det.ClassInfo")
#define INFSTR_SECT_MANUALDEV           TEXT("Det.ManualDev")
#define INFSTR_SECT_AVOIDCFGSYSDEV      TEXT("Det.AvoidCfgSysDev")
#define INFSTR_SECT_REGCFGSYSDEV        TEXT("Det.RegCfgSysDev")
#define INFSTR_SECT_DEVINFS             TEXT("Det.DevINFs")
#define INFSTR_SECT_AVOIDINIDEV         TEXT("Det.AvoidIniDev")
#define INFSTR_SECT_AVOIDENVDEV         TEXT("Det.AvoidEnvDev")
#define INFSTR_SECT_REGINIDEV           TEXT("Det.RegIniDev")
#define INFSTR_SECT_REGENVDEV           TEXT("Det.RegEnvDev")
#define INFSTR_SECT_HPOMNIBOOK          TEXT("Det.HPOmnibook")
#define INFSTR_SECT_FORCEHWVERIFY       TEXT("Det.ForceHWVerify")
#define INFSTR_SECT_DETOPTIONS          TEXT("Det.Options")
#define INFSTR_SECT_BADPNPBIOS          TEXT("BadPnpBios")
#define INFSTR_SECT_GOODACPIBIOS        TEXT("GoodACPIBios")
#define INFSTR_SECT_BADACPIBIOS         TEXT("BadACPIBios")
#define INFSTR_SECT_BADROUTINGTABLEBIOS TEXT("BadPCIIRQRoutingTableBios")
#define INFSTR_SECT_BADPMCALLBIOS       TEXT("BadProtectedModeCallBios")
#define INFSTR_SECT_BADRMCALLBIOS       TEXT("BadRealModeCallBios")
#define INFSTR_SECT_MACHINEIDBIOS       TEXT("MachineIDBios")
#define INFSTR_SECT_BADDISKBIOS         TEXT("BadDiskBios")
#define INFSTR_SECT_BADDSBIOS           TEXT("BadDSBios")
#define INFSTR_KEY_DETPARAMS            TEXT("Params")
#define INFSTR_KEY_SKIPLIST             TEXT("SkipList")
#define INFSTR_KEY_DETECTLIST           TEXT("DetectList")
#define INFSTR_KEY_EXCLUDERES           TEXT("ExcludeRes")

//Subkeys are used in the form x.<subkey>
#define INFSTR_SUBKEY_LOGCONFIG         TEXT("LogConfig")
#define INFSTR_SUBKEY_DET               TEXT("Det")
#define INFSTR_SUBKEY_FACTDEF           TEXT("FactDef")
#define INFSTR_SUBKEY_POSSIBLEDUPS      TEXT("PosDup")
#define INFSTR_SUBKEY_NORESOURCEDUPS    TEXT("NoResDup")
#define INFSTR_SUBKEY_HW                TEXT("Hw")
#define INFSTR_SUBKEY_CTL               TEXT("CTL")
#define INFSTR_SUBKEY_SERVICES          TEXT("Services")
#define INFSTR_SUBKEY_SOFTWARE          TEXT("Software")
#define INFSTR_SUBKEY_INTERFACES        TEXT("Interfaces")
#define INFSTR_SUBKEY_COINSTALLERS      TEXT("CoInstallers")
#define INFSTR_SUBKEY_LOGCONFIGOVERRIDE TEXT("LogConfigOverride")
#define INFSTR_SUBKEY_WMI               TEXT("WMI")
#define INFSTR_SUBKEY_EVENTS            TEXT("Events")
#define INFSTR_SUBKEY_COM               TEXT("COM")
#define INFSTR_SUBKEY_FILTERS           TEXT("Filters")
#define INFSTR_SUBKEY_COMPONENTS        TEXT("Components")

// Control Section
#define INFSTR_CONTROLFLAGS_SECTION         TEXT("ControlFlags")
#define INFSTR_KEY_COPYFILESONLY            TEXT("CopyFilesOnly")
#define INFSTR_KEY_EXCLUDEFROMSELECT        TEXT("ExcludeFromSelect")
#define INFSTR_KEY_ALWAYSEXCLUDEFROMSELECT  TEXT("AlwaysExcludeFromSelect")
#define INFSTR_KEY_INTERACTIVEINSTALL       TEXT("InteractiveInstall")
#define INFSTR_KEY_REQUESTADDITIONALSOFTWARE TEXT("RequestAdditionalSoftware")

// Platform-specific suffixes (e.g., "ExcludeFromSelect.NT")
#define INFSTR_PLATFORM_WIN             TEXT("Win")
#define INFSTR_PLATFORM_NT              TEXT("NT")
#define INFSTR_PLATFORM_NTX86           TEXT("NTx86")
#define INFSTR_PLATFORM_NTMIPS          TEXT("NTMIPS")
#define INFSTR_PLATFORM_NTALPHA         TEXT("NTAlpha")
#define INFSTR_PLATFORM_NTPPC           TEXT("NTPPC")
#define INFSTR_PLATFORM_NTIA64          TEXT("NTIA64")
#define INFSTR_PLATFORM_NTAXP64         TEXT("NTAXP64")
#define INFSTR_PLATFORM_NTAMD64         TEXT("NTAMD64")
#define INFSTR_PLATFORM_NTARM           TEXT("NTARM")
#define INFSTR_PLATFORM_NTARM64         TEXT("NTARM64")

// Fields that will by used to dereference strings.
// These are of the form x.<strkey> were strkey is limited to
// MAX_INFSTR_STRKEY_LEN characters
#define MAX_INFSTR_STRKEY_LEN           32
#define INFSTR_STRKEY_DRVDESC           TEXT("DriverDesc")
// DriverSelect
#define INFSTR_DRIVERSELECT_SECTION     TEXT("DriverSelect")
#define INFSTR_DRIVERSELECT_FUNCTIONS   TEXT("DriverSelectFunctions")

// Driver Version
#define INFSTR_DRIVERVERSION_SECTION    TEXT("DriverVer")

// Software Version
#define INFSTR_SOFTWAREVERSION_SECTION  TEXT("SoftwareVersion")

// The following is for PCMCIA.INF parsing
#define INFSTR_SECT_CFGSYS              TEXT("ConfigSysDrivers")
#define INFSTR_SECT_AUTOEXECBAT         TEXT("AutoexecBatDrivers")
#define INFSTR_SECT_SYSINI              TEXT("SystemIniDrivers")
#define INFSTR_SECT_SYSINIDRV           TEXT("SystemIniDriversLine")
#define INFSTR_SECT_WININIRUN           TEXT("WinIniRunLine")

//Keys in the config.sys device sections
#define INFSTR_KEY_PATH         TEXT("Path")
#define INFSTR_KEY_NAME         TEXT("Name")
#define INFSTR_KEY_IO           TEXT("IO")
#define INFSTR_KEY_MEM          TEXT("Mem")
#define INFSTR_KEY_IRQ          TEXT("IRQ")
#define INFSTR_KEY_DMA          TEXT("DMA")

//Fields of detection function registration
#define INFSTR_BUS_ISA          TEXT("BUS_ISA")
#define INFSTR_BUS_EISA         TEXT("BUS_EISA")
#define INFSTR_BUS_MCA          TEXT("BUS_MCA")
#define INFSTR_BUS_ALL          TEXT("BUS_ALL")
#define INFSTR_RISK_NONE        TEXT("RISK_NONE")
#define INFSTR_RISK_VERYLOW     TEXT("RISK_VERYLOW")
#define INFSTR_RISK_BIOSROMRD   TEXT("RISK_BIOSROMRD")
#define INFSTR_RISK_QUERYDRV    TEXT("RISK_QUERYDRV")
#define INFSTR_RISK_SWINT       TEXT("RISK_SWINT")
#define INFSTR_RISK_LOW         TEXT("RISK_LOW")
#define INFSTR_RISK_DELICATE    TEXT("RISK_DELICATE")
#define INFSTR_RISK_MEMRD       TEXT("RISK_MEMRD")
#define INFSTR_RISK_IORD        TEXT("RISK_IORD")
#define INFSTR_RISK_MEMWR       TEXT("RISK_MEMWR")
#define INFSTR_RISK_IOWR        TEXT("RISK_IOWR")
#define INFSTR_RISK_UNRELIABLE  TEXT("RISK_UNRELIABLE")
#define INFSTR_RISK_VERYHIGH    TEXT("RISK_VERYHIGH")
#define INFSTR_CLASS_SAFEEXCL   TEXT("SAFE_EXCL")

#define INFSTR_SECT_DISPLAY_CLEANUP    TEXT("DisplayCleanup")


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

#endif  //_INC_INFSTR
