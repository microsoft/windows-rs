#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdjustWindowRect();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AdjustWindowRectEx();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AllowSetForegroundWindow();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AnimateWindow();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AnyPopup();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppendMenuA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AppendMenuW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ArrangeIconicWindows();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn BeginDeferWindowPos();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BringWindowToTop();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CalculatePopupWindowPosition();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CallMsgFilterA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CallMsgFilterW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CallNextHookEx();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CallWindowProcA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CallWindowProcW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CancelShutdown();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CascadeWindows();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeMenuA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeMenuW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeWindowMessageFilter();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChangeWindowMessageFilterEx();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharLowerA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharLowerBuffA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharLowerBuffW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharLowerW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharNextA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharNextExA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharNextW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharPrevA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharPrevExA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharPrevW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharToOemA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharToOemBuffA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharToOemBuffW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharToOemW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharUpperA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharUpperBuffA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharUpperBuffW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CharUpperW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn CheckMenuItem();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckMenuRadioItem();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChildWindowFromPoint();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ChildWindowFromPointEx();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ClipCursor();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CloseWindow();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn CopyAcceleratorTableA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn CopyAcceleratorTableW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn CopyIcon();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CopyImage();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn CreateAcceleratorTableA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn CreateAcceleratorTableW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CreateCaret();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateCursor();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDialogIndirectParamA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDialogIndirectParamW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDialogParamA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateDialogParamW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateIcon();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateIconFromResource();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateIconFromResourceEx();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CreateIconIndirect();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateMDIWindowA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateMDIWindowW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn CreateMenu();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn CreatePopupMenu();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateResourceIndexer();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateWindowExA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateWindowExW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefDlgProcA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefDlgProcW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefFrameProcA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefFrameProcW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefMDIChildProcA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefMDIChildProcW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefWindowProcA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DefWindowProcW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeferWindowPos();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeleteMenu();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DeregisterShellHookWindow();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyAcceleratorTable();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyCaret();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyCursor();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyIcon();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyIndexedResults();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyMenu();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn DestroyResourceIndexer();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyWindow();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DialogBoxIndirectParamA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DialogBoxIndirectParamW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DialogBoxParamA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DialogBoxParamW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn DisableProcessWindowsGhosting();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DispatchMessageA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DispatchMessageW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DragObject();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawIcon();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawIconEx();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawMenuBar();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableMenuItem();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndDeferWindowPos();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndDialog();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndMenu();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumChildWindows();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPropsA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPropsExA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPropsExW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumPropsW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumThreadWindows();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnumWindows();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindWindowA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindWindowExA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindWindowExW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FindWindowW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlashWindow();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlashWindowEx();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAltTabInfoA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAltTabInfoW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetAncestor();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetCaretBlinkTime();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCaretPos();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetClassInfoA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetClassInfoExA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetClassInfoExW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetClassInfoW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClassLongA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClassLongPtrA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClassLongPtrW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClassLongW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClassNameA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClassNameW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClassWord();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClientRect();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetClipCursor();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetCursor();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCursorInfo();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCursorPos();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDesktopWindow();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetDialogBaseUnits();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDlgCtrlID();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDlgItem();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDlgItemInt();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDlgItemTextA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetDlgItemTextW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetForegroundWindow();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetGUIThreadInfo();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetIconInfo();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetIconInfoExA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetIconInfoExW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetInputState();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLastActivePopup();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetLayeredWindowAttributes();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMenu();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMenuBarInfo();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetMenuCheckMarkDimensions();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetMenuDefaultItem();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetMenuInfo();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetMenuItemCount();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetMenuItemID();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetMenuItemInfoA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetMenuItemInfoW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMenuItemRect();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetMenuState();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMenuStringA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMenuStringW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMessageA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMessageExtraInfo();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetMessagePos();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetMessageTime();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetMessageW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNextDlgGroupItem();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetNextDlgTabItem();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetParent();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPhysicalCursorPos();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetProcessDefaultLayout();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPropA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetPropW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetQueueStatus();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetScrollBarInfo();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetScrollInfo();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetScrollPos();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetScrollRange();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetShellWindow();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetSubMenu();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetSysColor();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetSystemMenu();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn GetSystemMetrics();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTitleBarInfo();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetTopWindow();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindow();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowDisplayAffinity();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowInfo();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowLongA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowLongPtrA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowLongPtrW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowLongW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowModuleFileNameA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowModuleFileNameW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowPlacement();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowRect();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowTextA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowTextLengthA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowTextLengthW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowTextW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowThreadProcessId();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowWord();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HideCaret();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn HiliteMenuItem();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InSendMessage();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn InSendMessageEx();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IndexFilePath();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InheritWindowMonitor();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InsertMenuA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn InsertMenuItemA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn InsertMenuItemW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InsertMenuW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InternalGetWindowText();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCharAlphaA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCharAlphaNumericA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCharAlphaNumericW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCharAlphaW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCharLowerA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCharUpperA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCharUpperW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsChild();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDialogMessageA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDialogMessageW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsGUIThread();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsHungAppWindow();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsIconic();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsMenu();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsProcessDPIAware();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWindow();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWindowUnicode();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWindowVisible();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWow64Message();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsZoomed();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn KillTimer();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadAcceleratorsA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadAcceleratorsW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadCursorA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadCursorFromFileA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadCursorFromFileW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadCursorW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadIconA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadIconW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadImageA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadImageW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadMenuA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn LoadMenuIndirectA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn LoadMenuIndirectW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadMenuW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadStringA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LoadStringW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LockSetForegroundWindow();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LogicalToPhysicalPoint();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupIconIdFromDirectory();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LookupIconIdFromDirectoryEx();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MapDialogRect();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MenuItemFromPoint();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MessageBoxA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MessageBoxExA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MessageBoxExW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_UI_Shell`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
    pub fn MessageBoxIndirectA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_UI_Shell`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_Shell"))]
    pub fn MessageBoxIndirectW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MessageBoxW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ModifyMenuA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ModifyMenuW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MoveWindow();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmCreateConfig();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmCreateConfigInMemory();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmCreateResourceFile();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn MrmCreateResourceFileInMemory();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmCreateResourceFileWithChecksum();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmCreateResourceIndexer();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmCreateResourceIndexerFromPreviousPriData();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmCreateResourceIndexerFromPreviousPriFile();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmCreateResourceIndexerFromPreviousSchemaData();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmCreateResourceIndexerFromPreviousSchemaFile();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmCreateResourceIndexerWithFlags();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn MrmDestroyIndexerAndMessages();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn MrmDumpPriDataInMemory();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmDumpPriFile();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmDumpPriFileInMemory();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn MrmFreeMemory();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmGetPriFileContentChecksum();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmIndexEmbeddedData();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmIndexFile();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmIndexFileAutoQualifiers();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmIndexResourceContainerAutoQualifiers();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmIndexString();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MrmPeekResourceIndexerMessages();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsgWaitForMultipleObjects();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MsgWaitForMultipleObjectsEx();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OemToCharA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OemToCharBuffA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OemToCharBuffW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OemToCharW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenIcon();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeekMessageA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PeekMessageW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PhysicalToLogicalPoint();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PostMessageA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PostMessageW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn PostQuitMessage();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PostThreadMessageA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PostThreadMessageW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrivateExtractIconsA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PrivateExtractIconsW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RealChildWindowFromPoint();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RealGetWindowClassA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RealGetWindowClassW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn RegisterClassA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn RegisterClassExA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn RegisterClassExW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn RegisterClassW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_System_Power`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Power"))]
    pub fn RegisterDeviceNotificationA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_System_Power`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Power"))]
    pub fn RegisterDeviceNotificationW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterShellHookWindow();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterWindowMessageA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterWindowMessageW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemoveMenu();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemovePropA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RemovePropW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ReplyMessage();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScrollDC();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ScrollWindow();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ScrollWindowEx();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendDlgItemMessageA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendDlgItemMessageW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendMessageA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendMessageCallbackA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendMessageCallbackW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendMessageTimeoutA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendMessageTimeoutW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendMessageW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendNotifyMessageA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SendNotifyMessageW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCaretBlinkTime();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCaretPos();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClassLongA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClassLongPtrA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClassLongPtrW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClassLongW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetClassWord();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCoalescableTimer();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn SetCursor();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetCursorPos();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`*"]
    pub fn SetDebugErrorLevel();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDlgItemInt();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDlgItemTextA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetDlgItemTextW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetForegroundWindow();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetLayeredWindowAttributes();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMenu();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMenuDefaultItem();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetMenuInfo();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetMenuItemBitmaps();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetMenuItemInfoA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn SetMenuItemInfoW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMessageExtraInfo();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetMessageQueue();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetParent();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPhysicalCursorPos();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessDPIAware();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetProcessDefaultLayout();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPropA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetPropW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSysColors();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetSystemCursor();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetTimer();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowDisplayAffinity();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowLongA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowLongPtrA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64",))]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowLongPtrW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowLongW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowPlacement();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowPos();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowTextA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowTextW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowWord();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowsHookA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowsHookExA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowsHookExW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowsHookW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShowCaret();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShowCursor();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShowOwnedPopups();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShowWindow();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShowWindowAsync();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SoundSentry();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SwitchToThisWindow();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SystemParametersInfoA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SystemParametersInfoW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TileWindows();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TrackPopupMenu();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TrackPopupMenuEx();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateAcceleratorA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateAcceleratorW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateMDISysAccel();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TranslateMessage();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnhookWindowsHook();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnhookWindowsHookEx();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterClassA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnregisterClassW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn UpdateLayeredWindow();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn UpdateLayeredWindowIndirect();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WaitMessage();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowFromPhysicalPoint();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowFromPoint();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn wsprintfA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn wsprintfW();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn wvsprintfA();
    #[doc = "*Required features: `Win32_UI_WindowsAndMessaging`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn wvsprintfW();
}
