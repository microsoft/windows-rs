pub trait IActiveIMEImpl: Sized {
    fn Inquire();
    fn ConversionList();
    fn Configure();
    fn Destroy();
    fn Escape();
    fn SetActiveContext();
    fn ProcessKey();
    fn Notify();
    fn Select();
    fn SetCompositionString();
    fn ToAsciiEx();
    fn RegisterWord();
    fn UnregisterWord();
    fn GetRegisterWordStyle();
    fn EnumRegisterWord();
    fn GetCodePageA();
    fn GetLangId();
}
pub trait IActiveIME2Impl: Sized + IActiveIMEImpl {
    fn Sleep();
    fn Unsleep();
}
pub trait IActiveIMMAppImpl: Sized {
    fn AssociateContext();
    fn ConfigureIMEA();
    fn ConfigureIMEW();
    fn CreateContext();
    fn DestroyContext();
    fn EnumRegisterWordA();
    fn EnumRegisterWordW();
    fn EscapeA();
    fn EscapeW();
    fn GetCandidateListA();
    fn GetCandidateListW();
    fn GetCandidateListCountA();
    fn GetCandidateListCountW();
    fn GetCandidateWindow();
    fn GetCompositionFontA();
    fn GetCompositionFontW();
    fn GetCompositionStringA();
    fn GetCompositionStringW();
    fn GetCompositionWindow();
    fn GetContext();
    fn GetConversionListA();
    fn GetConversionListW();
    fn GetConversionStatus();
    fn GetDefaultIMEWnd();
    fn GetDescriptionA();
    fn GetDescriptionW();
    fn GetGuideLineA();
    fn GetGuideLineW();
    fn GetIMEFileNameA();
    fn GetIMEFileNameW();
    fn GetOpenStatus();
    fn GetProperty();
    fn GetRegisterWordStyleA();
    fn GetRegisterWordStyleW();
    fn GetStatusWindowPos();
    fn GetVirtualKey();
    fn InstallIMEA();
    fn InstallIMEW();
    fn IsIME();
    fn IsUIMessageA();
    fn IsUIMessageW();
    fn NotifyIME();
    fn RegisterWordA();
    fn RegisterWordW();
    fn ReleaseContext();
    fn SetCandidateWindow();
    fn SetCompositionFontA();
    fn SetCompositionFontW();
    fn SetCompositionStringA();
    fn SetCompositionStringW();
    fn SetCompositionWindow();
    fn SetConversionStatus();
    fn SetOpenStatus();
    fn SetStatusWindowPos();
    fn SimulateHotKey();
    fn UnregisterWordA();
    fn UnregisterWordW();
    fn Activate();
    fn Deactivate();
    fn OnDefWindowProc();
    fn FilterClientWindows();
    fn GetCodePageA();
    fn GetLangId();
    fn AssociateContextEx();
    fn DisableIME();
    fn GetImeMenuItemsA();
    fn GetImeMenuItemsW();
    fn EnumInputContext();
}
pub trait IActiveIMMIMEImpl: Sized {
    fn AssociateContext();
    fn ConfigureIMEA();
    fn ConfigureIMEW();
    fn CreateContext();
    fn DestroyContext();
    fn EnumRegisterWordA();
    fn EnumRegisterWordW();
    fn EscapeA();
    fn EscapeW();
    fn GetCandidateListA();
    fn GetCandidateListW();
    fn GetCandidateListCountA();
    fn GetCandidateListCountW();
    fn GetCandidateWindow();
    fn GetCompositionFontA();
    fn GetCompositionFontW();
    fn GetCompositionStringA();
    fn GetCompositionStringW();
    fn GetCompositionWindow();
    fn GetContext();
    fn GetConversionListA();
    fn GetConversionListW();
    fn GetConversionStatus();
    fn GetDefaultIMEWnd();
    fn GetDescriptionA();
    fn GetDescriptionW();
    fn GetGuideLineA();
    fn GetGuideLineW();
    fn GetIMEFileNameA();
    fn GetIMEFileNameW();
    fn GetOpenStatus();
    fn GetProperty();
    fn GetRegisterWordStyleA();
    fn GetRegisterWordStyleW();
    fn GetStatusWindowPos();
    fn GetVirtualKey();
    fn InstallIMEA();
    fn InstallIMEW();
    fn IsIME();
    fn IsUIMessageA();
    fn IsUIMessageW();
    fn NotifyIME();
    fn RegisterWordA();
    fn RegisterWordW();
    fn ReleaseContext();
    fn SetCandidateWindow();
    fn SetCompositionFontA();
    fn SetCompositionFontW();
    fn SetCompositionStringA();
    fn SetCompositionStringW();
    fn SetCompositionWindow();
    fn SetConversionStatus();
    fn SetOpenStatus();
    fn SetStatusWindowPos();
    fn SimulateHotKey();
    fn UnregisterWordA();
    fn UnregisterWordW();
    fn GenerateMessage();
    fn LockIMC();
    fn UnlockIMC();
    fn GetIMCLockCount();
    fn CreateIMCC();
    fn DestroyIMCC();
    fn LockIMCC();
    fn UnlockIMCC();
    fn ReSizeIMCC();
    fn GetIMCCSize();
    fn GetIMCCLockCount();
    fn GetHotKey();
    fn SetHotKey();
    fn CreateSoftKeyboard();
    fn DestroySoftKeyboard();
    fn ShowSoftKeyboard();
    fn GetCodePageA();
    fn GetLangId();
    fn KeybdEvent();
    fn LockModal();
    fn UnlockModal();
    fn AssociateContextEx();
    fn DisableIME();
    fn GetImeMenuItemsA();
    fn GetImeMenuItemsW();
    fn EnumInputContext();
    fn RequestMessageA();
    fn RequestMessageW();
    fn SendIMCA();
    fn SendIMCW();
    fn IsSleeping();
}
pub trait IActiveIMMMessagePumpOwnerImpl: Sized {
    fn Start();
    fn End();
    fn OnTranslateMessage();
    fn Pause();
    fn Resume();
}
pub trait IActiveIMMRegistrarImpl: Sized {
    fn RegisterIME();
    fn UnregisterIME();
}
pub trait IEnumInputContextImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
pub trait IEnumRegisterWordAImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
pub trait IEnumRegisterWordWImpl: Sized {
    fn Clone();
    fn Next();
    fn Reset();
    fn Skip();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IFEClassFactoryImpl: Sized + IClassFactoryImpl {}
pub trait IFECommonImpl: Sized {
    fn IsDefaultIME();
    fn SetDefaultIME();
    fn InvokeWordRegDialog();
    fn InvokeDictToolDialog();
}
pub trait IFEDictionaryImpl: Sized {
    fn Open();
    fn Close();
    fn GetHeader();
    fn DisplayProperty();
    fn GetPosTable();
    fn GetWords();
    fn NextWords();
    fn Create();
    fn SetHeader();
    fn ExistWord();
    fn ExistDependency();
    fn RegisterWord();
    fn RegisterDependency();
    fn GetDependencies();
    fn NextDependencies();
    fn ConvertFromOldMSIME();
    fn ConvertFromUserToSys();
}
pub trait IFELanguageImpl: Sized {
    fn Open();
    fn Close();
    fn GetJMorphResult();
    fn GetConversionModeCaps();
    fn GetPhonetic();
    fn GetConversion();
}
pub trait IImePadImpl: Sized {
    fn Request();
}
pub trait IImePadAppletImpl: Sized {
    fn Initialize();
    fn Terminate();
    fn GetAppletConfig();
    fn CreateUI();
    fn Notify();
}
pub trait IImePlugInDictDictionaryListImpl: Sized {
    fn GetDictionariesInUse();
    fn DeleteDictionary();
}
pub trait IImeSpecifyAppletsImpl: Sized {
    fn GetAppletIIDList();
}
