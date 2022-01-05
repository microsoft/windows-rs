pub trait IRichEditOleImpl: Sized {
    fn GetClientSite();
    fn GetObjectCount();
    fn GetLinkCount();
    fn GetObject();
    fn InsertObject();
    fn ConvertObject();
    fn ActivateAs();
    fn SetHostNames();
    fn SetLinkAvailable();
    fn SetDvaspect();
    fn HandsOffStorage();
    fn SaveCompleted();
    fn InPlaceDeactivate();
    fn ContextSensitiveHelp();
    fn GetClipboardData();
    fn ImportDataObject();
}
pub trait IRichEditOleCallbackImpl: Sized {
    fn GetNewStorage();
    fn GetInPlaceContext();
    fn ShowContainerUI();
    fn QueryInsertObject();
    fn DeleteObject();
    fn QueryAcceptData();
    fn ContextSensitiveHelp();
    fn GetClipboardData();
    fn GetDragDropEffect();
    fn GetContextMenu();
}
pub trait IRicheditUiaOverridesImpl: Sized {
    fn GetPropertyOverrideValue();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextDisplaysImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextDocumentImpl: Sized + IDispatchImpl {
    fn GetName();
    fn GetSelection();
    fn GetStoryCount();
    fn GetStoryRanges();
    fn GetSaved();
    fn SetSaved();
    fn GetDefaultTabStop();
    fn SetDefaultTabStop();
    fn New();
    fn Open();
    fn Save();
    fn Freeze();
    fn Unfreeze();
    fn BeginEditCollection();
    fn EndEditCollection();
    fn Undo();
    fn Redo();
    fn Range();
    fn RangeFromPoint();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextDocument2Impl: Sized + ITextDocumentImpl + IDispatchImpl {
    fn GetCaretType();
    fn SetCaretType();
    fn GetDisplays();
    fn GetDocumentFont();
    fn SetDocumentFont();
    fn GetDocumentPara();
    fn SetDocumentPara();
    fn GetEastAsianFlags();
    fn GetGenerator();
    fn SetIMEInProgress();
    fn GetNotificationMode();
    fn SetNotificationMode();
    fn GetSelection2();
    fn GetStoryRanges2();
    fn GetTypographyOptions();
    fn GetVersion();
    fn GetWindow();
    fn AttachMsgFilter();
    fn CheckTextLimit();
    fn GetCallManager();
    fn GetClientRect();
    fn GetEffectColor();
    fn GetImmContext();
    fn GetPreferredFont();
    fn GetProperty();
    fn GetStrings();
    fn Notify();
    fn Range2();
    fn RangeFromPoint2();
    fn ReleaseCallManager();
    fn ReleaseImmContext();
    fn SetEffectColor();
    fn SetProperty();
    fn SetTypographyOptions();
    fn SysBeep();
    fn Update();
    fn UpdateWindow();
    fn GetMathProperties();
    fn SetMathProperties();
    fn GetActiveStory();
    fn SetActiveStory();
    fn GetMainStory();
    fn GetNewStory();
    fn GetStory();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextDocument2OldImpl: Sized + ITextDocumentImpl + IDispatchImpl {
    fn AttachMsgFilter();
    fn SetEffectColor();
    fn GetEffectColor();
    fn GetCaretType();
    fn SetCaretType();
    fn GetImmContext();
    fn ReleaseImmContext();
    fn GetPreferredFont();
    fn GetNotificationMode();
    fn SetNotificationMode();
    fn GetClientRect();
    fn GetSelection2();
    fn GetWindow();
    fn GetFEFlags();
    fn UpdateWindow();
    fn CheckTextLimit();
    fn IMEInProgress();
    fn SysBeep();
    fn Update();
    fn Notify();
    fn GetDocumentFont();
    fn GetDocumentPara();
    fn GetCallManager();
    fn ReleaseCallManager();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextFontImpl: Sized + IDispatchImpl {
    fn GetDuplicate();
    fn SetDuplicate();
    fn CanChange();
    fn IsEqual();
    fn Reset();
    fn GetStyle();
    fn SetStyle();
    fn GetAllCaps();
    fn SetAllCaps();
    fn GetAnimation();
    fn SetAnimation();
    fn GetBackColor();
    fn SetBackColor();
    fn GetBold();
    fn SetBold();
    fn GetEmboss();
    fn SetEmboss();
    fn GetForeColor();
    fn SetForeColor();
    fn GetHidden();
    fn SetHidden();
    fn GetEngrave();
    fn SetEngrave();
    fn GetItalic();
    fn SetItalic();
    fn GetKerning();
    fn SetKerning();
    fn GetLanguageID();
    fn SetLanguageID();
    fn GetName();
    fn SetName();
    fn GetOutline();
    fn SetOutline();
    fn GetPosition();
    fn SetPosition();
    fn GetProtected();
    fn SetProtected();
    fn GetShadow();
    fn SetShadow();
    fn GetSize();
    fn SetSize();
    fn GetSmallCaps();
    fn SetSmallCaps();
    fn GetSpacing();
    fn SetSpacing();
    fn GetStrikeThrough();
    fn SetStrikeThrough();
    fn GetSubscript();
    fn SetSubscript();
    fn GetSuperscript();
    fn SetSuperscript();
    fn GetUnderline();
    fn SetUnderline();
    fn GetWeight();
    fn SetWeight();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextFont2Impl: Sized + ITextFontImpl + IDispatchImpl {
    fn GetCount();
    fn GetAutoLigatures();
    fn SetAutoLigatures();
    fn GetAutospaceAlpha();
    fn SetAutospaceAlpha();
    fn GetAutospaceNumeric();
    fn SetAutospaceNumeric();
    fn GetAutospaceParens();
    fn SetAutospaceParens();
    fn GetCharRep();
    fn SetCharRep();
    fn GetCompressionMode();
    fn SetCompressionMode();
    fn GetCookie();
    fn SetCookie();
    fn GetDoubleStrike();
    fn SetDoubleStrike();
    fn GetDuplicate2();
    fn SetDuplicate2();
    fn GetLinkType();
    fn GetMathZone();
    fn SetMathZone();
    fn GetModWidthPairs();
    fn SetModWidthPairs();
    fn GetModWidthSpace();
    fn SetModWidthSpace();
    fn GetOldNumbers();
    fn SetOldNumbers();
    fn GetOverlapping();
    fn SetOverlapping();
    fn GetPositionSubSuper();
    fn SetPositionSubSuper();
    fn GetScaling();
    fn SetScaling();
    fn GetSpaceExtension();
    fn SetSpaceExtension();
    fn GetUnderlinePositionMode();
    fn SetUnderlinePositionMode();
    fn GetEffects();
    fn GetEffects2();
    fn GetProperty();
    fn GetPropertyInfo();
    fn IsEqual2();
    fn SetEffects();
    fn SetEffects2();
    fn SetProperty();
}
pub trait ITextHostImpl: Sized {
    fn TxGetDC();
    fn TxReleaseDC();
    fn TxShowScrollBar();
    fn TxEnableScrollBar();
    fn TxSetScrollRange();
    fn TxSetScrollPos();
    fn TxInvalidateRect();
    fn TxViewChange();
    fn TxCreateCaret();
    fn TxShowCaret();
    fn TxSetCaretPos();
    fn TxSetTimer();
    fn TxKillTimer();
    fn TxScrollWindowEx();
    fn TxSetCapture();
    fn TxSetFocus();
    fn TxSetCursor();
    fn TxScreenToClient();
    fn TxClientToScreen();
    fn TxActivate();
    fn TxDeactivate();
    fn TxGetClientRect();
    fn TxGetViewInset();
    fn TxGetCharFormat();
    fn TxGetParaFormat();
    fn TxGetSysColor();
    fn TxGetBackStyle();
    fn TxGetMaxLength();
    fn TxGetScrollBars();
    fn TxGetPasswordChar();
    fn TxGetAcceleratorPos();
    fn TxGetExtent();
    fn OnTxCharFormatChange();
    fn OnTxParaFormatChange();
    fn TxGetPropertyBits();
    fn TxNotify();
    fn TxImmGetContext();
    fn TxImmReleaseContext();
    fn TxGetSelectionBarWidth();
}
pub trait ITextHost2Impl: Sized + ITextHostImpl {
    fn TxIsDoubleClickPending();
    fn TxGetWindow();
    fn TxSetForegroundWindow();
    fn TxGetPalette();
    fn TxGetEastAsianFlags();
    fn TxSetCursor2();
    fn TxFreeTextServicesNotification();
    fn TxGetEditStyle();
    fn TxGetWindowStyles();
    fn TxShowDropCaret();
    fn TxDestroyCaret();
    fn TxGetHorzExtent();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextParaImpl: Sized + IDispatchImpl {
    fn GetDuplicate();
    fn SetDuplicate();
    fn CanChange();
    fn IsEqual();
    fn Reset();
    fn GetStyle();
    fn SetStyle();
    fn GetAlignment();
    fn SetAlignment();
    fn GetHyphenation();
    fn SetHyphenation();
    fn GetFirstLineIndent();
    fn GetKeepTogether();
    fn SetKeepTogether();
    fn GetKeepWithNext();
    fn SetKeepWithNext();
    fn GetLeftIndent();
    fn GetLineSpacing();
    fn GetLineSpacingRule();
    fn GetListAlignment();
    fn SetListAlignment();
    fn GetListLevelIndex();
    fn SetListLevelIndex();
    fn GetListStart();
    fn SetListStart();
    fn GetListTab();
    fn SetListTab();
    fn GetListType();
    fn SetListType();
    fn GetNoLineNumber();
    fn SetNoLineNumber();
    fn GetPageBreakBefore();
    fn SetPageBreakBefore();
    fn GetRightIndent();
    fn SetRightIndent();
    fn SetIndents();
    fn SetLineSpacing();
    fn GetSpaceAfter();
    fn SetSpaceAfter();
    fn GetSpaceBefore();
    fn SetSpaceBefore();
    fn GetWidowControl();
    fn SetWidowControl();
    fn GetTabCount();
    fn AddTab();
    fn ClearAllTabs();
    fn DeleteTab();
    fn GetTab();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextPara2Impl: Sized + ITextParaImpl + IDispatchImpl {
    fn GetBorders();
    fn GetDuplicate2();
    fn SetDuplicate2();
    fn GetFontAlignment();
    fn SetFontAlignment();
    fn GetHangingPunctuation();
    fn SetHangingPunctuation();
    fn GetSnapToGrid();
    fn SetSnapToGrid();
    fn GetTrimPunctuationAtStart();
    fn SetTrimPunctuationAtStart();
    fn GetEffects();
    fn GetProperty();
    fn IsEqual2();
    fn SetEffects();
    fn SetProperty();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextRangeImpl: Sized + IDispatchImpl {
    fn GetText();
    fn SetText();
    fn GetChar();
    fn SetChar();
    fn GetDuplicate();
    fn GetFormattedText();
    fn SetFormattedText();
    fn GetStart();
    fn SetStart();
    fn GetEnd();
    fn SetEnd();
    fn GetFont();
    fn SetFont();
    fn GetPara();
    fn SetPara();
    fn GetStoryLength();
    fn GetStoryType();
    fn Collapse();
    fn Expand();
    fn GetIndex();
    fn SetIndex();
    fn SetRange();
    fn InRange();
    fn InStory();
    fn IsEqual();
    fn Select();
    fn StartOf();
    fn EndOf();
    fn Move();
    fn MoveStart();
    fn MoveEnd();
    fn MoveWhile();
    fn MoveStartWhile();
    fn MoveEndWhile();
    fn MoveUntil();
    fn MoveStartUntil();
    fn MoveEndUntil();
    fn FindText();
    fn FindTextStart();
    fn FindTextEnd();
    fn Delete();
    fn Cut();
    fn Copy();
    fn Paste();
    fn CanPaste();
    fn CanEdit();
    fn ChangeCase();
    fn GetPoint();
    fn SetPoint();
    fn ScrollIntoView();
    fn GetEmbeddedObject();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextRange2Impl: Sized + ITextSelectionImpl + ITextRangeImpl + IDispatchImpl {
    fn GetCch();
    fn GetCells();
    fn GetColumn();
    fn GetCount();
    fn GetDuplicate2();
    fn GetFont2();
    fn SetFont2();
    fn GetFormattedText2();
    fn SetFormattedText2();
    fn GetGravity();
    fn SetGravity();
    fn GetPara2();
    fn SetPara2();
    fn GetRow();
    fn GetStartPara();
    fn GetTable();
    fn GetURL();
    fn SetURL();
    fn AddSubrange();
    fn BuildUpMath();
    fn DeleteSubrange();
    fn Find();
    fn GetChar2();
    fn GetDropCap();
    fn GetInlineObject();
    fn GetProperty();
    fn GetRect();
    fn GetSubrange();
    fn GetText2();
    fn HexToUnicode();
    fn InsertTable();
    fn Linearize();
    fn SetActiveSubrange();
    fn SetDropCap();
    fn SetProperty();
    fn SetText2();
    fn UnicodeToHex();
    fn SetInlineObject();
    fn GetMathFunctionType();
    fn InsertImage();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextRowImpl: Sized + IDispatchImpl {
    fn GetAlignment();
    fn SetAlignment();
    fn GetCellCount();
    fn SetCellCount();
    fn GetCellCountCache();
    fn SetCellCountCache();
    fn GetCellIndex();
    fn SetCellIndex();
    fn GetCellMargin();
    fn SetCellMargin();
    fn GetHeight();
    fn SetHeight();
    fn GetIndent();
    fn SetIndent();
    fn GetKeepTogether();
    fn SetKeepTogether();
    fn GetKeepWithNext();
    fn SetKeepWithNext();
    fn GetNestLevel();
    fn GetRTL();
    fn SetRTL();
    fn GetCellAlignment();
    fn SetCellAlignment();
    fn GetCellColorBack();
    fn SetCellColorBack();
    fn GetCellColorFore();
    fn SetCellColorFore();
    fn GetCellMergeFlags();
    fn SetCellMergeFlags();
    fn GetCellShading();
    fn SetCellShading();
    fn GetCellVerticalText();
    fn SetCellVerticalText();
    fn GetCellWidth();
    fn SetCellWidth();
    fn GetCellBorderColors();
    fn GetCellBorderWidths();
    fn SetCellBorderColors();
    fn SetCellBorderWidths();
    fn Apply();
    fn CanChange();
    fn GetProperty();
    fn Insert();
    fn IsEqual();
    fn Reset();
    fn SetProperty();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextSelectionImpl: Sized + ITextRangeImpl + IDispatchImpl {
    fn GetFlags();
    fn SetFlags();
    fn GetType();
    fn MoveLeft();
    fn MoveRight();
    fn MoveUp();
    fn MoveDown();
    fn HomeKey();
    fn EndKey();
    fn TypeText();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextSelection2Impl: Sized + ITextRange2Impl + ITextSelectionImpl + ITextRangeImpl + IDispatchImpl {}
pub trait ITextServicesImpl: Sized {
    fn TxSendMessage();
    fn TxDraw();
    fn TxGetHScroll();
    fn TxGetVScroll();
    fn OnTxSetCursor();
    fn TxQueryHitPoint();
    fn OnTxInPlaceActivate();
    fn OnTxInPlaceDeactivate();
    fn OnTxUIActivate();
    fn OnTxUIDeactivate();
    fn TxGetText();
    fn TxSetText();
    fn TxGetCurTargetX();
    fn TxGetBaseLinePos();
    fn TxGetNaturalSize();
    fn TxGetDropTarget();
    fn OnTxPropertyBitsChange();
    fn TxGetCachedSize();
}
pub trait ITextServices2Impl: Sized + ITextServicesImpl {
    fn TxGetNaturalSize2();
    fn TxDrawD2D();
}
pub trait ITextStoryImpl: Sized {
    fn GetActive();
    fn SetActive();
    fn GetDisplay();
    fn GetIndex();
    fn GetType();
    fn SetType();
    fn GetProperty();
    fn GetRange();
    fn GetText();
    fn SetFormattedText();
    fn SetProperty();
    fn SetText();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextStoryRangesImpl: Sized + IDispatchImpl {
    fn _NewEnum();
    fn Item();
    fn GetCount();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextStoryRanges2Impl: Sized + ITextStoryRangesImpl + IDispatchImpl {
    fn Item2();
}
#[cfg(feature = "Win32_System_Com")]
pub trait ITextStringsImpl: Sized + IDispatchImpl {
    fn Item();
    fn GetCount();
    fn Add();
    fn Append();
    fn Cat2();
    fn CatTop2();
    fn DeleteRange();
    fn EncodeFunction();
    fn GetCch();
    fn InsertNullStr();
    fn MoveBoundary();
    fn PrefixTop();
    fn Remove();
    fn SetFormattedText();
    fn SetOpCp();
    fn SuffixTop();
    fn Swap();
}
