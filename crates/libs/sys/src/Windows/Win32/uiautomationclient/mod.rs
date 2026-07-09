pub type AutomationElementMode = i32;
pub const AutomationElementMode_Full: AutomationElementMode = 1;
pub const AutomationElementMode_None: AutomationElementMode = 0;
pub const CUIAutomation: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xff48dba4_60ef_4201_aa87_54103eef594e);
pub const CUIAutomation8: windows_sys::core::GUID = windows_sys::core::GUID::from_u128(0xe22ad333_b25f_460c_83d0_0581107395c9);
pub type CoalesceEventsOptions = i32;
pub const CoalesceEventsOptions_Disabled: CoalesceEventsOptions = 0;
pub const CoalesceEventsOptions_Enabled: CoalesceEventsOptions = 1;
pub type ConnectionRecoveryBehaviorOptions = i32;
pub const ConnectionRecoveryBehaviorOptions_Disabled: ConnectionRecoveryBehaviorOptions = 0;
pub const ConnectionRecoveryBehaviorOptions_Enabled: ConnectionRecoveryBehaviorOptions = 1;
#[repr(C)]
#[derive(Clone, Copy)]
pub struct ExtendedProperty {
    pub PropertyName: windows_sys::core::BSTR,
    pub PropertyValue: windows_sys::core::BSTR,
}
impl Default for ExtendedProperty {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
pub type PropertyConditionFlags = i32;
pub const PropertyConditionFlags_IgnoreCase: PropertyConditionFlags = 1;
pub const PropertyConditionFlags_MatchSubstring: PropertyConditionFlags = 2;
pub const PropertyConditionFlags_None: PropertyConditionFlags = 0;
pub type TreeScope = i32;
pub const TreeScope_Ancestors: TreeScope = 16;
pub const TreeScope_Children: TreeScope = 2;
pub const TreeScope_Descendants: TreeScope = 4;
pub const TreeScope_Element: TreeScope = 1;
pub const TreeScope_None: TreeScope = 0;
pub const TreeScope_Parent: TreeScope = 8;
pub const TreeScope_Subtree: TreeScope = 7;
pub type TreeTraversalOptions = i32;
pub const TreeTraversalOptions_Default: TreeTraversalOptions = 0;
pub const TreeTraversalOptions_LastToFirstOrder: TreeTraversalOptions = 2;
pub const TreeTraversalOptions_PostOrder: TreeTraversalOptions = 1;
pub type UIA_HWND = *mut core::ffi::c_void;
