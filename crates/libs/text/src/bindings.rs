windows_core::link!("dwrite.dll" "system" fn DWriteCreateFactory(factorytype : DWRITE_FACTORY_TYPE, iid : *const IID, factory : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
pub type DWRITE_FACTORY_TYPE = i32;
pub const DWRITE_FACTORY_TYPE_SHARED: DWRITE_FACTORY_TYPE = 0;
pub type DWRITE_FONT_STRETCH = i32;
pub const DWRITE_FONT_STRETCH_NORMAL: DWRITE_FONT_STRETCH = 5;
pub type DWRITE_FONT_STYLE = i32;
pub const DWRITE_FONT_STYLE_NORMAL: DWRITE_FONT_STYLE = 0;
pub type DWRITE_FONT_WEIGHT = i32;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_HIT_TEST_METRICS {
    pub textPosition: UINT32,
    pub length: UINT32,
    pub left: FLOAT,
    pub top: FLOAT,
    pub width: FLOAT,
    pub height: FLOAT,
    pub bidiLevel: UINT32,
    pub isText: windows_core::BOOL,
    pub isTrimmed: windows_core::BOOL,
}
pub type DWRITE_PARAGRAPH_ALIGNMENT = i32;
pub const DWRITE_PARAGRAPH_ALIGNMENT_CENTER: DWRITE_PARAGRAPH_ALIGNMENT = 2;
pub const DWRITE_PARAGRAPH_ALIGNMENT_FAR: DWRITE_PARAGRAPH_ALIGNMENT = 1;
pub const DWRITE_PARAGRAPH_ALIGNMENT_NEAR: DWRITE_PARAGRAPH_ALIGNMENT = 0;
pub type DWRITE_TEXT_ALIGNMENT = i32;
pub const DWRITE_TEXT_ALIGNMENT_CENTER: DWRITE_TEXT_ALIGNMENT = 2;
pub const DWRITE_TEXT_ALIGNMENT_LEADING: DWRITE_TEXT_ALIGNMENT = 0;
pub const DWRITE_TEXT_ALIGNMENT_TRAILING: DWRITE_TEXT_ALIGNMENT = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DWRITE_TEXT_METRICS {
    pub left: FLOAT,
    pub top: FLOAT,
    pub width: FLOAT,
    pub widthIncludingTrailingWhitespace: FLOAT,
    pub height: FLOAT,
    pub layoutWidth: FLOAT,
    pub layoutHeight: FLOAT,
    pub maxBidiReorderingDepth: UINT32,
    pub lineCount: UINT32,
}
pub type DWRITE_WORD_WRAPPING = i32;
pub const DWRITE_WORD_WRAPPING_NO_WRAP: DWRITE_WORD_WRAPPING = 1;
pub const DWRITE_WORD_WRAPPING_WRAP: DWRITE_WORD_WRAPPING = 0;
pub type FLOAT = f32;
windows_core::imp::define_interface!(
    IDWriteFactory,
    IDWriteFactory_Vtbl,
    0xb859ee5a_d838_4b5b_a2e8_1adc7d93db48
);
windows_core::imp::interface_hierarchy!(IDWriteFactory, windows_core::IUnknown);
impl IDWriteFactory {
    pub(crate) unsafe fn CreateTextFormat<P1>(
        &self,
        fontfamilyname: *const WCHAR,
        fontcollection: P1,
        fontweight: DWRITE_FONT_WEIGHT,
        fontstyle: DWRITE_FONT_STYLE,
        fontstretch: DWRITE_FONT_STRETCH,
        fontsize: FLOAT,
        localename: *const WCHAR,
    ) -> windows_core::Result<IDWriteTextFormat>
    where
        P1: windows_core::Param<IDWriteFontCollection>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTextFormat)(
                windows_core::Interface::as_raw(self),
                fontfamilyname,
                fontcollection.param().abi(),
                fontweight,
                fontstyle,
                fontstretch,
                fontsize,
                localename,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateTextLayout<P2>(
        &self,
        string: *const WCHAR,
        stringlength: UINT32,
        textformat: P2,
        maxwidth: FLOAT,
        maxheight: FLOAT,
    ) -> windows_core::Result<IDWriteTextLayout>
    where
        P2: windows_core::Param<IDWriteTextFormat>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTextLayout)(
                windows_core::Interface::as_raw(self),
                string,
                stringlength,
                textformat.param().abi(),
                maxwidth,
                maxheight,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IDWriteFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetSystemFontCollection: usize,
    CreateCustomFontCollection: usize,
    RegisterFontCollectionLoader: usize,
    UnregisterFontCollectionLoader: usize,
    CreateFontFileReference: usize,
    CreateCustomFontFileReference: usize,
    CreateFontFace: usize,
    CreateRenderingParams: usize,
    CreateMonitorRenderingParams: usize,
    CreateCustomRenderingParams: usize,
    RegisterFontFileLoader: usize,
    UnregisterFontFileLoader: usize,
    pub CreateTextFormat: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const WCHAR,
        *mut core::ffi::c_void,
        DWRITE_FONT_WEIGHT,
        DWRITE_FONT_STYLE,
        DWRITE_FONT_STRETCH,
        FLOAT,
        *const WCHAR,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateTypography: usize,
    GetGdiInterop: usize,
    pub CreateTextLayout: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *const WCHAR,
        UINT32,
        *mut core::ffi::c_void,
        FLOAT,
        FLOAT,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateGdiCompatibleTextLayout: usize,
    CreateEllipsisTrimmingSign: usize,
    CreateTextAnalyzer: usize,
    CreateNumberSubstitution: usize,
    CreateGlyphRunAnalysis: usize,
}
windows_core::imp::define_interface!(
    IDWriteFontCollection,
    IDWriteFontCollection_Vtbl,
    0xa84cee02_3eea_4eee_a827_87c1a02a0fcc
);
windows_core::imp::interface_hierarchy!(IDWriteFontCollection, windows_core::IUnknown);
#[repr(C)]
pub struct IDWriteFontCollection_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    GetFontFamilyCount: usize,
    GetFontFamily: usize,
    FindFamilyName: usize,
    GetFontFromFontFace: usize,
}
windows_core::imp::define_interface!(
    IDWriteTextFormat,
    IDWriteTextFormat_Vtbl,
    0x9c906818_31d7_4fd3_a151_7c5e225db55a
);
windows_core::imp::interface_hierarchy!(IDWriteTextFormat, windows_core::IUnknown);
impl IDWriteTextFormat {
    pub(crate) unsafe fn SetTextAlignment(
        &self,
        textalignment: DWRITE_TEXT_ALIGNMENT,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetTextAlignment)(
                windows_core::Interface::as_raw(self),
                textalignment,
            )
        }
    }
    pub(crate) unsafe fn SetParagraphAlignment(
        &self,
        paragraphalignment: DWRITE_PARAGRAPH_ALIGNMENT,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetParagraphAlignment)(
                windows_core::Interface::as_raw(self),
                paragraphalignment,
            )
        }
    }
    pub(crate) unsafe fn SetWordWrapping(
        &self,
        wordwrapping: DWRITE_WORD_WRAPPING,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetWordWrapping)(
                windows_core::Interface::as_raw(self),
                wordwrapping,
            )
        }
    }
}
#[repr(C)]
pub struct IDWriteTextFormat_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetTextAlignment: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_TEXT_ALIGNMENT,
    ) -> windows_core::HRESULT,
    pub SetParagraphAlignment: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_PARAGRAPH_ALIGNMENT,
    ) -> windows_core::HRESULT,
    pub SetWordWrapping: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        DWRITE_WORD_WRAPPING,
    ) -> windows_core::HRESULT,
    SetReadingDirection: usize,
    SetFlowDirection: usize,
    SetIncrementalTabStop: usize,
    SetTrimming: usize,
    SetLineSpacing: usize,
    GetTextAlignment: usize,
    GetParagraphAlignment: usize,
    GetWordWrapping: usize,
    GetReadingDirection: usize,
    GetFlowDirection: usize,
    GetIncrementalTabStop: usize,
    GetTrimming: usize,
    GetLineSpacing: usize,
    GetFontCollection: usize,
    GetFontFamilyNameLength: usize,
    GetFontFamilyName: usize,
    GetFontWeight: usize,
    GetFontStyle: usize,
    GetFontStretch: usize,
    GetFontSize: usize,
    GetLocaleNameLength: usize,
    GetLocaleName: usize,
}
windows_core::imp::define_interface!(
    IDWriteTextLayout,
    IDWriteTextLayout_Vtbl,
    0x53737037_6d14_410b_9bfe_0b182bb70961
);
impl core::ops::Deref for IDWriteTextLayout {
    type Target = IDWriteTextFormat;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(
    IDWriteTextLayout,
    windows_core::IUnknown,
    IDWriteTextFormat
);
impl IDWriteTextLayout {
    pub(crate) unsafe fn SetMaxWidth(&self, maxwidth: FLOAT) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetMaxWidth)(
                windows_core::Interface::as_raw(self),
                maxwidth,
            )
        }
    }
    pub(crate) unsafe fn SetMaxHeight(&self, maxheight: FLOAT) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).SetMaxHeight)(
                windows_core::Interface::as_raw(self),
                maxheight,
            )
        }
    }
    pub(crate) unsafe fn GetMaxWidth(&self) -> FLOAT {
        unsafe {
            (windows_core::Interface::vtable(self).GetMaxWidth)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub(crate) unsafe fn GetMaxHeight(&self) -> FLOAT {
        unsafe {
            (windows_core::Interface::vtable(self).GetMaxHeight)(windows_core::Interface::as_raw(
                self,
            ))
        }
    }
    pub(crate) unsafe fn GetMetrics(
        &self,
        textmetrics: *mut DWRITE_TEXT_METRICS,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).GetMetrics)(
                windows_core::Interface::as_raw(self),
                textmetrics as _,
            )
        }
    }
    pub(crate) unsafe fn HitTestPoint(
        &self,
        pointx: FLOAT,
        pointy: FLOAT,
        istrailinghit: *mut windows_core::BOOL,
        isinside: *mut windows_core::BOOL,
        hittestmetrics: *mut DWRITE_HIT_TEST_METRICS,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).HitTestPoint)(
                windows_core::Interface::as_raw(self),
                pointx,
                pointy,
                istrailinghit as _,
                isinside as _,
                hittestmetrics as _,
            )
        }
    }
    pub(crate) unsafe fn HitTestTextPosition(
        &self,
        textposition: UINT32,
        istrailinghit: bool,
        pointx: *mut FLOAT,
        pointy: *mut FLOAT,
        hittestmetrics: *mut DWRITE_HIT_TEST_METRICS,
    ) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).HitTestTextPosition)(
                windows_core::Interface::as_raw(self),
                textposition,
                istrailinghit.into(),
                pointx as _,
                pointy as _,
                hittestmetrics as _,
            )
        }
    }
}
#[repr(C)]
pub struct IDWriteTextLayout_Vtbl {
    pub base__: IDWriteTextFormat_Vtbl,
    pub SetMaxWidth:
        unsafe extern "system" fn(*mut core::ffi::c_void, FLOAT) -> windows_core::HRESULT,
    pub SetMaxHeight:
        unsafe extern "system" fn(*mut core::ffi::c_void, FLOAT) -> windows_core::HRESULT,
    SetFontCollection: usize,
    SetFontFamilyName: usize,
    SetFontWeight: usize,
    SetFontStyle: usize,
    SetFontStretch: usize,
    SetFontSize: usize,
    SetUnderline: usize,
    SetStrikethrough: usize,
    SetDrawingEffect: usize,
    SetInlineObject: usize,
    SetTypography: usize,
    SetLocaleName: usize,
    pub GetMaxWidth: unsafe extern "system" fn(*mut core::ffi::c_void) -> FLOAT,
    pub GetMaxHeight: unsafe extern "system" fn(*mut core::ffi::c_void) -> FLOAT,
    GetFontCollection: usize,
    GetFontFamilyNameLength: usize,
    GetFontFamilyName: usize,
    GetFontWeight: usize,
    GetFontStyle: usize,
    GetFontStretch: usize,
    GetFontSize: usize,
    GetUnderline: usize,
    GetStrikethrough: usize,
    GetDrawingEffect: usize,
    GetInlineObject: usize,
    GetTypography: usize,
    GetLocaleNameLength: usize,
    GetLocaleName: usize,
    Draw: usize,
    GetLineMetrics: usize,
    pub GetMetrics: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut DWRITE_TEXT_METRICS,
    ) -> windows_core::HRESULT,
    GetOverhangMetrics: usize,
    GetClusterMetrics: usize,
    DetermineMinWidth: usize,
    pub HitTestPoint: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        FLOAT,
        FLOAT,
        *mut windows_core::BOOL,
        *mut windows_core::BOOL,
        *mut DWRITE_HIT_TEST_METRICS,
    ) -> windows_core::HRESULT,
    pub HitTestTextPosition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        UINT32,
        windows_core::BOOL,
        *mut FLOAT,
        *mut FLOAT,
        *mut DWRITE_HIT_TEST_METRICS,
    ) -> windows_core::HRESULT,
    HitTestTextRange: usize,
}
pub type IID = windows_core::GUID;
pub type UINT32 = u32;
pub type WCHAR = u16;
