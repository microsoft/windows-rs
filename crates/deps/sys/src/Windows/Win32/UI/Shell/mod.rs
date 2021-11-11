#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_UI_Shell_Common")]
pub mod Common;
#[cfg(feature = "Win32_UI_Shell_PropertiesSystem")]
pub mod PropertiesSystem;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn AssocCreate();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn AssocCreateForClasses();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`, `Win32_UI_Shell_Common`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_UI_Shell_Common", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn AssocGetDetailsOfPropKey();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn AssocGetPerceivedType();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AssocIsDangerous();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn AssocQueryKeyA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn AssocQueryKeyW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AssocQueryStringA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn AssocQueryStringByKeyA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn AssocQueryStringByKeyW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AssocQueryStringW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Registry`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Registry", feature = "Win32_UI_Shell_Common"))]
    pub fn CDefFolderMenu_Create2();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn CIDLData_CreateFromIDArray();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChrCmpIA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChrCmpIW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ColorAdjustLuma();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn ColorHLSToRGB();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn ColorRGBToHLS();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CommandLineToArgvW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ConnectToConnectionPoint();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateProfile();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DAD_AutoScroll();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DAD_DragEnterEx();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn DAD_DragEnterEx2();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DAD_DragLeave();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DAD_DragMove();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn DAD_SetDragImage();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DAD_ShowDragImage();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefSubclassProc();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteProfileA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteProfileW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DoEnvironmentSubstA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DoEnvironmentSubstW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DragAcceptFiles();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn DragFinish();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DragQueryFileA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DragQueryFileW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DragQueryPoint();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn DriveType();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn DuplicateIcon();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ExtractAssociatedIconA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ExtractAssociatedIconExA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ExtractAssociatedIconExW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ExtractAssociatedIconW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ExtractIconA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ExtractIconExA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ExtractIconExW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ExtractIconW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindExecutableA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindExecutableW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAcceptLanguagesA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAcceptLanguagesW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAllUsersProfileDirectoryA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAllUsersProfileDirectoryW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentProcessExplicitAppUserModelID();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDefaultUserProfileDirectoryA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDefaultUserProfileDirectoryW();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn GetDpiForShellUIComponent();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetFileNameFromBrowse();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn GetMenuContextHelpId();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn GetMenuPosFromID();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfileType();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfilesDirectoryA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProfilesDirectoryW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn GetScaleFactorForDevice();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Graphics_Gdi`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Graphics_Gdi", feature = "Win32_UI_Shell_Common"))]
    pub fn GetScaleFactorForMonitor();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserProfileDirectoryA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetUserProfileDirectoryW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowContextHelpId();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowSubclass();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HMONITOR_UserFree();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HMONITOR_UserFree64();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HMONITOR_UserMarshal();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HMONITOR_UserMarshal64();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HMONITOR_UserSize();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HMONITOR_UserSize64();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HMONITOR_UserUnmarshal();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn HMONITOR_UserUnmarshal64();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn HashData();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn HlinkClone();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn HlinkCreateBrowseContext();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkCreateExtensionServices();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn HlinkCreateFromData();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HlinkCreateFromMoniker();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkCreateFromString();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkCreateShortcut();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HlinkCreateShortcutFromMoniker();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkCreateShortcutFromString();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkGetSpecialReference();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkGetValueFromParams();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkIsShortcut();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn HlinkNavigate();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HlinkNavigateToStringReference();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HlinkOnNavigate();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn HlinkOnRenameDocument();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HlinkParseDisplayName();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn HlinkPreprocessMoniker();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn HlinkQueryCreateFromData();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn HlinkResolveMonikerForData();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkResolveShortcut();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HlinkResolveShortcutToMoniker();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkResolveShortcutToString();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HlinkResolveStringForData();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkSetSpecialReference();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HlinkTranslateURL();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn HlinkUpdateStackItem();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn ILAppendID();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn ILClone();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn ILCloneFirst();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn ILCombine();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn ILCreateFromPathA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn ILCreateFromPathW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn ILFindChild();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn ILFindLastID();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn ILFree();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn ILGetNext();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn ILGetSize();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn ILIsEqual();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn ILIsParent();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn ILLoadFromStreamEx();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn ILRemoveLastID();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn ILSaveToStream();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn IStream_Copy();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn IStream_Read();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn IStream_ReadPidl();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn IStream_ReadStr();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn IStream_Reset();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn IStream_Size();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn IStream_Write();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn IStream_WritePidl();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn IStream_WriteStr();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn IUnknown_AtomicRelease();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn IUnknown_GetSite();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IUnknown_GetWindow();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn IUnknown_QueryService();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn IUnknown_Set();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn IUnknown_SetSite();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImportPrivacySettings();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitNetworkAddressControl();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IntlStrEqWorkerA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IntlStrEqWorkerW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCharSpaceA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCharSpaceW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsInternetESCEnabled();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsLFNDriveA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsLFNDriveW();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn IsNetDrive();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsOS();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsUserAnAdmin();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadUserProfileA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadUserProfileW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn OleSaveToStreamEx();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Registry"))]
    pub fn OpenRegStream();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ParseURLA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ParseURLW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathAddBackslashA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathAddBackslashW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathAddExtensionA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathAddExtensionW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathAllocCanonicalize();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathAllocCombine();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathAppendA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathAppendW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathBuildRootA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathBuildRootW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCanonicalizeA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCanonicalizeW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchAddBackslash();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchAddBackslashEx();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchAddExtension();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchAppend();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchAppendEx();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchCanonicalize();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchCanonicalizeEx();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchCombine();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchCombineEx();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchFindExtension();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchIsRoot();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchRemoveBackslash();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchRemoveBackslashEx();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchRemoveExtension();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchRemoveFileSpec();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchRenameExtension();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchSkipRoot();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchStripPrefix();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCchStripToRoot();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCleanupSpec();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCombineA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCombineW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCommonPrefixA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCommonPrefixW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn PathCompactPathA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCompactPathExA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCompactPathExW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn PathCompactPathW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCreateFromUrlA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCreateFromUrlAlloc();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathCreateFromUrlW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFileExistsA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFileExistsW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindExtensionA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindExtensionW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindFileNameA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindFileNameW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindNextComponentA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindNextComponentW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindOnPathA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindOnPathW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindSuffixArrayA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathFindSuffixArrayW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathGetArgsA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathGetArgsW();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn PathGetCharTypeA();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn PathGetCharTypeW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathGetDriveNumberA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathGetDriveNumberW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathGetShortPath();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsContentTypeA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsContentTypeW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsDirectoryA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsDirectoryEmptyA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsDirectoryEmptyW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsDirectoryW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsExe();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsFileSpecA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsFileSpecW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsLFNFileSpecA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsLFNFileSpecW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsNetworkPathA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsNetworkPathW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsPrefixA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsPrefixW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsRelativeA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsRelativeW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsRootA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsRootW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsSameRootA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsSameRootW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsSlowA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsSlowW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsSystemFolderA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsSystemFolderW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsUNCA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsUNCEx();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsUNCServerA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsUNCServerShareA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsUNCServerShareW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsUNCServerW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsUNCW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsURLA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathIsURLW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMakePrettyA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMakePrettyW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMakeSystemFolderA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMakeSystemFolderW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMakeUniqueName();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMatchSpecA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMatchSpecExA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMatchSpecExW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathMatchSpecW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathParseIconLocationA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathParseIconLocationW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathQualify();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathQuoteSpacesA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathQuoteSpacesW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRelativePathToA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRelativePathToW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveArgsA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveArgsW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveBackslashA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveBackslashW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveBlanksA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveBlanksW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveExtensionA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveExtensionW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveFileSpecA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRemoveFileSpecW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRenameExtensionA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathRenameExtensionW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathResolve();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathSearchAndQualifyA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathSearchAndQualifyW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathSetDlgItemPathA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathSetDlgItemPathW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathSkipRootA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathSkipRootW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathStripPathA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathStripPathW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathStripToRootA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathStripToRootW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathUnExpandEnvStringsA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathUnExpandEnvStringsW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathUndecorateA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathUndecorateW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathUnmakeSystemFolderA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathUnmakeSystemFolderW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathUnquoteSpacesA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathUnquoteSpacesW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PathYetAnotherMakeUniqueName();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PickIconDlg();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn QISearch();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReadCabinetState();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RealDriveType();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterAppConstrainedChangeNotification();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterAppStateChangeNotification();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterScaleChangeEvent();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterScaleChangeNotifications();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveWindowSubclass();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RestartDialog();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RestartDialogEx();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn RevokeScaleChangeNotifications();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn SHAddFromPropSheetExtArray();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHAddToRecentDocs();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHAlloc();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHAllocShared();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHAnsiToAnsi();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHAnsiToUnicode();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHAppBarMessage();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHAssocEnumHandlers();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHAssocEnumHandlersForProtocolByApplication();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHAutoComplete();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHBindToFolderIDListParent();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn SHBindToFolderIDListParentEx();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn SHBindToObject();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHBindToParent();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHBrowseForFolderA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHBrowseForFolderW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHCLSIDFromString();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHChangeNotification_Lock();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHChangeNotification_Unlock();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHChangeNotify();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHChangeNotifyDeregister();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHChangeNotifyRegister();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHChangeNotifyRegisterThread();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHCloneSpecialIDList();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHCoCreateInstance();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHCopyKeyA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHCopyKeyW();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHCreateAssociationRegistration();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn SHCreateDataObject();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry", feature = "Win32_UI_Shell_Common"))]
    pub fn SHCreateDefaultContextMenu();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHCreateDefaultExtractIcon();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHCreateDefaultPropertiesOp();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHCreateDirectory();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn SHCreateDirectoryExA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_Security`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security"))]
    pub fn SHCreateDirectoryExW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHCreateFileExtractIconW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHCreateItemFromIDList();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SHCreateItemFromParsingName();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SHCreateItemFromRelativeName();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHCreateItemInKnownFolder();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHCreateItemWithParent();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SHCreateMemStream();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_Security`, `Win32_System_Threading`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Security", feature = "Win32_System_Threading"))]
    pub fn SHCreateProcessAsUserW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHCreatePropSheetExtArray();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SHCreateQueryCancelAutoPlayMoniker();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHCreateShellFolderView();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHCreateShellFolderViewEx();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHCreateShellItem();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHCreateShellItemArray();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SHCreateShellItemArrayFromDataObject();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHCreateShellItemArrayFromIDLists();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHCreateShellItemArrayFromShellItem();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn SHCreateShellPalette();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SHCreateStdEnumFmtEtc();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SHCreateStreamOnFileA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SHCreateStreamOnFileEx();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SHCreateStreamOnFileW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Threading`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub fn SHCreateThread();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHCreateThreadRef();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Threading`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Threading"))]
    pub fn SHCreateThreadWithHandle();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SHDefExtractIconA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SHDefExtractIconW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHDeleteEmptyKeyA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHDeleteEmptyKeyW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHDeleteKeyA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHDeleteKeyW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHDeleteValueA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHDeleteValueW();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHDestroyPropSheetExtArray();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn SHDoDragDrop();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHEmptyRecycleBinA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHEmptyRecycleBinW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHEnumKeyExA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHEnumKeyExW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHEnumValueA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHEnumValueW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHEnumerateUnreadMailAccountsW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHEvaluateSystemCommandTemplate();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHFileOperationA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHFileOperationW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHFindFiles();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SHFind_InitMenuPopup();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHFlushSFCache();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHFormatDateTimeA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHFormatDateTimeW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHFormatDrive();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHFree();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHFreeNameMappings();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHFreeShared();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SHGetAttributesFromDataObject();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHGetDataFromIDListA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHGetDataFromIDListW();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHGetDesktopFolder();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetDiskFreeSpaceExA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetDiskFreeSpaceExW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetDriveMedia();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_Storage_FileSystem`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SHGetFileInfoA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_Storage_FileSystem`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Storage_FileSystem", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SHGetFileInfoW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHGetFolderLocation();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetFolderPathA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetFolderPathAndSubDirA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetFolderPathAndSubDirW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetFolderPathW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHGetIDListFromObject();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetIconOverlayIndexA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetIconOverlayIndexW();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHGetImageList();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHGetInstanceExplorer();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHGetInverseCMAP();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SHGetItemFromDataObject();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHGetItemFromObject();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHGetKnownFolderIDList();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetKnownFolderItem();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetKnownFolderPath();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetLocalizedName();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SHGetMalloc();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHGetNameFromIDList();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetNewLinkInfoA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetNewLinkInfoW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHGetPathFromIDListA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHGetPathFromIDListEx();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHGetPathFromIDListW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHGetRealIDL();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetSetFolderCustomSettings();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetSetSettings();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHGetSettings();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHGetSpecialFolderLocation();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetSpecialFolderPathA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHGetSpecialFolderPathW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn SHGetStockIconInfo();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn SHGetTemporaryPropertyForItem();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHGetThreadRef();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHGetUnreadMailCountW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHGetValueA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHGetValueW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHGetViewStatePropertyBag();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHGlobalCounterDecrement();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHGlobalCounterGetValue();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHGlobalCounterIncrement();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHHandleUpdateImage();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHILCreateFromPath();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHInvokePrinterCommandA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHInvokePrinterCommandW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHIsFileAvailableOffline();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHIsLowMemoryMachine();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHLimitInputEdit();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHLoadInProc();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHLoadIndirectString();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHLoadNonloadedIconOverlayIdentifiers();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHLockShared();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHMapPIDLToSystemImageListIndex();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHMessageBoxCheckA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHMessageBoxCheckW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn SHMultiFileProperties();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHObjectProperties();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_Shell_Common`*"]
    #[cfg(feature = "Win32_UI_Shell_Common")]
    pub fn SHOpenFolderAndSelectItems();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Registry"))]
    pub fn SHOpenPropSheetW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Registry"))]
    pub fn SHOpenRegStream2A();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Registry"))]
    pub fn SHOpenRegStream2W();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Registry"))]
    pub fn SHOpenRegStreamA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Registry"))]
    pub fn SHOpenRegStreamW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHOpenWithDialog();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_UI_Shell_Common"))]
    pub fn SHParseDisplayName();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHPathPrepareForWriteA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHPathPrepareForWriteW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHQueryInfoKeyA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHQueryInfoKeyW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHQueryRecycleBinA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHQueryRecycleBinW();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHQueryUserNotificationState();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHQueryValueExA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHQueryValueExW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegCloseUSKey();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegCreateUSKeyA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegCreateUSKeyW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegDeleteEmptyUSKeyA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegDeleteEmptyUSKeyW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegDeleteUSValueA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegDeleteUSValueW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_System_Registry`*"]
    #[cfg(feature = "Win32_System_Registry")]
    pub fn SHRegDuplicateHKey();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegEnumUSKeyA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegEnumUSKeyW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegEnumUSValueA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegEnumUSValueW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegGetBoolUSValueA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegGetBoolUSValueW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHRegGetIntW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHRegGetPathA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHRegGetPathW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegGetUSValueA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegGetUSValueW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHRegGetValueA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegGetValueFromHKCUHKLM();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHRegGetValueW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegOpenUSKeyA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegOpenUSKeyW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegQueryInfoUSKeyA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegQueryInfoUSKeyW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegQueryUSValueA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegQueryUSValueW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHRegSetPathA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHRegSetPathW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegSetUSValueA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegSetUSValueW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegWriteUSValueA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRegWriteUSValueW();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHReleaseThreadRef();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHRemoveLocalizedName();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn SHReplaceFromPropSheetExtArray();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHResolveLibrary();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHRestricted();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHSendMessageBroadcastA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHSendMessageBroadcastW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHSetDefaultProperties();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHSetFolderPathA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHSetFolderPathW();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHSetInstanceExplorer();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHSetKnownFolderPath();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHSetLocalizedName();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Com_StructuredStorage`, `Win32_UI_Shell_PropertiesSystem`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Com_StructuredStorage", feature = "Win32_UI_Shell_PropertiesSystem"))]
    pub fn SHSetTemporaryPropertyForItem();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn SHSetThreadRef();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHSetUnreadMailCountW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHSetValueA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn SHSetValueW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHShellFolderView_Message();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHShowManageLibraryUI();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SHSimpleIDListFromPath();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn SHSkipJunction();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHStartNetConnectionDialogW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHStrDupA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHStrDupW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHStripMneumonicA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHStripMneumonicW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHTestTokenMembership();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHUnicodeToAnsi();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHUnicodeToUnicode();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHUnlockShared();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHUpdateImageA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHUpdateImageW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SHValidateUNC();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCurrentProcessExplicitAppUserModelID();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetMenuContextHelpId();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowContextHelpId();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowSubclass();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ShellAboutA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ShellAboutW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShellExecuteA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ShellExecuteExA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Registry`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Registry"))]
    pub fn ShellExecuteExW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShellExecuteW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShellMessageBoxA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShellMessageBoxW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Shell_GetCachedImageIndex();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Shell_GetCachedImageIndexA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Shell_GetCachedImageIndexW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Controls`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Controls"))]
    pub fn Shell_GetImageLists();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn Shell_MergeMenus();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn Shell_NotifyIconA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Shell_NotifyIconGetRect();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn Shell_NotifyIconW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn SignalFileOpen();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com_Urlmon`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_Urlmon"))]
    pub fn SoftwareUpdateMessageBox();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_System_Com_StructuredStorage`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com_StructuredStorage"))]
    pub fn StgMakeUniqueName();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCSpnA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCSpnIA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCSpnIW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCSpnW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCatBuffA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCatBuffW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCatChainW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCatW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrChrA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrChrIA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrChrIW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrChrNIW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrChrNW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrChrW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpCA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpCW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpICA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpICW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpIW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpLogicalW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpNA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpNCA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpNCW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpNIA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpNICA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpNICW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpNIW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpNW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCmpW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCpyNW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrCpyW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrDupA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrDupW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrFormatByteSize64A();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrFormatByteSizeA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrFormatByteSizeEx();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrFormatByteSizeW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrFormatKBSizeA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrFormatKBSizeW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrFromTimeIntervalA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrFromTimeIntervalW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrIsIntlEqualA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrIsIntlEqualW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrNCatA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrNCatW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrPBrkA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrPBrkW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrRChrA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrRChrIA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrRChrIW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrRChrW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrRStrIA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrRStrIW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn StrRetToBSTR();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn StrRetToBufA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn StrRetToBufW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn StrRetToStrA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`, `Win32_UI_Shell_Common`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell_Common"))]
    pub fn StrRetToStrW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrSpnA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrSpnW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrStrA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrStrIA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrStrIW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrStrNIW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrStrNW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrStrW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrToInt64ExA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrToInt64ExW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrToIntA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrToIntExA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrToIntExW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrToIntW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrTrimA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn StrTrimW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnloadUserProfile();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn UnregisterAppConstrainedChangeNotification();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn UnregisterAppStateChangeNotification();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn UnregisterScaleChangeEvent();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlApplySchemeA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlApplySchemeW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCanonicalizeA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCanonicalizeW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCombineA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCombineW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCompareA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCompareW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCreateFromPathA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlCreateFromPathW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlEscapeA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlEscapeW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlFixupW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlGetLocationA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlGetLocationW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlGetPartA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlGetPartW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlHashA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlHashW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlIsA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlIsNoHistoryA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlIsNoHistoryW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlIsOpaqueA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlIsOpaqueW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlIsW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlUnescapeA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UrlUnescapeW();
    #[doc = "*Required features: `Win32_UI_Shell`*"]
    pub fn WhichPlatform();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Win32DeleteFile();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHelpA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WinHelpW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WriteCabinetState();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn wnsprintfA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn wnsprintfW();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn wvnsprintfA();
    #[doc = "*Required features: `Win32_UI_Shell`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn wvnsprintfW();
}
