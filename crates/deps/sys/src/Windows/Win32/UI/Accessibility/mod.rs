#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccNotifyTouchInteraction();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccSetRunningUtilityState();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn AccessibleChildren();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn AccessibleObjectFromEvent();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn AccessibleObjectFromPoint();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessibleObjectFromWindow();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateStdAccessibleObject();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateStdAccessibleProxyA();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateStdAccessibleProxyW();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn DockPattern_SetDockPosition();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn ExpandCollapsePattern_Collapse();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn ExpandCollapsePattern_Expand();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn GetOleaccVersionInfo();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRoleTextA();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRoleTextW();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStateTextA();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStateTextW();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn GridPattern_GetItem();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn InvokePattern_Invoke();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWinEventHookInstalled();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn ItemContainerPattern_FindItemByProperty();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn LegacyIAccessiblePattern_DoDefaultAction();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn LegacyIAccessiblePattern_GetIAccessible();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn LegacyIAccessiblePattern_Select();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LegacyIAccessiblePattern_SetValue();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LresultFromObject();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MultipleViewPattern_GetViewName();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn MultipleViewPattern_SetCurrentView();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NotifyWinEvent();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObjectFromLresult();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn RangeValuePattern_SetValue();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn RegisterPointerInputTarget();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn RegisterPointerInputTargetEx();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn ScrollItemPattern_ScrollIntoView();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn ScrollPattern_Scroll();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn ScrollPattern_SetScrollPercent();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn SelectionItemPattern_AddToSelection();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn SelectionItemPattern_RemoveFromSelection();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn SelectionItemPattern_Select();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWinEventHook();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn SynchronizedInputPattern_Cancel();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn SynchronizedInputPattern_StartListening();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn TextPattern_GetSelection();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn TextPattern_GetVisibleRanges();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextPattern_RangeFromChild();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextPattern_RangeFromPoint();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextPattern_get_DocumentRange();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextPattern_get_SupportedTextSelection();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextRange_AddToSelection();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextRange_Clone();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TextRange_Compare();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextRange_CompareEndpoints();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextRange_ExpandToEnclosingUnit();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn TextRange_FindAttribute();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TextRange_FindText();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn TextRange_GetAttributeValue();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn TextRange_GetBoundingRectangles();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn TextRange_GetChildren();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextRange_GetEnclosingElement();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TextRange_GetText();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextRange_Move();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextRange_MoveEndpointByRange();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextRange_MoveEndpointByUnit();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextRange_RemoveFromSelection();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TextRange_ScrollIntoView();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextRange_Select();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TogglePattern_Toggle();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TransformPattern_Move();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TransformPattern_Resize();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TransformPattern_Rotate();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn UiaAddEvent();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UiaClientsAreListening();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaDisconnectAllProviders();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaDisconnectProvider();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UiaEventAddWindow();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UiaEventRemoveWindow();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn UiaFind();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UiaGetErrorDescription();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaGetPatternProvider();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn UiaGetPropertyValue();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaGetReservedMixedAttributeValue();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaGetReservedNotSupportedValue();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaGetRootNode();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn UiaGetRuntimeId();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn UiaGetUpdatedCache();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn UiaHPatternObjectFromVariant();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn UiaHTextRangeFromVariant();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn UiaHUiaNodeFromVariant();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UiaHasServerSideProvider();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UiaHostProviderFromHwnd();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn UiaIAccessibleFromProvider();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaLookupId();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn UiaNavigate();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn UiaNodeFromFocus();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UiaNodeFromHandle();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn UiaNodeFromPoint();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaNodeFromProvider();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UiaNodeRelease();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UiaPatternRelease();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UiaProviderForNonClient();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaProviderFromIAccessible();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaRaiseActiveTextPositionChangedEvent();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaRaiseAsyncContentLoadedEvent();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaRaiseAutomationEvent();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn UiaRaiseAutomationPropertyChangedEvent();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn UiaRaiseChangesEvent();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UiaRaiseNotificationEvent();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaRaiseStructureChangedEvent();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn UiaRaiseTextEditTextChangedEvent();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn UiaRegisterProviderCallback();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaRemoveEvent();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UiaReturnRawElementProvider();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaSetFocus();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UiaTextRangeRelease();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnhookWinEvent();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn UnregisterPointerInputTarget();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn UnregisterPointerInputTargetEx();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ValuePattern_SetValue();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn VirtualizedItemPattern_Realize();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowFromAccessibleObject();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn WindowPattern_Close();
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn WindowPattern_SetWindowVisualState();
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowPattern_WaitForInputIdle();
}
