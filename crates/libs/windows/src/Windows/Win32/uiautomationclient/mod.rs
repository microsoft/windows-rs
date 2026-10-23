pub const AnnotationType_AdvancedProofingIssue: i32 = 60020;
pub const AnnotationType_Author: i32 = 60019;
pub const AnnotationType_CircularReferenceError: i32 = 60022;
pub const AnnotationType_Comment: i32 = 60003;
pub const AnnotationType_ConflictingChange: i32 = 60018;
pub const AnnotationType_DataValidationError: i32 = 60021;
pub const AnnotationType_DeletionChange: i32 = 60012;
pub const AnnotationType_EditingLockedChange: i32 = 60016;
pub const AnnotationType_Endnote: i32 = 60009;
pub const AnnotationType_ExternalChange: i32 = 60017;
pub const AnnotationType_Footer: i32 = 60007;
pub const AnnotationType_Footnote: i32 = 60010;
pub const AnnotationType_FormatChange: i32 = 60014;
pub const AnnotationType_FormulaError: i32 = 60004;
pub const AnnotationType_GrammarError: i32 = 60002;
pub const AnnotationType_Header: i32 = 60006;
pub const AnnotationType_Highlighted: i32 = 60008;
pub const AnnotationType_InsertionChange: i32 = 60011;
pub const AnnotationType_Mathematics: i32 = 60023;
pub const AnnotationType_MoveChange: i32 = 60013;
pub const AnnotationType_Sensitive: i32 = 60024;
pub const AnnotationType_SpellingError: i32 = 60001;
pub const AnnotationType_TrackChanges: i32 = 60005;
pub const AnnotationType_Unknown: i32 = 60000;
pub const AnnotationType_UnsyncedChange: i32 = 60015;
pub type AutomationElementMode = i32;
pub const AutomationElementMode_Full: AutomationElementMode = 1;
pub const AutomationElementMode_None: AutomationElementMode = 0;
pub const CUIAutomation: windows_core::GUID = windows_core::GUID::from_u128(0xff48dba4_60ef_4201_aa87_54103eef594e);
pub const CUIAutomation8: windows_core::GUID = windows_core::GUID::from_u128(0xe22ad333_b25f_460c_83d0_0581107395c9);
pub type CoalesceEventsOptions = i32;
pub const CoalesceEventsOptions_Disabled: CoalesceEventsOptions = 0;
pub const CoalesceEventsOptions_Enabled: CoalesceEventsOptions = 1;
pub type ConnectionRecoveryBehaviorOptions = i32;
pub const ConnectionRecoveryBehaviorOptions_Disabled: ConnectionRecoveryBehaviorOptions = 0;
pub const ConnectionRecoveryBehaviorOptions_Enabled: ConnectionRecoveryBehaviorOptions = 1;
#[repr(C)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ExtendedProperty {
    pub PropertyName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub PropertyValue: core::mem::ManuallyDrop<windows_core::BSTR>,
}
pub const HeadingLevel1: i32 = 80051;
pub const HeadingLevel2: i32 = 80052;
pub const HeadingLevel3: i32 = 80053;
pub const HeadingLevel4: i32 = 80054;
pub const HeadingLevel5: i32 = 80055;
pub const HeadingLevel6: i32 = 80056;
pub const HeadingLevel7: i32 = 80057;
pub const HeadingLevel8: i32 = 80058;
pub const HeadingLevel9: i32 = 80059;
pub const HeadingLevel_None: i32 = 80050;
windows_core::imp::define_interface!(IUIAutomation, IUIAutomation_Vtbl, 0x30cbe57d_d9d0_452a_ab13_7ac5ac4825ee);
windows_core::imp::interface_hierarchy!(IUIAutomation, windows_core::IUnknown);
impl IUIAutomation {
    pub unsafe fn CompareElements<P0, P1>(&self, el1: P0, el2: P1) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CompareElements)(windows_core::Interface::as_raw(self), el1.param().abi(), el2.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn CompareRuntimeIds(&self, runtimeid1: *const super::SAFEARRAY, runtimeid2: *const super::SAFEARRAY) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CompareRuntimeIds)(windows_core::Interface::as_raw(self), runtimeid1, runtimeid2, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRootElement(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRootElement)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn ElementFromHandle(&self, hwnd: UIA_HWND) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElementFromHandle)(windows_core::Interface::as_raw(self), hwnd, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn ElementFromPoint(&self, pt: super::POINT) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElementFromPoint)(windows_core::Interface::as_raw(self), pt, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFocusedElement(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFocusedElement)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRootElementBuildCache<P0>(&self, cacherequest: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRootElementBuildCache)(windows_core::Interface::as_raw(self), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn ElementFromHandleBuildCache<P1>(&self, hwnd: UIA_HWND, cacherequest: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElementFromHandleBuildCache)(windows_core::Interface::as_raw(self), hwnd, cacherequest.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn ElementFromPointBuildCache<P1>(&self, pt: super::POINT, cacherequest: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElementFromPointBuildCache)(windows_core::Interface::as_raw(self), pt, cacherequest.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFocusedElementBuildCache<P0>(&self, cacherequest: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFocusedElementBuildCache)(windows_core::Interface::as_raw(self), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTreeWalker<P0>(&self, pcondition: P0) -> windows_core::Result<IUIAutomationTreeWalker>
    where
        P0: windows_core::Param<IUIAutomationCondition>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTreeWalker)(windows_core::Interface::as_raw(self), pcondition.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_ControlViewWalker(&self) -> windows_core::Result<IUIAutomationTreeWalker> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ControlViewWalker)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_ContentViewWalker(&self) -> windows_core::Result<IUIAutomationTreeWalker> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ContentViewWalker)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_RawViewWalker(&self) -> windows_core::Result<IUIAutomationTreeWalker> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RawViewWalker)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_RawViewCondition(&self) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RawViewCondition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_ControlViewCondition(&self) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ControlViewCondition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_ContentViewCondition(&self) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ContentViewCondition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateCacheRequest(&self) -> windows_core::Result<IUIAutomationCacheRequest> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCacheRequest)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTrueCondition(&self) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTrueCondition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateFalseCondition(&self) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFalseCondition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn CreatePropertyCondition(&self, propertyid: super::PROPERTYID, value: &super::VARIANT) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePropertyCondition)(windows_core::Interface::as_raw(self), propertyid, core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn CreatePropertyConditionEx(&self, propertyid: super::PROPERTYID, value: &super::VARIANT, flags: PropertyConditionFlags) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePropertyConditionEx)(windows_core::Interface::as_raw(self), propertyid, core::mem::transmute_copy(value), flags, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateAndCondition<P0, P1>(&self, condition1: P0, condition2: P1) -> windows_core::Result<IUIAutomationCondition>
    where
        P0: windows_core::Param<IUIAutomationCondition>,
        P1: windows_core::Param<IUIAutomationCondition>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAndCondition)(windows_core::Interface::as_raw(self), condition1.param().abi(), condition2.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn CreateAndConditionFromArray(&self, conditions: Option<*const super::SAFEARRAY>) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAndConditionFromArray)(windows_core::Interface::as_raw(self), conditions.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateAndConditionFromNativeArray(&self, conditions: &[Option<IUIAutomationCondition>]) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAndConditionFromNativeArray)(windows_core::Interface::as_raw(self), core::mem::transmute(conditions.as_ptr()), conditions.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateOrCondition<P0, P1>(&self, condition1: P0, condition2: P1) -> windows_core::Result<IUIAutomationCondition>
    where
        P0: windows_core::Param<IUIAutomationCondition>,
        P1: windows_core::Param<IUIAutomationCondition>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateOrCondition)(windows_core::Interface::as_raw(self), condition1.param().abi(), condition2.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn CreateOrConditionFromArray(&self, conditions: Option<*const super::SAFEARRAY>) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateOrConditionFromArray)(windows_core::Interface::as_raw(self), conditions.unwrap_or(core::mem::zeroed()) as _, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateOrConditionFromNativeArray(&self, conditions: &[Option<IUIAutomationCondition>]) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateOrConditionFromNativeArray)(windows_core::Interface::as_raw(self), core::mem::transmute(conditions.as_ptr()), conditions.len().try_into().unwrap(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateNotCondition<P0>(&self, condition: P0) -> windows_core::Result<IUIAutomationCondition>
    where
        P0: windows_core::Param<IUIAutomationCondition>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateNotCondition)(windows_core::Interface::as_raw(self), condition.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn AddAutomationEventHandler<P1, P3, P4>(&self, eventid: super::EVENTID, element: P1, scope: TreeScope, cacherequest: P3, handler: P4) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IUIAutomationElement>,
        P3: windows_core::Param<IUIAutomationCacheRequest>,
        P4: windows_core::Param<IUIAutomationEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddAutomationEventHandler)(windows_core::Interface::as_raw(self), eventid, element.param().abi(), scope, cacherequest.param().abi(), handler.param().abi()) }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn RemoveAutomationEventHandler<P1, P2>(&self, eventid: super::EVENTID, element: P1, handler: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IUIAutomationElement>,
        P2: windows_core::Param<IUIAutomationEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveAutomationEventHandler)(windows_core::Interface::as_raw(self), eventid, element.param().abi(), handler.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddPropertyChangedEventHandlerNativeArray<P0, P2, P3>(&self, element: P0, scope: TreeScope, cacherequest: P2, handler: P3, propertyarray: &[super::PROPERTYID]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P2: windows_core::Param<IUIAutomationCacheRequest>,
        P3: windows_core::Param<IUIAutomationPropertyChangedEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddPropertyChangedEventHandlerNativeArray)(windows_core::Interface::as_raw(self), element.param().abi(), scope, cacherequest.param().abi(), handler.param().abi(), propertyarray.as_ptr(), propertyarray.len().try_into().unwrap()) }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn AddPropertyChangedEventHandler<P0, P2, P3>(&self, element: P0, scope: TreeScope, cacherequest: P2, handler: P3, propertyarray: *const super::SAFEARRAY) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P2: windows_core::Param<IUIAutomationCacheRequest>,
        P3: windows_core::Param<IUIAutomationPropertyChangedEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddPropertyChangedEventHandler)(windows_core::Interface::as_raw(self), element.param().abi(), scope, cacherequest.param().abi(), handler.param().abi(), propertyarray) }
    }
    pub unsafe fn RemovePropertyChangedEventHandler<P0, P1>(&self, element: P0, handler: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationPropertyChangedEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemovePropertyChangedEventHandler)(windows_core::Interface::as_raw(self), element.param().abi(), handler.param().abi()) }
    }
    pub unsafe fn AddStructureChangedEventHandler<P0, P2, P3>(&self, element: P0, scope: TreeScope, cacherequest: P2, handler: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P2: windows_core::Param<IUIAutomationCacheRequest>,
        P3: windows_core::Param<IUIAutomationStructureChangedEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddStructureChangedEventHandler)(windows_core::Interface::as_raw(self), element.param().abi(), scope, cacherequest.param().abi(), handler.param().abi()) }
    }
    pub unsafe fn RemoveStructureChangedEventHandler<P0, P1>(&self, element: P0, handler: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationStructureChangedEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveStructureChangedEventHandler)(windows_core::Interface::as_raw(self), element.param().abi(), handler.param().abi()) }
    }
    pub unsafe fn AddFocusChangedEventHandler<P0, P1>(&self, cacherequest: P0, handler: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
        P1: windows_core::Param<IUIAutomationFocusChangedEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddFocusChangedEventHandler)(windows_core::Interface::as_raw(self), cacherequest.param().abi(), handler.param().abi()) }
    }
    pub unsafe fn RemoveFocusChangedEventHandler<P0>(&self, handler: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationFocusChangedEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveFocusChangedEventHandler)(windows_core::Interface::as_raw(self), handler.param().abi()) }
    }
    pub unsafe fn RemoveAllEventHandlers(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveAllEventHandlers)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn IntNativeArrayToSafeArray(&self, array: &[i32]) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IntNativeArrayToSafeArray)(windows_core::Interface::as_raw(self), array.as_ptr(), array.len().try_into().unwrap(), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn IntSafeArrayToNativeArray(&self, intarray: *const super::SAFEARRAY, array: *mut *mut i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IntSafeArrayToNativeArray)(windows_core::Interface::as_raw(self), intarray, array as _, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn RectToVariant(&self, rc: super::RECT) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RectToVariant)(windows_core::Interface::as_raw(self), rc, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn VariantToRect(&self, var: &super::VARIANT) -> windows_core::Result<super::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VariantToRect)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(var), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt"))]
    pub unsafe fn SafeArrayToRectNativeArray(&self, rects: *const super::SAFEARRAY, rectarray: *mut *mut super::RECT) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SafeArrayToRectNativeArray)(windows_core::Interface::as_raw(self), rects, rectarray as _, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CreateProxyFactoryEntry<P0>(&self, factory: P0) -> windows_core::Result<IUIAutomationProxyFactoryEntry>
    where
        P0: windows_core::Param<IUIAutomationProxyFactory>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateProxyFactoryEntry)(windows_core::Interface::as_raw(self), factory.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_ProxyFactoryMapping(&self) -> windows_core::Result<IUIAutomationProxyFactoryMapping> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ProxyFactoryMapping)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetPropertyProgrammaticName(&self, property: super::PROPERTYID) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyProgrammaticName)(windows_core::Interface::as_raw(self), property, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn GetPatternProgrammaticName(&self, pattern: super::PATTERNID) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPatternProgrammaticName)(windows_core::Interface::as_raw(self), pattern, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn PollForPotentialSupportedPatterns<P0>(&self, pelement: P0, patternids: *mut *mut super::SAFEARRAY, patternnames: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe { (windows_core::Interface::vtable(self).PollForPotentialSupportedPatterns)(windows_core::Interface::as_raw(self), pelement.param().abi(), patternids as _, patternnames as _) }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn PollForPotentialSupportedProperties<P0>(&self, pelement: P0, propertyids: *mut *mut super::SAFEARRAY, propertynames: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe { (windows_core::Interface::vtable(self).PollForPotentialSupportedProperties)(windows_core::Interface::as_raw(self), pelement.param().abi(), propertyids as _, propertynames as _) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn CheckNotSupported(&self, value: &super::VARIANT) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckNotSupported)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ReservedNotSupportedValue(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ReservedNotSupportedValue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_ReservedMixedAttributeValue(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ReservedMixedAttributeValue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "oleacc"))]
    pub unsafe fn ElementFromIAccessible<P0>(&self, accessible: P0, childid: i32) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<super::IAccessible>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElementFromIAccessible)(windows_core::Interface::as_raw(self), accessible.param().abi(), childid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "oleacc"))]
    pub unsafe fn ElementFromIAccessibleBuildCache<P0, P2>(&self, accessible: P0, childid: i32, cacherequest: P2) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<super::IAccessible>,
        P2: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElementFromIAccessibleBuildCache)(windows_core::Interface::as_raw(self), accessible.param().abi(), childid, cacherequest.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CompareElements: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub CompareRuntimeIds: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY, *const super::SAFEARRAY, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    CompareRuntimeIds: usize,
    pub GetRootElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ElementFromHandle: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_HWND, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub ElementFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, super::POINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    ElementFromPoint: usize,
    pub GetFocusedElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRootElementBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ElementFromHandleBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_HWND, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub ElementFromPointBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, super::POINT, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    ElementFromPointBuildCache: usize,
    pub GetFocusedElementBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTreeWalker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ControlViewWalker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ContentViewWalker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_RawViewWalker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_RawViewCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ControlViewCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ContentViewCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCacheRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTrueCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFalseCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub CreatePropertyCondition: unsafe extern "system" fn(*mut core::ffi::c_void, super::PROPERTYID, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    CreatePropertyCondition: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub CreatePropertyConditionEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::PROPERTYID, super::VARIANT, PropertyConditionFlags, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    CreatePropertyConditionEx: usize,
    pub CreateAndCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub CreateAndConditionFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    CreateAndConditionFromArray: usize,
    pub CreateAndConditionFromNativeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateOrCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub CreateOrConditionFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    CreateOrConditionFromArray: usize,
    pub CreateOrConditionFromNativeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateNotCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub AddAutomationEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, super::EVENTID, *mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    AddAutomationEventHandler: usize,
    #[cfg(feature = "uiautomationcore")]
    pub RemoveAutomationEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, super::EVENTID, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    RemoveAutomationEventHandler: usize,
    #[cfg(feature = "shared")]
    pub AddPropertyChangedEventHandlerNativeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::PROPERTYID, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddPropertyChangedEventHandlerNativeArray: usize,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub AddPropertyChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    AddPropertyChangedEventHandler: usize,
    pub RemovePropertyChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddStructureChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveStructureChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddFocusChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveFocusChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAllEventHandlers: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub IntNativeArrayToSafeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const i32, i32, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    IntNativeArrayToSafeArray: usize,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub IntSafeArrayToNativeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY, *mut *mut i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    IntSafeArrayToNativeArray: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub RectToVariant: unsafe extern "system" fn(*mut core::ffi::c_void, super::RECT, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    RectToVariant: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub VariantToRect: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut super::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    VariantToRect: usize,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt"))]
    pub SafeArrayToRectNativeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY, *mut *mut super::RECT, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt")))]
    SafeArrayToRectNativeArray: usize,
    pub CreateProxyFactoryEntry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ProxyFactoryMapping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub GetPropertyProgrammaticName: unsafe extern "system" fn(*mut core::ffi::c_void, super::PROPERTYID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetPropertyProgrammaticName: usize,
    #[cfg(feature = "uiautomationcore")]
    pub GetPatternProgrammaticName: unsafe extern "system" fn(*mut core::ffi::c_void, super::PATTERNID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    GetPatternProgrammaticName: usize,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub PollForPotentialSupportedPatterns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut super::SAFEARRAY, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    PollForPotentialSupportedPatterns: usize,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub PollForPotentialSupportedProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut super::SAFEARRAY, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    PollForPotentialSupportedProperties: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub CheckNotSupported: unsafe extern "system" fn(*mut core::ffi::c_void, super::VARIANT, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    CheckNotSupported: usize,
    pub get_ReservedNotSupportedValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ReservedMixedAttributeValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "oleacc"))]
    pub ElementFromIAccessible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "oleacc")))]
    ElementFromIAccessible: usize,
    #[cfg(all(feature = "oaidl", feature = "oleacc"))]
    pub ElementFromIAccessibleBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "oleacc")))]
    ElementFromIAccessibleBuildCache: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "oleacc", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomation_Impl: windows_core::IUnknownImpl {
    fn CompareElements(&self, el1: windows_core::Ref<IUIAutomationElement>, el2: windows_core::Ref<IUIAutomationElement>) -> windows_core::Result<windows_core::BOOL>;
    fn CompareRuntimeIds(&self, runtimeid1: *const super::SAFEARRAY, runtimeid2: *const super::SAFEARRAY) -> windows_core::Result<windows_core::BOOL>;
    fn GetRootElement(&self) -> windows_core::Result<IUIAutomationElement>;
    fn ElementFromHandle(&self, hwnd: UIA_HWND) -> windows_core::Result<IUIAutomationElement>;
    fn ElementFromPoint(&self, pt: &super::POINT) -> windows_core::Result<IUIAutomationElement>;
    fn GetFocusedElement(&self) -> windows_core::Result<IUIAutomationElement>;
    fn GetRootElementBuildCache(&self, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>) -> windows_core::Result<IUIAutomationElement>;
    fn ElementFromHandleBuildCache(&self, hwnd: UIA_HWND, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>) -> windows_core::Result<IUIAutomationElement>;
    fn ElementFromPointBuildCache(&self, pt: &super::POINT, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>) -> windows_core::Result<IUIAutomationElement>;
    fn GetFocusedElementBuildCache(&self, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>) -> windows_core::Result<IUIAutomationElement>;
    fn CreateTreeWalker(&self, pcondition: windows_core::Ref<IUIAutomationCondition>) -> windows_core::Result<IUIAutomationTreeWalker>;
    fn get_ControlViewWalker(&self) -> windows_core::Result<IUIAutomationTreeWalker>;
    fn get_ContentViewWalker(&self) -> windows_core::Result<IUIAutomationTreeWalker>;
    fn get_RawViewWalker(&self) -> windows_core::Result<IUIAutomationTreeWalker>;
    fn get_RawViewCondition(&self) -> windows_core::Result<IUIAutomationCondition>;
    fn get_ControlViewCondition(&self) -> windows_core::Result<IUIAutomationCondition>;
    fn get_ContentViewCondition(&self) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateCacheRequest(&self) -> windows_core::Result<IUIAutomationCacheRequest>;
    fn CreateTrueCondition(&self) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateFalseCondition(&self) -> windows_core::Result<IUIAutomationCondition>;
    fn CreatePropertyCondition(&self, propertyid: super::PROPERTYID, value: &super::VARIANT) -> windows_core::Result<IUIAutomationCondition>;
    fn CreatePropertyConditionEx(&self, propertyid: super::PROPERTYID, value: &super::VARIANT, flags: PropertyConditionFlags) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateAndCondition(&self, condition1: windows_core::Ref<IUIAutomationCondition>, condition2: windows_core::Ref<IUIAutomationCondition>) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateAndConditionFromArray(&self, conditions: *const super::SAFEARRAY) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateAndConditionFromNativeArray(&self, conditions: *const Option<IUIAutomationCondition>, conditioncount: i32) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateOrCondition(&self, condition1: windows_core::Ref<IUIAutomationCondition>, condition2: windows_core::Ref<IUIAutomationCondition>) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateOrConditionFromArray(&self, conditions: *const super::SAFEARRAY) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateOrConditionFromNativeArray(&self, conditions: *const Option<IUIAutomationCondition>, conditioncount: i32) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateNotCondition(&self, condition: windows_core::Ref<IUIAutomationCondition>) -> windows_core::Result<IUIAutomationCondition>;
    fn AddAutomationEventHandler(&self, eventid: super::EVENTID, element: windows_core::Ref<IUIAutomationElement>, scope: TreeScope, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationEventHandler>) -> windows_core::Result<()>;
    fn RemoveAutomationEventHandler(&self, eventid: super::EVENTID, element: windows_core::Ref<IUIAutomationElement>, handler: windows_core::Ref<IUIAutomationEventHandler>) -> windows_core::Result<()>;
    fn AddPropertyChangedEventHandlerNativeArray(&self, element: windows_core::Ref<IUIAutomationElement>, scope: TreeScope, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationPropertyChangedEventHandler>, propertyarray: *const super::PROPERTYID, propertycount: i32) -> windows_core::Result<()>;
    fn AddPropertyChangedEventHandler(&self, element: windows_core::Ref<IUIAutomationElement>, scope: TreeScope, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationPropertyChangedEventHandler>, propertyarray: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn RemovePropertyChangedEventHandler(&self, element: windows_core::Ref<IUIAutomationElement>, handler: windows_core::Ref<IUIAutomationPropertyChangedEventHandler>) -> windows_core::Result<()>;
    fn AddStructureChangedEventHandler(&self, element: windows_core::Ref<IUIAutomationElement>, scope: TreeScope, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationStructureChangedEventHandler>) -> windows_core::Result<()>;
    fn RemoveStructureChangedEventHandler(&self, element: windows_core::Ref<IUIAutomationElement>, handler: windows_core::Ref<IUIAutomationStructureChangedEventHandler>) -> windows_core::Result<()>;
    fn AddFocusChangedEventHandler(&self, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationFocusChangedEventHandler>) -> windows_core::Result<()>;
    fn RemoveFocusChangedEventHandler(&self, handler: windows_core::Ref<IUIAutomationFocusChangedEventHandler>) -> windows_core::Result<()>;
    fn RemoveAllEventHandlers(&self) -> windows_core::Result<()>;
    fn IntNativeArrayToSafeArray(&self, array: *const i32, arraycount: i32) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn IntSafeArrayToNativeArray(&self, intarray: *const super::SAFEARRAY, array: *mut *mut i32) -> windows_core::Result<i32>;
    fn RectToVariant(&self, rc: &super::RECT) -> windows_core::Result<super::VARIANT>;
    fn VariantToRect(&self, var: &super::VARIANT) -> windows_core::Result<super::RECT>;
    fn SafeArrayToRectNativeArray(&self, rects: *const super::SAFEARRAY, rectarray: *mut *mut super::RECT) -> windows_core::Result<i32>;
    fn CreateProxyFactoryEntry(&self, factory: windows_core::Ref<IUIAutomationProxyFactory>) -> windows_core::Result<IUIAutomationProxyFactoryEntry>;
    fn get_ProxyFactoryMapping(&self) -> windows_core::Result<IUIAutomationProxyFactoryMapping>;
    fn GetPropertyProgrammaticName(&self, property: super::PROPERTYID) -> windows_core::Result<windows_core::BSTR>;
    fn GetPatternProgrammaticName(&self, pattern: super::PATTERNID) -> windows_core::Result<windows_core::BSTR>;
    fn PollForPotentialSupportedPatterns(&self, pelement: windows_core::Ref<IUIAutomationElement>, patternids: *mut *mut super::SAFEARRAY, patternnames: *mut *mut super::SAFEARRAY) -> windows_core::Result<()>;
    fn PollForPotentialSupportedProperties(&self, pelement: windows_core::Ref<IUIAutomationElement>, propertyids: *mut *mut super::SAFEARRAY, propertynames: *mut *mut super::SAFEARRAY) -> windows_core::Result<()>;
    fn CheckNotSupported(&self, value: &super::VARIANT) -> windows_core::Result<windows_core::BOOL>;
    fn get_ReservedNotSupportedValue(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn get_ReservedMixedAttributeValue(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn ElementFromIAccessible(&self, accessible: windows_core::Ref<super::IAccessible>, childid: i32) -> windows_core::Result<IUIAutomationElement>;
    fn ElementFromIAccessibleBuildCache(&self, accessible: windows_core::Ref<super::IAccessible>, childid: i32, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>) -> windows_core::Result<IUIAutomationElement>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "oleacc", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomation_Vtbl {
    pub const fn new<Identity: IUIAutomation_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CompareElements<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, el1: *mut core::ffi::c_void, el2: *mut core::ffi::c_void, aresame: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CompareElements(this, core::mem::transmute_copy(&el1), core::mem::transmute_copy(&el2)) {
                    Ok(ok__) => {
                        aresame.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CompareRuntimeIds<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runtimeid1: *const super::SAFEARRAY, runtimeid2: *const super::SAFEARRAY, aresame: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CompareRuntimeIds(this, core::mem::transmute_copy(&runtimeid1), core::mem::transmute_copy(&runtimeid2)) {
                    Ok(ok__) => {
                        aresame.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRootElement<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, root: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::GetRootElement(this) {
                    Ok(ok__) => {
                        root.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ElementFromHandle<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: UIA_HWND, element: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::ElementFromHandle(this, core::mem::transmute_copy(&hwnd)) {
                    Ok(ok__) => {
                        element.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ElementFromPoint<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pt: super::POINT, element: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::ElementFromPoint(this, core::mem::transmute(&pt)) {
                    Ok(ok__) => {
                        element.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFocusedElement<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::GetFocusedElement(this) {
                    Ok(ok__) => {
                        element.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRootElementBuildCache<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cacherequest: *mut core::ffi::c_void, root: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::GetRootElementBuildCache(this, core::mem::transmute_copy(&cacherequest)) {
                    Ok(ok__) => {
                        root.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ElementFromHandleBuildCache<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: UIA_HWND, cacherequest: *mut core::ffi::c_void, element: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::ElementFromHandleBuildCache(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&cacherequest)) {
                    Ok(ok__) => {
                        element.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ElementFromPointBuildCache<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pt: super::POINT, cacherequest: *mut core::ffi::c_void, element: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::ElementFromPointBuildCache(this, core::mem::transmute(&pt), core::mem::transmute_copy(&cacherequest)) {
                    Ok(ok__) => {
                        element.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFocusedElementBuildCache<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cacherequest: *mut core::ffi::c_void, element: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::GetFocusedElementBuildCache(this, core::mem::transmute_copy(&cacherequest)) {
                    Ok(ok__) => {
                        element.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTreeWalker<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pcondition: *mut core::ffi::c_void, walker: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreateTreeWalker(this, core::mem::transmute_copy(&pcondition)) {
                    Ok(ok__) => {
                        walker.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ControlViewWalker<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, walker: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::get_ControlViewWalker(this) {
                    Ok(ok__) => {
                        walker.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ContentViewWalker<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, walker: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::get_ContentViewWalker(this) {
                    Ok(ok__) => {
                        walker.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RawViewWalker<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, walker: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::get_RawViewWalker(this) {
                    Ok(ok__) => {
                        walker.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RawViewCondition<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, condition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::get_RawViewCondition(this) {
                    Ok(ok__) => {
                        condition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ControlViewCondition<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, condition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::get_ControlViewCondition(this) {
                    Ok(ok__) => {
                        condition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ContentViewCondition<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, condition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::get_ContentViewCondition(this) {
                    Ok(ok__) => {
                        condition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateCacheRequest<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cacherequest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreateCacheRequest(this) {
                    Ok(ok__) => {
                        cacherequest.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateTrueCondition<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newcondition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreateTrueCondition(this) {
                    Ok(ok__) => {
                        newcondition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateFalseCondition<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, newcondition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreateFalseCondition(this) {
                    Ok(ok__) => {
                        newcondition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePropertyCondition<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: super::PROPERTYID, value: super::VARIANT, newcondition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreatePropertyCondition(this, core::mem::transmute_copy(&propertyid), core::mem::transmute(&value)) {
                    Ok(ok__) => {
                        newcondition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreatePropertyConditionEx<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: super::PROPERTYID, value: super::VARIANT, flags: PropertyConditionFlags, newcondition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreatePropertyConditionEx(this, core::mem::transmute_copy(&propertyid), core::mem::transmute(&value), core::mem::transmute_copy(&flags)) {
                    Ok(ok__) => {
                        newcondition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateAndCondition<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, condition1: *mut core::ffi::c_void, condition2: *mut core::ffi::c_void, newcondition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreateAndCondition(this, core::mem::transmute_copy(&condition1), core::mem::transmute_copy(&condition2)) {
                    Ok(ok__) => {
                        newcondition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateAndConditionFromArray<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, conditions: *const super::SAFEARRAY, newcondition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreateAndConditionFromArray(this, core::mem::transmute_copy(&conditions)) {
                    Ok(ok__) => {
                        newcondition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateAndConditionFromNativeArray<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, conditions: *const *mut core::ffi::c_void, conditioncount: i32, newcondition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreateAndConditionFromNativeArray(this, core::mem::transmute_copy(&conditions), core::mem::transmute_copy(&conditioncount)) {
                    Ok(ok__) => {
                        newcondition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateOrCondition<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, condition1: *mut core::ffi::c_void, condition2: *mut core::ffi::c_void, newcondition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreateOrCondition(this, core::mem::transmute_copy(&condition1), core::mem::transmute_copy(&condition2)) {
                    Ok(ok__) => {
                        newcondition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateOrConditionFromArray<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, conditions: *const super::SAFEARRAY, newcondition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreateOrConditionFromArray(this, core::mem::transmute_copy(&conditions)) {
                    Ok(ok__) => {
                        newcondition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateOrConditionFromNativeArray<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, conditions: *const *mut core::ffi::c_void, conditioncount: i32, newcondition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreateOrConditionFromNativeArray(this, core::mem::transmute_copy(&conditions), core::mem::transmute_copy(&conditioncount)) {
                    Ok(ok__) => {
                        newcondition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateNotCondition<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, condition: *mut core::ffi::c_void, newcondition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreateNotCondition(this, core::mem::transmute_copy(&condition)) {
                    Ok(ok__) => {
                        newcondition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddAutomationEventHandler<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: super::EVENTID, element: *mut core::ffi::c_void, scope: TreeScope, cacherequest: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::AddAutomationEventHandler(this, core::mem::transmute_copy(&eventid), core::mem::transmute_copy(&element), core::mem::transmute_copy(&scope), core::mem::transmute_copy(&cacherequest), core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn RemoveAutomationEventHandler<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: super::EVENTID, element: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::RemoveAutomationEventHandler(this, core::mem::transmute_copy(&eventid), core::mem::transmute_copy(&element), core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn AddPropertyChangedEventHandlerNativeArray<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, scope: TreeScope, cacherequest: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, propertyarray: *const super::PROPERTYID, propertycount: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::AddPropertyChangedEventHandlerNativeArray(this, core::mem::transmute_copy(&element), core::mem::transmute_copy(&scope), core::mem::transmute_copy(&cacherequest), core::mem::transmute_copy(&handler), core::mem::transmute_copy(&propertyarray), core::mem::transmute_copy(&propertycount)).into()
            }
        }
        unsafe extern "system" fn AddPropertyChangedEventHandler<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, scope: TreeScope, cacherequest: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, propertyarray: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::AddPropertyChangedEventHandler(this, core::mem::transmute_copy(&element), core::mem::transmute_copy(&scope), core::mem::transmute_copy(&cacherequest), core::mem::transmute_copy(&handler), core::mem::transmute_copy(&propertyarray)).into()
            }
        }
        unsafe extern "system" fn RemovePropertyChangedEventHandler<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::RemovePropertyChangedEventHandler(this, core::mem::transmute_copy(&element), core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn AddStructureChangedEventHandler<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, scope: TreeScope, cacherequest: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::AddStructureChangedEventHandler(this, core::mem::transmute_copy(&element), core::mem::transmute_copy(&scope), core::mem::transmute_copy(&cacherequest), core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn RemoveStructureChangedEventHandler<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::RemoveStructureChangedEventHandler(this, core::mem::transmute_copy(&element), core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn AddFocusChangedEventHandler<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cacherequest: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::AddFocusChangedEventHandler(this, core::mem::transmute_copy(&cacherequest), core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn RemoveFocusChangedEventHandler<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::RemoveFocusChangedEventHandler(this, core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn RemoveAllEventHandlers<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::RemoveAllEventHandlers(this).into()
            }
        }
        unsafe extern "system" fn IntNativeArrayToSafeArray<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, array: *const i32, arraycount: i32, safearray: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::IntNativeArrayToSafeArray(this, core::mem::transmute_copy(&array), core::mem::transmute_copy(&arraycount)) {
                    Ok(ok__) => {
                        safearray.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IntSafeArrayToNativeArray<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, intarray: *const super::SAFEARRAY, array: *mut *mut i32, arraycount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::IntSafeArrayToNativeArray(this, core::mem::transmute_copy(&intarray), core::mem::transmute_copy(&array)) {
                    Ok(ok__) => {
                        arraycount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RectToVariant<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rc: super::RECT, var: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::RectToVariant(this, core::mem::transmute(&rc)) {
                    Ok(ok__) => {
                        var.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VariantToRect<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, var: super::VARIANT, rc: *mut super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::VariantToRect(this, core::mem::transmute(&var)) {
                    Ok(ok__) => {
                        rc.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SafeArrayToRectNativeArray<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rects: *const super::SAFEARRAY, rectarray: *mut *mut super::RECT, rectarraycount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::SafeArrayToRectNativeArray(this, core::mem::transmute_copy(&rects), core::mem::transmute_copy(&rectarray)) {
                    Ok(ok__) => {
                        rectarraycount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CreateProxyFactoryEntry<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, factory: *mut core::ffi::c_void, factoryentry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CreateProxyFactoryEntry(this, core::mem::transmute_copy(&factory)) {
                    Ok(ok__) => {
                        factoryentry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ProxyFactoryMapping<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, factorymapping: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::get_ProxyFactoryMapping(this) {
                    Ok(ok__) => {
                        factorymapping.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropertyProgrammaticName<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: super::PROPERTYID, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::GetPropertyProgrammaticName(this, core::mem::transmute_copy(&property)) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPatternProgrammaticName<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattern: super::PATTERNID, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::GetPatternProgrammaticName(this, core::mem::transmute_copy(&pattern)) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PollForPotentialSupportedPatterns<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pelement: *mut core::ffi::c_void, patternids: *mut *mut super::SAFEARRAY, patternnames: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::PollForPotentialSupportedPatterns(this, core::mem::transmute_copy(&pelement), core::mem::transmute_copy(&patternids), core::mem::transmute_copy(&patternnames)).into()
            }
        }
        unsafe extern "system" fn PollForPotentialSupportedProperties<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pelement: *mut core::ffi::c_void, propertyids: *mut *mut super::SAFEARRAY, propertynames: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::PollForPotentialSupportedProperties(this, core::mem::transmute_copy(&pelement), core::mem::transmute_copy(&propertyids), core::mem::transmute_copy(&propertynames)).into()
            }
        }
        unsafe extern "system" fn CheckNotSupported<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::VARIANT, isnotsupported: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::CheckNotSupported(this, core::mem::transmute(&value)) {
                    Ok(ok__) => {
                        isnotsupported.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ReservedNotSupportedValue<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notsupportedvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::get_ReservedNotSupportedValue(this) {
                    Ok(ok__) => {
                        notsupportedvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ReservedMixedAttributeValue<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mixedattributevalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::get_ReservedMixedAttributeValue(this) {
                    Ok(ok__) => {
                        mixedattributevalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ElementFromIAccessible<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, accessible: *mut core::ffi::c_void, childid: i32, element: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::ElementFromIAccessible(this, core::mem::transmute_copy(&accessible), core::mem::transmute_copy(&childid)) {
                    Ok(ok__) => {
                        element.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ElementFromIAccessibleBuildCache<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, accessible: *mut core::ffi::c_void, childid: i32, cacherequest: *mut core::ffi::c_void, element: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::ElementFromIAccessibleBuildCache(this, core::mem::transmute_copy(&accessible), core::mem::transmute_copy(&childid), core::mem::transmute_copy(&cacherequest)) {
                    Ok(ok__) => {
                        element.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CompareElements: CompareElements::<Identity, OFFSET>,
            CompareRuntimeIds: CompareRuntimeIds::<Identity, OFFSET>,
            GetRootElement: GetRootElement::<Identity, OFFSET>,
            ElementFromHandle: ElementFromHandle::<Identity, OFFSET>,
            ElementFromPoint: ElementFromPoint::<Identity, OFFSET>,
            GetFocusedElement: GetFocusedElement::<Identity, OFFSET>,
            GetRootElementBuildCache: GetRootElementBuildCache::<Identity, OFFSET>,
            ElementFromHandleBuildCache: ElementFromHandleBuildCache::<Identity, OFFSET>,
            ElementFromPointBuildCache: ElementFromPointBuildCache::<Identity, OFFSET>,
            GetFocusedElementBuildCache: GetFocusedElementBuildCache::<Identity, OFFSET>,
            CreateTreeWalker: CreateTreeWalker::<Identity, OFFSET>,
            get_ControlViewWalker: get_ControlViewWalker::<Identity, OFFSET>,
            get_ContentViewWalker: get_ContentViewWalker::<Identity, OFFSET>,
            get_RawViewWalker: get_RawViewWalker::<Identity, OFFSET>,
            get_RawViewCondition: get_RawViewCondition::<Identity, OFFSET>,
            get_ControlViewCondition: get_ControlViewCondition::<Identity, OFFSET>,
            get_ContentViewCondition: get_ContentViewCondition::<Identity, OFFSET>,
            CreateCacheRequest: CreateCacheRequest::<Identity, OFFSET>,
            CreateTrueCondition: CreateTrueCondition::<Identity, OFFSET>,
            CreateFalseCondition: CreateFalseCondition::<Identity, OFFSET>,
            CreatePropertyCondition: CreatePropertyCondition::<Identity, OFFSET>,
            CreatePropertyConditionEx: CreatePropertyConditionEx::<Identity, OFFSET>,
            CreateAndCondition: CreateAndCondition::<Identity, OFFSET>,
            CreateAndConditionFromArray: CreateAndConditionFromArray::<Identity, OFFSET>,
            CreateAndConditionFromNativeArray: CreateAndConditionFromNativeArray::<Identity, OFFSET>,
            CreateOrCondition: CreateOrCondition::<Identity, OFFSET>,
            CreateOrConditionFromArray: CreateOrConditionFromArray::<Identity, OFFSET>,
            CreateOrConditionFromNativeArray: CreateOrConditionFromNativeArray::<Identity, OFFSET>,
            CreateNotCondition: CreateNotCondition::<Identity, OFFSET>,
            AddAutomationEventHandler: AddAutomationEventHandler::<Identity, OFFSET>,
            RemoveAutomationEventHandler: RemoveAutomationEventHandler::<Identity, OFFSET>,
            AddPropertyChangedEventHandlerNativeArray: AddPropertyChangedEventHandlerNativeArray::<Identity, OFFSET>,
            AddPropertyChangedEventHandler: AddPropertyChangedEventHandler::<Identity, OFFSET>,
            RemovePropertyChangedEventHandler: RemovePropertyChangedEventHandler::<Identity, OFFSET>,
            AddStructureChangedEventHandler: AddStructureChangedEventHandler::<Identity, OFFSET>,
            RemoveStructureChangedEventHandler: RemoveStructureChangedEventHandler::<Identity, OFFSET>,
            AddFocusChangedEventHandler: AddFocusChangedEventHandler::<Identity, OFFSET>,
            RemoveFocusChangedEventHandler: RemoveFocusChangedEventHandler::<Identity, OFFSET>,
            RemoveAllEventHandlers: RemoveAllEventHandlers::<Identity, OFFSET>,
            IntNativeArrayToSafeArray: IntNativeArrayToSafeArray::<Identity, OFFSET>,
            IntSafeArrayToNativeArray: IntSafeArrayToNativeArray::<Identity, OFFSET>,
            RectToVariant: RectToVariant::<Identity, OFFSET>,
            VariantToRect: VariantToRect::<Identity, OFFSET>,
            SafeArrayToRectNativeArray: SafeArrayToRectNativeArray::<Identity, OFFSET>,
            CreateProxyFactoryEntry: CreateProxyFactoryEntry::<Identity, OFFSET>,
            get_ProxyFactoryMapping: get_ProxyFactoryMapping::<Identity, OFFSET>,
            GetPropertyProgrammaticName: GetPropertyProgrammaticName::<Identity, OFFSET>,
            GetPatternProgrammaticName: GetPatternProgrammaticName::<Identity, OFFSET>,
            PollForPotentialSupportedPatterns: PollForPotentialSupportedPatterns::<Identity, OFFSET>,
            PollForPotentialSupportedProperties: PollForPotentialSupportedProperties::<Identity, OFFSET>,
            CheckNotSupported: CheckNotSupported::<Identity, OFFSET>,
            get_ReservedNotSupportedValue: get_ReservedNotSupportedValue::<Identity, OFFSET>,
            get_ReservedMixedAttributeValue: get_ReservedMixedAttributeValue::<Identity, OFFSET>,
            ElementFromIAccessible: ElementFromIAccessible::<Identity, OFFSET>,
            ElementFromIAccessibleBuildCache: ElementFromIAccessibleBuildCache::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomation as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "oleacc", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUIAutomation {}
windows_core::imp::define_interface!(IUIAutomation2, IUIAutomation2_Vtbl, 0x34723aff_0c9d_49d0_9896_7ab52df8cd8a);
impl core::ops::Deref for IUIAutomation2 {
    type Target = IUIAutomation;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomation2, windows_core::IUnknown, IUIAutomation);
impl IUIAutomation2 {
    pub unsafe fn get_AutoSetFocus(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AutoSetFocus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_AutoSetFocus(&self, autosetfocus: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AutoSetFocus)(windows_core::Interface::as_raw(self), autosetfocus.into()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_ConnectionTimeout(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ConnectionTimeout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_ConnectionTimeout(&self, timeout: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ConnectionTimeout)(windows_core::Interface::as_raw(self), timeout) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_TransactionTimeout(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TransactionTimeout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn put_TransactionTimeout(&self, timeout: super::DWORD) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_TransactionTimeout)(windows_core::Interface::as_raw(self), timeout) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomation2_Vtbl {
    pub base__: IUIAutomation_Vtbl,
    pub get_AutoSetFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub put_AutoSetFocus: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_ConnectionTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_ConnectionTimeout: usize,
    #[cfg(feature = "shared")]
    pub put_ConnectionTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_ConnectionTimeout: usize,
    #[cfg(feature = "shared")]
    pub get_TransactionTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_TransactionTimeout: usize,
    #[cfg(feature = "shared")]
    pub put_TransactionTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    put_TransactionTimeout: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "oleacc", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomation2_Impl: IUIAutomation_Impl {
    fn get_AutoSetFocus(&self) -> windows_core::Result<windows_core::BOOL>;
    fn put_AutoSetFocus(&self, autosetfocus: windows_core::BOOL) -> windows_core::Result<()>;
    fn get_ConnectionTimeout(&self) -> windows_core::Result<super::DWORD>;
    fn put_ConnectionTimeout(&self, timeout: super::DWORD) -> windows_core::Result<()>;
    fn get_TransactionTimeout(&self) -> windows_core::Result<super::DWORD>;
    fn put_TransactionTimeout(&self, timeout: super::DWORD) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "oleacc", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomation2_Vtbl {
    pub const fn new<Identity: IUIAutomation2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_AutoSetFocus<Identity: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, autosetfocus: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation2_Impl::get_AutoSetFocus(this) {
                    Ok(ok__) => {
                        autosetfocus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AutoSetFocus<Identity: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, autosetfocus: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation2_Impl::put_AutoSetFocus(this, core::mem::transmute_copy(&autosetfocus)).into()
            }
        }
        unsafe extern "system" fn get_ConnectionTimeout<Identity: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeout: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation2_Impl::get_ConnectionTimeout(this) {
                    Ok(ok__) => {
                        timeout.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ConnectionTimeout<Identity: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeout: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation2_Impl::put_ConnectionTimeout(this, core::mem::transmute_copy(&timeout)).into()
            }
        }
        unsafe extern "system" fn get_TransactionTimeout<Identity: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeout: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation2_Impl::get_TransactionTimeout(this) {
                    Ok(ok__) => {
                        timeout.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_TransactionTimeout<Identity: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeout: super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation2_Impl::put_TransactionTimeout(this, core::mem::transmute_copy(&timeout)).into()
            }
        }
        Self {
            base__: IUIAutomation_Vtbl::new::<Identity, OFFSET>(),
            get_AutoSetFocus: get_AutoSetFocus::<Identity, OFFSET>,
            put_AutoSetFocus: put_AutoSetFocus::<Identity, OFFSET>,
            get_ConnectionTimeout: get_ConnectionTimeout::<Identity, OFFSET>,
            put_ConnectionTimeout: put_ConnectionTimeout::<Identity, OFFSET>,
            get_TransactionTimeout: get_TransactionTimeout::<Identity, OFFSET>,
            put_TransactionTimeout: put_TransactionTimeout::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomation2 as windows_core::Interface>::IID || iid == &<IUIAutomation as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "oleacc", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUIAutomation2 {}
windows_core::imp::define_interface!(IUIAutomation3, IUIAutomation3_Vtbl, 0x73d768da_9b51_4b89_936e_c209290973e7);
impl core::ops::Deref for IUIAutomation3 {
    type Target = IUIAutomation2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomation3, windows_core::IUnknown, IUIAutomation, IUIAutomation2);
impl IUIAutomation3 {
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn AddTextEditTextChangedEventHandler<P0, P3, P4>(&self, element: P0, scope: TreeScope, texteditchangetype: super::TextEditChangeType, cacherequest: P3, handler: P4) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P3: windows_core::Param<IUIAutomationCacheRequest>,
        P4: windows_core::Param<IUIAutomationTextEditTextChangedEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddTextEditTextChangedEventHandler)(windows_core::Interface::as_raw(self), element.param().abi(), scope, texteditchangetype, cacherequest.param().abi(), handler.param().abi()) }
    }
    pub unsafe fn RemoveTextEditTextChangedEventHandler<P0, P1>(&self, element: P0, handler: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationTextEditTextChangedEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveTextEditTextChangedEventHandler)(windows_core::Interface::as_raw(self), element.param().abi(), handler.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomation3_Vtbl {
    pub base__: IUIAutomation2_Vtbl,
    #[cfg(feature = "uiautomationcore")]
    pub AddTextEditTextChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TreeScope, super::TextEditChangeType, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    AddTextEditTextChangedEventHandler: usize,
    pub RemoveTextEditTextChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "oleacc", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomation3_Impl: IUIAutomation2_Impl {
    fn AddTextEditTextChangedEventHandler(&self, element: windows_core::Ref<IUIAutomationElement>, scope: TreeScope, texteditchangetype: super::TextEditChangeType, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationTextEditTextChangedEventHandler>) -> windows_core::Result<()>;
    fn RemoveTextEditTextChangedEventHandler(&self, element: windows_core::Ref<IUIAutomationElement>, handler: windows_core::Ref<IUIAutomationTextEditTextChangedEventHandler>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "oleacc", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomation3_Vtbl {
    pub const fn new<Identity: IUIAutomation3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddTextEditTextChangedEventHandler<Identity: IUIAutomation3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, scope: TreeScope, texteditchangetype: super::TextEditChangeType, cacherequest: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation3_Impl::AddTextEditTextChangedEventHandler(this, core::mem::transmute_copy(&element), core::mem::transmute_copy(&scope), core::mem::transmute_copy(&texteditchangetype), core::mem::transmute_copy(&cacherequest), core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn RemoveTextEditTextChangedEventHandler<Identity: IUIAutomation3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation3_Impl::RemoveTextEditTextChangedEventHandler(this, core::mem::transmute_copy(&element), core::mem::transmute_copy(&handler)).into()
            }
        }
        Self {
            base__: IUIAutomation2_Vtbl::new::<Identity, OFFSET>(),
            AddTextEditTextChangedEventHandler: AddTextEditTextChangedEventHandler::<Identity, OFFSET>,
            RemoveTextEditTextChangedEventHandler: RemoveTextEditTextChangedEventHandler::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomation3 as windows_core::Interface>::IID || iid == &<IUIAutomation as windows_core::Interface>::IID || iid == &<IUIAutomation2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "oleacc", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUIAutomation3 {}
windows_core::imp::define_interface!(IUIAutomation4, IUIAutomation4_Vtbl, 0x1189c02a_05f8_4319_8e21_e817e3db2860);
impl core::ops::Deref for IUIAutomation4 {
    type Target = IUIAutomation3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomation4, windows_core::IUnknown, IUIAutomation, IUIAutomation2, IUIAutomation3);
impl IUIAutomation4 {
    pub unsafe fn AddChangesEventHandler<P0, P4, P5>(&self, element: P0, scope: TreeScope, changetypes: &[i32], pcacherequest: P4, handler: P5) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P4: windows_core::Param<IUIAutomationCacheRequest>,
        P5: windows_core::Param<IUIAutomationChangesEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddChangesEventHandler)(windows_core::Interface::as_raw(self), element.param().abi(), scope, changetypes.as_ptr(), changetypes.len().try_into().unwrap(), pcacherequest.param().abi(), handler.param().abi()) }
    }
    pub unsafe fn RemoveChangesEventHandler<P0, P1>(&self, element: P0, handler: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationChangesEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveChangesEventHandler)(windows_core::Interface::as_raw(self), element.param().abi(), handler.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomation4_Vtbl {
    pub base__: IUIAutomation3_Vtbl,
    pub AddChangesEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TreeScope, *const i32, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveChangesEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "oleacc", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomation4_Impl: IUIAutomation3_Impl {
    fn AddChangesEventHandler(&self, element: windows_core::Ref<IUIAutomationElement>, scope: TreeScope, changetypes: *const i32, changescount: i32, pcacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationChangesEventHandler>) -> windows_core::Result<()>;
    fn RemoveChangesEventHandler(&self, element: windows_core::Ref<IUIAutomationElement>, handler: windows_core::Ref<IUIAutomationChangesEventHandler>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "oleacc", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomation4_Vtbl {
    pub const fn new<Identity: IUIAutomation4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddChangesEventHandler<Identity: IUIAutomation4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, scope: TreeScope, changetypes: *const i32, changescount: i32, pcacherequest: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation4_Impl::AddChangesEventHandler(this, core::mem::transmute_copy(&element), core::mem::transmute_copy(&scope), core::mem::transmute_copy(&changetypes), core::mem::transmute_copy(&changescount), core::mem::transmute_copy(&pcacherequest), core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn RemoveChangesEventHandler<Identity: IUIAutomation4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation4_Impl::RemoveChangesEventHandler(this, core::mem::transmute_copy(&element), core::mem::transmute_copy(&handler)).into()
            }
        }
        Self {
            base__: IUIAutomation3_Vtbl::new::<Identity, OFFSET>(),
            AddChangesEventHandler: AddChangesEventHandler::<Identity, OFFSET>,
            RemoveChangesEventHandler: RemoveChangesEventHandler::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomation4 as windows_core::Interface>::IID || iid == &<IUIAutomation as windows_core::Interface>::IID || iid == &<IUIAutomation2 as windows_core::Interface>::IID || iid == &<IUIAutomation3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "oleacc", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUIAutomation4 {}
windows_core::imp::define_interface!(IUIAutomation5, IUIAutomation5_Vtbl, 0x25f700c8_d816_4057_a9dc_3cbdee77e256);
impl core::ops::Deref for IUIAutomation5 {
    type Target = IUIAutomation4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomation5, windows_core::IUnknown, IUIAutomation, IUIAutomation2, IUIAutomation3, IUIAutomation4);
impl IUIAutomation5 {
    pub unsafe fn AddNotificationEventHandler<P0, P2, P3>(&self, element: P0, scope: TreeScope, cacherequest: P2, handler: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P2: windows_core::Param<IUIAutomationCacheRequest>,
        P3: windows_core::Param<IUIAutomationNotificationEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddNotificationEventHandler)(windows_core::Interface::as_raw(self), element.param().abi(), scope, cacherequest.param().abi(), handler.param().abi()) }
    }
    pub unsafe fn RemoveNotificationEventHandler<P0, P1>(&self, element: P0, handler: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationNotificationEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveNotificationEventHandler)(windows_core::Interface::as_raw(self), element.param().abi(), handler.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomation5_Vtbl {
    pub base__: IUIAutomation4_Vtbl,
    pub AddNotificationEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveNotificationEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "oleacc", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomation5_Impl: IUIAutomation4_Impl {
    fn AddNotificationEventHandler(&self, element: windows_core::Ref<IUIAutomationElement>, scope: TreeScope, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationNotificationEventHandler>) -> windows_core::Result<()>;
    fn RemoveNotificationEventHandler(&self, element: windows_core::Ref<IUIAutomationElement>, handler: windows_core::Ref<IUIAutomationNotificationEventHandler>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "oleacc", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomation5_Vtbl {
    pub const fn new<Identity: IUIAutomation5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddNotificationEventHandler<Identity: IUIAutomation5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, scope: TreeScope, cacherequest: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation5_Impl::AddNotificationEventHandler(this, core::mem::transmute_copy(&element), core::mem::transmute_copy(&scope), core::mem::transmute_copy(&cacherequest), core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn RemoveNotificationEventHandler<Identity: IUIAutomation5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation5_Impl::RemoveNotificationEventHandler(this, core::mem::transmute_copy(&element), core::mem::transmute_copy(&handler)).into()
            }
        }
        Self {
            base__: IUIAutomation4_Vtbl::new::<Identity, OFFSET>(),
            AddNotificationEventHandler: AddNotificationEventHandler::<Identity, OFFSET>,
            RemoveNotificationEventHandler: RemoveNotificationEventHandler::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomation5 as windows_core::Interface>::IID || iid == &<IUIAutomation as windows_core::Interface>::IID || iid == &<IUIAutomation2 as windows_core::Interface>::IID || iid == &<IUIAutomation3 as windows_core::Interface>::IID || iid == &<IUIAutomation4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "oleacc", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUIAutomation5 {}
windows_core::imp::define_interface!(IUIAutomation6, IUIAutomation6_Vtbl, 0xaae072da_29e3_413d_87a7_192dbf81ed10);
impl core::ops::Deref for IUIAutomation6 {
    type Target = IUIAutomation5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomation6, windows_core::IUnknown, IUIAutomation, IUIAutomation2, IUIAutomation3, IUIAutomation4, IUIAutomation5);
impl IUIAutomation6 {
    pub unsafe fn CreateEventHandlerGroup(&self) -> windows_core::Result<IUIAutomationEventHandlerGroup> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateEventHandlerGroup)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn AddEventHandlerGroup<P0, P1>(&self, element: P0, handlergroup: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationEventHandlerGroup>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddEventHandlerGroup)(windows_core::Interface::as_raw(self), element.param().abi(), handlergroup.param().abi()) }
    }
    pub unsafe fn RemoveEventHandlerGroup<P0, P1>(&self, element: P0, handlergroup: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationEventHandlerGroup>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveEventHandlerGroup)(windows_core::Interface::as_raw(self), element.param().abi(), handlergroup.param().abi()) }
    }
    pub unsafe fn get_ConnectionRecoveryBehavior(&self) -> windows_core::Result<ConnectionRecoveryBehaviorOptions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ConnectionRecoveryBehavior)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ConnectionRecoveryBehavior(&self, connectionrecoverybehavioroptions: ConnectionRecoveryBehaviorOptions) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_ConnectionRecoveryBehavior)(windows_core::Interface::as_raw(self), connectionrecoverybehavioroptions) }
    }
    pub unsafe fn get_CoalesceEvents(&self) -> windows_core::Result<CoalesceEventsOptions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CoalesceEvents)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_CoalesceEvents(&self, coalesceeventsoptions: CoalesceEventsOptions) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_CoalesceEvents)(windows_core::Interface::as_raw(self), coalesceeventsoptions) }
    }
    pub unsafe fn AddActiveTextPositionChangedEventHandler<P0, P2, P3>(&self, element: P0, scope: TreeScope, cacherequest: P2, handler: P3) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P2: windows_core::Param<IUIAutomationCacheRequest>,
        P3: windows_core::Param<IUIAutomationActiveTextPositionChangedEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddActiveTextPositionChangedEventHandler)(windows_core::Interface::as_raw(self), element.param().abi(), scope, cacherequest.param().abi(), handler.param().abi()) }
    }
    pub unsafe fn RemoveActiveTextPositionChangedEventHandler<P0, P1>(&self, element: P0, handler: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationActiveTextPositionChangedEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveActiveTextPositionChangedEventHandler)(windows_core::Interface::as_raw(self), element.param().abi(), handler.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomation6_Vtbl {
    pub base__: IUIAutomation5_Vtbl,
    pub CreateEventHandlerGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddEventHandlerGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveEventHandlerGroup: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ConnectionRecoveryBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ConnectionRecoveryBehaviorOptions) -> windows_core::HRESULT,
    pub put_ConnectionRecoveryBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, ConnectionRecoveryBehaviorOptions) -> windows_core::HRESULT,
    pub get_CoalesceEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoalesceEventsOptions) -> windows_core::HRESULT,
    pub put_CoalesceEvents: unsafe extern "system" fn(*mut core::ffi::c_void, CoalesceEventsOptions) -> windows_core::HRESULT,
    pub AddActiveTextPositionChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveActiveTextPositionChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "oleacc", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomation6_Impl: IUIAutomation5_Impl {
    fn CreateEventHandlerGroup(&self) -> windows_core::Result<IUIAutomationEventHandlerGroup>;
    fn AddEventHandlerGroup(&self, element: windows_core::Ref<IUIAutomationElement>, handlergroup: windows_core::Ref<IUIAutomationEventHandlerGroup>) -> windows_core::Result<()>;
    fn RemoveEventHandlerGroup(&self, element: windows_core::Ref<IUIAutomationElement>, handlergroup: windows_core::Ref<IUIAutomationEventHandlerGroup>) -> windows_core::Result<()>;
    fn get_ConnectionRecoveryBehavior(&self) -> windows_core::Result<ConnectionRecoveryBehaviorOptions>;
    fn put_ConnectionRecoveryBehavior(&self, connectionrecoverybehavioroptions: ConnectionRecoveryBehaviorOptions) -> windows_core::Result<()>;
    fn get_CoalesceEvents(&self) -> windows_core::Result<CoalesceEventsOptions>;
    fn put_CoalesceEvents(&self, coalesceeventsoptions: CoalesceEventsOptions) -> windows_core::Result<()>;
    fn AddActiveTextPositionChangedEventHandler(&self, element: windows_core::Ref<IUIAutomationElement>, scope: TreeScope, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationActiveTextPositionChangedEventHandler>) -> windows_core::Result<()>;
    fn RemoveActiveTextPositionChangedEventHandler(&self, element: windows_core::Ref<IUIAutomationElement>, handler: windows_core::Ref<IUIAutomationActiveTextPositionChangedEventHandler>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "oleacc", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomation6_Vtbl {
    pub const fn new<Identity: IUIAutomation6_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateEventHandlerGroup<Identity: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handlergroup: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation6_Impl::CreateEventHandlerGroup(this) {
                    Ok(ok__) => {
                        handlergroup.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AddEventHandlerGroup<Identity: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, handlergroup: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation6_Impl::AddEventHandlerGroup(this, core::mem::transmute_copy(&element), core::mem::transmute_copy(&handlergroup)).into()
            }
        }
        unsafe extern "system" fn RemoveEventHandlerGroup<Identity: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, handlergroup: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation6_Impl::RemoveEventHandlerGroup(this, core::mem::transmute_copy(&element), core::mem::transmute_copy(&handlergroup)).into()
            }
        }
        unsafe extern "system" fn get_ConnectionRecoveryBehavior<Identity: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectionrecoverybehavioroptions: *mut ConnectionRecoveryBehaviorOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation6_Impl::get_ConnectionRecoveryBehavior(this) {
                    Ok(ok__) => {
                        connectionrecoverybehavioroptions.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ConnectionRecoveryBehavior<Identity: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectionrecoverybehavioroptions: ConnectionRecoveryBehaviorOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation6_Impl::put_ConnectionRecoveryBehavior(this, core::mem::transmute_copy(&connectionrecoverybehavioroptions)).into()
            }
        }
        unsafe extern "system" fn get_CoalesceEvents<Identity: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, coalesceeventsoptions: *mut CoalesceEventsOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation6_Impl::get_CoalesceEvents(this) {
                    Ok(ok__) => {
                        coalesceeventsoptions.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_CoalesceEvents<Identity: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, coalesceeventsoptions: CoalesceEventsOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation6_Impl::put_CoalesceEvents(this, core::mem::transmute_copy(&coalesceeventsoptions)).into()
            }
        }
        unsafe extern "system" fn AddActiveTextPositionChangedEventHandler<Identity: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, scope: TreeScope, cacherequest: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation6_Impl::AddActiveTextPositionChangedEventHandler(this, core::mem::transmute_copy(&element), core::mem::transmute_copy(&scope), core::mem::transmute_copy(&cacherequest), core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn RemoveActiveTextPositionChangedEventHandler<Identity: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation6_Impl::RemoveActiveTextPositionChangedEventHandler(this, core::mem::transmute_copy(&element), core::mem::transmute_copy(&handler)).into()
            }
        }
        Self {
            base__: IUIAutomation5_Vtbl::new::<Identity, OFFSET>(),
            CreateEventHandlerGroup: CreateEventHandlerGroup::<Identity, OFFSET>,
            AddEventHandlerGroup: AddEventHandlerGroup::<Identity, OFFSET>,
            RemoveEventHandlerGroup: RemoveEventHandlerGroup::<Identity, OFFSET>,
            get_ConnectionRecoveryBehavior: get_ConnectionRecoveryBehavior::<Identity, OFFSET>,
            put_ConnectionRecoveryBehavior: put_ConnectionRecoveryBehavior::<Identity, OFFSET>,
            get_CoalesceEvents: get_CoalesceEvents::<Identity, OFFSET>,
            put_CoalesceEvents: put_CoalesceEvents::<Identity, OFFSET>,
            AddActiveTextPositionChangedEventHandler: AddActiveTextPositionChangedEventHandler::<Identity, OFFSET>,
            RemoveActiveTextPositionChangedEventHandler: RemoveActiveTextPositionChangedEventHandler::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomation6 as windows_core::Interface>::IID || iid == &<IUIAutomation as windows_core::Interface>::IID || iid == &<IUIAutomation2 as windows_core::Interface>::IID || iid == &<IUIAutomation3 as windows_core::Interface>::IID || iid == &<IUIAutomation4 as windows_core::Interface>::IID || iid == &<IUIAutomation5 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "oleacc", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUIAutomation6 {}
windows_core::imp::define_interface!(IUIAutomationActiveTextPositionChangedEventHandler, IUIAutomationActiveTextPositionChangedEventHandler_Vtbl, 0xf97933b0_8dae_4496_8997_5ba015fe0d82);
windows_core::imp::interface_hierarchy!(IUIAutomationActiveTextPositionChangedEventHandler, windows_core::IUnknown);
impl IUIAutomationActiveTextPositionChangedEventHandler {
    pub unsafe fn HandleActiveTextPositionChangedEvent<P0, P1>(&self, sender: P0, range: P1) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationTextRange>,
    {
        unsafe { (windows_core::Interface::vtable(self).HandleActiveTextPositionChangedEvent)(windows_core::Interface::as_raw(self), sender.param().abi(), range.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationActiveTextPositionChangedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub HandleActiveTextPositionChangedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAutomationActiveTextPositionChangedEventHandler_Impl: windows_core::IUnknownImpl {
    fn HandleActiveTextPositionChangedEvent(&self, sender: windows_core::Ref<IUIAutomationElement>, range: windows_core::Ref<IUIAutomationTextRange>) -> windows_core::Result<()>;
}
impl IUIAutomationActiveTextPositionChangedEventHandler_Vtbl {
    pub const fn new<Identity: IUIAutomationActiveTextPositionChangedEventHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HandleActiveTextPositionChangedEvent<Identity: IUIAutomationActiveTextPositionChangedEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, range: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationActiveTextPositionChangedEventHandler_Impl::HandleActiveTextPositionChangedEvent(this, core::mem::transmute_copy(&sender), core::mem::transmute_copy(&range)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            HandleActiveTextPositionChangedEvent: HandleActiveTextPositionChangedEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationActiveTextPositionChangedEventHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationActiveTextPositionChangedEventHandler {}
windows_core::imp::define_interface!(IUIAutomationAndCondition, IUIAutomationAndCondition_Vtbl, 0xa7d0af36_b912_45fe_9855_091ddc174aec);
impl core::ops::Deref for IUIAutomationAndCondition {
    type Target = IUIAutomationCondition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationAndCondition, windows_core::IUnknown, IUIAutomationCondition);
impl IUIAutomationAndCondition {
    pub unsafe fn get_ChildCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ChildCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetChildrenAsNativeArray(&self, childarray: *mut *mut Option<IUIAutomationCondition>, childarraycount: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetChildrenAsNativeArray)(windows_core::Interface::as_raw(self), childarray as _, childarraycount as _) }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetChildren(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChildren)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationAndCondition_Vtbl {
    pub base__: IUIAutomationCondition_Vtbl,
    pub get_ChildCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetChildrenAsNativeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetChildren: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetChildren: usize,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
pub trait IUIAutomationAndCondition_Impl: IUIAutomationCondition_Impl {
    fn get_ChildCount(&self) -> windows_core::Result<i32>;
    fn GetChildrenAsNativeArray(&self, childarray: *mut *mut Option<IUIAutomationCondition>, childarraycount: *mut i32) -> windows_core::Result<()>;
    fn GetChildren(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl IUIAutomationAndCondition_Vtbl {
    pub const fn new<Identity: IUIAutomationAndCondition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ChildCount<Identity: IUIAutomationAndCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, childcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationAndCondition_Impl::get_ChildCount(this) {
                    Ok(ok__) => {
                        childcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetChildrenAsNativeArray<Identity: IUIAutomationAndCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, childarray: *mut *mut *mut core::ffi::c_void, childarraycount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationAndCondition_Impl::GetChildrenAsNativeArray(this, core::mem::transmute_copy(&childarray), core::mem::transmute_copy(&childarraycount)).into()
            }
        }
        unsafe extern "system" fn GetChildren<Identity: IUIAutomationAndCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, childarray: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationAndCondition_Impl::GetChildren(this) {
                    Ok(ok__) => {
                        childarray.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IUIAutomationCondition_Vtbl::new::<Identity, OFFSET>(),
            get_ChildCount: get_ChildCount::<Identity, OFFSET>,
            GetChildrenAsNativeArray: GetChildrenAsNativeArray::<Identity, OFFSET>,
            GetChildren: GetChildren::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationAndCondition as windows_core::Interface>::IID || iid == &<IUIAutomationCondition as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IUIAutomationAndCondition {}
windows_core::imp::define_interface!(IUIAutomationAnnotationPattern, IUIAutomationAnnotationPattern_Vtbl, 0x9a175b21_339e_41b1_8e8b_623f6b681098);
windows_core::imp::interface_hierarchy!(IUIAutomationAnnotationPattern, windows_core::IUnknown);
impl IUIAutomationAnnotationPattern {
    pub unsafe fn get_CurrentAnnotationTypeId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentAnnotationTypeId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentAnnotationTypeName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentAnnotationTypeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CurrentAuthor(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentAuthor)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CurrentDateTime(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentDateTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CurrentTarget(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CachedAnnotationTypeId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedAnnotationTypeId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedAnnotationTypeName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedAnnotationTypeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CachedAuthor(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedAuthor)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CachedDateTime(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedDateTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CachedTarget(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationAnnotationPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_CurrentAnnotationTypeId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CurrentAnnotationTypeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentAuthor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentDateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedAnnotationTypeId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CachedAnnotationTypeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedAuthor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedDateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAutomationAnnotationPattern_Impl: windows_core::IUnknownImpl {
    fn get_CurrentAnnotationTypeId(&self) -> windows_core::Result<i32>;
    fn get_CurrentAnnotationTypeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CurrentAuthor(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CurrentDateTime(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CurrentTarget(&self) -> windows_core::Result<IUIAutomationElement>;
    fn get_CachedAnnotationTypeId(&self) -> windows_core::Result<i32>;
    fn get_CachedAnnotationTypeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedAuthor(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedDateTime(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedTarget(&self) -> windows_core::Result<IUIAutomationElement>;
}
impl IUIAutomationAnnotationPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_CurrentAnnotationTypeId<Identity: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationAnnotationPattern_Impl::get_CurrentAnnotationTypeId(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentAnnotationTypeName<Identity: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationAnnotationPattern_Impl::get_CurrentAnnotationTypeName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentAuthor<Identity: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationAnnotationPattern_Impl::get_CurrentAuthor(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentDateTime<Identity: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationAnnotationPattern_Impl::get_CurrentDateTime(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentTarget<Identity: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationAnnotationPattern_Impl::get_CurrentTarget(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedAnnotationTypeId<Identity: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationAnnotationPattern_Impl::get_CachedAnnotationTypeId(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedAnnotationTypeName<Identity: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationAnnotationPattern_Impl::get_CachedAnnotationTypeName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedAuthor<Identity: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationAnnotationPattern_Impl::get_CachedAuthor(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedDateTime<Identity: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationAnnotationPattern_Impl::get_CachedDateTime(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedTarget<Identity: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationAnnotationPattern_Impl::get_CachedTarget(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_CurrentAnnotationTypeId: get_CurrentAnnotationTypeId::<Identity, OFFSET>,
            get_CurrentAnnotationTypeName: get_CurrentAnnotationTypeName::<Identity, OFFSET>,
            get_CurrentAuthor: get_CurrentAuthor::<Identity, OFFSET>,
            get_CurrentDateTime: get_CurrentDateTime::<Identity, OFFSET>,
            get_CurrentTarget: get_CurrentTarget::<Identity, OFFSET>,
            get_CachedAnnotationTypeId: get_CachedAnnotationTypeId::<Identity, OFFSET>,
            get_CachedAnnotationTypeName: get_CachedAnnotationTypeName::<Identity, OFFSET>,
            get_CachedAuthor: get_CachedAuthor::<Identity, OFFSET>,
            get_CachedDateTime: get_CachedDateTime::<Identity, OFFSET>,
            get_CachedTarget: get_CachedTarget::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationAnnotationPattern as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationAnnotationPattern {}
windows_core::imp::define_interface!(IUIAutomationBoolCondition, IUIAutomationBoolCondition_Vtbl, 0x1b4e1f2e_75eb_4d0b_8952_5a69988e2307);
impl core::ops::Deref for IUIAutomationBoolCondition {
    type Target = IUIAutomationCondition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationBoolCondition, windows_core::IUnknown, IUIAutomationCondition);
impl IUIAutomationBoolCondition {
    pub unsafe fn get_BooleanValue(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BooleanValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationBoolCondition_Vtbl {
    pub base__: IUIAutomationCondition_Vtbl,
    pub get_BooleanValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IUIAutomationBoolCondition_Impl: IUIAutomationCondition_Impl {
    fn get_BooleanValue(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IUIAutomationBoolCondition_Vtbl {
    pub const fn new<Identity: IUIAutomationBoolCondition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_BooleanValue<Identity: IUIAutomationBoolCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, boolval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationBoolCondition_Impl::get_BooleanValue(this) {
                    Ok(ok__) => {
                        boolval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IUIAutomationCondition_Vtbl::new::<Identity, OFFSET>(), get_BooleanValue: get_BooleanValue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationBoolCondition as windows_core::Interface>::IID || iid == &<IUIAutomationCondition as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationBoolCondition {}
windows_core::imp::define_interface!(IUIAutomationCacheRequest, IUIAutomationCacheRequest_Vtbl, 0xb32a92b5_bc25_4078_9c08_d7ee95c48e03);
windows_core::imp::interface_hierarchy!(IUIAutomationCacheRequest, windows_core::IUnknown);
impl IUIAutomationCacheRequest {
    #[cfg(feature = "shared")]
    pub unsafe fn AddProperty(&self, propertyid: super::PROPERTYID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddProperty)(windows_core::Interface::as_raw(self), propertyid) }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn AddPattern(&self, patternid: super::PATTERNID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddPattern)(windows_core::Interface::as_raw(self), patternid) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_TreeScope(&self) -> windows_core::Result<TreeScope> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TreeScope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_TreeScope(&self, scope: TreeScope) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_TreeScope)(windows_core::Interface::as_raw(self), scope) }
    }
    pub unsafe fn get_TreeFilter(&self) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TreeFilter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn put_TreeFilter<P0>(&self, filter: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationCondition>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_TreeFilter)(windows_core::Interface::as_raw(self), filter.param().abi()) }
    }
    pub unsafe fn get_AutomationElementMode(&self) -> windows_core::Result<AutomationElementMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AutomationElementMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_AutomationElementMode(&self, mode: AutomationElementMode) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AutomationElementMode)(windows_core::Interface::as_raw(self), mode) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationCacheRequest_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub AddProperty: unsafe extern "system" fn(*mut core::ffi::c_void, super::PROPERTYID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddProperty: usize,
    #[cfg(feature = "uiautomationcore")]
    pub AddPattern: unsafe extern "system" fn(*mut core::ffi::c_void, super::PATTERNID) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    AddPattern: usize,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_TreeScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TreeScope) -> windows_core::HRESULT,
    pub put_TreeScope: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope) -> windows_core::HRESULT,
    pub get_TreeFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub put_TreeFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_AutomationElementMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutomationElementMode) -> windows_core::HRESULT,
    pub put_AutomationElementMode: unsafe extern "system" fn(*mut core::ffi::c_void, AutomationElementMode) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "uiautomationcore"))]
pub trait IUIAutomationCacheRequest_Impl: windows_core::IUnknownImpl {
    fn AddProperty(&self, propertyid: super::PROPERTYID) -> windows_core::Result<()>;
    fn AddPattern(&self, patternid: super::PATTERNID) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IUIAutomationCacheRequest>;
    fn get_TreeScope(&self) -> windows_core::Result<TreeScope>;
    fn put_TreeScope(&self, scope: TreeScope) -> windows_core::Result<()>;
    fn get_TreeFilter(&self) -> windows_core::Result<IUIAutomationCondition>;
    fn put_TreeFilter(&self, filter: windows_core::Ref<IUIAutomationCondition>) -> windows_core::Result<()>;
    fn get_AutomationElementMode(&self) -> windows_core::Result<AutomationElementMode>;
    fn put_AutomationElementMode(&self, mode: AutomationElementMode) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "uiautomationcore"))]
impl IUIAutomationCacheRequest_Vtbl {
    pub const fn new<Identity: IUIAutomationCacheRequest_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddProperty<Identity: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: super::PROPERTYID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationCacheRequest_Impl::AddProperty(this, core::mem::transmute_copy(&propertyid)).into()
            }
        }
        unsafe extern "system" fn AddPattern<Identity: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, patternid: super::PATTERNID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationCacheRequest_Impl::AddPattern(this, core::mem::transmute_copy(&patternid)).into()
            }
        }
        unsafe extern "system" fn Clone<Identity: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clonedrequest: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationCacheRequest_Impl::Clone(this) {
                    Ok(ok__) => {
                        clonedrequest.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_TreeScope<Identity: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: *mut TreeScope) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationCacheRequest_Impl::get_TreeScope(this) {
                    Ok(ok__) => {
                        scope.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_TreeScope<Identity: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: TreeScope) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationCacheRequest_Impl::put_TreeScope(this, core::mem::transmute_copy(&scope)).into()
            }
        }
        unsafe extern "system" fn get_TreeFilter<Identity: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationCacheRequest_Impl::get_TreeFilter(this) {
                    Ok(ok__) => {
                        filter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_TreeFilter<Identity: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationCacheRequest_Impl::put_TreeFilter(this, core::mem::transmute_copy(&filter)).into()
            }
        }
        unsafe extern "system" fn get_AutomationElementMode<Identity: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: *mut AutomationElementMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationCacheRequest_Impl::get_AutomationElementMode(this) {
                    Ok(ok__) => {
                        mode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_AutomationElementMode<Identity: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: AutomationElementMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationCacheRequest_Impl::put_AutomationElementMode(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddProperty: AddProperty::<Identity, OFFSET>,
            AddPattern: AddPattern::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            get_TreeScope: get_TreeScope::<Identity, OFFSET>,
            put_TreeScope: put_TreeScope::<Identity, OFFSET>,
            get_TreeFilter: get_TreeFilter::<Identity, OFFSET>,
            put_TreeFilter: put_TreeFilter::<Identity, OFFSET>,
            get_AutomationElementMode: get_AutomationElementMode::<Identity, OFFSET>,
            put_AutomationElementMode: put_AutomationElementMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationCacheRequest as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "uiautomationcore"))]
impl windows_core::RuntimeName for IUIAutomationCacheRequest {}
windows_core::imp::define_interface!(IUIAutomationChangesEventHandler, IUIAutomationChangesEventHandler_Vtbl, 0x58edca55_2c3e_4980_b1b9_56c17f27a2a0);
windows_core::imp::interface_hierarchy!(IUIAutomationChangesEventHandler, windows_core::IUnknown);
impl IUIAutomationChangesEventHandler {
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn HandleChangesEvent<P0>(&self, sender: P0, uiachanges: &[super::UiaChangeInfo]) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe { (windows_core::Interface::vtable(self).HandleChangesEvent)(windows_core::Interface::as_raw(self), sender.param().abi(), uiachanges.as_ptr(), uiachanges.len().try_into().unwrap()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationChangesEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub HandleChangesEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::UiaChangeInfo, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    HandleChangesEvent: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationChangesEventHandler_Impl: windows_core::IUnknownImpl {
    fn HandleChangesEvent(&self, sender: windows_core::Ref<IUIAutomationElement>, uiachanges: *const super::UiaChangeInfo, changescount: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationChangesEventHandler_Vtbl {
    pub const fn new<Identity: IUIAutomationChangesEventHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HandleChangesEvent<Identity: IUIAutomationChangesEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, uiachanges: *const super::UiaChangeInfo, changescount: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationChangesEventHandler_Impl::HandleChangesEvent(this, core::mem::transmute_copy(&sender), core::mem::transmute_copy(&uiachanges), core::mem::transmute_copy(&changescount)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HandleChangesEvent: HandleChangesEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationChangesEventHandler as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUIAutomationChangesEventHandler {}
windows_core::imp::define_interface!(IUIAutomationCondition, IUIAutomationCondition_Vtbl, 0x352ffba8_0973_437c_a61f_f64cafd81df9);
windows_core::imp::interface_hierarchy!(IUIAutomationCondition, windows_core::IUnknown);
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationCondition_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait IUIAutomationCondition_Impl: windows_core::IUnknownImpl {}
impl IUIAutomationCondition_Vtbl {
    pub const fn new<Identity: IUIAutomationCondition_Impl, const OFFSET: isize>() -> Self {
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>() }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationCondition as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationCondition {}
windows_core::imp::define_interface!(IUIAutomationCustomNavigationPattern, IUIAutomationCustomNavigationPattern_Vtbl, 0x01ea217a_1766_47ed_a6cc_acf492854b1f);
windows_core::imp::interface_hierarchy!(IUIAutomationCustomNavigationPattern, windows_core::IUnknown);
impl IUIAutomationCustomNavigationPattern {
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn Navigate(&self, direction: super::NavigateDirection) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Navigate)(windows_core::Interface::as_raw(self), direction, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationCustomNavigationPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "uiautomationcore")]
    pub Navigate: unsafe extern "system" fn(*mut core::ffi::c_void, super::NavigateDirection, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    Navigate: usize,
}
#[cfg(feature = "uiautomationcore")]
pub trait IUIAutomationCustomNavigationPattern_Impl: windows_core::IUnknownImpl {
    fn Navigate(&self, direction: super::NavigateDirection) -> windows_core::Result<IUIAutomationElement>;
}
#[cfg(feature = "uiautomationcore")]
impl IUIAutomationCustomNavigationPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationCustomNavigationPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Navigate<Identity: IUIAutomationCustomNavigationPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, direction: super::NavigateDirection, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationCustomNavigationPattern_Impl::Navigate(this, core::mem::transmute_copy(&direction)) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Navigate: Navigate::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationCustomNavigationPattern as windows_core::Interface>::IID
    }
}
#[cfg(feature = "uiautomationcore")]
impl windows_core::RuntimeName for IUIAutomationCustomNavigationPattern {}
windows_core::imp::define_interface!(IUIAutomationDockPattern, IUIAutomationDockPattern_Vtbl, 0xfde5ef97_1464_48f6_90bf_43d0948e86ec);
windows_core::imp::interface_hierarchy!(IUIAutomationDockPattern, windows_core::IUnknown);
impl IUIAutomationDockPattern {
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn SetDockPosition(&self, dockpos: super::DockPosition) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDockPosition)(windows_core::Interface::as_raw(self), dockpos) }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn get_CurrentDockPosition(&self) -> windows_core::Result<super::DockPosition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentDockPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn get_CachedDockPosition(&self) -> windows_core::Result<super::DockPosition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedDockPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationDockPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "uiautomationcore")]
    pub SetDockPosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::DockPosition) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    SetDockPosition: usize,
    #[cfg(feature = "uiautomationcore")]
    pub get_CurrentDockPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DockPosition) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    get_CurrentDockPosition: usize,
    #[cfg(feature = "uiautomationcore")]
    pub get_CachedDockPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DockPosition) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    get_CachedDockPosition: usize,
}
#[cfg(feature = "uiautomationcore")]
pub trait IUIAutomationDockPattern_Impl: windows_core::IUnknownImpl {
    fn SetDockPosition(&self, dockpos: super::DockPosition) -> windows_core::Result<()>;
    fn get_CurrentDockPosition(&self) -> windows_core::Result<super::DockPosition>;
    fn get_CachedDockPosition(&self) -> windows_core::Result<super::DockPosition>;
}
#[cfg(feature = "uiautomationcore")]
impl IUIAutomationDockPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationDockPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDockPosition<Identity: IUIAutomationDockPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dockpos: super::DockPosition) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationDockPattern_Impl::SetDockPosition(this, core::mem::transmute_copy(&dockpos)).into()
            }
        }
        unsafe extern "system" fn get_CurrentDockPosition<Identity: IUIAutomationDockPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DockPosition) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationDockPattern_Impl::get_CurrentDockPosition(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedDockPosition<Identity: IUIAutomationDockPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::DockPosition) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationDockPattern_Impl::get_CachedDockPosition(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDockPosition: SetDockPosition::<Identity, OFFSET>,
            get_CurrentDockPosition: get_CurrentDockPosition::<Identity, OFFSET>,
            get_CachedDockPosition: get_CachedDockPosition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationDockPattern as windows_core::Interface>::IID
    }
}
#[cfg(feature = "uiautomationcore")]
impl windows_core::RuntimeName for IUIAutomationDockPattern {}
windows_core::imp::define_interface!(IUIAutomationDragPattern, IUIAutomationDragPattern_Vtbl, 0x1dc7b570_1f54_4bad_bcda_d36a722fb7bd);
windows_core::imp::interface_hierarchy!(IUIAutomationDragPattern, windows_core::IUnknown);
impl IUIAutomationDragPattern {
    pub unsafe fn get_CurrentIsGrabbed(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentIsGrabbed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedIsGrabbed(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedIsGrabbed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentDropEffect(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentDropEffect)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CachedDropEffect(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedDropEffect)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn get_CurrentDropEffects(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentDropEffects)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn get_CachedDropEffects(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedDropEffects)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCurrentGrabbedItems(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentGrabbedItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCachedGrabbedItems(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedGrabbedItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationDragPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_CurrentIsGrabbed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CachedIsGrabbed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CurrentDropEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedDropEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub get_CurrentDropEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    get_CurrentDropEffects: usize,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub get_CachedDropEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    get_CachedDropEffects: usize,
    pub GetCurrentGrabbedItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCachedGrabbedItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
pub trait IUIAutomationDragPattern_Impl: windows_core::IUnknownImpl {
    fn get_CurrentIsGrabbed(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedIsGrabbed(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CurrentDropEffect(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedDropEffect(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CurrentDropEffects(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn get_CachedDropEffects(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn GetCurrentGrabbedItems(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn GetCachedGrabbedItems(&self) -> windows_core::Result<IUIAutomationElementArray>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl IUIAutomationDragPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationDragPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_CurrentIsGrabbed<Identity: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationDragPattern_Impl::get_CurrentIsGrabbed(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedIsGrabbed<Identity: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationDragPattern_Impl::get_CachedIsGrabbed(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentDropEffect<Identity: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationDragPattern_Impl::get_CurrentDropEffect(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedDropEffect<Identity: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationDragPattern_Impl::get_CachedDropEffect(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentDropEffects<Identity: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationDragPattern_Impl::get_CurrentDropEffects(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedDropEffects<Identity: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationDragPattern_Impl::get_CachedDropEffects(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentGrabbedItems<Identity: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationDragPattern_Impl::GetCurrentGrabbedItems(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCachedGrabbedItems<Identity: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationDragPattern_Impl::GetCachedGrabbedItems(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_CurrentIsGrabbed: get_CurrentIsGrabbed::<Identity, OFFSET>,
            get_CachedIsGrabbed: get_CachedIsGrabbed::<Identity, OFFSET>,
            get_CurrentDropEffect: get_CurrentDropEffect::<Identity, OFFSET>,
            get_CachedDropEffect: get_CachedDropEffect::<Identity, OFFSET>,
            get_CurrentDropEffects: get_CurrentDropEffects::<Identity, OFFSET>,
            get_CachedDropEffects: get_CachedDropEffects::<Identity, OFFSET>,
            GetCurrentGrabbedItems: GetCurrentGrabbedItems::<Identity, OFFSET>,
            GetCachedGrabbedItems: GetCachedGrabbedItems::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationDragPattern as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IUIAutomationDragPattern {}
windows_core::imp::define_interface!(IUIAutomationDropTargetPattern, IUIAutomationDropTargetPattern_Vtbl, 0x69a095f7_eee4_430e_a46b_fb73b1ae39a5);
windows_core::imp::interface_hierarchy!(IUIAutomationDropTargetPattern, windows_core::IUnknown);
impl IUIAutomationDropTargetPattern {
    pub unsafe fn get_CurrentDropTargetEffect(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentDropTargetEffect)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CachedDropTargetEffect(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedDropTargetEffect)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn get_CurrentDropTargetEffects(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentDropTargetEffects)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn get_CachedDropTargetEffects(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedDropTargetEffects)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationDropTargetPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_CurrentDropTargetEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedDropTargetEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub get_CurrentDropTargetEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    get_CurrentDropTargetEffects: usize,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub get_CachedDropTargetEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    get_CachedDropTargetEffects: usize,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
pub trait IUIAutomationDropTargetPattern_Impl: windows_core::IUnknownImpl {
    fn get_CurrentDropTargetEffect(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedDropTargetEffect(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CurrentDropTargetEffects(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn get_CachedDropTargetEffects(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl IUIAutomationDropTargetPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationDropTargetPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_CurrentDropTargetEffect<Identity: IUIAutomationDropTargetPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationDropTargetPattern_Impl::get_CurrentDropTargetEffect(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedDropTargetEffect<Identity: IUIAutomationDropTargetPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationDropTargetPattern_Impl::get_CachedDropTargetEffect(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentDropTargetEffects<Identity: IUIAutomationDropTargetPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationDropTargetPattern_Impl::get_CurrentDropTargetEffects(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedDropTargetEffects<Identity: IUIAutomationDropTargetPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationDropTargetPattern_Impl::get_CachedDropTargetEffects(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_CurrentDropTargetEffect: get_CurrentDropTargetEffect::<Identity, OFFSET>,
            get_CachedDropTargetEffect: get_CachedDropTargetEffect::<Identity, OFFSET>,
            get_CurrentDropTargetEffects: get_CurrentDropTargetEffects::<Identity, OFFSET>,
            get_CachedDropTargetEffects: get_CachedDropTargetEffects::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationDropTargetPattern as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IUIAutomationDropTargetPattern {}
windows_core::imp::define_interface!(IUIAutomationElement, IUIAutomationElement_Vtbl, 0xd22108aa_8ac5_49a5_837b_37bbb3d7591e);
windows_core::imp::interface_hierarchy!(IUIAutomationElement, windows_core::IUnknown);
impl IUIAutomationElement {
    pub unsafe fn SetFocus(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFocus)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetRuntimeId(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRuntimeId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FindFirst<P1>(&self, scope: TreeScope, condition: P1) -> windows_core::Result<Self>
    where
        P1: windows_core::Param<IUIAutomationCondition>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindFirst)(windows_core::Interface::as_raw(self), scope, condition.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindAll<P1>(&self, scope: TreeScope, condition: P1) -> windows_core::Result<IUIAutomationElementArray>
    where
        P1: windows_core::Param<IUIAutomationCondition>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindAll)(windows_core::Interface::as_raw(self), scope, condition.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindFirstBuildCache<P1, P2>(&self, scope: TreeScope, condition: P1, cacherequest: P2) -> windows_core::Result<Self>
    where
        P1: windows_core::Param<IUIAutomationCondition>,
        P2: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindFirstBuildCache)(windows_core::Interface::as_raw(self), scope, condition.param().abi(), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindAllBuildCache<P1, P2>(&self, scope: TreeScope, condition: P1, cacherequest: P2) -> windows_core::Result<IUIAutomationElementArray>
    where
        P1: windows_core::Param<IUIAutomationCondition>,
        P2: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindAllBuildCache)(windows_core::Interface::as_raw(self), scope, condition.param().abi(), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn BuildUpdatedCache<P0>(&self, cacherequest: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BuildUpdatedCache)(windows_core::Interface::as_raw(self), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetCurrentPropertyValue(&self, propertyid: super::PROPERTYID) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentPropertyValue)(windows_core::Interface::as_raw(self), propertyid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetCurrentPropertyValueEx(&self, propertyid: super::PROPERTYID, ignoredefaultvalue: bool) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentPropertyValueEx)(windows_core::Interface::as_raw(self), propertyid, ignoredefaultvalue.into(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetCachedPropertyValue(&self, propertyid: super::PROPERTYID) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedPropertyValue)(windows_core::Interface::as_raw(self), propertyid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetCachedPropertyValueEx(&self, propertyid: super::PROPERTYID, ignoredefaultvalue: bool) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedPropertyValueEx)(windows_core::Interface::as_raw(self), propertyid, ignoredefaultvalue.into(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "guiddef", feature = "uiautomationcore"))]
    pub unsafe fn GetCurrentPatternAs<T>(&self, patternid: super::PATTERNID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetCurrentPatternAs)(windows_core::Interface::as_raw(self), patternid, &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
    }
    #[cfg(all(feature = "guiddef", feature = "uiautomationcore"))]
    pub unsafe fn GetCachedPatternAs<T>(&self, patternid: super::PATTERNID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetCachedPatternAs)(windows_core::Interface::as_raw(self), patternid, &T::IID, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__)) }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn GetCurrentPattern(&self, patternid: super::PATTERNID) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentPattern)(windows_core::Interface::as_raw(self), patternid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn GetCachedPattern(&self, patternid: super::PATTERNID) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedPattern)(windows_core::Interface::as_raw(self), patternid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCachedParent(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedParent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCachedChildren(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedChildren)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CurrentProcessId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentProcessId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn get_CurrentControlType(&self) -> windows_core::Result<super::CONTROLTYPEID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentControlType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentLocalizedControlType(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentLocalizedControlType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CurrentName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CurrentAcceleratorKey(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentAcceleratorKey)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CurrentAccessKey(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentAccessKey)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CurrentHasKeyboardFocus(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentHasKeyboardFocus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentIsKeyboardFocusable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentIsKeyboardFocusable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentIsEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentIsEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentAutomationId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentAutomationId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CurrentClassName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentClassName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CurrentHelpText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentHelpText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CurrentCulture(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentCulture)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentIsControlElement(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentIsControlElement)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentIsContentElement(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentIsContentElement)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentIsPassword(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentIsPassword)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentNativeWindowHandle(&self) -> windows_core::Result<UIA_HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentNativeWindowHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentItemType(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentItemType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CurrentIsOffscreen(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentIsOffscreen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn get_CurrentOrientation(&self) -> windows_core::Result<super::OrientationType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentOrientation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentFrameworkId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentFrameworkId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CurrentIsRequiredForForm(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentIsRequiredForForm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentItemStatus(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentItemStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn get_CurrentBoundingRectangle(&self) -> windows_core::Result<super::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentBoundingRectangle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentLabeledBy(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentLabeledBy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CurrentAriaRole(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentAriaRole)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CurrentAriaProperties(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentAriaProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CurrentIsDataValidForForm(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentIsDataValidForForm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentControllerFor(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentControllerFor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CurrentDescribedBy(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentDescribedBy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CurrentFlowsTo(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentFlowsTo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CurrentProviderDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentProviderDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CachedProcessId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedProcessId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn get_CachedControlType(&self) -> windows_core::Result<super::CONTROLTYPEID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedControlType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedLocalizedControlType(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedLocalizedControlType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CachedName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CachedAcceleratorKey(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedAcceleratorKey)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CachedAccessKey(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedAccessKey)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CachedHasKeyboardFocus(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedHasKeyboardFocus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedIsKeyboardFocusable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedIsKeyboardFocusable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedIsEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedIsEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedAutomationId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedAutomationId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CachedClassName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedClassName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CachedHelpText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedHelpText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CachedCulture(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedCulture)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedIsControlElement(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedIsControlElement)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedIsContentElement(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedIsContentElement)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedIsPassword(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedIsPassword)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedNativeWindowHandle(&self) -> windows_core::Result<UIA_HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedNativeWindowHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedItemType(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedItemType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CachedIsOffscreen(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedIsOffscreen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn get_CachedOrientation(&self) -> windows_core::Result<super::OrientationType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedOrientation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedFrameworkId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedFrameworkId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CachedIsRequiredForForm(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedIsRequiredForForm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedItemStatus(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedItemStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn get_CachedBoundingRectangle(&self) -> windows_core::Result<super::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedBoundingRectangle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedLabeledBy(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedLabeledBy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CachedAriaRole(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedAriaRole)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CachedAriaProperties(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedAriaProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CachedIsDataValidForForm(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedIsDataValidForForm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedControllerFor(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedControllerFor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CachedDescribedBy(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedDescribedBy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CachedFlowsTo(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedFlowsTo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CachedProviderDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedProviderDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn GetClickablePoint(&self, clickable: *mut super::POINT) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetClickablePoint)(windows_core::Interface::as_raw(self), clickable as _, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElement_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetFocus: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetRuntimeId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetRuntimeId: usize,
    pub FindFirst: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindAll: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindFirstBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindAllBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BuildUpdatedCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetCurrentPropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::PROPERTYID, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetCurrentPropertyValue: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetCurrentPropertyValueEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::PROPERTYID, windows_core::BOOL, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetCurrentPropertyValueEx: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetCachedPropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::PROPERTYID, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetCachedPropertyValue: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetCachedPropertyValueEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::PROPERTYID, windows_core::BOOL, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetCachedPropertyValueEx: usize,
    #[cfg(all(feature = "guiddef", feature = "uiautomationcore"))]
    pub GetCurrentPatternAs: unsafe extern "system" fn(*mut core::ffi::c_void, super::PATTERNID, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "uiautomationcore")))]
    GetCurrentPatternAs: usize,
    #[cfg(all(feature = "guiddef", feature = "uiautomationcore"))]
    pub GetCachedPatternAs: unsafe extern "system" fn(*mut core::ffi::c_void, super::PATTERNID, *const super::IID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "guiddef", feature = "uiautomationcore")))]
    GetCachedPatternAs: usize,
    #[cfg(feature = "uiautomationcore")]
    pub GetCurrentPattern: unsafe extern "system" fn(*mut core::ffi::c_void, super::PATTERNID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    GetCurrentPattern: usize,
    #[cfg(feature = "uiautomationcore")]
    pub GetCachedPattern: unsafe extern "system" fn(*mut core::ffi::c_void, super::PATTERNID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    GetCachedPattern: usize,
    pub GetCachedParent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCachedChildren: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentProcessId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub get_CurrentControlType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::CONTROLTYPEID) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    get_CurrentControlType: usize,
    pub get_CurrentLocalizedControlType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentAcceleratorKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentAccessKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentHasKeyboardFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CurrentIsKeyboardFocusable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CurrentIsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CurrentAutomationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentClassName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentHelpText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentCulture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CurrentIsControlElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CurrentIsContentElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CurrentIsPassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CurrentNativeWindowHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UIA_HWND) -> windows_core::HRESULT,
    pub get_CurrentItemType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentIsOffscreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub get_CurrentOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::OrientationType) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    get_CurrentOrientation: usize,
    pub get_CurrentFrameworkId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentIsRequiredForForm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CurrentItemStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub get_CurrentBoundingRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    get_CurrentBoundingRectangle: usize,
    pub get_CurrentLabeledBy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentAriaRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentAriaProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentIsDataValidForForm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CurrentControllerFor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentDescribedBy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentFlowsTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentProviderDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedProcessId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub get_CachedControlType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::CONTROLTYPEID) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    get_CachedControlType: usize,
    pub get_CachedLocalizedControlType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedAcceleratorKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedAccessKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedHasKeyboardFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CachedIsKeyboardFocusable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CachedIsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CachedAutomationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedClassName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedHelpText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedCulture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CachedIsControlElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CachedIsContentElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CachedIsPassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CachedNativeWindowHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UIA_HWND) -> windows_core::HRESULT,
    pub get_CachedItemType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedIsOffscreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub get_CachedOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::OrientationType) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    get_CachedOrientation: usize,
    pub get_CachedFrameworkId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedIsRequiredForForm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CachedItemStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub get_CachedBoundingRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    get_CachedBoundingRectangle: usize,
    pub get_CachedLabeledBy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedAriaRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedAriaProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedIsDataValidForForm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CachedControllerFor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedDescribedBy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedFlowsTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedProviderDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub GetClickablePoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::POINT, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    GetClickablePoint: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationElement_Impl: windows_core::IUnknownImpl {
    fn SetFocus(&self) -> windows_core::Result<()>;
    fn GetRuntimeId(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn FindFirst(&self, scope: TreeScope, condition: windows_core::Ref<IUIAutomationCondition>) -> windows_core::Result<IUIAutomationElement>;
    fn FindAll(&self, scope: TreeScope, condition: windows_core::Ref<IUIAutomationCondition>) -> windows_core::Result<IUIAutomationElementArray>;
    fn FindFirstBuildCache(&self, scope: TreeScope, condition: windows_core::Ref<IUIAutomationCondition>, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>) -> windows_core::Result<IUIAutomationElement>;
    fn FindAllBuildCache(&self, scope: TreeScope, condition: windows_core::Ref<IUIAutomationCondition>, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>) -> windows_core::Result<IUIAutomationElementArray>;
    fn BuildUpdatedCache(&self, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>) -> windows_core::Result<IUIAutomationElement>;
    fn GetCurrentPropertyValue(&self, propertyid: super::PROPERTYID) -> windows_core::Result<super::VARIANT>;
    fn GetCurrentPropertyValueEx(&self, propertyid: super::PROPERTYID, ignoredefaultvalue: windows_core::BOOL) -> windows_core::Result<super::VARIANT>;
    fn GetCachedPropertyValue(&self, propertyid: super::PROPERTYID) -> windows_core::Result<super::VARIANT>;
    fn GetCachedPropertyValueEx(&self, propertyid: super::PROPERTYID, ignoredefaultvalue: windows_core::BOOL) -> windows_core::Result<super::VARIANT>;
    fn GetCurrentPatternAs(&self, patternid: super::PATTERNID, riid: *const super::IID, patternobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetCachedPatternAs(&self, patternid: super::PATTERNID, riid: *const super::IID, patternobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetCurrentPattern(&self, patternid: super::PATTERNID) -> windows_core::Result<windows_core::IUnknown>;
    fn GetCachedPattern(&self, patternid: super::PATTERNID) -> windows_core::Result<windows_core::IUnknown>;
    fn GetCachedParent(&self) -> windows_core::Result<IUIAutomationElement>;
    fn GetCachedChildren(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn get_CurrentProcessId(&self) -> windows_core::Result<i32>;
    fn get_CurrentControlType(&self) -> windows_core::Result<super::CONTROLTYPEID>;
    fn get_CurrentLocalizedControlType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CurrentName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CurrentAcceleratorKey(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CurrentAccessKey(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CurrentHasKeyboardFocus(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CurrentIsKeyboardFocusable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CurrentIsEnabled(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CurrentAutomationId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CurrentClassName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CurrentHelpText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CurrentCulture(&self) -> windows_core::Result<i32>;
    fn get_CurrentIsControlElement(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CurrentIsContentElement(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CurrentIsPassword(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CurrentNativeWindowHandle(&self) -> windows_core::Result<UIA_HWND>;
    fn get_CurrentItemType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CurrentIsOffscreen(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CurrentOrientation(&self) -> windows_core::Result<super::OrientationType>;
    fn get_CurrentFrameworkId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CurrentIsRequiredForForm(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CurrentItemStatus(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CurrentBoundingRectangle(&self) -> windows_core::Result<super::RECT>;
    fn get_CurrentLabeledBy(&self) -> windows_core::Result<IUIAutomationElement>;
    fn get_CurrentAriaRole(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CurrentAriaProperties(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CurrentIsDataValidForForm(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CurrentControllerFor(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn get_CurrentDescribedBy(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn get_CurrentFlowsTo(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn get_CurrentProviderDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedProcessId(&self) -> windows_core::Result<i32>;
    fn get_CachedControlType(&self) -> windows_core::Result<super::CONTROLTYPEID>;
    fn get_CachedLocalizedControlType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedAcceleratorKey(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedAccessKey(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedHasKeyboardFocus(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedIsKeyboardFocusable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedIsEnabled(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedAutomationId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedClassName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedHelpText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedCulture(&self) -> windows_core::Result<i32>;
    fn get_CachedIsControlElement(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedIsContentElement(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedIsPassword(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedNativeWindowHandle(&self) -> windows_core::Result<UIA_HWND>;
    fn get_CachedItemType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedIsOffscreen(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedOrientation(&self) -> windows_core::Result<super::OrientationType>;
    fn get_CachedFrameworkId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedIsRequiredForForm(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedItemStatus(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedBoundingRectangle(&self) -> windows_core::Result<super::RECT>;
    fn get_CachedLabeledBy(&self) -> windows_core::Result<IUIAutomationElement>;
    fn get_CachedAriaRole(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedAriaProperties(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedIsDataValidForForm(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedControllerFor(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn get_CachedDescribedBy(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn get_CachedFlowsTo(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn get_CachedProviderDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetClickablePoint(&self, clickable: *mut super::POINT) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationElement_Vtbl {
    pub const fn new<Identity: IUIAutomationElement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetFocus<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationElement_Impl::SetFocus(this).into()
            }
        }
        unsafe extern "system" fn GetRuntimeId<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runtimeid: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::GetRuntimeId(this) {
                    Ok(ok__) => {
                        runtimeid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindFirst<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: TreeScope, condition: *mut core::ffi::c_void, found: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::FindFirst(this, core::mem::transmute_copy(&scope), core::mem::transmute_copy(&condition)) {
                    Ok(ok__) => {
                        found.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindAll<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: TreeScope, condition: *mut core::ffi::c_void, found: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::FindAll(this, core::mem::transmute_copy(&scope), core::mem::transmute_copy(&condition)) {
                    Ok(ok__) => {
                        found.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindFirstBuildCache<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: TreeScope, condition: *mut core::ffi::c_void, cacherequest: *mut core::ffi::c_void, found: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::FindFirstBuildCache(this, core::mem::transmute_copy(&scope), core::mem::transmute_copy(&condition), core::mem::transmute_copy(&cacherequest)) {
                    Ok(ok__) => {
                        found.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindAllBuildCache<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: TreeScope, condition: *mut core::ffi::c_void, cacherequest: *mut core::ffi::c_void, found: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::FindAllBuildCache(this, core::mem::transmute_copy(&scope), core::mem::transmute_copy(&condition), core::mem::transmute_copy(&cacherequest)) {
                    Ok(ok__) => {
                        found.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn BuildUpdatedCache<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cacherequest: *mut core::ffi::c_void, updatedelement: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::BuildUpdatedCache(this, core::mem::transmute_copy(&cacherequest)) {
                    Ok(ok__) => {
                        updatedelement.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentPropertyValue<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: super::PROPERTYID, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::GetCurrentPropertyValue(this, core::mem::transmute_copy(&propertyid)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentPropertyValueEx<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: super::PROPERTYID, ignoredefaultvalue: windows_core::BOOL, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::GetCurrentPropertyValueEx(this, core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&ignoredefaultvalue)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCachedPropertyValue<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: super::PROPERTYID, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::GetCachedPropertyValue(this, core::mem::transmute_copy(&propertyid)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCachedPropertyValueEx<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: super::PROPERTYID, ignoredefaultvalue: windows_core::BOOL, retval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::GetCachedPropertyValueEx(this, core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&ignoredefaultvalue)) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentPatternAs<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, patternid: super::PATTERNID, riid: *const super::IID, patternobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationElement_Impl::GetCurrentPatternAs(this, core::mem::transmute_copy(&patternid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&patternobject)).into()
            }
        }
        unsafe extern "system" fn GetCachedPatternAs<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, patternid: super::PATTERNID, riid: *const super::IID, patternobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationElement_Impl::GetCachedPatternAs(this, core::mem::transmute_copy(&patternid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&patternobject)).into()
            }
        }
        unsafe extern "system" fn GetCurrentPattern<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, patternid: super::PATTERNID, patternobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::GetCurrentPattern(this, core::mem::transmute_copy(&patternid)) {
                    Ok(ok__) => {
                        patternobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCachedPattern<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, patternid: super::PATTERNID, patternobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::GetCachedPattern(this, core::mem::transmute_copy(&patternid)) {
                    Ok(ok__) => {
                        patternobject.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCachedParent<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, parent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::GetCachedParent(this) {
                    Ok(ok__) => {
                        parent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCachedChildren<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, children: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::GetCachedChildren(this) {
                    Ok(ok__) => {
                        children.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentProcessId<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentProcessId(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentControlType<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::CONTROLTYPEID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentControlType(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentLocalizedControlType<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentLocalizedControlType(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentName<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentAcceleratorKey<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentAcceleratorKey(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentAccessKey<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentAccessKey(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentHasKeyboardFocus<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentHasKeyboardFocus(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentIsKeyboardFocusable<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentIsKeyboardFocusable(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentIsEnabled<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentIsEnabled(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentAutomationId<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentAutomationId(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentClassName<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentClassName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentHelpText<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentHelpText(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentCulture<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentCulture(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentIsControlElement<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentIsControlElement(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentIsContentElement<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentIsContentElement(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentIsPassword<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentIsPassword(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentNativeWindowHandle<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut UIA_HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentNativeWindowHandle(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentItemType<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentItemType(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentIsOffscreen<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentIsOffscreen(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentOrientation<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::OrientationType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentOrientation(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentFrameworkId<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentFrameworkId(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentIsRequiredForForm<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentIsRequiredForForm(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentItemStatus<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentItemStatus(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentBoundingRectangle<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentBoundingRectangle(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentLabeledBy<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentLabeledBy(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentAriaRole<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentAriaRole(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentAriaProperties<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentAriaProperties(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentIsDataValidForForm<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentIsDataValidForForm(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentControllerFor<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentControllerFor(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentDescribedBy<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentDescribedBy(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentFlowsTo<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentFlowsTo(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentProviderDescription<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CurrentProviderDescription(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedProcessId<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedProcessId(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedControlType<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::CONTROLTYPEID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedControlType(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedLocalizedControlType<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedLocalizedControlType(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedName<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedAcceleratorKey<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedAcceleratorKey(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedAccessKey<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedAccessKey(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedHasKeyboardFocus<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedHasKeyboardFocus(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedIsKeyboardFocusable<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedIsKeyboardFocusable(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedIsEnabled<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedIsEnabled(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedAutomationId<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedAutomationId(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedClassName<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedClassName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedHelpText<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedHelpText(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedCulture<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedCulture(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedIsControlElement<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedIsControlElement(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedIsContentElement<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedIsContentElement(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedIsPassword<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedIsPassword(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedNativeWindowHandle<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut UIA_HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedNativeWindowHandle(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedItemType<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedItemType(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedIsOffscreen<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedIsOffscreen(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedOrientation<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::OrientationType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedOrientation(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedFrameworkId<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedFrameworkId(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedIsRequiredForForm<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedIsRequiredForForm(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedItemStatus<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedItemStatus(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedBoundingRectangle<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedBoundingRectangle(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedLabeledBy<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedLabeledBy(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedAriaRole<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedAriaRole(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedAriaProperties<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedAriaProperties(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedIsDataValidForForm<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedIsDataValidForForm(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedControllerFor<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedControllerFor(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedDescribedBy<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedDescribedBy(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedFlowsTo<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedFlowsTo(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedProviderDescription<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::get_CachedProviderDescription(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetClickablePoint<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clickable: *mut super::POINT, gotclickable: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::GetClickablePoint(this, core::mem::transmute_copy(&clickable)) {
                    Ok(ok__) => {
                        gotclickable.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetFocus: SetFocus::<Identity, OFFSET>,
            GetRuntimeId: GetRuntimeId::<Identity, OFFSET>,
            FindFirst: FindFirst::<Identity, OFFSET>,
            FindAll: FindAll::<Identity, OFFSET>,
            FindFirstBuildCache: FindFirstBuildCache::<Identity, OFFSET>,
            FindAllBuildCache: FindAllBuildCache::<Identity, OFFSET>,
            BuildUpdatedCache: BuildUpdatedCache::<Identity, OFFSET>,
            GetCurrentPropertyValue: GetCurrentPropertyValue::<Identity, OFFSET>,
            GetCurrentPropertyValueEx: GetCurrentPropertyValueEx::<Identity, OFFSET>,
            GetCachedPropertyValue: GetCachedPropertyValue::<Identity, OFFSET>,
            GetCachedPropertyValueEx: GetCachedPropertyValueEx::<Identity, OFFSET>,
            GetCurrentPatternAs: GetCurrentPatternAs::<Identity, OFFSET>,
            GetCachedPatternAs: GetCachedPatternAs::<Identity, OFFSET>,
            GetCurrentPattern: GetCurrentPattern::<Identity, OFFSET>,
            GetCachedPattern: GetCachedPattern::<Identity, OFFSET>,
            GetCachedParent: GetCachedParent::<Identity, OFFSET>,
            GetCachedChildren: GetCachedChildren::<Identity, OFFSET>,
            get_CurrentProcessId: get_CurrentProcessId::<Identity, OFFSET>,
            get_CurrentControlType: get_CurrentControlType::<Identity, OFFSET>,
            get_CurrentLocalizedControlType: get_CurrentLocalizedControlType::<Identity, OFFSET>,
            get_CurrentName: get_CurrentName::<Identity, OFFSET>,
            get_CurrentAcceleratorKey: get_CurrentAcceleratorKey::<Identity, OFFSET>,
            get_CurrentAccessKey: get_CurrentAccessKey::<Identity, OFFSET>,
            get_CurrentHasKeyboardFocus: get_CurrentHasKeyboardFocus::<Identity, OFFSET>,
            get_CurrentIsKeyboardFocusable: get_CurrentIsKeyboardFocusable::<Identity, OFFSET>,
            get_CurrentIsEnabled: get_CurrentIsEnabled::<Identity, OFFSET>,
            get_CurrentAutomationId: get_CurrentAutomationId::<Identity, OFFSET>,
            get_CurrentClassName: get_CurrentClassName::<Identity, OFFSET>,
            get_CurrentHelpText: get_CurrentHelpText::<Identity, OFFSET>,
            get_CurrentCulture: get_CurrentCulture::<Identity, OFFSET>,
            get_CurrentIsControlElement: get_CurrentIsControlElement::<Identity, OFFSET>,
            get_CurrentIsContentElement: get_CurrentIsContentElement::<Identity, OFFSET>,
            get_CurrentIsPassword: get_CurrentIsPassword::<Identity, OFFSET>,
            get_CurrentNativeWindowHandle: get_CurrentNativeWindowHandle::<Identity, OFFSET>,
            get_CurrentItemType: get_CurrentItemType::<Identity, OFFSET>,
            get_CurrentIsOffscreen: get_CurrentIsOffscreen::<Identity, OFFSET>,
            get_CurrentOrientation: get_CurrentOrientation::<Identity, OFFSET>,
            get_CurrentFrameworkId: get_CurrentFrameworkId::<Identity, OFFSET>,
            get_CurrentIsRequiredForForm: get_CurrentIsRequiredForForm::<Identity, OFFSET>,
            get_CurrentItemStatus: get_CurrentItemStatus::<Identity, OFFSET>,
            get_CurrentBoundingRectangle: get_CurrentBoundingRectangle::<Identity, OFFSET>,
            get_CurrentLabeledBy: get_CurrentLabeledBy::<Identity, OFFSET>,
            get_CurrentAriaRole: get_CurrentAriaRole::<Identity, OFFSET>,
            get_CurrentAriaProperties: get_CurrentAriaProperties::<Identity, OFFSET>,
            get_CurrentIsDataValidForForm: get_CurrentIsDataValidForForm::<Identity, OFFSET>,
            get_CurrentControllerFor: get_CurrentControllerFor::<Identity, OFFSET>,
            get_CurrentDescribedBy: get_CurrentDescribedBy::<Identity, OFFSET>,
            get_CurrentFlowsTo: get_CurrentFlowsTo::<Identity, OFFSET>,
            get_CurrentProviderDescription: get_CurrentProviderDescription::<Identity, OFFSET>,
            get_CachedProcessId: get_CachedProcessId::<Identity, OFFSET>,
            get_CachedControlType: get_CachedControlType::<Identity, OFFSET>,
            get_CachedLocalizedControlType: get_CachedLocalizedControlType::<Identity, OFFSET>,
            get_CachedName: get_CachedName::<Identity, OFFSET>,
            get_CachedAcceleratorKey: get_CachedAcceleratorKey::<Identity, OFFSET>,
            get_CachedAccessKey: get_CachedAccessKey::<Identity, OFFSET>,
            get_CachedHasKeyboardFocus: get_CachedHasKeyboardFocus::<Identity, OFFSET>,
            get_CachedIsKeyboardFocusable: get_CachedIsKeyboardFocusable::<Identity, OFFSET>,
            get_CachedIsEnabled: get_CachedIsEnabled::<Identity, OFFSET>,
            get_CachedAutomationId: get_CachedAutomationId::<Identity, OFFSET>,
            get_CachedClassName: get_CachedClassName::<Identity, OFFSET>,
            get_CachedHelpText: get_CachedHelpText::<Identity, OFFSET>,
            get_CachedCulture: get_CachedCulture::<Identity, OFFSET>,
            get_CachedIsControlElement: get_CachedIsControlElement::<Identity, OFFSET>,
            get_CachedIsContentElement: get_CachedIsContentElement::<Identity, OFFSET>,
            get_CachedIsPassword: get_CachedIsPassword::<Identity, OFFSET>,
            get_CachedNativeWindowHandle: get_CachedNativeWindowHandle::<Identity, OFFSET>,
            get_CachedItemType: get_CachedItemType::<Identity, OFFSET>,
            get_CachedIsOffscreen: get_CachedIsOffscreen::<Identity, OFFSET>,
            get_CachedOrientation: get_CachedOrientation::<Identity, OFFSET>,
            get_CachedFrameworkId: get_CachedFrameworkId::<Identity, OFFSET>,
            get_CachedIsRequiredForForm: get_CachedIsRequiredForForm::<Identity, OFFSET>,
            get_CachedItemStatus: get_CachedItemStatus::<Identity, OFFSET>,
            get_CachedBoundingRectangle: get_CachedBoundingRectangle::<Identity, OFFSET>,
            get_CachedLabeledBy: get_CachedLabeledBy::<Identity, OFFSET>,
            get_CachedAriaRole: get_CachedAriaRole::<Identity, OFFSET>,
            get_CachedAriaProperties: get_CachedAriaProperties::<Identity, OFFSET>,
            get_CachedIsDataValidForForm: get_CachedIsDataValidForForm::<Identity, OFFSET>,
            get_CachedControllerFor: get_CachedControllerFor::<Identity, OFFSET>,
            get_CachedDescribedBy: get_CachedDescribedBy::<Identity, OFFSET>,
            get_CachedFlowsTo: get_CachedFlowsTo::<Identity, OFFSET>,
            get_CachedProviderDescription: get_CachedProviderDescription::<Identity, OFFSET>,
            GetClickablePoint: GetClickablePoint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUIAutomationElement {}
windows_core::imp::define_interface!(IUIAutomationElement2, IUIAutomationElement2_Vtbl, 0x6749c683_f70d_4487_a698_5f79d55290d6);
impl core::ops::Deref for IUIAutomationElement2 {
    type Target = IUIAutomationElement;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationElement2, windows_core::IUnknown, IUIAutomationElement);
impl IUIAutomationElement2 {
    pub unsafe fn get_CurrentOptimizeForVisualContent(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentOptimizeForVisualContent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedOptimizeForVisualContent(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedOptimizeForVisualContent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn get_CurrentLiveSetting(&self) -> windows_core::Result<super::LiveSetting> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentLiveSetting)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn get_CachedLiveSetting(&self) -> windows_core::Result<super::LiveSetting> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedLiveSetting)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentFlowsFrom(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentFlowsFrom)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CachedFlowsFrom(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedFlowsFrom)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElement2_Vtbl {
    pub base__: IUIAutomationElement_Vtbl,
    pub get_CurrentOptimizeForVisualContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CachedOptimizeForVisualContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub get_CurrentLiveSetting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LiveSetting) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    get_CurrentLiveSetting: usize,
    #[cfg(feature = "uiautomationcore")]
    pub get_CachedLiveSetting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LiveSetting) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    get_CachedLiveSetting: usize,
    pub get_CurrentFlowsFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedFlowsFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationElement2_Impl: IUIAutomationElement_Impl {
    fn get_CurrentOptimizeForVisualContent(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedOptimizeForVisualContent(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CurrentLiveSetting(&self) -> windows_core::Result<super::LiveSetting>;
    fn get_CachedLiveSetting(&self) -> windows_core::Result<super::LiveSetting>;
    fn get_CurrentFlowsFrom(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn get_CachedFlowsFrom(&self) -> windows_core::Result<IUIAutomationElementArray>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationElement2_Vtbl {
    pub const fn new<Identity: IUIAutomationElement2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_CurrentOptimizeForVisualContent<Identity: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement2_Impl::get_CurrentOptimizeForVisualContent(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedOptimizeForVisualContent<Identity: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement2_Impl::get_CachedOptimizeForVisualContent(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentLiveSetting<Identity: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LiveSetting) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement2_Impl::get_CurrentLiveSetting(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedLiveSetting<Identity: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LiveSetting) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement2_Impl::get_CachedLiveSetting(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentFlowsFrom<Identity: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement2_Impl::get_CurrentFlowsFrom(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedFlowsFrom<Identity: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement2_Impl::get_CachedFlowsFrom(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IUIAutomationElement_Vtbl::new::<Identity, OFFSET>(),
            get_CurrentOptimizeForVisualContent: get_CurrentOptimizeForVisualContent::<Identity, OFFSET>,
            get_CachedOptimizeForVisualContent: get_CachedOptimizeForVisualContent::<Identity, OFFSET>,
            get_CurrentLiveSetting: get_CurrentLiveSetting::<Identity, OFFSET>,
            get_CachedLiveSetting: get_CachedLiveSetting::<Identity, OFFSET>,
            get_CurrentFlowsFrom: get_CurrentFlowsFrom::<Identity, OFFSET>,
            get_CachedFlowsFrom: get_CachedFlowsFrom::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement2 as windows_core::Interface>::IID || iid == &<IUIAutomationElement as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUIAutomationElement2 {}
windows_core::imp::define_interface!(IUIAutomationElement3, IUIAutomationElement3_Vtbl, 0x8471df34_aee0_4a01_a7de_7db9af12c296);
impl core::ops::Deref for IUIAutomationElement3 {
    type Target = IUIAutomationElement2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationElement3, windows_core::IUnknown, IUIAutomationElement, IUIAutomationElement2);
impl IUIAutomationElement3 {
    pub unsafe fn ShowContextMenu(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShowContextMenu)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn get_CurrentIsPeripheral(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentIsPeripheral)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedIsPeripheral(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedIsPeripheral)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElement3_Vtbl {
    pub base__: IUIAutomationElement2_Vtbl,
    pub ShowContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentIsPeripheral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CachedIsPeripheral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationElement3_Impl: IUIAutomationElement2_Impl {
    fn ShowContextMenu(&self) -> windows_core::Result<()>;
    fn get_CurrentIsPeripheral(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedIsPeripheral(&self) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationElement3_Vtbl {
    pub const fn new<Identity: IUIAutomationElement3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ShowContextMenu<Identity: IUIAutomationElement3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationElement3_Impl::ShowContextMenu(this).into()
            }
        }
        unsafe extern "system" fn get_CurrentIsPeripheral<Identity: IUIAutomationElement3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement3_Impl::get_CurrentIsPeripheral(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedIsPeripheral<Identity: IUIAutomationElement3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement3_Impl::get_CachedIsPeripheral(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IUIAutomationElement2_Vtbl::new::<Identity, OFFSET>(),
            ShowContextMenu: ShowContextMenu::<Identity, OFFSET>,
            get_CurrentIsPeripheral: get_CurrentIsPeripheral::<Identity, OFFSET>,
            get_CachedIsPeripheral: get_CachedIsPeripheral::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement3 as windows_core::Interface>::IID || iid == &<IUIAutomationElement as windows_core::Interface>::IID || iid == &<IUIAutomationElement2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUIAutomationElement3 {}
windows_core::imp::define_interface!(IUIAutomationElement4, IUIAutomationElement4_Vtbl, 0x3b6e233c_52fb_4063_a4c9_77c075c2a06b);
impl core::ops::Deref for IUIAutomationElement4 {
    type Target = IUIAutomationElement3;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationElement4, windows_core::IUnknown, IUIAutomationElement, IUIAutomationElement2, IUIAutomationElement3);
impl IUIAutomationElement4 {
    pub unsafe fn get_CurrentPositionInSet(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentPositionInSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentSizeOfSet(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentSizeOfSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentLevel(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn get_CurrentAnnotationTypes(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentAnnotationTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentAnnotationObjects(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentAnnotationObjects)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CachedPositionInSet(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedPositionInSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedSizeOfSet(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedSizeOfSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedLevel(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn get_CachedAnnotationTypes(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedAnnotationTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedAnnotationObjects(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedAnnotationObjects)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElement4_Vtbl {
    pub base__: IUIAutomationElement3_Vtbl,
    pub get_CurrentPositionInSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CurrentSizeOfSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CurrentLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub get_CurrentAnnotationTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    get_CurrentAnnotationTypes: usize,
    pub get_CurrentAnnotationObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedPositionInSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CachedSizeOfSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CachedLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub get_CachedAnnotationTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    get_CachedAnnotationTypes: usize,
    pub get_CachedAnnotationObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationElement4_Impl: IUIAutomationElement3_Impl {
    fn get_CurrentPositionInSet(&self) -> windows_core::Result<i32>;
    fn get_CurrentSizeOfSet(&self) -> windows_core::Result<i32>;
    fn get_CurrentLevel(&self) -> windows_core::Result<i32>;
    fn get_CurrentAnnotationTypes(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn get_CurrentAnnotationObjects(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn get_CachedPositionInSet(&self) -> windows_core::Result<i32>;
    fn get_CachedSizeOfSet(&self) -> windows_core::Result<i32>;
    fn get_CachedLevel(&self) -> windows_core::Result<i32>;
    fn get_CachedAnnotationTypes(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn get_CachedAnnotationObjects(&self) -> windows_core::Result<IUIAutomationElementArray>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationElement4_Vtbl {
    pub const fn new<Identity: IUIAutomationElement4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_CurrentPositionInSet<Identity: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement4_Impl::get_CurrentPositionInSet(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentSizeOfSet<Identity: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement4_Impl::get_CurrentSizeOfSet(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentLevel<Identity: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement4_Impl::get_CurrentLevel(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentAnnotationTypes<Identity: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement4_Impl::get_CurrentAnnotationTypes(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentAnnotationObjects<Identity: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement4_Impl::get_CurrentAnnotationObjects(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedPositionInSet<Identity: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement4_Impl::get_CachedPositionInSet(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedSizeOfSet<Identity: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement4_Impl::get_CachedSizeOfSet(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedLevel<Identity: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement4_Impl::get_CachedLevel(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedAnnotationTypes<Identity: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement4_Impl::get_CachedAnnotationTypes(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedAnnotationObjects<Identity: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement4_Impl::get_CachedAnnotationObjects(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IUIAutomationElement3_Vtbl::new::<Identity, OFFSET>(),
            get_CurrentPositionInSet: get_CurrentPositionInSet::<Identity, OFFSET>,
            get_CurrentSizeOfSet: get_CurrentSizeOfSet::<Identity, OFFSET>,
            get_CurrentLevel: get_CurrentLevel::<Identity, OFFSET>,
            get_CurrentAnnotationTypes: get_CurrentAnnotationTypes::<Identity, OFFSET>,
            get_CurrentAnnotationObjects: get_CurrentAnnotationObjects::<Identity, OFFSET>,
            get_CachedPositionInSet: get_CachedPositionInSet::<Identity, OFFSET>,
            get_CachedSizeOfSet: get_CachedSizeOfSet::<Identity, OFFSET>,
            get_CachedLevel: get_CachedLevel::<Identity, OFFSET>,
            get_CachedAnnotationTypes: get_CachedAnnotationTypes::<Identity, OFFSET>,
            get_CachedAnnotationObjects: get_CachedAnnotationObjects::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement4 as windows_core::Interface>::IID || iid == &<IUIAutomationElement as windows_core::Interface>::IID || iid == &<IUIAutomationElement2 as windows_core::Interface>::IID || iid == &<IUIAutomationElement3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUIAutomationElement4 {}
windows_core::imp::define_interface!(IUIAutomationElement5, IUIAutomationElement5_Vtbl, 0x98141c1d_0d0e_4175_bbe2_6bff455842a7);
impl core::ops::Deref for IUIAutomationElement5 {
    type Target = IUIAutomationElement4;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationElement5, windows_core::IUnknown, IUIAutomationElement, IUIAutomationElement2, IUIAutomationElement3, IUIAutomationElement4);
impl IUIAutomationElement5 {
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn get_CurrentLandmarkType(&self) -> windows_core::Result<super::LANDMARKTYPEID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentLandmarkType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentLocalizedLandmarkType(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentLocalizedLandmarkType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn get_CachedLandmarkType(&self) -> windows_core::Result<super::LANDMARKTYPEID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedLandmarkType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedLocalizedLandmarkType(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedLocalizedLandmarkType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElement5_Vtbl {
    pub base__: IUIAutomationElement4_Vtbl,
    #[cfg(feature = "uiautomationcore")]
    pub get_CurrentLandmarkType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LANDMARKTYPEID) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    get_CurrentLandmarkType: usize,
    pub get_CurrentLocalizedLandmarkType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub get_CachedLandmarkType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::LANDMARKTYPEID) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    get_CachedLandmarkType: usize,
    pub get_CachedLocalizedLandmarkType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationElement5_Impl: IUIAutomationElement4_Impl {
    fn get_CurrentLandmarkType(&self) -> windows_core::Result<super::LANDMARKTYPEID>;
    fn get_CurrentLocalizedLandmarkType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedLandmarkType(&self) -> windows_core::Result<super::LANDMARKTYPEID>;
    fn get_CachedLocalizedLandmarkType(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationElement5_Vtbl {
    pub const fn new<Identity: IUIAutomationElement5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_CurrentLandmarkType<Identity: IUIAutomationElement5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LANDMARKTYPEID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement5_Impl::get_CurrentLandmarkType(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentLocalizedLandmarkType<Identity: IUIAutomationElement5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement5_Impl::get_CurrentLocalizedLandmarkType(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedLandmarkType<Identity: IUIAutomationElement5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::LANDMARKTYPEID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement5_Impl::get_CachedLandmarkType(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedLocalizedLandmarkType<Identity: IUIAutomationElement5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement5_Impl::get_CachedLocalizedLandmarkType(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IUIAutomationElement4_Vtbl::new::<Identity, OFFSET>(),
            get_CurrentLandmarkType: get_CurrentLandmarkType::<Identity, OFFSET>,
            get_CurrentLocalizedLandmarkType: get_CurrentLocalizedLandmarkType::<Identity, OFFSET>,
            get_CachedLandmarkType: get_CachedLandmarkType::<Identity, OFFSET>,
            get_CachedLocalizedLandmarkType: get_CachedLocalizedLandmarkType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement5 as windows_core::Interface>::IID || iid == &<IUIAutomationElement as windows_core::Interface>::IID || iid == &<IUIAutomationElement2 as windows_core::Interface>::IID || iid == &<IUIAutomationElement3 as windows_core::Interface>::IID || iid == &<IUIAutomationElement4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUIAutomationElement5 {}
windows_core::imp::define_interface!(IUIAutomationElement6, IUIAutomationElement6_Vtbl, 0x4780d450_8bca_4977_afa5_a4a517f555e3);
impl core::ops::Deref for IUIAutomationElement6 {
    type Target = IUIAutomationElement5;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationElement6, windows_core::IUnknown, IUIAutomationElement, IUIAutomationElement2, IUIAutomationElement3, IUIAutomationElement4, IUIAutomationElement5);
impl IUIAutomationElement6 {
    pub unsafe fn get_CurrentFullDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentFullDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CachedFullDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedFullDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElement6_Vtbl {
    pub base__: IUIAutomationElement5_Vtbl,
    pub get_CurrentFullDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedFullDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationElement6_Impl: IUIAutomationElement5_Impl {
    fn get_CurrentFullDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedFullDescription(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationElement6_Vtbl {
    pub const fn new<Identity: IUIAutomationElement6_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_CurrentFullDescription<Identity: IUIAutomationElement6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement6_Impl::get_CurrentFullDescription(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedFullDescription<Identity: IUIAutomationElement6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement6_Impl::get_CachedFullDescription(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IUIAutomationElement5_Vtbl::new::<Identity, OFFSET>(),
            get_CurrentFullDescription: get_CurrentFullDescription::<Identity, OFFSET>,
            get_CachedFullDescription: get_CachedFullDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement6 as windows_core::Interface>::IID || iid == &<IUIAutomationElement as windows_core::Interface>::IID || iid == &<IUIAutomationElement2 as windows_core::Interface>::IID || iid == &<IUIAutomationElement3 as windows_core::Interface>::IID || iid == &<IUIAutomationElement4 as windows_core::Interface>::IID || iid == &<IUIAutomationElement5 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUIAutomationElement6 {}
windows_core::imp::define_interface!(IUIAutomationElement7, IUIAutomationElement7_Vtbl, 0x204e8572_cfc3_4c11_b0c8_7da7420750b7);
impl core::ops::Deref for IUIAutomationElement7 {
    type Target = IUIAutomationElement6;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationElement7, windows_core::IUnknown, IUIAutomationElement, IUIAutomationElement2, IUIAutomationElement3, IUIAutomationElement4, IUIAutomationElement5, IUIAutomationElement6);
impl IUIAutomationElement7 {
    pub unsafe fn FindFirstWithOptions<P1, P3>(&self, scope: TreeScope, condition: P1, traversaloptions: TreeTraversalOptions, root: P3) -> windows_core::Result<IUIAutomationElement>
    where
        P1: windows_core::Param<IUIAutomationCondition>,
        P3: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindFirstWithOptions)(windows_core::Interface::as_raw(self), scope, condition.param().abi(), traversaloptions, root.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindAllWithOptions<P1, P3>(&self, scope: TreeScope, condition: P1, traversaloptions: TreeTraversalOptions, root: P3) -> windows_core::Result<IUIAutomationElementArray>
    where
        P1: windows_core::Param<IUIAutomationCondition>,
        P3: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindAllWithOptions)(windows_core::Interface::as_raw(self), scope, condition.param().abi(), traversaloptions, root.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindFirstWithOptionsBuildCache<P1, P2, P4>(&self, scope: TreeScope, condition: P1, cacherequest: P2, traversaloptions: TreeTraversalOptions, root: P4) -> windows_core::Result<IUIAutomationElement>
    where
        P1: windows_core::Param<IUIAutomationCondition>,
        P2: windows_core::Param<IUIAutomationCacheRequest>,
        P4: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindFirstWithOptionsBuildCache)(windows_core::Interface::as_raw(self), scope, condition.param().abi(), cacherequest.param().abi(), traversaloptions, root.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindAllWithOptionsBuildCache<P1, P2, P4>(&self, scope: TreeScope, condition: P1, cacherequest: P2, traversaloptions: TreeTraversalOptions, root: P4) -> windows_core::Result<IUIAutomationElementArray>
    where
        P1: windows_core::Param<IUIAutomationCondition>,
        P2: windows_core::Param<IUIAutomationCacheRequest>,
        P4: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindAllWithOptionsBuildCache)(windows_core::Interface::as_raw(self), scope, condition.param().abi(), cacherequest.param().abi(), traversaloptions, root.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetCurrentMetadataValue(&self, targetid: i32, metadataid: super::METADATAID) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentMetadataValue)(windows_core::Interface::as_raw(self), targetid, metadataid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElement7_Vtbl {
    pub base__: IUIAutomationElement6_Vtbl,
    pub FindFirstWithOptions: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, TreeTraversalOptions, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindAllWithOptions: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, TreeTraversalOptions, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindFirstWithOptionsBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void, TreeTraversalOptions, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindAllWithOptionsBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void, TreeTraversalOptions, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetCurrentMetadataValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::METADATAID, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetCurrentMetadataValue: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationElement7_Impl: IUIAutomationElement6_Impl {
    fn FindFirstWithOptions(&self, scope: TreeScope, condition: windows_core::Ref<IUIAutomationCondition>, traversaloptions: TreeTraversalOptions, root: windows_core::Ref<IUIAutomationElement>) -> windows_core::Result<IUIAutomationElement>;
    fn FindAllWithOptions(&self, scope: TreeScope, condition: windows_core::Ref<IUIAutomationCondition>, traversaloptions: TreeTraversalOptions, root: windows_core::Ref<IUIAutomationElement>) -> windows_core::Result<IUIAutomationElementArray>;
    fn FindFirstWithOptionsBuildCache(&self, scope: TreeScope, condition: windows_core::Ref<IUIAutomationCondition>, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, traversaloptions: TreeTraversalOptions, root: windows_core::Ref<IUIAutomationElement>) -> windows_core::Result<IUIAutomationElement>;
    fn FindAllWithOptionsBuildCache(&self, scope: TreeScope, condition: windows_core::Ref<IUIAutomationCondition>, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, traversaloptions: TreeTraversalOptions, root: windows_core::Ref<IUIAutomationElement>) -> windows_core::Result<IUIAutomationElementArray>;
    fn GetCurrentMetadataValue(&self, targetid: i32, metadataid: super::METADATAID) -> windows_core::Result<super::VARIANT>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationElement7_Vtbl {
    pub const fn new<Identity: IUIAutomationElement7_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindFirstWithOptions<Identity: IUIAutomationElement7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: TreeScope, condition: *mut core::ffi::c_void, traversaloptions: TreeTraversalOptions, root: *mut core::ffi::c_void, found: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement7_Impl::FindFirstWithOptions(this, core::mem::transmute_copy(&scope), core::mem::transmute_copy(&condition), core::mem::transmute_copy(&traversaloptions), core::mem::transmute_copy(&root)) {
                    Ok(ok__) => {
                        found.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindAllWithOptions<Identity: IUIAutomationElement7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: TreeScope, condition: *mut core::ffi::c_void, traversaloptions: TreeTraversalOptions, root: *mut core::ffi::c_void, found: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement7_Impl::FindAllWithOptions(this, core::mem::transmute_copy(&scope), core::mem::transmute_copy(&condition), core::mem::transmute_copy(&traversaloptions), core::mem::transmute_copy(&root)) {
                    Ok(ok__) => {
                        found.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindFirstWithOptionsBuildCache<Identity: IUIAutomationElement7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: TreeScope, condition: *mut core::ffi::c_void, cacherequest: *mut core::ffi::c_void, traversaloptions: TreeTraversalOptions, root: *mut core::ffi::c_void, found: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement7_Impl::FindFirstWithOptionsBuildCache(this, core::mem::transmute_copy(&scope), core::mem::transmute_copy(&condition), core::mem::transmute_copy(&cacherequest), core::mem::transmute_copy(&traversaloptions), core::mem::transmute_copy(&root)) {
                    Ok(ok__) => {
                        found.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindAllWithOptionsBuildCache<Identity: IUIAutomationElement7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: TreeScope, condition: *mut core::ffi::c_void, cacherequest: *mut core::ffi::c_void, traversaloptions: TreeTraversalOptions, root: *mut core::ffi::c_void, found: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement7_Impl::FindAllWithOptionsBuildCache(this, core::mem::transmute_copy(&scope), core::mem::transmute_copy(&condition), core::mem::transmute_copy(&cacherequest), core::mem::transmute_copy(&traversaloptions), core::mem::transmute_copy(&root)) {
                    Ok(ok__) => {
                        found.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentMetadataValue<Identity: IUIAutomationElement7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetid: i32, metadataid: super::METADATAID, returnval: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement7_Impl::GetCurrentMetadataValue(this, core::mem::transmute_copy(&targetid), core::mem::transmute_copy(&metadataid)) {
                    Ok(ok__) => {
                        returnval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IUIAutomationElement6_Vtbl::new::<Identity, OFFSET>(),
            FindFirstWithOptions: FindFirstWithOptions::<Identity, OFFSET>,
            FindAllWithOptions: FindAllWithOptions::<Identity, OFFSET>,
            FindFirstWithOptionsBuildCache: FindFirstWithOptionsBuildCache::<Identity, OFFSET>,
            FindAllWithOptionsBuildCache: FindAllWithOptionsBuildCache::<Identity, OFFSET>,
            GetCurrentMetadataValue: GetCurrentMetadataValue::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement7 as windows_core::Interface>::IID || iid == &<IUIAutomationElement as windows_core::Interface>::IID || iid == &<IUIAutomationElement2 as windows_core::Interface>::IID || iid == &<IUIAutomationElement3 as windows_core::Interface>::IID || iid == &<IUIAutomationElement4 as windows_core::Interface>::IID || iid == &<IUIAutomationElement5 as windows_core::Interface>::IID || iid == &<IUIAutomationElement6 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUIAutomationElement7 {}
windows_core::imp::define_interface!(IUIAutomationElement8, IUIAutomationElement8_Vtbl, 0x8c60217d_5411_4cde_bcc0_1ceda223830c);
impl core::ops::Deref for IUIAutomationElement8 {
    type Target = IUIAutomationElement7;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationElement8, windows_core::IUnknown, IUIAutomationElement, IUIAutomationElement2, IUIAutomationElement3, IUIAutomationElement4, IUIAutomationElement5, IUIAutomationElement6, IUIAutomationElement7);
impl IUIAutomationElement8 {
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn get_CurrentHeadingLevel(&self) -> windows_core::Result<super::HEADINGLEVELID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentHeadingLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn get_CachedHeadingLevel(&self) -> windows_core::Result<super::HEADINGLEVELID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedHeadingLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElement8_Vtbl {
    pub base__: IUIAutomationElement7_Vtbl,
    #[cfg(feature = "uiautomationcore")]
    pub get_CurrentHeadingLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HEADINGLEVELID) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    get_CurrentHeadingLevel: usize,
    #[cfg(feature = "uiautomationcore")]
    pub get_CachedHeadingLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::HEADINGLEVELID) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    get_CachedHeadingLevel: usize,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationElement8_Impl: IUIAutomationElement7_Impl {
    fn get_CurrentHeadingLevel(&self) -> windows_core::Result<super::HEADINGLEVELID>;
    fn get_CachedHeadingLevel(&self) -> windows_core::Result<super::HEADINGLEVELID>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationElement8_Vtbl {
    pub const fn new<Identity: IUIAutomationElement8_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_CurrentHeadingLevel<Identity: IUIAutomationElement8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::HEADINGLEVELID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement8_Impl::get_CurrentHeadingLevel(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedHeadingLevel<Identity: IUIAutomationElement8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::HEADINGLEVELID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement8_Impl::get_CachedHeadingLevel(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IUIAutomationElement7_Vtbl::new::<Identity, OFFSET>(),
            get_CurrentHeadingLevel: get_CurrentHeadingLevel::<Identity, OFFSET>,
            get_CachedHeadingLevel: get_CachedHeadingLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement8 as windows_core::Interface>::IID || iid == &<IUIAutomationElement as windows_core::Interface>::IID || iid == &<IUIAutomationElement2 as windows_core::Interface>::IID || iid == &<IUIAutomationElement3 as windows_core::Interface>::IID || iid == &<IUIAutomationElement4 as windows_core::Interface>::IID || iid == &<IUIAutomationElement5 as windows_core::Interface>::IID || iid == &<IUIAutomationElement6 as windows_core::Interface>::IID || iid == &<IUIAutomationElement7 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUIAutomationElement8 {}
windows_core::imp::define_interface!(IUIAutomationElement9, IUIAutomationElement9_Vtbl, 0x39325fac_039d_440e_a3a3_5eb81a5cecc3);
impl core::ops::Deref for IUIAutomationElement9 {
    type Target = IUIAutomationElement8;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationElement9, windows_core::IUnknown, IUIAutomationElement, IUIAutomationElement2, IUIAutomationElement3, IUIAutomationElement4, IUIAutomationElement5, IUIAutomationElement6, IUIAutomationElement7, IUIAutomationElement8);
impl IUIAutomationElement9 {
    pub unsafe fn get_CurrentIsDialog(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentIsDialog)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedIsDialog(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedIsDialog)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElement9_Vtbl {
    pub base__: IUIAutomationElement8_Vtbl,
    pub get_CurrentIsDialog: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CachedIsDialog: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationElement9_Impl: IUIAutomationElement8_Impl {
    fn get_CurrentIsDialog(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedIsDialog(&self) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationElement9_Vtbl {
    pub const fn new<Identity: IUIAutomationElement9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_CurrentIsDialog<Identity: IUIAutomationElement9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement9_Impl::get_CurrentIsDialog(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedIsDialog<Identity: IUIAutomationElement9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement9_Impl::get_CachedIsDialog(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IUIAutomationElement8_Vtbl::new::<Identity, OFFSET>(),
            get_CurrentIsDialog: get_CurrentIsDialog::<Identity, OFFSET>,
            get_CachedIsDialog: get_CachedIsDialog::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement9 as windows_core::Interface>::IID || iid == &<IUIAutomationElement as windows_core::Interface>::IID || iid == &<IUIAutomationElement2 as windows_core::Interface>::IID || iid == &<IUIAutomationElement3 as windows_core::Interface>::IID || iid == &<IUIAutomationElement4 as windows_core::Interface>::IID || iid == &<IUIAutomationElement5 as windows_core::Interface>::IID || iid == &<IUIAutomationElement6 as windows_core::Interface>::IID || iid == &<IUIAutomationElement7 as windows_core::Interface>::IID || iid == &<IUIAutomationElement8 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "guiddef", feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "windef", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUIAutomationElement9 {}
windows_core::imp::define_interface!(IUIAutomationElementArray, IUIAutomationElementArray_Vtbl, 0x14314595_b4bc_4055_95f2_58f2e42c9855);
windows_core::imp::interface_hierarchy!(IUIAutomationElementArray, windows_core::IUnknown);
impl IUIAutomationElementArray {
    pub unsafe fn get_Length(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Length)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetElement(&self, index: i32) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElementArray_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAutomationElementArray_Impl: windows_core::IUnknownImpl {
    fn get_Length(&self) -> windows_core::Result<i32>;
    fn GetElement(&self, index: i32) -> windows_core::Result<IUIAutomationElement>;
}
impl IUIAutomationElementArray_Vtbl {
    pub const fn new<Identity: IUIAutomationElementArray_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Length<Identity: IUIAutomationElementArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElementArray_Impl::get_Length(this) {
                    Ok(ok__) => {
                        length.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetElement<Identity: IUIAutomationElementArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, element: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElementArray_Impl::GetElement(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        element.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Length: get_Length::<Identity, OFFSET>,
            GetElement: GetElement::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationElementArray as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationElementArray {}
windows_core::imp::define_interface!(IUIAutomationEventHandler, IUIAutomationEventHandler_Vtbl, 0x146c3c17_f12e_4e22_8c27_f894b9b79c69);
windows_core::imp::interface_hierarchy!(IUIAutomationEventHandler, windows_core::IUnknown);
impl IUIAutomationEventHandler {
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn HandleAutomationEvent<P0>(&self, sender: P0, eventid: super::EVENTID) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe { (windows_core::Interface::vtable(self).HandleAutomationEvent)(windows_core::Interface::as_raw(self), sender.param().abi(), eventid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "uiautomationcore")]
    pub HandleAutomationEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::EVENTID) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    HandleAutomationEvent: usize,
}
#[cfg(feature = "uiautomationcore")]
pub trait IUIAutomationEventHandler_Impl: windows_core::IUnknownImpl {
    fn HandleAutomationEvent(&self, sender: windows_core::Ref<IUIAutomationElement>, eventid: super::EVENTID) -> windows_core::Result<()>;
}
#[cfg(feature = "uiautomationcore")]
impl IUIAutomationEventHandler_Vtbl {
    pub const fn new<Identity: IUIAutomationEventHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HandleAutomationEvent<Identity: IUIAutomationEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, eventid: super::EVENTID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationEventHandler_Impl::HandleAutomationEvent(this, core::mem::transmute_copy(&sender), core::mem::transmute_copy(&eventid)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HandleAutomationEvent: HandleAutomationEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationEventHandler as windows_core::Interface>::IID
    }
}
#[cfg(feature = "uiautomationcore")]
impl windows_core::RuntimeName for IUIAutomationEventHandler {}
windows_core::imp::define_interface!(IUIAutomationEventHandlerGroup, IUIAutomationEventHandlerGroup_Vtbl, 0xc9ee12f2_c13b_4408_997c_639914377f4e);
windows_core::imp::interface_hierarchy!(IUIAutomationEventHandlerGroup, windows_core::IUnknown);
impl IUIAutomationEventHandlerGroup {
    pub unsafe fn AddActiveTextPositionChangedEventHandler<P1, P2>(&self, scope: TreeScope, cacherequest: P1, handler: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IUIAutomationCacheRequest>,
        P2: windows_core::Param<IUIAutomationActiveTextPositionChangedEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddActiveTextPositionChangedEventHandler)(windows_core::Interface::as_raw(self), scope, cacherequest.param().abi(), handler.param().abi()) }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn AddAutomationEventHandler<P2, P3>(&self, eventid: super::EVENTID, scope: TreeScope, cacherequest: P2, handler: P3) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IUIAutomationCacheRequest>,
        P3: windows_core::Param<IUIAutomationEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddAutomationEventHandler)(windows_core::Interface::as_raw(self), eventid, scope, cacherequest.param().abi(), handler.param().abi()) }
    }
    pub unsafe fn AddChangesEventHandler<P3, P4>(&self, scope: TreeScope, changetypes: &[i32], cacherequest: P3, handler: P4) -> windows_core::HRESULT
    where
        P3: windows_core::Param<IUIAutomationCacheRequest>,
        P4: windows_core::Param<IUIAutomationChangesEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddChangesEventHandler)(windows_core::Interface::as_raw(self), scope, changetypes.as_ptr(), changetypes.len().try_into().unwrap(), cacherequest.param().abi(), handler.param().abi()) }
    }
    pub unsafe fn AddNotificationEventHandler<P1, P2>(&self, scope: TreeScope, cacherequest: P1, handler: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IUIAutomationCacheRequest>,
        P2: windows_core::Param<IUIAutomationNotificationEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddNotificationEventHandler)(windows_core::Interface::as_raw(self), scope, cacherequest.param().abi(), handler.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn AddPropertyChangedEventHandler<P1, P2>(&self, scope: TreeScope, cacherequest: P1, handler: P2, propertyarray: &[super::PROPERTYID]) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IUIAutomationCacheRequest>,
        P2: windows_core::Param<IUIAutomationPropertyChangedEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddPropertyChangedEventHandler)(windows_core::Interface::as_raw(self), scope, cacherequest.param().abi(), handler.param().abi(), propertyarray.as_ptr(), propertyarray.len().try_into().unwrap()) }
    }
    pub unsafe fn AddStructureChangedEventHandler<P1, P2>(&self, scope: TreeScope, cacherequest: P1, handler: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IUIAutomationCacheRequest>,
        P2: windows_core::Param<IUIAutomationStructureChangedEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddStructureChangedEventHandler)(windows_core::Interface::as_raw(self), scope, cacherequest.param().abi(), handler.param().abi()) }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn AddTextEditTextChangedEventHandler<P2, P3>(&self, scope: TreeScope, texteditchangetype: super::TextEditChangeType, cacherequest: P2, handler: P3) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IUIAutomationCacheRequest>,
        P3: windows_core::Param<IUIAutomationTextEditTextChangedEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddTextEditTextChangedEventHandler)(windows_core::Interface::as_raw(self), scope, texteditchangetype, cacherequest.param().abi(), handler.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationEventHandlerGroup_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AddActiveTextPositionChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub AddAutomationEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, super::EVENTID, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    AddAutomationEventHandler: usize,
    pub AddChangesEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *const i32, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddNotificationEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub AddPropertyChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::PROPERTYID, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    AddPropertyChangedEventHandler: usize,
    pub AddStructureChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub AddTextEditTextChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, super::TextEditChangeType, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    AddTextEditTextChangedEventHandler: usize,
}
#[cfg(all(feature = "shared", feature = "uiautomationcore"))]
pub trait IUIAutomationEventHandlerGroup_Impl: windows_core::IUnknownImpl {
    fn AddActiveTextPositionChangedEventHandler(&self, scope: TreeScope, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationActiveTextPositionChangedEventHandler>) -> windows_core::Result<()>;
    fn AddAutomationEventHandler(&self, eventid: super::EVENTID, scope: TreeScope, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationEventHandler>) -> windows_core::Result<()>;
    fn AddChangesEventHandler(&self, scope: TreeScope, changetypes: *const i32, changescount: i32, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationChangesEventHandler>) -> windows_core::Result<()>;
    fn AddNotificationEventHandler(&self, scope: TreeScope, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationNotificationEventHandler>) -> windows_core::Result<()>;
    fn AddPropertyChangedEventHandler(&self, scope: TreeScope, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationPropertyChangedEventHandler>, propertyarray: *const super::PROPERTYID, propertycount: i32) -> windows_core::Result<()>;
    fn AddStructureChangedEventHandler(&self, scope: TreeScope, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationStructureChangedEventHandler>) -> windows_core::Result<()>;
    fn AddTextEditTextChangedEventHandler(&self, scope: TreeScope, texteditchangetype: super::TextEditChangeType, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationTextEditTextChangedEventHandler>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "uiautomationcore"))]
impl IUIAutomationEventHandlerGroup_Vtbl {
    pub const fn new<Identity: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddActiveTextPositionChangedEventHandler<Identity: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: TreeScope, cacherequest: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationEventHandlerGroup_Impl::AddActiveTextPositionChangedEventHandler(this, core::mem::transmute_copy(&scope), core::mem::transmute_copy(&cacherequest), core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn AddAutomationEventHandler<Identity: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: super::EVENTID, scope: TreeScope, cacherequest: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationEventHandlerGroup_Impl::AddAutomationEventHandler(this, core::mem::transmute_copy(&eventid), core::mem::transmute_copy(&scope), core::mem::transmute_copy(&cacherequest), core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn AddChangesEventHandler<Identity: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: TreeScope, changetypes: *const i32, changescount: i32, cacherequest: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationEventHandlerGroup_Impl::AddChangesEventHandler(this, core::mem::transmute_copy(&scope), core::mem::transmute_copy(&changetypes), core::mem::transmute_copy(&changescount), core::mem::transmute_copy(&cacherequest), core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn AddNotificationEventHandler<Identity: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: TreeScope, cacherequest: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationEventHandlerGroup_Impl::AddNotificationEventHandler(this, core::mem::transmute_copy(&scope), core::mem::transmute_copy(&cacherequest), core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn AddPropertyChangedEventHandler<Identity: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: TreeScope, cacherequest: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, propertyarray: *const super::PROPERTYID, propertycount: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationEventHandlerGroup_Impl::AddPropertyChangedEventHandler(this, core::mem::transmute_copy(&scope), core::mem::transmute_copy(&cacherequest), core::mem::transmute_copy(&handler), core::mem::transmute_copy(&propertyarray), core::mem::transmute_copy(&propertycount)).into()
            }
        }
        unsafe extern "system" fn AddStructureChangedEventHandler<Identity: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: TreeScope, cacherequest: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationEventHandlerGroup_Impl::AddStructureChangedEventHandler(this, core::mem::transmute_copy(&scope), core::mem::transmute_copy(&cacherequest), core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn AddTextEditTextChangedEventHandler<Identity: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: TreeScope, texteditchangetype: super::TextEditChangeType, cacherequest: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationEventHandlerGroup_Impl::AddTextEditTextChangedEventHandler(this, core::mem::transmute_copy(&scope), core::mem::transmute_copy(&texteditchangetype), core::mem::transmute_copy(&cacherequest), core::mem::transmute_copy(&handler)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddActiveTextPositionChangedEventHandler: AddActiveTextPositionChangedEventHandler::<Identity, OFFSET>,
            AddAutomationEventHandler: AddAutomationEventHandler::<Identity, OFFSET>,
            AddChangesEventHandler: AddChangesEventHandler::<Identity, OFFSET>,
            AddNotificationEventHandler: AddNotificationEventHandler::<Identity, OFFSET>,
            AddPropertyChangedEventHandler: AddPropertyChangedEventHandler::<Identity, OFFSET>,
            AddStructureChangedEventHandler: AddStructureChangedEventHandler::<Identity, OFFSET>,
            AddTextEditTextChangedEventHandler: AddTextEditTextChangedEventHandler::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationEventHandlerGroup as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "uiautomationcore"))]
impl windows_core::RuntimeName for IUIAutomationEventHandlerGroup {}
windows_core::imp::define_interface!(IUIAutomationExpandCollapsePattern, IUIAutomationExpandCollapsePattern_Vtbl, 0x619be086_1f4e_4ee4_bafa_210128738730);
windows_core::imp::interface_hierarchy!(IUIAutomationExpandCollapsePattern, windows_core::IUnknown);
impl IUIAutomationExpandCollapsePattern {
    pub unsafe fn Expand(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Expand)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Collapse(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Collapse)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn get_CurrentExpandCollapseState(&self) -> windows_core::Result<super::ExpandCollapseState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentExpandCollapseState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn get_CachedExpandCollapseState(&self) -> windows_core::Result<super::ExpandCollapseState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedExpandCollapseState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationExpandCollapsePattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Expand: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Collapse: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub get_CurrentExpandCollapseState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ExpandCollapseState) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    get_CurrentExpandCollapseState: usize,
    #[cfg(feature = "uiautomationcore")]
    pub get_CachedExpandCollapseState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ExpandCollapseState) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    get_CachedExpandCollapseState: usize,
}
#[cfg(feature = "uiautomationcore")]
pub trait IUIAutomationExpandCollapsePattern_Impl: windows_core::IUnknownImpl {
    fn Expand(&self) -> windows_core::Result<()>;
    fn Collapse(&self) -> windows_core::Result<()>;
    fn get_CurrentExpandCollapseState(&self) -> windows_core::Result<super::ExpandCollapseState>;
    fn get_CachedExpandCollapseState(&self) -> windows_core::Result<super::ExpandCollapseState>;
}
#[cfg(feature = "uiautomationcore")]
impl IUIAutomationExpandCollapsePattern_Vtbl {
    pub const fn new<Identity: IUIAutomationExpandCollapsePattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Expand<Identity: IUIAutomationExpandCollapsePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationExpandCollapsePattern_Impl::Expand(this).into()
            }
        }
        unsafe extern "system" fn Collapse<Identity: IUIAutomationExpandCollapsePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationExpandCollapsePattern_Impl::Collapse(this).into()
            }
        }
        unsafe extern "system" fn get_CurrentExpandCollapseState<Identity: IUIAutomationExpandCollapsePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::ExpandCollapseState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationExpandCollapsePattern_Impl::get_CurrentExpandCollapseState(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedExpandCollapseState<Identity: IUIAutomationExpandCollapsePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::ExpandCollapseState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationExpandCollapsePattern_Impl::get_CachedExpandCollapseState(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Expand: Expand::<Identity, OFFSET>,
            Collapse: Collapse::<Identity, OFFSET>,
            get_CurrentExpandCollapseState: get_CurrentExpandCollapseState::<Identity, OFFSET>,
            get_CachedExpandCollapseState: get_CachedExpandCollapseState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationExpandCollapsePattern as windows_core::Interface>::IID
    }
}
#[cfg(feature = "uiautomationcore")]
impl windows_core::RuntimeName for IUIAutomationExpandCollapsePattern {}
windows_core::imp::define_interface!(IUIAutomationFocusChangedEventHandler, IUIAutomationFocusChangedEventHandler_Vtbl, 0xc270f6b5_5c69_4290_9745_7a7f97169468);
windows_core::imp::interface_hierarchy!(IUIAutomationFocusChangedEventHandler, windows_core::IUnknown);
impl IUIAutomationFocusChangedEventHandler {
    pub unsafe fn HandleFocusChangedEvent<P0>(&self, sender: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe { (windows_core::Interface::vtable(self).HandleFocusChangedEvent)(windows_core::Interface::as_raw(self), sender.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationFocusChangedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub HandleFocusChangedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAutomationFocusChangedEventHandler_Impl: windows_core::IUnknownImpl {
    fn HandleFocusChangedEvent(&self, sender: windows_core::Ref<IUIAutomationElement>) -> windows_core::Result<()>;
}
impl IUIAutomationFocusChangedEventHandler_Vtbl {
    pub const fn new<Identity: IUIAutomationFocusChangedEventHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HandleFocusChangedEvent<Identity: IUIAutomationFocusChangedEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationFocusChangedEventHandler_Impl::HandleFocusChangedEvent(this, core::mem::transmute_copy(&sender)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HandleFocusChangedEvent: HandleFocusChangedEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationFocusChangedEventHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationFocusChangedEventHandler {}
windows_core::imp::define_interface!(IUIAutomationGridItemPattern, IUIAutomationGridItemPattern_Vtbl, 0x78f8ef57_66c3_4e09_bd7c_e79b2004894d);
windows_core::imp::interface_hierarchy!(IUIAutomationGridItemPattern, windows_core::IUnknown);
impl IUIAutomationGridItemPattern {
    pub unsafe fn get_CurrentContainingGrid(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentContainingGrid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CurrentRow(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentRow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentColumn(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentColumn)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentRowSpan(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentRowSpan)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentColumnSpan(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentColumnSpan)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedContainingGrid(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedContainingGrid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CachedRow(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedRow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedColumn(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedColumn)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedRowSpan(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedRowSpan)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedColumnSpan(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedColumnSpan)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationGridItemPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_CurrentContainingGrid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentRow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CurrentColumn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CurrentRowSpan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CurrentColumnSpan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CachedContainingGrid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedRow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CachedColumn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CachedRowSpan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CachedColumnSpan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
pub trait IUIAutomationGridItemPattern_Impl: windows_core::IUnknownImpl {
    fn get_CurrentContainingGrid(&self) -> windows_core::Result<IUIAutomationElement>;
    fn get_CurrentRow(&self) -> windows_core::Result<i32>;
    fn get_CurrentColumn(&self) -> windows_core::Result<i32>;
    fn get_CurrentRowSpan(&self) -> windows_core::Result<i32>;
    fn get_CurrentColumnSpan(&self) -> windows_core::Result<i32>;
    fn get_CachedContainingGrid(&self) -> windows_core::Result<IUIAutomationElement>;
    fn get_CachedRow(&self) -> windows_core::Result<i32>;
    fn get_CachedColumn(&self) -> windows_core::Result<i32>;
    fn get_CachedRowSpan(&self) -> windows_core::Result<i32>;
    fn get_CachedColumnSpan(&self) -> windows_core::Result<i32>;
}
impl IUIAutomationGridItemPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_CurrentContainingGrid<Identity: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridItemPattern_Impl::get_CurrentContainingGrid(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentRow<Identity: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridItemPattern_Impl::get_CurrentRow(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentColumn<Identity: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridItemPattern_Impl::get_CurrentColumn(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentRowSpan<Identity: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridItemPattern_Impl::get_CurrentRowSpan(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentColumnSpan<Identity: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridItemPattern_Impl::get_CurrentColumnSpan(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedContainingGrid<Identity: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridItemPattern_Impl::get_CachedContainingGrid(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedRow<Identity: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridItemPattern_Impl::get_CachedRow(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedColumn<Identity: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridItemPattern_Impl::get_CachedColumn(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedRowSpan<Identity: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridItemPattern_Impl::get_CachedRowSpan(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedColumnSpan<Identity: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridItemPattern_Impl::get_CachedColumnSpan(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_CurrentContainingGrid: get_CurrentContainingGrid::<Identity, OFFSET>,
            get_CurrentRow: get_CurrentRow::<Identity, OFFSET>,
            get_CurrentColumn: get_CurrentColumn::<Identity, OFFSET>,
            get_CurrentRowSpan: get_CurrentRowSpan::<Identity, OFFSET>,
            get_CurrentColumnSpan: get_CurrentColumnSpan::<Identity, OFFSET>,
            get_CachedContainingGrid: get_CachedContainingGrid::<Identity, OFFSET>,
            get_CachedRow: get_CachedRow::<Identity, OFFSET>,
            get_CachedColumn: get_CachedColumn::<Identity, OFFSET>,
            get_CachedRowSpan: get_CachedRowSpan::<Identity, OFFSET>,
            get_CachedColumnSpan: get_CachedColumnSpan::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationGridItemPattern as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationGridItemPattern {}
windows_core::imp::define_interface!(IUIAutomationGridPattern, IUIAutomationGridPattern_Vtbl, 0x414c3cdc_856b_4f5b_8538_3131c6302550);
windows_core::imp::interface_hierarchy!(IUIAutomationGridPattern, windows_core::IUnknown);
impl IUIAutomationGridPattern {
    pub unsafe fn GetItem(&self, row: i32, column: i32) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItem)(windows_core::Interface::as_raw(self), row, column, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CurrentRowCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentRowCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentColumnCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentColumnCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedRowCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedRowCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedColumnCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedColumnCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationGridPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentRowCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CurrentColumnCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CachedRowCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CachedColumnCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
pub trait IUIAutomationGridPattern_Impl: windows_core::IUnknownImpl {
    fn GetItem(&self, row: i32, column: i32) -> windows_core::Result<IUIAutomationElement>;
    fn get_CurrentRowCount(&self) -> windows_core::Result<i32>;
    fn get_CurrentColumnCount(&self) -> windows_core::Result<i32>;
    fn get_CachedRowCount(&self) -> windows_core::Result<i32>;
    fn get_CachedColumnCount(&self) -> windows_core::Result<i32>;
}
impl IUIAutomationGridPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationGridPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetItem<Identity: IUIAutomationGridPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, row: i32, column: i32, element: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridPattern_Impl::GetItem(this, core::mem::transmute_copy(&row), core::mem::transmute_copy(&column)) {
                    Ok(ok__) => {
                        element.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentRowCount<Identity: IUIAutomationGridPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridPattern_Impl::get_CurrentRowCount(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentColumnCount<Identity: IUIAutomationGridPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridPattern_Impl::get_CurrentColumnCount(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedRowCount<Identity: IUIAutomationGridPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridPattern_Impl::get_CachedRowCount(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedColumnCount<Identity: IUIAutomationGridPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridPattern_Impl::get_CachedColumnCount(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItem: GetItem::<Identity, OFFSET>,
            get_CurrentRowCount: get_CurrentRowCount::<Identity, OFFSET>,
            get_CurrentColumnCount: get_CurrentColumnCount::<Identity, OFFSET>,
            get_CachedRowCount: get_CachedRowCount::<Identity, OFFSET>,
            get_CachedColumnCount: get_CachedColumnCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationGridPattern as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationGridPattern {}
windows_core::imp::define_interface!(IUIAutomationInvokePattern, IUIAutomationInvokePattern_Vtbl, 0xfb377fbe_8ea6_46d5_9c73_6499642d3059);
windows_core::imp::interface_hierarchy!(IUIAutomationInvokePattern, windows_core::IUnknown);
impl IUIAutomationInvokePattern {
    pub unsafe fn Invoke(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationInvokePattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAutomationInvokePattern_Impl: windows_core::IUnknownImpl {
    fn Invoke(&self) -> windows_core::Result<()>;
}
impl IUIAutomationInvokePattern_Vtbl {
    pub const fn new<Identity: IUIAutomationInvokePattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Invoke<Identity: IUIAutomationInvokePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationInvokePattern_Impl::Invoke(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationInvokePattern as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationInvokePattern {}
windows_core::imp::define_interface!(IUIAutomationItemContainerPattern, IUIAutomationItemContainerPattern_Vtbl, 0xc690fdb2_27a8_423c_812d_429773c9084e);
windows_core::imp::interface_hierarchy!(IUIAutomationItemContainerPattern, windows_core::IUnknown);
impl IUIAutomationItemContainerPattern {
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn FindItemByProperty<P0>(&self, pstartafter: P0, propertyid: super::PROPERTYID, value: &super::VARIANT) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindItemByProperty)(windows_core::Interface::as_raw(self), pstartafter.param().abi(), propertyid, core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationItemContainerPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub FindItemByProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::PROPERTYID, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    FindItemByProperty: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationItemContainerPattern_Impl: windows_core::IUnknownImpl {
    fn FindItemByProperty(&self, pstartafter: windows_core::Ref<IUIAutomationElement>, propertyid: super::PROPERTYID, value: &super::VARIANT) -> windows_core::Result<IUIAutomationElement>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationItemContainerPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationItemContainerPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindItemByProperty<Identity: IUIAutomationItemContainerPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstartafter: *mut core::ffi::c_void, propertyid: super::PROPERTYID, value: super::VARIANT, pfound: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationItemContainerPattern_Impl::FindItemByProperty(this, core::mem::transmute_copy(&pstartafter), core::mem::transmute_copy(&propertyid), core::mem::transmute(&value)) {
                    Ok(ok__) => {
                        pfound.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), FindItemByProperty: FindItemByProperty::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationItemContainerPattern as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUIAutomationItemContainerPattern {}
windows_core::imp::define_interface!(IUIAutomationLegacyIAccessiblePattern, IUIAutomationLegacyIAccessiblePattern_Vtbl, 0x828055ad_355b_4435_86d5_3b51c14a9b1b);
windows_core::imp::interface_hierarchy!(IUIAutomationLegacyIAccessiblePattern, windows_core::IUnknown);
impl IUIAutomationLegacyIAccessiblePattern {
    pub unsafe fn Select(&self, flagsselect: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Select)(windows_core::Interface::as_raw(self), flagsselect) }
    }
    pub unsafe fn DoDefaultAction(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).DoDefaultAction)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn SetValue<P0>(&self, szvalue: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), szvalue.param().abi()) }
    }
    pub unsafe fn get_CurrentChildId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentChildId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CurrentValue(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CurrentDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_CurrentRole(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentRole)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_CurrentState(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentHelp(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentHelp)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CurrentKeyboardShortcut(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentKeyboardShortcut)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetCurrentSelection(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentSelection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CurrentDefaultAction(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentDefaultAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CachedChildId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedChildId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CachedValue(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CachedDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_CachedRole(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedRole)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_CachedState(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedHelp(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedHelp)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CachedKeyboardShortcut(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedKeyboardShortcut)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetCachedSelection(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedSelection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CachedDefaultAction(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedDefaultAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "oleacc"))]
    pub unsafe fn GetIAccessible(&self) -> windows_core::Result<super::IAccessible> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIAccessible)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationLegacyIAccessiblePattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Select: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DoDefaultAction: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub get_CurrentChildId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CurrentName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_CurrentRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_CurrentRole: usize,
    #[cfg(feature = "shared")]
    pub get_CurrentState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_CurrentState: usize,
    pub get_CurrentHelp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentKeyboardShortcut: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentDefaultAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedChildId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CachedName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_CachedRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_CachedRole: usize,
    #[cfg(feature = "shared")]
    pub get_CachedState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_CachedState: usize,
    pub get_CachedHelp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedKeyboardShortcut: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCachedSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedDefaultAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "oleacc"))]
    pub GetIAccessible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "oleacc")))]
    GetIAccessible: usize,
}
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "shared"))]
pub trait IUIAutomationLegacyIAccessiblePattern_Impl: windows_core::IUnknownImpl {
    fn Select(&self, flagsselect: i32) -> windows_core::Result<()>;
    fn DoDefaultAction(&self) -> windows_core::Result<()>;
    fn SetValue(&self, szvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn get_CurrentChildId(&self) -> windows_core::Result<i32>;
    fn get_CurrentName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CurrentValue(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CurrentDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CurrentRole(&self) -> windows_core::Result<super::DWORD>;
    fn get_CurrentState(&self) -> windows_core::Result<super::DWORD>;
    fn get_CurrentHelp(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CurrentKeyboardShortcut(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetCurrentSelection(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn get_CurrentDefaultAction(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedChildId(&self) -> windows_core::Result<i32>;
    fn get_CachedName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedValue(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedRole(&self) -> windows_core::Result<super::DWORD>;
    fn get_CachedState(&self) -> windows_core::Result<super::DWORD>;
    fn get_CachedHelp(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedKeyboardShortcut(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetCachedSelection(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn get_CachedDefaultAction(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetIAccessible(&self) -> windows_core::Result<super::IAccessible>;
}
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "shared"))]
impl IUIAutomationLegacyIAccessiblePattern_Vtbl {
    pub const fn new<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Select<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flagsselect: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationLegacyIAccessiblePattern_Impl::Select(this, core::mem::transmute_copy(&flagsselect)).into()
            }
        }
        unsafe extern "system" fn DoDefaultAction<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationLegacyIAccessiblePattern_Impl::DoDefaultAction(this).into()
            }
        }
        unsafe extern "system" fn SetValue<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationLegacyIAccessiblePattern_Impl::SetValue(this, core::mem::transmute(&szvalue)).into()
            }
        }
        unsafe extern "system" fn get_CurrentChildId<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::get_CurrentChildId(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentName<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::get_CurrentName(this) {
                    Ok(ok__) => {
                        pszname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentValue<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::get_CurrentValue(this) {
                    Ok(ok__) => {
                        pszvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentDescription<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::get_CurrentDescription(this) {
                    Ok(ok__) => {
                        pszdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentRole<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwrole: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::get_CurrentRole(this) {
                    Ok(ok__) => {
                        pdwrole.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentState<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstate: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::get_CurrentState(this) {
                    Ok(ok__) => {
                        pdwstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentHelp<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszhelp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::get_CurrentHelp(this) {
                    Ok(ok__) => {
                        pszhelp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentKeyboardShortcut<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszkeyboardshortcut: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::get_CurrentKeyboardShortcut(this) {
                    Ok(ok__) => {
                        pszkeyboardshortcut.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentSelection<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarselectedchildren: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::GetCurrentSelection(this) {
                    Ok(ok__) => {
                        pvarselectedchildren.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentDefaultAction<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdefaultaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::get_CurrentDefaultAction(this) {
                    Ok(ok__) => {
                        pszdefaultaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedChildId<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::get_CachedChildId(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedName<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::get_CachedName(this) {
                    Ok(ok__) => {
                        pszname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedValue<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::get_CachedValue(this) {
                    Ok(ok__) => {
                        pszvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedDescription<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::get_CachedDescription(this) {
                    Ok(ok__) => {
                        pszdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedRole<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwrole: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::get_CachedRole(this) {
                    Ok(ok__) => {
                        pdwrole.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedState<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstate: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::get_CachedState(this) {
                    Ok(ok__) => {
                        pdwstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedHelp<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszhelp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::get_CachedHelp(this) {
                    Ok(ok__) => {
                        pszhelp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedKeyboardShortcut<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszkeyboardshortcut: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::get_CachedKeyboardShortcut(this) {
                    Ok(ok__) => {
                        pszkeyboardshortcut.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCachedSelection<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarselectedchildren: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::GetCachedSelection(this) {
                    Ok(ok__) => {
                        pvarselectedchildren.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedDefaultAction<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdefaultaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::get_CachedDefaultAction(this) {
                    Ok(ok__) => {
                        pszdefaultaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIAccessible<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaccessible: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::GetIAccessible(this) {
                    Ok(ok__) => {
                        ppaccessible.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Select: Select::<Identity, OFFSET>,
            DoDefaultAction: DoDefaultAction::<Identity, OFFSET>,
            SetValue: SetValue::<Identity, OFFSET>,
            get_CurrentChildId: get_CurrentChildId::<Identity, OFFSET>,
            get_CurrentName: get_CurrentName::<Identity, OFFSET>,
            get_CurrentValue: get_CurrentValue::<Identity, OFFSET>,
            get_CurrentDescription: get_CurrentDescription::<Identity, OFFSET>,
            get_CurrentRole: get_CurrentRole::<Identity, OFFSET>,
            get_CurrentState: get_CurrentState::<Identity, OFFSET>,
            get_CurrentHelp: get_CurrentHelp::<Identity, OFFSET>,
            get_CurrentKeyboardShortcut: get_CurrentKeyboardShortcut::<Identity, OFFSET>,
            GetCurrentSelection: GetCurrentSelection::<Identity, OFFSET>,
            get_CurrentDefaultAction: get_CurrentDefaultAction::<Identity, OFFSET>,
            get_CachedChildId: get_CachedChildId::<Identity, OFFSET>,
            get_CachedName: get_CachedName::<Identity, OFFSET>,
            get_CachedValue: get_CachedValue::<Identity, OFFSET>,
            get_CachedDescription: get_CachedDescription::<Identity, OFFSET>,
            get_CachedRole: get_CachedRole::<Identity, OFFSET>,
            get_CachedState: get_CachedState::<Identity, OFFSET>,
            get_CachedHelp: get_CachedHelp::<Identity, OFFSET>,
            get_CachedKeyboardShortcut: get_CachedKeyboardShortcut::<Identity, OFFSET>,
            GetCachedSelection: GetCachedSelection::<Identity, OFFSET>,
            get_CachedDefaultAction: get_CachedDefaultAction::<Identity, OFFSET>,
            GetIAccessible: GetIAccessible::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationLegacyIAccessiblePattern as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "shared"))]
impl windows_core::RuntimeName for IUIAutomationLegacyIAccessiblePattern {}
windows_core::imp::define_interface!(IUIAutomationMultipleViewPattern, IUIAutomationMultipleViewPattern_Vtbl, 0x8d253c91_1dc5_4bb5_b18f_ade16fa495e8);
windows_core::imp::interface_hierarchy!(IUIAutomationMultipleViewPattern, windows_core::IUnknown);
impl IUIAutomationMultipleViewPattern {
    pub unsafe fn GetViewName(&self, view: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetViewName)(windows_core::Interface::as_raw(self), view, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetCurrentView(&self, view: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentView)(windows_core::Interface::as_raw(self), view) }
    }
    pub unsafe fn get_CurrentCurrentView(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentCurrentView)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetCurrentSupportedViews(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentSupportedViews)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedCurrentView(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedCurrentView)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetCachedSupportedViews(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedSupportedViews)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationMultipleViewPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetViewName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub get_CurrentCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetCurrentSupportedViews: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetCurrentSupportedViews: usize,
    pub get_CachedCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetCachedSupportedViews: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetCachedSupportedViews: usize,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
pub trait IUIAutomationMultipleViewPattern_Impl: windows_core::IUnknownImpl {
    fn GetViewName(&self, view: i32) -> windows_core::Result<windows_core::BSTR>;
    fn SetCurrentView(&self, view: i32) -> windows_core::Result<()>;
    fn get_CurrentCurrentView(&self) -> windows_core::Result<i32>;
    fn GetCurrentSupportedViews(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn get_CachedCurrentView(&self) -> windows_core::Result<i32>;
    fn GetCachedSupportedViews(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl IUIAutomationMultipleViewPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetViewName<Identity: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, view: i32, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationMultipleViewPattern_Impl::GetViewName(this, core::mem::transmute_copy(&view)) {
                    Ok(ok__) => {
                        name.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCurrentView<Identity: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, view: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationMultipleViewPattern_Impl::SetCurrentView(this, core::mem::transmute_copy(&view)).into()
            }
        }
        unsafe extern "system" fn get_CurrentCurrentView<Identity: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationMultipleViewPattern_Impl::get_CurrentCurrentView(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentSupportedViews<Identity: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationMultipleViewPattern_Impl::GetCurrentSupportedViews(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedCurrentView<Identity: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationMultipleViewPattern_Impl::get_CachedCurrentView(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCachedSupportedViews<Identity: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationMultipleViewPattern_Impl::GetCachedSupportedViews(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetViewName: GetViewName::<Identity, OFFSET>,
            SetCurrentView: SetCurrentView::<Identity, OFFSET>,
            get_CurrentCurrentView: get_CurrentCurrentView::<Identity, OFFSET>,
            GetCurrentSupportedViews: GetCurrentSupportedViews::<Identity, OFFSET>,
            get_CachedCurrentView: get_CachedCurrentView::<Identity, OFFSET>,
            GetCachedSupportedViews: GetCachedSupportedViews::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationMultipleViewPattern as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IUIAutomationMultipleViewPattern {}
windows_core::imp::define_interface!(IUIAutomationNotCondition, IUIAutomationNotCondition_Vtbl, 0xf528b657_847b_498c_8896_d52b565407a1);
impl core::ops::Deref for IUIAutomationNotCondition {
    type Target = IUIAutomationCondition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationNotCondition, windows_core::IUnknown, IUIAutomationCondition);
impl IUIAutomationNotCondition {
    pub unsafe fn GetChild(&self) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChild)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationNotCondition_Vtbl {
    pub base__: IUIAutomationCondition_Vtbl,
    pub GetChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAutomationNotCondition_Impl: IUIAutomationCondition_Impl {
    fn GetChild(&self) -> windows_core::Result<IUIAutomationCondition>;
}
impl IUIAutomationNotCondition_Vtbl {
    pub const fn new<Identity: IUIAutomationNotCondition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetChild<Identity: IUIAutomationNotCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, condition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationNotCondition_Impl::GetChild(this) {
                    Ok(ok__) => {
                        condition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IUIAutomationCondition_Vtbl::new::<Identity, OFFSET>(), GetChild: GetChild::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationNotCondition as windows_core::Interface>::IID || iid == &<IUIAutomationCondition as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationNotCondition {}
windows_core::imp::define_interface!(IUIAutomationNotificationEventHandler, IUIAutomationNotificationEventHandler_Vtbl, 0xc7cb2637_e6c2_4d0c_85de_4948c02175c7);
windows_core::imp::interface_hierarchy!(IUIAutomationNotificationEventHandler, windows_core::IUnknown);
impl IUIAutomationNotificationEventHandler {
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn HandleNotificationEvent<P0>(&self, sender: P0, notificationkind: super::NotificationKind, notificationprocessing: super::NotificationProcessing, displaystring: &windows_core::BSTR, activityid: &windows_core::BSTR) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe { (windows_core::Interface::vtable(self).HandleNotificationEvent)(windows_core::Interface::as_raw(self), sender.param().abi(), notificationkind, notificationprocessing, core::mem::transmute_copy(displaystring), core::mem::transmute_copy(activityid)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationNotificationEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "uiautomationcore")]
    pub HandleNotificationEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::NotificationKind, super::NotificationProcessing, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    HandleNotificationEvent: usize,
}
#[cfg(feature = "uiautomationcore")]
pub trait IUIAutomationNotificationEventHandler_Impl: windows_core::IUnknownImpl {
    fn HandleNotificationEvent(&self, sender: windows_core::Ref<IUIAutomationElement>, notificationkind: super::NotificationKind, notificationprocessing: super::NotificationProcessing, displaystring: &windows_core::BSTR, activityid: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "uiautomationcore")]
impl IUIAutomationNotificationEventHandler_Vtbl {
    pub const fn new<Identity: IUIAutomationNotificationEventHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HandleNotificationEvent<Identity: IUIAutomationNotificationEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, notificationkind: super::NotificationKind, notificationprocessing: super::NotificationProcessing, displaystring: *mut core::ffi::c_void, activityid: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationNotificationEventHandler_Impl::HandleNotificationEvent(this, core::mem::transmute_copy(&sender), core::mem::transmute_copy(&notificationkind), core::mem::transmute_copy(&notificationprocessing), core::mem::transmute(&displaystring), core::mem::transmute(&activityid)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HandleNotificationEvent: HandleNotificationEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationNotificationEventHandler as windows_core::Interface>::IID
    }
}
#[cfg(feature = "uiautomationcore")]
impl windows_core::RuntimeName for IUIAutomationNotificationEventHandler {}
windows_core::imp::define_interface!(IUIAutomationObjectModelPattern, IUIAutomationObjectModelPattern_Vtbl, 0x71c284b3_c14d_4d14_981e_19751b0d756d);
windows_core::imp::interface_hierarchy!(IUIAutomationObjectModelPattern, windows_core::IUnknown);
impl IUIAutomationObjectModelPattern {
    pub unsafe fn GetUnderlyingObjectModel(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUnderlyingObjectModel)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationObjectModelPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetUnderlyingObjectModel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAutomationObjectModelPattern_Impl: windows_core::IUnknownImpl {
    fn GetUnderlyingObjectModel(&self) -> windows_core::Result<windows_core::IUnknown>;
}
impl IUIAutomationObjectModelPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationObjectModelPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetUnderlyingObjectModel<Identity: IUIAutomationObjectModelPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationObjectModelPattern_Impl::GetUnderlyingObjectModel(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetUnderlyingObjectModel: GetUnderlyingObjectModel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationObjectModelPattern as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationObjectModelPattern {}
windows_core::imp::define_interface!(IUIAutomationOrCondition, IUIAutomationOrCondition_Vtbl, 0x8753f032_3db1_47b5_a1fc_6e34a266c712);
impl core::ops::Deref for IUIAutomationOrCondition {
    type Target = IUIAutomationCondition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationOrCondition, windows_core::IUnknown, IUIAutomationCondition);
impl IUIAutomationOrCondition {
    pub unsafe fn get_ChildCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ChildCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetChildrenAsNativeArray(&self, childarray: *mut *mut Option<IUIAutomationCondition>, childarraycount: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetChildrenAsNativeArray)(windows_core::Interface::as_raw(self), childarray as _, childarraycount as _) }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetChildren(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChildren)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationOrCondition_Vtbl {
    pub base__: IUIAutomationCondition_Vtbl,
    pub get_ChildCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetChildrenAsNativeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetChildren: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetChildren: usize,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
pub trait IUIAutomationOrCondition_Impl: IUIAutomationCondition_Impl {
    fn get_ChildCount(&self) -> windows_core::Result<i32>;
    fn GetChildrenAsNativeArray(&self, childarray: *mut *mut Option<IUIAutomationCondition>, childarraycount: *mut i32) -> windows_core::Result<()>;
    fn GetChildren(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl IUIAutomationOrCondition_Vtbl {
    pub const fn new<Identity: IUIAutomationOrCondition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ChildCount<Identity: IUIAutomationOrCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, childcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationOrCondition_Impl::get_ChildCount(this) {
                    Ok(ok__) => {
                        childcount.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetChildrenAsNativeArray<Identity: IUIAutomationOrCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, childarray: *mut *mut *mut core::ffi::c_void, childarraycount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationOrCondition_Impl::GetChildrenAsNativeArray(this, core::mem::transmute_copy(&childarray), core::mem::transmute_copy(&childarraycount)).into()
            }
        }
        unsafe extern "system" fn GetChildren<Identity: IUIAutomationOrCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, childarray: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationOrCondition_Impl::GetChildren(this) {
                    Ok(ok__) => {
                        childarray.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IUIAutomationCondition_Vtbl::new::<Identity, OFFSET>(),
            get_ChildCount: get_ChildCount::<Identity, OFFSET>,
            GetChildrenAsNativeArray: GetChildrenAsNativeArray::<Identity, OFFSET>,
            GetChildren: GetChildren::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationOrCondition as windows_core::Interface>::IID || iid == &<IUIAutomationCondition as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IUIAutomationOrCondition {}
windows_core::imp::define_interface!(IUIAutomationPropertyChangedEventHandler, IUIAutomationPropertyChangedEventHandler_Vtbl, 0x40cd37d4_c756_4b0c_8c6f_bddfeeb13b50);
windows_core::imp::interface_hierarchy!(IUIAutomationPropertyChangedEventHandler, windows_core::IUnknown);
impl IUIAutomationPropertyChangedEventHandler {
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn HandlePropertyChangedEvent<P0>(&self, sender: P0, propertyid: super::PROPERTYID, newvalue: &super::VARIANT) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe { (windows_core::Interface::vtable(self).HandlePropertyChangedEvent)(windows_core::Interface::as_raw(self), sender.param().abi(), propertyid, core::mem::transmute_copy(newvalue)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationPropertyChangedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub HandlePropertyChangedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::PROPERTYID, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    HandlePropertyChangedEvent: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationPropertyChangedEventHandler_Impl: windows_core::IUnknownImpl {
    fn HandlePropertyChangedEvent(&self, sender: windows_core::Ref<IUIAutomationElement>, propertyid: super::PROPERTYID, newvalue: &super::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationPropertyChangedEventHandler_Vtbl {
    pub const fn new<Identity: IUIAutomationPropertyChangedEventHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HandlePropertyChangedEvent<Identity: IUIAutomationPropertyChangedEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, propertyid: super::PROPERTYID, newvalue: super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationPropertyChangedEventHandler_Impl::HandlePropertyChangedEvent(this, core::mem::transmute_copy(&sender), core::mem::transmute_copy(&propertyid), core::mem::transmute(&newvalue)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HandlePropertyChangedEvent: HandlePropertyChangedEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationPropertyChangedEventHandler as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUIAutomationPropertyChangedEventHandler {}
windows_core::imp::define_interface!(IUIAutomationPropertyCondition, IUIAutomationPropertyCondition_Vtbl, 0x99ebf2cb_5578_4267_9ad4_afd6ea77e94b);
impl core::ops::Deref for IUIAutomationPropertyCondition {
    type Target = IUIAutomationCondition;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationPropertyCondition, windows_core::IUnknown, IUIAutomationCondition);
impl IUIAutomationPropertyCondition {
    #[cfg(feature = "shared")]
    pub unsafe fn get_PropertyId(&self) -> windows_core::Result<super::PROPERTYID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PropertyId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn get_PropertyValue(&self) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PropertyValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_PropertyConditionFlags(&self) -> windows_core::Result<PropertyConditionFlags> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_PropertyConditionFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationPropertyCondition_Vtbl {
    pub base__: IUIAutomationCondition_Vtbl,
    #[cfg(feature = "shared")]
    pub get_PropertyId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::PROPERTYID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_PropertyId: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub get_PropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    get_PropertyValue: usize,
    pub get_PropertyConditionFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PropertyConditionFlags) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationPropertyCondition_Impl: IUIAutomationCondition_Impl {
    fn get_PropertyId(&self) -> windows_core::Result<super::PROPERTYID>;
    fn get_PropertyValue(&self) -> windows_core::Result<super::VARIANT>;
    fn get_PropertyConditionFlags(&self) -> windows_core::Result<PropertyConditionFlags>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationPropertyCondition_Vtbl {
    pub const fn new<Identity: IUIAutomationPropertyCondition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_PropertyId<Identity: IUIAutomationPropertyCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: *mut super::PROPERTYID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationPropertyCondition_Impl::get_PropertyId(this) {
                    Ok(ok__) => {
                        propertyid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_PropertyValue<Identity: IUIAutomationPropertyCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyvalue: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationPropertyCondition_Impl::get_PropertyValue(this) {
                    Ok(ok__) => {
                        propertyvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_PropertyConditionFlags<Identity: IUIAutomationPropertyCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut PropertyConditionFlags) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationPropertyCondition_Impl::get_PropertyConditionFlags(this) {
                    Ok(ok__) => {
                        flags.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IUIAutomationCondition_Vtbl::new::<Identity, OFFSET>(),
            get_PropertyId: get_PropertyId::<Identity, OFFSET>,
            get_PropertyValue: get_PropertyValue::<Identity, OFFSET>,
            get_PropertyConditionFlags: get_PropertyConditionFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationPropertyCondition as windows_core::Interface>::IID || iid == &<IUIAutomationCondition as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUIAutomationPropertyCondition {}
windows_core::imp::define_interface!(IUIAutomationProxyFactory, IUIAutomationProxyFactory_Vtbl, 0x85b94ecd_849d_42b6_b94d_d6db23fdf5a4);
windows_core::imp::interface_hierarchy!(IUIAutomationProxyFactory, windows_core::IUnknown);
impl IUIAutomationProxyFactory {
    #[cfg(all(feature = "shared", feature = "uiautomationcore"))]
    pub unsafe fn CreateProvider(&self, hwnd: UIA_HWND, idobject: super::LONG, idchild: super::LONG) -> windows_core::Result<super::IRawElementProviderSimple> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateProvider)(windows_core::Interface::as_raw(self), hwnd, idobject, idchild, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_ProxyFactoryId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ProxyFactoryId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationProxyFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "uiautomationcore"))]
    pub CreateProvider: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_HWND, super::LONG, super::LONG, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "uiautomationcore")))]
    CreateProvider: usize,
    pub get_ProxyFactoryId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "uiautomationcore"))]
pub trait IUIAutomationProxyFactory_Impl: windows_core::IUnknownImpl {
    fn CreateProvider(&self, hwnd: UIA_HWND, idobject: super::LONG, idchild: super::LONG) -> windows_core::Result<super::IRawElementProviderSimple>;
    fn get_ProxyFactoryId(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "shared", feature = "uiautomationcore"))]
impl IUIAutomationProxyFactory_Vtbl {
    pub const fn new<Identity: IUIAutomationProxyFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateProvider<Identity: IUIAutomationProxyFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: UIA_HWND, idobject: super::LONG, idchild: super::LONG, provider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationProxyFactory_Impl::CreateProvider(this, core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&idobject), core::mem::transmute_copy(&idchild)) {
                    Ok(ok__) => {
                        provider.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ProxyFactoryId<Identity: IUIAutomationProxyFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, factoryid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationProxyFactory_Impl::get_ProxyFactoryId(this) {
                    Ok(ok__) => {
                        factoryid.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateProvider: CreateProvider::<Identity, OFFSET>,
            get_ProxyFactoryId: get_ProxyFactoryId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationProxyFactory as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "uiautomationcore"))]
impl windows_core::RuntimeName for IUIAutomationProxyFactory {}
windows_core::imp::define_interface!(IUIAutomationProxyFactoryEntry, IUIAutomationProxyFactoryEntry_Vtbl, 0xd50e472e_b64b_490c_bca1_d30696f9f289);
windows_core::imp::interface_hierarchy!(IUIAutomationProxyFactoryEntry, windows_core::IUnknown);
impl IUIAutomationProxyFactoryEntry {
    pub unsafe fn get_ProxyFactory(&self) -> windows_core::Result<IUIAutomationProxyFactory> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ProxyFactory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_ClassName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ClassName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_ImageName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ImageName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_AllowSubstringMatch(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AllowSubstringMatch)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CanCheckBaseClass(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CanCheckBaseClass)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_NeedsAdviseEvents(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_NeedsAdviseEvents)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn put_ClassName<P0>(&self, classname: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_ClassName)(windows_core::Interface::as_raw(self), classname.param().abi()) }
    }
    pub unsafe fn put_ImageName<P0>(&self, imagename: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).put_ImageName)(windows_core::Interface::as_raw(self), imagename.param().abi()) }
    }
    pub unsafe fn put_AllowSubstringMatch(&self, allowsubstringmatch: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_AllowSubstringMatch)(windows_core::Interface::as_raw(self), allowsubstringmatch.into()) }
    }
    pub unsafe fn put_CanCheckBaseClass(&self, cancheckbaseclass: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_CanCheckBaseClass)(windows_core::Interface::as_raw(self), cancheckbaseclass.into()) }
    }
    pub unsafe fn put_NeedsAdviseEvents(&self, adviseevents: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).put_NeedsAdviseEvents)(windows_core::Interface::as_raw(self), adviseevents.into()) }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt"))]
    pub unsafe fn SetWinEventsForAutomationEvent(&self, eventid: super::EVENTID, propertyid: super::PROPERTYID, winevents: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWinEventsForAutomationEvent)(windows_core::Interface::as_raw(self), eventid, propertyid, winevents) }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt"))]
    pub unsafe fn GetWinEventsForAutomationEvent(&self, eventid: super::EVENTID, propertyid: super::PROPERTYID) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetWinEventsForAutomationEvent)(windows_core::Interface::as_raw(self), eventid, propertyid, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationProxyFactoryEntry_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_ProxyFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ClassName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ImageName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_AllowSubstringMatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CanCheckBaseClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_NeedsAdviseEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub put_ClassName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub put_ImageName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub put_AllowSubstringMatch: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub put_CanCheckBaseClass: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub put_NeedsAdviseEvents: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt"))]
    pub SetWinEventsForAutomationEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::EVENTID, super::PROPERTYID, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt")))]
    SetWinEventsForAutomationEvent: usize,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt"))]
    pub GetWinEventsForAutomationEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::EVENTID, super::PROPERTYID, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt")))]
    GetWinEventsForAutomationEvent: usize,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt"))]
pub trait IUIAutomationProxyFactoryEntry_Impl: windows_core::IUnknownImpl {
    fn get_ProxyFactory(&self) -> windows_core::Result<IUIAutomationProxyFactory>;
    fn get_ClassName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_ImageName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_AllowSubstringMatch(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CanCheckBaseClass(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_NeedsAdviseEvents(&self) -> windows_core::Result<windows_core::BOOL>;
    fn put_ClassName(&self, classname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn put_ImageName(&self, imagename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn put_AllowSubstringMatch(&self, allowsubstringmatch: windows_core::BOOL) -> windows_core::Result<()>;
    fn put_CanCheckBaseClass(&self, cancheckbaseclass: windows_core::BOOL) -> windows_core::Result<()>;
    fn put_NeedsAdviseEvents(&self, adviseevents: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetWinEventsForAutomationEvent(&self, eventid: super::EVENTID, propertyid: super::PROPERTYID, winevents: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn GetWinEventsForAutomationEvent(&self, eventid: super::EVENTID, propertyid: super::PROPERTYID) -> windows_core::Result<*mut super::SAFEARRAY>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt"))]
impl IUIAutomationProxyFactoryEntry_Vtbl {
    pub const fn new<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ProxyFactory<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, factory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationProxyFactoryEntry_Impl::get_ProxyFactory(this) {
                    Ok(ok__) => {
                        factory.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ClassName<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationProxyFactoryEntry_Impl::get_ClassName(this) {
                    Ok(ok__) => {
                        classname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ImageName<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationProxyFactoryEntry_Impl::get_ImageName(this) {
                    Ok(ok__) => {
                        imagename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_AllowSubstringMatch<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allowsubstringmatch: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationProxyFactoryEntry_Impl::get_AllowSubstringMatch(this) {
                    Ok(ok__) => {
                        allowsubstringmatch.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CanCheckBaseClass<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cancheckbaseclass: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationProxyFactoryEntry_Impl::get_CanCheckBaseClass(this) {
                    Ok(ok__) => {
                        cancheckbaseclass.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_NeedsAdviseEvents<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adviseevents: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationProxyFactoryEntry_Impl::get_NeedsAdviseEvents(this) {
                    Ok(ok__) => {
                        adviseevents.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn put_ClassName<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationProxyFactoryEntry_Impl::put_ClassName(this, core::mem::transmute(&classname)).into()
            }
        }
        unsafe extern "system" fn put_ImageName<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagename: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationProxyFactoryEntry_Impl::put_ImageName(this, core::mem::transmute(&imagename)).into()
            }
        }
        unsafe extern "system" fn put_AllowSubstringMatch<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allowsubstringmatch: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationProxyFactoryEntry_Impl::put_AllowSubstringMatch(this, core::mem::transmute_copy(&allowsubstringmatch)).into()
            }
        }
        unsafe extern "system" fn put_CanCheckBaseClass<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cancheckbaseclass: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationProxyFactoryEntry_Impl::put_CanCheckBaseClass(this, core::mem::transmute_copy(&cancheckbaseclass)).into()
            }
        }
        unsafe extern "system" fn put_NeedsAdviseEvents<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adviseevents: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationProxyFactoryEntry_Impl::put_NeedsAdviseEvents(this, core::mem::transmute_copy(&adviseevents)).into()
            }
        }
        unsafe extern "system" fn SetWinEventsForAutomationEvent<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: super::EVENTID, propertyid: super::PROPERTYID, winevents: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationProxyFactoryEntry_Impl::SetWinEventsForAutomationEvent(this, core::mem::transmute_copy(&eventid), core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&winevents)).into()
            }
        }
        unsafe extern "system" fn GetWinEventsForAutomationEvent<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: super::EVENTID, propertyid: super::PROPERTYID, winevents: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationProxyFactoryEntry_Impl::GetWinEventsForAutomationEvent(this, core::mem::transmute_copy(&eventid), core::mem::transmute_copy(&propertyid)) {
                    Ok(ok__) => {
                        winevents.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_ProxyFactory: get_ProxyFactory::<Identity, OFFSET>,
            get_ClassName: get_ClassName::<Identity, OFFSET>,
            get_ImageName: get_ImageName::<Identity, OFFSET>,
            get_AllowSubstringMatch: get_AllowSubstringMatch::<Identity, OFFSET>,
            get_CanCheckBaseClass: get_CanCheckBaseClass::<Identity, OFFSET>,
            get_NeedsAdviseEvents: get_NeedsAdviseEvents::<Identity, OFFSET>,
            put_ClassName: put_ClassName::<Identity, OFFSET>,
            put_ImageName: put_ImageName::<Identity, OFFSET>,
            put_AllowSubstringMatch: put_AllowSubstringMatch::<Identity, OFFSET>,
            put_CanCheckBaseClass: put_CanCheckBaseClass::<Identity, OFFSET>,
            put_NeedsAdviseEvents: put_NeedsAdviseEvents::<Identity, OFFSET>,
            SetWinEventsForAutomationEvent: SetWinEventsForAutomationEvent::<Identity, OFFSET>,
            GetWinEventsForAutomationEvent: GetWinEventsForAutomationEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationProxyFactoryEntry as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt"))]
impl windows_core::RuntimeName for IUIAutomationProxyFactoryEntry {}
windows_core::imp::define_interface!(IUIAutomationProxyFactoryMapping, IUIAutomationProxyFactoryMapping_Vtbl, 0x09e31e18_872d_4873_93d1_1e541ec133fd);
windows_core::imp::interface_hierarchy!(IUIAutomationProxyFactoryMapping, windows_core::IUnknown);
impl IUIAutomationProxyFactoryMapping {
    #[cfg(feature = "shared")]
    pub unsafe fn get_Count(&self) -> windows_core::Result<super::UINT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetTable(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn GetEntry(&self, index: super::UINT) -> windows_core::Result<IUIAutomationProxyFactoryEntry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEntry)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn SetTable(&self, factorylist: Option<*const super::SAFEARRAY>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTable)(windows_core::Interface::as_raw(self), factorylist.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn InsertEntries(&self, before: super::UINT, factorylist: Option<*const super::SAFEARRAY>) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InsertEntries)(windows_core::Interface::as_raw(self), before, factorylist.unwrap_or(core::mem::zeroed()) as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn InsertEntry<P1>(&self, before: super::UINT, factory: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IUIAutomationProxyFactoryEntry>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertEntry)(windows_core::Interface::as_raw(self), before, factory.param().abi()) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn RemoveEntry(&self, index: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveEntry)(windows_core::Interface::as_raw(self), index) }
    }
    pub unsafe fn ClearTable(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ClearTable)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn RestoreDefaultTable(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RestoreDefaultTable)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationProxyFactoryMapping_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub get_Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Count: usize,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetTable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetTable: usize,
    #[cfg(feature = "shared")]
    pub GetEntry: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetEntry: usize,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub SetTable: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    SetTable: usize,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub InsertEntries: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    InsertEntries: usize,
    #[cfg(feature = "shared")]
    pub InsertEntry: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    InsertEntry: usize,
    #[cfg(feature = "shared")]
    pub RemoveEntry: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RemoveEntry: usize,
    pub ClearTable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RestoreDefaultTable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
pub trait IUIAutomationProxyFactoryMapping_Impl: windows_core::IUnknownImpl {
    fn get_Count(&self) -> windows_core::Result<super::UINT>;
    fn GetTable(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn GetEntry(&self, index: super::UINT) -> windows_core::Result<IUIAutomationProxyFactoryEntry>;
    fn SetTable(&self, factorylist: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn InsertEntries(&self, before: super::UINT, factorylist: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn InsertEntry(&self, before: super::UINT, factory: windows_core::Ref<IUIAutomationProxyFactoryEntry>) -> windows_core::Result<()>;
    fn RemoveEntry(&self, index: super::UINT) -> windows_core::Result<()>;
    fn ClearTable(&self) -> windows_core::Result<()>;
    fn RestoreDefaultTable(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl IUIAutomationProxyFactoryMapping_Vtbl {
    pub const fn new<Identity: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Count<Identity: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationProxyFactoryMapping_Impl::get_Count(this) {
                    Ok(ok__) => {
                        count.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTable<Identity: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, table: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationProxyFactoryMapping_Impl::GetTable(this) {
                    Ok(ok__) => {
                        table.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEntry<Identity: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, entry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationProxyFactoryMapping_Impl::GetEntry(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        entry.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTable<Identity: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, factorylist: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationProxyFactoryMapping_Impl::SetTable(this, core::mem::transmute_copy(&factorylist)).into()
            }
        }
        unsafe extern "system" fn InsertEntries<Identity: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, before: super::UINT, factorylist: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationProxyFactoryMapping_Impl::InsertEntries(this, core::mem::transmute_copy(&before), core::mem::transmute_copy(&factorylist)).into()
            }
        }
        unsafe extern "system" fn InsertEntry<Identity: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, before: super::UINT, factory: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationProxyFactoryMapping_Impl::InsertEntry(this, core::mem::transmute_copy(&before), core::mem::transmute_copy(&factory)).into()
            }
        }
        unsafe extern "system" fn RemoveEntry<Identity: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationProxyFactoryMapping_Impl::RemoveEntry(this, core::mem::transmute_copy(&index)).into()
            }
        }
        unsafe extern "system" fn ClearTable<Identity: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationProxyFactoryMapping_Impl::ClearTable(this).into()
            }
        }
        unsafe extern "system" fn RestoreDefaultTable<Identity: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationProxyFactoryMapping_Impl::RestoreDefaultTable(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Count: get_Count::<Identity, OFFSET>,
            GetTable: GetTable::<Identity, OFFSET>,
            GetEntry: GetEntry::<Identity, OFFSET>,
            SetTable: SetTable::<Identity, OFFSET>,
            InsertEntries: InsertEntries::<Identity, OFFSET>,
            InsertEntry: InsertEntry::<Identity, OFFSET>,
            RemoveEntry: RemoveEntry::<Identity, OFFSET>,
            ClearTable: ClearTable::<Identity, OFFSET>,
            RestoreDefaultTable: RestoreDefaultTable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationProxyFactoryMapping as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IUIAutomationProxyFactoryMapping {}
windows_core::imp::define_interface!(IUIAutomationRangeValuePattern, IUIAutomationRangeValuePattern_Vtbl, 0x59213f4f_7346_49e5_b120_80555987a148);
windows_core::imp::interface_hierarchy!(IUIAutomationRangeValuePattern, windows_core::IUnknown);
impl IUIAutomationRangeValuePattern {
    pub unsafe fn SetValue(&self, val: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), val) }
    }
    pub unsafe fn get_CurrentValue(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentIsReadOnly(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentIsReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentMaximum(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentMaximum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentMinimum(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentMinimum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentLargeChange(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentLargeChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentSmallChange(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentSmallChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedValue(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedIsReadOnly(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedIsReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedMaximum(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedMaximum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedMinimum(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedMinimum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedLargeChange(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedLargeChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedSmallChange(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedSmallChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationRangeValuePattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub get_CurrentValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_CurrentIsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CurrentMaximum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_CurrentMinimum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_CurrentLargeChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_CurrentSmallChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_CachedValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_CachedIsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CachedMaximum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_CachedMinimum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_CachedLargeChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_CachedSmallChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
pub trait IUIAutomationRangeValuePattern_Impl: windows_core::IUnknownImpl {
    fn SetValue(&self, val: f64) -> windows_core::Result<()>;
    fn get_CurrentValue(&self) -> windows_core::Result<f64>;
    fn get_CurrentIsReadOnly(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CurrentMaximum(&self) -> windows_core::Result<f64>;
    fn get_CurrentMinimum(&self) -> windows_core::Result<f64>;
    fn get_CurrentLargeChange(&self) -> windows_core::Result<f64>;
    fn get_CurrentSmallChange(&self) -> windows_core::Result<f64>;
    fn get_CachedValue(&self) -> windows_core::Result<f64>;
    fn get_CachedIsReadOnly(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedMaximum(&self) -> windows_core::Result<f64>;
    fn get_CachedMinimum(&self) -> windows_core::Result<f64>;
    fn get_CachedLargeChange(&self) -> windows_core::Result<f64>;
    fn get_CachedSmallChange(&self) -> windows_core::Result<f64>;
}
impl IUIAutomationRangeValuePattern_Vtbl {
    pub const fn new<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetValue<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationRangeValuePattern_Impl::SetValue(this, core::mem::transmute_copy(&val)).into()
            }
        }
        unsafe extern "system" fn get_CurrentValue<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationRangeValuePattern_Impl::get_CurrentValue(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentIsReadOnly<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationRangeValuePattern_Impl::get_CurrentIsReadOnly(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentMaximum<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationRangeValuePattern_Impl::get_CurrentMaximum(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentMinimum<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationRangeValuePattern_Impl::get_CurrentMinimum(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentLargeChange<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationRangeValuePattern_Impl::get_CurrentLargeChange(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentSmallChange<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationRangeValuePattern_Impl::get_CurrentSmallChange(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedValue<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationRangeValuePattern_Impl::get_CachedValue(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedIsReadOnly<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationRangeValuePattern_Impl::get_CachedIsReadOnly(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedMaximum<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationRangeValuePattern_Impl::get_CachedMaximum(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedMinimum<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationRangeValuePattern_Impl::get_CachedMinimum(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedLargeChange<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationRangeValuePattern_Impl::get_CachedLargeChange(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedSmallChange<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationRangeValuePattern_Impl::get_CachedSmallChange(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetValue: SetValue::<Identity, OFFSET>,
            get_CurrentValue: get_CurrentValue::<Identity, OFFSET>,
            get_CurrentIsReadOnly: get_CurrentIsReadOnly::<Identity, OFFSET>,
            get_CurrentMaximum: get_CurrentMaximum::<Identity, OFFSET>,
            get_CurrentMinimum: get_CurrentMinimum::<Identity, OFFSET>,
            get_CurrentLargeChange: get_CurrentLargeChange::<Identity, OFFSET>,
            get_CurrentSmallChange: get_CurrentSmallChange::<Identity, OFFSET>,
            get_CachedValue: get_CachedValue::<Identity, OFFSET>,
            get_CachedIsReadOnly: get_CachedIsReadOnly::<Identity, OFFSET>,
            get_CachedMaximum: get_CachedMaximum::<Identity, OFFSET>,
            get_CachedMinimum: get_CachedMinimum::<Identity, OFFSET>,
            get_CachedLargeChange: get_CachedLargeChange::<Identity, OFFSET>,
            get_CachedSmallChange: get_CachedSmallChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationRangeValuePattern as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationRangeValuePattern {}
windows_core::imp::define_interface!(IUIAutomationScrollItemPattern, IUIAutomationScrollItemPattern_Vtbl, 0xb488300f_d015_4f19_9c29_bb595e3645ef);
windows_core::imp::interface_hierarchy!(IUIAutomationScrollItemPattern, windows_core::IUnknown);
impl IUIAutomationScrollItemPattern {
    pub unsafe fn ScrollIntoView(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ScrollIntoView)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationScrollItemPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ScrollIntoView: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAutomationScrollItemPattern_Impl: windows_core::IUnknownImpl {
    fn ScrollIntoView(&self) -> windows_core::Result<()>;
}
impl IUIAutomationScrollItemPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationScrollItemPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ScrollIntoView<Identity: IUIAutomationScrollItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationScrollItemPattern_Impl::ScrollIntoView(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ScrollIntoView: ScrollIntoView::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationScrollItemPattern as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationScrollItemPattern {}
windows_core::imp::define_interface!(IUIAutomationScrollPattern, IUIAutomationScrollPattern_Vtbl, 0x88f4d42a_e881_459d_a77c_73bbbb7e02dc);
windows_core::imp::interface_hierarchy!(IUIAutomationScrollPattern, windows_core::IUnknown);
impl IUIAutomationScrollPattern {
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn Scroll(&self, horizontalamount: super::ScrollAmount, verticalamount: super::ScrollAmount) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Scroll)(windows_core::Interface::as_raw(self), horizontalamount, verticalamount) }
    }
    pub unsafe fn SetScrollPercent(&self, horizontalpercent: f64, verticalpercent: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetScrollPercent)(windows_core::Interface::as_raw(self), horizontalpercent, verticalpercent) }
    }
    pub unsafe fn get_CurrentHorizontalScrollPercent(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentHorizontalScrollPercent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentVerticalScrollPercent(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentVerticalScrollPercent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentHorizontalViewSize(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentHorizontalViewSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentVerticalViewSize(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentVerticalViewSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentHorizontallyScrollable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentHorizontallyScrollable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentVerticallyScrollable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentVerticallyScrollable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedHorizontalScrollPercent(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedHorizontalScrollPercent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedVerticalScrollPercent(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedVerticalScrollPercent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedHorizontalViewSize(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedHorizontalViewSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedVerticalViewSize(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedVerticalViewSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedHorizontallyScrollable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedHorizontallyScrollable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedVerticallyScrollable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedVerticallyScrollable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationScrollPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "uiautomationcore")]
    pub Scroll: unsafe extern "system" fn(*mut core::ffi::c_void, super::ScrollAmount, super::ScrollAmount) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    Scroll: usize,
    pub SetScrollPercent: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub get_CurrentHorizontalScrollPercent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_CurrentVerticalScrollPercent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_CurrentHorizontalViewSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_CurrentVerticalViewSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_CurrentHorizontallyScrollable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CurrentVerticallyScrollable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CachedHorizontalScrollPercent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_CachedVerticalScrollPercent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_CachedHorizontalViewSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_CachedVerticalViewSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_CachedHorizontallyScrollable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CachedVerticallyScrollable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "uiautomationcore")]
pub trait IUIAutomationScrollPattern_Impl: windows_core::IUnknownImpl {
    fn Scroll(&self, horizontalamount: super::ScrollAmount, verticalamount: super::ScrollAmount) -> windows_core::Result<()>;
    fn SetScrollPercent(&self, horizontalpercent: f64, verticalpercent: f64) -> windows_core::Result<()>;
    fn get_CurrentHorizontalScrollPercent(&self) -> windows_core::Result<f64>;
    fn get_CurrentVerticalScrollPercent(&self) -> windows_core::Result<f64>;
    fn get_CurrentHorizontalViewSize(&self) -> windows_core::Result<f64>;
    fn get_CurrentVerticalViewSize(&self) -> windows_core::Result<f64>;
    fn get_CurrentHorizontallyScrollable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CurrentVerticallyScrollable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedHorizontalScrollPercent(&self) -> windows_core::Result<f64>;
    fn get_CachedVerticalScrollPercent(&self) -> windows_core::Result<f64>;
    fn get_CachedHorizontalViewSize(&self) -> windows_core::Result<f64>;
    fn get_CachedVerticalViewSize(&self) -> windows_core::Result<f64>;
    fn get_CachedHorizontallyScrollable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedVerticallyScrollable(&self) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(feature = "uiautomationcore")]
impl IUIAutomationScrollPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Scroll<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizontalamount: super::ScrollAmount, verticalamount: super::ScrollAmount) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationScrollPattern_Impl::Scroll(this, core::mem::transmute_copy(&horizontalamount), core::mem::transmute_copy(&verticalamount)).into()
            }
        }
        unsafe extern "system" fn SetScrollPercent<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizontalpercent: f64, verticalpercent: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationScrollPattern_Impl::SetScrollPercent(this, core::mem::transmute_copy(&horizontalpercent), core::mem::transmute_copy(&verticalpercent)).into()
            }
        }
        unsafe extern "system" fn get_CurrentHorizontalScrollPercent<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationScrollPattern_Impl::get_CurrentHorizontalScrollPercent(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentVerticalScrollPercent<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationScrollPattern_Impl::get_CurrentVerticalScrollPercent(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentHorizontalViewSize<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationScrollPattern_Impl::get_CurrentHorizontalViewSize(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentVerticalViewSize<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationScrollPattern_Impl::get_CurrentVerticalViewSize(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentHorizontallyScrollable<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationScrollPattern_Impl::get_CurrentHorizontallyScrollable(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentVerticallyScrollable<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationScrollPattern_Impl::get_CurrentVerticallyScrollable(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedHorizontalScrollPercent<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationScrollPattern_Impl::get_CachedHorizontalScrollPercent(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedVerticalScrollPercent<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationScrollPattern_Impl::get_CachedVerticalScrollPercent(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedHorizontalViewSize<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationScrollPattern_Impl::get_CachedHorizontalViewSize(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedVerticalViewSize<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationScrollPattern_Impl::get_CachedVerticalViewSize(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedHorizontallyScrollable<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationScrollPattern_Impl::get_CachedHorizontallyScrollable(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedVerticallyScrollable<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationScrollPattern_Impl::get_CachedVerticallyScrollable(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Scroll: Scroll::<Identity, OFFSET>,
            SetScrollPercent: SetScrollPercent::<Identity, OFFSET>,
            get_CurrentHorizontalScrollPercent: get_CurrentHorizontalScrollPercent::<Identity, OFFSET>,
            get_CurrentVerticalScrollPercent: get_CurrentVerticalScrollPercent::<Identity, OFFSET>,
            get_CurrentHorizontalViewSize: get_CurrentHorizontalViewSize::<Identity, OFFSET>,
            get_CurrentVerticalViewSize: get_CurrentVerticalViewSize::<Identity, OFFSET>,
            get_CurrentHorizontallyScrollable: get_CurrentHorizontallyScrollable::<Identity, OFFSET>,
            get_CurrentVerticallyScrollable: get_CurrentVerticallyScrollable::<Identity, OFFSET>,
            get_CachedHorizontalScrollPercent: get_CachedHorizontalScrollPercent::<Identity, OFFSET>,
            get_CachedVerticalScrollPercent: get_CachedVerticalScrollPercent::<Identity, OFFSET>,
            get_CachedHorizontalViewSize: get_CachedHorizontalViewSize::<Identity, OFFSET>,
            get_CachedVerticalViewSize: get_CachedVerticalViewSize::<Identity, OFFSET>,
            get_CachedHorizontallyScrollable: get_CachedHorizontallyScrollable::<Identity, OFFSET>,
            get_CachedVerticallyScrollable: get_CachedVerticallyScrollable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationScrollPattern as windows_core::Interface>::IID
    }
}
#[cfg(feature = "uiautomationcore")]
impl windows_core::RuntimeName for IUIAutomationScrollPattern {}
windows_core::imp::define_interface!(IUIAutomationSelectionItemPattern, IUIAutomationSelectionItemPattern_Vtbl, 0xa8efa66a_0fda_421a_9194_38021f3578ea);
windows_core::imp::interface_hierarchy!(IUIAutomationSelectionItemPattern, windows_core::IUnknown);
impl IUIAutomationSelectionItemPattern {
    pub unsafe fn Select(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Select)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AddToSelection(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddToSelection)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn RemoveFromSelection(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveFromSelection)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn get_CurrentIsSelected(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentIsSelected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentSelectionContainer(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentSelectionContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CachedIsSelected(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedIsSelected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedSelectionContainer(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedSelectionContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationSelectionItemPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Select: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddToSelection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveFromSelection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentIsSelected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CurrentSelectionContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedIsSelected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CachedSelectionContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAutomationSelectionItemPattern_Impl: windows_core::IUnknownImpl {
    fn Select(&self) -> windows_core::Result<()>;
    fn AddToSelection(&self) -> windows_core::Result<()>;
    fn RemoveFromSelection(&self) -> windows_core::Result<()>;
    fn get_CurrentIsSelected(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CurrentSelectionContainer(&self) -> windows_core::Result<IUIAutomationElement>;
    fn get_CachedIsSelected(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedSelectionContainer(&self) -> windows_core::Result<IUIAutomationElement>;
}
impl IUIAutomationSelectionItemPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Select<Identity: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationSelectionItemPattern_Impl::Select(this).into()
            }
        }
        unsafe extern "system" fn AddToSelection<Identity: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationSelectionItemPattern_Impl::AddToSelection(this).into()
            }
        }
        unsafe extern "system" fn RemoveFromSelection<Identity: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationSelectionItemPattern_Impl::RemoveFromSelection(this).into()
            }
        }
        unsafe extern "system" fn get_CurrentIsSelected<Identity: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionItemPattern_Impl::get_CurrentIsSelected(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentSelectionContainer<Identity: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionItemPattern_Impl::get_CurrentSelectionContainer(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedIsSelected<Identity: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionItemPattern_Impl::get_CachedIsSelected(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedSelectionContainer<Identity: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionItemPattern_Impl::get_CachedSelectionContainer(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Select: Select::<Identity, OFFSET>,
            AddToSelection: AddToSelection::<Identity, OFFSET>,
            RemoveFromSelection: RemoveFromSelection::<Identity, OFFSET>,
            get_CurrentIsSelected: get_CurrentIsSelected::<Identity, OFFSET>,
            get_CurrentSelectionContainer: get_CurrentSelectionContainer::<Identity, OFFSET>,
            get_CachedIsSelected: get_CachedIsSelected::<Identity, OFFSET>,
            get_CachedSelectionContainer: get_CachedSelectionContainer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationSelectionItemPattern as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationSelectionItemPattern {}
windows_core::imp::define_interface!(IUIAutomationSelectionPattern, IUIAutomationSelectionPattern_Vtbl, 0x5ed5202e_b2ac_47a6_b638_4b0bf140d78e);
windows_core::imp::interface_hierarchy!(IUIAutomationSelectionPattern, windows_core::IUnknown);
impl IUIAutomationSelectionPattern {
    pub unsafe fn GetCurrentSelection(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentSelection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CurrentCanSelectMultiple(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentCanSelectMultiple)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentIsSelectionRequired(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentIsSelectionRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCachedSelection(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedSelection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CachedCanSelectMultiple(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedCanSelectMultiple)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedIsSelectionRequired(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedIsSelectionRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationSelectionPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrentSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentCanSelectMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CurrentIsSelectionRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetCachedSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedCanSelectMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CachedIsSelectionRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IUIAutomationSelectionPattern_Impl: windows_core::IUnknownImpl {
    fn GetCurrentSelection(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn get_CurrentCanSelectMultiple(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CurrentIsSelectionRequired(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetCachedSelection(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn get_CachedCanSelectMultiple(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedIsSelectionRequired(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IUIAutomationSelectionPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrentSelection<Identity: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionPattern_Impl::GetCurrentSelection(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentCanSelectMultiple<Identity: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionPattern_Impl::get_CurrentCanSelectMultiple(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentIsSelectionRequired<Identity: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionPattern_Impl::get_CurrentIsSelectionRequired(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCachedSelection<Identity: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionPattern_Impl::GetCachedSelection(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedCanSelectMultiple<Identity: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionPattern_Impl::get_CachedCanSelectMultiple(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedIsSelectionRequired<Identity: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionPattern_Impl::get_CachedIsSelectionRequired(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentSelection: GetCurrentSelection::<Identity, OFFSET>,
            get_CurrentCanSelectMultiple: get_CurrentCanSelectMultiple::<Identity, OFFSET>,
            get_CurrentIsSelectionRequired: get_CurrentIsSelectionRequired::<Identity, OFFSET>,
            GetCachedSelection: GetCachedSelection::<Identity, OFFSET>,
            get_CachedCanSelectMultiple: get_CachedCanSelectMultiple::<Identity, OFFSET>,
            get_CachedIsSelectionRequired: get_CachedIsSelectionRequired::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationSelectionPattern as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationSelectionPattern {}
windows_core::imp::define_interface!(IUIAutomationSelectionPattern2, IUIAutomationSelectionPattern2_Vtbl, 0x0532bfae_c011_4e32_a343_6d642d798555);
impl core::ops::Deref for IUIAutomationSelectionPattern2 {
    type Target = IUIAutomationSelectionPattern;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationSelectionPattern2, windows_core::IUnknown, IUIAutomationSelectionPattern);
impl IUIAutomationSelectionPattern2 {
    pub unsafe fn get_CurrentFirstSelectedItem(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentFirstSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CurrentLastSelectedItem(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentLastSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CurrentCurrentSelectedItem(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentCurrentSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CurrentItemCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentItemCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedFirstSelectedItem(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedFirstSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CachedLastSelectedItem(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedLastSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CachedCurrentSelectedItem(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedCurrentSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CachedItemCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedItemCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationSelectionPattern2_Vtbl {
    pub base__: IUIAutomationSelectionPattern_Vtbl,
    pub get_CurrentFirstSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentLastSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentCurrentSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentItemCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CachedFirstSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedLastSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedCurrentSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedItemCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
pub trait IUIAutomationSelectionPattern2_Impl: IUIAutomationSelectionPattern_Impl {
    fn get_CurrentFirstSelectedItem(&self) -> windows_core::Result<IUIAutomationElement>;
    fn get_CurrentLastSelectedItem(&self) -> windows_core::Result<IUIAutomationElement>;
    fn get_CurrentCurrentSelectedItem(&self) -> windows_core::Result<IUIAutomationElement>;
    fn get_CurrentItemCount(&self) -> windows_core::Result<i32>;
    fn get_CachedFirstSelectedItem(&self) -> windows_core::Result<IUIAutomationElement>;
    fn get_CachedLastSelectedItem(&self) -> windows_core::Result<IUIAutomationElement>;
    fn get_CachedCurrentSelectedItem(&self) -> windows_core::Result<IUIAutomationElement>;
    fn get_CachedItemCount(&self) -> windows_core::Result<i32>;
}
impl IUIAutomationSelectionPattern2_Vtbl {
    pub const fn new<Identity: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_CurrentFirstSelectedItem<Identity: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionPattern2_Impl::get_CurrentFirstSelectedItem(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentLastSelectedItem<Identity: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionPattern2_Impl::get_CurrentLastSelectedItem(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentCurrentSelectedItem<Identity: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionPattern2_Impl::get_CurrentCurrentSelectedItem(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentItemCount<Identity: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionPattern2_Impl::get_CurrentItemCount(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedFirstSelectedItem<Identity: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionPattern2_Impl::get_CachedFirstSelectedItem(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedLastSelectedItem<Identity: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionPattern2_Impl::get_CachedLastSelectedItem(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedCurrentSelectedItem<Identity: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionPattern2_Impl::get_CachedCurrentSelectedItem(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedItemCount<Identity: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionPattern2_Impl::get_CachedItemCount(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IUIAutomationSelectionPattern_Vtbl::new::<Identity, OFFSET>(),
            get_CurrentFirstSelectedItem: get_CurrentFirstSelectedItem::<Identity, OFFSET>,
            get_CurrentLastSelectedItem: get_CurrentLastSelectedItem::<Identity, OFFSET>,
            get_CurrentCurrentSelectedItem: get_CurrentCurrentSelectedItem::<Identity, OFFSET>,
            get_CurrentItemCount: get_CurrentItemCount::<Identity, OFFSET>,
            get_CachedFirstSelectedItem: get_CachedFirstSelectedItem::<Identity, OFFSET>,
            get_CachedLastSelectedItem: get_CachedLastSelectedItem::<Identity, OFFSET>,
            get_CachedCurrentSelectedItem: get_CachedCurrentSelectedItem::<Identity, OFFSET>,
            get_CachedItemCount: get_CachedItemCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationSelectionPattern2 as windows_core::Interface>::IID || iid == &<IUIAutomationSelectionPattern as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationSelectionPattern2 {}
windows_core::imp::define_interface!(IUIAutomationSpreadsheetItemPattern, IUIAutomationSpreadsheetItemPattern_Vtbl, 0x7d4fb86c_8d34_40e1_8e83_62c15204e335);
windows_core::imp::interface_hierarchy!(IUIAutomationSpreadsheetItemPattern, windows_core::IUnknown);
impl IUIAutomationSpreadsheetItemPattern {
    pub unsafe fn get_CurrentFormula(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentFormula)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetCurrentAnnotationObjects(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentAnnotationObjects)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetCurrentAnnotationTypes(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentAnnotationTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedFormula(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedFormula)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetCachedAnnotationObjects(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedAnnotationObjects)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetCachedAnnotationTypes(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedAnnotationTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationSpreadsheetItemPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_CurrentFormula: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentAnnotationObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetCurrentAnnotationTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetCurrentAnnotationTypes: usize,
    pub get_CachedFormula: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCachedAnnotationObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetCachedAnnotationTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetCachedAnnotationTypes: usize,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
pub trait IUIAutomationSpreadsheetItemPattern_Impl: windows_core::IUnknownImpl {
    fn get_CurrentFormula(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetCurrentAnnotationObjects(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn GetCurrentAnnotationTypes(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn get_CachedFormula(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetCachedAnnotationObjects(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn GetCachedAnnotationTypes(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl IUIAutomationSpreadsheetItemPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_CurrentFormula<Identity: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSpreadsheetItemPattern_Impl::get_CurrentFormula(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentAnnotationObjects<Identity: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSpreadsheetItemPattern_Impl::GetCurrentAnnotationObjects(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentAnnotationTypes<Identity: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSpreadsheetItemPattern_Impl::GetCurrentAnnotationTypes(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedFormula<Identity: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSpreadsheetItemPattern_Impl::get_CachedFormula(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCachedAnnotationObjects<Identity: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSpreadsheetItemPattern_Impl::GetCachedAnnotationObjects(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCachedAnnotationTypes<Identity: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSpreadsheetItemPattern_Impl::GetCachedAnnotationTypes(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_CurrentFormula: get_CurrentFormula::<Identity, OFFSET>,
            GetCurrentAnnotationObjects: GetCurrentAnnotationObjects::<Identity, OFFSET>,
            GetCurrentAnnotationTypes: GetCurrentAnnotationTypes::<Identity, OFFSET>,
            get_CachedFormula: get_CachedFormula::<Identity, OFFSET>,
            GetCachedAnnotationObjects: GetCachedAnnotationObjects::<Identity, OFFSET>,
            GetCachedAnnotationTypes: GetCachedAnnotationTypes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationSpreadsheetItemPattern as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IUIAutomationSpreadsheetItemPattern {}
windows_core::imp::define_interface!(IUIAutomationSpreadsheetPattern, IUIAutomationSpreadsheetPattern_Vtbl, 0x7517a7c8_faae_4de9_9f08_29b91e8595c1);
windows_core::imp::interface_hierarchy!(IUIAutomationSpreadsheetPattern, windows_core::IUnknown);
impl IUIAutomationSpreadsheetPattern {
    pub unsafe fn GetItemByName(&self, name: &windows_core::BSTR) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemByName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationSpreadsheetPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetItemByName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAutomationSpreadsheetPattern_Impl: windows_core::IUnknownImpl {
    fn GetItemByName(&self, name: &windows_core::BSTR) -> windows_core::Result<IUIAutomationElement>;
}
impl IUIAutomationSpreadsheetPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationSpreadsheetPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetItemByName<Identity: IUIAutomationSpreadsheetPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: *mut core::ffi::c_void, element: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSpreadsheetPattern_Impl::GetItemByName(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        element.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetItemByName: GetItemByName::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationSpreadsheetPattern as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationSpreadsheetPattern {}
windows_core::imp::define_interface!(IUIAutomationStructureChangedEventHandler, IUIAutomationStructureChangedEventHandler_Vtbl, 0xe81d1b4e_11c5_42f8_9754_e7036c79f054);
windows_core::imp::interface_hierarchy!(IUIAutomationStructureChangedEventHandler, windows_core::IUnknown);
impl IUIAutomationStructureChangedEventHandler {
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt"))]
    pub unsafe fn HandleStructureChangedEvent<P0>(&self, sender: P0, changetype: super::StructureChangeType, runtimeid: *const super::SAFEARRAY) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe { (windows_core::Interface::vtable(self).HandleStructureChangedEvent)(windows_core::Interface::as_raw(self), sender.param().abi(), changetype, runtimeid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationStructureChangedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt"))]
    pub HandleStructureChangedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::StructureChangeType, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt")))]
    HandleStructureChangedEvent: usize,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt"))]
pub trait IUIAutomationStructureChangedEventHandler_Impl: windows_core::IUnknownImpl {
    fn HandleStructureChangedEvent(&self, sender: windows_core::Ref<IUIAutomationElement>, changetype: super::StructureChangeType, runtimeid: *const super::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt"))]
impl IUIAutomationStructureChangedEventHandler_Vtbl {
    pub const fn new<Identity: IUIAutomationStructureChangedEventHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HandleStructureChangedEvent<Identity: IUIAutomationStructureChangedEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, changetype: super::StructureChangeType, runtimeid: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationStructureChangedEventHandler_Impl::HandleStructureChangedEvent(this, core::mem::transmute_copy(&sender), core::mem::transmute_copy(&changetype), core::mem::transmute_copy(&runtimeid)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), HandleStructureChangedEvent: HandleStructureChangedEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationStructureChangedEventHandler as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt"))]
impl windows_core::RuntimeName for IUIAutomationStructureChangedEventHandler {}
windows_core::imp::define_interface!(IUIAutomationStylesPattern, IUIAutomationStylesPattern_Vtbl, 0x85b5f0a2_bd79_484a_ad2b_388c9838d5fb);
windows_core::imp::interface_hierarchy!(IUIAutomationStylesPattern, windows_core::IUnknown);
impl IUIAutomationStylesPattern {
    pub unsafe fn get_CurrentStyleId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentStyleId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentStyleName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentStyleName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CurrentFillColor(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentFillColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentFillPatternStyle(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentFillPatternStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CurrentShape(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentShape)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CurrentFillPatternColor(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentFillPatternColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentExtendedProperties(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentExtendedProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetCurrentExtendedPropertiesAsArray(&self, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentExtendedPropertiesAsArray)(windows_core::Interface::as_raw(self), propertyarray as _, propertycount as _) }
    }
    pub unsafe fn get_CachedStyleId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedStyleId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedStyleName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedStyleName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CachedFillColor(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedFillColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedFillPatternStyle(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedFillPatternStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CachedShape(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedShape)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CachedFillPatternColor(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedFillPatternColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedExtendedProperties(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedExtendedProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetCachedExtendedPropertiesAsArray(&self, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCachedExtendedPropertiesAsArray)(windows_core::Interface::as_raw(self), propertyarray as _, propertycount as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationStylesPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_CurrentStyleId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CurrentStyleName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentFillColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CurrentFillPatternStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentShape: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentFillPatternColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CurrentExtendedProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentExtendedPropertiesAsArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut ExtendedProperty, *mut i32) -> windows_core::HRESULT,
    pub get_CachedStyleId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CachedStyleName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedFillColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CachedFillPatternStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedShape: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedFillPatternColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_CachedExtendedProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCachedExtendedPropertiesAsArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut ExtendedProperty, *mut i32) -> windows_core::HRESULT,
}
pub trait IUIAutomationStylesPattern_Impl: windows_core::IUnknownImpl {
    fn get_CurrentStyleId(&self) -> windows_core::Result<i32>;
    fn get_CurrentStyleName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CurrentFillColor(&self) -> windows_core::Result<i32>;
    fn get_CurrentFillPatternStyle(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CurrentShape(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CurrentFillPatternColor(&self) -> windows_core::Result<i32>;
    fn get_CurrentExtendedProperties(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetCurrentExtendedPropertiesAsArray(&self, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> windows_core::Result<()>;
    fn get_CachedStyleId(&self) -> windows_core::Result<i32>;
    fn get_CachedStyleName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedFillColor(&self) -> windows_core::Result<i32>;
    fn get_CachedFillPatternStyle(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedShape(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedFillPatternColor(&self) -> windows_core::Result<i32>;
    fn get_CachedExtendedProperties(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetCachedExtendedPropertiesAsArray(&self, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> windows_core::Result<()>;
}
impl IUIAutomationStylesPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_CurrentStyleId<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::get_CurrentStyleId(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentStyleName<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::get_CurrentStyleName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentFillColor<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::get_CurrentFillColor(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentFillPatternStyle<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::get_CurrentFillPatternStyle(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentShape<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::get_CurrentShape(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentFillPatternColor<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::get_CurrentFillPatternColor(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentExtendedProperties<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::get_CurrentExtendedProperties(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentExtendedPropertiesAsArray<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationStylesPattern_Impl::GetCurrentExtendedPropertiesAsArray(this, core::mem::transmute_copy(&propertyarray), core::mem::transmute_copy(&propertycount)).into()
            }
        }
        unsafe extern "system" fn get_CachedStyleId<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::get_CachedStyleId(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedStyleName<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::get_CachedStyleName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedFillColor<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::get_CachedFillColor(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedFillPatternStyle<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::get_CachedFillPatternStyle(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedShape<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::get_CachedShape(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedFillPatternColor<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::get_CachedFillPatternColor(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedExtendedProperties<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::get_CachedExtendedProperties(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCachedExtendedPropertiesAsArray<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationStylesPattern_Impl::GetCachedExtendedPropertiesAsArray(this, core::mem::transmute_copy(&propertyarray), core::mem::transmute_copy(&propertycount)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_CurrentStyleId: get_CurrentStyleId::<Identity, OFFSET>,
            get_CurrentStyleName: get_CurrentStyleName::<Identity, OFFSET>,
            get_CurrentFillColor: get_CurrentFillColor::<Identity, OFFSET>,
            get_CurrentFillPatternStyle: get_CurrentFillPatternStyle::<Identity, OFFSET>,
            get_CurrentShape: get_CurrentShape::<Identity, OFFSET>,
            get_CurrentFillPatternColor: get_CurrentFillPatternColor::<Identity, OFFSET>,
            get_CurrentExtendedProperties: get_CurrentExtendedProperties::<Identity, OFFSET>,
            GetCurrentExtendedPropertiesAsArray: GetCurrentExtendedPropertiesAsArray::<Identity, OFFSET>,
            get_CachedStyleId: get_CachedStyleId::<Identity, OFFSET>,
            get_CachedStyleName: get_CachedStyleName::<Identity, OFFSET>,
            get_CachedFillColor: get_CachedFillColor::<Identity, OFFSET>,
            get_CachedFillPatternStyle: get_CachedFillPatternStyle::<Identity, OFFSET>,
            get_CachedShape: get_CachedShape::<Identity, OFFSET>,
            get_CachedFillPatternColor: get_CachedFillPatternColor::<Identity, OFFSET>,
            get_CachedExtendedProperties: get_CachedExtendedProperties::<Identity, OFFSET>,
            GetCachedExtendedPropertiesAsArray: GetCachedExtendedPropertiesAsArray::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationStylesPattern as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationStylesPattern {}
windows_core::imp::define_interface!(IUIAutomationSynchronizedInputPattern, IUIAutomationSynchronizedInputPattern_Vtbl, 0x2233be0b_afb7_448b_9fda_3b378aa5eae1);
windows_core::imp::interface_hierarchy!(IUIAutomationSynchronizedInputPattern, windows_core::IUnknown);
impl IUIAutomationSynchronizedInputPattern {
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn StartListening(&self, inputtype: super::SynchronizedInputType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StartListening)(windows_core::Interface::as_raw(self), inputtype) }
    }
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationSynchronizedInputPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "uiautomationcore")]
    pub StartListening: unsafe extern "system" fn(*mut core::ffi::c_void, super::SynchronizedInputType) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    StartListening: usize,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "uiautomationcore")]
pub trait IUIAutomationSynchronizedInputPattern_Impl: windows_core::IUnknownImpl {
    fn StartListening(&self, inputtype: super::SynchronizedInputType) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "uiautomationcore")]
impl IUIAutomationSynchronizedInputPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationSynchronizedInputPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartListening<Identity: IUIAutomationSynchronizedInputPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputtype: super::SynchronizedInputType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationSynchronizedInputPattern_Impl::StartListening(this, core::mem::transmute_copy(&inputtype)).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: IUIAutomationSynchronizedInputPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationSynchronizedInputPattern_Impl::Cancel(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartListening: StartListening::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationSynchronizedInputPattern as windows_core::Interface>::IID
    }
}
#[cfg(feature = "uiautomationcore")]
impl windows_core::RuntimeName for IUIAutomationSynchronizedInputPattern {}
windows_core::imp::define_interface!(IUIAutomationTableItemPattern, IUIAutomationTableItemPattern_Vtbl, 0x0b964eb3_ef2e_4464_9c79_61d61737a27e);
windows_core::imp::interface_hierarchy!(IUIAutomationTableItemPattern, windows_core::IUnknown);
impl IUIAutomationTableItemPattern {
    pub unsafe fn GetCurrentRowHeaderItems(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentRowHeaderItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCurrentColumnHeaderItems(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentColumnHeaderItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCachedRowHeaderItems(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedRowHeaderItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCachedColumnHeaderItems(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedColumnHeaderItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTableItemPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrentRowHeaderItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentColumnHeaderItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCachedRowHeaderItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCachedColumnHeaderItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAutomationTableItemPattern_Impl: windows_core::IUnknownImpl {
    fn GetCurrentRowHeaderItems(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn GetCurrentColumnHeaderItems(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn GetCachedRowHeaderItems(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn GetCachedColumnHeaderItems(&self) -> windows_core::Result<IUIAutomationElementArray>;
}
impl IUIAutomationTableItemPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationTableItemPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrentRowHeaderItems<Identity: IUIAutomationTableItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTableItemPattern_Impl::GetCurrentRowHeaderItems(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentColumnHeaderItems<Identity: IUIAutomationTableItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTableItemPattern_Impl::GetCurrentColumnHeaderItems(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCachedRowHeaderItems<Identity: IUIAutomationTableItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTableItemPattern_Impl::GetCachedRowHeaderItems(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCachedColumnHeaderItems<Identity: IUIAutomationTableItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTableItemPattern_Impl::GetCachedColumnHeaderItems(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentRowHeaderItems: GetCurrentRowHeaderItems::<Identity, OFFSET>,
            GetCurrentColumnHeaderItems: GetCurrentColumnHeaderItems::<Identity, OFFSET>,
            GetCachedRowHeaderItems: GetCachedRowHeaderItems::<Identity, OFFSET>,
            GetCachedColumnHeaderItems: GetCachedColumnHeaderItems::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationTableItemPattern as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationTableItemPattern {}
windows_core::imp::define_interface!(IUIAutomationTablePattern, IUIAutomationTablePattern_Vtbl, 0x620e691c_ea96_4710_a850_754b24ce2417);
windows_core::imp::interface_hierarchy!(IUIAutomationTablePattern, windows_core::IUnknown);
impl IUIAutomationTablePattern {
    pub unsafe fn GetCurrentRowHeaders(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentRowHeaders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCurrentColumnHeaders(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentColumnHeaders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn get_CurrentRowOrColumnMajor(&self) -> windows_core::Result<super::RowOrColumnMajor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentRowOrColumnMajor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCachedRowHeaders(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedRowHeaders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCachedColumnHeaders(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedColumnHeaders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn get_CachedRowOrColumnMajor(&self) -> windows_core::Result<super::RowOrColumnMajor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedRowOrColumnMajor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTablePattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrentRowHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentColumnHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub get_CurrentRowOrColumnMajor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RowOrColumnMajor) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    get_CurrentRowOrColumnMajor: usize,
    pub GetCachedRowHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCachedColumnHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub get_CachedRowOrColumnMajor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::RowOrColumnMajor) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    get_CachedRowOrColumnMajor: usize,
}
#[cfg(feature = "uiautomationcore")]
pub trait IUIAutomationTablePattern_Impl: windows_core::IUnknownImpl {
    fn GetCurrentRowHeaders(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn GetCurrentColumnHeaders(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn get_CurrentRowOrColumnMajor(&self) -> windows_core::Result<super::RowOrColumnMajor>;
    fn GetCachedRowHeaders(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn GetCachedColumnHeaders(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn get_CachedRowOrColumnMajor(&self) -> windows_core::Result<super::RowOrColumnMajor>;
}
#[cfg(feature = "uiautomationcore")]
impl IUIAutomationTablePattern_Vtbl {
    pub const fn new<Identity: IUIAutomationTablePattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetCurrentRowHeaders<Identity: IUIAutomationTablePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTablePattern_Impl::GetCurrentRowHeaders(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentColumnHeaders<Identity: IUIAutomationTablePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTablePattern_Impl::GetCurrentColumnHeaders(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentRowOrColumnMajor<Identity: IUIAutomationTablePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::RowOrColumnMajor) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTablePattern_Impl::get_CurrentRowOrColumnMajor(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCachedRowHeaders<Identity: IUIAutomationTablePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTablePattern_Impl::GetCachedRowHeaders(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCachedColumnHeaders<Identity: IUIAutomationTablePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTablePattern_Impl::GetCachedColumnHeaders(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedRowOrColumnMajor<Identity: IUIAutomationTablePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::RowOrColumnMajor) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTablePattern_Impl::get_CachedRowOrColumnMajor(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCurrentRowHeaders: GetCurrentRowHeaders::<Identity, OFFSET>,
            GetCurrentColumnHeaders: GetCurrentColumnHeaders::<Identity, OFFSET>,
            get_CurrentRowOrColumnMajor: get_CurrentRowOrColumnMajor::<Identity, OFFSET>,
            GetCachedRowHeaders: GetCachedRowHeaders::<Identity, OFFSET>,
            GetCachedColumnHeaders: GetCachedColumnHeaders::<Identity, OFFSET>,
            get_CachedRowOrColumnMajor: get_CachedRowOrColumnMajor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationTablePattern as windows_core::Interface>::IID
    }
}
#[cfg(feature = "uiautomationcore")]
impl windows_core::RuntimeName for IUIAutomationTablePattern {}
windows_core::imp::define_interface!(IUIAutomationTextChildPattern, IUIAutomationTextChildPattern_Vtbl, 0x6552b038_ae05_40c8_abfd_aa08352aab86);
windows_core::imp::interface_hierarchy!(IUIAutomationTextChildPattern, windows_core::IUnknown);
impl IUIAutomationTextChildPattern {
    pub unsafe fn get_TextContainer(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TextContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_TextRange(&self) -> windows_core::Result<IUIAutomationTextRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TextRange)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTextChildPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_TextContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_TextRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAutomationTextChildPattern_Impl: windows_core::IUnknownImpl {
    fn get_TextContainer(&self) -> windows_core::Result<IUIAutomationElement>;
    fn get_TextRange(&self) -> windows_core::Result<IUIAutomationTextRange>;
}
impl IUIAutomationTextChildPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationTextChildPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_TextContainer<Identity: IUIAutomationTextChildPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, container: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextChildPattern_Impl::get_TextContainer(this) {
                    Ok(ok__) => {
                        container.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_TextRange<Identity: IUIAutomationTextChildPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, range: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextChildPattern_Impl::get_TextRange(this) {
                    Ok(ok__) => {
                        range.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_TextContainer: get_TextContainer::<Identity, OFFSET>,
            get_TextRange: get_TextRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationTextChildPattern as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationTextChildPattern {}
windows_core::imp::define_interface!(IUIAutomationTextEditPattern, IUIAutomationTextEditPattern_Vtbl, 0x17e21576_996c_4870_99d9_bff323380c06);
impl core::ops::Deref for IUIAutomationTextEditPattern {
    type Target = IUIAutomationTextPattern;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationTextEditPattern, windows_core::IUnknown, IUIAutomationTextPattern);
impl IUIAutomationTextEditPattern {
    pub unsafe fn GetActiveComposition(&self) -> windows_core::Result<IUIAutomationTextRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActiveComposition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetConversionTarget(&self) -> windows_core::Result<IUIAutomationTextRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConversionTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTextEditPattern_Vtbl {
    pub base__: IUIAutomationTextPattern_Vtbl,
    pub GetActiveComposition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetConversionTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "uiautomationcore", feature = "windef"))]
pub trait IUIAutomationTextEditPattern_Impl: IUIAutomationTextPattern_Impl {
    fn GetActiveComposition(&self) -> windows_core::Result<IUIAutomationTextRange>;
    fn GetConversionTarget(&self) -> windows_core::Result<IUIAutomationTextRange>;
}
#[cfg(all(feature = "shared", feature = "uiautomationcore", feature = "windef"))]
impl IUIAutomationTextEditPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationTextEditPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetActiveComposition<Identity: IUIAutomationTextEditPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, range: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextEditPattern_Impl::GetActiveComposition(this) {
                    Ok(ok__) => {
                        range.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConversionTarget<Identity: IUIAutomationTextEditPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, range: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextEditPattern_Impl::GetConversionTarget(this) {
                    Ok(ok__) => {
                        range.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IUIAutomationTextPattern_Vtbl::new::<Identity, OFFSET>(),
            GetActiveComposition: GetActiveComposition::<Identity, OFFSET>,
            GetConversionTarget: GetConversionTarget::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationTextEditPattern as windows_core::Interface>::IID || iid == &<IUIAutomationTextPattern as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "uiautomationcore", feature = "windef"))]
impl windows_core::RuntimeName for IUIAutomationTextEditPattern {}
windows_core::imp::define_interface!(IUIAutomationTextEditTextChangedEventHandler, IUIAutomationTextEditTextChangedEventHandler_Vtbl, 0x92faa680_e704_4156_931a_e32d5bb38f3f);
windows_core::imp::interface_hierarchy!(IUIAutomationTextEditTextChangedEventHandler, windows_core::IUnknown);
impl IUIAutomationTextEditTextChangedEventHandler {
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt"))]
    pub unsafe fn HandleTextEditTextChangedEvent<P0>(&self, sender: P0, texteditchangetype: super::TextEditChangeType, eventstrings: *const super::SAFEARRAY) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe { (windows_core::Interface::vtable(self).HandleTextEditTextChangedEvent)(windows_core::Interface::as_raw(self), sender.param().abi(), texteditchangetype, eventstrings) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTextEditTextChangedEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt"))]
    pub HandleTextEditTextChangedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::TextEditChangeType, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt")))]
    HandleTextEditTextChangedEvent: usize,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt"))]
pub trait IUIAutomationTextEditTextChangedEventHandler_Impl: windows_core::IUnknownImpl {
    fn HandleTextEditTextChangedEvent(&self, sender: windows_core::Ref<IUIAutomationElement>, texteditchangetype: super::TextEditChangeType, eventstrings: *const super::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt"))]
impl IUIAutomationTextEditTextChangedEventHandler_Vtbl {
    pub const fn new<Identity: IUIAutomationTextEditTextChangedEventHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HandleTextEditTextChangedEvent<Identity: IUIAutomationTextEditTextChangedEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, texteditchangetype: super::TextEditChangeType, eventstrings: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationTextEditTextChangedEventHandler_Impl::HandleTextEditTextChangedEvent(this, core::mem::transmute_copy(&sender), core::mem::transmute_copy(&texteditchangetype), core::mem::transmute_copy(&eventstrings)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            HandleTextEditTextChangedEvent: HandleTextEditTextChangedEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationTextEditTextChangedEventHandler as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt"))]
impl windows_core::RuntimeName for IUIAutomationTextEditTextChangedEventHandler {}
windows_core::imp::define_interface!(IUIAutomationTextPattern, IUIAutomationTextPattern_Vtbl, 0x32eba289_3583_42c9_9c59_3b6d9a1e9b6a);
windows_core::imp::interface_hierarchy!(IUIAutomationTextPattern, windows_core::IUnknown);
impl IUIAutomationTextPattern {
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn RangeFromPoint(&self, pt: super::POINT) -> windows_core::Result<IUIAutomationTextRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RangeFromPoint)(windows_core::Interface::as_raw(self), pt, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn RangeFromChild<P0>(&self, child: P0) -> windows_core::Result<IUIAutomationTextRange>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RangeFromChild)(windows_core::Interface::as_raw(self), child.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSelection(&self) -> windows_core::Result<IUIAutomationTextRangeArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetVisibleRanges(&self) -> windows_core::Result<IUIAutomationTextRangeArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVisibleRanges)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_DocumentRange(&self) -> windows_core::Result<IUIAutomationTextRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DocumentRange)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn get_SupportedTextSelection(&self) -> windows_core::Result<super::SupportedTextSelection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SupportedTextSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTextPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub RangeFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, super::POINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    RangeFromPoint: usize,
    pub RangeFromChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVisibleRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DocumentRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub get_SupportedTextSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::SupportedTextSelection) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    get_SupportedTextSelection: usize,
}
#[cfg(all(feature = "shared", feature = "uiautomationcore", feature = "windef"))]
pub trait IUIAutomationTextPattern_Impl: windows_core::IUnknownImpl {
    fn RangeFromPoint(&self, pt: &super::POINT) -> windows_core::Result<IUIAutomationTextRange>;
    fn RangeFromChild(&self, child: windows_core::Ref<IUIAutomationElement>) -> windows_core::Result<IUIAutomationTextRange>;
    fn GetSelection(&self) -> windows_core::Result<IUIAutomationTextRangeArray>;
    fn GetVisibleRanges(&self) -> windows_core::Result<IUIAutomationTextRangeArray>;
    fn get_DocumentRange(&self) -> windows_core::Result<IUIAutomationTextRange>;
    fn get_SupportedTextSelection(&self) -> windows_core::Result<super::SupportedTextSelection>;
}
#[cfg(all(feature = "shared", feature = "uiautomationcore", feature = "windef"))]
impl IUIAutomationTextPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationTextPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RangeFromPoint<Identity: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pt: super::POINT, range: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextPattern_Impl::RangeFromPoint(this, core::mem::transmute(&pt)) {
                    Ok(ok__) => {
                        range.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RangeFromChild<Identity: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, child: *mut core::ffi::c_void, range: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextPattern_Impl::RangeFromChild(this, core::mem::transmute_copy(&child)) {
                    Ok(ok__) => {
                        range.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSelection<Identity: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ranges: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextPattern_Impl::GetSelection(this) {
                    Ok(ok__) => {
                        ranges.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVisibleRanges<Identity: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ranges: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextPattern_Impl::GetVisibleRanges(this) {
                    Ok(ok__) => {
                        ranges.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DocumentRange<Identity: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, range: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextPattern_Impl::get_DocumentRange(this) {
                    Ok(ok__) => {
                        range.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SupportedTextSelection<Identity: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedtextselection: *mut super::SupportedTextSelection) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextPattern_Impl::get_SupportedTextSelection(this) {
                    Ok(ok__) => {
                        supportedtextselection.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RangeFromPoint: RangeFromPoint::<Identity, OFFSET>,
            RangeFromChild: RangeFromChild::<Identity, OFFSET>,
            GetSelection: GetSelection::<Identity, OFFSET>,
            GetVisibleRanges: GetVisibleRanges::<Identity, OFFSET>,
            get_DocumentRange: get_DocumentRange::<Identity, OFFSET>,
            get_SupportedTextSelection: get_SupportedTextSelection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationTextPattern as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "uiautomationcore", feature = "windef"))]
impl windows_core::RuntimeName for IUIAutomationTextPattern {}
windows_core::imp::define_interface!(IUIAutomationTextPattern2, IUIAutomationTextPattern2_Vtbl, 0x506a921a_fcc9_409f_b23b_37eb74106872);
impl core::ops::Deref for IUIAutomationTextPattern2 {
    type Target = IUIAutomationTextPattern;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationTextPattern2, windows_core::IUnknown, IUIAutomationTextPattern);
impl IUIAutomationTextPattern2 {
    pub unsafe fn RangeFromAnnotation<P0>(&self, annotation: P0) -> windows_core::Result<IUIAutomationTextRange>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RangeFromAnnotation)(windows_core::Interface::as_raw(self), annotation.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCaretRange(&self, isactive: *mut windows_core::BOOL) -> windows_core::Result<IUIAutomationTextRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCaretRange)(windows_core::Interface::as_raw(self), isactive as _, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTextPattern2_Vtbl {
    pub base__: IUIAutomationTextPattern_Vtbl,
    pub RangeFromAnnotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCaretRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "shared", feature = "uiautomationcore", feature = "windef"))]
pub trait IUIAutomationTextPattern2_Impl: IUIAutomationTextPattern_Impl {
    fn RangeFromAnnotation(&self, annotation: windows_core::Ref<IUIAutomationElement>) -> windows_core::Result<IUIAutomationTextRange>;
    fn GetCaretRange(&self, isactive: *mut windows_core::BOOL) -> windows_core::Result<IUIAutomationTextRange>;
}
#[cfg(all(feature = "shared", feature = "uiautomationcore", feature = "windef"))]
impl IUIAutomationTextPattern2_Vtbl {
    pub const fn new<Identity: IUIAutomationTextPattern2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RangeFromAnnotation<Identity: IUIAutomationTextPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, annotation: *mut core::ffi::c_void, range: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextPattern2_Impl::RangeFromAnnotation(this, core::mem::transmute_copy(&annotation)) {
                    Ok(ok__) => {
                        range.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCaretRange<Identity: IUIAutomationTextPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isactive: *mut windows_core::BOOL, range: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextPattern2_Impl::GetCaretRange(this, core::mem::transmute_copy(&isactive)) {
                    Ok(ok__) => {
                        range.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IUIAutomationTextPattern_Vtbl::new::<Identity, OFFSET>(),
            RangeFromAnnotation: RangeFromAnnotation::<Identity, OFFSET>,
            GetCaretRange: GetCaretRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationTextPattern2 as windows_core::Interface>::IID || iid == &<IUIAutomationTextPattern as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "shared", feature = "uiautomationcore", feature = "windef"))]
impl windows_core::RuntimeName for IUIAutomationTextPattern2 {}
windows_core::imp::define_interface!(IUIAutomationTextRange, IUIAutomationTextRange_Vtbl, 0xa543cc6a_f4ae_494b_8239_c814481187a8);
windows_core::imp::interface_hierarchy!(IUIAutomationTextRange, windows_core::IUnknown);
impl IUIAutomationTextRange {
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn Compare<P0>(&self, range: P0) -> windows_core::Result<windows_core::BOOL>
    where
        P0: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Compare)(windows_core::Interface::as_raw(self), range.param().abi(), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn CompareEndpoints<P1>(&self, srcendpoint: super::TextPatternRangeEndpoint, range: P1, targetendpoint: super::TextPatternRangeEndpoint) -> windows_core::Result<i32>
    where
        P1: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CompareEndpoints)(windows_core::Interface::as_raw(self), srcendpoint, range.param().abi(), targetendpoint, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn ExpandToEnclosingUnit(&self, textunit: super::TextUnit) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ExpandToEnclosingUnit)(windows_core::Interface::as_raw(self), textunit) }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn FindAttribute(&self, attr: super::TEXTATTRIBUTEID, val: &super::VARIANT, backward: bool) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindAttribute)(windows_core::Interface::as_raw(self), attr, core::mem::transmute_copy(val), backward.into(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindTextA(&self, text: &windows_core::BSTR, backward: bool, ignorecase: bool) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindTextA)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(text), backward.into(), ignorecase.into(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetAttributeValue(&self, attr: super::TEXTATTRIBUTEID) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttributeValue)(windows_core::Interface::as_raw(self), attr, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetBoundingRectangles(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBoundingRectangles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetEnclosingElement(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnclosingElement)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetText(&self, maxlength: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), maxlength, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn Move(&self, unit: super::TextUnit, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Move)(windows_core::Interface::as_raw(self), unit, count, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn MoveEndpointByUnit(&self, endpoint: super::TextPatternRangeEndpoint, unit: super::TextUnit, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveEndpointByUnit)(windows_core::Interface::as_raw(self), endpoint, unit, count, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn MoveEndpointByRange<P1>(&self, srcendpoint: super::TextPatternRangeEndpoint, range: P1, targetendpoint: super::TextPatternRangeEndpoint) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).MoveEndpointByRange)(windows_core::Interface::as_raw(self), srcendpoint, range.param().abi(), targetendpoint) }
    }
    pub unsafe fn Select(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Select)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AddToSelection(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddToSelection)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn RemoveFromSelection(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveFromSelection)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ScrollIntoView(&self, aligntotop: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ScrollIntoView)(windows_core::Interface::as_raw(self), aligntotop.into()) }
    }
    pub unsafe fn GetChildren(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChildren)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTextRange_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Compare: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub CompareEndpoints: unsafe extern "system" fn(*mut core::ffi::c_void, super::TextPatternRangeEndpoint, *mut core::ffi::c_void, super::TextPatternRangeEndpoint, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CompareEndpoints: usize,
    #[cfg(feature = "uiautomationcore")]
    pub ExpandToEnclosingUnit: unsafe extern "system" fn(*mut core::ffi::c_void, super::TextUnit) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    ExpandToEnclosingUnit: usize,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub FindAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, super::TEXTATTRIBUTEID, super::VARIANT, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    FindAttribute: usize,
    pub FindTextA: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetAttributeValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::TEXTATTRIBUTEID, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetAttributeValue: usize,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetBoundingRectangles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetBoundingRectangles: usize,
    pub GetEnclosingElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub Move: unsafe extern "system" fn(*mut core::ffi::c_void, super::TextUnit, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    Move: usize,
    #[cfg(feature = "uiautomationcore")]
    pub MoveEndpointByUnit: unsafe extern "system" fn(*mut core::ffi::c_void, super::TextPatternRangeEndpoint, super::TextUnit, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    MoveEndpointByUnit: usize,
    #[cfg(feature = "uiautomationcore")]
    pub MoveEndpointByRange: unsafe extern "system" fn(*mut core::ffi::c_void, super::TextPatternRangeEndpoint, *mut core::ffi::c_void, super::TextPatternRangeEndpoint) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    MoveEndpointByRange: usize,
    pub Select: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddToSelection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveFromSelection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ScrollIntoView: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetChildren: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationTextRange_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IUIAutomationTextRange>;
    fn Compare(&self, range: windows_core::Ref<IUIAutomationTextRange>) -> windows_core::Result<windows_core::BOOL>;
    fn CompareEndpoints(&self, srcendpoint: super::TextPatternRangeEndpoint, range: windows_core::Ref<IUIAutomationTextRange>, targetendpoint: super::TextPatternRangeEndpoint) -> windows_core::Result<i32>;
    fn ExpandToEnclosingUnit(&self, textunit: super::TextUnit) -> windows_core::Result<()>;
    fn FindAttribute(&self, attr: super::TEXTATTRIBUTEID, val: &super::VARIANT, backward: windows_core::BOOL) -> windows_core::Result<IUIAutomationTextRange>;
    fn FindTextA(&self, text: &windows_core::BSTR, backward: windows_core::BOOL, ignorecase: windows_core::BOOL) -> windows_core::Result<IUIAutomationTextRange>;
    fn GetAttributeValue(&self, attr: super::TEXTATTRIBUTEID) -> windows_core::Result<super::VARIANT>;
    fn GetBoundingRectangles(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn GetEnclosingElement(&self) -> windows_core::Result<IUIAutomationElement>;
    fn GetText(&self, maxlength: i32) -> windows_core::Result<windows_core::BSTR>;
    fn Move(&self, unit: super::TextUnit, count: i32) -> windows_core::Result<i32>;
    fn MoveEndpointByUnit(&self, endpoint: super::TextPatternRangeEndpoint, unit: super::TextUnit, count: i32) -> windows_core::Result<i32>;
    fn MoveEndpointByRange(&self, srcendpoint: super::TextPatternRangeEndpoint, range: windows_core::Ref<IUIAutomationTextRange>, targetendpoint: super::TextPatternRangeEndpoint) -> windows_core::Result<()>;
    fn Select(&self) -> windows_core::Result<()>;
    fn AddToSelection(&self) -> windows_core::Result<()>;
    fn RemoveFromSelection(&self) -> windows_core::Result<()>;
    fn ScrollIntoView(&self, aligntotop: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetChildren(&self) -> windows_core::Result<IUIAutomationElementArray>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationTextRange_Vtbl {
    pub const fn new<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clonedrange: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextRange_Impl::Clone(this) {
                    Ok(ok__) => {
                        clonedrange.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Compare<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, range: *mut core::ffi::c_void, aresame: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextRange_Impl::Compare(this, core::mem::transmute_copy(&range)) {
                    Ok(ok__) => {
                        aresame.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CompareEndpoints<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, srcendpoint: super::TextPatternRangeEndpoint, range: *mut core::ffi::c_void, targetendpoint: super::TextPatternRangeEndpoint, compvalue: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextRange_Impl::CompareEndpoints(this, core::mem::transmute_copy(&srcendpoint), core::mem::transmute_copy(&range), core::mem::transmute_copy(&targetendpoint)) {
                    Ok(ok__) => {
                        compvalue.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExpandToEnclosingUnit<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textunit: super::TextUnit) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationTextRange_Impl::ExpandToEnclosingUnit(this, core::mem::transmute_copy(&textunit)).into()
            }
        }
        unsafe extern "system" fn FindAttribute<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attr: super::TEXTATTRIBUTEID, val: super::VARIANT, backward: windows_core::BOOL, found: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextRange_Impl::FindAttribute(this, core::mem::transmute_copy(&attr), core::mem::transmute(&val), core::mem::transmute_copy(&backward)) {
                    Ok(ok__) => {
                        found.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindTextA<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut core::ffi::c_void, backward: windows_core::BOOL, ignorecase: windows_core::BOOL, found: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextRange_Impl::FindTextA(this, core::mem::transmute(&text), core::mem::transmute_copy(&backward), core::mem::transmute_copy(&ignorecase)) {
                    Ok(ok__) => {
                        found.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAttributeValue<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attr: super::TEXTATTRIBUTEID, value: *mut super::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextRange_Impl::GetAttributeValue(this, core::mem::transmute_copy(&attr)) {
                    Ok(ok__) => {
                        value.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBoundingRectangles<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, boundingrects: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextRange_Impl::GetBoundingRectangles(this) {
                    Ok(ok__) => {
                        boundingrects.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEnclosingElement<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, enclosingelement: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextRange_Impl::GetEnclosingElement(this) {
                    Ok(ok__) => {
                        enclosingelement.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetText<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxlength: i32, text: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextRange_Impl::GetText(this, core::mem::transmute_copy(&maxlength)) {
                    Ok(ok__) => {
                        text.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Move<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: super::TextUnit, count: i32, moved: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextRange_Impl::Move(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        moved.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveEndpointByUnit<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endpoint: super::TextPatternRangeEndpoint, unit: super::TextUnit, count: i32, moved: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextRange_Impl::MoveEndpointByUnit(this, core::mem::transmute_copy(&endpoint), core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        moved.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveEndpointByRange<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, srcendpoint: super::TextPatternRangeEndpoint, range: *mut core::ffi::c_void, targetendpoint: super::TextPatternRangeEndpoint) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationTextRange_Impl::MoveEndpointByRange(this, core::mem::transmute_copy(&srcendpoint), core::mem::transmute_copy(&range), core::mem::transmute_copy(&targetendpoint)).into()
            }
        }
        unsafe extern "system" fn Select<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationTextRange_Impl::Select(this).into()
            }
        }
        unsafe extern "system" fn AddToSelection<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationTextRange_Impl::AddToSelection(this).into()
            }
        }
        unsafe extern "system" fn RemoveFromSelection<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationTextRange_Impl::RemoveFromSelection(this).into()
            }
        }
        unsafe extern "system" fn ScrollIntoView<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, aligntotop: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationTextRange_Impl::ScrollIntoView(this, core::mem::transmute_copy(&aligntotop)).into()
            }
        }
        unsafe extern "system" fn GetChildren<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, children: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextRange_Impl::GetChildren(this) {
                    Ok(ok__) => {
                        children.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Clone: Clone::<Identity, OFFSET>,
            Compare: Compare::<Identity, OFFSET>,
            CompareEndpoints: CompareEndpoints::<Identity, OFFSET>,
            ExpandToEnclosingUnit: ExpandToEnclosingUnit::<Identity, OFFSET>,
            FindAttribute: FindAttribute::<Identity, OFFSET>,
            FindTextA: FindTextA::<Identity, OFFSET>,
            GetAttributeValue: GetAttributeValue::<Identity, OFFSET>,
            GetBoundingRectangles: GetBoundingRectangles::<Identity, OFFSET>,
            GetEnclosingElement: GetEnclosingElement::<Identity, OFFSET>,
            GetText: GetText::<Identity, OFFSET>,
            Move: Move::<Identity, OFFSET>,
            MoveEndpointByUnit: MoveEndpointByUnit::<Identity, OFFSET>,
            MoveEndpointByRange: MoveEndpointByRange::<Identity, OFFSET>,
            Select: Select::<Identity, OFFSET>,
            AddToSelection: AddToSelection::<Identity, OFFSET>,
            RemoveFromSelection: RemoveFromSelection::<Identity, OFFSET>,
            ScrollIntoView: ScrollIntoView::<Identity, OFFSET>,
            GetChildren: GetChildren::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationTextRange as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUIAutomationTextRange {}
windows_core::imp::define_interface!(IUIAutomationTextRange2, IUIAutomationTextRange2_Vtbl, 0xbb9b40e0_5e04_46bd_9be0_4b601b9afad4);
impl core::ops::Deref for IUIAutomationTextRange2 {
    type Target = IUIAutomationTextRange;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationTextRange2, windows_core::IUnknown, IUIAutomationTextRange);
impl IUIAutomationTextRange2 {
    pub unsafe fn ShowContextMenu(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShowContextMenu)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTextRange2_Vtbl {
    pub base__: IUIAutomationTextRange_Vtbl,
    pub ShowContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationTextRange2_Impl: IUIAutomationTextRange_Impl {
    fn ShowContextMenu(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationTextRange2_Vtbl {
    pub const fn new<Identity: IUIAutomationTextRange2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ShowContextMenu<Identity: IUIAutomationTextRange2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationTextRange2_Impl::ShowContextMenu(this).into()
            }
        }
        Self { base__: IUIAutomationTextRange_Vtbl::new::<Identity, OFFSET>(), ShowContextMenu: ShowContextMenu::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationTextRange2 as windows_core::Interface>::IID || iid == &<IUIAutomationTextRange as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUIAutomationTextRange2 {}
windows_core::imp::define_interface!(IUIAutomationTextRange3, IUIAutomationTextRange3_Vtbl, 0x6a315d69_5512_4c2e_85f0_53fce6dd4bc2);
impl core::ops::Deref for IUIAutomationTextRange3 {
    type Target = IUIAutomationTextRange2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationTextRange3, windows_core::IUnknown, IUIAutomationTextRange, IUIAutomationTextRange2);
impl IUIAutomationTextRange3 {
    pub unsafe fn GetEnclosingElementBuildCache<P0>(&self, cacherequest: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnclosingElementBuildCache)(windows_core::Interface::as_raw(self), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetChildrenBuildCache<P0>(&self, cacherequest: P0) -> windows_core::Result<IUIAutomationElementArray>
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChildrenBuildCache)(windows_core::Interface::as_raw(self), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt"))]
    pub unsafe fn GetAttributeValues(&self, attributeids: &[super::TEXTATTRIBUTEID]) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttributeValues)(windows_core::Interface::as_raw(self), attributeids.as_ptr(), attributeids.len().try_into().unwrap(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTextRange3_Vtbl {
    pub base__: IUIAutomationTextRange2_Vtbl,
    pub GetEnclosingElementBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetChildrenBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt"))]
    pub GetAttributeValues: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::TEXTATTRIBUTEID, i32, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt")))]
    GetAttributeValues: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationTextRange3_Impl: IUIAutomationTextRange2_Impl {
    fn GetEnclosingElementBuildCache(&self, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>) -> windows_core::Result<IUIAutomationElement>;
    fn GetChildrenBuildCache(&self, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>) -> windows_core::Result<IUIAutomationElementArray>;
    fn GetAttributeValues(&self, attributeids: *const super::TEXTATTRIBUTEID, attributeidcount: i32) -> windows_core::Result<*mut super::SAFEARRAY>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationTextRange3_Vtbl {
    pub const fn new<Identity: IUIAutomationTextRange3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEnclosingElementBuildCache<Identity: IUIAutomationTextRange3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cacherequest: *mut core::ffi::c_void, enclosingelement: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextRange3_Impl::GetEnclosingElementBuildCache(this, core::mem::transmute_copy(&cacherequest)) {
                    Ok(ok__) => {
                        enclosingelement.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetChildrenBuildCache<Identity: IUIAutomationTextRange3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cacherequest: *mut core::ffi::c_void, children: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextRange3_Impl::GetChildrenBuildCache(this, core::mem::transmute_copy(&cacherequest)) {
                    Ok(ok__) => {
                        children.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAttributeValues<Identity: IUIAutomationTextRange3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributeids: *const super::TEXTATTRIBUTEID, attributeidcount: i32, attributevalues: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextRange3_Impl::GetAttributeValues(this, core::mem::transmute_copy(&attributeids), core::mem::transmute_copy(&attributeidcount)) {
                    Ok(ok__) => {
                        attributevalues.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IUIAutomationTextRange2_Vtbl::new::<Identity, OFFSET>(),
            GetEnclosingElementBuildCache: GetEnclosingElementBuildCache::<Identity, OFFSET>,
            GetChildrenBuildCache: GetChildrenBuildCache::<Identity, OFFSET>,
            GetAttributeValues: GetAttributeValues::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationTextRange3 as windows_core::Interface>::IID || iid == &<IUIAutomationTextRange as windows_core::Interface>::IID || iid == &<IUIAutomationTextRange2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUIAutomationTextRange3 {}
windows_core::imp::define_interface!(IUIAutomationTextRangeArray, IUIAutomationTextRangeArray_Vtbl, 0xce4ae76a_e717_4c98_81ea_47371d028eb6);
windows_core::imp::interface_hierarchy!(IUIAutomationTextRangeArray, windows_core::IUnknown);
impl IUIAutomationTextRangeArray {
    pub unsafe fn get_Length(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Length)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetElement(&self, index: i32) -> windows_core::Result<IUIAutomationTextRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTextRangeArray_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAutomationTextRangeArray_Impl: windows_core::IUnknownImpl {
    fn get_Length(&self) -> windows_core::Result<i32>;
    fn GetElement(&self, index: i32) -> windows_core::Result<IUIAutomationTextRange>;
}
impl IUIAutomationTextRangeArray_Vtbl {
    pub const fn new<Identity: IUIAutomationTextRangeArray_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Length<Identity: IUIAutomationTextRangeArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextRangeArray_Impl::get_Length(this) {
                    Ok(ok__) => {
                        length.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetElement<Identity: IUIAutomationTextRangeArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: i32, element: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextRangeArray_Impl::GetElement(this, core::mem::transmute_copy(&index)) {
                    Ok(ok__) => {
                        element.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            get_Length: get_Length::<Identity, OFFSET>,
            GetElement: GetElement::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationTextRangeArray as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationTextRangeArray {}
windows_core::imp::define_interface!(IUIAutomationTogglePattern, IUIAutomationTogglePattern_Vtbl, 0x94cf8058_9b8d_4ab9_8bfd_4cd0a33c8c70);
windows_core::imp::interface_hierarchy!(IUIAutomationTogglePattern, windows_core::IUnknown);
impl IUIAutomationTogglePattern {
    pub unsafe fn Toggle(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Toggle)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn get_CurrentToggleState(&self) -> windows_core::Result<super::ToggleState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentToggleState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn get_CachedToggleState(&self) -> windows_core::Result<super::ToggleState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedToggleState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTogglePattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Toggle: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub get_CurrentToggleState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ToggleState) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    get_CurrentToggleState: usize,
    #[cfg(feature = "uiautomationcore")]
    pub get_CachedToggleState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::ToggleState) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    get_CachedToggleState: usize,
}
#[cfg(feature = "uiautomationcore")]
pub trait IUIAutomationTogglePattern_Impl: windows_core::IUnknownImpl {
    fn Toggle(&self) -> windows_core::Result<()>;
    fn get_CurrentToggleState(&self) -> windows_core::Result<super::ToggleState>;
    fn get_CachedToggleState(&self) -> windows_core::Result<super::ToggleState>;
}
#[cfg(feature = "uiautomationcore")]
impl IUIAutomationTogglePattern_Vtbl {
    pub const fn new<Identity: IUIAutomationTogglePattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Toggle<Identity: IUIAutomationTogglePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationTogglePattern_Impl::Toggle(this).into()
            }
        }
        unsafe extern "system" fn get_CurrentToggleState<Identity: IUIAutomationTogglePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::ToggleState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTogglePattern_Impl::get_CurrentToggleState(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedToggleState<Identity: IUIAutomationTogglePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::ToggleState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTogglePattern_Impl::get_CachedToggleState(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Toggle: Toggle::<Identity, OFFSET>,
            get_CurrentToggleState: get_CurrentToggleState::<Identity, OFFSET>,
            get_CachedToggleState: get_CachedToggleState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationTogglePattern as windows_core::Interface>::IID
    }
}
#[cfg(feature = "uiautomationcore")]
impl windows_core::RuntimeName for IUIAutomationTogglePattern {}
windows_core::imp::define_interface!(IUIAutomationTransformPattern, IUIAutomationTransformPattern_Vtbl, 0xa9b55844_a55d_4ef0_926d_569c16ff89bb);
windows_core::imp::interface_hierarchy!(IUIAutomationTransformPattern, windows_core::IUnknown);
impl IUIAutomationTransformPattern {
    pub unsafe fn Move(&self, x: f64, y: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Move)(windows_core::Interface::as_raw(self), x, y) }
    }
    pub unsafe fn Resize(&self, width: f64, height: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Resize)(windows_core::Interface::as_raw(self), width, height) }
    }
    pub unsafe fn Rotate(&self, degrees: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Rotate)(windows_core::Interface::as_raw(self), degrees) }
    }
    pub unsafe fn get_CurrentCanMove(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentCanMove)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentCanResize(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentCanResize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentCanRotate(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentCanRotate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedCanMove(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedCanMove)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedCanResize(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedCanResize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedCanRotate(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedCanRotate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTransformPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Move: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub Resize: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub Rotate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub get_CurrentCanMove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CurrentCanResize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CurrentCanRotate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CachedCanMove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CachedCanResize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CachedCanRotate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IUIAutomationTransformPattern_Impl: windows_core::IUnknownImpl {
    fn Move(&self, x: f64, y: f64) -> windows_core::Result<()>;
    fn Resize(&self, width: f64, height: f64) -> windows_core::Result<()>;
    fn Rotate(&self, degrees: f64) -> windows_core::Result<()>;
    fn get_CurrentCanMove(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CurrentCanResize(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CurrentCanRotate(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedCanMove(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedCanResize(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedCanRotate(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IUIAutomationTransformPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationTransformPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Move<Identity: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f64, y: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationTransformPattern_Impl::Move(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
            }
        }
        unsafe extern "system" fn Resize<Identity: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: f64, height: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationTransformPattern_Impl::Resize(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
            }
        }
        unsafe extern "system" fn Rotate<Identity: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, degrees: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationTransformPattern_Impl::Rotate(this, core::mem::transmute_copy(&degrees)).into()
            }
        }
        unsafe extern "system" fn get_CurrentCanMove<Identity: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern_Impl::get_CurrentCanMove(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentCanResize<Identity: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern_Impl::get_CurrentCanResize(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentCanRotate<Identity: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern_Impl::get_CurrentCanRotate(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedCanMove<Identity: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern_Impl::get_CachedCanMove(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedCanResize<Identity: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern_Impl::get_CachedCanResize(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedCanRotate<Identity: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern_Impl::get_CachedCanRotate(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Move: Move::<Identity, OFFSET>,
            Resize: Resize::<Identity, OFFSET>,
            Rotate: Rotate::<Identity, OFFSET>,
            get_CurrentCanMove: get_CurrentCanMove::<Identity, OFFSET>,
            get_CurrentCanResize: get_CurrentCanResize::<Identity, OFFSET>,
            get_CurrentCanRotate: get_CurrentCanRotate::<Identity, OFFSET>,
            get_CachedCanMove: get_CachedCanMove::<Identity, OFFSET>,
            get_CachedCanResize: get_CachedCanResize::<Identity, OFFSET>,
            get_CachedCanRotate: get_CachedCanRotate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationTransformPattern as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationTransformPattern {}
windows_core::imp::define_interface!(IUIAutomationTransformPattern2, IUIAutomationTransformPattern2_Vtbl, 0x6d74d017_6ecb_4381_b38b_3c17a48ff1c2);
impl core::ops::Deref for IUIAutomationTransformPattern2 {
    type Target = IUIAutomationTransformPattern;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IUIAutomationTransformPattern2, windows_core::IUnknown, IUIAutomationTransformPattern);
impl IUIAutomationTransformPattern2 {
    pub unsafe fn Zoom(&self, zoomvalue: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Zoom)(windows_core::Interface::as_raw(self), zoomvalue) }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn ZoomByUnit(&self, zoomunit: super::ZoomUnit) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ZoomByUnit)(windows_core::Interface::as_raw(self), zoomunit) }
    }
    pub unsafe fn get_CurrentCanZoom(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentCanZoom)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedCanZoom(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedCanZoom)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentZoomLevel(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentZoomLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedZoomLevel(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedZoomLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentZoomMinimum(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentZoomMinimum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedZoomMinimum(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedZoomMinimum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentZoomMaximum(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentZoomMaximum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedZoomMaximum(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedZoomMaximum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTransformPattern2_Vtbl {
    pub base__: IUIAutomationTransformPattern_Vtbl,
    pub Zoom: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub ZoomByUnit: unsafe extern "system" fn(*mut core::ffi::c_void, super::ZoomUnit) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    ZoomByUnit: usize,
    pub get_CurrentCanZoom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CachedCanZoom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CurrentZoomLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_CachedZoomLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_CurrentZoomMinimum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_CachedZoomMinimum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_CurrentZoomMaximum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_CachedZoomMaximum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
#[cfg(feature = "uiautomationcore")]
pub trait IUIAutomationTransformPattern2_Impl: IUIAutomationTransformPattern_Impl {
    fn Zoom(&self, zoomvalue: f64) -> windows_core::Result<()>;
    fn ZoomByUnit(&self, zoomunit: super::ZoomUnit) -> windows_core::Result<()>;
    fn get_CurrentCanZoom(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedCanZoom(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CurrentZoomLevel(&self) -> windows_core::Result<f64>;
    fn get_CachedZoomLevel(&self) -> windows_core::Result<f64>;
    fn get_CurrentZoomMinimum(&self) -> windows_core::Result<f64>;
    fn get_CachedZoomMinimum(&self) -> windows_core::Result<f64>;
    fn get_CurrentZoomMaximum(&self) -> windows_core::Result<f64>;
    fn get_CachedZoomMaximum(&self) -> windows_core::Result<f64>;
}
#[cfg(feature = "uiautomationcore")]
impl IUIAutomationTransformPattern2_Vtbl {
    pub const fn new<Identity: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Zoom<Identity: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, zoomvalue: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationTransformPattern2_Impl::Zoom(this, core::mem::transmute_copy(&zoomvalue)).into()
            }
        }
        unsafe extern "system" fn ZoomByUnit<Identity: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, zoomunit: super::ZoomUnit) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationTransformPattern2_Impl::ZoomByUnit(this, core::mem::transmute_copy(&zoomunit)).into()
            }
        }
        unsafe extern "system" fn get_CurrentCanZoom<Identity: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern2_Impl::get_CurrentCanZoom(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedCanZoom<Identity: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern2_Impl::get_CachedCanZoom(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentZoomLevel<Identity: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern2_Impl::get_CurrentZoomLevel(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedZoomLevel<Identity: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern2_Impl::get_CachedZoomLevel(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentZoomMinimum<Identity: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern2_Impl::get_CurrentZoomMinimum(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedZoomMinimum<Identity: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern2_Impl::get_CachedZoomMinimum(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentZoomMaximum<Identity: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern2_Impl::get_CurrentZoomMaximum(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedZoomMaximum<Identity: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern2_Impl::get_CachedZoomMaximum(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: IUIAutomationTransformPattern_Vtbl::new::<Identity, OFFSET>(),
            Zoom: Zoom::<Identity, OFFSET>,
            ZoomByUnit: ZoomByUnit::<Identity, OFFSET>,
            get_CurrentCanZoom: get_CurrentCanZoom::<Identity, OFFSET>,
            get_CachedCanZoom: get_CachedCanZoom::<Identity, OFFSET>,
            get_CurrentZoomLevel: get_CurrentZoomLevel::<Identity, OFFSET>,
            get_CachedZoomLevel: get_CachedZoomLevel::<Identity, OFFSET>,
            get_CurrentZoomMinimum: get_CurrentZoomMinimum::<Identity, OFFSET>,
            get_CachedZoomMinimum: get_CachedZoomMinimum::<Identity, OFFSET>,
            get_CurrentZoomMaximum: get_CurrentZoomMaximum::<Identity, OFFSET>,
            get_CachedZoomMaximum: get_CachedZoomMaximum::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationTransformPattern2 as windows_core::Interface>::IID || iid == &<IUIAutomationTransformPattern as windows_core::Interface>::IID
    }
}
#[cfg(feature = "uiautomationcore")]
impl windows_core::RuntimeName for IUIAutomationTransformPattern2 {}
windows_core::imp::define_interface!(IUIAutomationTreeWalker, IUIAutomationTreeWalker_Vtbl, 0x4042c624_389c_4afc_a630_9df854a541fc);
windows_core::imp::interface_hierarchy!(IUIAutomationTreeWalker, windows_core::IUnknown);
impl IUIAutomationTreeWalker {
    pub unsafe fn GetParentElement<P0>(&self, element: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParentElement)(windows_core::Interface::as_raw(self), element.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFirstChildElement<P0>(&self, element: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFirstChildElement)(windows_core::Interface::as_raw(self), element.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetLastChildElement<P0>(&self, element: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastChildElement)(windows_core::Interface::as_raw(self), element.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetNextSiblingElement<P0>(&self, element: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNextSiblingElement)(windows_core::Interface::as_raw(self), element.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPreviousSiblingElement<P0>(&self, element: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPreviousSiblingElement)(windows_core::Interface::as_raw(self), element.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn NormalizeElement<P0>(&self, element: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NormalizeElement)(windows_core::Interface::as_raw(self), element.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetParentElementBuildCache<P0, P1>(&self, element: P0, cacherequest: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParentElementBuildCache)(windows_core::Interface::as_raw(self), element.param().abi(), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFirstChildElementBuildCache<P0, P1>(&self, element: P0, cacherequest: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFirstChildElementBuildCache)(windows_core::Interface::as_raw(self), element.param().abi(), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetLastChildElementBuildCache<P0, P1>(&self, element: P0, cacherequest: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastChildElementBuildCache)(windows_core::Interface::as_raw(self), element.param().abi(), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetNextSiblingElementBuildCache<P0, P1>(&self, element: P0, cacherequest: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNextSiblingElementBuildCache)(windows_core::Interface::as_raw(self), element.param().abi(), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPreviousSiblingElementBuildCache<P0, P1>(&self, element: P0, cacherequest: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPreviousSiblingElementBuildCache)(windows_core::Interface::as_raw(self), element.param().abi(), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn NormalizeElementBuildCache<P0, P1>(&self, element: P0, cacherequest: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NormalizeElementBuildCache)(windows_core::Interface::as_raw(self), element.param().abi(), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_Condition(&self) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Condition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTreeWalker_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetParentElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFirstChildElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLastChildElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNextSiblingElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPreviousSiblingElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NormalizeElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetParentElementBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFirstChildElementBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetLastChildElementBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetNextSiblingElementBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetPreviousSiblingElementBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub NormalizeElementBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Condition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAutomationTreeWalker_Impl: windows_core::IUnknownImpl {
    fn GetParentElement(&self, element: windows_core::Ref<IUIAutomationElement>) -> windows_core::Result<IUIAutomationElement>;
    fn GetFirstChildElement(&self, element: windows_core::Ref<IUIAutomationElement>) -> windows_core::Result<IUIAutomationElement>;
    fn GetLastChildElement(&self, element: windows_core::Ref<IUIAutomationElement>) -> windows_core::Result<IUIAutomationElement>;
    fn GetNextSiblingElement(&self, element: windows_core::Ref<IUIAutomationElement>) -> windows_core::Result<IUIAutomationElement>;
    fn GetPreviousSiblingElement(&self, element: windows_core::Ref<IUIAutomationElement>) -> windows_core::Result<IUIAutomationElement>;
    fn NormalizeElement(&self, element: windows_core::Ref<IUIAutomationElement>) -> windows_core::Result<IUIAutomationElement>;
    fn GetParentElementBuildCache(&self, element: windows_core::Ref<IUIAutomationElement>, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>) -> windows_core::Result<IUIAutomationElement>;
    fn GetFirstChildElementBuildCache(&self, element: windows_core::Ref<IUIAutomationElement>, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>) -> windows_core::Result<IUIAutomationElement>;
    fn GetLastChildElementBuildCache(&self, element: windows_core::Ref<IUIAutomationElement>, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>) -> windows_core::Result<IUIAutomationElement>;
    fn GetNextSiblingElementBuildCache(&self, element: windows_core::Ref<IUIAutomationElement>, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>) -> windows_core::Result<IUIAutomationElement>;
    fn GetPreviousSiblingElementBuildCache(&self, element: windows_core::Ref<IUIAutomationElement>, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>) -> windows_core::Result<IUIAutomationElement>;
    fn NormalizeElementBuildCache(&self, element: windows_core::Ref<IUIAutomationElement>, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>) -> windows_core::Result<IUIAutomationElement>;
    fn get_Condition(&self) -> windows_core::Result<IUIAutomationCondition>;
}
impl IUIAutomationTreeWalker_Vtbl {
    pub const fn new<Identity: IUIAutomationTreeWalker_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetParentElement<Identity: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, parent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::GetParentElement(this, core::mem::transmute_copy(&element)) {
                    Ok(ok__) => {
                        parent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFirstChildElement<Identity: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, first: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::GetFirstChildElement(this, core::mem::transmute_copy(&element)) {
                    Ok(ok__) => {
                        first.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLastChildElement<Identity: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, last: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::GetLastChildElement(this, core::mem::transmute_copy(&element)) {
                    Ok(ok__) => {
                        last.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNextSiblingElement<Identity: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, next: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::GetNextSiblingElement(this, core::mem::transmute_copy(&element)) {
                    Ok(ok__) => {
                        next.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPreviousSiblingElement<Identity: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, previous: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::GetPreviousSiblingElement(this, core::mem::transmute_copy(&element)) {
                    Ok(ok__) => {
                        previous.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NormalizeElement<Identity: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, normalized: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::NormalizeElement(this, core::mem::transmute_copy(&element)) {
                    Ok(ok__) => {
                        normalized.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetParentElementBuildCache<Identity: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, cacherequest: *mut core::ffi::c_void, parent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::GetParentElementBuildCache(this, core::mem::transmute_copy(&element), core::mem::transmute_copy(&cacherequest)) {
                    Ok(ok__) => {
                        parent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFirstChildElementBuildCache<Identity: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, cacherequest: *mut core::ffi::c_void, first: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::GetFirstChildElementBuildCache(this, core::mem::transmute_copy(&element), core::mem::transmute_copy(&cacherequest)) {
                    Ok(ok__) => {
                        first.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetLastChildElementBuildCache<Identity: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, cacherequest: *mut core::ffi::c_void, last: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::GetLastChildElementBuildCache(this, core::mem::transmute_copy(&element), core::mem::transmute_copy(&cacherequest)) {
                    Ok(ok__) => {
                        last.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetNextSiblingElementBuildCache<Identity: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, cacherequest: *mut core::ffi::c_void, next: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::GetNextSiblingElementBuildCache(this, core::mem::transmute_copy(&element), core::mem::transmute_copy(&cacherequest)) {
                    Ok(ok__) => {
                        next.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPreviousSiblingElementBuildCache<Identity: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, cacherequest: *mut core::ffi::c_void, previous: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::GetPreviousSiblingElementBuildCache(this, core::mem::transmute_copy(&element), core::mem::transmute_copy(&cacherequest)) {
                    Ok(ok__) => {
                        previous.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NormalizeElementBuildCache<Identity: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, cacherequest: *mut core::ffi::c_void, normalized: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::NormalizeElementBuildCache(this, core::mem::transmute_copy(&element), core::mem::transmute_copy(&cacherequest)) {
                    Ok(ok__) => {
                        normalized.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Condition<Identity: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, condition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::get_Condition(this) {
                    Ok(ok__) => {
                        condition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetParentElement: GetParentElement::<Identity, OFFSET>,
            GetFirstChildElement: GetFirstChildElement::<Identity, OFFSET>,
            GetLastChildElement: GetLastChildElement::<Identity, OFFSET>,
            GetNextSiblingElement: GetNextSiblingElement::<Identity, OFFSET>,
            GetPreviousSiblingElement: GetPreviousSiblingElement::<Identity, OFFSET>,
            NormalizeElement: NormalizeElement::<Identity, OFFSET>,
            GetParentElementBuildCache: GetParentElementBuildCache::<Identity, OFFSET>,
            GetFirstChildElementBuildCache: GetFirstChildElementBuildCache::<Identity, OFFSET>,
            GetLastChildElementBuildCache: GetLastChildElementBuildCache::<Identity, OFFSET>,
            GetNextSiblingElementBuildCache: GetNextSiblingElementBuildCache::<Identity, OFFSET>,
            GetPreviousSiblingElementBuildCache: GetPreviousSiblingElementBuildCache::<Identity, OFFSET>,
            NormalizeElementBuildCache: NormalizeElementBuildCache::<Identity, OFFSET>,
            get_Condition: get_Condition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationTreeWalker as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationTreeWalker {}
windows_core::imp::define_interface!(IUIAutomationValuePattern, IUIAutomationValuePattern_Vtbl, 0xa94cd8b1_0844_4cd6_9d2d_640537ab39e9);
windows_core::imp::interface_hierarchy!(IUIAutomationValuePattern, windows_core::IUnknown);
impl IUIAutomationValuePattern {
    pub unsafe fn SetValue(&self, val: &windows_core::BSTR) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(val)) }
    }
    pub unsafe fn get_CurrentValue(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CurrentIsReadOnly(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentIsReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedValue(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_CachedIsReadOnly(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedIsReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationValuePattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentIsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CachedValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CachedIsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IUIAutomationValuePattern_Impl: windows_core::IUnknownImpl {
    fn SetValue(&self, val: &windows_core::BSTR) -> windows_core::Result<()>;
    fn get_CurrentValue(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CurrentIsReadOnly(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedValue(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_CachedIsReadOnly(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IUIAutomationValuePattern_Vtbl {
    pub const fn new<Identity: IUIAutomationValuePattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetValue<Identity: IUIAutomationValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationValuePattern_Impl::SetValue(this, core::mem::transmute(&val)).into()
            }
        }
        unsafe extern "system" fn get_CurrentValue<Identity: IUIAutomationValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationValuePattern_Impl::get_CurrentValue(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentIsReadOnly<Identity: IUIAutomationValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationValuePattern_Impl::get_CurrentIsReadOnly(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedValue<Identity: IUIAutomationValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationValuePattern_Impl::get_CachedValue(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedIsReadOnly<Identity: IUIAutomationValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationValuePattern_Impl::get_CachedIsReadOnly(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetValue: SetValue::<Identity, OFFSET>,
            get_CurrentValue: get_CurrentValue::<Identity, OFFSET>,
            get_CurrentIsReadOnly: get_CurrentIsReadOnly::<Identity, OFFSET>,
            get_CachedValue: get_CachedValue::<Identity, OFFSET>,
            get_CachedIsReadOnly: get_CachedIsReadOnly::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationValuePattern as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationValuePattern {}
windows_core::imp::define_interface!(IUIAutomationVirtualizedItemPattern, IUIAutomationVirtualizedItemPattern_Vtbl, 0x6ba3d7a6_04cf_4f11_8793_a8d1cde9969f);
windows_core::imp::interface_hierarchy!(IUIAutomationVirtualizedItemPattern, windows_core::IUnknown);
impl IUIAutomationVirtualizedItemPattern {
    pub unsafe fn Realize(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Realize)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationVirtualizedItemPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Realize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAutomationVirtualizedItemPattern_Impl: windows_core::IUnknownImpl {
    fn Realize(&self) -> windows_core::Result<()>;
}
impl IUIAutomationVirtualizedItemPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationVirtualizedItemPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Realize<Identity: IUIAutomationVirtualizedItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationVirtualizedItemPattern_Impl::Realize(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Realize: Realize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationVirtualizedItemPattern as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationVirtualizedItemPattern {}
windows_core::imp::define_interface!(IUIAutomationWindowPattern, IUIAutomationWindowPattern_Vtbl, 0x0faef453_9208_43ef_bbb2_3b485177864f);
windows_core::imp::interface_hierarchy!(IUIAutomationWindowPattern, windows_core::IUnknown);
impl IUIAutomationWindowPattern {
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn WaitForInputIdle(&self, milliseconds: i32) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WaitForInputIdle)(windows_core::Interface::as_raw(self), milliseconds, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn SetWindowVisualState(&self, state: super::WindowVisualState) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWindowVisualState)(windows_core::Interface::as_raw(self), state) }
    }
    pub unsafe fn get_CurrentCanMaximize(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentCanMaximize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentCanMinimize(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentCanMinimize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentIsModal(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentIsModal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CurrentIsTopmost(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentIsTopmost)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn get_CurrentWindowVisualState(&self) -> windows_core::Result<super::WindowVisualState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentWindowVisualState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn get_CurrentWindowInteractionState(&self) -> windows_core::Result<super::WindowInteractionState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentWindowInteractionState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedCanMaximize(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedCanMaximize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedCanMinimize(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedCanMinimize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedIsModal(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedIsModal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CachedIsTopmost(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedIsTopmost)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn get_CachedWindowVisualState(&self) -> windows_core::Result<super::WindowVisualState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedWindowVisualState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn get_CachedWindowInteractionState(&self) -> windows_core::Result<super::WindowInteractionState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CachedWindowInteractionState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationWindowPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WaitForInputIdle: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub SetWindowVisualState: unsafe extern "system" fn(*mut core::ffi::c_void, super::WindowVisualState) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    SetWindowVisualState: usize,
    pub get_CurrentCanMaximize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CurrentCanMinimize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CurrentIsModal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CurrentIsTopmost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub get_CurrentWindowVisualState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WindowVisualState) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    get_CurrentWindowVisualState: usize,
    #[cfg(feature = "uiautomationcore")]
    pub get_CurrentWindowInteractionState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WindowInteractionState) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    get_CurrentWindowInteractionState: usize,
    pub get_CachedCanMaximize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CachedCanMinimize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CachedIsModal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CachedIsTopmost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub get_CachedWindowVisualState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WindowVisualState) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    get_CachedWindowVisualState: usize,
    #[cfg(feature = "uiautomationcore")]
    pub get_CachedWindowInteractionState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::WindowInteractionState) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    get_CachedWindowInteractionState: usize,
}
#[cfg(feature = "uiautomationcore")]
pub trait IUIAutomationWindowPattern_Impl: windows_core::IUnknownImpl {
    fn Close(&self) -> windows_core::Result<()>;
    fn WaitForInputIdle(&self, milliseconds: i32) -> windows_core::Result<windows_core::BOOL>;
    fn SetWindowVisualState(&self, state: super::WindowVisualState) -> windows_core::Result<()>;
    fn get_CurrentCanMaximize(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CurrentCanMinimize(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CurrentIsModal(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CurrentIsTopmost(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CurrentWindowVisualState(&self) -> windows_core::Result<super::WindowVisualState>;
    fn get_CurrentWindowInteractionState(&self) -> windows_core::Result<super::WindowInteractionState>;
    fn get_CachedCanMaximize(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedCanMinimize(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedIsModal(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedIsTopmost(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CachedWindowVisualState(&self) -> windows_core::Result<super::WindowVisualState>;
    fn get_CachedWindowInteractionState(&self) -> windows_core::Result<super::WindowInteractionState>;
}
#[cfg(feature = "uiautomationcore")]
impl IUIAutomationWindowPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Close<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationWindowPattern_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn WaitForInputIdle<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, milliseconds: i32, success: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationWindowPattern_Impl::WaitForInputIdle(this, core::mem::transmute_copy(&milliseconds)) {
                    Ok(ok__) => {
                        success.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetWindowVisualState<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: super::WindowVisualState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationWindowPattern_Impl::SetWindowVisualState(this, core::mem::transmute_copy(&state)).into()
            }
        }
        unsafe extern "system" fn get_CurrentCanMaximize<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationWindowPattern_Impl::get_CurrentCanMaximize(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentCanMinimize<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationWindowPattern_Impl::get_CurrentCanMinimize(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentIsModal<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationWindowPattern_Impl::get_CurrentIsModal(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentIsTopmost<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationWindowPattern_Impl::get_CurrentIsTopmost(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentWindowVisualState<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::WindowVisualState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationWindowPattern_Impl::get_CurrentWindowVisualState(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentWindowInteractionState<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::WindowInteractionState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationWindowPattern_Impl::get_CurrentWindowInteractionState(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedCanMaximize<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationWindowPattern_Impl::get_CachedCanMaximize(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedCanMinimize<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationWindowPattern_Impl::get_CachedCanMinimize(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedIsModal<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationWindowPattern_Impl::get_CachedIsModal(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedIsTopmost<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationWindowPattern_Impl::get_CachedIsTopmost(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedWindowVisualState<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::WindowVisualState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationWindowPattern_Impl::get_CachedWindowVisualState(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CachedWindowInteractionState<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::WindowInteractionState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationWindowPattern_Impl::get_CachedWindowInteractionState(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Close: Close::<Identity, OFFSET>,
            WaitForInputIdle: WaitForInputIdle::<Identity, OFFSET>,
            SetWindowVisualState: SetWindowVisualState::<Identity, OFFSET>,
            get_CurrentCanMaximize: get_CurrentCanMaximize::<Identity, OFFSET>,
            get_CurrentCanMinimize: get_CurrentCanMinimize::<Identity, OFFSET>,
            get_CurrentIsModal: get_CurrentIsModal::<Identity, OFFSET>,
            get_CurrentIsTopmost: get_CurrentIsTopmost::<Identity, OFFSET>,
            get_CurrentWindowVisualState: get_CurrentWindowVisualState::<Identity, OFFSET>,
            get_CurrentWindowInteractionState: get_CurrentWindowInteractionState::<Identity, OFFSET>,
            get_CachedCanMaximize: get_CachedCanMaximize::<Identity, OFFSET>,
            get_CachedCanMinimize: get_CachedCanMinimize::<Identity, OFFSET>,
            get_CachedIsModal: get_CachedIsModal::<Identity, OFFSET>,
            get_CachedIsTopmost: get_CachedIsTopmost::<Identity, OFFSET>,
            get_CachedWindowVisualState: get_CachedWindowVisualState::<Identity, OFFSET>,
            get_CachedWindowInteractionState: get_CachedWindowInteractionState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationWindowPattern as windows_core::Interface>::IID
    }
}
#[cfg(feature = "uiautomationcore")]
impl windows_core::RuntimeName for IUIAutomationWindowPattern {}
pub type PropertyConditionFlags = i32;
pub const PropertyConditionFlags_IgnoreCase: PropertyConditionFlags = 1;
pub const PropertyConditionFlags_MatchSubstring: PropertyConditionFlags = 2;
pub const PropertyConditionFlags_None: PropertyConditionFlags = 0;
pub const StyleId_BulletedList: i32 = 70015;
pub const StyleId_Custom: i32 = 70000;
pub const StyleId_Emphasis: i32 = 70013;
pub const StyleId_Heading1: i32 = 70001;
pub const StyleId_Heading2: i32 = 70002;
pub const StyleId_Heading3: i32 = 70003;
pub const StyleId_Heading4: i32 = 70004;
pub const StyleId_Heading5: i32 = 70005;
pub const StyleId_Heading6: i32 = 70006;
pub const StyleId_Heading7: i32 = 70007;
pub const StyleId_Heading8: i32 = 70008;
pub const StyleId_Heading9: i32 = 70009;
pub const StyleId_Normal: i32 = 70012;
pub const StyleId_NumberedList: i32 = 70016;
pub const StyleId_Quote: i32 = 70014;
pub const StyleId_Subtitle: i32 = 70011;
pub const StyleId_Title: i32 = 70010;
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
pub const UIA_AcceleratorKeyPropertyId: i32 = 30006;
pub const UIA_AccessKeyPropertyId: i32 = 30007;
pub const UIA_ActiveTextPositionChangedEventId: i32 = 20036;
pub const UIA_AfterParagraphSpacingAttributeId: i32 = 40042;
pub const UIA_AnimationStyleAttributeId: i32 = 40000;
pub const UIA_AnnotationAnnotationTypeIdPropertyId: i32 = 30113;
pub const UIA_AnnotationAnnotationTypeNamePropertyId: i32 = 30114;
pub const UIA_AnnotationAuthorPropertyId: i32 = 30115;
pub const UIA_AnnotationDateTimePropertyId: i32 = 30116;
pub const UIA_AnnotationObjectsAttributeId: i32 = 40032;
pub const UIA_AnnotationObjectsPropertyId: i32 = 30156;
pub const UIA_AnnotationPatternId: i32 = 10023;
pub const UIA_AnnotationTargetPropertyId: i32 = 30117;
pub const UIA_AnnotationTypesAttributeId: i32 = 40031;
pub const UIA_AnnotationTypesPropertyId: i32 = 30155;
pub const UIA_AppBarControlTypeId: i32 = 50040;
pub const UIA_AriaPropertiesPropertyId: i32 = 30102;
pub const UIA_AriaRolePropertyId: i32 = 30101;
pub const UIA_AsyncContentLoadedEventId: i32 = 20006;
pub const UIA_AutomationFocusChangedEventId: i32 = 20005;
pub const UIA_AutomationIdPropertyId: i32 = 30011;
pub const UIA_AutomationPropertyChangedEventId: i32 = 20004;
pub const UIA_BackgroundColorAttributeId: i32 = 40001;
pub const UIA_BeforeParagraphSpacingAttributeId: i32 = 40041;
pub const UIA_BoundingRectanglePropertyId: i32 = 30001;
pub const UIA_BulletStyleAttributeId: i32 = 40002;
pub const UIA_ButtonControlTypeId: i32 = 50000;
pub const UIA_CalendarControlTypeId: i32 = 50001;
pub const UIA_CapStyleAttributeId: i32 = 40003;
pub const UIA_CaretBidiModeAttributeId: i32 = 40039;
pub const UIA_CaretPositionAttributeId: i32 = 40038;
pub const UIA_CenterPointPropertyId: i32 = 30165;
pub const UIA_ChangesEventId: i32 = 20034;
pub const UIA_CheckBoxControlTypeId: i32 = 50002;
pub const UIA_ClassNamePropertyId: i32 = 30012;
pub const UIA_ClickablePointPropertyId: i32 = 30014;
pub const UIA_ComboBoxControlTypeId: i32 = 50003;
pub const UIA_ControlTypePropertyId: i32 = 30003;
pub const UIA_ControllerForPropertyId: i32 = 30104;
pub const UIA_CultureAttributeId: i32 = 40004;
pub const UIA_CulturePropertyId: i32 = 30015;
pub const UIA_CustomControlTypeId: i32 = 50025;
pub const UIA_CustomLandmarkTypeId: i32 = 80000;
pub const UIA_CustomNavigationPatternId: i32 = 10033;
pub const UIA_DataGridControlTypeId: i32 = 50028;
pub const UIA_DataItemControlTypeId: i32 = 50029;
pub const UIA_DescribedByPropertyId: i32 = 30105;
pub const UIA_DockDockPositionPropertyId: i32 = 30069;
pub const UIA_DockPatternId: i32 = 10011;
pub const UIA_DocumentControlTypeId: i32 = 50030;
pub const UIA_DragDropEffectPropertyId: i32 = 30139;
pub const UIA_DragDropEffectsPropertyId: i32 = 30140;
pub const UIA_DragGrabbedItemsPropertyId: i32 = 30144;
pub const UIA_DragIsGrabbedPropertyId: i32 = 30138;
pub const UIA_DragPatternId: i32 = 10030;
pub const UIA_Drag_DragCancelEventId: i32 = 20027;
pub const UIA_Drag_DragCompleteEventId: i32 = 20028;
pub const UIA_Drag_DragStartEventId: i32 = 20026;
pub const UIA_DropTargetDropTargetEffectPropertyId: i32 = 30142;
pub const UIA_DropTargetDropTargetEffectsPropertyId: i32 = 30143;
pub const UIA_DropTargetPatternId: i32 = 10031;
pub const UIA_DropTarget_DragEnterEventId: i32 = 20029;
pub const UIA_DropTarget_DragLeaveEventId: i32 = 20030;
pub const UIA_DropTarget_DroppedEventId: i32 = 20031;
pub const UIA_EditControlTypeId: i32 = 50004;
pub const UIA_ExpandCollapseExpandCollapseStatePropertyId: i32 = 30070;
pub const UIA_ExpandCollapsePatternId: i32 = 10005;
pub const UIA_FillColorPropertyId: i32 = 30160;
pub const UIA_FillTypePropertyId: i32 = 30162;
pub const UIA_FlowsFromPropertyId: i32 = 30148;
pub const UIA_FlowsToPropertyId: i32 = 30106;
pub const UIA_FontNameAttributeId: i32 = 40005;
pub const UIA_FontSizeAttributeId: i32 = 40006;
pub const UIA_FontWeightAttributeId: i32 = 40007;
pub const UIA_ForegroundColorAttributeId: i32 = 40008;
pub const UIA_FormLandmarkTypeId: i32 = 80001;
pub const UIA_FrameworkIdPropertyId: i32 = 30024;
pub const UIA_FullDescriptionPropertyId: i32 = 30159;
pub const UIA_GridColumnCountPropertyId: i32 = 30063;
pub const UIA_GridItemColumnPropertyId: i32 = 30065;
pub const UIA_GridItemColumnSpanPropertyId: i32 = 30067;
pub const UIA_GridItemContainingGridPropertyId: i32 = 30068;
pub const UIA_GridItemPatternId: i32 = 10007;
pub const UIA_GridItemRowPropertyId: i32 = 30064;
pub const UIA_GridItemRowSpanPropertyId: i32 = 30066;
pub const UIA_GridPatternId: i32 = 10006;
pub const UIA_GridRowCountPropertyId: i32 = 30062;
pub const UIA_GroupControlTypeId: i32 = 50026;
pub type UIA_HWND = *mut core::ffi::c_void;
pub const UIA_HasKeyboardFocusPropertyId: i32 = 30008;
pub const UIA_HeaderControlTypeId: i32 = 50034;
pub const UIA_HeaderItemControlTypeId: i32 = 50035;
pub const UIA_HeadingLevelPropertyId: i32 = 30173;
pub const UIA_HelpTextPropertyId: i32 = 30013;
pub const UIA_HorizontalTextAlignmentAttributeId: i32 = 40009;
pub const UIA_HostedFragmentRootsInvalidatedEventId: i32 = 20025;
pub const UIA_HyperlinkControlTypeId: i32 = 50005;
pub const UIA_ImageControlTypeId: i32 = 50006;
pub const UIA_IndentationFirstLineAttributeId: i32 = 40010;
pub const UIA_IndentationLeadingAttributeId: i32 = 40011;
pub const UIA_IndentationTrailingAttributeId: i32 = 40012;
pub const UIA_InputDiscardedEventId: i32 = 20022;
pub const UIA_InputReachedOtherElementEventId: i32 = 20021;
pub const UIA_InputReachedTargetEventId: i32 = 20020;
pub const UIA_InvokePatternId: i32 = 10000;
pub const UIA_Invoke_InvokedEventId: i32 = 20009;
pub const UIA_IsActiveAttributeId: i32 = 40036;
pub const UIA_IsAnnotationPatternAvailablePropertyId: i32 = 30118;
pub const UIA_IsContentElementPropertyId: i32 = 30017;
pub const UIA_IsControlElementPropertyId: i32 = 30016;
pub const UIA_IsCustomNavigationPatternAvailablePropertyId: i32 = 30151;
pub const UIA_IsDataValidForFormPropertyId: i32 = 30103;
pub const UIA_IsDialogPropertyId: i32 = 30174;
pub const UIA_IsDockPatternAvailablePropertyId: i32 = 30027;
pub const UIA_IsDragPatternAvailablePropertyId: i32 = 30137;
pub const UIA_IsDropTargetPatternAvailablePropertyId: i32 = 30141;
pub const UIA_IsEnabledPropertyId: i32 = 30010;
pub const UIA_IsExpandCollapsePatternAvailablePropertyId: i32 = 30028;
pub const UIA_IsGridItemPatternAvailablePropertyId: i32 = 30029;
pub const UIA_IsGridPatternAvailablePropertyId: i32 = 30030;
pub const UIA_IsHiddenAttributeId: i32 = 40013;
pub const UIA_IsInvokePatternAvailablePropertyId: i32 = 30031;
pub const UIA_IsItalicAttributeId: i32 = 40014;
pub const UIA_IsItemContainerPatternAvailablePropertyId: i32 = 30108;
pub const UIA_IsKeyboardFocusablePropertyId: i32 = 30009;
pub const UIA_IsLegacyIAccessiblePatternAvailablePropertyId: i32 = 30090;
pub const UIA_IsMultipleViewPatternAvailablePropertyId: i32 = 30032;
pub const UIA_IsObjectModelPatternAvailablePropertyId: i32 = 30112;
pub const UIA_IsOffscreenPropertyId: i32 = 30022;
pub const UIA_IsPasswordPropertyId: i32 = 30019;
pub const UIA_IsPeripheralPropertyId: i32 = 30150;
pub const UIA_IsRangeValuePatternAvailablePropertyId: i32 = 30033;
pub const UIA_IsReadOnlyAttributeId: i32 = 40015;
pub const UIA_IsRequiredForFormPropertyId: i32 = 30025;
pub const UIA_IsScrollItemPatternAvailablePropertyId: i32 = 30035;
pub const UIA_IsScrollPatternAvailablePropertyId: i32 = 30034;
pub const UIA_IsSelectionItemPatternAvailablePropertyId: i32 = 30036;
pub const UIA_IsSelectionPattern2AvailablePropertyId: i32 = 30168;
pub const UIA_IsSelectionPatternAvailablePropertyId: i32 = 30037;
pub const UIA_IsSpreadsheetItemPatternAvailablePropertyId: i32 = 30132;
pub const UIA_IsSpreadsheetPatternAvailablePropertyId: i32 = 30128;
pub const UIA_IsStylesPatternAvailablePropertyId: i32 = 30127;
pub const UIA_IsSubscriptAttributeId: i32 = 40016;
pub const UIA_IsSuperscriptAttributeId: i32 = 40017;
pub const UIA_IsSynchronizedInputPatternAvailablePropertyId: i32 = 30110;
pub const UIA_IsTableItemPatternAvailablePropertyId: i32 = 30039;
pub const UIA_IsTablePatternAvailablePropertyId: i32 = 30038;
pub const UIA_IsTextChildPatternAvailablePropertyId: i32 = 30136;
pub const UIA_IsTextEditPatternAvailablePropertyId: i32 = 30149;
pub const UIA_IsTextPattern2AvailablePropertyId: i32 = 30119;
pub const UIA_IsTextPatternAvailablePropertyId: i32 = 30040;
pub const UIA_IsTogglePatternAvailablePropertyId: i32 = 30041;
pub const UIA_IsTransformPattern2AvailablePropertyId: i32 = 30134;
pub const UIA_IsTransformPatternAvailablePropertyId: i32 = 30042;
pub const UIA_IsValuePatternAvailablePropertyId: i32 = 30043;
pub const UIA_IsVirtualizedItemPatternAvailablePropertyId: i32 = 30109;
pub const UIA_IsWindowPatternAvailablePropertyId: i32 = 30044;
pub const UIA_ItemContainerPatternId: i32 = 10019;
pub const UIA_ItemStatusPropertyId: i32 = 30026;
pub const UIA_ItemTypePropertyId: i32 = 30021;
pub const UIA_LabeledByPropertyId: i32 = 30018;
pub const UIA_LandmarkTypePropertyId: i32 = 30157;
pub const UIA_LayoutInvalidatedEventId: i32 = 20008;
pub const UIA_LegacyIAccessibleChildIdPropertyId: i32 = 30091;
pub const UIA_LegacyIAccessibleDefaultActionPropertyId: i32 = 30100;
pub const UIA_LegacyIAccessibleDescriptionPropertyId: i32 = 30094;
pub const UIA_LegacyIAccessibleHelpPropertyId: i32 = 30097;
pub const UIA_LegacyIAccessibleKeyboardShortcutPropertyId: i32 = 30098;
pub const UIA_LegacyIAccessibleNamePropertyId: i32 = 30092;
pub const UIA_LegacyIAccessiblePatternId: i32 = 10018;
pub const UIA_LegacyIAccessibleRolePropertyId: i32 = 30095;
pub const UIA_LegacyIAccessibleSelectionPropertyId: i32 = 30099;
pub const UIA_LegacyIAccessibleStatePropertyId: i32 = 30096;
pub const UIA_LegacyIAccessibleValuePropertyId: i32 = 30093;
pub const UIA_LevelPropertyId: i32 = 30154;
pub const UIA_LineSpacingAttributeId: i32 = 40040;
pub const UIA_LinkAttributeId: i32 = 40035;
pub const UIA_ListControlTypeId: i32 = 50008;
pub const UIA_ListItemControlTypeId: i32 = 50007;
pub const UIA_LiveRegionChangedEventId: i32 = 20024;
pub const UIA_LiveSettingPropertyId: i32 = 30135;
pub const UIA_LocalizedControlTypePropertyId: i32 = 30004;
pub const UIA_LocalizedLandmarkTypePropertyId: i32 = 30158;
pub const UIA_MainLandmarkTypeId: i32 = 80002;
pub const UIA_MarginBottomAttributeId: i32 = 40018;
pub const UIA_MarginLeadingAttributeId: i32 = 40019;
pub const UIA_MarginTopAttributeId: i32 = 40020;
pub const UIA_MarginTrailingAttributeId: i32 = 40021;
pub const UIA_MenuBarControlTypeId: i32 = 50010;
pub const UIA_MenuClosedEventId: i32 = 20007;
pub const UIA_MenuControlTypeId: i32 = 50009;
pub const UIA_MenuItemControlTypeId: i32 = 50011;
pub const UIA_MenuModeEndEventId: i32 = 20019;
pub const UIA_MenuModeStartEventId: i32 = 20018;
pub const UIA_MenuOpenedEventId: i32 = 20003;
pub const UIA_MultipleViewCurrentViewPropertyId: i32 = 30071;
pub const UIA_MultipleViewPatternId: i32 = 10008;
pub const UIA_MultipleViewSupportedViewsPropertyId: i32 = 30072;
pub const UIA_NamePropertyId: i32 = 30005;
pub const UIA_NativeWindowHandlePropertyId: i32 = 30020;
pub const UIA_NavigationLandmarkTypeId: i32 = 80003;
pub const UIA_NotificationEventId: i32 = 20035;
pub const UIA_ObjectModelPatternId: i32 = 10022;
pub const UIA_OptimizeForVisualContentPropertyId: i32 = 30111;
pub const UIA_OrientationPropertyId: i32 = 30023;
pub const UIA_OutlineColorPropertyId: i32 = 30161;
pub const UIA_OutlineStylesAttributeId: i32 = 40022;
pub const UIA_OutlineThicknessPropertyId: i32 = 30164;
pub const UIA_OverlineColorAttributeId: i32 = 40023;
pub const UIA_OverlineStyleAttributeId: i32 = 40024;
pub const UIA_PaneControlTypeId: i32 = 50033;
pub const UIA_PositionInSetPropertyId: i32 = 30152;
pub const UIA_ProcessIdPropertyId: i32 = 30002;
pub const UIA_ProgressBarControlTypeId: i32 = 50012;
pub const UIA_ProviderDescriptionPropertyId: i32 = 30107;
pub const UIA_RadioButtonControlTypeId: i32 = 50013;
pub const UIA_RangeValueIsReadOnlyPropertyId: i32 = 30048;
pub const UIA_RangeValueLargeChangePropertyId: i32 = 30051;
pub const UIA_RangeValueMaximumPropertyId: i32 = 30050;
pub const UIA_RangeValueMinimumPropertyId: i32 = 30049;
pub const UIA_RangeValuePatternId: i32 = 10003;
pub const UIA_RangeValueSmallChangePropertyId: i32 = 30052;
pub const UIA_RangeValueValuePropertyId: i32 = 30047;
pub const UIA_RotationPropertyId: i32 = 30166;
pub const UIA_RuntimeIdPropertyId: i32 = 30000;
pub const UIA_SayAsInterpretAsAttributeId: i32 = 40043;
pub const UIA_SayAsInterpretAsMetadataId: i32 = 100000;
pub const UIA_ScrollBarControlTypeId: i32 = 50014;
pub const UIA_ScrollHorizontalScrollPercentPropertyId: i32 = 30053;
pub const UIA_ScrollHorizontalViewSizePropertyId: i32 = 30054;
pub const UIA_ScrollHorizontallyScrollablePropertyId: i32 = 30057;
pub const UIA_ScrollItemPatternId: i32 = 10017;
pub const UIA_ScrollPatternId: i32 = 10004;
pub const UIA_ScrollVerticalScrollPercentPropertyId: i32 = 30055;
pub const UIA_ScrollVerticalViewSizePropertyId: i32 = 30056;
pub const UIA_ScrollVerticallyScrollablePropertyId: i32 = 30058;
pub const UIA_SearchLandmarkTypeId: i32 = 80004;
pub const UIA_Selection2CurrentSelectedItemPropertyId: i32 = 30171;
pub const UIA_Selection2FirstSelectedItemPropertyId: i32 = 30169;
pub const UIA_Selection2ItemCountPropertyId: i32 = 30172;
pub const UIA_Selection2LastSelectedItemPropertyId: i32 = 30170;
pub const UIA_SelectionActiveEndAttributeId: i32 = 40037;
pub const UIA_SelectionCanSelectMultiplePropertyId: i32 = 30060;
pub const UIA_SelectionIsSelectionRequiredPropertyId: i32 = 30061;
pub const UIA_SelectionItemIsSelectedPropertyId: i32 = 30079;
pub const UIA_SelectionItemPatternId: i32 = 10010;
pub const UIA_SelectionItemSelectionContainerPropertyId: i32 = 30080;
pub const UIA_SelectionItem_ElementAddedToSelectionEventId: i32 = 20010;
pub const UIA_SelectionItem_ElementRemovedFromSelectionEventId: i32 = 20011;
pub const UIA_SelectionItem_ElementSelectedEventId: i32 = 20012;
pub const UIA_SelectionPattern2Id: i32 = 10034;
pub const UIA_SelectionPatternId: i32 = 10001;
pub const UIA_SelectionSelectionPropertyId: i32 = 30059;
pub const UIA_Selection_InvalidatedEventId: i32 = 20013;
pub const UIA_SemanticZoomControlTypeId: i32 = 50039;
pub const UIA_SeparatorControlTypeId: i32 = 50038;
pub const UIA_SizeOfSetPropertyId: i32 = 30153;
pub const UIA_SizePropertyId: i32 = 30167;
pub const UIA_SliderControlTypeId: i32 = 50015;
pub const UIA_SpinnerControlTypeId: i32 = 50016;
pub const UIA_SplitButtonControlTypeId: i32 = 50031;
pub const UIA_SpreadsheetItemAnnotationObjectsPropertyId: i32 = 30130;
pub const UIA_SpreadsheetItemAnnotationTypesPropertyId: i32 = 30131;
pub const UIA_SpreadsheetItemFormulaPropertyId: i32 = 30129;
pub const UIA_SpreadsheetItemPatternId: i32 = 10027;
pub const UIA_SpreadsheetPatternId: i32 = 10026;
pub const UIA_StatusBarControlTypeId: i32 = 50017;
pub const UIA_StrikethroughColorAttributeId: i32 = 40025;
pub const UIA_StrikethroughStyleAttributeId: i32 = 40026;
pub const UIA_StructureChangedEventId: i32 = 20002;
pub const UIA_StyleIdAttributeId: i32 = 40034;
pub const UIA_StyleNameAttributeId: i32 = 40033;
pub const UIA_StylesExtendedPropertiesPropertyId: i32 = 30126;
pub const UIA_StylesFillColorPropertyId: i32 = 30122;
pub const UIA_StylesFillPatternColorPropertyId: i32 = 30125;
pub const UIA_StylesFillPatternStylePropertyId: i32 = 30123;
pub const UIA_StylesPatternId: i32 = 10025;
pub const UIA_StylesShapePropertyId: i32 = 30124;
pub const UIA_StylesStyleIdPropertyId: i32 = 30120;
pub const UIA_StylesStyleNamePropertyId: i32 = 30121;
pub const UIA_SummaryChangeId: i32 = 90000;
pub const UIA_SynchronizedInputPatternId: i32 = 10021;
pub const UIA_SystemAlertEventId: i32 = 20023;
pub const UIA_TabControlTypeId: i32 = 50018;
pub const UIA_TabItemControlTypeId: i32 = 50019;
pub const UIA_TableColumnHeadersPropertyId: i32 = 30082;
pub const UIA_TableControlTypeId: i32 = 50036;
pub const UIA_TableItemColumnHeaderItemsPropertyId: i32 = 30085;
pub const UIA_TableItemPatternId: i32 = 10013;
pub const UIA_TableItemRowHeaderItemsPropertyId: i32 = 30084;
pub const UIA_TablePatternId: i32 = 10012;
pub const UIA_TableRowHeadersPropertyId: i32 = 30081;
pub const UIA_TableRowOrColumnMajorPropertyId: i32 = 30083;
pub const UIA_TabsAttributeId: i32 = 40027;
pub const UIA_TextChildPatternId: i32 = 10029;
pub const UIA_TextControlTypeId: i32 = 50020;
pub const UIA_TextEditPatternId: i32 = 10032;
pub const UIA_TextEdit_ConversionTargetChangedEventId: i32 = 20033;
pub const UIA_TextEdit_TextChangedEventId: i32 = 20032;
pub const UIA_TextFlowDirectionsAttributeId: i32 = 40028;
pub const UIA_TextPattern2Id: i32 = 10024;
pub const UIA_TextPatternId: i32 = 10014;
pub const UIA_Text_TextChangedEventId: i32 = 20015;
pub const UIA_Text_TextSelectionChangedEventId: i32 = 20014;
pub const UIA_ThumbControlTypeId: i32 = 50027;
pub const UIA_TitleBarControlTypeId: i32 = 50037;
pub const UIA_TogglePatternId: i32 = 10015;
pub const UIA_ToggleToggleStatePropertyId: i32 = 30086;
pub const UIA_ToolBarControlTypeId: i32 = 50021;
pub const UIA_ToolTipClosedEventId: i32 = 20001;
pub const UIA_ToolTipControlTypeId: i32 = 50022;
pub const UIA_ToolTipOpenedEventId: i32 = 20000;
pub const UIA_Transform2CanZoomPropertyId: i32 = 30133;
pub const UIA_Transform2ZoomLevelPropertyId: i32 = 30145;
pub const UIA_Transform2ZoomMaximumPropertyId: i32 = 30147;
pub const UIA_Transform2ZoomMinimumPropertyId: i32 = 30146;
pub const UIA_TransformCanMovePropertyId: i32 = 30087;
pub const UIA_TransformCanResizePropertyId: i32 = 30088;
pub const UIA_TransformCanRotatePropertyId: i32 = 30089;
pub const UIA_TransformPattern2Id: i32 = 10028;
pub const UIA_TransformPatternId: i32 = 10016;
pub const UIA_TreeControlTypeId: i32 = 50023;
pub const UIA_TreeItemControlTypeId: i32 = 50024;
pub const UIA_UnderlineColorAttributeId: i32 = 40029;
pub const UIA_UnderlineStyleAttributeId: i32 = 40030;
pub const UIA_ValueIsReadOnlyPropertyId: i32 = 30046;
pub const UIA_ValuePatternId: i32 = 10002;
pub const UIA_ValueValuePropertyId: i32 = 30045;
pub const UIA_VirtualizedItemPatternId: i32 = 10020;
pub const UIA_VisualEffectsPropertyId: i32 = 30163;
pub const UIA_WindowCanMaximizePropertyId: i32 = 30073;
pub const UIA_WindowCanMinimizePropertyId: i32 = 30074;
pub const UIA_WindowControlTypeId: i32 = 50032;
pub const UIA_WindowIsModalPropertyId: i32 = 30077;
pub const UIA_WindowIsTopmostPropertyId: i32 = 30078;
pub const UIA_WindowPatternId: i32 = 10009;
pub const UIA_WindowWindowInteractionStatePropertyId: i32 = 30076;
pub const UIA_WindowWindowVisualStatePropertyId: i32 = 30075;
pub const UIA_Window_WindowClosedEventId: i32 = 20017;
pub const UIA_Window_WindowOpenedEventId: i32 = 20016;
