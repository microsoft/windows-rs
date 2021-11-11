#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAtomA();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddAtomW();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AddClipboardFormatListener();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeClipboardChain();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseClipboard();
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn CountClipboardFormats();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeAbandonTransaction();
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn DdeAccessData();
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn DdeAddData();
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn DdeClientTransaction();
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn DdeCmpStringHandles();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn DdeConnect();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn DdeConnectList();
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn DdeCreateDataHandle();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeCreateStringHandleA();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeCreateStringHandleW();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeDisconnect();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeDisconnectList();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeEnableCallback();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeFreeDataHandle();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeFreeStringHandle();
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn DdeGetData();
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn DdeGetLastError();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeImpersonateClient();
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn DdeInitializeA();
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn DdeInitializeW();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeKeepStringHandle();
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn DdeNameService();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdePostAdvise();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn DdeQueryConvInfo();
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn DdeQueryNextServer();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeQueryStringA();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeQueryStringW();
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn DdeReconnect();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn DdeSetQualityOfService();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeSetUserHandle();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeUnaccessData();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DdeUninitialize();
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn DeleteAtom();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EmptyClipboard();
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn EnumClipboardFormats();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindAtomA();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindAtomW();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FreeDDElParam();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAtomNameA();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAtomNameW();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClipboardData();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClipboardFormatNameA();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClipboardFormatNameW();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClipboardOwner();
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn GetClipboardSequenceNumber();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClipboardViewer();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetOpenClipboardWindow();
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn GetPriorityClipboardFormat();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUpdatedClipboardFormats();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalAddAtomA();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalAddAtomExA();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalAddAtomExW();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalAddAtomW();
    #[doc = "*Required features: `Win32_System_DataExchange`*"]
    pub fn GlobalDeleteAtom();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalFindAtomA();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalFindAtomW();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalGetAtomNameA();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GlobalGetAtomNameW();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImpersonateDdeClientWindow();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitAtomTable();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsClipboardFormatAvailable();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenClipboard();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PackDDElParam();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterClipboardFormatA();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterClipboardFormatW();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveClipboardFormatListener();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReuseDDElParam();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClipboardData();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClipboardViewer();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn SetWinMetaFileBits();
    #[doc = "*Required features: `Win32_System_DataExchange`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnpackDDElParam();
}
