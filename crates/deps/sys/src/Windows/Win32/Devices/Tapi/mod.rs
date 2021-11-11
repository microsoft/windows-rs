#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn GetTnefStreamCodepage();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_System_AddressBook`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
    pub fn OpenTnefStream();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_System_AddressBook`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_System_AddressBook", feature = "Win32_System_Com"))]
    pub fn OpenTnefStreamEx();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineAccept();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineAddProvider();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineAddProviderA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineAddProviderW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineAddToConference();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineAgentSpecific();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineAnswer();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineBlindTransfer();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineBlindTransferA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineBlindTransferW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineClose();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineCompleteCall();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineCompleteTransfer();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineConfigDialog();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineConfigDialogA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineConfigDialogEdit();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineConfigDialogEditA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineConfigDialogEditW();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineConfigDialogW();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineConfigProvider();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineCreateAgentA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineCreateAgentSessionA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineCreateAgentSessionW();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineCreateAgentW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineDeallocateCall();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineDevSpecific();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineDevSpecificFeature();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineDial();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineDialA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineDialW();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineDrop();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineForward();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineForwardA();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineForwardW();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGatherDigits();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGatherDigitsA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGatherDigitsW();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGenerateDigits();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGenerateDigitsA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGenerateDigitsW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGenerateTone();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAddressCaps();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAddressCapsA();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAddressCapsW();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetAddressID();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetAddressIDA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetAddressIDW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAddressStatus();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAddressStatusA();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAddressStatusW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAgentActivityListA();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAgentActivityListW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAgentCapsA();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAgentCapsW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAgentGroupListA();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAgentGroupListW();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn lineGetAgentInfo();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn lineGetAgentSessionInfo();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAgentSessionList();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAgentStatusA();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetAgentStatusW();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetAppPriority();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetAppPriorityA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetAppPriorityW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetCallInfo();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetCallInfoA();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetCallInfoW();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetCallStatus();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetConfRelatedCalls();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetCountry();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetCountryA();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetCountryW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetDevCaps();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetDevCapsA();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetDevCapsW();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetDevConfig();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetDevConfigA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetDevConfigW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetGroupListA();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetGroupListW();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetID();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetIDA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetIDW();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetIcon();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetIconA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineGetIconW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetLineDevStatus();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetLineDevStatusA();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetLineDevStatusW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetMessage();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetNewCalls();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetNumRings();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetProviderList();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetProviderListA();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetProviderListW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetProxyStatus();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetQueueInfo();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetQueueListA();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetQueueListW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetRequest();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetRequestA();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetRequestW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetStatusMessages();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetTranslateCaps();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetTranslateCapsA();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineGetTranslateCapsW();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineHandoff();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineHandoffA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineHandoffW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineHold();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineInitialize();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineInitializeExA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineInitializeExW();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineMakeCall();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineMakeCallA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineMakeCallW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineMonitorDigits();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineMonitorMedia();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineMonitorTones();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineNegotiateAPIVersion();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineNegotiateExtVersion();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineOpen();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineOpenA();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineOpenW();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn linePark();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineParkA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineParkW();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn linePickup();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn linePickupA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn linePickupW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn linePrepareAddToConference();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn linePrepareAddToConferenceA();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn linePrepareAddToConferenceW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineProxyMessage();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn lineProxyResponse();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineRedirect();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineRedirectA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineRedirectW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineRegisterRequestRecipient();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineReleaseUserUserInfo();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineRemoveFromConference();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineRemoveProvider();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSecureCall();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineSendUserUserInfo();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetAgentActivity();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetAgentGroup();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetAgentMeasurementPeriod();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetAgentSessionState();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetAgentState();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetAgentStateEx();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineSetAppPriority();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineSetAppPriorityA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineSetAppPriorityW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetAppSpecific();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetCallData();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetCallParams();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetCallPrivilege();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetCallQualityOfService();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetCallTreatment();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetCurrentLocation();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineSetDevConfig();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineSetDevConfigA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineSetDevConfigW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetLineDevStatus();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetMediaControl();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetMediaMode();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetNumRings();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetQueueMeasurementPeriod();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetStatusMessages();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetTerminal();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineSetTollList();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineSetTollListA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineSetTollListW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetupConference();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetupConferenceA();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetupConferenceW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetupTransfer();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetupTransferA();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSetupTransferW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineShutdown();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineSwapHold();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineTranslateAddress();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineTranslateAddressA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineTranslateAddressW();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineTranslateDialog();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineTranslateDialogA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineTranslateDialogW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineUncompleteCall();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn lineUnhold();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineUnpark();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineUnparkA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn lineUnparkW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneClose();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneConfigDialog();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneConfigDialogA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneConfigDialogW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneDevSpecific();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetButtonInfo();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetButtonInfoA();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetButtonInfoW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetData();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetDevCaps();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetDevCapsA();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetDevCapsW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetDisplay();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetGain();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetHookSwitch();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneGetID();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneGetIDA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneGetIDW();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneGetIcon();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneGetIconA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneGetIconW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetLamp();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetMessage();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetRing();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetStatus();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetStatusA();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetStatusMessages();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetStatusW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneGetVolume();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneInitialize();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneInitializeExA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneInitializeExW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneNegotiateAPIVersion();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneNegotiateExtVersion();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneOpen();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneSetButtonInfo();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneSetButtonInfoA();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneSetButtonInfoW();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneSetData();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn phoneSetDisplay();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneSetGain();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneSetHookSwitch();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneSetLamp();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneSetRing();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneSetStatusMessages();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneSetVolume();
    #[doc = "*Required features: `Win32_Devices_Tapi`*"]
    pub fn phoneShutdown();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn tapiGetLocationInfo();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn tapiGetLocationInfoA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn tapiGetLocationInfoW();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn tapiRequestDrop();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn tapiRequestMakeCall();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn tapiRequestMakeCallA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn tapiRequestMakeCallW();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn tapiRequestMediaCall();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn tapiRequestMediaCallA();
    #[doc = "*Required features: `Win32_Devices_Tapi`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn tapiRequestMediaCallW();
}
