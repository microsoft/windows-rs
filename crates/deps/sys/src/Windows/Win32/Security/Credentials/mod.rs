#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredDeleteA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredDeleteW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredEnumerateA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredEnumerateW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredFindBestCredentialA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredFindBestCredentialW();
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn CredFree();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredGetSessionTypes();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredGetTargetInfoA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredGetTargetInfoW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredIsMarshaledCredentialA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredIsMarshaledCredentialW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredIsProtectedA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredIsProtectedW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredMarshalCredentialA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredMarshalCredentialW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredPackAuthenticationBufferA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredPackAuthenticationBufferW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredProtectA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredProtectW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredReadA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredReadDomainCredentialsA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredReadDomainCredentialsW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredReadW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredRenameA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredRenameW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUICmdLinePromptForCredentialsA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUICmdLinePromptForCredentialsW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUIConfirmCredentialsA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUIConfirmCredentialsW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUIParseUserNameA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUIParseUserNameW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CredUIPromptForCredentialsA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CredUIPromptForCredentialsW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CredUIPromptForWindowsCredentialsA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CredUIPromptForWindowsCredentialsW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUIReadSSOCredW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUIStoreSSOCredW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUnPackAuthenticationBufferA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUnPackAuthenticationBufferW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUnmarshalCredentialA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUnmarshalCredentialW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUnprotectA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredUnprotectW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredWriteA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredWriteDomainCredentialsA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredWriteDomainCredentialsW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CredWriteW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOpenCardNameA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOpenCardNameW();
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn KeyCredentialManagerFreeInformation();
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn KeyCredentialManagerGetInformation();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KeyCredentialManagerGetOperationErrorStates();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KeyCredentialManagerShowUIOperation();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardAccessStartedEvent();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardAddReaderToGroupA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardAddReaderToGroupW();
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardAudit();
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardBeginTransaction();
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardCancel();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardConnectA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardConnectW();
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardControl();
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardDisconnect();
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardDlgExtendedError();
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardEndTransaction();
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardEstablishContext();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardForgetCardTypeA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardForgetCardTypeW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardForgetReaderA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardForgetReaderGroupA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardForgetReaderGroupW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardForgetReaderW();
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardFreeMemory();
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardGetAttrib();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetCardTypeProviderNameA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetCardTypeProviderNameW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetDeviceTypeIdA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetDeviceTypeIdW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetProviderIdA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetProviderIdW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetReaderDeviceInstanceIdA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetReaderDeviceInstanceIdW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetReaderIconA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetReaderIconW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetStatusChangeA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardGetStatusChangeW();
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardGetTransmitCount();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardIntroduceCardTypeA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardIntroduceCardTypeW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardIntroduceReaderA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardIntroduceReaderGroupA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardIntroduceReaderGroupW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardIntroduceReaderW();
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardIsValidContext();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListCardsA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListCardsW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListInterfacesA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListInterfacesW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListReaderGroupsA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListReaderGroupsW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListReadersA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListReadersW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListReadersWithDeviceInstanceIdA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardListReadersWithDeviceInstanceIdW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardLocateCardsA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardLocateCardsByATRA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardLocateCardsByATRW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardLocateCardsW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardReadCacheA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardReadCacheW();
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardReconnect();
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardReleaseContext();
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardReleaseStartedEvent();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardRemoveReaderFromGroupA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardRemoveReaderFromGroupW();
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardSetAttrib();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardSetCardTypeProviderNameA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardSetCardTypeProviderNameW();
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardState();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardStatusA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardStatusW();
    #[doc = "*Required features: `Win32_Security_Credentials`*"]
    pub fn SCardTransmit();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SCardUIDlgSelectCardA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SCardUIDlgSelectCardW();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardWriteCacheA();
    #[doc = "*Required features: `Win32_Security_Credentials`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SCardWriteCacheW();
}
