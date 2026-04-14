//
//    Copyright (C) Microsoft.  All rights reserved.
//
#pragma once
#include <winapifamily.h>

#pragma region Desktop Family
#if WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP)


#if(_WIN32_WINNT >= 0x0600)
typedef enum _WSC_SECURITY_PROVIDER
{
    // Represents the aggregation of all firewalls for this computer.
    WSC_SECURITY_PROVIDER_FIREWALL =                   0x1,
    // Represents the Automatic updating settings for this computer.
    WSC_SECURITY_PROVIDER_AUTOUPDATE_SETTINGS  =       0x2,
    // Represents the aggregation of all antivirus products for this comptuer.
    WSC_SECURITY_PROVIDER_ANTIVIRUS =                  0x4,
    // Represents the aggregation of all antispyware products for this comptuer.
    WSC_SECURITY_PROVIDER_ANTISPYWARE =                0x8,
    // Represents the settings that restrict the access of web sites in each of the internet zones.
    WSC_SECURITY_PROVIDER_INTERNET_SETTINGS =          0x10,
    // Represents the User Account Control settings on this machine.
    WSC_SECURITY_PROVIDER_USER_ACCOUNT_CONTROL =       0x20,
    // Represents the running state of the Security Center service on this machine.
    WSC_SECURITY_PROVIDER_SERVICE =                    0x40,

    WSC_SECURITY_PROVIDER_NONE =                       0,

    // Aggregates all of the items that Security Center monitors.
    WSC_SECURITY_PROVIDER_ALL =                             WSC_SECURITY_PROVIDER_FIREWALL | 
                                                            WSC_SECURITY_PROVIDER_AUTOUPDATE_SETTINGS | 
                                                            WSC_SECURITY_PROVIDER_ANTIVIRUS | 
                                                            WSC_SECURITY_PROVIDER_ANTISPYWARE | 
                                                            WSC_SECURITY_PROVIDER_INTERNET_SETTINGS | 
                                                            WSC_SECURITY_PROVIDER_USER_ACCOUNT_CONTROL | 
                                                            WSC_SECURITY_PROVIDER_SERVICE
} WSC_SECURITY_PROVIDER, *PWSC_SECURITY_PROVIDER;

typedef enum _WSC_SECURITY_PROVIDER_HEALTH
{
    WSC_SECURITY_PROVIDER_HEALTH_GOOD, // Green pillar in English locales
    WSC_SECURITY_PROVIDER_HEALTH_NOTMONITORED, // Yellow pillar in English locales
    WSC_SECURITY_PROVIDER_HEALTH_POOR,  // Red pillar in English locales
    WSC_SECURITY_PROVIDER_HEALTH_SNOOZE, // Yellow pillar in English locales
} WSC_SECURITY_PROVIDER_HEALTH, *PWSC_SECURITY_PROVIDER_HEALTH;


// The WscRegisterForChanges function registers a callback function to be run when Windows Security Center (WSC) detects 
// a change that could affect the health of one of the security providers.
// 
// Parameters:
// Reserved [in]: Reserved. Must be NULL.
// phCallbackRegistration [out]: A pointer to a handle to the callback registration. When you are finished using 
//                               the callback function, unregister it by calling the WscUnRegisterChanges function.
// lpCallbackAddress [in]: A pointer to the application-defined function to be called when a change to the WSC 
//                         service occurs. This function is also called when the WSC service is started or stopped.
// pContext [in]: A pointer to a variable to be passed as the lpParameter parameter to the function pointed to by the 
//                lpCallbackAddress parameter.
//
// Return Values:
// Returns S_OK if the function succeeds, otherwise returns an error code.
//
// Remarks:
// When you want to cease receiving notification to your callback method, you can unregister it by calling the 
// WscUnRegisterChanges function.




STDAPI WscRegisterForChanges(LPVOID Reserved, 
                             PHANDLE phCallbackRegistration,
                             LPTHREAD_START_ROUTINE lpCallbackAddress, 
                             PVOID pContext);

// The WscUnRegisterChanges function cancels a callback registration that was made by a call to the WscRegisterForChanges 
// function.
// 
// Parameters:
// hRegistrationHandle [in]: The handle to the registration context returned as the phCallbackRegistration of the 
//                           WscRegisterForChanges function. 
//
// Return Values:
// Returns S_OK if the function succeeds, otherwise returns an error code.

STDAPI WscUnRegisterChanges(HANDLE hRegistrationHandle);

// The WscRegisterForUserNotifications function registers a callback function to be run when the Security Center service
// fires a WNF event.
// 
// Parameters:
//
// Return Values:
// Returns S_OK if the function succeeds, otherwise returns an error code.

STDAPI WscRegisterForUserNotifications();

// The WscGetSecurityProviderHealth function gets the aggregate health state of the security provider categories represented 
// by the specified WSC_SECURITY_PROVIDER enumeration values.
// 
// Parameters:
// Providers [in]: One or more of the values in the WSC_SECURITY_PROVIDER enumeration. To specify more than one value, 
//                 combine the individual values by performing a bitwise OR operation.
// pHealth [out]: A pointer to a variable that takes the value of one of the members of the WSC_SECURITY_PROVIDER_HEALTH 
//                enumeration. If more than one provider is specified in the Providers parameter, the value of this parameter 
//                is the health of the least healthy of the specified provider categories. 
//
// Return values:  
// Returns S_OK if the function succeeds, otherwise returns an error code. If the WSC service is not running, the return value 
// is always S_FALSE and the pHealth out parameter is always set to WSC_SECURITY_PROVIDER_HEALTH_POOR.
// 

STDAPI WscGetSecurityProviderHealth(DWORD Providers,
                                    PWSC_SECURITY_PROVIDER_HEALTH pHealth);

HRESULT wscShowAMSCN(_In_ DWORD InputFlags, _Out_opt_ PDWORD pdwResultFlags);

HRESULT wscLaunchAdminMakeDefaultUI(_In_z_ PCWSTR pwszProductName);

// The WscQueryAntiMalwareUri function attempts to call the Windows Store API to retrieve the uri for the antimalware page. 
// If the call succeeds, the uri for invoking the store is returned, and will be used to redirect the user to the Windows 
// store for purchases. If the uri cannot be retrieved, the handler functions in wscui.cpl will fall back to sending users to 
// the landing page. The uri can fail to be retrieved if the store is not available for the user's locale, or if there 
// are no antimalware products in the store for the user's locale.
//
STDAPI WscQueryAntiMalwareUri();

// The WscGetAntiMalwareUri function retrieves the Windows Store uri that was obtained by WscQueryAntiMalwareUri. If 
// the uri could not be retrieved, then null is returned.
//
// Parameters:
// ppszUri [out] The Windows Store uri string used to launch the antimalware page of the store.
//
STDAPI WscGetAntiMalwareUri(_Outptr_ LPWSTR *ppszUri);

#endif


#endif /* WINAPI_FAMILY_PARTITION(WINAPI_PARTITION_DESKTOP) */
#pragma endregion

