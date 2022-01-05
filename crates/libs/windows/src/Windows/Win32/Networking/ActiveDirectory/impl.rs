#[cfg(feature = "Win32_System_Com")]
pub trait IADsImpl: Sized + IDispatchImpl {
    fn Name();
    fn Class();
    fn GUID();
    fn ADsPath();
    fn Parent();
    fn Schema();
    fn GetInfo();
    fn SetInfo();
    fn Get();
    fn Put();
    fn GetEx();
    fn PutEx();
    fn GetInfoEx();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsADSystemInfoImpl: Sized + IDispatchImpl {
    fn UserName();
    fn ComputerName();
    fn SiteName();
    fn DomainShortName();
    fn DomainDNSName();
    fn ForestDNSName();
    fn PDCRoleOwner();
    fn SchemaRoleOwner();
    fn IsNativeMode();
    fn GetAnyDCName();
    fn GetDCSiteName();
    fn RefreshSchemaCache();
    fn GetTrees();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsAccessControlEntryImpl: Sized + IDispatchImpl {
    fn AccessMask();
    fn SetAccessMask();
    fn AceType();
    fn SetAceType();
    fn AceFlags();
    fn SetAceFlags();
    fn Flags();
    fn SetFlags();
    fn ObjectType();
    fn SetObjectType();
    fn InheritedObjectType();
    fn SetInheritedObjectType();
    fn Trustee();
    fn SetTrustee();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsAccessControlListImpl: Sized + IDispatchImpl {
    fn AclRevision();
    fn SetAclRevision();
    fn AceCount();
    fn SetAceCount();
    fn AddAce();
    fn RemoveAce();
    fn CopyAccessList();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsAclImpl: Sized + IDispatchImpl {
    fn ProtectedAttrName();
    fn SetProtectedAttrName();
    fn SubjectName();
    fn SetSubjectName();
    fn Privileges();
    fn SetPrivileges();
    fn CopyAcl();
}
pub trait IADsAggregateeImpl: Sized {
    fn ConnectAsAggregatee();
    fn DisconnectAsAggregatee();
    fn RelinquishInterface();
    fn RestoreInterface();
}
pub trait IADsAggregatorImpl: Sized {
    fn ConnectAsAggregator();
    fn DisconnectAsAggregator();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsBackLinkImpl: Sized + IDispatchImpl {
    fn RemoteID();
    fn SetRemoteID();
    fn ObjectName();
    fn SetObjectName();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsCaseIgnoreListImpl: Sized + IDispatchImpl {
    fn CaseIgnoreList();
    fn SetCaseIgnoreList();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsClassImpl: Sized + IADsImpl + IDispatchImpl {
    fn PrimaryInterface();
    fn CLSID();
    fn SetCLSID();
    fn OID();
    fn SetOID();
    fn Abstract();
    fn SetAbstract();
    fn Auxiliary();
    fn SetAuxiliary();
    fn MandatoryProperties();
    fn SetMandatoryProperties();
    fn OptionalProperties();
    fn SetOptionalProperties();
    fn NamingProperties();
    fn SetNamingProperties();
    fn DerivedFrom();
    fn SetDerivedFrom();
    fn AuxDerivedFrom();
    fn SetAuxDerivedFrom();
    fn PossibleSuperiors();
    fn SetPossibleSuperiors();
    fn Containment();
    fn SetContainment();
    fn Container();
    fn SetContainer();
    fn HelpFileName();
    fn SetHelpFileName();
    fn HelpFileContext();
    fn SetHelpFileContext();
    fn Qualifiers();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsCollectionImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Add();
    fn Remove();
    fn GetObject();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsComputerImpl: Sized + IADsImpl + IDispatchImpl {
    fn ComputerID();
    fn Site();
    fn Description();
    fn SetDescription();
    fn Location();
    fn SetLocation();
    fn PrimaryUser();
    fn SetPrimaryUser();
    fn Owner();
    fn SetOwner();
    fn Division();
    fn SetDivision();
    fn Department();
    fn SetDepartment();
    fn Role();
    fn SetRole();
    fn OperatingSystem();
    fn SetOperatingSystem();
    fn OperatingSystemVersion();
    fn SetOperatingSystemVersion();
    fn Model();
    fn SetModel();
    fn Processor();
    fn SetProcessor();
    fn ProcessorCount();
    fn SetProcessorCount();
    fn MemorySize();
    fn SetMemorySize();
    fn StorageCapacity();
    fn SetStorageCapacity();
    fn NetAddresses();
    fn SetNetAddresses();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsComputerOperationsImpl: Sized + IADsImpl + IDispatchImpl {
    fn Status();
    fn Shutdown();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsContainerImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Filter();
    fn SetFilter();
    fn Hints();
    fn SetHints();
    fn GetObject();
    fn Create();
    fn Delete();
    fn CopyHere();
    fn MoveHere();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsDNWithBinaryImpl: Sized + IDispatchImpl {
    fn BinaryValue();
    fn SetBinaryValue();
    fn DNString();
    fn SetDNString();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsDNWithStringImpl: Sized + IDispatchImpl {
    fn StringValue();
    fn SetStringValue();
    fn DNString();
    fn SetDNString();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsDeleteOpsImpl: Sized + IDispatchImpl {
    fn DeleteObject();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsDomainImpl: Sized + IADsImpl + IDispatchImpl {
    fn IsWorkgroup();
    fn MinPasswordLength();
    fn SetMinPasswordLength();
    fn MinPasswordAge();
    fn SetMinPasswordAge();
    fn MaxPasswordAge();
    fn SetMaxPasswordAge();
    fn MaxBadPasswordsAllowed();
    fn SetMaxBadPasswordsAllowed();
    fn PasswordHistoryLength();
    fn SetPasswordHistoryLength();
    fn PasswordAttributes();
    fn SetPasswordAttributes();
    fn AutoUnlockInterval();
    fn SetAutoUnlockInterval();
    fn LockoutObservationInterval();
    fn SetLockoutObservationInterval();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsEmailImpl: Sized + IDispatchImpl {
    fn Type();
    fn SetType();
    fn Address();
    fn SetAddress();
}
pub trait IADsExtensionImpl: Sized {
    fn Operate();
    fn PrivateGetIDsOfNames();
    fn PrivateInvoke();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsFaxNumberImpl: Sized + IDispatchImpl {
    fn TelephoneNumber();
    fn SetTelephoneNumber();
    fn Parameters();
    fn SetParameters();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsFileServiceImpl: Sized + IADsServiceImpl + IADsImpl + IDispatchImpl {
    fn Description();
    fn SetDescription();
    fn MaxUserCount();
    fn SetMaxUserCount();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsFileServiceOperationsImpl: Sized + IADsServiceOperationsImpl + IADsImpl + IDispatchImpl {
    fn Sessions();
    fn Resources();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsFileShareImpl: Sized + IADsImpl + IDispatchImpl {
    fn CurrentUserCount();
    fn Description();
    fn SetDescription();
    fn HostComputer();
    fn SetHostComputer();
    fn Path();
    fn SetPath();
    fn MaxUserCount();
    fn SetMaxUserCount();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsGroupImpl: Sized + IADsImpl + IDispatchImpl {
    fn Description();
    fn SetDescription();
    fn Members();
    fn IsMember();
    fn Add();
    fn Remove();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsHoldImpl: Sized + IDispatchImpl {
    fn ObjectName();
    fn SetObjectName();
    fn Amount();
    fn SetAmount();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsLargeIntegerImpl: Sized + IDispatchImpl {
    fn HighPart();
    fn SetHighPart();
    fn LowPart();
    fn SetLowPart();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsLocalityImpl: Sized + IADsImpl + IDispatchImpl {
    fn Description();
    fn SetDescription();
    fn LocalityName();
    fn SetLocalityName();
    fn PostalAddress();
    fn SetPostalAddress();
    fn SeeAlso();
    fn SetSeeAlso();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsMembersImpl: Sized + IDispatchImpl {
    fn Count();
    fn _NewEnum();
    fn Filter();
    fn SetFilter();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsNameTranslateImpl: Sized + IDispatchImpl {
    fn SetChaseReferral();
    fn Init();
    fn InitEx();
    fn Set();
    fn Get();
    fn SetEx();
    fn GetEx();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsNamespacesImpl: Sized + IADsImpl + IDispatchImpl {
    fn DefaultContainer();
    fn SetDefaultContainer();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsNetAddressImpl: Sized + IDispatchImpl {
    fn AddressType();
    fn SetAddressType();
    fn Address();
    fn SetAddress();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsOImpl: Sized + IADsImpl + IDispatchImpl {
    fn Description();
    fn SetDescription();
    fn LocalityName();
    fn SetLocalityName();
    fn PostalAddress();
    fn SetPostalAddress();
    fn TelephoneNumber();
    fn SetTelephoneNumber();
    fn FaxNumber();
    fn SetFaxNumber();
    fn SeeAlso();
    fn SetSeeAlso();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsOUImpl: Sized + IADsImpl + IDispatchImpl {
    fn Description();
    fn SetDescription();
    fn LocalityName();
    fn SetLocalityName();
    fn PostalAddress();
    fn SetPostalAddress();
    fn TelephoneNumber();
    fn SetTelephoneNumber();
    fn FaxNumber();
    fn SetFaxNumber();
    fn SeeAlso();
    fn SetSeeAlso();
    fn BusinessCategory();
    fn SetBusinessCategory();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsObjectOptionsImpl: Sized + IDispatchImpl {
    fn GetOption();
    fn SetOption();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsOctetListImpl: Sized + IDispatchImpl {
    fn OctetList();
    fn SetOctetList();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsOpenDSObjectImpl: Sized + IDispatchImpl {
    fn OpenDSObject();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsPathImpl: Sized + IDispatchImpl {
    fn Type();
    fn SetType();
    fn VolumeName();
    fn SetVolumeName();
    fn Path();
    fn SetPath();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsPathnameImpl: Sized + IDispatchImpl {
    fn Set();
    fn SetDisplayType();
    fn Retrieve();
    fn GetNumElements();
    fn GetElement();
    fn AddLeafElement();
    fn RemoveLeafElement();
    fn CopyPath();
    fn GetEscapedElement();
    fn EscapedMode();
    fn SetEscapedMode();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsPostalAddressImpl: Sized + IDispatchImpl {
    fn PostalAddress();
    fn SetPostalAddress();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsPrintJobImpl: Sized + IADsImpl + IDispatchImpl {
    fn HostPrintQueue();
    fn User();
    fn UserPath();
    fn TimeSubmitted();
    fn TotalPages();
    fn Size();
    fn Description();
    fn SetDescription();
    fn Priority();
    fn SetPriority();
    fn StartTime();
    fn SetStartTime();
    fn UntilTime();
    fn SetUntilTime();
    fn Notify();
    fn SetNotify();
    fn NotifyPath();
    fn SetNotifyPath();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsPrintJobOperationsImpl: Sized + IADsImpl + IDispatchImpl {
    fn Status();
    fn TimeElapsed();
    fn PagesPrinted();
    fn Position();
    fn SetPosition();
    fn Pause();
    fn Resume();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsPrintQueueImpl: Sized + IADsImpl + IDispatchImpl {
    fn PrinterPath();
    fn SetPrinterPath();
    fn Model();
    fn SetModel();
    fn Datatype();
    fn SetDatatype();
    fn PrintProcessor();
    fn SetPrintProcessor();
    fn Description();
    fn SetDescription();
    fn Location();
    fn SetLocation();
    fn StartTime();
    fn SetStartTime();
    fn UntilTime();
    fn SetUntilTime();
    fn DefaultJobPriority();
    fn SetDefaultJobPriority();
    fn Priority();
    fn SetPriority();
    fn BannerPage();
    fn SetBannerPage();
    fn PrintDevices();
    fn SetPrintDevices();
    fn NetAddresses();
    fn SetNetAddresses();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsPrintQueueOperationsImpl: Sized + IADsImpl + IDispatchImpl {
    fn Status();
    fn PrintJobs();
    fn Pause();
    fn Resume();
    fn Purge();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsPropertyImpl: Sized + IADsImpl + IDispatchImpl {
    fn OID();
    fn SetOID();
    fn Syntax();
    fn SetSyntax();
    fn MaxRange();
    fn SetMaxRange();
    fn MinRange();
    fn SetMinRange();
    fn MultiValued();
    fn SetMultiValued();
    fn Qualifiers();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsPropertyEntryImpl: Sized + IDispatchImpl {
    fn Clear();
    fn Name();
    fn SetName();
    fn ADsType();
    fn SetADsType();
    fn ControlCode();
    fn SetControlCode();
    fn Values();
    fn SetValues();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsPropertyListImpl: Sized + IDispatchImpl {
    fn PropertyCount();
    fn Next();
    fn Skip();
    fn Reset();
    fn Item();
    fn GetPropertyItem();
    fn PutPropertyItem();
    fn ResetPropertyItem();
    fn PurgePropertyList();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsPropertyValueImpl: Sized + IDispatchImpl {
    fn Clear();
    fn ADsType();
    fn SetADsType();
    fn DNString();
    fn SetDNString();
    fn CaseExactString();
    fn SetCaseExactString();
    fn CaseIgnoreString();
    fn SetCaseIgnoreString();
    fn PrintableString();
    fn SetPrintableString();
    fn NumericString();
    fn SetNumericString();
    fn Boolean();
    fn SetBoolean();
    fn Integer();
    fn SetInteger();
    fn OctetString();
    fn SetOctetString();
    fn SecurityDescriptor();
    fn SetSecurityDescriptor();
    fn LargeInteger();
    fn SetLargeInteger();
    fn UTCTime();
    fn SetUTCTime();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsPropertyValue2Impl: Sized + IDispatchImpl {
    fn GetObjectProperty();
    fn PutObjectProperty();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsReplicaPointerImpl: Sized + IDispatchImpl {
    fn ServerName();
    fn SetServerName();
    fn ReplicaType();
    fn SetReplicaType();
    fn ReplicaNumber();
    fn SetReplicaNumber();
    fn Count();
    fn SetCount();
    fn ReplicaAddressHints();
    fn SetReplicaAddressHints();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsResourceImpl: Sized + IADsImpl + IDispatchImpl {
    fn User();
    fn UserPath();
    fn Path();
    fn LockCount();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsSecurityDescriptorImpl: Sized + IDispatchImpl {
    fn Revision();
    fn SetRevision();
    fn Control();
    fn SetControl();
    fn Owner();
    fn SetOwner();
    fn OwnerDefaulted();
    fn SetOwnerDefaulted();
    fn Group();
    fn SetGroup();
    fn GroupDefaulted();
    fn SetGroupDefaulted();
    fn DiscretionaryAcl();
    fn SetDiscretionaryAcl();
    fn DaclDefaulted();
    fn SetDaclDefaulted();
    fn SystemAcl();
    fn SetSystemAcl();
    fn SaclDefaulted();
    fn SetSaclDefaulted();
    fn CopySecurityDescriptor();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsSecurityUtilityImpl: Sized + IDispatchImpl {
    fn GetSecurityDescriptor();
    fn SetSecurityDescriptor();
    fn ConvertSecurityDescriptor();
    fn SecurityMask();
    fn SetSecurityMask();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsServiceImpl: Sized + IADsImpl + IDispatchImpl {
    fn HostComputer();
    fn SetHostComputer();
    fn DisplayName();
    fn SetDisplayName();
    fn Version();
    fn SetVersion();
    fn ServiceType();
    fn SetServiceType();
    fn StartType();
    fn SetStartType();
    fn Path();
    fn SetPath();
    fn StartupParameters();
    fn SetStartupParameters();
    fn ErrorControl();
    fn SetErrorControl();
    fn LoadOrderGroup();
    fn SetLoadOrderGroup();
    fn ServiceAccountName();
    fn SetServiceAccountName();
    fn ServiceAccountPath();
    fn SetServiceAccountPath();
    fn Dependencies();
    fn SetDependencies();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsServiceOperationsImpl: Sized + IADsImpl + IDispatchImpl {
    fn Status();
    fn Start();
    fn Stop();
    fn Pause();
    fn Continue();
    fn SetPassword();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsSessionImpl: Sized + IADsImpl + IDispatchImpl {
    fn User();
    fn UserPath();
    fn Computer();
    fn ComputerPath();
    fn ConnectTime();
    fn IdleTime();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsSyntaxImpl: Sized + IADsImpl + IDispatchImpl {
    fn OleAutoDataType();
    fn SetOleAutoDataType();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsTimestampImpl: Sized + IDispatchImpl {
    fn WholeSeconds();
    fn SetWholeSeconds();
    fn EventID();
    fn SetEventID();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsTypedNameImpl: Sized + IDispatchImpl {
    fn ObjectName();
    fn SetObjectName();
    fn Level();
    fn SetLevel();
    fn Interval();
    fn SetInterval();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsUserImpl: Sized + IADsImpl + IDispatchImpl {
    fn BadLoginAddress();
    fn BadLoginCount();
    fn LastLogin();
    fn LastLogoff();
    fn LastFailedLogin();
    fn PasswordLastChanged();
    fn Description();
    fn SetDescription();
    fn Division();
    fn SetDivision();
    fn Department();
    fn SetDepartment();
    fn EmployeeID();
    fn SetEmployeeID();
    fn FullName();
    fn SetFullName();
    fn FirstName();
    fn SetFirstName();
    fn LastName();
    fn SetLastName();
    fn OtherName();
    fn SetOtherName();
    fn NamePrefix();
    fn SetNamePrefix();
    fn NameSuffix();
    fn SetNameSuffix();
    fn Title();
    fn SetTitle();
    fn Manager();
    fn SetManager();
    fn TelephoneHome();
    fn SetTelephoneHome();
    fn TelephoneMobile();
    fn SetTelephoneMobile();
    fn TelephoneNumber();
    fn SetTelephoneNumber();
    fn TelephonePager();
    fn SetTelephonePager();
    fn FaxNumber();
    fn SetFaxNumber();
    fn OfficeLocations();
    fn SetOfficeLocations();
    fn PostalAddresses();
    fn SetPostalAddresses();
    fn PostalCodes();
    fn SetPostalCodes();
    fn SeeAlso();
    fn SetSeeAlso();
    fn AccountDisabled();
    fn SetAccountDisabled();
    fn AccountExpirationDate();
    fn SetAccountExpirationDate();
    fn GraceLoginsAllowed();
    fn SetGraceLoginsAllowed();
    fn GraceLoginsRemaining();
    fn SetGraceLoginsRemaining();
    fn IsAccountLocked();
    fn SetIsAccountLocked();
    fn LoginHours();
    fn SetLoginHours();
    fn LoginWorkstations();
    fn SetLoginWorkstations();
    fn MaxLogins();
    fn SetMaxLogins();
    fn MaxStorage();
    fn SetMaxStorage();
    fn PasswordExpirationDate();
    fn SetPasswordExpirationDate();
    fn PasswordMinimumLength();
    fn SetPasswordMinimumLength();
    fn PasswordRequired();
    fn SetPasswordRequired();
    fn RequireUniquePassword();
    fn SetRequireUniquePassword();
    fn EmailAddress();
    fn SetEmailAddress();
    fn HomeDirectory();
    fn SetHomeDirectory();
    fn Languages();
    fn SetLanguages();
    fn Profile();
    fn SetProfile();
    fn LoginScript();
    fn SetLoginScript();
    fn Picture();
    fn SetPicture();
    fn HomePage();
    fn SetHomePage();
    fn Groups();
    fn SetPassword();
    fn ChangePassword();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IADsWinNTSystemInfoImpl: Sized + IDispatchImpl {
    fn UserName();
    fn ComputerName();
    fn DomainName();
    fn PDC();
}
pub trait ICommonQueryImpl: Sized {
    fn OpenQueryWindow();
}
pub trait IDirectoryObjectImpl: Sized {
    fn GetObjectInformation();
    fn GetObjectAttributes();
    fn SetObjectAttributes();
    fn CreateDSObject();
    fn DeleteDSObject();
}
pub trait IDirectorySchemaMgmtImpl: Sized {
    fn EnumAttributes();
    fn CreateAttributeDefinition();
    fn WriteAttributeDefinition();
    fn DeleteAttributeDefinition();
    fn EnumClasses();
    fn WriteClassDefinition();
    fn CreateClassDefinition();
    fn DeleteClassDefinition();
}
pub trait IDirectorySearchImpl: Sized {
    fn SetSearchPreference();
    fn ExecuteSearch();
    fn AbandonSearch();
    fn GetFirstRow();
    fn GetNextRow();
    fn GetPreviousRow();
    fn GetNextColumnName();
    fn GetColumn();
    fn FreeColumn();
    fn CloseSearchHandle();
}
pub trait IDsAdminCreateObjImpl: Sized {
    fn Initialize();
    fn CreateModal();
}
pub trait IDsAdminNewObjImpl: Sized {
    fn SetButtons();
    fn GetPageCounts();
}
pub trait IDsAdminNewObjExtImpl: Sized {
    fn Initialize();
    fn AddPages();
    fn SetObject();
    fn WriteData();
    fn OnError();
    fn GetSummaryInfo();
}
pub trait IDsAdminNewObjPrimarySiteImpl: Sized {
    fn CreateNew();
    fn Commit();
}
pub trait IDsAdminNotifyHandlerImpl: Sized {
    fn Initialize();
    fn Begin();
    fn Notify();
    fn End();
}
pub trait IDsBrowseDomainTreeImpl: Sized {
    fn BrowseTo();
    fn GetDomains();
    fn FreeDomains();
    fn FlushCachedDomains();
    fn SetComputer();
}
pub trait IDsDisplaySpecifierImpl: Sized {
    fn SetServer();
    fn SetLanguageID();
    fn GetDisplaySpecifier();
    fn GetIconLocation();
    fn GetIcon();
    fn GetFriendlyClassName();
    fn GetFriendlyAttributeName();
    fn IsClassContainer();
    fn GetClassCreationInfo();
    fn EnumClassAttributes();
    fn GetAttributeADsType();
}
pub trait IDsObjectPickerImpl: Sized {
    fn Initialize();
    fn InvokeDialog();
}
pub trait IDsObjectPickerCredentialsImpl: Sized + IDsObjectPickerImpl {
    fn SetCredentials();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IPersistQueryImpl: Sized + IPersistImpl {
    fn WriteString();
    fn ReadString();
    fn WriteInt();
    fn ReadInt();
    fn WriteStruct();
    fn ReadStruct();
    fn Clear();
}
pub trait IPrivateDispatchImpl: Sized {
    fn ADSIInitializeDispatchManager();
    fn ADSIGetTypeInfoCount();
    fn ADSIGetTypeInfo();
    fn ADSIGetIDsOfNames();
    fn ADSIInvoke();
}
pub trait IPrivateUnknownImpl: Sized {
    fn ADSIInitializeObject();
    fn ADSIReleaseObject();
}
pub trait IQueryFormImpl: Sized {
    fn Initialize();
    fn AddForms();
    fn AddPages();
}
