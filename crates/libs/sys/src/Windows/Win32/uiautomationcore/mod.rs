pub type ActiveEnd = i32;
pub const ActiveEnd_End: ActiveEnd = 2;
pub const ActiveEnd_None: ActiveEnd = 0;
pub const ActiveEnd_Start: ActiveEnd = 1;
pub type AnimationStyle = i32;
pub const AnimationStyle_BlinkingBackground: AnimationStyle = 2;
pub const AnimationStyle_LasVegasLights: AnimationStyle = 1;
pub const AnimationStyle_MarchingBlackAnts: AnimationStyle = 4;
pub const AnimationStyle_MarchingRedAnts: AnimationStyle = 5;
pub const AnimationStyle_None: AnimationStyle = 0;
pub const AnimationStyle_Other: AnimationStyle = -1;
pub const AnimationStyle_Shimmer: AnimationStyle = 6;
pub const AnimationStyle_SparkleText: AnimationStyle = 3;
pub const Assertive: LiveSetting = 2;
pub type BulletStyle = i32;
pub const BulletStyle_DashBullet: BulletStyle = 5;
pub const BulletStyle_FilledRoundBullet: BulletStyle = 2;
pub const BulletStyle_FilledSquareBullet: BulletStyle = 4;
pub const BulletStyle_HollowRoundBullet: BulletStyle = 1;
pub const BulletStyle_HollowSquareBullet: BulletStyle = 3;
pub const BulletStyle_None: BulletStyle = 0;
pub const BulletStyle_Other: BulletStyle = -1;
pub type CONTROLTYPEID = i32;
pub const CUIAutomationClientInfo: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xc2d4f567_8a9b_4c3e_9f1a_2b5c7d8e0f3a);
pub const CUIAutomationClientInfoSource: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xa8d4f123_7b2c_4e5f_9a1b_3c8d6e9f0a2b);
pub const CUIAutomationRegistrar: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0x6e29fabf_9977_42d1_8d0e_ca7e61ad87e6);
pub type CapStyle = i32;
pub const CapStyle_AllCap: CapStyle = 2;
pub const CapStyle_AllPetiteCaps: CapStyle = 3;
pub const CapStyle_None: CapStyle = 0;
pub const CapStyle_Other: CapStyle = -1;
pub const CapStyle_PetiteCaps: CapStyle = 4;
pub const CapStyle_SmallCap: CapStyle = 1;
pub const CapStyle_Titling: CapStyle = 6;
pub const CapStyle_Unicase: CapStyle = 5;
pub type CaretBidiMode = i32;
pub const CaretBidiMode_LTR: CaretBidiMode = 0;
pub const CaretBidiMode_RTL: CaretBidiMode = 1;
pub type CaretPosition = i32;
pub const CaretPosition_BeginningOfLine: CaretPosition = 2;
pub const CaretPosition_EndOfLine: CaretPosition = 1;
pub const CaretPosition_Unknown: CaretPosition = 0;
pub type DockPosition = i32;
pub const DockPosition_Bottom: DockPosition = 2;
pub const DockPosition_Fill: DockPosition = 4;
pub const DockPosition_Left: DockPosition = 1;
pub const DockPosition_None: DockPosition = 5;
pub const DockPosition_Right: DockPosition = 3;
pub const DockPosition_Top: DockPosition = 0;
pub type EVENTID = i32;
pub type ExpandCollapseState = i32;
pub const ExpandCollapseState_Collapsed: ExpandCollapseState = 0;
pub const ExpandCollapseState_Expanded: ExpandCollapseState = 1;
pub const ExpandCollapseState_LeafNode: ExpandCollapseState = 3;
pub const ExpandCollapseState_PartiallyExpanded: ExpandCollapseState = 2;
pub type FillType = i32;
pub const FillType_Color: FillType = 1;
pub const FillType_Gradient: FillType = 2;
pub const FillType_None: FillType = 0;
pub const FillType_Pattern: FillType = 4;
pub const FillType_Picture: FillType = 3;
pub type FlowDirections = i32;
pub const FlowDirections_BottomToTop: FlowDirections = 2;
pub const FlowDirections_Default: FlowDirections = 0;
pub const FlowDirections_RightToLeft: FlowDirections = 1;
pub const FlowDirections_Vertical: FlowDirections = 4;
pub type HEADINGLEVELID = i32;
pub type HorizontalTextAlignment = i32;
pub const HorizontalTextAlignment_Centered: HorizontalTextAlignment = 1;
pub const HorizontalTextAlignment_Justified: HorizontalTextAlignment = 3;
pub const HorizontalTextAlignment_Left: HorizontalTextAlignment = 0;
pub const HorizontalTextAlignment_Right: HorizontalTextAlignment = 2;
pub type LANDMARKTYPEID = i32;
pub type LiveSetting = i32;
pub type METADATAID = i32;
pub type NavigateDirection = i32;
pub const NavigateDirection_FirstChild: NavigateDirection = 3;
pub const NavigateDirection_LastChild: NavigateDirection = 4;
pub const NavigateDirection_NextSibling: NavigateDirection = 1;
pub const NavigateDirection_Parent: NavigateDirection = 0;
pub const NavigateDirection_PreviousSibling: NavigateDirection = 2;
pub type NotificationKind = i32;
pub const NotificationKind_ActionAborted: NotificationKind = 3;
pub const NotificationKind_ActionCompleted: NotificationKind = 2;
pub const NotificationKind_ItemAdded: NotificationKind = 0;
pub const NotificationKind_ItemRemoved: NotificationKind = 1;
pub const NotificationKind_Other: NotificationKind = 4;
pub type NotificationProcessing = i32;
pub const NotificationProcessing_All: NotificationProcessing = 2;
pub const NotificationProcessing_CurrentThenMostRecent: NotificationProcessing = 4;
pub const NotificationProcessing_ImportantAll: NotificationProcessing = 0;
pub const NotificationProcessing_ImportantCurrentThenMostRecent: NotificationProcessing = 5;
pub const NotificationProcessing_ImportantMostRecent: NotificationProcessing = 1;
pub const NotificationProcessing_MostRecent: NotificationProcessing = 3;
pub const Off: LiveSetting = 0;
pub type OrientationType = i32;
pub const OrientationType_Horizontal: OrientationType = 1;
pub const OrientationType_None: OrientationType = 0;
pub const OrientationType_Vertical: OrientationType = 2;
pub type OutlineStyles = i32;
pub const OutlineStyles_Embossed: OutlineStyles = 8;
pub const OutlineStyles_Engraved: OutlineStyles = 4;
pub const OutlineStyles_None: OutlineStyles = 0;
pub const OutlineStyles_Outline: OutlineStyles = 1;
pub const OutlineStyles_Shadow: OutlineStyles = 2;
pub type PATTERNID = i32;
pub type PROPERTYID = i32;
pub const Polite: LiveSetting = 1;
pub type ProviderOptions = u32;
pub const ProviderOptions_ClientSideProvider: ProviderOptions = 1;
pub const ProviderOptions_HasNativeIAccessible: ProviderOptions = 128;
pub const ProviderOptions_NonClientAreaProvider: ProviderOptions = 4;
pub const ProviderOptions_OverrideProvider: ProviderOptions = 8;
pub const ProviderOptions_ProviderOwnsSetFocus: ProviderOptions = 16;
pub const ProviderOptions_RefuseNonClientSupport: ProviderOptions = 64;
pub const ProviderOptions_ServerSideProvider: ProviderOptions = 2;
pub const ProviderOptions_UseClientCoordinates: ProviderOptions = 256;
pub const ProviderOptions_UseComThreading: ProviderOptions = 32;
pub type RowOrColumnMajor = i32;
pub const RowOrColumnMajor_ColumnMajor: RowOrColumnMajor = 1;
pub const RowOrColumnMajor_Indeterminate: RowOrColumnMajor = 2;
pub const RowOrColumnMajor_RowMajor: RowOrColumnMajor = 0;
pub type SayAsInterpretAs = i32;
pub const SayAsInterpretAs_Address: SayAsInterpretAs = 11;
pub const SayAsInterpretAs_Alphanumeric: SayAsInterpretAs = 12;
pub const SayAsInterpretAs_Cardinal: SayAsInterpretAs = 2;
pub const SayAsInterpretAs_Currency: SayAsInterpretAs = 8;
pub const SayAsInterpretAs_Date: SayAsInterpretAs = 5;
pub const SayAsInterpretAs_Date_DayMonth: SayAsInterpretAs = 20;
pub const SayAsInterpretAs_Date_DayMonthYear: SayAsInterpretAs = 16;
pub const SayAsInterpretAs_Date_MonthDay: SayAsInterpretAs = 21;
pub const SayAsInterpretAs_Date_MonthDayYear: SayAsInterpretAs = 15;
pub const SayAsInterpretAs_Date_MonthYear: SayAsInterpretAs = 19;
pub const SayAsInterpretAs_Date_Year: SayAsInterpretAs = 22;
pub const SayAsInterpretAs_Date_YearMonth: SayAsInterpretAs = 18;
pub const SayAsInterpretAs_Date_YearMonthDay: SayAsInterpretAs = 17;
pub const SayAsInterpretAs_Media: SayAsInterpretAs = 14;
pub const SayAsInterpretAs_Name: SayAsInterpretAs = 13;
pub const SayAsInterpretAs_Net: SayAsInterpretAs = 9;
pub const SayAsInterpretAs_None: SayAsInterpretAs = 0;
pub const SayAsInterpretAs_Number: SayAsInterpretAs = 4;
pub const SayAsInterpretAs_Ordinal: SayAsInterpretAs = 3;
pub const SayAsInterpretAs_Spell: SayAsInterpretAs = 1;
pub const SayAsInterpretAs_Telephone: SayAsInterpretAs = 7;
pub const SayAsInterpretAs_Time: SayAsInterpretAs = 6;
pub const SayAsInterpretAs_Time_HoursMinutes12: SayAsInterpretAs = 24;
pub const SayAsInterpretAs_Time_HoursMinutes24: SayAsInterpretAs = 26;
pub const SayAsInterpretAs_Time_HoursMinutesSeconds12: SayAsInterpretAs = 23;
pub const SayAsInterpretAs_Time_HoursMinutesSeconds24: SayAsInterpretAs = 25;
pub const SayAsInterpretAs_Url: SayAsInterpretAs = 10;
pub type ScrollAmount = i32;
pub const ScrollAmount_LargeDecrement: ScrollAmount = 0;
pub const ScrollAmount_LargeIncrement: ScrollAmount = 3;
pub const ScrollAmount_NoAmount: ScrollAmount = 2;
pub const ScrollAmount_SmallDecrement: ScrollAmount = 1;
pub const ScrollAmount_SmallIncrement: ScrollAmount = 4;
pub type StructureChangeType = i32;
pub const StructureChangeType_ChildAdded: StructureChangeType = 0;
pub const StructureChangeType_ChildRemoved: StructureChangeType = 1;
pub const StructureChangeType_ChildrenBulkAdded: StructureChangeType = 3;
pub const StructureChangeType_ChildrenBulkRemoved: StructureChangeType = 4;
pub const StructureChangeType_ChildrenInvalidated: StructureChangeType = 2;
pub const StructureChangeType_ChildrenReordered: StructureChangeType = 5;
pub type SupportedTextSelection = i32;
pub const SupportedTextSelection_Multiple: SupportedTextSelection = 2;
pub const SupportedTextSelection_None: SupportedTextSelection = 0;
pub const SupportedTextSelection_Single: SupportedTextSelection = 1;
pub type SynchronizedInputType = u32;
pub const SynchronizedInputType_KeyDown: SynchronizedInputType = 2;
pub const SynchronizedInputType_KeyUp: SynchronizedInputType = 1;
pub const SynchronizedInputType_LeftMouseDown: SynchronizedInputType = 8;
pub const SynchronizedInputType_LeftMouseUp: SynchronizedInputType = 4;
pub const SynchronizedInputType_RightMouseDown: SynchronizedInputType = 32;
pub const SynchronizedInputType_RightMouseUp: SynchronizedInputType = 16;
pub type TEXTATTRIBUTEID = i32;
pub type TextDecorationLineStyle = i32;
pub const TextDecorationLineStyle_Dash: TextDecorationLineStyle = 5;
pub const TextDecorationLineStyle_DashDot: TextDecorationLineStyle = 6;
pub const TextDecorationLineStyle_DashDotDot: TextDecorationLineStyle = 7;
pub const TextDecorationLineStyle_Dot: TextDecorationLineStyle = 4;
pub const TextDecorationLineStyle_Double: TextDecorationLineStyle = 3;
pub const TextDecorationLineStyle_DoubleWavy: TextDecorationLineStyle = 11;
pub const TextDecorationLineStyle_LongDash: TextDecorationLineStyle = 13;
pub const TextDecorationLineStyle_None: TextDecorationLineStyle = 0;
pub const TextDecorationLineStyle_Other: TextDecorationLineStyle = -1;
pub const TextDecorationLineStyle_Single: TextDecorationLineStyle = 1;
pub const TextDecorationLineStyle_ThickDash: TextDecorationLineStyle = 14;
pub const TextDecorationLineStyle_ThickDashDot: TextDecorationLineStyle = 15;
pub const TextDecorationLineStyle_ThickDashDotDot: TextDecorationLineStyle = 16;
pub const TextDecorationLineStyle_ThickDot: TextDecorationLineStyle = 17;
pub const TextDecorationLineStyle_ThickLongDash: TextDecorationLineStyle = 18;
pub const TextDecorationLineStyle_ThickSingle: TextDecorationLineStyle = 9;
pub const TextDecorationLineStyle_ThickWavy: TextDecorationLineStyle = 12;
pub const TextDecorationLineStyle_Wavy: TextDecorationLineStyle = 8;
pub const TextDecorationLineStyle_WordsOnly: TextDecorationLineStyle = 2;
pub type TextEditChangeType = i32;
pub const TextEditChangeType_AutoComplete: TextEditChangeType = 4;
pub const TextEditChangeType_AutoCorrect: TextEditChangeType = 1;
pub const TextEditChangeType_Composition: TextEditChangeType = 2;
pub const TextEditChangeType_CompositionFinalized: TextEditChangeType = 3;
pub const TextEditChangeType_None: TextEditChangeType = 0;
pub type TextPatternRangeEndpoint = i32;
pub const TextPatternRangeEndpoint_End: TextPatternRangeEndpoint = 1;
pub const TextPatternRangeEndpoint_Start: TextPatternRangeEndpoint = 0;
pub type TextUnit = i32;
pub const TextUnit_Character: TextUnit = 0;
pub const TextUnit_Document: TextUnit = 6;
pub const TextUnit_Format: TextUnit = 1;
pub const TextUnit_Line: TextUnit = 3;
pub const TextUnit_Page: TextUnit = 5;
pub const TextUnit_Paragraph: TextUnit = 4;
pub const TextUnit_Word: TextUnit = 2;
pub type ToggleState = i32;
pub const ToggleState_Indeterminate: ToggleState = 2;
pub const ToggleState_Off: ToggleState = 0;
pub const ToggleState_On: ToggleState = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UIAutomationEventInfo {
    pub guid: windows_sys::core::GUID,
    pub pProgrammaticName: windows_sys::core::PCWSTR,
}
impl Default for UIAutomationEventInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UIAutomationMethodInfo {
    pub pProgrammaticName: windows_sys::core::PCWSTR,
    pub doSetFocus: windows_sys::core::BOOL,
    pub cInParameters: u32,
    pub cOutParameters: u32,
    pub pParameterTypes: *mut UIAutomationType,
    pub pParameterNames: *mut windows_sys::core::PCWSTR,
}
impl Default for UIAutomationMethodInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UIAutomationParameter {
    pub r#type: UIAutomationType,
    pub pData: *mut core::ffi::c_void,
}
impl Default for UIAutomationParameter {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UIAutomationPatternInfo {
    pub guid: windows_sys::core::GUID,
    pub pProgrammaticName: windows_sys::core::PCWSTR,
    pub providerInterfaceId: windows_sys::core::GUID,
    pub clientInterfaceId: windows_sys::core::GUID,
    pub cProperties: u32,
    pub pProperties: *mut UIAutomationPropertyInfo,
    pub cMethods: u32,
    pub pMethods: *mut UIAutomationMethodInfo,
    pub cEvents: u32,
    pub pEvents: *mut UIAutomationEventInfo,
    pub pPatternHandler: *mut core::ffi::c_void,
}
impl Default for UIAutomationPatternInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy)]
pub struct UIAutomationPropertyInfo {
    pub guid: windows_sys::core::GUID,
    pub pProgrammaticName: windows_sys::core::PCWSTR,
    pub r#type: UIAutomationType,
}
impl Default for UIAutomationPropertyInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type UIAutomationType = u32;
pub const UIAutomationType_Array: UIAutomationType = 65536;
pub const UIAutomationType_Bool: UIAutomationType = 2;
pub const UIAutomationType_BoolArray: UIAutomationType = 65538;
pub const UIAutomationType_Double: UIAutomationType = 4;
pub const UIAutomationType_DoubleArray: UIAutomationType = 65540;
pub const UIAutomationType_Element: UIAutomationType = 7;
pub const UIAutomationType_ElementArray: UIAutomationType = 65543;
pub const UIAutomationType_Int: UIAutomationType = 1;
pub const UIAutomationType_IntArray: UIAutomationType = 65537;
pub const UIAutomationType_Out: UIAutomationType = 131072;
pub const UIAutomationType_OutBool: UIAutomationType = 131074;
pub const UIAutomationType_OutBoolArray: UIAutomationType = 196610;
pub const UIAutomationType_OutDouble: UIAutomationType = 131076;
pub const UIAutomationType_OutDoubleArray: UIAutomationType = 196612;
pub const UIAutomationType_OutElement: UIAutomationType = 131079;
pub const UIAutomationType_OutElementArray: UIAutomationType = 196615;
pub const UIAutomationType_OutInt: UIAutomationType = 131073;
pub const UIAutomationType_OutIntArray: UIAutomationType = 196609;
pub const UIAutomationType_OutPoint: UIAutomationType = 131077;
pub const UIAutomationType_OutPointArray: UIAutomationType = 196613;
pub const UIAutomationType_OutRect: UIAutomationType = 131078;
pub const UIAutomationType_OutRectArray: UIAutomationType = 196614;
pub const UIAutomationType_OutString: UIAutomationType = 131075;
pub const UIAutomationType_OutStringArray: UIAutomationType = 196611;
pub const UIAutomationType_Point: UIAutomationType = 5;
pub const UIAutomationType_PointArray: UIAutomationType = 65541;
pub const UIAutomationType_Rect: UIAutomationType = 6;
pub const UIAutomationType_RectArray: UIAutomationType = 65542;
pub const UIAutomationType_String: UIAutomationType = 3;
pub const UIAutomationType_StringArray: UIAutomationType = 65539;
#[repr(C)]
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy)]
pub struct UiaChangeInfo {
    pub uiaId: i32,
    pub payload: super::oaidl::VARIANT,
    pub extraInfo: super::oaidl::VARIANT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Default for UiaChangeInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct UiaPoint {
    pub x: f64,
    pub y: f64,
}
#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct UiaRect {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}
pub type VisualEffects = i32;
pub const VisualEffects_Bevel: VisualEffects = 16;
pub const VisualEffects_Glow: VisualEffects = 4;
pub const VisualEffects_None: VisualEffects = 0;
pub const VisualEffects_Reflection: VisualEffects = 2;
pub const VisualEffects_Shadow: VisualEffects = 1;
pub const VisualEffects_SoftEdges: VisualEffects = 8;
pub type WindowInteractionState = i32;
pub const WindowInteractionState_BlockedByModalWindow: WindowInteractionState = 3;
pub const WindowInteractionState_Closing: WindowInteractionState = 1;
pub const WindowInteractionState_NotResponding: WindowInteractionState = 4;
pub const WindowInteractionState_ReadyForUserInteraction: WindowInteractionState = 2;
pub const WindowInteractionState_Running: WindowInteractionState = 0;
pub type WindowVisualState = i32;
pub const WindowVisualState_Maximized: WindowVisualState = 1;
pub const WindowVisualState_Minimized: WindowVisualState = 2;
pub const WindowVisualState_Normal: WindowVisualState = 0;
pub type ZoomUnit = i32;
pub const ZoomUnit_LargeDecrement: ZoomUnit = 1;
pub const ZoomUnit_LargeIncrement: ZoomUnit = 3;
pub const ZoomUnit_NoAmount: ZoomUnit = 0;
pub const ZoomUnit_SmallDecrement: ZoomUnit = 2;
pub const ZoomUnit_SmallIncrement: ZoomUnit = 4;
