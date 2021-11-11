#![allow(non_snake_case, non_camel_case_types)]
#[cfg(feature = "Win32_UI_Controls_Dialogs")]
pub mod Dialogs;
#[cfg(feature = "Win32_UI_Controls_RichEdit")]
pub mod RichEdit;
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn BeginBufferedAnimation();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn BeginBufferedPaint();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BeginPanningFeedback();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BufferedPaintClear();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn BufferedPaintInit();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn BufferedPaintRenderAnimation();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BufferedPaintSetAlpha();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn BufferedPaintStopAllAnimations();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn BufferedPaintUnInit();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckDlgButton();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CheckRadioButton();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn CloseThemeData();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn CreateMappedBitmap();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn CreatePropertySheetPageA();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn CreatePropertySheetPageW();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateStatusWindowA();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateStatusWindowW();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn CreateSyntheticPointerDevice();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateToolbarEx();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateUpDownControl();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DPA_Clone();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DPA_Create();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPA_CreateEx();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPA_DeleteAllPtrs();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DPA_DeletePtr();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPA_Destroy();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DPA_DestroyCallback();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DPA_EnumCallback();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DPA_GetPtr();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DPA_GetPtrIndex();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DPA_GetSize();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPA_Grow();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DPA_InsertPtr();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn DPA_LoadStream();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPA_Merge();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn DPA_SaveStream();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPA_Search();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPA_SetPtr();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DPA_Sort();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DSA_Clone();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DSA_Create();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DSA_DeleteAllItems();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DSA_DeleteItem();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DSA_Destroy();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DSA_DestroyCallback();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DSA_EnumCallback();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DSA_GetItem();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DSA_GetItemPtr();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DSA_GetSize();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DSA_InsertItem();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DSA_SetItem();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DSA_Sort();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DestroyPropertySheetPage();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn DestroySyntheticPointerDevice();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DlgDirListA();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DlgDirListComboBoxA();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DlgDirListComboBoxW();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DlgDirListW();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DlgDirSelectComboBoxExA();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DlgDirSelectComboBoxExW();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DlgDirSelectExA();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DlgDirSelectExW();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn DrawInsert();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawShadowText();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawStatusTextA();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawStatusTextW();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawThemeBackground();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawThemeBackgroundEx();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawThemeEdge();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawThemeIcon();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawThemeParentBackground();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawThemeParentBackgroundEx();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawThemeText();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn DrawThemeTextEx();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn EnableScrollBar();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableThemeDialogTexture();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EnableTheming();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndBufferedAnimation();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndBufferedPaint();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EndPanningFeedback();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvaluateProximityToPolygon();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn EvaluateProximityToRect();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlatSB_EnableScrollBar();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn FlatSB_GetScrollInfo();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn FlatSB_GetScrollPos();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlatSB_GetScrollProp();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn FlatSB_GetScrollRange();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn FlatSB_SetScrollInfo();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn FlatSB_SetScrollPos();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn FlatSB_SetScrollProp();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn FlatSB_SetScrollRange();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn FlatSB_ShowScrollBar();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetBufferedPaintBits();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetBufferedPaintDC();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetBufferedPaintTargetDC();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetBufferedPaintTargetRect();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetComboBoxInfo();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetCurrentThemeName();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetEffectiveClientRect();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetListBoxInfo();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetMUILanguage();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetThemeAnimationProperty();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetThemeAnimationTransform();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetThemeAppProperties();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetThemeBackgroundContentRect();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetThemeBackgroundExtent();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetThemeBackgroundRegion();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetThemeBitmap();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemeBool();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetThemeColor();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemeDocumentationProperty();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetThemeEnumValue();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemeFilename();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetThemeFont();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetThemeInt();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetThemeIntList();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetThemeMargins();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetThemeMetric();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetThemePartSize();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemePosition();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetThemePropertyOrigin();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemeRect();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemeStream();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemeString();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemeSysBool();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetThemeSysColor();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetThemeSysColorBrush();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetThemeSysFont();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetThemeSysInt();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetThemeSysSize();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetThemeSysString();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn GetThemeTextExtent();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn GetThemeTextMetrics();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetThemeTimingFunction();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn GetThemeTransitionDuration();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowFeedbackSetting();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetWindowTheme();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn HIMAGELIST_QueryInterface();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn HitTestThemeBackground();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ImageList_Add();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Graphics_Gdi`*"]
    #[cfg(feature = "Win32_Graphics_Gdi")]
    pub fn ImageList_AddMasked();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_BeginDrag();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn ImageList_CoCreateInstance();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_Copy();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn ImageList_Create();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_Destroy();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_DragEnter();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_DragLeave();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_DragMove();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_DragShowNolock();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ImageList_Draw();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ImageList_DrawEx();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ImageList_DrawIndirect();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn ImageList_Duplicate();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn ImageList_EndDrag();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn ImageList_GetBkColor();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_GetDragImage();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn ImageList_GetIcon();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_GetIconSize();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn ImageList_GetImageCount();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ImageList_GetImageInfo();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ImageList_LoadImageA();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ImageList_LoadImageW();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn ImageList_Merge();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn ImageList_Read();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn ImageList_ReadEx();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_Remove();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi"))]
    pub fn ImageList_Replace();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(feature = "Win32_UI_WindowsAndMessaging")]
    pub fn ImageList_ReplaceIcon();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn ImageList_SetBkColor();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_SetDragCursorImage();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_SetIconSize();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_SetImageCount();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ImageList_SetOverlayImage();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn ImageList_Write();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn ImageList_WriteEx();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn InitCommonControls();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitCommonControlsEx();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn InitMUILanguage();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn InitializeFlatSB();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsAppThemed();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCharLowerW();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsCompositionActive();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsDlgButtonChecked();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsThemeActive();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsThemeBackgroundPartiallyTransparent();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsThemeDialogTextureEnabled();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsThemePartDefined();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LBItemFromPt();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn LoadIconMetric();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn LoadIconWithScaleDown();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MakeDragList();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn MenuHelp();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenThemeData();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn OpenThemeDataEx();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn PackTouchHitTestingProximityEvaluation();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn PropertySheetA();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_Graphics_Gdi`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_Graphics_Gdi", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn PropertySheetW();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterPointerDeviceNotifications();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn RegisterTouchHitTestingWindow();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetScrollInfo();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetScrollPos();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn SetScrollRange();
    #[doc = "*Required features: `Win32_UI_Controls`*"]
    pub fn SetThemeAppProperties();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowFeedbackSetting();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowTheme();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWindowThemeAttribute();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ShowHideMenuCtl();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn ShowScrollBar();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn Str_SetPtrW();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TaskDialog();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn TaskDialogIndirect();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UninitializeFlatSB();
    #[doc = "*Required features: `Win32_UI_Controls`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UpdatePanningFeedback();
}
