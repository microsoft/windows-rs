#[cfg(feature = "Win32_System_Com")]
pub trait IAzApplicationImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetName();
    fn Description();
    fn SetDescription();
    fn ApplicationData();
    fn SetApplicationData();
    fn AuthzInterfaceClsid();
    fn SetAuthzInterfaceClsid();
    fn Version();
    fn SetVersion();
    fn GenerateAudits();
    fn SetGenerateAudits();
    fn ApplyStoreSacl();
    fn SetApplyStoreSacl();
    fn Writable();
    fn GetProperty();
    fn SetProperty();
    fn PolicyAdministrators();
    fn PolicyReaders();
    fn AddPolicyAdministrator();
    fn DeletePolicyAdministrator();
    fn AddPolicyReader();
    fn DeletePolicyReader();
    fn Scopes();
    fn OpenScope();
    fn CreateScope();
    fn DeleteScope();
    fn Operations();
    fn OpenOperation();
    fn CreateOperation();
    fn DeleteOperation();
    fn Tasks();
    fn OpenTask();
    fn CreateTask();
    fn DeleteTask();
    fn ApplicationGroups();
    fn OpenApplicationGroup();
    fn CreateApplicationGroup();
    fn DeleteApplicationGroup();
    fn Roles();
    fn OpenRole();
    fn CreateRole();
    fn DeleteRole();
    fn InitializeClientContextFromToken();
    fn AddPropertyItem();
    fn DeletePropertyItem();
    fn Submit();
    fn InitializeClientContextFromName();
    fn DelegatedPolicyUsers();
    fn AddDelegatedPolicyUser();
    fn DeleteDelegatedPolicyUser();
    fn InitializeClientContextFromStringSid();
    fn PolicyAdministratorsName();
    fn PolicyReadersName();
    fn AddPolicyAdministratorName();
    fn DeletePolicyAdministratorName();
    fn AddPolicyReaderName();
    fn DeletePolicyReaderName();
    fn DelegatedPolicyUsersName();
    fn AddDelegatedPolicyUserName();
    fn DeleteDelegatedPolicyUserName();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzApplication2Impl: Sized + IAzApplicationImpl + IDispatchImpl {
    fn InitializeClientContextFromToken2();
    fn InitializeClientContext2();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzApplication3Impl: Sized + IAzApplication2Impl + IAzApplicationImpl + IDispatchImpl {
    fn ScopeExists();
    fn OpenScope2();
    fn CreateScope2();
    fn DeleteScope2();
    fn RoleDefinitions();
    fn CreateRoleDefinition();
    fn OpenRoleDefinition();
    fn DeleteRoleDefinition();
    fn RoleAssignments();
    fn CreateRoleAssignment();
    fn OpenRoleAssignment();
    fn DeleteRoleAssignment();
    fn BizRulesEnabled();
    fn SetBizRulesEnabled();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzApplicationGroupImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetName();
    fn Type();
    fn SetType();
    fn LdapQuery();
    fn SetLdapQuery();
    fn AppMembers();
    fn AppNonMembers();
    fn Members();
    fn NonMembers();
    fn Description();
    fn SetDescription();
    fn AddAppMember();
    fn DeleteAppMember();
    fn AddAppNonMember();
    fn DeleteAppNonMember();
    fn AddMember();
    fn DeleteMember();
    fn AddNonMember();
    fn DeleteNonMember();
    fn Writable();
    fn GetProperty();
    fn SetProperty();
    fn AddPropertyItem();
    fn DeletePropertyItem();
    fn Submit();
    fn AddMemberName();
    fn DeleteMemberName();
    fn AddNonMemberName();
    fn DeleteNonMemberName();
    fn MembersName();
    fn NonMembersName();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzApplicationGroup2Impl: Sized + IAzApplicationGroupImpl + IDispatchImpl {
    fn BizRule();
    fn SetBizRule();
    fn BizRuleLanguage();
    fn SetBizRuleLanguage();
    fn BizRuleImportedPath();
    fn SetBizRuleImportedPath();
    fn RoleAssignments();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzApplicationGroupsImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzApplicationsImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzAuthorizationStoreImpl: Sized + IDispatchImpl {
    fn Description();
    fn SetDescription();
    fn ApplicationData();
    fn SetApplicationData();
    fn DomainTimeout();
    fn SetDomainTimeout();
    fn ScriptEngineTimeout();
    fn SetScriptEngineTimeout();
    fn MaxScriptEngines();
    fn SetMaxScriptEngines();
    fn GenerateAudits();
    fn SetGenerateAudits();
    fn Writable();
    fn GetProperty();
    fn SetProperty();
    fn AddPropertyItem();
    fn DeletePropertyItem();
    fn PolicyAdministrators();
    fn PolicyReaders();
    fn AddPolicyAdministrator();
    fn DeletePolicyAdministrator();
    fn AddPolicyReader();
    fn DeletePolicyReader();
    fn Initialize();
    fn UpdateCache();
    fn Delete();
    fn Applications();
    fn OpenApplication();
    fn CreateApplication();
    fn DeleteApplication();
    fn ApplicationGroups();
    fn CreateApplicationGroup();
    fn OpenApplicationGroup();
    fn DeleteApplicationGroup();
    fn Submit();
    fn DelegatedPolicyUsers();
    fn AddDelegatedPolicyUser();
    fn DeleteDelegatedPolicyUser();
    fn TargetMachine();
    fn ApplyStoreSacl();
    fn SetApplyStoreSacl();
    fn PolicyAdministratorsName();
    fn PolicyReadersName();
    fn AddPolicyAdministratorName();
    fn DeletePolicyAdministratorName();
    fn AddPolicyReaderName();
    fn DeletePolicyReaderName();
    fn DelegatedPolicyUsersName();
    fn AddDelegatedPolicyUserName();
    fn DeleteDelegatedPolicyUserName();
    fn CloseApplication();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzAuthorizationStore2Impl: Sized + IAzAuthorizationStoreImpl + IDispatchImpl {
    fn OpenApplication2();
    fn CreateApplication2();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzAuthorizationStore3Impl: Sized + IAzAuthorizationStore2Impl + IAzAuthorizationStoreImpl + IDispatchImpl {
    fn IsUpdateNeeded();
    fn BizruleGroupSupported();
    fn UpgradeStoresFunctionalLevel();
    fn IsFunctionalLevelUpgradeSupported();
    fn GetSchemaVersion();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzBizRuleContextImpl: Sized + IDispatchImpl {
    fn SetBusinessRuleResult();
    fn SetBusinessRuleString();
    fn BusinessRuleString();
    fn GetParameter();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzBizRuleInterfacesImpl: Sized + IDispatchImpl {
    fn AddInterface();
    fn AddInterfaces();
    fn GetInterfaceValue();
    fn Remove();
    fn RemoveAll();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzBizRuleParametersImpl: Sized + IDispatchImpl {
    fn AddParameter();
    fn AddParameters();
    fn GetParameterValue();
    fn Remove();
    fn RemoveAll();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzClientContextImpl: Sized + IDispatchImpl {
    fn AccessCheck();
    fn GetBusinessRuleString();
    fn UserDn();
    fn UserSamCompat();
    fn UserDisplay();
    fn UserGuid();
    fn UserCanonical();
    fn UserUpn();
    fn UserDnsSamCompat();
    fn GetProperty();
    fn GetRoles();
    fn RoleForAccessCheck();
    fn SetRoleForAccessCheck();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzClientContext2Impl: Sized + IAzClientContextImpl + IDispatchImpl {
    fn GetAssignedScopesPage();
    fn AddRoles();
    fn AddApplicationGroups();
    fn AddStringSids();
    fn SetLDAPQueryDN();
    fn LDAPQueryDN();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzClientContext3Impl: Sized + IAzClientContext2Impl + IAzClientContextImpl + IDispatchImpl {
    fn AccessCheck2();
    fn IsInRoleAssignment();
    fn GetOperations();
    fn GetTasks();
    fn BizRuleParameters();
    fn BizRuleInterfaces();
    fn GetGroups();
    fn Sids();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzNameResolverImpl: Sized + IDispatchImpl {
    fn NameFromSid();
    fn NamesFromSids();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzObjectPickerImpl: Sized + IDispatchImpl {
    fn GetPrincipals();
    fn Name();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzOperationImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetName();
    fn Description();
    fn SetDescription();
    fn ApplicationData();
    fn SetApplicationData();
    fn OperationID();
    fn SetOperationID();
    fn Writable();
    fn GetProperty();
    fn SetProperty();
    fn Submit();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzOperation2Impl: Sized + IAzOperationImpl + IDispatchImpl {
    fn RoleAssignments();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzOperationsImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzPrincipalLocatorImpl: Sized + IDispatchImpl {
    fn NameResolver();
    fn ObjectPicker();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzRoleImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetName();
    fn Description();
    fn SetDescription();
    fn ApplicationData();
    fn SetApplicationData();
    fn AddAppMember();
    fn DeleteAppMember();
    fn AddTask();
    fn DeleteTask();
    fn AddOperation();
    fn DeleteOperation();
    fn AddMember();
    fn DeleteMember();
    fn Writable();
    fn GetProperty();
    fn SetProperty();
    fn AppMembers();
    fn Members();
    fn Operations();
    fn Tasks();
    fn AddPropertyItem();
    fn DeletePropertyItem();
    fn Submit();
    fn AddMemberName();
    fn DeleteMemberName();
    fn MembersName();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzRoleAssignmentImpl: Sized + IAzRoleImpl + IDispatchImpl {
    fn AddRoleDefinition();
    fn DeleteRoleDefinition();
    fn RoleDefinitions();
    fn Scope();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzRoleAssignmentsImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzRoleDefinitionImpl: Sized + IAzTaskImpl + IDispatchImpl {
    fn RoleAssignments();
    fn AddRoleDefinition();
    fn DeleteRoleDefinition();
    fn RoleDefinitions();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzRoleDefinitionsImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzRolesImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzScopeImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetName();
    fn Description();
    fn SetDescription();
    fn ApplicationData();
    fn SetApplicationData();
    fn Writable();
    fn GetProperty();
    fn SetProperty();
    fn AddPropertyItem();
    fn DeletePropertyItem();
    fn PolicyAdministrators();
    fn PolicyReaders();
    fn AddPolicyAdministrator();
    fn DeletePolicyAdministrator();
    fn AddPolicyReader();
    fn DeletePolicyReader();
    fn ApplicationGroups();
    fn OpenApplicationGroup();
    fn CreateApplicationGroup();
    fn DeleteApplicationGroup();
    fn Roles();
    fn OpenRole();
    fn CreateRole();
    fn DeleteRole();
    fn Tasks();
    fn OpenTask();
    fn CreateTask();
    fn DeleteTask();
    fn Submit();
    fn CanBeDelegated();
    fn BizrulesWritable();
    fn PolicyAdministratorsName();
    fn PolicyReadersName();
    fn AddPolicyAdministratorName();
    fn DeletePolicyAdministratorName();
    fn AddPolicyReaderName();
    fn DeletePolicyReaderName();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzScope2Impl: Sized + IAzScopeImpl + IDispatchImpl {
    fn RoleDefinitions();
    fn CreateRoleDefinition();
    fn OpenRoleDefinition();
    fn DeleteRoleDefinition();
    fn RoleAssignments();
    fn CreateRoleAssignment();
    fn OpenRoleAssignment();
    fn DeleteRoleAssignment();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzScopesImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzTaskImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetName();
    fn Description();
    fn SetDescription();
    fn ApplicationData();
    fn SetApplicationData();
    fn BizRule();
    fn SetBizRule();
    fn BizRuleLanguage();
    fn SetBizRuleLanguage();
    fn BizRuleImportedPath();
    fn SetBizRuleImportedPath();
    fn IsRoleDefinition();
    fn SetIsRoleDefinition();
    fn Operations();
    fn Tasks();
    fn AddOperation();
    fn DeleteOperation();
    fn AddTask();
    fn DeleteTask();
    fn Writable();
    fn GetProperty();
    fn SetProperty();
    fn AddPropertyItem();
    fn DeletePropertyItem();
    fn Submit();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzTask2Impl: Sized + IAzTaskImpl + IDispatchImpl {
    fn RoleAssignments();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzTasksImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
