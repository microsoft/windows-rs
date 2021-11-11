#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn BstrFromVector();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ClearCustData();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn CreateDispTypeInfo();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn CreateErrorInfo();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn CreateOleAdviseHolder();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn CreateStdDispatch();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn CreateTypeLib();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn CreateTypeLib2();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn DispCallFunc();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn DispGetIDsOfNames();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn DispGetParam();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn DispInvoke();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn DoDragDrop();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn DosDateTimeToVariantTime();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn GetActiveObject();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAltMonthNames();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn GetRecordInfoFromGuids();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn GetRecordInfoFromTypeInfo();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserFree();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserFree64();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserMarshal();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserMarshal64();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserSize();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserSize64();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserUnmarshal();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HRGN_UserUnmarshal64();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn IsAccelerator();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn LHashValOfNameSys();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn LHashValOfNameSysA();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn LoadRegTypeLib();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn LoadTypeLib();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn LoadTypeLibEx();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn OaBuildVersion();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn OaEnablePerUserTLibRegistration();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn OleBuildVersion();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreate();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn OleCreateDefaultHandler();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleCreateEmbeddingHelper();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateEx();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleCreateFontIndirect();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateFromData();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateFromDataEx();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateFromFile();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateFromFileEx();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateLink();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateLinkEx();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateLinkFromData();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateLinkFromDataEx();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateLinkToFile();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateLinkToFileEx();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn OleCreateMenuDescriptor();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleCreatePictureIndirect();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleCreatePropertyFrame();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleCreatePropertyFrameIndirect();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleCreateStaticFromData();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn OleDestroyMenuDescriptor();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleDoAutoConvert();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn OleDraw();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleDuplicateData();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn OleFlushClipboard();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn OleGetAutoConvert();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleGetClipboard();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleGetClipboardWithEnterpriseInfo();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleGetIconOfClass();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleGetIconOfFile();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleIconToCursor();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn OleInitialize();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleIsCurrentClipboard();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleIsRunning();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(feature = "Win32_System_Com_StructuredStorage")]
    pub fn OleLoad();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleLoadFromStream();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleLoadPicture();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleLoadPictureEx();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleLoadPictureFile();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleLoadPictureFileEx();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleLoadPicturePath();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleLockRunning();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleMetafilePictFromIconAndLabel();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleNoteObjectVisible();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleQueryCreateFromData();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleQueryLinkFromData();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleRegEnumFormatEtc();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn OleRegEnumVerbs();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn OleRegGetMiscStatus();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleRegGetUserType();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn OleRun();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleSave();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleSavePictureFile();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleSaveToStream();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn OleSetAutoConvert();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn OleSetClipboard();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleSetContainedObject();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleSetMenuDescriptor();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleTranslateAccelerator();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn OleTranslateColor();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleUIAddVerbMenuA();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleUIAddVerbMenuW();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Media`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
    pub fn OleUIBusyA();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Media`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Media"))]
    pub fn OleUIBusyW();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUICanConvertOrActivateAs();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIChangeIconA();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIChangeIconW();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_Controls_Dialogs`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
    pub fn OleUIChangeSourceA();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_UI_Controls_Dialogs`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls_Dialogs"))]
    pub fn OleUIChangeSourceW();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIConvertA();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIConvertW();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIEditLinksA();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIEditLinksW();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleUIInsertObjectA();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn OleUIInsertObjectW();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleUIObjectPropertiesA();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Controls", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn OleUIObjectPropertiesW();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleUIPasteSpecialA();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleUIPasteSpecialW();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIPromptUserA();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIPromptUserW();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIUpdateLinksA();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OleUIUpdateLinksW();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn OleUninitialize();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn QueryPathOfRegTypeLib();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn RegisterActiveObject();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterDragDrop();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn RegisterTypeLib();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn RegisterTypeLibForUser();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn ReleaseStgMedium();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn RevokeActiveObject();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RevokeDragDrop();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayAccessData();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayAddRef();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayAllocData();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayAllocDescriptor();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayAllocDescriptorEx();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCopy();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCopyData();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCreate();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCreateEx();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCreateVector();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayCreateVectorEx();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayDestroy();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayDestroyData();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayDestroyDescriptor();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetDim();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetElement();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetElemsize();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetIID();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetLBound();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetRecordInfo();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetUBound();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayGetVartype();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayLock();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayPtrOfIndex();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayPutElement();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayRedim();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn SafeArrayReleaseData();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayReleaseDescriptor();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArraySetIID();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArraySetRecordInfo();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayUnaccessData();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SafeArrayUnlock();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SystemTimeToVariantTime();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn UnRegisterTypeLib();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn UnRegisterTypeLibForUser();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarAbs();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarAdd();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarAnd();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarBoolFromCy();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarBoolFromDate();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBoolFromDec();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarBoolFromDisp();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBoolFromI1();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarBoolFromI2();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarBoolFromI4();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarBoolFromI8();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarBoolFromR4();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarBoolFromR8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBoolFromStr();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarBoolFromUI1();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarBoolFromUI2();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarBoolFromUI4();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarBoolFromUI8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrCat();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrCmp();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromBool();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarBstrFromCy();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromDate();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromDec();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarBstrFromDisp();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromI1();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromI2();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromI4();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromI8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromR4();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromR8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromUI1();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromUI2();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromUI4();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarBstrFromUI8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarCat();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarCmp();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyAbs();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyAdd();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyCmp();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyCmpR8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFix();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromBool();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromDate();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarCyFromDec();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromDisp();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarCyFromI1();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromI2();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromI4();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromI8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromR4();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromR8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarCyFromStr();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromUI1();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromUI2();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromUI4();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyFromUI8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyInt();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyMul();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyMulI4();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyMulI8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyNeg();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCyRound();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarCySub();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarDateFromBool();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarDateFromCy();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDateFromDec();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarDateFromDisp();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDateFromI1();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarDateFromI2();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarDateFromI4();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarDateFromI8();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarDateFromR4();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarDateFromR8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDateFromStr();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarDateFromUI1();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarDateFromUI2();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarDateFromUI4();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarDateFromUI8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDateFromUdate();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDateFromUdateEx();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecAbs();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecAdd();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecCmp();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecCmpR8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecDiv();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFix();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromBool();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarDecFromCy();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromDate();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarDecFromDisp();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromI1();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromI2();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromI4();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromI8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromR4();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromR8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromStr();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromUI1();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromUI2();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromUI4();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecFromUI8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecInt();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecMul();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecNeg();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecRound();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarDecSub();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarDiv();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarEqv();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFix();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFormat();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFormatCurrency();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFormatDateTime();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFormatFromTokens();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFormatNumber();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarFormatPercent();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromBool();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarI1FromCy();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromDate();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromDec();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarI1FromDisp();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromI2();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromI4();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromI8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromR4();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromR8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromStr();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromUI1();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromUI2();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromUI4();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI1FromUI8();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI2FromBool();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI2FromCy();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI2FromDate();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI2FromDec();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI2FromDisp();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI2FromI1();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI2FromI4();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI2FromI8();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI2FromR4();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI2FromR8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI2FromStr();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI2FromUI1();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI2FromUI2();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI2FromUI4();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI2FromUI8();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI4FromBool();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI4FromCy();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI4FromDate();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI4FromDec();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI4FromDisp();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI4FromI1();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI4FromI2();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI4FromI8();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI4FromR4();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI4FromR8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI4FromStr();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI4FromUI1();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI4FromUI2();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI4FromUI4();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI4FromUI8();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI8FromBool();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI8FromCy();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI8FromDate();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI8FromDec();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarI8FromDisp();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI8FromI1();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI8FromI2();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI8FromR4();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI8FromR8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarI8FromStr();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI8FromUI1();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI8FromUI2();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI8FromUI4();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarI8FromUI8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarIdiv();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarImp();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarInt();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarMod();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarMonthName();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarMul();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarNeg();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarNot();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarNumFromParseNum();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarOr();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarParseNumFromStr();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarPow();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR4CmpR8();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR4FromBool();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarR4FromCy();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR4FromDate();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarR4FromDec();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarR4FromDisp();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarR4FromI1();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR4FromI2();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR4FromI4();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR4FromI8();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR4FromR8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarR4FromStr();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR4FromUI1();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR4FromUI2();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR4FromUI4();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR4FromUI8();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR8FromBool();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarR8FromCy();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR8FromDate();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarR8FromDec();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarR8FromDisp();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarR8FromI1();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR8FromI2();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR8FromI4();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR8FromI8();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR8FromR4();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarR8FromStr();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR8FromUI1();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR8FromUI2();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR8FromUI4();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR8FromUI8();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR8Pow();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarR8Round();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarRound();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarSub();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarTokenizeFormatString();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI1FromBool();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI1FromCy();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI1FromDate();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI1FromDec();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI1FromDisp();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI1FromI1();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI1FromI2();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI1FromI4();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI1FromI8();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI1FromR4();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI1FromR8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI1FromStr();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI1FromUI2();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI1FromUI4();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI1FromUI8();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI2FromBool();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI2FromCy();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI2FromDate();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI2FromDec();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI2FromDisp();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI2FromI1();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI2FromI2();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI2FromI4();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI2FromI8();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI2FromR4();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI2FromR8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI2FromStr();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI2FromUI1();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI2FromUI4();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI2FromUI8();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI4FromBool();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI4FromCy();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI4FromDate();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI4FromDec();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI4FromDisp();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI4FromI1();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI4FromI2();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI4FromI4();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI4FromI8();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI4FromR4();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI4FromR8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI4FromStr();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI4FromUI1();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI4FromUI2();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI4FromUI8();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI8FromBool();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI8FromCy();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI8FromDate();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI8FromDec();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn VarUI8FromDisp();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI8FromI1();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI8FromI2();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI8FromI8();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI8FromR4();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI8FromR8();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUI8FromStr();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI8FromUI1();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI8FromUI2();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VarUI8FromUI4();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarUdateFromDate();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VarWeekdayName();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VarXor();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VariantChangeType();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VariantChangeTypeEx();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VariantClear();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VariantCopy();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VariantCopyInd();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VariantInit();
    #[doc = "*Required features: `Win32_System_Ole`*"]
    pub fn VariantTimeToDosDateTime();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn VariantTimeToSystemTime();
    #[doc = "*Required features: `Win32_System_Ole`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn VectorFromBstr();
}
