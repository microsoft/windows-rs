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
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExtendedProperty {
    pub PropertyName: core::mem::ManuallyDrop<windows_core::BSTR>,
    pub PropertyValue: core::mem::ManuallyDrop<windows_core::BSTR>,
}
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
    #[cfg(feature = "oaidl")]
    pub unsafe fn CompareRuntimeIds(&self, runtimeid1: *const super::oaidl::SAFEARRAY, runtimeid2: *const super::oaidl::SAFEARRAY) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CompareRuntimeIds)(windows_core::Interface::as_raw(self), runtimeid1, runtimeid2, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetRootElement(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRootElement)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ElementFromHandle(&self, hwnd: UIA_HWND) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElementFromHandle)(windows_core::Interface::as_raw(self), hwnd, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ElementFromPoint(&self, pt: super::windef::POINT) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElementFromPoint)(windows_core::Interface::as_raw(self), core::mem::transmute(pt), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFocusedElement(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFocusedElement)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetRootElementBuildCache<P0>(&self, cacherequest: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRootElementBuildCache)(windows_core::Interface::as_raw(self), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ElementFromHandleBuildCache<P1>(&self, hwnd: UIA_HWND, cacherequest: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElementFromHandleBuildCache)(windows_core::Interface::as_raw(self), hwnd, cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn ElementFromPointBuildCache<P1>(&self, pt: super::windef::POINT, cacherequest: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElementFromPointBuildCache)(windows_core::Interface::as_raw(self), core::mem::transmute(pt), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFocusedElementBuildCache<P0>(&self, cacherequest: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFocusedElementBuildCache)(windows_core::Interface::as_raw(self), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTreeWalker<P0>(&self, pcondition: P0) -> windows_core::Result<IUIAutomationTreeWalker>
    where
        P0: windows_core::Param<IUIAutomationCondition>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTreeWalker)(windows_core::Interface::as_raw(self), pcondition.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ControlViewWalker(&self) -> windows_core::Result<IUIAutomationTreeWalker> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ControlViewWalker)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ContentViewWalker(&self) -> windows_core::Result<IUIAutomationTreeWalker> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContentViewWalker)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RawViewWalker(&self) -> windows_core::Result<IUIAutomationTreeWalker> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RawViewWalker)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RawViewCondition(&self) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RawViewCondition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ControlViewCondition(&self) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ControlViewCondition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ContentViewCondition(&self) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContentViewCondition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateCacheRequest(&self) -> windows_core::Result<IUIAutomationCacheRequest> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateCacheRequest)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateTrueCondition(&self) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTrueCondition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateFalseCondition(&self) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateFalseCondition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn CreatePropertyCondition(&self, propertyid: super::uiautomationcore::PROPERTYID, value: &super::oaidl::VARIANT) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePropertyCondition)(windows_core::Interface::as_raw(self), propertyid, core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn CreatePropertyConditionEx(&self, propertyid: super::uiautomationcore::PROPERTYID, value: &super::oaidl::VARIANT, flags: PropertyConditionFlags) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreatePropertyConditionEx)(windows_core::Interface::as_raw(self), propertyid, core::mem::transmute_copy(value), flags, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateAndCondition<P0, P1>(&self, condition1: P0, condition2: P1) -> windows_core::Result<IUIAutomationCondition>
    where
        P0: windows_core::Param<IUIAutomationCondition>,
        P1: windows_core::Param<IUIAutomationCondition>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAndCondition)(windows_core::Interface::as_raw(self), condition1.param().abi(), condition2.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn CreateAndConditionFromArray(&self, conditions: *const super::oaidl::SAFEARRAY) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAndConditionFromArray)(windows_core::Interface::as_raw(self), conditions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateAndConditionFromNativeArray(&self, conditions: *const Option<IUIAutomationCondition>, conditioncount: i32) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateAndConditionFromNativeArray)(windows_core::Interface::as_raw(self), core::mem::transmute(conditions), conditioncount, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateOrCondition<P0, P1>(&self, condition1: P0, condition2: P1) -> windows_core::Result<IUIAutomationCondition>
    where
        P0: windows_core::Param<IUIAutomationCondition>,
        P1: windows_core::Param<IUIAutomationCondition>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateOrCondition)(windows_core::Interface::as_raw(self), condition1.param().abi(), condition2.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn CreateOrConditionFromArray(&self, conditions: *const super::oaidl::SAFEARRAY) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateOrConditionFromArray)(windows_core::Interface::as_raw(self), conditions, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateOrConditionFromNativeArray(&self, conditions: *const Option<IUIAutomationCondition>, conditioncount: i32) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateOrConditionFromNativeArray)(windows_core::Interface::as_raw(self), core::mem::transmute(conditions), conditioncount, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CreateNotCondition<P0>(&self, condition: P0) -> windows_core::Result<IUIAutomationCondition>
    where
        P0: windows_core::Param<IUIAutomationCondition>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateNotCondition)(windows_core::Interface::as_raw(self), condition.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn AddAutomationEventHandler<P1, P3, P4>(&self, eventid: super::uiautomationcore::EVENTID, element: P1, scope: TreeScope, cacherequest: P3, handler: P4) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IUIAutomationElement>,
        P3: windows_core::Param<IUIAutomationCacheRequest>,
        P4: windows_core::Param<IUIAutomationEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddAutomationEventHandler)(windows_core::Interface::as_raw(self), eventid, element.param().abi(), scope, cacherequest.param().abi(), handler.param().abi()) }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn RemoveAutomationEventHandler<P1, P2>(&self, eventid: super::uiautomationcore::EVENTID, element: P1, handler: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IUIAutomationElement>,
        P2: windows_core::Param<IUIAutomationEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).RemoveAutomationEventHandler)(windows_core::Interface::as_raw(self), eventid, element.param().abi(), handler.param().abi()) }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn AddPropertyChangedEventHandlerNativeArray<P0, P2, P3>(&self, element: P0, scope: TreeScope, cacherequest: P2, handler: P3, propertyarray: *const super::uiautomationcore::PROPERTYID, propertycount: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P2: windows_core::Param<IUIAutomationCacheRequest>,
        P3: windows_core::Param<IUIAutomationPropertyChangedEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddPropertyChangedEventHandlerNativeArray)(windows_core::Interface::as_raw(self), element.param().abi(), scope, cacherequest.param().abi(), handler.param().abi(), propertyarray, propertycount) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn AddPropertyChangedEventHandler<P0, P2, P3>(&self, element: P0, scope: TreeScope, cacherequest: P2, handler: P3, propertyarray: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT
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
    #[cfg(feature = "oaidl")]
    pub unsafe fn IntNativeArrayToSafeArray(&self, array: *const i32, arraycount: i32) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IntNativeArrayToSafeArray)(windows_core::Interface::as_raw(self), array, arraycount, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn IntSafeArrayToNativeArray(&self, intarray: *const super::oaidl::SAFEARRAY, array: *mut *mut i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IntSafeArrayToNativeArray)(windows_core::Interface::as_raw(self), intarray, array as _, &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "oaidl", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn RectToVariant(&self, rc: super::windef::RECT) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RectToVariant)(windows_core::Interface::as_raw(self), core::mem::transmute(rc), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn VariantToRect(&self, var: &super::oaidl::VARIANT) -> windows_core::Result<super::windef::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VariantToRect)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(var), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "oaidl", feature = "windef"))]
    pub unsafe fn SafeArrayToRectNativeArray(&self, rects: *const super::oaidl::SAFEARRAY, rectarray: *mut *mut super::windef::RECT) -> windows_core::Result<i32> {
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
            (windows_core::Interface::vtable(self).CreateProxyFactoryEntry)(windows_core::Interface::as_raw(self), factory.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ProxyFactoryMapping(&self) -> windows_core::Result<IUIAutomationProxyFactoryMapping> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProxyFactoryMapping)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn GetPropertyProgrammaticName(&self, property: super::uiautomationcore::PROPERTYID) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyProgrammaticName)(windows_core::Interface::as_raw(self), property, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn GetPatternProgrammaticName(&self, pattern: super::uiautomationcore::PATTERNID) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPatternProgrammaticName)(windows_core::Interface::as_raw(self), pattern, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn PollForPotentialSupportedPatterns<P0>(&self, pelement: P0, patternids: *mut *mut super::oaidl::SAFEARRAY, patternnames: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe { (windows_core::Interface::vtable(self).PollForPotentialSupportedPatterns)(windows_core::Interface::as_raw(self), pelement.param().abi(), patternids as _, patternnames as _) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn PollForPotentialSupportedProperties<P0>(&self, pelement: P0, propertyids: *mut *mut super::oaidl::SAFEARRAY, propertynames: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe { (windows_core::Interface::vtable(self).PollForPotentialSupportedProperties)(windows_core::Interface::as_raw(self), pelement.param().abi(), propertyids as _, propertynames as _) }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn CheckNotSupported(&self, value: &super::oaidl::VARIANT) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CheckNotSupported)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(value), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ReservedNotSupportedValue(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReservedNotSupportedValue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ReservedMixedAttributeValue(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ReservedMixedAttributeValue)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "oleacc"))]
    pub unsafe fn ElementFromIAccessible<P0>(&self, accessible: P0, childid: i32) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<super::oleacc::IAccessible>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElementFromIAccessible)(windows_core::Interface::as_raw(self), accessible.param().abi(), childid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "oleacc"))]
    pub unsafe fn ElementFromIAccessibleBuildCache<P0, P2>(&self, accessible: P0, childid: i32, cacherequest: P2) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<super::oleacc::IAccessible>,
        P2: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElementFromIAccessibleBuildCache)(windows_core::Interface::as_raw(self), accessible.param().abi(), childid, cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CompareElements: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub CompareRuntimeIds: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::SAFEARRAY, *const super::oaidl::SAFEARRAY, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    CompareRuntimeIds: usize,
    pub GetRootElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ElementFromHandle: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_HWND, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub ElementFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::POINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ElementFromPoint: usize,
    pub GetFocusedElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetRootElementBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ElementFromHandleBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_HWND, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub ElementFromPointBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::POINT, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    ElementFromPointBuildCache: usize,
    pub GetFocusedElementBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTreeWalker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ControlViewWalker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ContentViewWalker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RawViewWalker: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RawViewCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ControlViewCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ContentViewCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateCacheRequest: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTrueCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateFalseCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
    pub CreatePropertyCondition: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::PROPERTYID, super::oaidl::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase")))]
    CreatePropertyCondition: usize,
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
    pub CreatePropertyConditionEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::PROPERTYID, super::oaidl::VARIANT, PropertyConditionFlags, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase")))]
    CreatePropertyConditionEx: usize,
    pub CreateAndCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub CreateAndConditionFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::SAFEARRAY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    CreateAndConditionFromArray: usize,
    pub CreateAndConditionFromNativeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateOrCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub CreateOrConditionFromArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::SAFEARRAY, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    CreateOrConditionFromArray: usize,
    pub CreateOrConditionFromNativeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateNotCondition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub AddAutomationEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::EVENTID, *mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    AddAutomationEventHandler: usize,
    #[cfg(feature = "uiautomationcore")]
    pub RemoveAutomationEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::EVENTID, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    RemoveAutomationEventHandler: usize,
    #[cfg(feature = "uiautomationcore")]
    pub AddPropertyChangedEventHandlerNativeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::uiautomationcore::PROPERTYID, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    AddPropertyChangedEventHandlerNativeArray: usize,
    #[cfg(feature = "oaidl")]
    pub AddPropertyChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    AddPropertyChangedEventHandler: usize,
    pub RemovePropertyChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddStructureChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveStructureChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddFocusChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveFocusChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveAllEventHandlers: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub IntNativeArrayToSafeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const i32, i32, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    IntNativeArrayToSafeArray: usize,
    #[cfg(feature = "oaidl")]
    pub IntSafeArrayToNativeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::SAFEARRAY, *mut *mut i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    IntSafeArrayToNativeArray: usize,
    #[cfg(all(feature = "oaidl", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
    pub RectToVariant: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::RECT, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "windef", feature = "wtypes", feature = "wtypesbase")))]
    RectToVariant: usize,
    #[cfg(all(feature = "oaidl", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
    pub VariantToRect: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "windef", feature = "wtypes", feature = "wtypesbase")))]
    VariantToRect: usize,
    #[cfg(all(feature = "oaidl", feature = "windef"))]
    pub SafeArrayToRectNativeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::SAFEARRAY, *mut *mut super::windef::RECT, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "windef")))]
    SafeArrayToRectNativeArray: usize,
    pub CreateProxyFactoryEntry: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ProxyFactoryMapping: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub GetPropertyProgrammaticName: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::PROPERTYID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    GetPropertyProgrammaticName: usize,
    #[cfg(feature = "uiautomationcore")]
    pub GetPatternProgrammaticName: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::PATTERNID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    GetPatternProgrammaticName: usize,
    #[cfg(feature = "oaidl")]
    pub PollForPotentialSupportedPatterns: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    PollForPotentialSupportedPatterns: usize,
    #[cfg(feature = "oaidl")]
    pub PollForPotentialSupportedProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    PollForPotentialSupportedProperties: usize,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub CheckNotSupported: unsafe extern "system" fn(*mut core::ffi::c_void, super::oaidl::VARIANT, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    CheckNotSupported: usize,
    pub ReservedNotSupportedValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ReservedMixedAttributeValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "oleacc"))]
    pub ElementFromIAccessible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "oleacc")))]
    ElementFromIAccessible: usize,
    #[cfg(all(feature = "oaidl", feature = "oleacc"))]
    pub ElementFromIAccessibleBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, i32, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "oleacc")))]
    ElementFromIAccessibleBuildCache: usize,
}
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomation_Impl: windows_core::IUnknownImpl {
    fn CompareElements(&self, el1: windows_core::Ref<IUIAutomationElement>, el2: windows_core::Ref<IUIAutomationElement>) -> windows_core::Result<windows_core::BOOL>;
    fn CompareRuntimeIds(&self, runtimeid1: *const super::oaidl::SAFEARRAY, runtimeid2: *const super::oaidl::SAFEARRAY) -> windows_core::Result<windows_core::BOOL>;
    fn GetRootElement(&self) -> windows_core::Result<IUIAutomationElement>;
    fn ElementFromHandle(&self, hwnd: UIA_HWND) -> windows_core::Result<IUIAutomationElement>;
    fn ElementFromPoint(&self, pt: &super::windef::POINT) -> windows_core::Result<IUIAutomationElement>;
    fn GetFocusedElement(&self) -> windows_core::Result<IUIAutomationElement>;
    fn GetRootElementBuildCache(&self, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>) -> windows_core::Result<IUIAutomationElement>;
    fn ElementFromHandleBuildCache(&self, hwnd: UIA_HWND, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>) -> windows_core::Result<IUIAutomationElement>;
    fn ElementFromPointBuildCache(&self, pt: &super::windef::POINT, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>) -> windows_core::Result<IUIAutomationElement>;
    fn GetFocusedElementBuildCache(&self, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>) -> windows_core::Result<IUIAutomationElement>;
    fn CreateTreeWalker(&self, pcondition: windows_core::Ref<IUIAutomationCondition>) -> windows_core::Result<IUIAutomationTreeWalker>;
    fn ControlViewWalker(&self) -> windows_core::Result<IUIAutomationTreeWalker>;
    fn ContentViewWalker(&self) -> windows_core::Result<IUIAutomationTreeWalker>;
    fn RawViewWalker(&self) -> windows_core::Result<IUIAutomationTreeWalker>;
    fn RawViewCondition(&self) -> windows_core::Result<IUIAutomationCondition>;
    fn ControlViewCondition(&self) -> windows_core::Result<IUIAutomationCondition>;
    fn ContentViewCondition(&self) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateCacheRequest(&self) -> windows_core::Result<IUIAutomationCacheRequest>;
    fn CreateTrueCondition(&self) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateFalseCondition(&self) -> windows_core::Result<IUIAutomationCondition>;
    fn CreatePropertyCondition(&self, propertyid: super::uiautomationcore::PROPERTYID, value: &super::oaidl::VARIANT) -> windows_core::Result<IUIAutomationCondition>;
    fn CreatePropertyConditionEx(&self, propertyid: super::uiautomationcore::PROPERTYID, value: &super::oaidl::VARIANT, flags: PropertyConditionFlags) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateAndCondition(&self, condition1: windows_core::Ref<IUIAutomationCondition>, condition2: windows_core::Ref<IUIAutomationCondition>) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateAndConditionFromArray(&self, conditions: *const super::oaidl::SAFEARRAY) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateAndConditionFromNativeArray(&self, conditions: *const Option<IUIAutomationCondition>, conditioncount: i32) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateOrCondition(&self, condition1: windows_core::Ref<IUIAutomationCondition>, condition2: windows_core::Ref<IUIAutomationCondition>) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateOrConditionFromArray(&self, conditions: *const super::oaidl::SAFEARRAY) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateOrConditionFromNativeArray(&self, conditions: *const Option<IUIAutomationCondition>, conditioncount: i32) -> windows_core::Result<IUIAutomationCondition>;
    fn CreateNotCondition(&self, condition: windows_core::Ref<IUIAutomationCondition>) -> windows_core::Result<IUIAutomationCondition>;
    fn AddAutomationEventHandler(&self, eventid: super::uiautomationcore::EVENTID, element: windows_core::Ref<IUIAutomationElement>, scope: TreeScope, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationEventHandler>) -> windows_core::Result<()>;
    fn RemoveAutomationEventHandler(&self, eventid: super::uiautomationcore::EVENTID, element: windows_core::Ref<IUIAutomationElement>, handler: windows_core::Ref<IUIAutomationEventHandler>) -> windows_core::Result<()>;
    fn AddPropertyChangedEventHandlerNativeArray(&self, element: windows_core::Ref<IUIAutomationElement>, scope: TreeScope, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationPropertyChangedEventHandler>, propertyarray: *const super::uiautomationcore::PROPERTYID, propertycount: i32) -> windows_core::Result<()>;
    fn AddPropertyChangedEventHandler(&self, element: windows_core::Ref<IUIAutomationElement>, scope: TreeScope, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationPropertyChangedEventHandler>, propertyarray: *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
    fn RemovePropertyChangedEventHandler(&self, element: windows_core::Ref<IUIAutomationElement>, handler: windows_core::Ref<IUIAutomationPropertyChangedEventHandler>) -> windows_core::Result<()>;
    fn AddStructureChangedEventHandler(&self, element: windows_core::Ref<IUIAutomationElement>, scope: TreeScope, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationStructureChangedEventHandler>) -> windows_core::Result<()>;
    fn RemoveStructureChangedEventHandler(&self, element: windows_core::Ref<IUIAutomationElement>, handler: windows_core::Ref<IUIAutomationStructureChangedEventHandler>) -> windows_core::Result<()>;
    fn AddFocusChangedEventHandler(&self, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationFocusChangedEventHandler>) -> windows_core::Result<()>;
    fn RemoveFocusChangedEventHandler(&self, handler: windows_core::Ref<IUIAutomationFocusChangedEventHandler>) -> windows_core::Result<()>;
    fn RemoveAllEventHandlers(&self) -> windows_core::Result<()>;
    fn IntNativeArrayToSafeArray(&self, array: *const i32, arraycount: i32) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn IntSafeArrayToNativeArray(&self, intarray: *const super::oaidl::SAFEARRAY, array: *mut *mut i32) -> windows_core::Result<i32>;
    fn RectToVariant(&self, rc: &super::windef::RECT) -> windows_core::Result<super::oaidl::VARIANT>;
    fn VariantToRect(&self, var: &super::oaidl::VARIANT) -> windows_core::Result<super::windef::RECT>;
    fn SafeArrayToRectNativeArray(&self, rects: *const super::oaidl::SAFEARRAY, rectarray: *mut *mut super::windef::RECT) -> windows_core::Result<i32>;
    fn CreateProxyFactoryEntry(&self, factory: windows_core::Ref<IUIAutomationProxyFactory>) -> windows_core::Result<IUIAutomationProxyFactoryEntry>;
    fn ProxyFactoryMapping(&self) -> windows_core::Result<IUIAutomationProxyFactoryMapping>;
    fn GetPropertyProgrammaticName(&self, property: super::uiautomationcore::PROPERTYID) -> windows_core::Result<windows_core::BSTR>;
    fn GetPatternProgrammaticName(&self, pattern: super::uiautomationcore::PATTERNID) -> windows_core::Result<windows_core::BSTR>;
    fn PollForPotentialSupportedPatterns(&self, pelement: windows_core::Ref<IUIAutomationElement>, patternids: *mut *mut super::oaidl::SAFEARRAY, patternnames: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
    fn PollForPotentialSupportedProperties(&self, pelement: windows_core::Ref<IUIAutomationElement>, propertyids: *mut *mut super::oaidl::SAFEARRAY, propertynames: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
    fn CheckNotSupported(&self, value: &super::oaidl::VARIANT) -> windows_core::Result<windows_core::BOOL>;
    fn ReservedNotSupportedValue(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn ReservedMixedAttributeValue(&self) -> windows_core::Result<windows_core::IUnknown>;
    fn ElementFromIAccessible(&self, accessible: windows_core::Ref<super::oleacc::IAccessible>, childid: i32) -> windows_core::Result<IUIAutomationElement>;
    fn ElementFromIAccessibleBuildCache(&self, accessible: windows_core::Ref<super::oleacc::IAccessible>, childid: i32, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>) -> windows_core::Result<IUIAutomationElement>;
}
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
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
        unsafe extern "system" fn CompareRuntimeIds<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runtimeid1: *const super::oaidl::SAFEARRAY, runtimeid2: *const super::oaidl::SAFEARRAY, aresame: *mut windows_core::BOOL) -> windows_core::HRESULT {
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
        unsafe extern "system" fn ElementFromPoint<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pt: super::windef::POINT, element: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn ElementFromPointBuildCache<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pt: super::windef::POINT, cacherequest: *mut core::ffi::c_void, element: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn ControlViewWalker<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, walker: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::ControlViewWalker(this) {
                    Ok(ok__) => {
                        walker.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ContentViewWalker<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, walker: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::ContentViewWalker(this) {
                    Ok(ok__) => {
                        walker.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RawViewWalker<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, walker: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::RawViewWalker(this) {
                    Ok(ok__) => {
                        walker.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RawViewCondition<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, condition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::RawViewCondition(this) {
                    Ok(ok__) => {
                        condition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ControlViewCondition<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, condition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::ControlViewCondition(this) {
                    Ok(ok__) => {
                        condition.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ContentViewCondition<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, condition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::ContentViewCondition(this) {
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
        unsafe extern "system" fn CreatePropertyCondition<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: super::uiautomationcore::PROPERTYID, value: super::oaidl::VARIANT, newcondition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn CreatePropertyConditionEx<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: super::uiautomationcore::PROPERTYID, value: super::oaidl::VARIANT, flags: PropertyConditionFlags, newcondition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn CreateAndConditionFromArray<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, conditions: *const super::oaidl::SAFEARRAY, newcondition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn CreateOrConditionFromArray<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, conditions: *const super::oaidl::SAFEARRAY, newcondition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn AddAutomationEventHandler<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: super::uiautomationcore::EVENTID, element: *mut core::ffi::c_void, scope: TreeScope, cacherequest: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::AddAutomationEventHandler(this, core::mem::transmute_copy(&eventid), core::mem::transmute_copy(&element), core::mem::transmute_copy(&scope), core::mem::transmute_copy(&cacherequest), core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn RemoveAutomationEventHandler<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: super::uiautomationcore::EVENTID, element: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::RemoveAutomationEventHandler(this, core::mem::transmute_copy(&eventid), core::mem::transmute_copy(&element), core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn AddPropertyChangedEventHandlerNativeArray<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, scope: TreeScope, cacherequest: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, propertyarray: *const super::uiautomationcore::PROPERTYID, propertycount: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::AddPropertyChangedEventHandlerNativeArray(this, core::mem::transmute_copy(&element), core::mem::transmute_copy(&scope), core::mem::transmute_copy(&cacherequest), core::mem::transmute_copy(&handler), core::mem::transmute_copy(&propertyarray), core::mem::transmute_copy(&propertycount)).into()
            }
        }
        unsafe extern "system" fn AddPropertyChangedEventHandler<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, scope: TreeScope, cacherequest: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, propertyarray: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
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
        unsafe extern "system" fn IntNativeArrayToSafeArray<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, array: *const i32, arraycount: i32, safearray: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
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
        unsafe extern "system" fn IntSafeArrayToNativeArray<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, intarray: *const super::oaidl::SAFEARRAY, array: *mut *mut i32, arraycount: *mut i32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn RectToVariant<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rc: super::windef::RECT, var: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn VariantToRect<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, var: super::oaidl::VARIANT, rc: *mut super::windef::RECT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SafeArrayToRectNativeArray<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, rects: *const super::oaidl::SAFEARRAY, rectarray: *mut *mut super::windef::RECT, rectarraycount: *mut i32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn ProxyFactoryMapping<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, factorymapping: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::ProxyFactoryMapping(this) {
                    Ok(ok__) => {
                        factorymapping.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropertyProgrammaticName<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: super::uiautomationcore::PROPERTYID, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetPatternProgrammaticName<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattern: super::uiautomationcore::PATTERNID, name: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn PollForPotentialSupportedPatterns<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pelement: *mut core::ffi::c_void, patternids: *mut *mut super::oaidl::SAFEARRAY, patternnames: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::PollForPotentialSupportedPatterns(this, core::mem::transmute_copy(&pelement), core::mem::transmute_copy(&patternids), core::mem::transmute_copy(&patternnames)).into()
            }
        }
        unsafe extern "system" fn PollForPotentialSupportedProperties<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pelement: *mut core::ffi::c_void, propertyids: *mut *mut super::oaidl::SAFEARRAY, propertynames: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation_Impl::PollForPotentialSupportedProperties(this, core::mem::transmute_copy(&pelement), core::mem::transmute_copy(&propertyids), core::mem::transmute_copy(&propertynames)).into()
            }
        }
        unsafe extern "system" fn CheckNotSupported<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, value: super::oaidl::VARIANT, isnotsupported: *mut windows_core::BOOL) -> windows_core::HRESULT {
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
        unsafe extern "system" fn ReservedNotSupportedValue<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, notsupportedvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::ReservedNotSupportedValue(this) {
                    Ok(ok__) => {
                        notsupportedvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ReservedMixedAttributeValue<Identity: IUIAutomation_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mixedattributevalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation_Impl::ReservedMixedAttributeValue(this) {
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
            ControlViewWalker: ControlViewWalker::<Identity, OFFSET>,
            ContentViewWalker: ContentViewWalker::<Identity, OFFSET>,
            RawViewWalker: RawViewWalker::<Identity, OFFSET>,
            RawViewCondition: RawViewCondition::<Identity, OFFSET>,
            ControlViewCondition: ControlViewCondition::<Identity, OFFSET>,
            ContentViewCondition: ContentViewCondition::<Identity, OFFSET>,
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
            ProxyFactoryMapping: ProxyFactoryMapping::<Identity, OFFSET>,
            GetPropertyProgrammaticName: GetPropertyProgrammaticName::<Identity, OFFSET>,
            GetPatternProgrammaticName: GetPatternProgrammaticName::<Identity, OFFSET>,
            PollForPotentialSupportedPatterns: PollForPotentialSupportedPatterns::<Identity, OFFSET>,
            PollForPotentialSupportedProperties: PollForPotentialSupportedProperties::<Identity, OFFSET>,
            CheckNotSupported: CheckNotSupported::<Identity, OFFSET>,
            ReservedNotSupportedValue: ReservedNotSupportedValue::<Identity, OFFSET>,
            ReservedMixedAttributeValue: ReservedMixedAttributeValue::<Identity, OFFSET>,
            ElementFromIAccessible: ElementFromIAccessible::<Identity, OFFSET>,
            ElementFromIAccessibleBuildCache: ElementFromIAccessibleBuildCache::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomation as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn AutoSetFocus(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AutoSetFocus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAutoSetFocus(&self, autosetfocus: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAutoSetFocus)(windows_core::Interface::as_raw(self), autosetfocus.into()) }
    }
    pub unsafe fn ConnectionTimeout(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConnectionTimeout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetConnectionTimeout(&self, timeout: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetConnectionTimeout)(windows_core::Interface::as_raw(self), timeout) }
    }
    pub unsafe fn TransactionTimeout(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TransactionTimeout)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTransactionTimeout(&self, timeout: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTransactionTimeout)(windows_core::Interface::as_raw(self), timeout) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomation2_Vtbl {
    pub base__: IUIAutomation_Vtbl,
    pub AutoSetFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetAutoSetFocus: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub ConnectionTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetConnectionTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub TransactionTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub SetTransactionTimeout: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomation2_Impl: IUIAutomation_Impl {
    fn AutoSetFocus(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetAutoSetFocus(&self, autosetfocus: windows_core::BOOL) -> windows_core::Result<()>;
    fn ConnectionTimeout(&self) -> windows_core::Result<u32>;
    fn SetConnectionTimeout(&self, timeout: u32) -> windows_core::Result<()>;
    fn TransactionTimeout(&self) -> windows_core::Result<u32>;
    fn SetTransactionTimeout(&self, timeout: u32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomation2_Vtbl {
    pub const fn new<Identity: IUIAutomation2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AutoSetFocus<Identity: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, autosetfocus: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation2_Impl::AutoSetFocus(this) {
                    Ok(ok__) => {
                        autosetfocus.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAutoSetFocus<Identity: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, autosetfocus: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation2_Impl::SetAutoSetFocus(this, core::mem::transmute_copy(&autosetfocus)).into()
            }
        }
        unsafe extern "system" fn ConnectionTimeout<Identity: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeout: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation2_Impl::ConnectionTimeout(this) {
                    Ok(ok__) => {
                        timeout.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetConnectionTimeout<Identity: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeout: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation2_Impl::SetConnectionTimeout(this, core::mem::transmute_copy(&timeout)).into()
            }
        }
        unsafe extern "system" fn TransactionTimeout<Identity: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeout: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation2_Impl::TransactionTimeout(this) {
                    Ok(ok__) => {
                        timeout.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTransactionTimeout<Identity: IUIAutomation2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, timeout: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation2_Impl::SetTransactionTimeout(this, core::mem::transmute_copy(&timeout)).into()
            }
        }
        Self {
            base__: IUIAutomation_Vtbl::new::<Identity, OFFSET>(),
            AutoSetFocus: AutoSetFocus::<Identity, OFFSET>,
            SetAutoSetFocus: SetAutoSetFocus::<Identity, OFFSET>,
            ConnectionTimeout: ConnectionTimeout::<Identity, OFFSET>,
            SetConnectionTimeout: SetConnectionTimeout::<Identity, OFFSET>,
            TransactionTimeout: TransactionTimeout::<Identity, OFFSET>,
            SetTransactionTimeout: SetTransactionTimeout::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomation2 as windows_core::Interface>::IID || iid == &<IUIAutomation as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn AddTextEditTextChangedEventHandler<P0, P3, P4>(&self, element: P0, scope: TreeScope, texteditchangetype: super::uiautomationcore::TextEditChangeType, cacherequest: P3, handler: P4) -> windows_core::HRESULT
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
    pub AddTextEditTextChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TreeScope, super::uiautomationcore::TextEditChangeType, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    AddTextEditTextChangedEventHandler: usize,
    pub RemoveTextEditTextChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomation3_Impl: IUIAutomation2_Impl {
    fn AddTextEditTextChangedEventHandler(&self, element: windows_core::Ref<IUIAutomationElement>, scope: TreeScope, texteditchangetype: super::uiautomationcore::TextEditChangeType, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationTextEditTextChangedEventHandler>) -> windows_core::Result<()>;
    fn RemoveTextEditTextChangedEventHandler(&self, element: windows_core::Ref<IUIAutomationElement>, handler: windows_core::Ref<IUIAutomationTextEditTextChangedEventHandler>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomation3_Vtbl {
    pub const fn new<Identity: IUIAutomation3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddTextEditTextChangedEventHandler<Identity: IUIAutomation3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, element: *mut core::ffi::c_void, scope: TreeScope, texteditchangetype: super::uiautomationcore::TextEditChangeType, cacherequest: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn AddChangesEventHandler<P0, P4, P5>(&self, element: P0, scope: TreeScope, changetypes: *const i32, changescount: i32, pcacherequest: P4, handler: P5) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P4: windows_core::Param<IUIAutomationCacheRequest>,
        P5: windows_core::Param<IUIAutomationChangesEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddChangesEventHandler)(windows_core::Interface::as_raw(self), element.param().abi(), scope, changetypes, changescount, pcacherequest.param().abi(), handler.param().abi()) }
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
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomation4_Impl: IUIAutomation3_Impl {
    fn AddChangesEventHandler(&self, element: windows_core::Ref<IUIAutomationElement>, scope: TreeScope, changetypes: *const i32, changescount: i32, pcacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationChangesEventHandler>) -> windows_core::Result<()>;
    fn RemoveChangesEventHandler(&self, element: windows_core::Ref<IUIAutomationElement>, handler: windows_core::Ref<IUIAutomationChangesEventHandler>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomation5_Impl: IUIAutomation4_Impl {
    fn AddNotificationEventHandler(&self, element: windows_core::Ref<IUIAutomationElement>, scope: TreeScope, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationNotificationEventHandler>) -> windows_core::Result<()>;
    fn RemoveNotificationEventHandler(&self, element: windows_core::Ref<IUIAutomationElement>, handler: windows_core::Ref<IUIAutomationNotificationEventHandler>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
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
            (windows_core::Interface::vtable(self).CreateEventHandlerGroup)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    pub unsafe fn ConnectionRecoveryBehavior(&self) -> windows_core::Result<ConnectionRecoveryBehaviorOptions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConnectionRecoveryBehavior)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetConnectionRecoveryBehavior(&self, connectionrecoverybehavioroptions: ConnectionRecoveryBehaviorOptions) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetConnectionRecoveryBehavior)(windows_core::Interface::as_raw(self), connectionrecoverybehavioroptions) }
    }
    pub unsafe fn CoalesceEvents(&self) -> windows_core::Result<CoalesceEventsOptions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CoalesceEvents)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetCoalesceEvents(&self, coalesceeventsoptions: CoalesceEventsOptions) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCoalesceEvents)(windows_core::Interface::as_raw(self), coalesceeventsoptions) }
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
    pub ConnectionRecoveryBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ConnectionRecoveryBehaviorOptions) -> windows_core::HRESULT,
    pub SetConnectionRecoveryBehavior: unsafe extern "system" fn(*mut core::ffi::c_void, ConnectionRecoveryBehaviorOptions) -> windows_core::HRESULT,
    pub CoalesceEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoalesceEventsOptions) -> windows_core::HRESULT,
    pub SetCoalesceEvents: unsafe extern "system" fn(*mut core::ffi::c_void, CoalesceEventsOptions) -> windows_core::HRESULT,
    pub AddActiveTextPositionChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveActiveTextPositionChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomation6_Impl: IUIAutomation5_Impl {
    fn CreateEventHandlerGroup(&self) -> windows_core::Result<IUIAutomationEventHandlerGroup>;
    fn AddEventHandlerGroup(&self, element: windows_core::Ref<IUIAutomationElement>, handlergroup: windows_core::Ref<IUIAutomationEventHandlerGroup>) -> windows_core::Result<()>;
    fn RemoveEventHandlerGroup(&self, element: windows_core::Ref<IUIAutomationElement>, handlergroup: windows_core::Ref<IUIAutomationEventHandlerGroup>) -> windows_core::Result<()>;
    fn ConnectionRecoveryBehavior(&self) -> windows_core::Result<ConnectionRecoveryBehaviorOptions>;
    fn SetConnectionRecoveryBehavior(&self, connectionrecoverybehavioroptions: ConnectionRecoveryBehaviorOptions) -> windows_core::Result<()>;
    fn CoalesceEvents(&self) -> windows_core::Result<CoalesceEventsOptions>;
    fn SetCoalesceEvents(&self, coalesceeventsoptions: CoalesceEventsOptions) -> windows_core::Result<()>;
    fn AddActiveTextPositionChangedEventHandler(&self, element: windows_core::Ref<IUIAutomationElement>, scope: TreeScope, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationActiveTextPositionChangedEventHandler>) -> windows_core::Result<()>;
    fn RemoveActiveTextPositionChangedEventHandler(&self, element: windows_core::Ref<IUIAutomationElement>, handler: windows_core::Ref<IUIAutomationActiveTextPositionChangedEventHandler>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
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
        unsafe extern "system" fn ConnectionRecoveryBehavior<Identity: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectionrecoverybehavioroptions: *mut ConnectionRecoveryBehaviorOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation6_Impl::ConnectionRecoveryBehavior(this) {
                    Ok(ok__) => {
                        connectionrecoverybehavioroptions.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetConnectionRecoveryBehavior<Identity: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, connectionrecoverybehavioroptions: ConnectionRecoveryBehaviorOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation6_Impl::SetConnectionRecoveryBehavior(this, core::mem::transmute_copy(&connectionrecoverybehavioroptions)).into()
            }
        }
        unsafe extern "system" fn CoalesceEvents<Identity: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, coalesceeventsoptions: *mut CoalesceEventsOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomation6_Impl::CoalesceEvents(this) {
                    Ok(ok__) => {
                        coalesceeventsoptions.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCoalesceEvents<Identity: IUIAutomation6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, coalesceeventsoptions: CoalesceEventsOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomation6_Impl::SetCoalesceEvents(this, core::mem::transmute_copy(&coalesceeventsoptions)).into()
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
            ConnectionRecoveryBehavior: ConnectionRecoveryBehavior::<Identity, OFFSET>,
            SetConnectionRecoveryBehavior: SetConnectionRecoveryBehavior::<Identity, OFFSET>,
            CoalesceEvents: CoalesceEvents::<Identity, OFFSET>,
            SetCoalesceEvents: SetCoalesceEvents::<Identity, OFFSET>,
            AddActiveTextPositionChangedEventHandler: AddActiveTextPositionChangedEventHandler::<Identity, OFFSET>,
            RemoveActiveTextPositionChangedEventHandler: RemoveActiveTextPositionChangedEventHandler::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomation6 as windows_core::Interface>::IID || iid == &<IUIAutomation as windows_core::Interface>::IID || iid == &<IUIAutomation2 as windows_core::Interface>::IID || iid == &<IUIAutomation3 as windows_core::Interface>::IID || iid == &<IUIAutomation4 as windows_core::Interface>::IID || iid == &<IUIAutomation5 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn ChildCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ChildCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetChildrenAsNativeArray(&self, childarray: *mut *mut Option<IUIAutomationCondition>, childarraycount: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetChildrenAsNativeArray)(windows_core::Interface::as_raw(self), childarray as _, childarraycount as _) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetChildren(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
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
    pub ChildCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetChildrenAsNativeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub GetChildren: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetChildren: usize,
}
#[cfg(feature = "oaidl")]
pub trait IUIAutomationAndCondition_Impl: IUIAutomationCondition_Impl {
    fn ChildCount(&self) -> windows_core::Result<i32>;
    fn GetChildrenAsNativeArray(&self, childarray: *mut *mut Option<IUIAutomationCondition>, childarraycount: *mut i32) -> windows_core::Result<()>;
    fn GetChildren(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
}
#[cfg(feature = "oaidl")]
impl IUIAutomationAndCondition_Vtbl {
    pub const fn new<Identity: IUIAutomationAndCondition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ChildCount<Identity: IUIAutomationAndCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, childcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationAndCondition_Impl::ChildCount(this) {
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
        unsafe extern "system" fn GetChildren<Identity: IUIAutomationAndCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, childarray: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
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
            ChildCount: ChildCount::<Identity, OFFSET>,
            GetChildrenAsNativeArray: GetChildrenAsNativeArray::<Identity, OFFSET>,
            GetChildren: GetChildren::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationAndCondition as windows_core::Interface>::IID || iid == &<IUIAutomationCondition as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IUIAutomationAndCondition {}
windows_core::imp::define_interface!(IUIAutomationAnnotationPattern, IUIAutomationAnnotationPattern_Vtbl, 0x9a175b21_339e_41b1_8e8b_623f6b681098);
windows_core::imp::interface_hierarchy!(IUIAutomationAnnotationPattern, windows_core::IUnknown);
impl IUIAutomationAnnotationPattern {
    pub unsafe fn CurrentAnnotationTypeId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentAnnotationTypeId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentAnnotationTypeName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentAnnotationTypeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentAuthor(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentAuthor)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentDateTime(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentDateTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentTarget(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CachedAnnotationTypeId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedAnnotationTypeId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedAnnotationTypeName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedAnnotationTypeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedAuthor(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedAuthor)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedDateTime(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedDateTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedTarget(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationAnnotationPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CurrentAnnotationTypeId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentAnnotationTypeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentAuthor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentDateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedAnnotationTypeId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedAnnotationTypeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedAuthor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedDateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAutomationAnnotationPattern_Impl: windows_core::IUnknownImpl {
    fn CurrentAnnotationTypeId(&self) -> windows_core::Result<i32>;
    fn CurrentAnnotationTypeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentAuthor(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentDateTime(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentTarget(&self) -> windows_core::Result<IUIAutomationElement>;
    fn CachedAnnotationTypeId(&self) -> windows_core::Result<i32>;
    fn CachedAnnotationTypeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedAuthor(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedDateTime(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedTarget(&self) -> windows_core::Result<IUIAutomationElement>;
}
impl IUIAutomationAnnotationPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentAnnotationTypeId<Identity: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationAnnotationPattern_Impl::CurrentAnnotationTypeId(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentAnnotationTypeName<Identity: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationAnnotationPattern_Impl::CurrentAnnotationTypeName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentAuthor<Identity: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationAnnotationPattern_Impl::CurrentAuthor(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentDateTime<Identity: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationAnnotationPattern_Impl::CurrentDateTime(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentTarget<Identity: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationAnnotationPattern_Impl::CurrentTarget(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedAnnotationTypeId<Identity: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationAnnotationPattern_Impl::CachedAnnotationTypeId(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedAnnotationTypeName<Identity: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationAnnotationPattern_Impl::CachedAnnotationTypeName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedAuthor<Identity: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationAnnotationPattern_Impl::CachedAuthor(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedDateTime<Identity: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationAnnotationPattern_Impl::CachedDateTime(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedTarget<Identity: IUIAutomationAnnotationPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationAnnotationPattern_Impl::CachedTarget(this) {
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
            CurrentAnnotationTypeId: CurrentAnnotationTypeId::<Identity, OFFSET>,
            CurrentAnnotationTypeName: CurrentAnnotationTypeName::<Identity, OFFSET>,
            CurrentAuthor: CurrentAuthor::<Identity, OFFSET>,
            CurrentDateTime: CurrentDateTime::<Identity, OFFSET>,
            CurrentTarget: CurrentTarget::<Identity, OFFSET>,
            CachedAnnotationTypeId: CachedAnnotationTypeId::<Identity, OFFSET>,
            CachedAnnotationTypeName: CachedAnnotationTypeName::<Identity, OFFSET>,
            CachedAuthor: CachedAuthor::<Identity, OFFSET>,
            CachedDateTime: CachedDateTime::<Identity, OFFSET>,
            CachedTarget: CachedTarget::<Identity, OFFSET>,
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
    pub unsafe fn BooleanValue(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BooleanValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationBoolCondition_Vtbl {
    pub base__: IUIAutomationCondition_Vtbl,
    pub BooleanValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IUIAutomationBoolCondition_Impl: IUIAutomationCondition_Impl {
    fn BooleanValue(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IUIAutomationBoolCondition_Vtbl {
    pub const fn new<Identity: IUIAutomationBoolCondition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn BooleanValue<Identity: IUIAutomationBoolCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, boolval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationBoolCondition_Impl::BooleanValue(this) {
                    Ok(ok__) => {
                        boolval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IUIAutomationCondition_Vtbl::new::<Identity, OFFSET>(), BooleanValue: BooleanValue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationBoolCondition as windows_core::Interface>::IID || iid == &<IUIAutomationCondition as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationBoolCondition {}
windows_core::imp::define_interface!(IUIAutomationCacheRequest, IUIAutomationCacheRequest_Vtbl, 0xb32a92b5_bc25_4078_9c08_d7ee95c48e03);
windows_core::imp::interface_hierarchy!(IUIAutomationCacheRequest, windows_core::IUnknown);
impl IUIAutomationCacheRequest {
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn AddProperty(&self, propertyid: super::uiautomationcore::PROPERTYID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddProperty)(windows_core::Interface::as_raw(self), propertyid) }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn AddPattern(&self, patternid: super::uiautomationcore::PATTERNID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddPattern)(windows_core::Interface::as_raw(self), patternid) }
    }
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn TreeScope(&self) -> windows_core::Result<TreeScope> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TreeScope)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetTreeScope(&self, scope: TreeScope) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTreeScope)(windows_core::Interface::as_raw(self), scope) }
    }
    pub unsafe fn TreeFilter(&self) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TreeFilter)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SetTreeFilter<P0>(&self, filter: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationCondition>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetTreeFilter)(windows_core::Interface::as_raw(self), filter.param().abi()) }
    }
    pub unsafe fn AutomationElementMode(&self) -> windows_core::Result<AutomationElementMode> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AutomationElementMode)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetAutomationElementMode(&self, mode: AutomationElementMode) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAutomationElementMode)(windows_core::Interface::as_raw(self), mode) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationCacheRequest_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "uiautomationcore")]
    pub AddProperty: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::PROPERTYID) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    AddProperty: usize,
    #[cfg(feature = "uiautomationcore")]
    pub AddPattern: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::PATTERNID) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    AddPattern: usize,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TreeScope: unsafe extern "system" fn(*mut core::ffi::c_void, *mut TreeScope) -> windows_core::HRESULT,
    pub SetTreeScope: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope) -> windows_core::HRESULT,
    pub TreeFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTreeFilter: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AutomationElementMode: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AutomationElementMode) -> windows_core::HRESULT,
    pub SetAutomationElementMode: unsafe extern "system" fn(*mut core::ffi::c_void, AutomationElementMode) -> windows_core::HRESULT,
}
#[cfg(feature = "uiautomationcore")]
pub trait IUIAutomationCacheRequest_Impl: windows_core::IUnknownImpl {
    fn AddProperty(&self, propertyid: super::uiautomationcore::PROPERTYID) -> windows_core::Result<()>;
    fn AddPattern(&self, patternid: super::uiautomationcore::PATTERNID) -> windows_core::Result<()>;
    fn Clone(&self) -> windows_core::Result<IUIAutomationCacheRequest>;
    fn TreeScope(&self) -> windows_core::Result<TreeScope>;
    fn SetTreeScope(&self, scope: TreeScope) -> windows_core::Result<()>;
    fn TreeFilter(&self) -> windows_core::Result<IUIAutomationCondition>;
    fn SetTreeFilter(&self, filter: windows_core::Ref<IUIAutomationCondition>) -> windows_core::Result<()>;
    fn AutomationElementMode(&self) -> windows_core::Result<AutomationElementMode>;
    fn SetAutomationElementMode(&self, mode: AutomationElementMode) -> windows_core::Result<()>;
}
#[cfg(feature = "uiautomationcore")]
impl IUIAutomationCacheRequest_Vtbl {
    pub const fn new<Identity: IUIAutomationCacheRequest_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddProperty<Identity: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: super::uiautomationcore::PROPERTYID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationCacheRequest_Impl::AddProperty(this, core::mem::transmute_copy(&propertyid)).into()
            }
        }
        unsafe extern "system" fn AddPattern<Identity: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, patternid: super::uiautomationcore::PATTERNID) -> windows_core::HRESULT {
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
        unsafe extern "system" fn TreeScope<Identity: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: *mut TreeScope) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationCacheRequest_Impl::TreeScope(this) {
                    Ok(ok__) => {
                        scope.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTreeScope<Identity: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: TreeScope) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationCacheRequest_Impl::SetTreeScope(this, core::mem::transmute_copy(&scope)).into()
            }
        }
        unsafe extern "system" fn TreeFilter<Identity: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filter: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationCacheRequest_Impl::TreeFilter(this) {
                    Ok(ok__) => {
                        filter.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetTreeFilter<Identity: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, filter: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationCacheRequest_Impl::SetTreeFilter(this, core::mem::transmute_copy(&filter)).into()
            }
        }
        unsafe extern "system" fn AutomationElementMode<Identity: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: *mut AutomationElementMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationCacheRequest_Impl::AutomationElementMode(this) {
                    Ok(ok__) => {
                        mode.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetAutomationElementMode<Identity: IUIAutomationCacheRequest_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, mode: AutomationElementMode) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationCacheRequest_Impl::SetAutomationElementMode(this, core::mem::transmute_copy(&mode)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddProperty: AddProperty::<Identity, OFFSET>,
            AddPattern: AddPattern::<Identity, OFFSET>,
            Clone: Clone::<Identity, OFFSET>,
            TreeScope: TreeScope::<Identity, OFFSET>,
            SetTreeScope: SetTreeScope::<Identity, OFFSET>,
            TreeFilter: TreeFilter::<Identity, OFFSET>,
            SetTreeFilter: SetTreeFilter::<Identity, OFFSET>,
            AutomationElementMode: AutomationElementMode::<Identity, OFFSET>,
            SetAutomationElementMode: SetAutomationElementMode::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationCacheRequest as windows_core::Interface>::IID
    }
}
#[cfg(feature = "uiautomationcore")]
impl windows_core::RuntimeName for IUIAutomationCacheRequest {}
windows_core::imp::define_interface!(IUIAutomationChangesEventHandler, IUIAutomationChangesEventHandler_Vtbl, 0x58edca55_2c3e_4980_b1b9_56c17f27a2a0);
windows_core::imp::interface_hierarchy!(IUIAutomationChangesEventHandler, windows_core::IUnknown);
impl IUIAutomationChangesEventHandler {
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn HandleChangesEvent<P0>(&self, sender: P0, uiachanges: *const super::uiautomationcore::UiaChangeInfo, changescount: i32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe { (windows_core::Interface::vtable(self).HandleChangesEvent)(windows_core::Interface::as_raw(self), sender.param().abi(), core::mem::transmute(uiachanges), changescount) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationChangesEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
    pub HandleChangesEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *const super::uiautomationcore::UiaChangeInfo, i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase")))]
    HandleChangesEvent: usize,
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationChangesEventHandler_Impl: windows_core::IUnknownImpl {
    fn HandleChangesEvent(&self, sender: windows_core::Ref<IUIAutomationElement>, uiachanges: *const super::uiautomationcore::UiaChangeInfo, changescount: i32) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationChangesEventHandler_Vtbl {
    pub const fn new<Identity: IUIAutomationChangesEventHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HandleChangesEvent<Identity: IUIAutomationChangesEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, uiachanges: *const super::uiautomationcore::UiaChangeInfo, changescount: i32) -> windows_core::HRESULT {
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
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn Navigate(&self, direction: super::uiautomationcore::NavigateDirection) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Navigate)(windows_core::Interface::as_raw(self), direction, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationCustomNavigationPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "uiautomationcore")]
    pub Navigate: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::NavigateDirection, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    Navigate: usize,
}
#[cfg(feature = "uiautomationcore")]
pub trait IUIAutomationCustomNavigationPattern_Impl: windows_core::IUnknownImpl {
    fn Navigate(&self, direction: super::uiautomationcore::NavigateDirection) -> windows_core::Result<IUIAutomationElement>;
}
#[cfg(feature = "uiautomationcore")]
impl IUIAutomationCustomNavigationPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationCustomNavigationPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Navigate<Identity: IUIAutomationCustomNavigationPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, direction: super::uiautomationcore::NavigateDirection, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
    pub unsafe fn SetDockPosition(&self, dockpos: super::uiautomationcore::DockPosition) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDockPosition)(windows_core::Interface::as_raw(self), dockpos) }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn CurrentDockPosition(&self) -> windows_core::Result<super::uiautomationcore::DockPosition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentDockPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn CachedDockPosition(&self) -> windows_core::Result<super::uiautomationcore::DockPosition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedDockPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationDockPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "uiautomationcore")]
    pub SetDockPosition: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::DockPosition) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    SetDockPosition: usize,
    #[cfg(feature = "uiautomationcore")]
    pub CurrentDockPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::uiautomationcore::DockPosition) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CurrentDockPosition: usize,
    #[cfg(feature = "uiautomationcore")]
    pub CachedDockPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::uiautomationcore::DockPosition) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CachedDockPosition: usize,
}
#[cfg(feature = "uiautomationcore")]
pub trait IUIAutomationDockPattern_Impl: windows_core::IUnknownImpl {
    fn SetDockPosition(&self, dockpos: super::uiautomationcore::DockPosition) -> windows_core::Result<()>;
    fn CurrentDockPosition(&self) -> windows_core::Result<super::uiautomationcore::DockPosition>;
    fn CachedDockPosition(&self) -> windows_core::Result<super::uiautomationcore::DockPosition>;
}
#[cfg(feature = "uiautomationcore")]
impl IUIAutomationDockPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationDockPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDockPosition<Identity: IUIAutomationDockPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dockpos: super::uiautomationcore::DockPosition) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationDockPattern_Impl::SetDockPosition(this, core::mem::transmute_copy(&dockpos)).into()
            }
        }
        unsafe extern "system" fn CurrentDockPosition<Identity: IUIAutomationDockPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::uiautomationcore::DockPosition) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationDockPattern_Impl::CurrentDockPosition(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedDockPosition<Identity: IUIAutomationDockPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::uiautomationcore::DockPosition) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationDockPattern_Impl::CachedDockPosition(this) {
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
            CurrentDockPosition: CurrentDockPosition::<Identity, OFFSET>,
            CachedDockPosition: CachedDockPosition::<Identity, OFFSET>,
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
    pub unsafe fn CurrentIsGrabbed(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentIsGrabbed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedIsGrabbed(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedIsGrabbed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentDropEffect(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentDropEffect)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedDropEffect(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedDropEffect)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn CurrentDropEffects(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentDropEffects)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn CachedDropEffects(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedDropEffects)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCurrentGrabbedItems(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentGrabbedItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCachedGrabbedItems(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedGrabbedItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationDragPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CurrentIsGrabbed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CachedIsGrabbed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CurrentDropEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedDropEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub CurrentDropEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    CurrentDropEffects: usize,
    #[cfg(feature = "oaidl")]
    pub CachedDropEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    CachedDropEffects: usize,
    pub GetCurrentGrabbedItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCachedGrabbedItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "oaidl")]
pub trait IUIAutomationDragPattern_Impl: windows_core::IUnknownImpl {
    fn CurrentIsGrabbed(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedIsGrabbed(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentDropEffect(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedDropEffect(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentDropEffects(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn CachedDropEffects(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn GetCurrentGrabbedItems(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn GetCachedGrabbedItems(&self) -> windows_core::Result<IUIAutomationElementArray>;
}
#[cfg(feature = "oaidl")]
impl IUIAutomationDragPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationDragPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentIsGrabbed<Identity: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationDragPattern_Impl::CurrentIsGrabbed(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedIsGrabbed<Identity: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationDragPattern_Impl::CachedIsGrabbed(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentDropEffect<Identity: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationDragPattern_Impl::CurrentDropEffect(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedDropEffect<Identity: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationDragPattern_Impl::CachedDropEffect(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentDropEffects<Identity: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationDragPattern_Impl::CurrentDropEffects(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedDropEffects<Identity: IUIAutomationDragPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationDragPattern_Impl::CachedDropEffects(this) {
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
            CurrentIsGrabbed: CurrentIsGrabbed::<Identity, OFFSET>,
            CachedIsGrabbed: CachedIsGrabbed::<Identity, OFFSET>,
            CurrentDropEffect: CurrentDropEffect::<Identity, OFFSET>,
            CachedDropEffect: CachedDropEffect::<Identity, OFFSET>,
            CurrentDropEffects: CurrentDropEffects::<Identity, OFFSET>,
            CachedDropEffects: CachedDropEffects::<Identity, OFFSET>,
            GetCurrentGrabbedItems: GetCurrentGrabbedItems::<Identity, OFFSET>,
            GetCachedGrabbedItems: GetCachedGrabbedItems::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationDragPattern as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IUIAutomationDragPattern {}
windows_core::imp::define_interface!(IUIAutomationDropTargetPattern, IUIAutomationDropTargetPattern_Vtbl, 0x69a095f7_eee4_430e_a46b_fb73b1ae39a5);
windows_core::imp::interface_hierarchy!(IUIAutomationDropTargetPattern, windows_core::IUnknown);
impl IUIAutomationDropTargetPattern {
    pub unsafe fn CurrentDropTargetEffect(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentDropTargetEffect)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedDropTargetEffect(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedDropTargetEffect)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn CurrentDropTargetEffects(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentDropTargetEffects)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn CachedDropTargetEffects(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedDropTargetEffects)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationDropTargetPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CurrentDropTargetEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedDropTargetEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub CurrentDropTargetEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    CurrentDropTargetEffects: usize,
    #[cfg(feature = "oaidl")]
    pub CachedDropTargetEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    CachedDropTargetEffects: usize,
}
#[cfg(feature = "oaidl")]
pub trait IUIAutomationDropTargetPattern_Impl: windows_core::IUnknownImpl {
    fn CurrentDropTargetEffect(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedDropTargetEffect(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentDropTargetEffects(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn CachedDropTargetEffects(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
}
#[cfg(feature = "oaidl")]
impl IUIAutomationDropTargetPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationDropTargetPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentDropTargetEffect<Identity: IUIAutomationDropTargetPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationDropTargetPattern_Impl::CurrentDropTargetEffect(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedDropTargetEffect<Identity: IUIAutomationDropTargetPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationDropTargetPattern_Impl::CachedDropTargetEffect(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentDropTargetEffects<Identity: IUIAutomationDropTargetPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationDropTargetPattern_Impl::CurrentDropTargetEffects(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedDropTargetEffects<Identity: IUIAutomationDropTargetPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationDropTargetPattern_Impl::CachedDropTargetEffects(this) {
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
            CurrentDropTargetEffect: CurrentDropTargetEffect::<Identity, OFFSET>,
            CachedDropTargetEffect: CachedDropTargetEffect::<Identity, OFFSET>,
            CurrentDropTargetEffects: CurrentDropTargetEffects::<Identity, OFFSET>,
            CachedDropTargetEffects: CachedDropTargetEffects::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationDropTargetPattern as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IUIAutomationDropTargetPattern {}
windows_core::imp::define_interface!(IUIAutomationElement, IUIAutomationElement_Vtbl, 0xd22108aa_8ac5_49a5_837b_37bbb3d7591e);
windows_core::imp::interface_hierarchy!(IUIAutomationElement, windows_core::IUnknown);
impl IUIAutomationElement {
    pub unsafe fn SetFocus(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFocus)(windows_core::Interface::as_raw(self)) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetRuntimeId(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
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
            (windows_core::Interface::vtable(self).FindFirst)(windows_core::Interface::as_raw(self), scope, condition.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindAll<P1>(&self, scope: TreeScope, condition: P1) -> windows_core::Result<IUIAutomationElementArray>
    where
        P1: windows_core::Param<IUIAutomationCondition>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindAll)(windows_core::Interface::as_raw(self), scope, condition.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindFirstBuildCache<P1, P2>(&self, scope: TreeScope, condition: P1, cacherequest: P2) -> windows_core::Result<Self>
    where
        P1: windows_core::Param<IUIAutomationCondition>,
        P2: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindFirstBuildCache)(windows_core::Interface::as_raw(self), scope, condition.param().abi(), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindAllBuildCache<P1, P2>(&self, scope: TreeScope, condition: P1, cacherequest: P2) -> windows_core::Result<IUIAutomationElementArray>
    where
        P1: windows_core::Param<IUIAutomationCondition>,
        P2: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindAllBuildCache)(windows_core::Interface::as_raw(self), scope, condition.param().abi(), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn BuildUpdatedCache<P0>(&self, cacherequest: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).BuildUpdatedCache)(windows_core::Interface::as_raw(self), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetCurrentPropertyValue(&self, propertyid: super::uiautomationcore::PROPERTYID) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentPropertyValue)(windows_core::Interface::as_raw(self), propertyid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetCurrentPropertyValueEx(&self, propertyid: super::uiautomationcore::PROPERTYID, ignoredefaultvalue: bool) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentPropertyValueEx)(windows_core::Interface::as_raw(self), propertyid, ignoredefaultvalue.into(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetCachedPropertyValue(&self, propertyid: super::uiautomationcore::PROPERTYID) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedPropertyValue)(windows_core::Interface::as_raw(self), propertyid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetCachedPropertyValueEx(&self, propertyid: super::uiautomationcore::PROPERTYID, ignoredefaultvalue: bool) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedPropertyValueEx)(windows_core::Interface::as_raw(self), propertyid, ignoredefaultvalue.into(), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn GetCurrentPatternAs<T>(&self, patternid: super::uiautomationcore::PATTERNID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetCurrentPatternAs)(windows_core::Interface::as_raw(self), patternid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn GetCachedPatternAs<T>(&self, patternid: super::uiautomationcore::PATTERNID) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe { (windows_core::Interface::vtable(self).GetCachedPatternAs)(windows_core::Interface::as_raw(self), patternid, &T::IID, &mut result__).and_then(|| windows_core::Type::from_abi(result__)) }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn GetCurrentPattern(&self, patternid: super::uiautomationcore::PATTERNID) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentPattern)(windows_core::Interface::as_raw(self), patternid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn GetCachedPattern(&self, patternid: super::uiautomationcore::PATTERNID) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedPattern)(windows_core::Interface::as_raw(self), patternid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCachedParent(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedParent)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCachedChildren(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedChildren)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CurrentProcessId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentProcessId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn CurrentControlType(&self) -> windows_core::Result<super::uiautomationcore::CONTROLTYPEID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentControlType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentLocalizedControlType(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentLocalizedControlType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentAcceleratorKey(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentAcceleratorKey)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentAccessKey(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentAccessKey)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentHasKeyboardFocus(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentHasKeyboardFocus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentIsKeyboardFocusable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentIsKeyboardFocusable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentIsEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentIsEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentAutomationId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentAutomationId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentClassName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentClassName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentHelpText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentHelpText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentCulture(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentCulture)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentIsControlElement(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentIsControlElement)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentIsContentElement(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentIsContentElement)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentIsPassword(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentIsPassword)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentNativeWindowHandle(&self) -> windows_core::Result<UIA_HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentNativeWindowHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentItemType(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentItemType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentIsOffscreen(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentIsOffscreen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn CurrentOrientation(&self) -> windows_core::Result<super::uiautomationcore::OrientationType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentOrientation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentFrameworkId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentFrameworkId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentIsRequiredForForm(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentIsRequiredForForm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentItemStatus(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentItemStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn CurrentBoundingRectangle(&self) -> windows_core::Result<super::windef::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentBoundingRectangle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentLabeledBy(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentLabeledBy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CurrentAriaRole(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentAriaRole)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentAriaProperties(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentAriaProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentIsDataValidForForm(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentIsDataValidForForm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentControllerFor(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentControllerFor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CurrentDescribedBy(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentDescribedBy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CurrentFlowsTo(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentFlowsTo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CurrentProviderDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentProviderDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedProcessId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedProcessId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn CachedControlType(&self) -> windows_core::Result<super::uiautomationcore::CONTROLTYPEID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedControlType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedLocalizedControlType(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedLocalizedControlType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedAcceleratorKey(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedAcceleratorKey)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedAccessKey(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedAccessKey)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedHasKeyboardFocus(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedHasKeyboardFocus)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedIsKeyboardFocusable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedIsKeyboardFocusable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedIsEnabled(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedIsEnabled)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedAutomationId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedAutomationId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedClassName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedClassName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedHelpText(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedHelpText)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedCulture(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedCulture)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedIsControlElement(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedIsControlElement)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedIsContentElement(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedIsContentElement)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedIsPassword(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedIsPassword)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedNativeWindowHandle(&self) -> windows_core::Result<UIA_HWND> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedNativeWindowHandle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedItemType(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedItemType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedIsOffscreen(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedIsOffscreen)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn CachedOrientation(&self) -> windows_core::Result<super::uiautomationcore::OrientationType> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedOrientation)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedFrameworkId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedFrameworkId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedIsRequiredForForm(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedIsRequiredForForm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedItemStatus(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedItemStatus)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn CachedBoundingRectangle(&self) -> windows_core::Result<super::windef::RECT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedBoundingRectangle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedLabeledBy(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedLabeledBy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CachedAriaRole(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedAriaRole)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedAriaProperties(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedAriaProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedIsDataValidForForm(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedIsDataValidForForm)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedControllerFor(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedControllerFor)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CachedDescribedBy(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedDescribedBy)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CachedFlowsTo(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedFlowsTo)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CachedProviderDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedProviderDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "windef")]
    pub unsafe fn GetClickablePoint(&self, clickable: *mut super::windef::POINT) -> windows_core::Result<windows_core::BOOL> {
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
    #[cfg(feature = "oaidl")]
    pub GetRuntimeId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetRuntimeId: usize,
    pub FindFirst: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindAll: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindFirstBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FindAllBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub BuildUpdatedCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
    pub GetCurrentPropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::PROPERTYID, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase")))]
    GetCurrentPropertyValue: usize,
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
    pub GetCurrentPropertyValueEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::PROPERTYID, windows_core::BOOL, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase")))]
    GetCurrentPropertyValueEx: usize,
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
    pub GetCachedPropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::PROPERTYID, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase")))]
    GetCachedPropertyValue: usize,
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
    pub GetCachedPropertyValueEx: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::PROPERTYID, windows_core::BOOL, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase")))]
    GetCachedPropertyValueEx: usize,
    #[cfg(feature = "uiautomationcore")]
    pub GetCurrentPatternAs: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::PATTERNID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    GetCurrentPatternAs: usize,
    #[cfg(feature = "uiautomationcore")]
    pub GetCachedPatternAs: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::PATTERNID, *const windows_core::GUID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    GetCachedPatternAs: usize,
    #[cfg(feature = "uiautomationcore")]
    pub GetCurrentPattern: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::PATTERNID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    GetCurrentPattern: usize,
    #[cfg(feature = "uiautomationcore")]
    pub GetCachedPattern: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::PATTERNID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    GetCachedPattern: usize,
    pub GetCachedParent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCachedChildren: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentProcessId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub CurrentControlType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::uiautomationcore::CONTROLTYPEID) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CurrentControlType: usize,
    pub CurrentLocalizedControlType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentAcceleratorKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentAccessKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentHasKeyboardFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CurrentIsKeyboardFocusable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CurrentIsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CurrentAutomationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentClassName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentHelpText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentCulture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentIsControlElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CurrentIsContentElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CurrentIsPassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CurrentNativeWindowHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UIA_HWND) -> windows_core::HRESULT,
    pub CurrentItemType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentIsOffscreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub CurrentOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::uiautomationcore::OrientationType) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CurrentOrientation: usize,
    pub CurrentFrameworkId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentIsRequiredForForm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CurrentItemStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub CurrentBoundingRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    CurrentBoundingRectangle: usize,
    pub CurrentLabeledBy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentAriaRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentAriaProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentIsDataValidForForm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CurrentControllerFor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentDescribedBy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentFlowsTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentProviderDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedProcessId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub CachedControlType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::uiautomationcore::CONTROLTYPEID) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CachedControlType: usize,
    pub CachedLocalizedControlType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedAcceleratorKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedAccessKey: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedHasKeyboardFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CachedIsKeyboardFocusable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CachedIsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CachedAutomationId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedClassName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedHelpText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedCulture: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedIsControlElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CachedIsContentElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CachedIsPassword: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CachedNativeWindowHandle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UIA_HWND) -> windows_core::HRESULT,
    pub CachedItemType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedIsOffscreen: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub CachedOrientation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::uiautomationcore::OrientationType) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CachedOrientation: usize,
    pub CachedFrameworkId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedIsRequiredForForm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CachedItemStatus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub CachedBoundingRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::RECT) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    CachedBoundingRectangle: usize,
    pub CachedLabeledBy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedAriaRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedAriaProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedIsDataValidForForm: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CachedControllerFor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedDescribedBy: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedFlowsTo: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedProviderDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "windef")]
    pub GetClickablePoint: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::windef::POINT, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetClickablePoint: usize,
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationElement_Impl: windows_core::IUnknownImpl {
    fn SetFocus(&self) -> windows_core::Result<()>;
    fn GetRuntimeId(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn FindFirst(&self, scope: TreeScope, condition: windows_core::Ref<IUIAutomationCondition>) -> windows_core::Result<IUIAutomationElement>;
    fn FindAll(&self, scope: TreeScope, condition: windows_core::Ref<IUIAutomationCondition>) -> windows_core::Result<IUIAutomationElementArray>;
    fn FindFirstBuildCache(&self, scope: TreeScope, condition: windows_core::Ref<IUIAutomationCondition>, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>) -> windows_core::Result<IUIAutomationElement>;
    fn FindAllBuildCache(&self, scope: TreeScope, condition: windows_core::Ref<IUIAutomationCondition>, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>) -> windows_core::Result<IUIAutomationElementArray>;
    fn BuildUpdatedCache(&self, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>) -> windows_core::Result<IUIAutomationElement>;
    fn GetCurrentPropertyValue(&self, propertyid: super::uiautomationcore::PROPERTYID) -> windows_core::Result<super::oaidl::VARIANT>;
    fn GetCurrentPropertyValueEx(&self, propertyid: super::uiautomationcore::PROPERTYID, ignoredefaultvalue: windows_core::BOOL) -> windows_core::Result<super::oaidl::VARIANT>;
    fn GetCachedPropertyValue(&self, propertyid: super::uiautomationcore::PROPERTYID) -> windows_core::Result<super::oaidl::VARIANT>;
    fn GetCachedPropertyValueEx(&self, propertyid: super::uiautomationcore::PROPERTYID, ignoredefaultvalue: windows_core::BOOL) -> windows_core::Result<super::oaidl::VARIANT>;
    fn GetCurrentPatternAs(&self, patternid: super::uiautomationcore::PATTERNID, riid: *const windows_core::GUID, patternobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetCachedPatternAs(&self, patternid: super::uiautomationcore::PATTERNID, riid: *const windows_core::GUID, patternobject: *mut *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn GetCurrentPattern(&self, patternid: super::uiautomationcore::PATTERNID) -> windows_core::Result<windows_core::IUnknown>;
    fn GetCachedPattern(&self, patternid: super::uiautomationcore::PATTERNID) -> windows_core::Result<windows_core::IUnknown>;
    fn GetCachedParent(&self) -> windows_core::Result<IUIAutomationElement>;
    fn GetCachedChildren(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn CurrentProcessId(&self) -> windows_core::Result<i32>;
    fn CurrentControlType(&self) -> windows_core::Result<super::uiautomationcore::CONTROLTYPEID>;
    fn CurrentLocalizedControlType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentAcceleratorKey(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentAccessKey(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentHasKeyboardFocus(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentIsKeyboardFocusable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentIsEnabled(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentAutomationId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentClassName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentHelpText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentCulture(&self) -> windows_core::Result<i32>;
    fn CurrentIsControlElement(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentIsContentElement(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentIsPassword(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentNativeWindowHandle(&self) -> windows_core::Result<UIA_HWND>;
    fn CurrentItemType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentIsOffscreen(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentOrientation(&self) -> windows_core::Result<super::uiautomationcore::OrientationType>;
    fn CurrentFrameworkId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentIsRequiredForForm(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentItemStatus(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentBoundingRectangle(&self) -> windows_core::Result<super::windef::RECT>;
    fn CurrentLabeledBy(&self) -> windows_core::Result<IUIAutomationElement>;
    fn CurrentAriaRole(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentAriaProperties(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentIsDataValidForForm(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentControllerFor(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn CurrentDescribedBy(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn CurrentFlowsTo(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn CurrentProviderDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedProcessId(&self) -> windows_core::Result<i32>;
    fn CachedControlType(&self) -> windows_core::Result<super::uiautomationcore::CONTROLTYPEID>;
    fn CachedLocalizedControlType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedAcceleratorKey(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedAccessKey(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedHasKeyboardFocus(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedIsKeyboardFocusable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedIsEnabled(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedAutomationId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedClassName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedHelpText(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedCulture(&self) -> windows_core::Result<i32>;
    fn CachedIsControlElement(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedIsContentElement(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedIsPassword(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedNativeWindowHandle(&self) -> windows_core::Result<UIA_HWND>;
    fn CachedItemType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedIsOffscreen(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedOrientation(&self) -> windows_core::Result<super::uiautomationcore::OrientationType>;
    fn CachedFrameworkId(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedIsRequiredForForm(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedItemStatus(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedBoundingRectangle(&self) -> windows_core::Result<super::windef::RECT>;
    fn CachedLabeledBy(&self) -> windows_core::Result<IUIAutomationElement>;
    fn CachedAriaRole(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedAriaProperties(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedIsDataValidForForm(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedControllerFor(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn CachedDescribedBy(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn CachedFlowsTo(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn CachedProviderDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetClickablePoint(&self, clickable: *mut super::windef::POINT) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationElement_Vtbl {
    pub const fn new<Identity: IUIAutomationElement_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetFocus<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationElement_Impl::SetFocus(this).into()
            }
        }
        unsafe extern "system" fn GetRuntimeId<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, runtimeid: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetCurrentPropertyValue<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: super::uiautomationcore::PROPERTYID, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetCurrentPropertyValueEx<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: super::uiautomationcore::PROPERTYID, ignoredefaultvalue: windows_core::BOOL, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetCachedPropertyValue<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: super::uiautomationcore::PROPERTYID, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetCachedPropertyValueEx<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: super::uiautomationcore::PROPERTYID, ignoredefaultvalue: windows_core::BOOL, retval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetCurrentPatternAs<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, patternid: super::uiautomationcore::PATTERNID, riid: *const windows_core::GUID, patternobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationElement_Impl::GetCurrentPatternAs(this, core::mem::transmute_copy(&patternid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&patternobject)).into()
            }
        }
        unsafe extern "system" fn GetCachedPatternAs<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, patternid: super::uiautomationcore::PATTERNID, riid: *const windows_core::GUID, patternobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationElement_Impl::GetCachedPatternAs(this, core::mem::transmute_copy(&patternid), core::mem::transmute_copy(&riid), core::mem::transmute_copy(&patternobject)).into()
            }
        }
        unsafe extern "system" fn GetCurrentPattern<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, patternid: super::uiautomationcore::PATTERNID, patternobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetCachedPattern<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, patternid: super::uiautomationcore::PATTERNID, patternobject: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn CurrentProcessId<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentProcessId(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentControlType<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::uiautomationcore::CONTROLTYPEID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentControlType(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentLocalizedControlType<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentLocalizedControlType(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentName<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentAcceleratorKey<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentAcceleratorKey(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentAccessKey<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentAccessKey(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentHasKeyboardFocus<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentHasKeyboardFocus(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentIsKeyboardFocusable<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentIsKeyboardFocusable(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentIsEnabled<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentIsEnabled(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentAutomationId<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentAutomationId(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentClassName<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentClassName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentHelpText<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentHelpText(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentCulture<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentCulture(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentIsControlElement<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentIsControlElement(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentIsContentElement<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentIsContentElement(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentIsPassword<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentIsPassword(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentNativeWindowHandle<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut UIA_HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentNativeWindowHandle(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentItemType<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentItemType(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentIsOffscreen<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentIsOffscreen(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentOrientation<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::uiautomationcore::OrientationType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentOrientation(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentFrameworkId<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentFrameworkId(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentIsRequiredForForm<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentIsRequiredForForm(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentItemStatus<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentItemStatus(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentBoundingRectangle<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentBoundingRectangle(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentLabeledBy<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentLabeledBy(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentAriaRole<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentAriaRole(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentAriaProperties<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentAriaProperties(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentIsDataValidForForm<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentIsDataValidForForm(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentControllerFor<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentControllerFor(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentDescribedBy<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentDescribedBy(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentFlowsTo<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentFlowsTo(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentProviderDescription<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CurrentProviderDescription(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedProcessId<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedProcessId(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedControlType<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::uiautomationcore::CONTROLTYPEID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedControlType(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedLocalizedControlType<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedLocalizedControlType(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedName<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedAcceleratorKey<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedAcceleratorKey(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedAccessKey<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedAccessKey(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedHasKeyboardFocus<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedHasKeyboardFocus(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedIsKeyboardFocusable<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedIsKeyboardFocusable(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedIsEnabled<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedIsEnabled(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedAutomationId<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedAutomationId(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedClassName<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedClassName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedHelpText<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedHelpText(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedCulture<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedCulture(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedIsControlElement<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedIsControlElement(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedIsContentElement<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedIsContentElement(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedIsPassword<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedIsPassword(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedNativeWindowHandle<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut UIA_HWND) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedNativeWindowHandle(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedItemType<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedItemType(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedIsOffscreen<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedIsOffscreen(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedOrientation<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::uiautomationcore::OrientationType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedOrientation(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedFrameworkId<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedFrameworkId(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedIsRequiredForForm<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedIsRequiredForForm(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedItemStatus<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedItemStatus(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedBoundingRectangle<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::windef::RECT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedBoundingRectangle(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedLabeledBy<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedLabeledBy(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedAriaRole<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedAriaRole(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedAriaProperties<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedAriaProperties(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedIsDataValidForForm<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedIsDataValidForForm(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedControllerFor<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedControllerFor(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedDescribedBy<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedDescribedBy(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedFlowsTo<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedFlowsTo(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedProviderDescription<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement_Impl::CachedProviderDescription(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetClickablePoint<Identity: IUIAutomationElement_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clickable: *mut super::windef::POINT, gotclickable: *mut windows_core::BOOL) -> windows_core::HRESULT {
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
            CurrentProcessId: CurrentProcessId::<Identity, OFFSET>,
            CurrentControlType: CurrentControlType::<Identity, OFFSET>,
            CurrentLocalizedControlType: CurrentLocalizedControlType::<Identity, OFFSET>,
            CurrentName: CurrentName::<Identity, OFFSET>,
            CurrentAcceleratorKey: CurrentAcceleratorKey::<Identity, OFFSET>,
            CurrentAccessKey: CurrentAccessKey::<Identity, OFFSET>,
            CurrentHasKeyboardFocus: CurrentHasKeyboardFocus::<Identity, OFFSET>,
            CurrentIsKeyboardFocusable: CurrentIsKeyboardFocusable::<Identity, OFFSET>,
            CurrentIsEnabled: CurrentIsEnabled::<Identity, OFFSET>,
            CurrentAutomationId: CurrentAutomationId::<Identity, OFFSET>,
            CurrentClassName: CurrentClassName::<Identity, OFFSET>,
            CurrentHelpText: CurrentHelpText::<Identity, OFFSET>,
            CurrentCulture: CurrentCulture::<Identity, OFFSET>,
            CurrentIsControlElement: CurrentIsControlElement::<Identity, OFFSET>,
            CurrentIsContentElement: CurrentIsContentElement::<Identity, OFFSET>,
            CurrentIsPassword: CurrentIsPassword::<Identity, OFFSET>,
            CurrentNativeWindowHandle: CurrentNativeWindowHandle::<Identity, OFFSET>,
            CurrentItemType: CurrentItemType::<Identity, OFFSET>,
            CurrentIsOffscreen: CurrentIsOffscreen::<Identity, OFFSET>,
            CurrentOrientation: CurrentOrientation::<Identity, OFFSET>,
            CurrentFrameworkId: CurrentFrameworkId::<Identity, OFFSET>,
            CurrentIsRequiredForForm: CurrentIsRequiredForForm::<Identity, OFFSET>,
            CurrentItemStatus: CurrentItemStatus::<Identity, OFFSET>,
            CurrentBoundingRectangle: CurrentBoundingRectangle::<Identity, OFFSET>,
            CurrentLabeledBy: CurrentLabeledBy::<Identity, OFFSET>,
            CurrentAriaRole: CurrentAriaRole::<Identity, OFFSET>,
            CurrentAriaProperties: CurrentAriaProperties::<Identity, OFFSET>,
            CurrentIsDataValidForForm: CurrentIsDataValidForForm::<Identity, OFFSET>,
            CurrentControllerFor: CurrentControllerFor::<Identity, OFFSET>,
            CurrentDescribedBy: CurrentDescribedBy::<Identity, OFFSET>,
            CurrentFlowsTo: CurrentFlowsTo::<Identity, OFFSET>,
            CurrentProviderDescription: CurrentProviderDescription::<Identity, OFFSET>,
            CachedProcessId: CachedProcessId::<Identity, OFFSET>,
            CachedControlType: CachedControlType::<Identity, OFFSET>,
            CachedLocalizedControlType: CachedLocalizedControlType::<Identity, OFFSET>,
            CachedName: CachedName::<Identity, OFFSET>,
            CachedAcceleratorKey: CachedAcceleratorKey::<Identity, OFFSET>,
            CachedAccessKey: CachedAccessKey::<Identity, OFFSET>,
            CachedHasKeyboardFocus: CachedHasKeyboardFocus::<Identity, OFFSET>,
            CachedIsKeyboardFocusable: CachedIsKeyboardFocusable::<Identity, OFFSET>,
            CachedIsEnabled: CachedIsEnabled::<Identity, OFFSET>,
            CachedAutomationId: CachedAutomationId::<Identity, OFFSET>,
            CachedClassName: CachedClassName::<Identity, OFFSET>,
            CachedHelpText: CachedHelpText::<Identity, OFFSET>,
            CachedCulture: CachedCulture::<Identity, OFFSET>,
            CachedIsControlElement: CachedIsControlElement::<Identity, OFFSET>,
            CachedIsContentElement: CachedIsContentElement::<Identity, OFFSET>,
            CachedIsPassword: CachedIsPassword::<Identity, OFFSET>,
            CachedNativeWindowHandle: CachedNativeWindowHandle::<Identity, OFFSET>,
            CachedItemType: CachedItemType::<Identity, OFFSET>,
            CachedIsOffscreen: CachedIsOffscreen::<Identity, OFFSET>,
            CachedOrientation: CachedOrientation::<Identity, OFFSET>,
            CachedFrameworkId: CachedFrameworkId::<Identity, OFFSET>,
            CachedIsRequiredForForm: CachedIsRequiredForForm::<Identity, OFFSET>,
            CachedItemStatus: CachedItemStatus::<Identity, OFFSET>,
            CachedBoundingRectangle: CachedBoundingRectangle::<Identity, OFFSET>,
            CachedLabeledBy: CachedLabeledBy::<Identity, OFFSET>,
            CachedAriaRole: CachedAriaRole::<Identity, OFFSET>,
            CachedAriaProperties: CachedAriaProperties::<Identity, OFFSET>,
            CachedIsDataValidForForm: CachedIsDataValidForForm::<Identity, OFFSET>,
            CachedControllerFor: CachedControllerFor::<Identity, OFFSET>,
            CachedDescribedBy: CachedDescribedBy::<Identity, OFFSET>,
            CachedFlowsTo: CachedFlowsTo::<Identity, OFFSET>,
            CachedProviderDescription: CachedProviderDescription::<Identity, OFFSET>,
            GetClickablePoint: GetClickablePoint::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn CurrentOptimizeForVisualContent(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentOptimizeForVisualContent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedOptimizeForVisualContent(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedOptimizeForVisualContent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn CurrentLiveSetting(&self) -> windows_core::Result<super::uiautomationcore::LiveSetting> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentLiveSetting)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn CachedLiveSetting(&self) -> windows_core::Result<super::uiautomationcore::LiveSetting> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedLiveSetting)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentFlowsFrom(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentFlowsFrom)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CachedFlowsFrom(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedFlowsFrom)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElement2_Vtbl {
    pub base__: IUIAutomationElement_Vtbl,
    pub CurrentOptimizeForVisualContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CachedOptimizeForVisualContent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub CurrentLiveSetting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::uiautomationcore::LiveSetting) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CurrentLiveSetting: usize,
    #[cfg(feature = "uiautomationcore")]
    pub CachedLiveSetting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::uiautomationcore::LiveSetting) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CachedLiveSetting: usize,
    pub CurrentFlowsFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedFlowsFrom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationElement2_Impl: IUIAutomationElement_Impl {
    fn CurrentOptimizeForVisualContent(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedOptimizeForVisualContent(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentLiveSetting(&self) -> windows_core::Result<super::uiautomationcore::LiveSetting>;
    fn CachedLiveSetting(&self) -> windows_core::Result<super::uiautomationcore::LiveSetting>;
    fn CurrentFlowsFrom(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn CachedFlowsFrom(&self) -> windows_core::Result<IUIAutomationElementArray>;
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationElement2_Vtbl {
    pub const fn new<Identity: IUIAutomationElement2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentOptimizeForVisualContent<Identity: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement2_Impl::CurrentOptimizeForVisualContent(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedOptimizeForVisualContent<Identity: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement2_Impl::CachedOptimizeForVisualContent(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentLiveSetting<Identity: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::uiautomationcore::LiveSetting) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement2_Impl::CurrentLiveSetting(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedLiveSetting<Identity: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::uiautomationcore::LiveSetting) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement2_Impl::CachedLiveSetting(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentFlowsFrom<Identity: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement2_Impl::CurrentFlowsFrom(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedFlowsFrom<Identity: IUIAutomationElement2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement2_Impl::CachedFlowsFrom(this) {
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
            CurrentOptimizeForVisualContent: CurrentOptimizeForVisualContent::<Identity, OFFSET>,
            CachedOptimizeForVisualContent: CachedOptimizeForVisualContent::<Identity, OFFSET>,
            CurrentLiveSetting: CurrentLiveSetting::<Identity, OFFSET>,
            CachedLiveSetting: CachedLiveSetting::<Identity, OFFSET>,
            CurrentFlowsFrom: CurrentFlowsFrom::<Identity, OFFSET>,
            CachedFlowsFrom: CachedFlowsFrom::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement2 as windows_core::Interface>::IID || iid == &<IUIAutomationElement as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn CurrentIsPeripheral(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentIsPeripheral)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedIsPeripheral(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedIsPeripheral)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElement3_Vtbl {
    pub base__: IUIAutomationElement2_Vtbl,
    pub ShowContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentIsPeripheral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CachedIsPeripheral: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationElement3_Impl: IUIAutomationElement2_Impl {
    fn ShowContextMenu(&self) -> windows_core::Result<()>;
    fn CurrentIsPeripheral(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedIsPeripheral(&self) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationElement3_Vtbl {
    pub const fn new<Identity: IUIAutomationElement3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ShowContextMenu<Identity: IUIAutomationElement3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationElement3_Impl::ShowContextMenu(this).into()
            }
        }
        unsafe extern "system" fn CurrentIsPeripheral<Identity: IUIAutomationElement3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement3_Impl::CurrentIsPeripheral(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedIsPeripheral<Identity: IUIAutomationElement3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement3_Impl::CachedIsPeripheral(this) {
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
            CurrentIsPeripheral: CurrentIsPeripheral::<Identity, OFFSET>,
            CachedIsPeripheral: CachedIsPeripheral::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement3 as windows_core::Interface>::IID || iid == &<IUIAutomationElement as windows_core::Interface>::IID || iid == &<IUIAutomationElement2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn CurrentPositionInSet(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentPositionInSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentSizeOfSet(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentSizeOfSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentLevel(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn CurrentAnnotationTypes(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentAnnotationTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentAnnotationObjects(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentAnnotationObjects)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CachedPositionInSet(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedPositionInSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedSizeOfSet(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedSizeOfSet)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedLevel(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn CachedAnnotationTypes(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedAnnotationTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedAnnotationObjects(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedAnnotationObjects)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElement4_Vtbl {
    pub base__: IUIAutomationElement3_Vtbl,
    pub CurrentPositionInSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentSizeOfSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub CurrentAnnotationTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    CurrentAnnotationTypes: usize,
    pub CurrentAnnotationObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedPositionInSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedSizeOfSet: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub CachedAnnotationTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    CachedAnnotationTypes: usize,
    pub CachedAnnotationObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationElement4_Impl: IUIAutomationElement3_Impl {
    fn CurrentPositionInSet(&self) -> windows_core::Result<i32>;
    fn CurrentSizeOfSet(&self) -> windows_core::Result<i32>;
    fn CurrentLevel(&self) -> windows_core::Result<i32>;
    fn CurrentAnnotationTypes(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn CurrentAnnotationObjects(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn CachedPositionInSet(&self) -> windows_core::Result<i32>;
    fn CachedSizeOfSet(&self) -> windows_core::Result<i32>;
    fn CachedLevel(&self) -> windows_core::Result<i32>;
    fn CachedAnnotationTypes(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn CachedAnnotationObjects(&self) -> windows_core::Result<IUIAutomationElementArray>;
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationElement4_Vtbl {
    pub const fn new<Identity: IUIAutomationElement4_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentPositionInSet<Identity: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement4_Impl::CurrentPositionInSet(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentSizeOfSet<Identity: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement4_Impl::CurrentSizeOfSet(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentLevel<Identity: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement4_Impl::CurrentLevel(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentAnnotationTypes<Identity: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement4_Impl::CurrentAnnotationTypes(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentAnnotationObjects<Identity: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement4_Impl::CurrentAnnotationObjects(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedPositionInSet<Identity: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement4_Impl::CachedPositionInSet(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedSizeOfSet<Identity: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement4_Impl::CachedSizeOfSet(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedLevel<Identity: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement4_Impl::CachedLevel(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedAnnotationTypes<Identity: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement4_Impl::CachedAnnotationTypes(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedAnnotationObjects<Identity: IUIAutomationElement4_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement4_Impl::CachedAnnotationObjects(this) {
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
            CurrentPositionInSet: CurrentPositionInSet::<Identity, OFFSET>,
            CurrentSizeOfSet: CurrentSizeOfSet::<Identity, OFFSET>,
            CurrentLevel: CurrentLevel::<Identity, OFFSET>,
            CurrentAnnotationTypes: CurrentAnnotationTypes::<Identity, OFFSET>,
            CurrentAnnotationObjects: CurrentAnnotationObjects::<Identity, OFFSET>,
            CachedPositionInSet: CachedPositionInSet::<Identity, OFFSET>,
            CachedSizeOfSet: CachedSizeOfSet::<Identity, OFFSET>,
            CachedLevel: CachedLevel::<Identity, OFFSET>,
            CachedAnnotationTypes: CachedAnnotationTypes::<Identity, OFFSET>,
            CachedAnnotationObjects: CachedAnnotationObjects::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement4 as windows_core::Interface>::IID || iid == &<IUIAutomationElement as windows_core::Interface>::IID || iid == &<IUIAutomationElement2 as windows_core::Interface>::IID || iid == &<IUIAutomationElement3 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn CurrentLandmarkType(&self) -> windows_core::Result<super::uiautomationcore::LANDMARKTYPEID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentLandmarkType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentLocalizedLandmarkType(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentLocalizedLandmarkType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn CachedLandmarkType(&self) -> windows_core::Result<super::uiautomationcore::LANDMARKTYPEID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedLandmarkType)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedLocalizedLandmarkType(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedLocalizedLandmarkType)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElement5_Vtbl {
    pub base__: IUIAutomationElement4_Vtbl,
    #[cfg(feature = "uiautomationcore")]
    pub CurrentLandmarkType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::uiautomationcore::LANDMARKTYPEID) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CurrentLandmarkType: usize,
    pub CurrentLocalizedLandmarkType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub CachedLandmarkType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::uiautomationcore::LANDMARKTYPEID) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CachedLandmarkType: usize,
    pub CachedLocalizedLandmarkType: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationElement5_Impl: IUIAutomationElement4_Impl {
    fn CurrentLandmarkType(&self) -> windows_core::Result<super::uiautomationcore::LANDMARKTYPEID>;
    fn CurrentLocalizedLandmarkType(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedLandmarkType(&self) -> windows_core::Result<super::uiautomationcore::LANDMARKTYPEID>;
    fn CachedLocalizedLandmarkType(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationElement5_Vtbl {
    pub const fn new<Identity: IUIAutomationElement5_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentLandmarkType<Identity: IUIAutomationElement5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::uiautomationcore::LANDMARKTYPEID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement5_Impl::CurrentLandmarkType(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentLocalizedLandmarkType<Identity: IUIAutomationElement5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement5_Impl::CurrentLocalizedLandmarkType(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedLandmarkType<Identity: IUIAutomationElement5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::uiautomationcore::LANDMARKTYPEID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement5_Impl::CachedLandmarkType(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedLocalizedLandmarkType<Identity: IUIAutomationElement5_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement5_Impl::CachedLocalizedLandmarkType(this) {
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
            CurrentLandmarkType: CurrentLandmarkType::<Identity, OFFSET>,
            CurrentLocalizedLandmarkType: CurrentLocalizedLandmarkType::<Identity, OFFSET>,
            CachedLandmarkType: CachedLandmarkType::<Identity, OFFSET>,
            CachedLocalizedLandmarkType: CachedLocalizedLandmarkType::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement5 as windows_core::Interface>::IID || iid == &<IUIAutomationElement as windows_core::Interface>::IID || iid == &<IUIAutomationElement2 as windows_core::Interface>::IID || iid == &<IUIAutomationElement3 as windows_core::Interface>::IID || iid == &<IUIAutomationElement4 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn CurrentFullDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentFullDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedFullDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedFullDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElement6_Vtbl {
    pub base__: IUIAutomationElement5_Vtbl,
    pub CurrentFullDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedFullDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationElement6_Impl: IUIAutomationElement5_Impl {
    fn CurrentFullDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedFullDescription(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationElement6_Vtbl {
    pub const fn new<Identity: IUIAutomationElement6_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentFullDescription<Identity: IUIAutomationElement6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement6_Impl::CurrentFullDescription(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedFullDescription<Identity: IUIAutomationElement6_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement6_Impl::CachedFullDescription(this) {
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
            CurrentFullDescription: CurrentFullDescription::<Identity, OFFSET>,
            CachedFullDescription: CachedFullDescription::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement6 as windows_core::Interface>::IID || iid == &<IUIAutomationElement as windows_core::Interface>::IID || iid == &<IUIAutomationElement2 as windows_core::Interface>::IID || iid == &<IUIAutomationElement3 as windows_core::Interface>::IID || iid == &<IUIAutomationElement4 as windows_core::Interface>::IID || iid == &<IUIAutomationElement5 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
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
            (windows_core::Interface::vtable(self).FindFirstWithOptions)(windows_core::Interface::as_raw(self), scope, condition.param().abi(), traversaloptions, root.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindAllWithOptions<P1, P3>(&self, scope: TreeScope, condition: P1, traversaloptions: TreeTraversalOptions, root: P3) -> windows_core::Result<IUIAutomationElementArray>
    where
        P1: windows_core::Param<IUIAutomationCondition>,
        P3: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindAllWithOptions)(windows_core::Interface::as_raw(self), scope, condition.param().abi(), traversaloptions, root.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
            (windows_core::Interface::vtable(self).FindFirstWithOptionsBuildCache)(windows_core::Interface::as_raw(self), scope, condition.param().abi(), cacherequest.param().abi(), traversaloptions, root.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
            (windows_core::Interface::vtable(self).FindAllWithOptionsBuildCache)(windows_core::Interface::as_raw(self), scope, condition.param().abi(), cacherequest.param().abi(), traversaloptions, root.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetCurrentMetadataValue(&self, targetid: i32, metadataid: super::uiautomationcore::METADATAID) -> windows_core::Result<super::oaidl::VARIANT> {
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
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
    pub GetCurrentMetadataValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, super::uiautomationcore::METADATAID, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase")))]
    GetCurrentMetadataValue: usize,
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationElement7_Impl: IUIAutomationElement6_Impl {
    fn FindFirstWithOptions(&self, scope: TreeScope, condition: windows_core::Ref<IUIAutomationCondition>, traversaloptions: TreeTraversalOptions, root: windows_core::Ref<IUIAutomationElement>) -> windows_core::Result<IUIAutomationElement>;
    fn FindAllWithOptions(&self, scope: TreeScope, condition: windows_core::Ref<IUIAutomationCondition>, traversaloptions: TreeTraversalOptions, root: windows_core::Ref<IUIAutomationElement>) -> windows_core::Result<IUIAutomationElementArray>;
    fn FindFirstWithOptionsBuildCache(&self, scope: TreeScope, condition: windows_core::Ref<IUIAutomationCondition>, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, traversaloptions: TreeTraversalOptions, root: windows_core::Ref<IUIAutomationElement>) -> windows_core::Result<IUIAutomationElement>;
    fn FindAllWithOptionsBuildCache(&self, scope: TreeScope, condition: windows_core::Ref<IUIAutomationCondition>, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, traversaloptions: TreeTraversalOptions, root: windows_core::Ref<IUIAutomationElement>) -> windows_core::Result<IUIAutomationElementArray>;
    fn GetCurrentMetadataValue(&self, targetid: i32, metadataid: super::uiautomationcore::METADATAID) -> windows_core::Result<super::oaidl::VARIANT>;
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
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
        unsafe extern "system" fn GetCurrentMetadataValue<Identity: IUIAutomationElement7_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetid: i32, metadataid: super::uiautomationcore::METADATAID, returnval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
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
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn CurrentHeadingLevel(&self) -> windows_core::Result<super::uiautomationcore::HEADINGLEVELID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentHeadingLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn CachedHeadingLevel(&self) -> windows_core::Result<super::uiautomationcore::HEADINGLEVELID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedHeadingLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElement8_Vtbl {
    pub base__: IUIAutomationElement7_Vtbl,
    #[cfg(feature = "uiautomationcore")]
    pub CurrentHeadingLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::uiautomationcore::HEADINGLEVELID) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CurrentHeadingLevel: usize,
    #[cfg(feature = "uiautomationcore")]
    pub CachedHeadingLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::uiautomationcore::HEADINGLEVELID) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CachedHeadingLevel: usize,
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationElement8_Impl: IUIAutomationElement7_Impl {
    fn CurrentHeadingLevel(&self) -> windows_core::Result<super::uiautomationcore::HEADINGLEVELID>;
    fn CachedHeadingLevel(&self) -> windows_core::Result<super::uiautomationcore::HEADINGLEVELID>;
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationElement8_Vtbl {
    pub const fn new<Identity: IUIAutomationElement8_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentHeadingLevel<Identity: IUIAutomationElement8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::uiautomationcore::HEADINGLEVELID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement8_Impl::CurrentHeadingLevel(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedHeadingLevel<Identity: IUIAutomationElement8_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::uiautomationcore::HEADINGLEVELID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement8_Impl::CachedHeadingLevel(this) {
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
            CurrentHeadingLevel: CurrentHeadingLevel::<Identity, OFFSET>,
            CachedHeadingLevel: CachedHeadingLevel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement8 as windows_core::Interface>::IID || iid == &<IUIAutomationElement as windows_core::Interface>::IID || iid == &<IUIAutomationElement2 as windows_core::Interface>::IID || iid == &<IUIAutomationElement3 as windows_core::Interface>::IID || iid == &<IUIAutomationElement4 as windows_core::Interface>::IID || iid == &<IUIAutomationElement5 as windows_core::Interface>::IID || iid == &<IUIAutomationElement6 as windows_core::Interface>::IID || iid == &<IUIAutomationElement7 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn CurrentIsDialog(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentIsDialog)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedIsDialog(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedIsDialog)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElement9_Vtbl {
    pub base__: IUIAutomationElement8_Vtbl,
    pub CurrentIsDialog: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CachedIsDialog: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationElement9_Impl: IUIAutomationElement8_Impl {
    fn CurrentIsDialog(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedIsDialog(&self) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationElement9_Vtbl {
    pub const fn new<Identity: IUIAutomationElement9_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentIsDialog<Identity: IUIAutomationElement9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement9_Impl::CurrentIsDialog(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedIsDialog<Identity: IUIAutomationElement9_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElement9_Impl::CachedIsDialog(this) {
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
            CurrentIsDialog: CurrentIsDialog::<Identity, OFFSET>,
            CachedIsDialog: CachedIsDialog::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationElement9 as windows_core::Interface>::IID || iid == &<IUIAutomationElement as windows_core::Interface>::IID || iid == &<IUIAutomationElement2 as windows_core::Interface>::IID || iid == &<IUIAutomationElement3 as windows_core::Interface>::IID || iid == &<IUIAutomationElement4 as windows_core::Interface>::IID || iid == &<IUIAutomationElement5 as windows_core::Interface>::IID || iid == &<IUIAutomationElement6 as windows_core::Interface>::IID || iid == &<IUIAutomationElement7 as windows_core::Interface>::IID || iid == &<IUIAutomationElement8 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "windef", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUIAutomationElement9 {}
windows_core::imp::define_interface!(IUIAutomationElementArray, IUIAutomationElementArray_Vtbl, 0x14314595_b4bc_4055_95f2_58f2e42c9855);
windows_core::imp::interface_hierarchy!(IUIAutomationElementArray, windows_core::IUnknown);
impl IUIAutomationElementArray {
    pub unsafe fn Length(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Length)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetElement(&self, index: i32) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationElementArray_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAutomationElementArray_Impl: windows_core::IUnknownImpl {
    fn Length(&self) -> windows_core::Result<i32>;
    fn GetElement(&self, index: i32) -> windows_core::Result<IUIAutomationElement>;
}
impl IUIAutomationElementArray_Vtbl {
    pub const fn new<Identity: IUIAutomationElementArray_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Length<Identity: IUIAutomationElementArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElementArray_Impl::Length(this) {
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
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Length: Length::<Identity, OFFSET>, GetElement: GetElement::<Identity, OFFSET> }
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
    pub unsafe fn HandleAutomationEvent<P0>(&self, sender: P0, eventid: super::uiautomationcore::EVENTID) -> windows_core::HRESULT
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
    pub HandleAutomationEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::uiautomationcore::EVENTID) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    HandleAutomationEvent: usize,
}
#[cfg(feature = "uiautomationcore")]
pub trait IUIAutomationEventHandler_Impl: windows_core::IUnknownImpl {
    fn HandleAutomationEvent(&self, sender: windows_core::Ref<IUIAutomationElement>, eventid: super::uiautomationcore::EVENTID) -> windows_core::Result<()>;
}
#[cfg(feature = "uiautomationcore")]
impl IUIAutomationEventHandler_Vtbl {
    pub const fn new<Identity: IUIAutomationEventHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HandleAutomationEvent<Identity: IUIAutomationEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, eventid: super::uiautomationcore::EVENTID) -> windows_core::HRESULT {
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
    pub unsafe fn AddAutomationEventHandler<P2, P3>(&self, eventid: super::uiautomationcore::EVENTID, scope: TreeScope, cacherequest: P2, handler: P3) -> windows_core::HRESULT
    where
        P2: windows_core::Param<IUIAutomationCacheRequest>,
        P3: windows_core::Param<IUIAutomationEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddAutomationEventHandler)(windows_core::Interface::as_raw(self), eventid, scope, cacherequest.param().abi(), handler.param().abi()) }
    }
    pub unsafe fn AddChangesEventHandler<P3, P4>(&self, scope: TreeScope, changetypes: *const i32, changescount: i32, cacherequest: P3, handler: P4) -> windows_core::HRESULT
    where
        P3: windows_core::Param<IUIAutomationCacheRequest>,
        P4: windows_core::Param<IUIAutomationChangesEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddChangesEventHandler)(windows_core::Interface::as_raw(self), scope, changetypes, changescount, cacherequest.param().abi(), handler.param().abi()) }
    }
    pub unsafe fn AddNotificationEventHandler<P1, P2>(&self, scope: TreeScope, cacherequest: P1, handler: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IUIAutomationCacheRequest>,
        P2: windows_core::Param<IUIAutomationNotificationEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddNotificationEventHandler)(windows_core::Interface::as_raw(self), scope, cacherequest.param().abi(), handler.param().abi()) }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn AddPropertyChangedEventHandler<P1, P2>(&self, scope: TreeScope, cacherequest: P1, handler: P2, propertyarray: *const super::uiautomationcore::PROPERTYID, propertycount: i32) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IUIAutomationCacheRequest>,
        P2: windows_core::Param<IUIAutomationPropertyChangedEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddPropertyChangedEventHandler)(windows_core::Interface::as_raw(self), scope, cacherequest.param().abi(), handler.param().abi(), propertyarray, propertycount) }
    }
    pub unsafe fn AddStructureChangedEventHandler<P1, P2>(&self, scope: TreeScope, cacherequest: P1, handler: P2) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IUIAutomationCacheRequest>,
        P2: windows_core::Param<IUIAutomationStructureChangedEventHandler>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddStructureChangedEventHandler)(windows_core::Interface::as_raw(self), scope, cacherequest.param().abi(), handler.param().abi()) }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn AddTextEditTextChangedEventHandler<P2, P3>(&self, scope: TreeScope, texteditchangetype: super::uiautomationcore::TextEditChangeType, cacherequest: P2, handler: P3) -> windows_core::HRESULT
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
    pub AddAutomationEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::EVENTID, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    AddAutomationEventHandler: usize,
    pub AddChangesEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *const i32, i32, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddNotificationEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub AddPropertyChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void, *const super::uiautomationcore::PROPERTYID, i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    AddPropertyChangedEventHandler: usize,
    pub AddStructureChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub AddTextEditTextChangedEventHandler: unsafe extern "system" fn(*mut core::ffi::c_void, TreeScope, super::uiautomationcore::TextEditChangeType, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    AddTextEditTextChangedEventHandler: usize,
}
#[cfg(feature = "uiautomationcore")]
pub trait IUIAutomationEventHandlerGroup_Impl: windows_core::IUnknownImpl {
    fn AddActiveTextPositionChangedEventHandler(&self, scope: TreeScope, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationActiveTextPositionChangedEventHandler>) -> windows_core::Result<()>;
    fn AddAutomationEventHandler(&self, eventid: super::uiautomationcore::EVENTID, scope: TreeScope, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationEventHandler>) -> windows_core::Result<()>;
    fn AddChangesEventHandler(&self, scope: TreeScope, changetypes: *const i32, changescount: i32, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationChangesEventHandler>) -> windows_core::Result<()>;
    fn AddNotificationEventHandler(&self, scope: TreeScope, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationNotificationEventHandler>) -> windows_core::Result<()>;
    fn AddPropertyChangedEventHandler(&self, scope: TreeScope, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationPropertyChangedEventHandler>, propertyarray: *const super::uiautomationcore::PROPERTYID, propertycount: i32) -> windows_core::Result<()>;
    fn AddStructureChangedEventHandler(&self, scope: TreeScope, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationStructureChangedEventHandler>) -> windows_core::Result<()>;
    fn AddTextEditTextChangedEventHandler(&self, scope: TreeScope, texteditchangetype: super::uiautomationcore::TextEditChangeType, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>, handler: windows_core::Ref<IUIAutomationTextEditTextChangedEventHandler>) -> windows_core::Result<()>;
}
#[cfg(feature = "uiautomationcore")]
impl IUIAutomationEventHandlerGroup_Vtbl {
    pub const fn new<Identity: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddActiveTextPositionChangedEventHandler<Identity: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: TreeScope, cacherequest: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationEventHandlerGroup_Impl::AddActiveTextPositionChangedEventHandler(this, core::mem::transmute_copy(&scope), core::mem::transmute_copy(&cacherequest), core::mem::transmute_copy(&handler)).into()
            }
        }
        unsafe extern "system" fn AddAutomationEventHandler<Identity: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: super::uiautomationcore::EVENTID, scope: TreeScope, cacherequest: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn AddPropertyChangedEventHandler<Identity: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: TreeScope, cacherequest: *mut core::ffi::c_void, handler: *mut core::ffi::c_void, propertyarray: *const super::uiautomationcore::PROPERTYID, propertycount: i32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn AddTextEditTextChangedEventHandler<Identity: IUIAutomationEventHandlerGroup_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, scope: TreeScope, texteditchangetype: super::uiautomationcore::TextEditChangeType, cacherequest: *mut core::ffi::c_void, handler: *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(feature = "uiautomationcore")]
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
    pub unsafe fn CurrentExpandCollapseState(&self) -> windows_core::Result<super::uiautomationcore::ExpandCollapseState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentExpandCollapseState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn CachedExpandCollapseState(&self) -> windows_core::Result<super::uiautomationcore::ExpandCollapseState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedExpandCollapseState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    pub CurrentExpandCollapseState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::uiautomationcore::ExpandCollapseState) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CurrentExpandCollapseState: usize,
    #[cfg(feature = "uiautomationcore")]
    pub CachedExpandCollapseState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::uiautomationcore::ExpandCollapseState) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CachedExpandCollapseState: usize,
}
#[cfg(feature = "uiautomationcore")]
pub trait IUIAutomationExpandCollapsePattern_Impl: windows_core::IUnknownImpl {
    fn Expand(&self) -> windows_core::Result<()>;
    fn Collapse(&self) -> windows_core::Result<()>;
    fn CurrentExpandCollapseState(&self) -> windows_core::Result<super::uiautomationcore::ExpandCollapseState>;
    fn CachedExpandCollapseState(&self) -> windows_core::Result<super::uiautomationcore::ExpandCollapseState>;
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
        unsafe extern "system" fn CurrentExpandCollapseState<Identity: IUIAutomationExpandCollapsePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::uiautomationcore::ExpandCollapseState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationExpandCollapsePattern_Impl::CurrentExpandCollapseState(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedExpandCollapseState<Identity: IUIAutomationExpandCollapsePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::uiautomationcore::ExpandCollapseState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationExpandCollapsePattern_Impl::CachedExpandCollapseState(this) {
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
            CurrentExpandCollapseState: CurrentExpandCollapseState::<Identity, OFFSET>,
            CachedExpandCollapseState: CachedExpandCollapseState::<Identity, OFFSET>,
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
    pub unsafe fn CurrentContainingGrid(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentContainingGrid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CurrentRow(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentRow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentColumn(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentColumn)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentRowSpan(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentRowSpan)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentColumnSpan(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentColumnSpan)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedContainingGrid(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedContainingGrid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CachedRow(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedRow)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedColumn(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedColumn)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedRowSpan(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedRowSpan)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedColumnSpan(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedColumnSpan)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationGridItemPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CurrentContainingGrid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentRow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentColumn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentRowSpan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentColumnSpan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedContainingGrid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedRow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedColumn: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedRowSpan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedColumnSpan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
pub trait IUIAutomationGridItemPattern_Impl: windows_core::IUnknownImpl {
    fn CurrentContainingGrid(&self) -> windows_core::Result<IUIAutomationElement>;
    fn CurrentRow(&self) -> windows_core::Result<i32>;
    fn CurrentColumn(&self) -> windows_core::Result<i32>;
    fn CurrentRowSpan(&self) -> windows_core::Result<i32>;
    fn CurrentColumnSpan(&self) -> windows_core::Result<i32>;
    fn CachedContainingGrid(&self) -> windows_core::Result<IUIAutomationElement>;
    fn CachedRow(&self) -> windows_core::Result<i32>;
    fn CachedColumn(&self) -> windows_core::Result<i32>;
    fn CachedRowSpan(&self) -> windows_core::Result<i32>;
    fn CachedColumnSpan(&self) -> windows_core::Result<i32>;
}
impl IUIAutomationGridItemPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentContainingGrid<Identity: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridItemPattern_Impl::CurrentContainingGrid(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentRow<Identity: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridItemPattern_Impl::CurrentRow(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentColumn<Identity: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridItemPattern_Impl::CurrentColumn(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentRowSpan<Identity: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridItemPattern_Impl::CurrentRowSpan(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentColumnSpan<Identity: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridItemPattern_Impl::CurrentColumnSpan(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedContainingGrid<Identity: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridItemPattern_Impl::CachedContainingGrid(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedRow<Identity: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridItemPattern_Impl::CachedRow(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedColumn<Identity: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridItemPattern_Impl::CachedColumn(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedRowSpan<Identity: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridItemPattern_Impl::CachedRowSpan(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedColumnSpan<Identity: IUIAutomationGridItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridItemPattern_Impl::CachedColumnSpan(this) {
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
            CurrentContainingGrid: CurrentContainingGrid::<Identity, OFFSET>,
            CurrentRow: CurrentRow::<Identity, OFFSET>,
            CurrentColumn: CurrentColumn::<Identity, OFFSET>,
            CurrentRowSpan: CurrentRowSpan::<Identity, OFFSET>,
            CurrentColumnSpan: CurrentColumnSpan::<Identity, OFFSET>,
            CachedContainingGrid: CachedContainingGrid::<Identity, OFFSET>,
            CachedRow: CachedRow::<Identity, OFFSET>,
            CachedColumn: CachedColumn::<Identity, OFFSET>,
            CachedRowSpan: CachedRowSpan::<Identity, OFFSET>,
            CachedColumnSpan: CachedColumnSpan::<Identity, OFFSET>,
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
            (windows_core::Interface::vtable(self).GetItem)(windows_core::Interface::as_raw(self), row, column, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CurrentRowCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentRowCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentColumnCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentColumnCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedRowCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedRowCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedColumnCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedColumnCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationGridPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentRowCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentColumnCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedRowCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedColumnCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
pub trait IUIAutomationGridPattern_Impl: windows_core::IUnknownImpl {
    fn GetItem(&self, row: i32, column: i32) -> windows_core::Result<IUIAutomationElement>;
    fn CurrentRowCount(&self) -> windows_core::Result<i32>;
    fn CurrentColumnCount(&self) -> windows_core::Result<i32>;
    fn CachedRowCount(&self) -> windows_core::Result<i32>;
    fn CachedColumnCount(&self) -> windows_core::Result<i32>;
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
        unsafe extern "system" fn CurrentRowCount<Identity: IUIAutomationGridPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridPattern_Impl::CurrentRowCount(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentColumnCount<Identity: IUIAutomationGridPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridPattern_Impl::CurrentColumnCount(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedRowCount<Identity: IUIAutomationGridPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridPattern_Impl::CachedRowCount(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedColumnCount<Identity: IUIAutomationGridPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationGridPattern_Impl::CachedColumnCount(this) {
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
            CurrentRowCount: CurrentRowCount::<Identity, OFFSET>,
            CurrentColumnCount: CurrentColumnCount::<Identity, OFFSET>,
            CachedRowCount: CachedRowCount::<Identity, OFFSET>,
            CachedColumnCount: CachedColumnCount::<Identity, OFFSET>,
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
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn FindItemByProperty<P0>(&self, pstartafter: P0, propertyid: super::uiautomationcore::PROPERTYID, value: &super::oaidl::VARIANT) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindItemByProperty)(windows_core::Interface::as_raw(self), pstartafter.param().abi(), propertyid, core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationItemContainerPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
    pub FindItemByProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::uiautomationcore::PROPERTYID, super::oaidl::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase")))]
    FindItemByProperty: usize,
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationItemContainerPattern_Impl: windows_core::IUnknownImpl {
    fn FindItemByProperty(&self, pstartafter: windows_core::Ref<IUIAutomationElement>, propertyid: super::uiautomationcore::PROPERTYID, value: &super::oaidl::VARIANT) -> windows_core::Result<IUIAutomationElement>;
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationItemContainerPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationItemContainerPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindItemByProperty<Identity: IUIAutomationItemContainerPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstartafter: *mut core::ffi::c_void, propertyid: super::uiautomationcore::PROPERTYID, value: super::oaidl::VARIANT, pfound: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn CurrentChildId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentChildId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentValue(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentRole(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentRole)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentState(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentHelp(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentHelp)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentKeyboardShortcut(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentKeyboardShortcut)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetCurrentSelection(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentSelection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CurrentDefaultAction(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentDefaultAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedChildId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedChildId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedValue(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedDescription(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedDescription)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedRole(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedRole)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedState(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedHelp(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedHelp)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedKeyboardShortcut(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedKeyboardShortcut)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetCachedSelection(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedSelection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CachedDefaultAction(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedDefaultAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "oleacc"))]
    pub unsafe fn GetIAccessible(&self) -> windows_core::Result<super::oleacc::IAccessible> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIAccessible)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    pub CurrentChildId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CurrentState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CurrentHelp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentKeyboardShortcut: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentDefaultAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedChildId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedDescription: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedRole: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CachedState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub CachedHelp: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedKeyboardShortcut: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCachedSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedDefaultAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "oleacc"))]
    pub GetIAccessible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "oleacc")))]
    GetIAccessible: usize,
}
#[cfg(all(feature = "oaidl", feature = "oleacc"))]
pub trait IUIAutomationLegacyIAccessiblePattern_Impl: windows_core::IUnknownImpl {
    fn Select(&self, flagsselect: i32) -> windows_core::Result<()>;
    fn DoDefaultAction(&self) -> windows_core::Result<()>;
    fn SetValue(&self, szvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn CurrentChildId(&self) -> windows_core::Result<i32>;
    fn CurrentName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentValue(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentRole(&self) -> windows_core::Result<u32>;
    fn CurrentState(&self) -> windows_core::Result<u32>;
    fn CurrentHelp(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentKeyboardShortcut(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetCurrentSelection(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn CurrentDefaultAction(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedChildId(&self) -> windows_core::Result<i32>;
    fn CachedName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedValue(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedDescription(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedRole(&self) -> windows_core::Result<u32>;
    fn CachedState(&self) -> windows_core::Result<u32>;
    fn CachedHelp(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedKeyboardShortcut(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetCachedSelection(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn CachedDefaultAction(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetIAccessible(&self) -> windows_core::Result<super::oleacc::IAccessible>;
}
#[cfg(all(feature = "oaidl", feature = "oleacc"))]
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
        unsafe extern "system" fn CurrentChildId<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::CurrentChildId(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentName<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::CurrentName(this) {
                    Ok(ok__) => {
                        pszname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentValue<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::CurrentValue(this) {
                    Ok(ok__) => {
                        pszvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentDescription<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::CurrentDescription(this) {
                    Ok(ok__) => {
                        pszdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentRole<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwrole: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::CurrentRole(this) {
                    Ok(ok__) => {
                        pdwrole.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentState<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstate: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::CurrentState(this) {
                    Ok(ok__) => {
                        pdwstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentHelp<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszhelp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::CurrentHelp(this) {
                    Ok(ok__) => {
                        pszhelp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentKeyboardShortcut<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszkeyboardshortcut: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::CurrentKeyboardShortcut(this) {
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
        unsafe extern "system" fn CurrentDefaultAction<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdefaultaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::CurrentDefaultAction(this) {
                    Ok(ok__) => {
                        pszdefaultaction.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedChildId<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::CachedChildId(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedName<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::CachedName(this) {
                    Ok(ok__) => {
                        pszname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedValue<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::CachedValue(this) {
                    Ok(ok__) => {
                        pszvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedDescription<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::CachedDescription(this) {
                    Ok(ok__) => {
                        pszdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedRole<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwrole: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::CachedRole(this) {
                    Ok(ok__) => {
                        pdwrole.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedState<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstate: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::CachedState(this) {
                    Ok(ok__) => {
                        pdwstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedHelp<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszhelp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::CachedHelp(this) {
                    Ok(ok__) => {
                        pszhelp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedKeyboardShortcut<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszkeyboardshortcut: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::CachedKeyboardShortcut(this) {
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
        unsafe extern "system" fn CachedDefaultAction<Identity: IUIAutomationLegacyIAccessiblePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdefaultaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationLegacyIAccessiblePattern_Impl::CachedDefaultAction(this) {
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
            CurrentChildId: CurrentChildId::<Identity, OFFSET>,
            CurrentName: CurrentName::<Identity, OFFSET>,
            CurrentValue: CurrentValue::<Identity, OFFSET>,
            CurrentDescription: CurrentDescription::<Identity, OFFSET>,
            CurrentRole: CurrentRole::<Identity, OFFSET>,
            CurrentState: CurrentState::<Identity, OFFSET>,
            CurrentHelp: CurrentHelp::<Identity, OFFSET>,
            CurrentKeyboardShortcut: CurrentKeyboardShortcut::<Identity, OFFSET>,
            GetCurrentSelection: GetCurrentSelection::<Identity, OFFSET>,
            CurrentDefaultAction: CurrentDefaultAction::<Identity, OFFSET>,
            CachedChildId: CachedChildId::<Identity, OFFSET>,
            CachedName: CachedName::<Identity, OFFSET>,
            CachedValue: CachedValue::<Identity, OFFSET>,
            CachedDescription: CachedDescription::<Identity, OFFSET>,
            CachedRole: CachedRole::<Identity, OFFSET>,
            CachedState: CachedState::<Identity, OFFSET>,
            CachedHelp: CachedHelp::<Identity, OFFSET>,
            CachedKeyboardShortcut: CachedKeyboardShortcut::<Identity, OFFSET>,
            GetCachedSelection: GetCachedSelection::<Identity, OFFSET>,
            CachedDefaultAction: CachedDefaultAction::<Identity, OFFSET>,
            GetIAccessible: GetIAccessible::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationLegacyIAccessiblePattern as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "oleacc"))]
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
    pub unsafe fn CurrentCurrentView(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentCurrentView)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetCurrentSupportedViews(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentSupportedViews)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedCurrentView(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedCurrentView)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetCachedSupportedViews(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
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
    pub CurrentCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub GetCurrentSupportedViews: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetCurrentSupportedViews: usize,
    pub CachedCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub GetCachedSupportedViews: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetCachedSupportedViews: usize,
}
#[cfg(feature = "oaidl")]
pub trait IUIAutomationMultipleViewPattern_Impl: windows_core::IUnknownImpl {
    fn GetViewName(&self, view: i32) -> windows_core::Result<windows_core::BSTR>;
    fn SetCurrentView(&self, view: i32) -> windows_core::Result<()>;
    fn CurrentCurrentView(&self) -> windows_core::Result<i32>;
    fn GetCurrentSupportedViews(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn CachedCurrentView(&self) -> windows_core::Result<i32>;
    fn GetCachedSupportedViews(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
}
#[cfg(feature = "oaidl")]
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
        unsafe extern "system" fn CurrentCurrentView<Identity: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationMultipleViewPattern_Impl::CurrentCurrentView(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCurrentSupportedViews<Identity: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
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
        unsafe extern "system" fn CachedCurrentView<Identity: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationMultipleViewPattern_Impl::CachedCurrentView(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCachedSupportedViews<Identity: IUIAutomationMultipleViewPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
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
            CurrentCurrentView: CurrentCurrentView::<Identity, OFFSET>,
            GetCurrentSupportedViews: GetCurrentSupportedViews::<Identity, OFFSET>,
            CachedCurrentView: CachedCurrentView::<Identity, OFFSET>,
            GetCachedSupportedViews: GetCachedSupportedViews::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationMultipleViewPattern as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
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
            (windows_core::Interface::vtable(self).GetChild)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    pub unsafe fn HandleNotificationEvent<P0>(&self, sender: P0, notificationkind: super::uiautomationcore::NotificationKind, notificationprocessing: super::uiautomationcore::NotificationProcessing, displaystring: &windows_core::BSTR, activityid: &windows_core::BSTR) -> windows_core::HRESULT
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
    pub HandleNotificationEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::uiautomationcore::NotificationKind, super::uiautomationcore::NotificationProcessing, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    HandleNotificationEvent: usize,
}
#[cfg(feature = "uiautomationcore")]
pub trait IUIAutomationNotificationEventHandler_Impl: windows_core::IUnknownImpl {
    fn HandleNotificationEvent(&self, sender: windows_core::Ref<IUIAutomationElement>, notificationkind: super::uiautomationcore::NotificationKind, notificationprocessing: super::uiautomationcore::NotificationProcessing, displaystring: &windows_core::BSTR, activityid: &windows_core::BSTR) -> windows_core::Result<()>;
}
#[cfg(feature = "uiautomationcore")]
impl IUIAutomationNotificationEventHandler_Vtbl {
    pub const fn new<Identity: IUIAutomationNotificationEventHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HandleNotificationEvent<Identity: IUIAutomationNotificationEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, notificationkind: super::uiautomationcore::NotificationKind, notificationprocessing: super::uiautomationcore::NotificationProcessing, displaystring: *mut core::ffi::c_void, activityid: *mut core::ffi::c_void) -> windows_core::HRESULT {
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
            (windows_core::Interface::vtable(self).GetUnderlyingObjectModel)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    pub unsafe fn ChildCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ChildCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetChildrenAsNativeArray(&self, childarray: *mut *mut Option<IUIAutomationCondition>, childarraycount: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetChildrenAsNativeArray)(windows_core::Interface::as_raw(self), childarray as _, childarraycount as _) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetChildren(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
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
    pub ChildCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetChildrenAsNativeArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub GetChildren: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetChildren: usize,
}
#[cfg(feature = "oaidl")]
pub trait IUIAutomationOrCondition_Impl: IUIAutomationCondition_Impl {
    fn ChildCount(&self) -> windows_core::Result<i32>;
    fn GetChildrenAsNativeArray(&self, childarray: *mut *mut Option<IUIAutomationCondition>, childarraycount: *mut i32) -> windows_core::Result<()>;
    fn GetChildren(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
}
#[cfg(feature = "oaidl")]
impl IUIAutomationOrCondition_Vtbl {
    pub const fn new<Identity: IUIAutomationOrCondition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ChildCount<Identity: IUIAutomationOrCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, childcount: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationOrCondition_Impl::ChildCount(this) {
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
        unsafe extern "system" fn GetChildren<Identity: IUIAutomationOrCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, childarray: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
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
            ChildCount: ChildCount::<Identity, OFFSET>,
            GetChildrenAsNativeArray: GetChildrenAsNativeArray::<Identity, OFFSET>,
            GetChildren: GetChildren::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationOrCondition as windows_core::Interface>::IID || iid == &<IUIAutomationCondition as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IUIAutomationOrCondition {}
windows_core::imp::define_interface!(IUIAutomationPropertyChangedEventHandler, IUIAutomationPropertyChangedEventHandler_Vtbl, 0x40cd37d4_c756_4b0c_8c6f_bddfeeb13b50);
windows_core::imp::interface_hierarchy!(IUIAutomationPropertyChangedEventHandler, windows_core::IUnknown);
impl IUIAutomationPropertyChangedEventHandler {
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn HandlePropertyChangedEvent<P0>(&self, sender: P0, propertyid: super::uiautomationcore::PROPERTYID, newvalue: &super::oaidl::VARIANT) -> windows_core::HRESULT
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
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
    pub HandlePropertyChangedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::uiautomationcore::PROPERTYID, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase")))]
    HandlePropertyChangedEvent: usize,
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationPropertyChangedEventHandler_Impl: windows_core::IUnknownImpl {
    fn HandlePropertyChangedEvent(&self, sender: windows_core::Ref<IUIAutomationElement>, propertyid: super::uiautomationcore::PROPERTYID, newvalue: &super::oaidl::VARIANT) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationPropertyChangedEventHandler_Vtbl {
    pub const fn new<Identity: IUIAutomationPropertyChangedEventHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HandlePropertyChangedEvent<Identity: IUIAutomationPropertyChangedEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, propertyid: super::uiautomationcore::PROPERTYID, newvalue: super::oaidl::VARIANT) -> windows_core::HRESULT {
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
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn PropertyId(&self) -> windows_core::Result<super::uiautomationcore::PROPERTYID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PropertyId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn PropertyValue(&self) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PropertyValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn PropertyConditionFlags(&self) -> windows_core::Result<PropertyConditionFlags> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).PropertyConditionFlags)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationPropertyCondition_Vtbl {
    pub base__: IUIAutomationCondition_Vtbl,
    #[cfg(feature = "uiautomationcore")]
    pub PropertyId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::uiautomationcore::PROPERTYID) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    PropertyId: usize,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub PropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    PropertyValue: usize,
    pub PropertyConditionFlags: unsafe extern "system" fn(*mut core::ffi::c_void, *mut PropertyConditionFlags) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationPropertyCondition_Impl: IUIAutomationCondition_Impl {
    fn PropertyId(&self) -> windows_core::Result<super::uiautomationcore::PROPERTYID>;
    fn PropertyValue(&self) -> windows_core::Result<super::oaidl::VARIANT>;
    fn PropertyConditionFlags(&self) -> windows_core::Result<PropertyConditionFlags>;
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
impl IUIAutomationPropertyCondition_Vtbl {
    pub const fn new<Identity: IUIAutomationPropertyCondition_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn PropertyId<Identity: IUIAutomationPropertyCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: *mut super::uiautomationcore::PROPERTYID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationPropertyCondition_Impl::PropertyId(this) {
                    Ok(ok__) => {
                        propertyid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PropertyValue<Identity: IUIAutomationPropertyCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyvalue: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationPropertyCondition_Impl::PropertyValue(this) {
                    Ok(ok__) => {
                        propertyvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn PropertyConditionFlags<Identity: IUIAutomationPropertyCondition_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flags: *mut PropertyConditionFlags) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationPropertyCondition_Impl::PropertyConditionFlags(this) {
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
            PropertyId: PropertyId::<Identity, OFFSET>,
            PropertyValue: PropertyValue::<Identity, OFFSET>,
            PropertyConditionFlags: PropertyConditionFlags::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationPropertyCondition as windows_core::Interface>::IID || iid == &<IUIAutomationCondition as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUIAutomationPropertyCondition {}
windows_core::imp::define_interface!(IUIAutomationProxyFactory, IUIAutomationProxyFactory_Vtbl, 0x85b94ecd_849d_42b6_b94d_d6db23fdf5a4);
windows_core::imp::interface_hierarchy!(IUIAutomationProxyFactory, windows_core::IUnknown);
impl IUIAutomationProxyFactory {
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn CreateProvider(&self, hwnd: UIA_HWND, idobject: i32, idchild: i32) -> windows_core::Result<super::uiautomationcore::IRawElementProviderSimple> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateProvider)(windows_core::Interface::as_raw(self), hwnd, idobject, idchild, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ProxyFactoryId(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProxyFactoryId)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationProxyFactory_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "uiautomationcore")]
    pub CreateProvider: unsafe extern "system" fn(*mut core::ffi::c_void, UIA_HWND, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CreateProvider: usize,
    pub ProxyFactoryId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "uiautomationcore")]
pub trait IUIAutomationProxyFactory_Impl: windows_core::IUnknownImpl {
    fn CreateProvider(&self, hwnd: UIA_HWND, idobject: i32, idchild: i32) -> windows_core::Result<super::uiautomationcore::IRawElementProviderSimple>;
    fn ProxyFactoryId(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "uiautomationcore")]
impl IUIAutomationProxyFactory_Vtbl {
    pub const fn new<Identity: IUIAutomationProxyFactory_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateProvider<Identity: IUIAutomationProxyFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: UIA_HWND, idobject: i32, idchild: i32, provider: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn ProxyFactoryId<Identity: IUIAutomationProxyFactory_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, factoryid: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationProxyFactory_Impl::ProxyFactoryId(this) {
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
            ProxyFactoryId: ProxyFactoryId::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationProxyFactory as windows_core::Interface>::IID
    }
}
#[cfg(feature = "uiautomationcore")]
impl windows_core::RuntimeName for IUIAutomationProxyFactory {}
windows_core::imp::define_interface!(IUIAutomationProxyFactoryEntry, IUIAutomationProxyFactoryEntry_Vtbl, 0xd50e472e_b64b_490c_bca1_d30696f9f289);
windows_core::imp::interface_hierarchy!(IUIAutomationProxyFactoryEntry, windows_core::IUnknown);
impl IUIAutomationProxyFactoryEntry {
    pub unsafe fn ProxyFactory(&self) -> windows_core::Result<IUIAutomationProxyFactory> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProxyFactory)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ClassName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ClassName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn ImageName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ImageName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn AllowSubstringMatch(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AllowSubstringMatch)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CanCheckBaseClass(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanCheckBaseClass)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn NeedsAdviseEvents(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NeedsAdviseEvents)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetClassName<P0>(&self, classname: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetClassName)(windows_core::Interface::as_raw(self), classname.param().abi()) }
    }
    pub unsafe fn SetImageName<P0>(&self, imagename: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetImageName)(windows_core::Interface::as_raw(self), imagename.param().abi()) }
    }
    pub unsafe fn SetAllowSubstringMatch(&self, allowsubstringmatch: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetAllowSubstringMatch)(windows_core::Interface::as_raw(self), allowsubstringmatch.into()) }
    }
    pub unsafe fn SetCanCheckBaseClass(&self, cancheckbaseclass: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCanCheckBaseClass)(windows_core::Interface::as_raw(self), cancheckbaseclass.into()) }
    }
    pub unsafe fn SetNeedsAdviseEvents(&self, adviseevents: bool) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetNeedsAdviseEvents)(windows_core::Interface::as_raw(self), adviseevents.into()) }
    }
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore"))]
    pub unsafe fn SetWinEventsForAutomationEvent(&self, eventid: super::uiautomationcore::EVENTID, propertyid: super::uiautomationcore::PROPERTYID, winevents: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWinEventsForAutomationEvent)(windows_core::Interface::as_raw(self), eventid, propertyid, winevents) }
    }
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore"))]
    pub unsafe fn GetWinEventsForAutomationEvent(&self, eventid: super::uiautomationcore::EVENTID, propertyid: super::uiautomationcore::PROPERTYID) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
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
    pub ProxyFactory: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ClassName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ImageName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AllowSubstringMatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CanCheckBaseClass: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub NeedsAdviseEvents: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SetClassName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetImageName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub SetAllowSubstringMatch: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetCanCheckBaseClass: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub SetNeedsAdviseEvents: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore"))]
    pub SetWinEventsForAutomationEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::EVENTID, super::uiautomationcore::PROPERTYID, *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "uiautomationcore")))]
    SetWinEventsForAutomationEvent: usize,
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore"))]
    pub GetWinEventsForAutomationEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::EVENTID, super::uiautomationcore::PROPERTYID, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "uiautomationcore")))]
    GetWinEventsForAutomationEvent: usize,
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore"))]
pub trait IUIAutomationProxyFactoryEntry_Impl: windows_core::IUnknownImpl {
    fn ProxyFactory(&self) -> windows_core::Result<IUIAutomationProxyFactory>;
    fn ClassName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn ImageName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn AllowSubstringMatch(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CanCheckBaseClass(&self) -> windows_core::Result<windows_core::BOOL>;
    fn NeedsAdviseEvents(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SetClassName(&self, classname: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetImageName(&self, imagename: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn SetAllowSubstringMatch(&self, allowsubstringmatch: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetCanCheckBaseClass(&self, cancheckbaseclass: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetNeedsAdviseEvents(&self, adviseevents: windows_core::BOOL) -> windows_core::Result<()>;
    fn SetWinEventsForAutomationEvent(&self, eventid: super::uiautomationcore::EVENTID, propertyid: super::uiautomationcore::PROPERTYID, winevents: *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
    fn GetWinEventsForAutomationEvent(&self, eventid: super::uiautomationcore::EVENTID, propertyid: super::uiautomationcore::PROPERTYID) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore"))]
impl IUIAutomationProxyFactoryEntry_Vtbl {
    pub const fn new<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProxyFactory<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, factory: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationProxyFactoryEntry_Impl::ProxyFactory(this) {
                    Ok(ok__) => {
                        factory.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ClassName<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationProxyFactoryEntry_Impl::ClassName(this) {
                    Ok(ok__) => {
                        classname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ImageName<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagename: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationProxyFactoryEntry_Impl::ImageName(this) {
                    Ok(ok__) => {
                        imagename.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AllowSubstringMatch<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allowsubstringmatch: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationProxyFactoryEntry_Impl::AllowSubstringMatch(this) {
                    Ok(ok__) => {
                        allowsubstringmatch.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanCheckBaseClass<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cancheckbaseclass: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationProxyFactoryEntry_Impl::CanCheckBaseClass(this) {
                    Ok(ok__) => {
                        cancheckbaseclass.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn NeedsAdviseEvents<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adviseevents: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationProxyFactoryEntry_Impl::NeedsAdviseEvents(this) {
                    Ok(ok__) => {
                        adviseevents.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetClassName<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, classname: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationProxyFactoryEntry_Impl::SetClassName(this, core::mem::transmute(&classname)).into()
            }
        }
        unsafe extern "system" fn SetImageName<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, imagename: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationProxyFactoryEntry_Impl::SetImageName(this, core::mem::transmute(&imagename)).into()
            }
        }
        unsafe extern "system" fn SetAllowSubstringMatch<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, allowsubstringmatch: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationProxyFactoryEntry_Impl::SetAllowSubstringMatch(this, core::mem::transmute_copy(&allowsubstringmatch)).into()
            }
        }
        unsafe extern "system" fn SetCanCheckBaseClass<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, cancheckbaseclass: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationProxyFactoryEntry_Impl::SetCanCheckBaseClass(this, core::mem::transmute_copy(&cancheckbaseclass)).into()
            }
        }
        unsafe extern "system" fn SetNeedsAdviseEvents<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, adviseevents: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationProxyFactoryEntry_Impl::SetNeedsAdviseEvents(this, core::mem::transmute_copy(&adviseevents)).into()
            }
        }
        unsafe extern "system" fn SetWinEventsForAutomationEvent<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: super::uiautomationcore::EVENTID, propertyid: super::uiautomationcore::PROPERTYID, winevents: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationProxyFactoryEntry_Impl::SetWinEventsForAutomationEvent(this, core::mem::transmute_copy(&eventid), core::mem::transmute_copy(&propertyid), core::mem::transmute_copy(&winevents)).into()
            }
        }
        unsafe extern "system" fn GetWinEventsForAutomationEvent<Identity: IUIAutomationProxyFactoryEntry_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: super::uiautomationcore::EVENTID, propertyid: super::uiautomationcore::PROPERTYID, winevents: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
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
            ProxyFactory: ProxyFactory::<Identity, OFFSET>,
            ClassName: ClassName::<Identity, OFFSET>,
            ImageName: ImageName::<Identity, OFFSET>,
            AllowSubstringMatch: AllowSubstringMatch::<Identity, OFFSET>,
            CanCheckBaseClass: CanCheckBaseClass::<Identity, OFFSET>,
            NeedsAdviseEvents: NeedsAdviseEvents::<Identity, OFFSET>,
            SetClassName: SetClassName::<Identity, OFFSET>,
            SetImageName: SetImageName::<Identity, OFFSET>,
            SetAllowSubstringMatch: SetAllowSubstringMatch::<Identity, OFFSET>,
            SetCanCheckBaseClass: SetCanCheckBaseClass::<Identity, OFFSET>,
            SetNeedsAdviseEvents: SetNeedsAdviseEvents::<Identity, OFFSET>,
            SetWinEventsForAutomationEvent: SetWinEventsForAutomationEvent::<Identity, OFFSET>,
            GetWinEventsForAutomationEvent: GetWinEventsForAutomationEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationProxyFactoryEntry as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore"))]
impl windows_core::RuntimeName for IUIAutomationProxyFactoryEntry {}
windows_core::imp::define_interface!(IUIAutomationProxyFactoryMapping, IUIAutomationProxyFactoryMapping_Vtbl, 0x09e31e18_872d_4873_93d1_1e541ec133fd);
windows_core::imp::interface_hierarchy!(IUIAutomationProxyFactoryMapping, windows_core::IUnknown);
impl IUIAutomationProxyFactoryMapping {
    pub unsafe fn Count(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Count)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetTable(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetTable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetEntry(&self, index: u32) -> windows_core::Result<IUIAutomationProxyFactoryEntry> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEntry)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn SetTable(&self, factorylist: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetTable)(windows_core::Interface::as_raw(self), factorylist) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn InsertEntries(&self, before: u32, factorylist: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).InsertEntries)(windows_core::Interface::as_raw(self), before, factorylist) }
    }
    pub unsafe fn InsertEntry<P1>(&self, before: u32, factory: P1) -> windows_core::HRESULT
    where
        P1: windows_core::Param<IUIAutomationProxyFactoryEntry>,
    {
        unsafe { (windows_core::Interface::vtable(self).InsertEntry)(windows_core::Interface::as_raw(self), before, factory.param().abi()) }
    }
    pub unsafe fn RemoveEntry(&self, index: u32) -> windows_core::HRESULT {
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
    pub Count: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub GetTable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetTable: usize,
    pub GetEntry: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub SetTable: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    SetTable: usize,
    #[cfg(feature = "oaidl")]
    pub InsertEntries: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    InsertEntries: usize,
    pub InsertEntry: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveEntry: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub ClearTable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RestoreDefaultTable: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "oaidl")]
pub trait IUIAutomationProxyFactoryMapping_Impl: windows_core::IUnknownImpl {
    fn Count(&self) -> windows_core::Result<u32>;
    fn GetTable(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn GetEntry(&self, index: u32) -> windows_core::Result<IUIAutomationProxyFactoryEntry>;
    fn SetTable(&self, factorylist: *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
    fn InsertEntries(&self, before: u32, factorylist: *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
    fn InsertEntry(&self, before: u32, factory: windows_core::Ref<IUIAutomationProxyFactoryEntry>) -> windows_core::Result<()>;
    fn RemoveEntry(&self, index: u32) -> windows_core::Result<()>;
    fn ClearTable(&self) -> windows_core::Result<()>;
    fn RestoreDefaultTable(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "oaidl")]
impl IUIAutomationProxyFactoryMapping_Vtbl {
    pub const fn new<Identity: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Count<Identity: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, count: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationProxyFactoryMapping_Impl::Count(this) {
                    Ok(ok__) => {
                        count.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetTable<Identity: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, table: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetEntry<Identity: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, entry: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn SetTable<Identity: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, factorylist: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationProxyFactoryMapping_Impl::SetTable(this, core::mem::transmute_copy(&factorylist)).into()
            }
        }
        unsafe extern "system" fn InsertEntries<Identity: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, before: u32, factorylist: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationProxyFactoryMapping_Impl::InsertEntries(this, core::mem::transmute_copy(&before), core::mem::transmute_copy(&factorylist)).into()
            }
        }
        unsafe extern "system" fn InsertEntry<Identity: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, before: u32, factory: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationProxyFactoryMapping_Impl::InsertEntry(this, core::mem::transmute_copy(&before), core::mem::transmute_copy(&factory)).into()
            }
        }
        unsafe extern "system" fn RemoveEntry<Identity: IUIAutomationProxyFactoryMapping_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32) -> windows_core::HRESULT {
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
            Count: Count::<Identity, OFFSET>,
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
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IUIAutomationProxyFactoryMapping {}
windows_core::imp::define_interface!(IUIAutomationRangeValuePattern, IUIAutomationRangeValuePattern_Vtbl, 0x59213f4f_7346_49e5_b120_80555987a148);
windows_core::imp::interface_hierarchy!(IUIAutomationRangeValuePattern, windows_core::IUnknown);
impl IUIAutomationRangeValuePattern {
    pub unsafe fn SetValue(&self, val: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), val) }
    }
    pub unsafe fn CurrentValue(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentIsReadOnly(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentIsReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentMaximum(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentMaximum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentMinimum(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentMinimum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentLargeChange(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentLargeChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentSmallChange(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentSmallChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedValue(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedIsReadOnly(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedIsReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedMaximum(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedMaximum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedMinimum(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedMinimum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedLargeChange(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedLargeChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedSmallChange(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedSmallChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationRangeValuePattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub CurrentValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CurrentIsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CurrentMaximum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CurrentMinimum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CurrentLargeChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CurrentSmallChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CachedValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CachedIsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CachedMaximum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CachedMinimum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CachedLargeChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CachedSmallChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
pub trait IUIAutomationRangeValuePattern_Impl: windows_core::IUnknownImpl {
    fn SetValue(&self, val: f64) -> windows_core::Result<()>;
    fn CurrentValue(&self) -> windows_core::Result<f64>;
    fn CurrentIsReadOnly(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentMaximum(&self) -> windows_core::Result<f64>;
    fn CurrentMinimum(&self) -> windows_core::Result<f64>;
    fn CurrentLargeChange(&self) -> windows_core::Result<f64>;
    fn CurrentSmallChange(&self) -> windows_core::Result<f64>;
    fn CachedValue(&self) -> windows_core::Result<f64>;
    fn CachedIsReadOnly(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedMaximum(&self) -> windows_core::Result<f64>;
    fn CachedMinimum(&self) -> windows_core::Result<f64>;
    fn CachedLargeChange(&self) -> windows_core::Result<f64>;
    fn CachedSmallChange(&self) -> windows_core::Result<f64>;
}
impl IUIAutomationRangeValuePattern_Vtbl {
    pub const fn new<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetValue<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationRangeValuePattern_Impl::SetValue(this, core::mem::transmute_copy(&val)).into()
            }
        }
        unsafe extern "system" fn CurrentValue<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationRangeValuePattern_Impl::CurrentValue(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentIsReadOnly<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationRangeValuePattern_Impl::CurrentIsReadOnly(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentMaximum<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationRangeValuePattern_Impl::CurrentMaximum(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentMinimum<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationRangeValuePattern_Impl::CurrentMinimum(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentLargeChange<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationRangeValuePattern_Impl::CurrentLargeChange(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentSmallChange<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationRangeValuePattern_Impl::CurrentSmallChange(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedValue<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationRangeValuePattern_Impl::CachedValue(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedIsReadOnly<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationRangeValuePattern_Impl::CachedIsReadOnly(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedMaximum<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationRangeValuePattern_Impl::CachedMaximum(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedMinimum<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationRangeValuePattern_Impl::CachedMinimum(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedLargeChange<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationRangeValuePattern_Impl::CachedLargeChange(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedSmallChange<Identity: IUIAutomationRangeValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationRangeValuePattern_Impl::CachedSmallChange(this) {
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
            CurrentValue: CurrentValue::<Identity, OFFSET>,
            CurrentIsReadOnly: CurrentIsReadOnly::<Identity, OFFSET>,
            CurrentMaximum: CurrentMaximum::<Identity, OFFSET>,
            CurrentMinimum: CurrentMinimum::<Identity, OFFSET>,
            CurrentLargeChange: CurrentLargeChange::<Identity, OFFSET>,
            CurrentSmallChange: CurrentSmallChange::<Identity, OFFSET>,
            CachedValue: CachedValue::<Identity, OFFSET>,
            CachedIsReadOnly: CachedIsReadOnly::<Identity, OFFSET>,
            CachedMaximum: CachedMaximum::<Identity, OFFSET>,
            CachedMinimum: CachedMinimum::<Identity, OFFSET>,
            CachedLargeChange: CachedLargeChange::<Identity, OFFSET>,
            CachedSmallChange: CachedSmallChange::<Identity, OFFSET>,
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
    pub unsafe fn Scroll(&self, horizontalamount: super::uiautomationcore::ScrollAmount, verticalamount: super::uiautomationcore::ScrollAmount) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Scroll)(windows_core::Interface::as_raw(self), horizontalamount, verticalamount) }
    }
    pub unsafe fn SetScrollPercent(&self, horizontalpercent: f64, verticalpercent: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetScrollPercent)(windows_core::Interface::as_raw(self), horizontalpercent, verticalpercent) }
    }
    pub unsafe fn CurrentHorizontalScrollPercent(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentHorizontalScrollPercent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentVerticalScrollPercent(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentVerticalScrollPercent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentHorizontalViewSize(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentHorizontalViewSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentVerticalViewSize(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentVerticalViewSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentHorizontallyScrollable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentHorizontallyScrollable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentVerticallyScrollable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentVerticallyScrollable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedHorizontalScrollPercent(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedHorizontalScrollPercent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedVerticalScrollPercent(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedVerticalScrollPercent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedHorizontalViewSize(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedHorizontalViewSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedVerticalViewSize(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedVerticalViewSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedHorizontallyScrollable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedHorizontallyScrollable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedVerticallyScrollable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedVerticallyScrollable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationScrollPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "uiautomationcore")]
    pub Scroll: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::ScrollAmount, super::uiautomationcore::ScrollAmount) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    Scroll: usize,
    pub SetScrollPercent: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub CurrentHorizontalScrollPercent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CurrentVerticalScrollPercent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CurrentHorizontalViewSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CurrentVerticalViewSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CurrentHorizontallyScrollable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CurrentVerticallyScrollable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CachedHorizontalScrollPercent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CachedVerticalScrollPercent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CachedHorizontalViewSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CachedVerticalViewSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CachedHorizontallyScrollable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CachedVerticallyScrollable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "uiautomationcore")]
pub trait IUIAutomationScrollPattern_Impl: windows_core::IUnknownImpl {
    fn Scroll(&self, horizontalamount: super::uiautomationcore::ScrollAmount, verticalamount: super::uiautomationcore::ScrollAmount) -> windows_core::Result<()>;
    fn SetScrollPercent(&self, horizontalpercent: f64, verticalpercent: f64) -> windows_core::Result<()>;
    fn CurrentHorizontalScrollPercent(&self) -> windows_core::Result<f64>;
    fn CurrentVerticalScrollPercent(&self) -> windows_core::Result<f64>;
    fn CurrentHorizontalViewSize(&self) -> windows_core::Result<f64>;
    fn CurrentVerticalViewSize(&self) -> windows_core::Result<f64>;
    fn CurrentHorizontallyScrollable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentVerticallyScrollable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedHorizontalScrollPercent(&self) -> windows_core::Result<f64>;
    fn CachedVerticalScrollPercent(&self) -> windows_core::Result<f64>;
    fn CachedHorizontalViewSize(&self) -> windows_core::Result<f64>;
    fn CachedVerticalViewSize(&self) -> windows_core::Result<f64>;
    fn CachedHorizontallyScrollable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedVerticallyScrollable(&self) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(feature = "uiautomationcore")]
impl IUIAutomationScrollPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Scroll<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizontalamount: super::uiautomationcore::ScrollAmount, verticalamount: super::uiautomationcore::ScrollAmount) -> windows_core::HRESULT {
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
        unsafe extern "system" fn CurrentHorizontalScrollPercent<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationScrollPattern_Impl::CurrentHorizontalScrollPercent(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentVerticalScrollPercent<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationScrollPattern_Impl::CurrentVerticalScrollPercent(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentHorizontalViewSize<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationScrollPattern_Impl::CurrentHorizontalViewSize(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentVerticalViewSize<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationScrollPattern_Impl::CurrentVerticalViewSize(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentHorizontallyScrollable<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationScrollPattern_Impl::CurrentHorizontallyScrollable(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentVerticallyScrollable<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationScrollPattern_Impl::CurrentVerticallyScrollable(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedHorizontalScrollPercent<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationScrollPattern_Impl::CachedHorizontalScrollPercent(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedVerticalScrollPercent<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationScrollPattern_Impl::CachedVerticalScrollPercent(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedHorizontalViewSize<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationScrollPattern_Impl::CachedHorizontalViewSize(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedVerticalViewSize<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationScrollPattern_Impl::CachedVerticalViewSize(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedHorizontallyScrollable<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationScrollPattern_Impl::CachedHorizontallyScrollable(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedVerticallyScrollable<Identity: IUIAutomationScrollPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationScrollPattern_Impl::CachedVerticallyScrollable(this) {
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
            CurrentHorizontalScrollPercent: CurrentHorizontalScrollPercent::<Identity, OFFSET>,
            CurrentVerticalScrollPercent: CurrentVerticalScrollPercent::<Identity, OFFSET>,
            CurrentHorizontalViewSize: CurrentHorizontalViewSize::<Identity, OFFSET>,
            CurrentVerticalViewSize: CurrentVerticalViewSize::<Identity, OFFSET>,
            CurrentHorizontallyScrollable: CurrentHorizontallyScrollable::<Identity, OFFSET>,
            CurrentVerticallyScrollable: CurrentVerticallyScrollable::<Identity, OFFSET>,
            CachedHorizontalScrollPercent: CachedHorizontalScrollPercent::<Identity, OFFSET>,
            CachedVerticalScrollPercent: CachedVerticalScrollPercent::<Identity, OFFSET>,
            CachedHorizontalViewSize: CachedHorizontalViewSize::<Identity, OFFSET>,
            CachedVerticalViewSize: CachedVerticalViewSize::<Identity, OFFSET>,
            CachedHorizontallyScrollable: CachedHorizontallyScrollable::<Identity, OFFSET>,
            CachedVerticallyScrollable: CachedVerticallyScrollable::<Identity, OFFSET>,
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
    pub unsafe fn CurrentIsSelected(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentIsSelected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentSelectionContainer(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentSelectionContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CachedIsSelected(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedIsSelected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedSelectionContainer(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedSelectionContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    pub CurrentIsSelected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CurrentSelectionContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedIsSelected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CachedSelectionContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAutomationSelectionItemPattern_Impl: windows_core::IUnknownImpl {
    fn Select(&self) -> windows_core::Result<()>;
    fn AddToSelection(&self) -> windows_core::Result<()>;
    fn RemoveFromSelection(&self) -> windows_core::Result<()>;
    fn CurrentIsSelected(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentSelectionContainer(&self) -> windows_core::Result<IUIAutomationElement>;
    fn CachedIsSelected(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedSelectionContainer(&self) -> windows_core::Result<IUIAutomationElement>;
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
        unsafe extern "system" fn CurrentIsSelected<Identity: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionItemPattern_Impl::CurrentIsSelected(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentSelectionContainer<Identity: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionItemPattern_Impl::CurrentSelectionContainer(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedIsSelected<Identity: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionItemPattern_Impl::CachedIsSelected(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedSelectionContainer<Identity: IUIAutomationSelectionItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionItemPattern_Impl::CachedSelectionContainer(this) {
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
            CurrentIsSelected: CurrentIsSelected::<Identity, OFFSET>,
            CurrentSelectionContainer: CurrentSelectionContainer::<Identity, OFFSET>,
            CachedIsSelected: CachedIsSelected::<Identity, OFFSET>,
            CachedSelectionContainer: CachedSelectionContainer::<Identity, OFFSET>,
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
            (windows_core::Interface::vtable(self).GetCurrentSelection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CurrentCanSelectMultiple(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentCanSelectMultiple)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentIsSelectionRequired(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentIsSelectionRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCachedSelection(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedSelection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CachedCanSelectMultiple(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedCanSelectMultiple)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedIsSelectionRequired(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedIsSelectionRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationSelectionPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetCurrentSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentCanSelectMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CurrentIsSelectionRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub GetCachedSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedCanSelectMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CachedIsSelectionRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IUIAutomationSelectionPattern_Impl: windows_core::IUnknownImpl {
    fn GetCurrentSelection(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn CurrentCanSelectMultiple(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentIsSelectionRequired(&self) -> windows_core::Result<windows_core::BOOL>;
    fn GetCachedSelection(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn CachedCanSelectMultiple(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedIsSelectionRequired(&self) -> windows_core::Result<windows_core::BOOL>;
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
        unsafe extern "system" fn CurrentCanSelectMultiple<Identity: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionPattern_Impl::CurrentCanSelectMultiple(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentIsSelectionRequired<Identity: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionPattern_Impl::CurrentIsSelectionRequired(this) {
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
        unsafe extern "system" fn CachedCanSelectMultiple<Identity: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionPattern_Impl::CachedCanSelectMultiple(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedIsSelectionRequired<Identity: IUIAutomationSelectionPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionPattern_Impl::CachedIsSelectionRequired(this) {
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
            CurrentCanSelectMultiple: CurrentCanSelectMultiple::<Identity, OFFSET>,
            CurrentIsSelectionRequired: CurrentIsSelectionRequired::<Identity, OFFSET>,
            GetCachedSelection: GetCachedSelection::<Identity, OFFSET>,
            CachedCanSelectMultiple: CachedCanSelectMultiple::<Identity, OFFSET>,
            CachedIsSelectionRequired: CachedIsSelectionRequired::<Identity, OFFSET>,
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
    pub unsafe fn CurrentFirstSelectedItem(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentFirstSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CurrentLastSelectedItem(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentLastSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CurrentCurrentSelectedItem(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentCurrentSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CurrentItemCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentItemCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedFirstSelectedItem(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedFirstSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CachedLastSelectedItem(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedLastSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CachedCurrentSelectedItem(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedCurrentSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CachedItemCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedItemCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationSelectionPattern2_Vtbl {
    pub base__: IUIAutomationSelectionPattern_Vtbl,
    pub CurrentFirstSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentLastSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentCurrentSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentItemCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedFirstSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedLastSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedCurrentSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedItemCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
pub trait IUIAutomationSelectionPattern2_Impl: IUIAutomationSelectionPattern_Impl {
    fn CurrentFirstSelectedItem(&self) -> windows_core::Result<IUIAutomationElement>;
    fn CurrentLastSelectedItem(&self) -> windows_core::Result<IUIAutomationElement>;
    fn CurrentCurrentSelectedItem(&self) -> windows_core::Result<IUIAutomationElement>;
    fn CurrentItemCount(&self) -> windows_core::Result<i32>;
    fn CachedFirstSelectedItem(&self) -> windows_core::Result<IUIAutomationElement>;
    fn CachedLastSelectedItem(&self) -> windows_core::Result<IUIAutomationElement>;
    fn CachedCurrentSelectedItem(&self) -> windows_core::Result<IUIAutomationElement>;
    fn CachedItemCount(&self) -> windows_core::Result<i32>;
}
impl IUIAutomationSelectionPattern2_Vtbl {
    pub const fn new<Identity: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentFirstSelectedItem<Identity: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionPattern2_Impl::CurrentFirstSelectedItem(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentLastSelectedItem<Identity: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionPattern2_Impl::CurrentLastSelectedItem(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentCurrentSelectedItem<Identity: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionPattern2_Impl::CurrentCurrentSelectedItem(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentItemCount<Identity: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionPattern2_Impl::CurrentItemCount(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedFirstSelectedItem<Identity: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionPattern2_Impl::CachedFirstSelectedItem(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedLastSelectedItem<Identity: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionPattern2_Impl::CachedLastSelectedItem(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedCurrentSelectedItem<Identity: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionPattern2_Impl::CachedCurrentSelectedItem(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedItemCount<Identity: IUIAutomationSelectionPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSelectionPattern2_Impl::CachedItemCount(this) {
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
            CurrentFirstSelectedItem: CurrentFirstSelectedItem::<Identity, OFFSET>,
            CurrentLastSelectedItem: CurrentLastSelectedItem::<Identity, OFFSET>,
            CurrentCurrentSelectedItem: CurrentCurrentSelectedItem::<Identity, OFFSET>,
            CurrentItemCount: CurrentItemCount::<Identity, OFFSET>,
            CachedFirstSelectedItem: CachedFirstSelectedItem::<Identity, OFFSET>,
            CachedLastSelectedItem: CachedLastSelectedItem::<Identity, OFFSET>,
            CachedCurrentSelectedItem: CachedCurrentSelectedItem::<Identity, OFFSET>,
            CachedItemCount: CachedItemCount::<Identity, OFFSET>,
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
    pub unsafe fn CurrentFormula(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentFormula)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetCurrentAnnotationObjects(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentAnnotationObjects)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetCurrentAnnotationTypes(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentAnnotationTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedFormula(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedFormula)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetCachedAnnotationObjects(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedAnnotationObjects)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetCachedAnnotationTypes(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
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
    pub CurrentFormula: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentAnnotationObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub GetCurrentAnnotationTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetCurrentAnnotationTypes: usize,
    pub CachedFormula: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCachedAnnotationObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub GetCachedAnnotationTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetCachedAnnotationTypes: usize,
}
#[cfg(feature = "oaidl")]
pub trait IUIAutomationSpreadsheetItemPattern_Impl: windows_core::IUnknownImpl {
    fn CurrentFormula(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetCurrentAnnotationObjects(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn GetCurrentAnnotationTypes(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn CachedFormula(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetCachedAnnotationObjects(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn GetCachedAnnotationTypes(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
}
#[cfg(feature = "oaidl")]
impl IUIAutomationSpreadsheetItemPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentFormula<Identity: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSpreadsheetItemPattern_Impl::CurrentFormula(this) {
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
        unsafe extern "system" fn GetCurrentAnnotationTypes<Identity: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
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
        unsafe extern "system" fn CachedFormula<Identity: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationSpreadsheetItemPattern_Impl::CachedFormula(this) {
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
        unsafe extern "system" fn GetCachedAnnotationTypes<Identity: IUIAutomationSpreadsheetItemPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
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
            CurrentFormula: CurrentFormula::<Identity, OFFSET>,
            GetCurrentAnnotationObjects: GetCurrentAnnotationObjects::<Identity, OFFSET>,
            GetCurrentAnnotationTypes: GetCurrentAnnotationTypes::<Identity, OFFSET>,
            CachedFormula: CachedFormula::<Identity, OFFSET>,
            GetCachedAnnotationObjects: GetCachedAnnotationObjects::<Identity, OFFSET>,
            GetCachedAnnotationTypes: GetCachedAnnotationTypes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationSpreadsheetItemPattern as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IUIAutomationSpreadsheetItemPattern {}
windows_core::imp::define_interface!(IUIAutomationSpreadsheetPattern, IUIAutomationSpreadsheetPattern_Vtbl, 0x7517a7c8_faae_4de9_9f08_29b91e8595c1);
windows_core::imp::interface_hierarchy!(IUIAutomationSpreadsheetPattern, windows_core::IUnknown);
impl IUIAutomationSpreadsheetPattern {
    pub unsafe fn GetItemByName(&self, name: &windows_core::BSTR) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemByName)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(name), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore"))]
    pub unsafe fn HandleStructureChangedEvent<P0>(&self, sender: P0, changetype: super::uiautomationcore::StructureChangeType, runtimeid: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT
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
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore"))]
    pub HandleStructureChangedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::uiautomationcore::StructureChangeType, *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "uiautomationcore")))]
    HandleStructureChangedEvent: usize,
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore"))]
pub trait IUIAutomationStructureChangedEventHandler_Impl: windows_core::IUnknownImpl {
    fn HandleStructureChangedEvent(&self, sender: windows_core::Ref<IUIAutomationElement>, changetype: super::uiautomationcore::StructureChangeType, runtimeid: *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore"))]
impl IUIAutomationStructureChangedEventHandler_Vtbl {
    pub const fn new<Identity: IUIAutomationStructureChangedEventHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HandleStructureChangedEvent<Identity: IUIAutomationStructureChangedEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, changetype: super::uiautomationcore::StructureChangeType, runtimeid: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
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
#[cfg(all(feature = "oaidl", feature = "uiautomationcore"))]
impl windows_core::RuntimeName for IUIAutomationStructureChangedEventHandler {}
windows_core::imp::define_interface!(IUIAutomationStylesPattern, IUIAutomationStylesPattern_Vtbl, 0x85b5f0a2_bd79_484a_ad2b_388c9838d5fb);
windows_core::imp::interface_hierarchy!(IUIAutomationStylesPattern, windows_core::IUnknown);
impl IUIAutomationStylesPattern {
    pub unsafe fn CurrentStyleId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentStyleId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentStyleName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentStyleName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentFillColor(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentFillColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentFillPatternStyle(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentFillPatternStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentShape(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentShape)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentFillPatternColor(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentFillPatternColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentExtendedProperties(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentExtendedProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn GetCurrentExtendedPropertiesAsArray(&self, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetCurrentExtendedPropertiesAsArray)(windows_core::Interface::as_raw(self), propertyarray as _, propertycount as _) }
    }
    pub unsafe fn CachedStyleId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedStyleId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedStyleName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedStyleName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedFillColor(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedFillColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedFillPatternStyle(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedFillPatternStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedShape(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedShape)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedFillPatternColor(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedFillPatternColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedExtendedProperties(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedExtendedProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
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
    pub CurrentStyleId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentStyleName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentFillColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentFillPatternStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentShape: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentFillPatternColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CurrentExtendedProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCurrentExtendedPropertiesAsArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut ExtendedProperty, *mut i32) -> windows_core::HRESULT,
    pub CachedStyleId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedStyleName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedFillColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedFillPatternStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedShape: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedFillPatternColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub CachedExtendedProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCachedExtendedPropertiesAsArray: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut ExtendedProperty, *mut i32) -> windows_core::HRESULT,
}
pub trait IUIAutomationStylesPattern_Impl: windows_core::IUnknownImpl {
    fn CurrentStyleId(&self) -> windows_core::Result<i32>;
    fn CurrentStyleName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentFillColor(&self) -> windows_core::Result<i32>;
    fn CurrentFillPatternStyle(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentShape(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentFillPatternColor(&self) -> windows_core::Result<i32>;
    fn CurrentExtendedProperties(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetCurrentExtendedPropertiesAsArray(&self, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> windows_core::Result<()>;
    fn CachedStyleId(&self) -> windows_core::Result<i32>;
    fn CachedStyleName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedFillColor(&self) -> windows_core::Result<i32>;
    fn CachedFillPatternStyle(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedShape(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedFillPatternColor(&self) -> windows_core::Result<i32>;
    fn CachedExtendedProperties(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetCachedExtendedPropertiesAsArray(&self, propertyarray: *mut *mut ExtendedProperty, propertycount: *mut i32) -> windows_core::Result<()>;
}
impl IUIAutomationStylesPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CurrentStyleId<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::CurrentStyleId(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentStyleName<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::CurrentStyleName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentFillColor<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::CurrentFillColor(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentFillPatternStyle<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::CurrentFillPatternStyle(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentShape<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::CurrentShape(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentFillPatternColor<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::CurrentFillPatternColor(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentExtendedProperties<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::CurrentExtendedProperties(this) {
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
        unsafe extern "system" fn CachedStyleId<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::CachedStyleId(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedStyleName<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::CachedStyleName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedFillColor<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::CachedFillColor(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedFillPatternStyle<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::CachedFillPatternStyle(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedShape<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::CachedShape(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedFillPatternColor<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::CachedFillPatternColor(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedExtendedProperties<Identity: IUIAutomationStylesPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationStylesPattern_Impl::CachedExtendedProperties(this) {
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
            CurrentStyleId: CurrentStyleId::<Identity, OFFSET>,
            CurrentStyleName: CurrentStyleName::<Identity, OFFSET>,
            CurrentFillColor: CurrentFillColor::<Identity, OFFSET>,
            CurrentFillPatternStyle: CurrentFillPatternStyle::<Identity, OFFSET>,
            CurrentShape: CurrentShape::<Identity, OFFSET>,
            CurrentFillPatternColor: CurrentFillPatternColor::<Identity, OFFSET>,
            CurrentExtendedProperties: CurrentExtendedProperties::<Identity, OFFSET>,
            GetCurrentExtendedPropertiesAsArray: GetCurrentExtendedPropertiesAsArray::<Identity, OFFSET>,
            CachedStyleId: CachedStyleId::<Identity, OFFSET>,
            CachedStyleName: CachedStyleName::<Identity, OFFSET>,
            CachedFillColor: CachedFillColor::<Identity, OFFSET>,
            CachedFillPatternStyle: CachedFillPatternStyle::<Identity, OFFSET>,
            CachedShape: CachedShape::<Identity, OFFSET>,
            CachedFillPatternColor: CachedFillPatternColor::<Identity, OFFSET>,
            CachedExtendedProperties: CachedExtendedProperties::<Identity, OFFSET>,
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
    pub unsafe fn StartListening(&self, inputtype: super::uiautomationcore::SynchronizedInputType) -> windows_core::HRESULT {
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
    pub StartListening: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::SynchronizedInputType) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    StartListening: usize,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "uiautomationcore")]
pub trait IUIAutomationSynchronizedInputPattern_Impl: windows_core::IUnknownImpl {
    fn StartListening(&self, inputtype: super::uiautomationcore::SynchronizedInputType) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
#[cfg(feature = "uiautomationcore")]
impl IUIAutomationSynchronizedInputPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationSynchronizedInputPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartListening<Identity: IUIAutomationSynchronizedInputPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputtype: super::uiautomationcore::SynchronizedInputType) -> windows_core::HRESULT {
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
            (windows_core::Interface::vtable(self).GetCurrentRowHeaderItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCurrentColumnHeaderItems(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentColumnHeaderItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCachedRowHeaderItems(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedRowHeaderItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCachedColumnHeaderItems(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedColumnHeaderItems)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
            (windows_core::Interface::vtable(self).GetCurrentRowHeaders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCurrentColumnHeaders(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCurrentColumnHeaders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn CurrentRowOrColumnMajor(&self) -> windows_core::Result<super::uiautomationcore::RowOrColumnMajor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentRowOrColumnMajor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetCachedRowHeaders(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedRowHeaders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCachedColumnHeaders(&self) -> windows_core::Result<IUIAutomationElementArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCachedColumnHeaders)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn CachedRowOrColumnMajor(&self) -> windows_core::Result<super::uiautomationcore::RowOrColumnMajor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedRowOrColumnMajor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    pub CurrentRowOrColumnMajor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::uiautomationcore::RowOrColumnMajor) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CurrentRowOrColumnMajor: usize,
    pub GetCachedRowHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCachedColumnHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub CachedRowOrColumnMajor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::uiautomationcore::RowOrColumnMajor) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CachedRowOrColumnMajor: usize,
}
#[cfg(feature = "uiautomationcore")]
pub trait IUIAutomationTablePattern_Impl: windows_core::IUnknownImpl {
    fn GetCurrentRowHeaders(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn GetCurrentColumnHeaders(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn CurrentRowOrColumnMajor(&self) -> windows_core::Result<super::uiautomationcore::RowOrColumnMajor>;
    fn GetCachedRowHeaders(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn GetCachedColumnHeaders(&self) -> windows_core::Result<IUIAutomationElementArray>;
    fn CachedRowOrColumnMajor(&self) -> windows_core::Result<super::uiautomationcore::RowOrColumnMajor>;
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
        unsafe extern "system" fn CurrentRowOrColumnMajor<Identity: IUIAutomationTablePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::uiautomationcore::RowOrColumnMajor) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTablePattern_Impl::CurrentRowOrColumnMajor(this) {
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
        unsafe extern "system" fn CachedRowOrColumnMajor<Identity: IUIAutomationTablePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::uiautomationcore::RowOrColumnMajor) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTablePattern_Impl::CachedRowOrColumnMajor(this) {
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
            CurrentRowOrColumnMajor: CurrentRowOrColumnMajor::<Identity, OFFSET>,
            GetCachedRowHeaders: GetCachedRowHeaders::<Identity, OFFSET>,
            GetCachedColumnHeaders: GetCachedColumnHeaders::<Identity, OFFSET>,
            CachedRowOrColumnMajor: CachedRowOrColumnMajor::<Identity, OFFSET>,
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
    pub unsafe fn TextContainer(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TextContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn TextRange(&self) -> windows_core::Result<IUIAutomationTextRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TextRange)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTextChildPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TextContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TextRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAutomationTextChildPattern_Impl: windows_core::IUnknownImpl {
    fn TextContainer(&self) -> windows_core::Result<IUIAutomationElement>;
    fn TextRange(&self) -> windows_core::Result<IUIAutomationTextRange>;
}
impl IUIAutomationTextChildPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationTextChildPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TextContainer<Identity: IUIAutomationTextChildPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, container: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextChildPattern_Impl::TextContainer(this) {
                    Ok(ok__) => {
                        container.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TextRange<Identity: IUIAutomationTextChildPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, range: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextChildPattern_Impl::TextRange(this) {
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
            TextContainer: TextContainer::<Identity, OFFSET>,
            TextRange: TextRange::<Identity, OFFSET>,
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
            (windows_core::Interface::vtable(self).GetActiveComposition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetConversionTarget(&self) -> windows_core::Result<IUIAutomationTextRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConversionTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
#[cfg(all(feature = "uiautomationcore", feature = "windef"))]
pub trait IUIAutomationTextEditPattern_Impl: IUIAutomationTextPattern_Impl {
    fn GetActiveComposition(&self) -> windows_core::Result<IUIAutomationTextRange>;
    fn GetConversionTarget(&self) -> windows_core::Result<IUIAutomationTextRange>;
}
#[cfg(all(feature = "uiautomationcore", feature = "windef"))]
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
#[cfg(all(feature = "uiautomationcore", feature = "windef"))]
impl windows_core::RuntimeName for IUIAutomationTextEditPattern {}
windows_core::imp::define_interface!(IUIAutomationTextEditTextChangedEventHandler, IUIAutomationTextEditTextChangedEventHandler_Vtbl, 0x92faa680_e704_4156_931a_e32d5bb38f3f);
windows_core::imp::interface_hierarchy!(IUIAutomationTextEditTextChangedEventHandler, windows_core::IUnknown);
impl IUIAutomationTextEditTextChangedEventHandler {
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore"))]
    pub unsafe fn HandleTextEditTextChangedEvent<P0>(&self, sender: P0, texteditchangetype: super::uiautomationcore::TextEditChangeType, eventstrings: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT
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
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore"))]
    pub HandleTextEditTextChangedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::uiautomationcore::TextEditChangeType, *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "uiautomationcore")))]
    HandleTextEditTextChangedEvent: usize,
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore"))]
pub trait IUIAutomationTextEditTextChangedEventHandler_Impl: windows_core::IUnknownImpl {
    fn HandleTextEditTextChangedEvent(&self, sender: windows_core::Ref<IUIAutomationElement>, texteditchangetype: super::uiautomationcore::TextEditChangeType, eventstrings: *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore"))]
impl IUIAutomationTextEditTextChangedEventHandler_Vtbl {
    pub const fn new<Identity: IUIAutomationTextEditTextChangedEventHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn HandleTextEditTextChangedEvent<Identity: IUIAutomationTextEditTextChangedEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, sender: *mut core::ffi::c_void, texteditchangetype: super::uiautomationcore::TextEditChangeType, eventstrings: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
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
#[cfg(all(feature = "oaidl", feature = "uiautomationcore"))]
impl windows_core::RuntimeName for IUIAutomationTextEditTextChangedEventHandler {}
windows_core::imp::define_interface!(IUIAutomationTextPattern, IUIAutomationTextPattern_Vtbl, 0x32eba289_3583_42c9_9c59_3b6d9a1e9b6a);
windows_core::imp::interface_hierarchy!(IUIAutomationTextPattern, windows_core::IUnknown);
impl IUIAutomationTextPattern {
    #[cfg(feature = "windef")]
    pub unsafe fn RangeFromPoint(&self, pt: super::windef::POINT) -> windows_core::Result<IUIAutomationTextRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RangeFromPoint)(windows_core::Interface::as_raw(self), core::mem::transmute(pt), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RangeFromChild<P0>(&self, child: P0) -> windows_core::Result<IUIAutomationTextRange>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RangeFromChild)(windows_core::Interface::as_raw(self), child.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetSelection(&self) -> windows_core::Result<IUIAutomationTextRangeArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetVisibleRanges(&self) -> windows_core::Result<IUIAutomationTextRangeArray> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVisibleRanges)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DocumentRange(&self) -> windows_core::Result<IUIAutomationTextRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DocumentRange)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn SupportedTextSelection(&self) -> windows_core::Result<super::uiautomationcore::SupportedTextSelection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedTextSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTextPattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub RangeFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::POINT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    RangeFromPoint: usize,
    pub RangeFromChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetVisibleRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DocumentRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub SupportedTextSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::uiautomationcore::SupportedTextSelection) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    SupportedTextSelection: usize,
}
#[cfg(all(feature = "uiautomationcore", feature = "windef"))]
pub trait IUIAutomationTextPattern_Impl: windows_core::IUnknownImpl {
    fn RangeFromPoint(&self, pt: &super::windef::POINT) -> windows_core::Result<IUIAutomationTextRange>;
    fn RangeFromChild(&self, child: windows_core::Ref<IUIAutomationElement>) -> windows_core::Result<IUIAutomationTextRange>;
    fn GetSelection(&self) -> windows_core::Result<IUIAutomationTextRangeArray>;
    fn GetVisibleRanges(&self) -> windows_core::Result<IUIAutomationTextRangeArray>;
    fn DocumentRange(&self) -> windows_core::Result<IUIAutomationTextRange>;
    fn SupportedTextSelection(&self) -> windows_core::Result<super::uiautomationcore::SupportedTextSelection>;
}
#[cfg(all(feature = "uiautomationcore", feature = "windef"))]
impl IUIAutomationTextPattern_Vtbl {
    pub const fn new<Identity: IUIAutomationTextPattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RangeFromPoint<Identity: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pt: super::windef::POINT, range: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn DocumentRange<Identity: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, range: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextPattern_Impl::DocumentRange(this) {
                    Ok(ok__) => {
                        range.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportedTextSelection<Identity: IUIAutomationTextPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, supportedtextselection: *mut super::uiautomationcore::SupportedTextSelection) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextPattern_Impl::SupportedTextSelection(this) {
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
            DocumentRange: DocumentRange::<Identity, OFFSET>,
            SupportedTextSelection: SupportedTextSelection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationTextPattern as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "uiautomationcore", feature = "windef"))]
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
            (windows_core::Interface::vtable(self).RangeFromAnnotation)(windows_core::Interface::as_raw(self), annotation.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCaretRange(&self, isactive: *mut windows_core::BOOL) -> windows_core::Result<IUIAutomationTextRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCaretRange)(windows_core::Interface::as_raw(self), isactive as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
#[cfg(all(feature = "uiautomationcore", feature = "windef"))]
pub trait IUIAutomationTextPattern2_Impl: IUIAutomationTextPattern_Impl {
    fn RangeFromAnnotation(&self, annotation: windows_core::Ref<IUIAutomationElement>) -> windows_core::Result<IUIAutomationTextRange>;
    fn GetCaretRange(&self, isactive: *mut windows_core::BOOL) -> windows_core::Result<IUIAutomationTextRange>;
}
#[cfg(all(feature = "uiautomationcore", feature = "windef"))]
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
#[cfg(all(feature = "uiautomationcore", feature = "windef"))]
impl windows_core::RuntimeName for IUIAutomationTextPattern2 {}
windows_core::imp::define_interface!(IUIAutomationTextRange, IUIAutomationTextRange_Vtbl, 0xa543cc6a_f4ae_494b_8239_c814481187a8);
windows_core::imp::interface_hierarchy!(IUIAutomationTextRange, windows_core::IUnknown);
impl IUIAutomationTextRange {
    pub unsafe fn Clone(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Clone)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    pub unsafe fn CompareEndpoints<P1>(&self, srcendpoint: super::uiautomationcore::TextPatternRangeEndpoint, range: P1, targetendpoint: super::uiautomationcore::TextPatternRangeEndpoint) -> windows_core::Result<i32>
    where
        P1: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CompareEndpoints)(windows_core::Interface::as_raw(self), srcendpoint, range.param().abi(), targetendpoint, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn ExpandToEnclosingUnit(&self, textunit: super::uiautomationcore::TextUnit) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ExpandToEnclosingUnit)(windows_core::Interface::as_raw(self), textunit) }
    }
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn FindAttribute(&self, attr: super::uiautomationcore::TEXTATTRIBUTEID, val: &super::oaidl::VARIANT, backward: bool) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindAttribute)(windows_core::Interface::as_raw(self), attr, core::mem::transmute_copy(val), backward.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindText(&self, text: &windows_core::BSTR, backward: bool, ignorecase: bool) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(text), backward.into(), ignorecase.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetAttributeValue(&self, attr: super::uiautomationcore::TEXTATTRIBUTEID) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttributeValue)(windows_core::Interface::as_raw(self), attr, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetBoundingRectangles(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBoundingRectangles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetEnclosingElement(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEnclosingElement)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetText(&self, maxlength: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetText)(windows_core::Interface::as_raw(self), maxlength, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn Move(&self, unit: super::uiautomationcore::TextUnit, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Move)(windows_core::Interface::as_raw(self), unit, count, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn MoveEndpointByUnit(&self, endpoint: super::uiautomationcore::TextPatternRangeEndpoint, unit: super::uiautomationcore::TextUnit, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveEndpointByUnit)(windows_core::Interface::as_raw(self), endpoint, unit, count, &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn MoveEndpointByRange<P1>(&self, srcendpoint: super::uiautomationcore::TextPatternRangeEndpoint, range: P1, targetendpoint: super::uiautomationcore::TextPatternRangeEndpoint) -> windows_core::HRESULT
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
            (windows_core::Interface::vtable(self).GetChildren)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    pub CompareEndpoints: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::TextPatternRangeEndpoint, *mut core::ffi::c_void, super::uiautomationcore::TextPatternRangeEndpoint, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CompareEndpoints: usize,
    #[cfg(feature = "uiautomationcore")]
    pub ExpandToEnclosingUnit: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::TextUnit) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    ExpandToEnclosingUnit: usize,
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
    pub FindAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::TEXTATTRIBUTEID, super::oaidl::VARIANT, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase")))]
    FindAttribute: usize,
    pub FindText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
    pub GetAttributeValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::TEXTATTRIBUTEID, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase")))]
    GetAttributeValue: usize,
    #[cfg(feature = "oaidl")]
    pub GetBoundingRectangles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetBoundingRectangles: usize,
    pub GetEnclosingElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub Move: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::TextUnit, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    Move: usize,
    #[cfg(feature = "uiautomationcore")]
    pub MoveEndpointByUnit: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::TextPatternRangeEndpoint, super::uiautomationcore::TextUnit, i32, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    MoveEndpointByUnit: usize,
    #[cfg(feature = "uiautomationcore")]
    pub MoveEndpointByRange: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::TextPatternRangeEndpoint, *mut core::ffi::c_void, super::uiautomationcore::TextPatternRangeEndpoint) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    MoveEndpointByRange: usize,
    pub Select: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddToSelection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveFromSelection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ScrollIntoView: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    pub GetChildren: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationTextRange_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<IUIAutomationTextRange>;
    fn Compare(&self, range: windows_core::Ref<IUIAutomationTextRange>) -> windows_core::Result<windows_core::BOOL>;
    fn CompareEndpoints(&self, srcendpoint: super::uiautomationcore::TextPatternRangeEndpoint, range: windows_core::Ref<IUIAutomationTextRange>, targetendpoint: super::uiautomationcore::TextPatternRangeEndpoint) -> windows_core::Result<i32>;
    fn ExpandToEnclosingUnit(&self, textunit: super::uiautomationcore::TextUnit) -> windows_core::Result<()>;
    fn FindAttribute(&self, attr: super::uiautomationcore::TEXTATTRIBUTEID, val: &super::oaidl::VARIANT, backward: windows_core::BOOL) -> windows_core::Result<IUIAutomationTextRange>;
    fn FindText(&self, text: &windows_core::BSTR, backward: windows_core::BOOL, ignorecase: windows_core::BOOL) -> windows_core::Result<IUIAutomationTextRange>;
    fn GetAttributeValue(&self, attr: super::uiautomationcore::TEXTATTRIBUTEID) -> windows_core::Result<super::oaidl::VARIANT>;
    fn GetBoundingRectangles(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn GetEnclosingElement(&self) -> windows_core::Result<IUIAutomationElement>;
    fn GetText(&self, maxlength: i32) -> windows_core::Result<windows_core::BSTR>;
    fn Move(&self, unit: super::uiautomationcore::TextUnit, count: i32) -> windows_core::Result<i32>;
    fn MoveEndpointByUnit(&self, endpoint: super::uiautomationcore::TextPatternRangeEndpoint, unit: super::uiautomationcore::TextUnit, count: i32) -> windows_core::Result<i32>;
    fn MoveEndpointByRange(&self, srcendpoint: super::uiautomationcore::TextPatternRangeEndpoint, range: windows_core::Ref<IUIAutomationTextRange>, targetendpoint: super::uiautomationcore::TextPatternRangeEndpoint) -> windows_core::Result<()>;
    fn Select(&self) -> windows_core::Result<()>;
    fn AddToSelection(&self) -> windows_core::Result<()>;
    fn RemoveFromSelection(&self) -> windows_core::Result<()>;
    fn ScrollIntoView(&self, aligntotop: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetChildren(&self) -> windows_core::Result<IUIAutomationElementArray>;
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
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
        unsafe extern "system" fn CompareEndpoints<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, srcendpoint: super::uiautomationcore::TextPatternRangeEndpoint, range: *mut core::ffi::c_void, targetendpoint: super::uiautomationcore::TextPatternRangeEndpoint, compvalue: *mut i32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn ExpandToEnclosingUnit<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, textunit: super::uiautomationcore::TextUnit) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationTextRange_Impl::ExpandToEnclosingUnit(this, core::mem::transmute_copy(&textunit)).into()
            }
        }
        unsafe extern "system" fn FindAttribute<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attr: super::uiautomationcore::TEXTATTRIBUTEID, val: super::oaidl::VARIANT, backward: windows_core::BOOL, found: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn FindText<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut core::ffi::c_void, backward: windows_core::BOOL, ignorecase: windows_core::BOOL, found: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextRange_Impl::FindText(this, core::mem::transmute(&text), core::mem::transmute_copy(&backward), core::mem::transmute_copy(&ignorecase)) {
                    Ok(ok__) => {
                        found.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAttributeValue<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attr: super::uiautomationcore::TEXTATTRIBUTEID, value: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetBoundingRectangles<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, boundingrects: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
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
        unsafe extern "system" fn Move<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: super::uiautomationcore::TextUnit, count: i32, moved: *mut i32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn MoveEndpointByUnit<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endpoint: super::uiautomationcore::TextPatternRangeEndpoint, unit: super::uiautomationcore::TextUnit, count: i32, moved: *mut i32) -> windows_core::HRESULT {
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
        unsafe extern "system" fn MoveEndpointByRange<Identity: IUIAutomationTextRange_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, srcendpoint: super::uiautomationcore::TextPatternRangeEndpoint, range: *mut core::ffi::c_void, targetendpoint: super::uiautomationcore::TextPatternRangeEndpoint) -> windows_core::HRESULT {
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
            FindText: FindText::<Identity, OFFSET>,
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
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationTextRange2_Impl: IUIAutomationTextRange_Impl {
    fn ShowContextMenu(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
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
            (windows_core::Interface::vtable(self).GetEnclosingElementBuildCache)(windows_core::Interface::as_raw(self), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetChildrenBuildCache<P0>(&self, cacherequest: P0) -> windows_core::Result<IUIAutomationElementArray>
    where
        P0: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetChildrenBuildCache)(windows_core::Interface::as_raw(self), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore"))]
    pub unsafe fn GetAttributeValues(&self, attributeids: *const super::uiautomationcore::TEXTATTRIBUTEID, attributeidcount: i32) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttributeValues)(windows_core::Interface::as_raw(self), attributeids, attributeidcount, &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTextRange3_Vtbl {
    pub base__: IUIAutomationTextRange2_Vtbl,
    pub GetEnclosingElementBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetChildrenBuildCache: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "uiautomationcore"))]
    pub GetAttributeValues: unsafe extern "system" fn(*mut core::ffi::c_void, *const super::uiautomationcore::TEXTATTRIBUTEID, i32, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "uiautomationcore")))]
    GetAttributeValues: usize,
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
pub trait IUIAutomationTextRange3_Impl: IUIAutomationTextRange2_Impl {
    fn GetEnclosingElementBuildCache(&self, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>) -> windows_core::Result<IUIAutomationElement>;
    fn GetChildrenBuildCache(&self, cacherequest: windows_core::Ref<IUIAutomationCacheRequest>) -> windows_core::Result<IUIAutomationElementArray>;
    fn GetAttributeValues(&self, attributeids: *const super::uiautomationcore::TEXTATTRIBUTEID, attributeidcount: i32) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
}
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
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
        unsafe extern "system" fn GetAttributeValues<Identity: IUIAutomationTextRange3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributeids: *const super::uiautomationcore::TEXTATTRIBUTEID, attributeidcount: i32, attributevalues: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
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
#[cfg(all(feature = "oaidl", feature = "uiautomationcore", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IUIAutomationTextRange3 {}
windows_core::imp::define_interface!(IUIAutomationTextRangeArray, IUIAutomationTextRangeArray_Vtbl, 0xce4ae76a_e717_4c98_81ea_47371d028eb6);
windows_core::imp::interface_hierarchy!(IUIAutomationTextRangeArray, windows_core::IUnknown);
impl IUIAutomationTextRangeArray {
    pub unsafe fn Length(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Length)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetElement(&self, index: i32) -> windows_core::Result<IUIAutomationTextRange> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetElement)(windows_core::Interface::as_raw(self), index, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTextRangeArray_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Length: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetElement: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAutomationTextRangeArray_Impl: windows_core::IUnknownImpl {
    fn Length(&self) -> windows_core::Result<i32>;
    fn GetElement(&self, index: i32) -> windows_core::Result<IUIAutomationTextRange>;
}
impl IUIAutomationTextRangeArray_Vtbl {
    pub const fn new<Identity: IUIAutomationTextRangeArray_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Length<Identity: IUIAutomationTextRangeArray_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, length: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTextRangeArray_Impl::Length(this) {
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
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Length: Length::<Identity, OFFSET>, GetElement: GetElement::<Identity, OFFSET> }
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
    pub unsafe fn CurrentToggleState(&self) -> windows_core::Result<super::uiautomationcore::ToggleState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentToggleState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn CachedToggleState(&self) -> windows_core::Result<super::uiautomationcore::ToggleState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedToggleState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTogglePattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Toggle: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub CurrentToggleState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::uiautomationcore::ToggleState) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CurrentToggleState: usize,
    #[cfg(feature = "uiautomationcore")]
    pub CachedToggleState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::uiautomationcore::ToggleState) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CachedToggleState: usize,
}
#[cfg(feature = "uiautomationcore")]
pub trait IUIAutomationTogglePattern_Impl: windows_core::IUnknownImpl {
    fn Toggle(&self) -> windows_core::Result<()>;
    fn CurrentToggleState(&self) -> windows_core::Result<super::uiautomationcore::ToggleState>;
    fn CachedToggleState(&self) -> windows_core::Result<super::uiautomationcore::ToggleState>;
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
        unsafe extern "system" fn CurrentToggleState<Identity: IUIAutomationTogglePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::uiautomationcore::ToggleState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTogglePattern_Impl::CurrentToggleState(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedToggleState<Identity: IUIAutomationTogglePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::uiautomationcore::ToggleState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTogglePattern_Impl::CachedToggleState(this) {
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
            CurrentToggleState: CurrentToggleState::<Identity, OFFSET>,
            CachedToggleState: CachedToggleState::<Identity, OFFSET>,
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
    pub unsafe fn CurrentCanMove(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentCanMove)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentCanResize(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentCanResize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentCanRotate(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentCanRotate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedCanMove(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedCanMove)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedCanResize(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedCanResize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedCanRotate(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedCanRotate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    pub CurrentCanMove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CurrentCanResize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CurrentCanRotate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CachedCanMove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CachedCanResize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CachedCanRotate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IUIAutomationTransformPattern_Impl: windows_core::IUnknownImpl {
    fn Move(&self, x: f64, y: f64) -> windows_core::Result<()>;
    fn Resize(&self, width: f64, height: f64) -> windows_core::Result<()>;
    fn Rotate(&self, degrees: f64) -> windows_core::Result<()>;
    fn CurrentCanMove(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentCanResize(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentCanRotate(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedCanMove(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedCanResize(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedCanRotate(&self) -> windows_core::Result<windows_core::BOOL>;
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
        unsafe extern "system" fn CurrentCanMove<Identity: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern_Impl::CurrentCanMove(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentCanResize<Identity: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern_Impl::CurrentCanResize(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentCanRotate<Identity: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern_Impl::CurrentCanRotate(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedCanMove<Identity: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern_Impl::CachedCanMove(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedCanResize<Identity: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern_Impl::CachedCanResize(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedCanRotate<Identity: IUIAutomationTransformPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern_Impl::CachedCanRotate(this) {
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
            CurrentCanMove: CurrentCanMove::<Identity, OFFSET>,
            CurrentCanResize: CurrentCanResize::<Identity, OFFSET>,
            CurrentCanRotate: CurrentCanRotate::<Identity, OFFSET>,
            CachedCanMove: CachedCanMove::<Identity, OFFSET>,
            CachedCanResize: CachedCanResize::<Identity, OFFSET>,
            CachedCanRotate: CachedCanRotate::<Identity, OFFSET>,
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
    pub unsafe fn ZoomByUnit(&self, zoomunit: super::uiautomationcore::ZoomUnit) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ZoomByUnit)(windows_core::Interface::as_raw(self), zoomunit) }
    }
    pub unsafe fn CurrentCanZoom(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentCanZoom)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedCanZoom(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedCanZoom)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentZoomLevel(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentZoomLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedZoomLevel(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedZoomLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentZoomMinimum(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentZoomMinimum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedZoomMinimum(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedZoomMinimum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentZoomMaximum(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentZoomMaximum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedZoomMaximum(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedZoomMaximum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationTransformPattern2_Vtbl {
    pub base__: IUIAutomationTransformPattern_Vtbl,
    pub Zoom: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub ZoomByUnit: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::ZoomUnit) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    ZoomByUnit: usize,
    pub CurrentCanZoom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CachedCanZoom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CurrentZoomLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CachedZoomLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CurrentZoomMinimum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CachedZoomMinimum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CurrentZoomMaximum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub CachedZoomMaximum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
#[cfg(feature = "uiautomationcore")]
pub trait IUIAutomationTransformPattern2_Impl: IUIAutomationTransformPattern_Impl {
    fn Zoom(&self, zoomvalue: f64) -> windows_core::Result<()>;
    fn ZoomByUnit(&self, zoomunit: super::uiautomationcore::ZoomUnit) -> windows_core::Result<()>;
    fn CurrentCanZoom(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedCanZoom(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentZoomLevel(&self) -> windows_core::Result<f64>;
    fn CachedZoomLevel(&self) -> windows_core::Result<f64>;
    fn CurrentZoomMinimum(&self) -> windows_core::Result<f64>;
    fn CachedZoomMinimum(&self) -> windows_core::Result<f64>;
    fn CurrentZoomMaximum(&self) -> windows_core::Result<f64>;
    fn CachedZoomMaximum(&self) -> windows_core::Result<f64>;
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
        unsafe extern "system" fn ZoomByUnit<Identity: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, zoomunit: super::uiautomationcore::ZoomUnit) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationTransformPattern2_Impl::ZoomByUnit(this, core::mem::transmute_copy(&zoomunit)).into()
            }
        }
        unsafe extern "system" fn CurrentCanZoom<Identity: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern2_Impl::CurrentCanZoom(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedCanZoom<Identity: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern2_Impl::CachedCanZoom(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentZoomLevel<Identity: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern2_Impl::CurrentZoomLevel(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedZoomLevel<Identity: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern2_Impl::CachedZoomLevel(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentZoomMinimum<Identity: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern2_Impl::CurrentZoomMinimum(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedZoomMinimum<Identity: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern2_Impl::CachedZoomMinimum(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentZoomMaximum<Identity: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern2_Impl::CurrentZoomMaximum(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedZoomMaximum<Identity: IUIAutomationTransformPattern2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTransformPattern2_Impl::CachedZoomMaximum(this) {
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
            CurrentCanZoom: CurrentCanZoom::<Identity, OFFSET>,
            CachedCanZoom: CachedCanZoom::<Identity, OFFSET>,
            CurrentZoomLevel: CurrentZoomLevel::<Identity, OFFSET>,
            CachedZoomLevel: CachedZoomLevel::<Identity, OFFSET>,
            CurrentZoomMinimum: CurrentZoomMinimum::<Identity, OFFSET>,
            CachedZoomMinimum: CachedZoomMinimum::<Identity, OFFSET>,
            CurrentZoomMaximum: CurrentZoomMaximum::<Identity, OFFSET>,
            CachedZoomMaximum: CachedZoomMaximum::<Identity, OFFSET>,
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
            (windows_core::Interface::vtable(self).GetParentElement)(windows_core::Interface::as_raw(self), element.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFirstChildElement<P0>(&self, element: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFirstChildElement)(windows_core::Interface::as_raw(self), element.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetLastChildElement<P0>(&self, element: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastChildElement)(windows_core::Interface::as_raw(self), element.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetNextSiblingElement<P0>(&self, element: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNextSiblingElement)(windows_core::Interface::as_raw(self), element.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPreviousSiblingElement<P0>(&self, element: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPreviousSiblingElement)(windows_core::Interface::as_raw(self), element.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn NormalizeElement<P0>(&self, element: P0) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NormalizeElement)(windows_core::Interface::as_raw(self), element.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetParentElementBuildCache<P0, P1>(&self, element: P0, cacherequest: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetParentElementBuildCache)(windows_core::Interface::as_raw(self), element.param().abi(), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFirstChildElementBuildCache<P0, P1>(&self, element: P0, cacherequest: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFirstChildElementBuildCache)(windows_core::Interface::as_raw(self), element.param().abi(), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetLastChildElementBuildCache<P0, P1>(&self, element: P0, cacherequest: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetLastChildElementBuildCache)(windows_core::Interface::as_raw(self), element.param().abi(), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetNextSiblingElementBuildCache<P0, P1>(&self, element: P0, cacherequest: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetNextSiblingElementBuildCache)(windows_core::Interface::as_raw(self), element.param().abi(), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetPreviousSiblingElementBuildCache<P0, P1>(&self, element: P0, cacherequest: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPreviousSiblingElementBuildCache)(windows_core::Interface::as_raw(self), element.param().abi(), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn NormalizeElementBuildCache<P0, P1>(&self, element: P0, cacherequest: P1) -> windows_core::Result<IUIAutomationElement>
    where
        P0: windows_core::Param<IUIAutomationElement>,
        P1: windows_core::Param<IUIAutomationCacheRequest>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).NormalizeElementBuildCache)(windows_core::Interface::as_raw(self), element.param().abi(), cacherequest.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Condition(&self) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Condition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
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
    pub Condition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
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
    fn Condition(&self) -> windows_core::Result<IUIAutomationCondition>;
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
        unsafe extern "system" fn Condition<Identity: IUIAutomationTreeWalker_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, condition: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationTreeWalker_Impl::Condition(this) {
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
            Condition: Condition::<Identity, OFFSET>,
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
    pub unsafe fn CurrentValue(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CurrentIsReadOnly(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentIsReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedValue(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedValue)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn CachedIsReadOnly(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedIsReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationValuePattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentIsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CachedValue: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CachedIsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IUIAutomationValuePattern_Impl: windows_core::IUnknownImpl {
    fn SetValue(&self, val: &windows_core::BSTR) -> windows_core::Result<()>;
    fn CurrentValue(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CurrentIsReadOnly(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedValue(&self) -> windows_core::Result<windows_core::BSTR>;
    fn CachedIsReadOnly(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IUIAutomationValuePattern_Vtbl {
    pub const fn new<Identity: IUIAutomationValuePattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetValue<Identity: IUIAutomationValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationValuePattern_Impl::SetValue(this, core::mem::transmute(&val)).into()
            }
        }
        unsafe extern "system" fn CurrentValue<Identity: IUIAutomationValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationValuePattern_Impl::CurrentValue(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentIsReadOnly<Identity: IUIAutomationValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationValuePattern_Impl::CurrentIsReadOnly(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedValue<Identity: IUIAutomationValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationValuePattern_Impl::CachedValue(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedIsReadOnly<Identity: IUIAutomationValuePattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationValuePattern_Impl::CachedIsReadOnly(this) {
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
            CurrentValue: CurrentValue::<Identity, OFFSET>,
            CurrentIsReadOnly: CurrentIsReadOnly::<Identity, OFFSET>,
            CachedValue: CachedValue::<Identity, OFFSET>,
            CachedIsReadOnly: CachedIsReadOnly::<Identity, OFFSET>,
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
    pub unsafe fn SetWindowVisualState(&self, state: super::uiautomationcore::WindowVisualState) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetWindowVisualState)(windows_core::Interface::as_raw(self), state) }
    }
    pub unsafe fn CurrentCanMaximize(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentCanMaximize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentCanMinimize(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentCanMinimize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentIsModal(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentIsModal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CurrentIsTopmost(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentIsTopmost)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn CurrentWindowVisualState(&self) -> windows_core::Result<super::uiautomationcore::WindowVisualState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentWindowVisualState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn CurrentWindowInteractionState(&self) -> windows_core::Result<super::uiautomationcore::WindowInteractionState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentWindowInteractionState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedCanMaximize(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedCanMaximize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedCanMinimize(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedCanMinimize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedIsModal(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedIsModal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CachedIsTopmost(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedIsTopmost)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn CachedWindowVisualState(&self) -> windows_core::Result<super::uiautomationcore::WindowVisualState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedWindowVisualState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "uiautomationcore")]
    pub unsafe fn CachedWindowInteractionState(&self) -> windows_core::Result<super::uiautomationcore::WindowInteractionState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CachedWindowInteractionState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    pub SetWindowVisualState: unsafe extern "system" fn(*mut core::ffi::c_void, super::uiautomationcore::WindowVisualState) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    SetWindowVisualState: usize,
    pub CurrentCanMaximize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CurrentCanMinimize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CurrentIsModal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CurrentIsTopmost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub CurrentWindowVisualState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::uiautomationcore::WindowVisualState) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CurrentWindowVisualState: usize,
    #[cfg(feature = "uiautomationcore")]
    pub CurrentWindowInteractionState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::uiautomationcore::WindowInteractionState) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CurrentWindowInteractionState: usize,
    pub CachedCanMaximize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CachedCanMinimize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CachedIsModal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CachedIsTopmost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "uiautomationcore")]
    pub CachedWindowVisualState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::uiautomationcore::WindowVisualState) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CachedWindowVisualState: usize,
    #[cfg(feature = "uiautomationcore")]
    pub CachedWindowInteractionState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::uiautomationcore::WindowInteractionState) -> windows_core::HRESULT,
    #[cfg(not(feature = "uiautomationcore"))]
    CachedWindowInteractionState: usize,
}
#[cfg(feature = "uiautomationcore")]
pub trait IUIAutomationWindowPattern_Impl: windows_core::IUnknownImpl {
    fn Close(&self) -> windows_core::Result<()>;
    fn WaitForInputIdle(&self, milliseconds: i32) -> windows_core::Result<windows_core::BOOL>;
    fn SetWindowVisualState(&self, state: super::uiautomationcore::WindowVisualState) -> windows_core::Result<()>;
    fn CurrentCanMaximize(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentCanMinimize(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentIsModal(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentIsTopmost(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CurrentWindowVisualState(&self) -> windows_core::Result<super::uiautomationcore::WindowVisualState>;
    fn CurrentWindowInteractionState(&self) -> windows_core::Result<super::uiautomationcore::WindowInteractionState>;
    fn CachedCanMaximize(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedCanMinimize(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedIsModal(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedIsTopmost(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CachedWindowVisualState(&self) -> windows_core::Result<super::uiautomationcore::WindowVisualState>;
    fn CachedWindowInteractionState(&self) -> windows_core::Result<super::uiautomationcore::WindowInteractionState>;
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
        unsafe extern "system" fn SetWindowVisualState<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: super::uiautomationcore::WindowVisualState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationWindowPattern_Impl::SetWindowVisualState(this, core::mem::transmute_copy(&state)).into()
            }
        }
        unsafe extern "system" fn CurrentCanMaximize<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationWindowPattern_Impl::CurrentCanMaximize(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentCanMinimize<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationWindowPattern_Impl::CurrentCanMinimize(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentIsModal<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationWindowPattern_Impl::CurrentIsModal(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentIsTopmost<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationWindowPattern_Impl::CurrentIsTopmost(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentWindowVisualState<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::uiautomationcore::WindowVisualState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationWindowPattern_Impl::CurrentWindowVisualState(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentWindowInteractionState<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::uiautomationcore::WindowInteractionState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationWindowPattern_Impl::CurrentWindowInteractionState(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedCanMaximize<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationWindowPattern_Impl::CachedCanMaximize(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedCanMinimize<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationWindowPattern_Impl::CachedCanMinimize(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedIsModal<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationWindowPattern_Impl::CachedIsModal(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedIsTopmost<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationWindowPattern_Impl::CachedIsTopmost(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedWindowVisualState<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::uiautomationcore::WindowVisualState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationWindowPattern_Impl::CachedWindowVisualState(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CachedWindowInteractionState<Identity: IUIAutomationWindowPattern_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut super::uiautomationcore::WindowInteractionState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationWindowPattern_Impl::CachedWindowInteractionState(this) {
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
            CurrentCanMaximize: CurrentCanMaximize::<Identity, OFFSET>,
            CurrentCanMinimize: CurrentCanMinimize::<Identity, OFFSET>,
            CurrentIsModal: CurrentIsModal::<Identity, OFFSET>,
            CurrentIsTopmost: CurrentIsTopmost::<Identity, OFFSET>,
            CurrentWindowVisualState: CurrentWindowVisualState::<Identity, OFFSET>,
            CurrentWindowInteractionState: CurrentWindowInteractionState::<Identity, OFFSET>,
            CachedCanMaximize: CachedCanMaximize::<Identity, OFFSET>,
            CachedCanMinimize: CachedCanMinimize::<Identity, OFFSET>,
            CachedIsModal: CachedIsModal::<Identity, OFFSET>,
            CachedIsTopmost: CachedIsTopmost::<Identity, OFFSET>,
            CachedWindowVisualState: CachedWindowVisualState::<Identity, OFFSET>,
            CachedWindowInteractionState: CachedWindowInteractionState::<Identity, OFFSET>,
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UIA_HWND(pub *mut core::ffi::c_void);
impl Default for UIA_HWND {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
