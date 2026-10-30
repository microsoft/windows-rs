#[cfg(feature = "uiautomationcore")]
#[inline]
pub unsafe fn DockPattern_SetDockPosition(hobj: HUIAPATTERNOBJECT, dockposition: super::DockPosition) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn DockPattern_SetDockPosition(hobj : HUIAPATTERNOBJECT, dockposition : super::DockPosition) -> windows_core::HRESULT);
    unsafe { DockPattern_SetDockPosition(hobj as _, dockposition) }
}
#[inline]
pub unsafe fn ExpandCollapsePattern_Collapse(hobj: HUIAPATTERNOBJECT) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn ExpandCollapsePattern_Collapse(hobj : HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    unsafe { ExpandCollapsePattern_Collapse(hobj as _) }
}
#[inline]
pub unsafe fn ExpandCollapsePattern_Expand(hobj: HUIAPATTERNOBJECT) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn ExpandCollapsePattern_Expand(hobj : HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    unsafe { ExpandCollapsePattern_Expand(hobj as _) }
}
#[inline]
pub unsafe fn GridPattern_GetItem(hobj: HUIAPATTERNOBJECT, row: i32, column: i32, presult: *mut HUIANODE) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn GridPattern_GetItem(hobj : HUIAPATTERNOBJECT, row : i32, column : i32, presult : *mut HUIANODE) -> windows_core::HRESULT);
    unsafe { GridPattern_GetItem(hobj as _, row, column, presult as _) }
}
#[inline]
pub unsafe fn InvokePattern_Invoke(hobj: HUIAPATTERNOBJECT) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn InvokePattern_Invoke(hobj : HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    unsafe { InvokePattern_Invoke(hobj as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn ItemContainerPattern_FindItemByProperty(hobj: HUIAPATTERNOBJECT, hnodestartafter: HUIANODE, propertyid: super::PROPERTYID, value: &super::VARIANT, pfound: *mut HUIANODE) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn ItemContainerPattern_FindItemByProperty(hobj : HUIAPATTERNOBJECT, hnodestartafter : HUIANODE, propertyid : super::PROPERTYID, value : super::VARIANT, pfound : *mut HUIANODE) -> windows_core::HRESULT);
    unsafe { ItemContainerPattern_FindItemByProperty(hobj as _, hnodestartafter as _, propertyid, core::mem::transmute_copy(value), pfound as _) }
}
#[inline]
pub unsafe fn LegacyIAccessiblePattern_DoDefaultAction(hobj: HUIAPATTERNOBJECT) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn LegacyIAccessiblePattern_DoDefaultAction(hobj : HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    unsafe { LegacyIAccessiblePattern_DoDefaultAction(hobj as _) }
}
#[cfg(all(feature = "oaidl", feature = "oleacc"))]
#[inline]
pub unsafe fn LegacyIAccessiblePattern_GetIAccessible(hobj: HUIAPATTERNOBJECT, paccessible: *mut Option<super::IAccessible>) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn LegacyIAccessiblePattern_GetIAccessible(hobj : HUIAPATTERNOBJECT, paccessible : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { LegacyIAccessiblePattern_GetIAccessible(hobj as _, paccessible as _) }
}
#[inline]
pub unsafe fn LegacyIAccessiblePattern_Select(hobj: HUIAPATTERNOBJECT, flagsselect: i32) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn LegacyIAccessiblePattern_Select(hobj : HUIAPATTERNOBJECT, flagsselect : i32) -> windows_core::HRESULT);
    unsafe { LegacyIAccessiblePattern_Select(hobj as _, flagsselect) }
}
#[inline]
pub unsafe fn LegacyIAccessiblePattern_SetValue<P1>(hobj: HUIAPATTERNOBJECT, szvalue: P1) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("uiautomationcore.dll" "system" fn LegacyIAccessiblePattern_SetValue(hobj : HUIAPATTERNOBJECT, szvalue : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { LegacyIAccessiblePattern_SetValue(hobj as _, szvalue.param().abi()) }
}
#[inline]
pub unsafe fn MultipleViewPattern_GetViewName(hobj: HUIAPATTERNOBJECT, viewid: i32, ppstr: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn MultipleViewPattern_GetViewName(hobj : HUIAPATTERNOBJECT, viewid : i32, ppstr : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { MultipleViewPattern_GetViewName(hobj as _, viewid, ppstr as _) }
}
#[inline]
pub unsafe fn MultipleViewPattern_SetCurrentView(hobj: HUIAPATTERNOBJECT, viewid: i32) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn MultipleViewPattern_SetCurrentView(hobj : HUIAPATTERNOBJECT, viewid : i32) -> windows_core::HRESULT);
    unsafe { MultipleViewPattern_SetCurrentView(hobj as _, viewid) }
}
#[inline]
pub unsafe fn RangeValuePattern_SetValue(hobj: HUIAPATTERNOBJECT, val: f64) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn RangeValuePattern_SetValue(hobj : HUIAPATTERNOBJECT, val : f64) -> windows_core::HRESULT);
    unsafe { RangeValuePattern_SetValue(hobj as _, val) }
}
#[inline]
pub unsafe fn ScrollItemPattern_ScrollIntoView(hobj: HUIAPATTERNOBJECT) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn ScrollItemPattern_ScrollIntoView(hobj : HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    unsafe { ScrollItemPattern_ScrollIntoView(hobj as _) }
}
#[cfg(feature = "uiautomationcore")]
#[inline]
pub unsafe fn ScrollPattern_Scroll(hobj: HUIAPATTERNOBJECT, horizontalamount: super::ScrollAmount, verticalamount: super::ScrollAmount) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn ScrollPattern_Scroll(hobj : HUIAPATTERNOBJECT, horizontalamount : super::ScrollAmount, verticalamount : super::ScrollAmount) -> windows_core::HRESULT);
    unsafe { ScrollPattern_Scroll(hobj as _, horizontalamount, verticalamount) }
}
#[inline]
pub unsafe fn ScrollPattern_SetScrollPercent(hobj: HUIAPATTERNOBJECT, horizontalpercent: f64, verticalpercent: f64) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn ScrollPattern_SetScrollPercent(hobj : HUIAPATTERNOBJECT, horizontalpercent : f64, verticalpercent : f64) -> windows_core::HRESULT);
    unsafe { ScrollPattern_SetScrollPercent(hobj as _, horizontalpercent, verticalpercent) }
}
#[inline]
pub unsafe fn SelectionItemPattern_AddToSelection(hobj: HUIAPATTERNOBJECT) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn SelectionItemPattern_AddToSelection(hobj : HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    unsafe { SelectionItemPattern_AddToSelection(hobj as _) }
}
#[inline]
pub unsafe fn SelectionItemPattern_RemoveFromSelection(hobj: HUIAPATTERNOBJECT) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn SelectionItemPattern_RemoveFromSelection(hobj : HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    unsafe { SelectionItemPattern_RemoveFromSelection(hobj as _) }
}
#[inline]
pub unsafe fn SelectionItemPattern_Select(hobj: HUIAPATTERNOBJECT) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn SelectionItemPattern_Select(hobj : HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    unsafe { SelectionItemPattern_Select(hobj as _) }
}
#[inline]
pub unsafe fn SynchronizedInputPattern_Cancel(hobj: HUIAPATTERNOBJECT) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn SynchronizedInputPattern_Cancel(hobj : HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    unsafe { SynchronizedInputPattern_Cancel(hobj as _) }
}
#[cfg(feature = "uiautomationcore")]
#[inline]
pub unsafe fn SynchronizedInputPattern_StartListening(hobj: HUIAPATTERNOBJECT, inputtype: super::SynchronizedInputType) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn SynchronizedInputPattern_StartListening(hobj : HUIAPATTERNOBJECT, inputtype : super::SynchronizedInputType) -> windows_core::HRESULT);
    unsafe { SynchronizedInputPattern_StartListening(hobj as _, inputtype) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TextPattern_GetSelection(hobj: HUIAPATTERNOBJECT, pretval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TextPattern_GetSelection(hobj : HUIAPATTERNOBJECT, pretval : *mut *mut super::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { TextPattern_GetSelection(hobj as _, pretval as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TextPattern_GetVisibleRanges(hobj: HUIAPATTERNOBJECT, pretval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TextPattern_GetVisibleRanges(hobj : HUIAPATTERNOBJECT, pretval : *mut *mut super::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { TextPattern_GetVisibleRanges(hobj as _, pretval as _) }
}
#[inline]
pub unsafe fn TextPattern_RangeFromChild(hobj: HUIAPATTERNOBJECT, hnodechild: HUIANODE, pretval: *mut HUIATEXTRANGE) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TextPattern_RangeFromChild(hobj : HUIAPATTERNOBJECT, hnodechild : HUIANODE, pretval : *mut HUIATEXTRANGE) -> windows_core::HRESULT);
    unsafe { TextPattern_RangeFromChild(hobj as _, hnodechild as _, pretval as _) }
}
#[cfg(feature = "uiautomationcore")]
#[inline]
pub unsafe fn TextPattern_RangeFromPoint(hobj: HUIAPATTERNOBJECT, point: super::UiaPoint, pretval: *mut HUIATEXTRANGE) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TextPattern_RangeFromPoint(hobj : HUIAPATTERNOBJECT, point : super::UiaPoint, pretval : *mut HUIATEXTRANGE) -> windows_core::HRESULT);
    unsafe { TextPattern_RangeFromPoint(hobj as _, point, pretval as _) }
}
#[inline]
pub unsafe fn TextPattern_get_DocumentRange(hobj: HUIAPATTERNOBJECT, pretval: *mut HUIATEXTRANGE) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TextPattern_get_DocumentRange(hobj : HUIAPATTERNOBJECT, pretval : *mut HUIATEXTRANGE) -> windows_core::HRESULT);
    unsafe { TextPattern_get_DocumentRange(hobj as _, pretval as _) }
}
#[cfg(feature = "uiautomationcore")]
#[inline]
pub unsafe fn TextPattern_get_SupportedTextSelection(hobj: HUIAPATTERNOBJECT, pretval: *mut super::SupportedTextSelection) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TextPattern_get_SupportedTextSelection(hobj : HUIAPATTERNOBJECT, pretval : *mut super::SupportedTextSelection) -> windows_core::HRESULT);
    unsafe { TextPattern_get_SupportedTextSelection(hobj as _, pretval as _) }
}
#[inline]
pub unsafe fn TextRange_AddToSelection(hobj: HUIATEXTRANGE) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TextRange_AddToSelection(hobj : HUIATEXTRANGE) -> windows_core::HRESULT);
    unsafe { TextRange_AddToSelection(hobj as _) }
}
#[inline]
pub unsafe fn TextRange_Clone(hobj: HUIATEXTRANGE, pretval: *mut HUIATEXTRANGE) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TextRange_Clone(hobj : HUIATEXTRANGE, pretval : *mut HUIATEXTRANGE) -> windows_core::HRESULT);
    unsafe { TextRange_Clone(hobj as _, pretval as _) }
}
#[inline]
pub unsafe fn TextRange_Compare(hobj: HUIATEXTRANGE, range: HUIATEXTRANGE, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TextRange_Compare(hobj : HUIATEXTRANGE, range : HUIATEXTRANGE, pretval : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { TextRange_Compare(hobj as _, range as _, pretval as _) }
}
#[cfg(feature = "uiautomationcore")]
#[inline]
pub unsafe fn TextRange_CompareEndpoints(hobj: HUIATEXTRANGE, endpoint: super::TextPatternRangeEndpoint, targetrange: HUIATEXTRANGE, targetendpoint: super::TextPatternRangeEndpoint, pretval: *mut i32) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TextRange_CompareEndpoints(hobj : HUIATEXTRANGE, endpoint : super::TextPatternRangeEndpoint, targetrange : HUIATEXTRANGE, targetendpoint : super::TextPatternRangeEndpoint, pretval : *mut i32) -> windows_core::HRESULT);
    unsafe { TextRange_CompareEndpoints(hobj as _, endpoint, targetrange as _, targetendpoint, pretval as _) }
}
#[cfg(feature = "uiautomationcore")]
#[inline]
pub unsafe fn TextRange_ExpandToEnclosingUnit(hobj: HUIATEXTRANGE, unit: super::TextUnit) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TextRange_ExpandToEnclosingUnit(hobj : HUIATEXTRANGE, unit : super::TextUnit) -> windows_core::HRESULT);
    unsafe { TextRange_ExpandToEnclosingUnit(hobj as _, unit) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn TextRange_FindAttribute(hobj: HUIATEXTRANGE, attributeid: super::TEXTATTRIBUTEID, val: &super::VARIANT, backward: bool, pretval: *mut HUIATEXTRANGE) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TextRange_FindAttribute(hobj : HUIATEXTRANGE, attributeid : super::TEXTATTRIBUTEID, val : super::VARIANT, backward : windows_core::BOOL, pretval : *mut HUIATEXTRANGE) -> windows_core::HRESULT);
    unsafe { TextRange_FindAttribute(hobj as _, attributeid, core::mem::transmute_copy(val), backward.into(), pretval as _) }
}
#[inline]
pub unsafe fn TextRange_FindText(hobj: HUIATEXTRANGE, text: &windows_core::BSTR, backward: bool, ignorecase: bool, pretval: *mut HUIATEXTRANGE) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TextRange_FindText(hobj : HUIATEXTRANGE, text : *mut core::ffi::c_void, backward : windows_core::BOOL, ignorecase : windows_core::BOOL, pretval : *mut HUIATEXTRANGE) -> windows_core::HRESULT);
    unsafe { TextRange_FindText(hobj as _, core::mem::transmute_copy(text), backward.into(), ignorecase.into(), pretval as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn TextRange_GetAttributeValue(hobj: HUIATEXTRANGE, attributeid: super::TEXTATTRIBUTEID, pretval: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TextRange_GetAttributeValue(hobj : HUIATEXTRANGE, attributeid : super::TEXTATTRIBUTEID, pretval : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { TextRange_GetAttributeValue(hobj as _, attributeid, pretval as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TextRange_GetBoundingRectangles(hobj: HUIATEXTRANGE, pretval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TextRange_GetBoundingRectangles(hobj : HUIATEXTRANGE, pretval : *mut *mut super::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { TextRange_GetBoundingRectangles(hobj as _, pretval as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn TextRange_GetChildren(hobj: HUIATEXTRANGE, pretval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TextRange_GetChildren(hobj : HUIATEXTRANGE, pretval : *mut *mut super::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { TextRange_GetChildren(hobj as _, pretval as _) }
}
#[inline]
pub unsafe fn TextRange_GetEnclosingElement(hobj: HUIATEXTRANGE, pretval: *mut HUIANODE) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TextRange_GetEnclosingElement(hobj : HUIATEXTRANGE, pretval : *mut HUIANODE) -> windows_core::HRESULT);
    unsafe { TextRange_GetEnclosingElement(hobj as _, pretval as _) }
}
#[inline]
pub unsafe fn TextRange_GetText(hobj: HUIATEXTRANGE, maxlength: i32, pretval: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TextRange_GetText(hobj : HUIATEXTRANGE, maxlength : i32, pretval : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { TextRange_GetText(hobj as _, maxlength, pretval as _) }
}
#[cfg(feature = "uiautomationcore")]
#[inline]
pub unsafe fn TextRange_Move(hobj: HUIATEXTRANGE, unit: super::TextUnit, count: i32, pretval: *mut i32) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TextRange_Move(hobj : HUIATEXTRANGE, unit : super::TextUnit, count : i32, pretval : *mut i32) -> windows_core::HRESULT);
    unsafe { TextRange_Move(hobj as _, unit, count, pretval as _) }
}
#[cfg(feature = "uiautomationcore")]
#[inline]
pub unsafe fn TextRange_MoveEndpointByRange(hobj: HUIATEXTRANGE, endpoint: super::TextPatternRangeEndpoint, targetrange: HUIATEXTRANGE, targetendpoint: super::TextPatternRangeEndpoint) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TextRange_MoveEndpointByRange(hobj : HUIATEXTRANGE, endpoint : super::TextPatternRangeEndpoint, targetrange : HUIATEXTRANGE, targetendpoint : super::TextPatternRangeEndpoint) -> windows_core::HRESULT);
    unsafe { TextRange_MoveEndpointByRange(hobj as _, endpoint, targetrange as _, targetendpoint) }
}
#[cfg(feature = "uiautomationcore")]
#[inline]
pub unsafe fn TextRange_MoveEndpointByUnit(hobj: HUIATEXTRANGE, endpoint: super::TextPatternRangeEndpoint, unit: super::TextUnit, count: i32, pretval: *mut i32) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TextRange_MoveEndpointByUnit(hobj : HUIATEXTRANGE, endpoint : super::TextPatternRangeEndpoint, unit : super::TextUnit, count : i32, pretval : *mut i32) -> windows_core::HRESULT);
    unsafe { TextRange_MoveEndpointByUnit(hobj as _, endpoint, unit, count, pretval as _) }
}
#[inline]
pub unsafe fn TextRange_RemoveFromSelection(hobj: HUIATEXTRANGE) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TextRange_RemoveFromSelection(hobj : HUIATEXTRANGE) -> windows_core::HRESULT);
    unsafe { TextRange_RemoveFromSelection(hobj as _) }
}
#[inline]
pub unsafe fn TextRange_ScrollIntoView(hobj: HUIATEXTRANGE, aligntotop: bool) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TextRange_ScrollIntoView(hobj : HUIATEXTRANGE, aligntotop : windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { TextRange_ScrollIntoView(hobj as _, aligntotop.into()) }
}
#[inline]
pub unsafe fn TextRange_Select(hobj: HUIATEXTRANGE) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TextRange_Select(hobj : HUIATEXTRANGE) -> windows_core::HRESULT);
    unsafe { TextRange_Select(hobj as _) }
}
#[inline]
pub unsafe fn TogglePattern_Toggle(hobj: HUIAPATTERNOBJECT) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TogglePattern_Toggle(hobj : HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    unsafe { TogglePattern_Toggle(hobj as _) }
}
#[inline]
pub unsafe fn TransformPattern_Move(hobj: HUIAPATTERNOBJECT, x: f64, y: f64) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TransformPattern_Move(hobj : HUIAPATTERNOBJECT, x : f64, y : f64) -> windows_core::HRESULT);
    unsafe { TransformPattern_Move(hobj as _, x, y) }
}
#[inline]
pub unsafe fn TransformPattern_Resize(hobj: HUIAPATTERNOBJECT, width: f64, height: f64) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TransformPattern_Resize(hobj : HUIAPATTERNOBJECT, width : f64, height : f64) -> windows_core::HRESULT);
    unsafe { TransformPattern_Resize(hobj as _, width, height) }
}
#[inline]
pub unsafe fn TransformPattern_Rotate(hobj: HUIAPATTERNOBJECT, degrees: f64) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn TransformPattern_Rotate(hobj : HUIAPATTERNOBJECT, degrees : f64) -> windows_core::HRESULT);
    unsafe { TransformPattern_Rotate(hobj as _, degrees) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationclient", feature = "uiautomationcore", feature = "winnt"))]
#[inline]
pub unsafe fn UiaAddEvent(hnode: HUIANODE, eventid: super::EVENTID, pcallback: UiaEventCallback, scope: super::TreeScope, pproperties: *mut super::PROPERTYID, cproperties: i32, prequest: *mut UiaCacheRequest, phevent: *mut HUIAEVENT) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaAddEvent(hnode : HUIANODE, eventid : super::EVENTID, pcallback : UiaEventCallback, scope : super::TreeScope, pproperties : *mut super::PROPERTYID, cproperties : i32, prequest : *mut UiaCacheRequest, phevent : *mut HUIAEVENT) -> windows_core::HRESULT);
    unsafe { UiaAddEvent(hnode as _, eventid, pcallback, scope, pproperties as _, cproperties, prequest as _, phevent as _) }
}
#[inline]
pub unsafe fn UiaClientsAreListening() -> windows_core::BOOL {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaClientsAreListening() -> windows_core::BOOL);
    unsafe { UiaClientsAreListening() }
}
#[inline]
pub unsafe fn UiaDisconnectAllProviders() -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaDisconnectAllProviders() -> windows_core::HRESULT);
    unsafe { UiaDisconnectAllProviders() }
}
#[cfg(feature = "uiautomationcore")]
#[inline]
pub unsafe fn UiaDisconnectProvider<P0>(pprovider: P0) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IRawElementProviderSimple>,
{
    windows_core::link!("uiautomationcore.dll" "system" fn UiaDisconnectProvider(pprovider : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { UiaDisconnectProvider(pprovider.param().abi()) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn UiaEventAddWindow(hevent: HUIAEVENT, hwnd: super::HWND) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaEventAddWindow(hevent : HUIAEVENT, hwnd : super::HWND) -> windows_core::HRESULT);
    unsafe { UiaEventAddWindow(hevent as _, hwnd as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn UiaEventRemoveWindow(hevent: HUIAEVENT, hwnd: super::HWND) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaEventRemoveWindow(hevent : HUIAEVENT, hwnd : super::HWND) -> windows_core::HRESULT);
    unsafe { UiaEventRemoveWindow(hevent as _, hwnd as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationclient", feature = "uiautomationcore", feature = "winnt"))]
#[inline]
pub unsafe fn UiaFind(hnode: HUIANODE, pparams: *mut UiaFindParams, prequest: *mut UiaCacheRequest, pprequesteddata: *mut *mut super::SAFEARRAY, ppoffsets: *mut *mut super::SAFEARRAY, pptreestructures: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaFind(hnode : HUIANODE, pparams : *mut UiaFindParams, prequest : *mut UiaCacheRequest, pprequesteddata : *mut *mut super::SAFEARRAY, ppoffsets : *mut *mut super::SAFEARRAY, pptreestructures : *mut *mut super::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { UiaFind(hnode as _, pparams as _, prequest as _, pprequesteddata as _, ppoffsets as _, pptreestructures as _) }
}
#[inline]
pub unsafe fn UiaGetErrorDescription(pdescription: *mut windows_core::BSTR) -> windows_core::BOOL {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaGetErrorDescription(pdescription : *mut *mut core::ffi::c_void) -> windows_core::BOOL);
    unsafe { UiaGetErrorDescription(pdescription as _) }
}
#[cfg(feature = "uiautomationcore")]
#[inline]
pub unsafe fn UiaGetPatternProvider(hnode: HUIANODE, patternid: super::PATTERNID, phobj: *mut HUIAPATTERNOBJECT) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaGetPatternProvider(hnode : HUIANODE, patternid : super::PATTERNID, phobj : *mut HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    unsafe { UiaGetPatternProvider(hnode as _, patternid, phobj as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn UiaGetPropertyValue(hnode: HUIANODE, propertyid: super::PROPERTYID, pvalue: *mut super::VARIANT) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaGetPropertyValue(hnode : HUIANODE, propertyid : super::PROPERTYID, pvalue : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { UiaGetPropertyValue(hnode as _, propertyid, pvalue as _) }
}
#[inline]
pub unsafe fn UiaGetReservedMixedAttributeValue(punkmixedattributevalue: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaGetReservedMixedAttributeValue(punkmixedattributevalue : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { UiaGetReservedMixedAttributeValue(punkmixedattributevalue as _) }
}
#[inline]
pub unsafe fn UiaGetReservedNotSupportedValue(punknotsupportedvalue: *mut Option<windows_core::IUnknown>) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaGetReservedNotSupportedValue(punknotsupportedvalue : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { UiaGetReservedNotSupportedValue(punknotsupportedvalue as _) }
}
#[inline]
pub unsafe fn UiaGetRootNode(phnode: *mut HUIANODE) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaGetRootNode(phnode : *mut HUIANODE) -> windows_core::HRESULT);
    unsafe { UiaGetRootNode(phnode as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
#[inline]
pub unsafe fn UiaGetRuntimeId(hnode: HUIANODE, pruntimeid: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaGetRuntimeId(hnode : HUIANODE, pruntimeid : *mut *mut super::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { UiaGetRuntimeId(hnode as _, pruntimeid as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationclient", feature = "uiautomationcore", feature = "winnt"))]
#[inline]
pub unsafe fn UiaGetUpdatedCache(hnode: HUIANODE, prequest: *mut UiaCacheRequest, normalizestate: NormalizeState, pnormalizecondition: *mut UiaCondition, pprequesteddata: *mut *mut super::SAFEARRAY, pptreestructure: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaGetUpdatedCache(hnode : HUIANODE, prequest : *mut UiaCacheRequest, normalizestate : NormalizeState, pnormalizecondition : *mut UiaCondition, pprequesteddata : *mut *mut super::SAFEARRAY, pptreestructure : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { UiaGetUpdatedCache(hnode as _, prequest as _, normalizestate, pnormalizecondition as _, pprequesteddata as _, pptreestructure as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn UiaHPatternObjectFromVariant(pvar: *mut super::VARIANT, phobj: *mut HUIAPATTERNOBJECT) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaHPatternObjectFromVariant(pvar : *mut super::VARIANT, phobj : *mut HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    unsafe { UiaHPatternObjectFromVariant(pvar as _, phobj as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn UiaHTextRangeFromVariant(pvar: *mut super::VARIANT, phtextrange: *mut HUIATEXTRANGE) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaHTextRangeFromVariant(pvar : *mut super::VARIANT, phtextrange : *mut HUIATEXTRANGE) -> windows_core::HRESULT);
    unsafe { UiaHTextRangeFromVariant(pvar as _, phtextrange as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn UiaHUiaNodeFromVariant(pvar: *mut super::VARIANT, phnode: *mut HUIANODE) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaHUiaNodeFromVariant(pvar : *mut super::VARIANT, phnode : *mut HUIANODE) -> windows_core::HRESULT);
    unsafe { UiaHUiaNodeFromVariant(pvar as _, phnode as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn UiaHasServerSideProvider(hwnd: super::HWND) -> windows_core::BOOL {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaHasServerSideProvider(hwnd : super::HWND) -> windows_core::BOOL);
    unsafe { UiaHasServerSideProvider(hwnd as _) }
}
#[cfg(all(feature = "uiautomationcore", feature = "windef"))]
#[inline]
pub unsafe fn UiaHostProviderFromHwnd(hwnd: super::HWND, ppprovider: *mut Option<super::IRawElementProviderSimple>) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaHostProviderFromHwnd(hwnd : super::HWND, ppprovider : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { UiaHostProviderFromHwnd(hwnd as _, ppprovider as _) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "oleacc", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn UiaIAccessibleFromProvider<P0>(pprovider: P0, dwflags: super::DWORD, ppaccessible: *mut Option<super::IAccessible>, pvarchild: *mut super::VARIANT) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IRawElementProviderSimple>,
{
    windows_core::link!("uiautomationcore.dll" "system" fn UiaIAccessibleFromProvider(pprovider : *mut core::ffi::c_void, dwflags : super::DWORD, ppaccessible : *mut *mut core::ffi::c_void, pvarchild : *mut super::VARIANT) -> windows_core::HRESULT);
    unsafe { UiaIAccessibleFromProvider(pprovider.param().abi(), dwflags, core::mem::transmute(ppaccessible), pvarchild) }
}
#[inline]
pub unsafe fn UiaLookupId(r#type: AutomationIdentifierType, pguid: *const windows_core::GUID) -> i32 {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaLookupId(r#type : AutomationIdentifierType, pguid : *const windows_core::GUID) -> i32);
    unsafe { UiaLookupId(r#type, pguid) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationclient", feature = "uiautomationcore", feature = "winnt"))]
#[inline]
pub unsafe fn UiaNavigate(hnode: HUIANODE, direction: super::NavigateDirection, pcondition: *mut UiaCondition, prequest: *mut UiaCacheRequest, pprequesteddata: *mut *mut super::SAFEARRAY, pptreestructure: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaNavigate(hnode : HUIANODE, direction : super::NavigateDirection, pcondition : *mut UiaCondition, prequest : *mut UiaCacheRequest, pprequesteddata : *mut *mut super::SAFEARRAY, pptreestructure : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { UiaNavigate(hnode as _, direction, pcondition as _, prequest as _, pprequesteddata as _, pptreestructure as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationclient", feature = "uiautomationcore", feature = "winnt"))]
#[inline]
pub unsafe fn UiaNodeFromFocus(prequest: *mut UiaCacheRequest, pprequesteddata: *mut *mut super::SAFEARRAY, pptreestructure: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaNodeFromFocus(prequest : *mut UiaCacheRequest, pprequesteddata : *mut *mut super::SAFEARRAY, pptreestructure : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { UiaNodeFromFocus(prequest as _, pprequesteddata as _, pptreestructure as _) }
}
#[cfg(feature = "windef")]
#[inline]
pub unsafe fn UiaNodeFromHandle(hwnd: super::HWND, phnode: *mut HUIANODE) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaNodeFromHandle(hwnd : super::HWND, phnode : *mut HUIANODE) -> windows_core::HRESULT);
    unsafe { UiaNodeFromHandle(hwnd as _, phnode as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationclient", feature = "uiautomationcore", feature = "winnt"))]
#[inline]
pub unsafe fn UiaNodeFromPoint(x: f64, y: f64, prequest: *mut UiaCacheRequest, pprequesteddata: *mut *mut super::SAFEARRAY, pptreestructure: *mut windows_core::BSTR) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaNodeFromPoint(x : f64, y : f64, prequest : *mut UiaCacheRequest, pprequesteddata : *mut *mut super::SAFEARRAY, pptreestructure : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { UiaNodeFromPoint(x, y, prequest as _, pprequesteddata as _, pptreestructure as _) }
}
#[cfg(feature = "uiautomationcore")]
#[inline]
pub unsafe fn UiaNodeFromProvider<P0>(pprovider: P0, phnode: *mut HUIANODE) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IRawElementProviderSimple>,
{
    windows_core::link!("uiautomationcore.dll" "system" fn UiaNodeFromProvider(pprovider : *mut core::ffi::c_void, phnode : *mut HUIANODE) -> windows_core::HRESULT);
    unsafe { UiaNodeFromProvider(pprovider.param().abi(), phnode as _) }
}
#[inline]
pub unsafe fn UiaNodeRelease(hnode: HUIANODE) -> windows_core::BOOL {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaNodeRelease(hnode : HUIANODE) -> windows_core::BOOL);
    unsafe { UiaNodeRelease(hnode as _) }
}
#[inline]
pub unsafe fn UiaPatternRelease(hobj: HUIAPATTERNOBJECT) -> windows_core::BOOL {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaPatternRelease(hobj : HUIAPATTERNOBJECT) -> windows_core::BOOL);
    unsafe { UiaPatternRelease(hobj as _) }
}
#[cfg(all(feature = "uiautomationcore", feature = "windef"))]
#[inline]
pub unsafe fn UiaProviderForNonClient(hwnd: super::HWND, idobject: i32, idchild: i32, ppprovider: *mut Option<super::IRawElementProviderSimple>) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaProviderForNonClient(hwnd : super::HWND, idobject : i32, idchild : i32, ppprovider : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { UiaProviderForNonClient(hwnd as _, idobject, idchild, ppprovider as _) }
}
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "shared", feature = "uiautomationcore"))]
#[inline]
pub unsafe fn UiaProviderFromIAccessible<P0>(paccessible: P0, idchild: i32, dwflags: super::DWORD, ppprovider: *mut Option<super::IRawElementProviderSimple>) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IAccessible>,
{
    windows_core::link!("uiautomationcore.dll" "system" fn UiaProviderFromIAccessible(paccessible : *mut core::ffi::c_void, idchild : i32, dwflags : super::DWORD, ppprovider : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { UiaProviderFromIAccessible(paccessible.param().abi(), idchild, dwflags, core::mem::transmute(ppprovider)) }
}
#[cfg(feature = "uiautomationcore")]
#[inline]
pub unsafe fn UiaRaiseActiveTextPositionChangedEvent<P0, P1>(provider: P0, textrange: P1) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IRawElementProviderSimple>,
    P1: windows_core::Param<super::ITextRangeProvider>,
{
    windows_core::link!("uiautomationcore.dll" "system" fn UiaRaiseActiveTextPositionChangedEvent(provider : *mut core::ffi::c_void, textrange : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { UiaRaiseActiveTextPositionChangedEvent(provider.param().abi(), textrange.param().abi()) }
}
#[cfg(feature = "uiautomationcore")]
#[inline]
pub unsafe fn UiaRaiseAsyncContentLoadedEvent<P0>(pprovider: P0, asynccontentloadedstate: AsyncContentLoadedState, percentcomplete: f64) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IRawElementProviderSimple>,
{
    windows_core::link!("uiautomationcore.dll" "system" fn UiaRaiseAsyncContentLoadedEvent(pprovider : *mut core::ffi::c_void, asynccontentloadedstate : AsyncContentLoadedState, percentcomplete : f64) -> windows_core::HRESULT);
    unsafe { UiaRaiseAsyncContentLoadedEvent(pprovider.param().abi(), asynccontentloadedstate, percentcomplete) }
}
#[cfg(feature = "uiautomationcore")]
#[inline]
pub unsafe fn UiaRaiseAutomationEvent<P0>(pprovider: P0, id: super::EVENTID) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IRawElementProviderSimple>,
{
    windows_core::link!("uiautomationcore.dll" "system" fn UiaRaiseAutomationEvent(pprovider : *mut core::ffi::c_void, id : super::EVENTID) -> windows_core::HRESULT);
    unsafe { UiaRaiseAutomationEvent(pprovider.param().abi(), id) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn UiaRaiseAutomationPropertyChangedEvent<P0>(pprovider: P0, id: super::PROPERTYID, oldvalue: &super::VARIANT, newvalue: &super::VARIANT) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IRawElementProviderSimple>,
{
    windows_core::link!("uiautomationcore.dll" "system" fn UiaRaiseAutomationPropertyChangedEvent(pprovider : *mut core::ffi::c_void, id : super::PROPERTYID, oldvalue : super::VARIANT, newvalue : super::VARIANT) -> windows_core::HRESULT);
    unsafe { UiaRaiseAutomationPropertyChangedEvent(pprovider.param().abi(), id, core::mem::transmute_copy(oldvalue), core::mem::transmute_copy(newvalue)) }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[inline]
pub unsafe fn UiaRaiseChangesEvent<P0>(pprovider: P0, eventidcount: i32, puiachanges: *mut super::UiaChangeInfo) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IRawElementProviderSimple>,
{
    windows_core::link!("uiautomationcore.dll" "system" fn UiaRaiseChangesEvent(pprovider : *mut core::ffi::c_void, eventidcount : i32, puiachanges : *mut super::UiaChangeInfo) -> windows_core::HRESULT);
    unsafe { UiaRaiseChangesEvent(pprovider.param().abi(), eventidcount, puiachanges as _) }
}
#[cfg(feature = "uiautomationcore")]
#[inline]
pub unsafe fn UiaRaiseNotificationEvent<P0>(provider: P0, notificationkind: super::NotificationKind, notificationprocessing: super::NotificationProcessing, displaystring: &windows_core::BSTR, activityid: &windows_core::BSTR) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IRawElementProviderSimple>,
{
    windows_core::link!("uiautomationcore.dll" "system" fn UiaRaiseNotificationEvent(provider : *mut core::ffi::c_void, notificationkind : super::NotificationKind, notificationprocessing : super::NotificationProcessing, displaystring : *mut core::ffi::c_void, activityid : *mut core::ffi::c_void) -> windows_core::HRESULT);
    unsafe { UiaRaiseNotificationEvent(provider.param().abi(), notificationkind, notificationprocessing, core::mem::transmute_copy(displaystring), core::mem::transmute_copy(activityid)) }
}
#[cfg(feature = "uiautomationcore")]
#[inline]
pub unsafe fn UiaRaiseStructureChangedEvent<P0>(pprovider: P0, structurechangetype: super::StructureChangeType, pruntimeid: *mut i32, cruntimeidlen: i32) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IRawElementProviderSimple>,
{
    windows_core::link!("uiautomationcore.dll" "system" fn UiaRaiseStructureChangedEvent(pprovider : *mut core::ffi::c_void, structurechangetype : super::StructureChangeType, pruntimeid : *mut i32, cruntimeidlen : i32) -> windows_core::HRESULT);
    unsafe { UiaRaiseStructureChangedEvent(pprovider.param().abi(), structurechangetype, pruntimeid as _, cruntimeidlen) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt"))]
#[inline]
pub unsafe fn UiaRaiseTextEditTextChangedEvent<P0>(pprovider: P0, texteditchangetype: super::TextEditChangeType, pchangeddata: *mut super::SAFEARRAY) -> windows_core::HRESULT
where
    P0: windows_core::Param<super::IRawElementProviderSimple>,
{
    windows_core::link!("uiautomationcore.dll" "system" fn UiaRaiseTextEditTextChangedEvent(pprovider : *mut core::ffi::c_void, texteditchangetype : super::TextEditChangeType, pchangeddata : *mut super::SAFEARRAY) -> windows_core::HRESULT);
    unsafe { UiaRaiseTextEditTextChangedEvent(pprovider.param().abi(), texteditchangetype, pchangeddata as _) }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt"))]
#[inline]
pub unsafe fn UiaRegisterProviderCallback(pcallback: UiaProviderCallback) {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaRegisterProviderCallback(pcallback : UiaProviderCallback));
    unsafe { UiaRegisterProviderCallback(pcallback) }
}
#[inline]
pub unsafe fn UiaRemoveEvent(hevent: HUIAEVENT) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaRemoveEvent(hevent : HUIAEVENT) -> windows_core::HRESULT);
    unsafe { UiaRemoveEvent(hevent as _) }
}
#[cfg(all(feature = "minwindef", feature = "uiautomationcore", feature = "windef"))]
#[inline]
pub unsafe fn UiaReturnRawElementProvider<P3>(hwnd: super::HWND, wparam: super::WPARAM, lparam: super::LPARAM, el: P3) -> super::LRESULT
where
    P3: windows_core::Param<super::IRawElementProviderSimple>,
{
    windows_core::link!("uiautomationcore.dll" "system" fn UiaReturnRawElementProvider(hwnd : super::HWND, wparam : super::WPARAM, lparam : super::LPARAM, el : *mut core::ffi::c_void) -> super::LRESULT);
    unsafe { UiaReturnRawElementProvider(hwnd as _, wparam, lparam, el.param().abi()) }
}
#[inline]
pub unsafe fn UiaSetFocus(hnode: HUIANODE) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaSetFocus(hnode : HUIANODE) -> windows_core::HRESULT);
    unsafe { UiaSetFocus(hnode as _) }
}
#[inline]
pub unsafe fn UiaTextRangeRelease(hobj: HUIATEXTRANGE) -> windows_core::BOOL {
    windows_core::link!("uiautomationcore.dll" "system" fn UiaTextRangeRelease(hobj : HUIATEXTRANGE) -> windows_core::BOOL);
    unsafe { UiaTextRangeRelease(hobj as _) }
}
#[inline]
pub unsafe fn ValuePattern_SetValue<P1>(hobj: HUIAPATTERNOBJECT, pval: P1) -> windows_core::HRESULT
where
    P1: windows_core::Param<windows_core::PCWSTR>,
{
    windows_core::link!("uiautomationcore.dll" "system" fn ValuePattern_SetValue(hobj : HUIAPATTERNOBJECT, pval : windows_core::PCWSTR) -> windows_core::HRESULT);
    unsafe { ValuePattern_SetValue(hobj as _, pval.param().abi()) }
}
#[inline]
pub unsafe fn VirtualizedItemPattern_Realize(hobj: HUIAPATTERNOBJECT) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn VirtualizedItemPattern_Realize(hobj : HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    unsafe { VirtualizedItemPattern_Realize(hobj as _) }
}
#[inline]
pub unsafe fn WindowPattern_Close(hobj: HUIAPATTERNOBJECT) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn WindowPattern_Close(hobj : HUIAPATTERNOBJECT) -> windows_core::HRESULT);
    unsafe { WindowPattern_Close(hobj as _) }
}
#[cfg(feature = "uiautomationcore")]
#[inline]
pub unsafe fn WindowPattern_SetWindowVisualState(hobj: HUIAPATTERNOBJECT, state: super::WindowVisualState) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn WindowPattern_SetWindowVisualState(hobj : HUIAPATTERNOBJECT, state : super::WindowVisualState) -> windows_core::HRESULT);
    unsafe { WindowPattern_SetWindowVisualState(hobj as _, state) }
}
#[inline]
pub unsafe fn WindowPattern_WaitForInputIdle(hobj: HUIAPATTERNOBJECT, milliseconds: i32, presult: *mut windows_core::BOOL) -> windows_core::HRESULT {
    windows_core::link!("uiautomationcore.dll" "system" fn WindowPattern_WaitForInputIdle(hobj : HUIAPATTERNOBJECT, milliseconds : i32, presult : *mut windows_core::BOOL) -> windows_core::HRESULT);
    unsafe { WindowPattern_WaitForInputIdle(hobj as _, milliseconds, presult as _) }
}
pub const AcceleratorKey_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x514865df_2557_4cb9_aeed_6ced084ce52c);
pub const AccessKey_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x06827b12_a7f9_4a15_917c_ffa5ad3eb0a7);
pub const ActiveTextPositionChanged_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa5c09e9c_c77d_4f25_b491_e5bb7017cbd4);
pub const AnnotationObjects_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x310910c8_7c6e_4f20_becd_4aaf6d191156);
pub const AnnotationTypes_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x64b71f76_53c4_4696_a219_20e940c9a176);
pub const Annotation_AdvancedProofingIssue_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xdac7b72c_c0f2_4b84_b90d_5fafc0f0ef1c);
pub const Annotation_AnnotationTypeId_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x20ae484f_69ef_4c48_8f5b_c4938b206ac7);
pub const Annotation_AnnotationTypeName_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x9b818892_5ac9_4af9_aa96_f58a77b058e3);
pub const Annotation_Author_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf161d3a7_f81b_4128_b17f_71f690914520);
pub const Annotation_Author_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7a528462_9c5c_4a03_a974_8b307a9937f2);
pub const Annotation_CircularReferenceError_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x25bd9cf4_1745_4659_ba67_727f0318c616);
pub const Annotation_Comment_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xfd2fda30_26b3_4c06_8bc7_98f1532e46fd);
pub const Annotation_ConflictingChange_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x98af8802_517c_459f_af13_016d3fab877e);
pub const Annotation_Custom_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x9ec82750_3931_4952_85bc_1dbff78a43e3);
pub const Annotation_DataValidationError_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc8649fa8_9775_437e_ad46_e709d93c2343);
pub const Annotation_DateTime_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x99b5ca5d_1acf_414b_a4d0_6b350b047578);
pub const Annotation_DeletionChange_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbe3d5b05_951d_42e7_901d_adc8c2cf34d0);
pub const Annotation_EditingLockedChange_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc31f3e1c_7423_4dac_8348_41f099ff6f64);
pub const Annotation_Endnote_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7565725c_2d99_4839_960d_33d3b866aba5);
pub const Annotation_ExternalChange_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x75a05b31_5f11_42fd_887d_dfa010db2392);
pub const Annotation_Footer_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xcceab046_1833_47aa_8080_701ed0b0c832);
pub const Annotation_Footnote_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x3de10e21_4125_42db_8620_be8083080624);
pub const Annotation_FormatChange_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xeb247345_d4f1_41ce_8e52_f79b69635e48);
pub const Annotation_FormulaError_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x95611982_0cab_46d5_a2f0_e30d1905f8bf);
pub const Annotation_GrammarError_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x757a048d_4518_41c6_854c_dc009b7cfb53);
pub const Annotation_Header_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x867b409b_b216_4472_a219_525e310681f8);
pub const Annotation_Highlighted_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x757c884e_8083_4081_8b9c_e87f5072f0e4);
pub const Annotation_InsertionChange_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x0dbeb3a6_df15_4164_a3c0_e21a8ce931c4);
pub const Annotation_Mathematics_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xeaab634b_26d0_40c1_8073_57ca1c633c9b);
pub const Annotation_MoveChange_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x9da587eb_23e5_4490_b385_1a22ddc8b187);
pub const Annotation_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf6c72ad7_356c_4850_9291_316f608a8c84);
pub const Annotation_Sensitive_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x37f4c04f_0f12_4464_929c_828fd15292e3);
pub const Annotation_SpellingError_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xae85567e_9ece_423f_81b7_96c43d53e50e);
pub const Annotation_Target_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb71b302d_2104_44ad_9c5c_092b4907d70f);
pub const Annotation_TrackChanges_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x21e6e888_dc14_4016_ac27_190553c8c470);
pub const Annotation_UnsyncedChange_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x1851116a_0e47_4b30_8cb5_d7dae4fbcd1b);
pub const AppBar_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x6114908d_cc02_4d37_875b_b530c7139554);
pub const AriaProperties_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x4213678c_e025_4922_beb5_e43ba08e6221);
pub const AriaRole_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xdd207b95_be4a_4e0d_b727_63ace94b6916);
pub type AsyncContentLoadedState = i32;
pub const AsyncContentLoadedState_Beginning: AsyncContentLoadedState = 0;
pub const AsyncContentLoadedState_Completed: AsyncContentLoadedState = 2;
pub const AsyncContentLoadedState_Progress: AsyncContentLoadedState = 1;
pub const AsyncContentLoaded_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5fdee11c_d2fa_4fb9_904e_5cbee894d5ef);
pub const AutomationFocusChanged_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb68a1f17_f60d_41a7_a3cc_b05292155fe0);
pub const AutomationId_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc82c0500_b60e_4310_a267_303c531f8ee5);
pub type AutomationIdentifierType = i32;
pub const AutomationIdentifierType_Annotation: AutomationIdentifierType = 6;
pub const AutomationIdentifierType_Changes: AutomationIdentifierType = 7;
pub const AutomationIdentifierType_ControlType: AutomationIdentifierType = 3;
pub const AutomationIdentifierType_Event: AutomationIdentifierType = 2;
pub const AutomationIdentifierType_LandmarkType: AutomationIdentifierType = 5;
pub const AutomationIdentifierType_Pattern: AutomationIdentifierType = 1;
pub const AutomationIdentifierType_Property: AutomationIdentifierType = 0;
pub const AutomationIdentifierType_Style: AutomationIdentifierType = 8;
pub const AutomationIdentifierType_TextAttribute: AutomationIdentifierType = 4;
pub const AutomationPropertyChanged_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x2527fba1_8d7a_4630_a4cc_e66315942f52);
pub const BoundingRectangle_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7bbfe8b2_3bfc_48dd_b729_c794b846e9a1);
pub const Button_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5a78e369_c6a1_4f33_a9d7_79f20d0c788e);
pub const Calendar_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8913eb88_00e5_46bc_8e4e_14a786e165a1);
pub const CenterPoint_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x0cb00c08_540c_4edb_9445_26359ea69785);
pub const Changes_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7df26714_614f_4e05_9488_716c5ba19436);
pub const Changes_Summary_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x313d65a6_e60f_4d62_9861_55afd728d207);
pub const CheckBox_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xfb50f922_a3db_49c0_8bc3_06dad55778e2);
pub const ClassName_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x157b7215_894f_4b65_84e2_aac0da08b16b);
pub const ClickablePoint_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x0196903b_b203_4818_a9f3_f08e675f2341);
pub const ComboBox_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x54cb426c_2f33_4fff_aaa1_aef60dac5deb);
pub type ConditionType = i32;
pub const ConditionType_And: ConditionType = 3;
pub const ConditionType_False: ConditionType = 1;
pub const ConditionType_Not: ConditionType = 5;
pub const ConditionType_Or: ConditionType = 4;
pub const ConditionType_Property: ConditionType = 2;
pub const ConditionType_True: ConditionType = 0;
pub const ControlType_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xca774fea_28ac_4bc2_94ca_acec6d6c10a3);
pub const ControllerFor_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x51124c8a_a5d2_4f13_9be6_7fa8ba9d3a90);
pub const Culture_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe2d74f27_3d79_4dc2_b88b_3044963a8afb);
pub const CustomNavigation_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xafea938a_621e_4054_bb2c_2f46114dac3f);
pub const Custom_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf29ea0c3_adb7_430a_ba90_e52c7313e6ed);
pub const DataGrid_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x84b783af_d103_4b0a_8415_e73942410f4b);
pub const DataItem_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa0177842_d94f_42a5_814b_6068addc8da5);
pub const DescribedBy_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7c5865b8_9992_40fd_8db0_6bf1d317f998);
pub const Dock_DockPosition_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x6d67f02e_c0b0_4b10_b5b9_18d6ecf98760);
pub const Dock_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x9cbaa846_83c8_428d_827f_7e6063fe0620);
pub const Document_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x3cd6bb6f_6f08_4562_b229_e4e2fc7a9eb4);
pub const Drag_DragCancel_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc3ede6fa_3451_4e0f_9e71_df9c280a4657);
pub const Drag_DragComplete_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x38e96188_ef1f_463e_91ca_3a7792c29caf);
pub const Drag_DragStart_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x883a480b_3aa9_429d_95e4_d9c8d011f0dd);
pub const Drag_DropEffect_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x646f2779_48d3_4b23_8902_4bf100005df3);
pub const Drag_DropEffects_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf5d61156_7ce6_49be_a836_9269dcec920f);
pub const Drag_GrabbedItems_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x77c1562c_7b86_4b21_9ed7_3cefda6f4c43);
pub const Drag_IsGrabbed_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x45f206f3_75cc_4cca_a9b9_fcdfb982d8a2);
pub const Drag_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc0bee21f_ccb3_4fed_995b_114f6e3d2728);
pub const DropTarget_DragEnter_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xaad9319b_032c_4a88_961d_1cf579581e34);
pub const DropTarget_DragLeave_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x0f82eb15_24a2_4988_9217_de162aee272b);
pub const DropTarget_DropTargetEffect_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8bb75975_a0ca_4981_b818_87fc66e9509d);
pub const DropTarget_DropTargetEffects_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbc1dd4ed_cb89_45f1_a592_e03b08ae790f);
pub const DropTarget_Dropped_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x622cead8_1edb_4a3d_abbc_be2211ff68b5);
pub const DropTarget_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x0bcbec56_bd34_4b7b_9fd5_2659905ea3dc);
pub const Edit_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x6504a5c8_2c86_4f87_ae7b_1abddc810cf9);
pub type EventArgsType = i32;
pub const EventArgsType_ActiveTextPositionChanged: EventArgsType = 8;
pub const EventArgsType_AsyncContentLoaded: EventArgsType = 3;
pub const EventArgsType_Changes: EventArgsType = 6;
pub const EventArgsType_Notification: EventArgsType = 7;
pub const EventArgsType_PropertyChanged: EventArgsType = 1;
pub const EventArgsType_Simple: EventArgsType = 0;
pub const EventArgsType_StructureChanged: EventArgsType = 2;
pub const EventArgsType_StructuredMarkup: EventArgsType = 9;
pub const EventArgsType_TextEditTextChanged: EventArgsType = 5;
pub const EventArgsType_WindowClosed: EventArgsType = 4;
pub const ExpandCollapse_ExpandCollapseState_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x275a4c48_85a7_4f69_aba0_af157610002b);
pub const ExpandCollapse_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xae05efa2_f9d1_428a_834c_53a5c52f9b8b);
pub const FillColor_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x6e0ec4d0_e2a8_4a56_9de7_953389933b39);
pub const FillType_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc6fc74e4_8cb9_429c_a9e1_9bc4ac372b62);
pub const FlowsFrom_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x05c6844f_19de_48f8_95fa_880d5b0fd615);
pub const FlowsTo_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe4f33d20_559a_47fb_a830_f9cb4ff1a70a);
pub const FrameworkId_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xdbfd9900_7e1a_4f58_b61b_7063120f773b);
pub const FullDescription_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x0d4450ff_6aef_4f33_95dd_7befa72a4391);
pub const GridItem_ColumnSpan_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x583ea3f5_86d0_4b08_a6ec_2c5463ffc109);
pub const GridItem_Column_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc774c15c_62c0_4519_8bdc_47be573c8ad5);
pub const GridItem_Parent_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x9d912252_b97f_4ecc_8510_ea0e33427c72);
pub const GridItem_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf2d5c877_a462_4957_a2a5_2c96b303bc63);
pub const GridItem_RowSpan_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x4582291c_466b_4e93_8e83_3d1715ec0c5e);
pub const GridItem_Row_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x6223972a_c945_4563_9329_fdc974af2553);
pub const Grid_ColumnCount_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xfe96f375_44aa_4536_ac7a_2a75d71a3efc);
pub const Grid_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x260a2ccb_93a8_4e44_a4c1_3df397f2b02b);
pub const Grid_RowCount_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x2a9505bf_c2eb_4fb6_b356_8245ae53703e);
pub const Group_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xad50aa1c_e8c8_4774_ae1b_dd86df0b3bdc);
pub type HUIAEVENT = *mut HUIAEVENT__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HUIAEVENT__ {
    pub unused: i32,
}
pub type HUIANODE = *mut HUIANODE__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HUIANODE__ {
    pub unused: i32,
}
pub type HUIAPATTERNOBJECT = *mut HUIAPATTERNOBJECT__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HUIAPATTERNOBJECT__ {
    pub unused: i32,
}
pub type HUIATEXTRANGE = *mut HUIATEXTRANGE__;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HUIATEXTRANGE__ {
    pub unused: i32,
}
pub const HasKeyboardFocus_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xcf8afd39_3f46_4800_9656_b2bf12529905);
pub const HeaderItem_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe6bc12cb_7c8e_49cf_b168_4a93a32bebb0);
pub const Header_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5b90cbce_78fb_4614_82b6_554d74718e67);
pub const HeadingLevel_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x29084272_aaaf_4a30_8796_3c12f62b6bbb);
pub const HelpText_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x08555685_0977_45c7_a7a6_abaf5684121a);
pub const HostedFragmentRootsInvalidated_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe6bdb03e_0921_4ec5_8dcf_eae877b0426b);
pub const Hyperlink_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8a56022c_b00d_4d15_8ff0_5b6b266e5e02);
pub const Image_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x2d3736e4_6b16_4c57_a962_f93260a75243);
pub const InputDiscarded_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7f36c367_7b18_417c_97e3_9d58ddc944ab);
pub const InputReachedOtherElement_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xed201d8a_4e6c_415e_a874_2460c9b66ba8);
pub const InputReachedTarget_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x93ed549a_0549_40f0_bedb_28e44f7de2a3);
pub const Invoke_Invoked_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xdfd699f0_c915_49dd_b422_dde785c3d24b);
pub const Invoke_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xd976c2fc_66ea_4a6e_b28f_c24c7546ad37);
pub const IsAnnotationPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x0b5b3238_6d5c_41b6_bcc4_5e807f6551c4);
pub const IsContentElement_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x4bda64a8_f5d8_480b_8155_ef2e89adb672);
pub const IsControlElement_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x95f35085_abcc_4afd_a5f4_dbb46c230fdb);
pub const IsCustomNavigationPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8f8e80d4_2351_48e0_874a_54aa7313889a);
pub const IsDataValidForForm_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x445ac684_c3fc_4dd9_acf8_845a579296ba);
pub const IsDialog_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x9d0dfb9b_8436_4501_bbbb_e534a4fb3b3f);
pub const IsDockPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x2600a4c4_2ff8_4c96_ae31_8fe619a13c6c);
pub const IsDragPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe997a7b7_1d39_4ca7_be0f_277fcf5605cc);
pub const IsDropTargetPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x0686b62e_8e19_4aaf_873d_384f6d3b92be);
pub const IsEnabled_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x2109427f_da60_4fed_bf1b_264bdce6eb3a);
pub const IsExpandCollapsePatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x929d3806_5287_4725_aa16_222afc63d595);
pub const IsGridItemPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5a43e524_f9a2_4b12_84c8_b48a3efedd34);
pub const IsGridPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5622c26c_f0ef_4f3b_97cb_714c0868588b);
pub const IsInvokePatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x4e725738_8364_4679_aa6c_f3f41931f750);
pub const IsItemContainerPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x624b5ca7_fe40_4957_a019_20c4cf11920f);
pub const IsKeyboardFocusable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf7b8552a_0859_4b37_b9cb_51e72092f29f);
pub const IsLegacyIAccessiblePatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xd8ebd0c7_929a_4ee7_8d3a_d3d94413027b);
pub const IsMultipleViewPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xff0a31eb_8e25_469d_8d6e_e771a27c1b90);
pub const IsObjectModelPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x6b21d89b_2841_412f_8ef2_15ca952318ba);
pub const IsOffscreen_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x03c3d160_db79_42db_a2ef_1c231eede507);
pub const IsPassword_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe8482eb1_687c_497b_bebc_03be53ec1454);
pub const IsPeripheral_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xda758276_7ed5_49d4_8e68_ecc9a2d300dd);
pub const IsRangeValuePatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xfda4244a_eb4d_43ff_b5ad_ed36d373ec4c);
pub const IsRequiredForForm_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x4f5f43cf_59fb_4bde_a270_602e5e1141e9);
pub const IsScrollItemPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x1cad1a05_0927_4b76_97e1_0fcdb209b98a);
pub const IsScrollPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x3ebb7b4a_828a_4b57_9d22_2fea1632ed0d);
pub const IsSelectionItemPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8becd62d_0bc3_4109_bee2_8e6715290e68);
pub const IsSelectionPattern2Available_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x490806fb_6e89_4a47_8319_d266e511f021);
pub const IsSelectionPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf588acbe_c769_4838_9a60_2686dc1188c4);
pub const IsSpreadsheetItemPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x9fe79b2a_2f94_43fd_996b_549e316f4acd);
pub const IsSpreadsheetPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x6ff43732_e4b4_4555_97bc_ecdbbc4d1888);
pub const IsStructuredMarkupPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb0d4c196_2c0b_489c_b165_a405928c6f3d);
pub const IsStylesPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x27f353d3_459c_4b59_a490_50611dacafb5);
pub const IsSynchronizedInputPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x75d69cc5_d2bf_4943_876e_b45b62a6cc66);
pub const IsTableItemPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xeb36b40d_8ea4_489b_a013_e60d5951fe34);
pub const IsTablePatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xcb83575f_45c2_4048_9c76_159715a139df);
pub const IsTextChildPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x559e65df_30ff_43b5_b5ed_5b283b80c7e9);
pub const IsTextEditPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7843425c_8b32_484c_9ab5_e3200571ffda);
pub const IsTextPattern2Available_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x41cf921d_e3f1_4b22_9c81_e1c3ed331c22);
pub const IsTextPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xfbe2d69d_aff6_4a45_82e2_fc92a82f5917);
pub const IsTogglePatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x78686d53_fcd0_4b83_9b78_5832ce63bb5b);
pub const IsTransformPattern2Available_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x25980b4b_be04_4710_ab4a_fda31dbd2895);
pub const IsTransformPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa7f78804_d68b_4077_a5c6_7a5ea1ac31c5);
pub const IsValuePatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x0b5020a7_2119_473b_be37_5ceb98bbfb22);
pub const IsVirtualizedItemPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x302cb151_2ac8_45d6_977b_d2b3a5a53f20);
pub const IsWindowPatternAvailable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe7a57bb1_5888_4155_98dc_b422fd57f2bc);
pub const ItemContainer_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x3d13da0f_8b9a_4a99_85fa_c5c9a69f1ed4);
pub const ItemStatus_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x51de0321_3973_43e7_8913_0b08e813c37f);
pub const ItemType_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xcdda434d_6222_413b_a68a_325dd1d40f39);
pub const LabeledBy_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe5b8924b_fc8a_4a35_8031_cf78ac43e55e);
pub const LandmarkType_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x454045f2_6f61_49f7_a4f8_b5f0cf82da1e);
pub const LayoutInvalidated_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xed7d6544_a6bd_4595_9bae_3d28946cc715);
pub const LegacyIAccessible_ChildId_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x9a191b5d_9ef2_4787_a459_dcde885dd4e8);
pub const LegacyIAccessible_DefaultAction_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x3b331729_eaad_4502_b85f_92615622913c);
pub const LegacyIAccessible_Description_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x46448418_7d70_4ea9_9d27_b7e775cf2ad7);
pub const LegacyIAccessible_Help_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x94402352_161c_4b77_a98d_a872cc33947a);
pub const LegacyIAccessible_KeyboardShortcut_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8f6909ac_00b8_4259_a41c_966266d43a8a);
pub const LegacyIAccessible_Name_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xcaeb063d_40ae_4869_aa5a_1b8e5d666739);
pub const LegacyIAccessible_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x54cc0a9f_3395_48af_ba8d_73f85690f3e0);
pub const LegacyIAccessible_Role_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x6856e59f_cbaf_4e31_93e8_bcbf6f7e491c);
pub const LegacyIAccessible_Selection_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8aa8b1e0_0891_40cc_8b06_90d7d4166219);
pub const LegacyIAccessible_State_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xdf985854_2281_4340_ab9c_c60e2c5803f6);
pub const LegacyIAccessible_Value_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb5c5b0b6_8217_4a77_97a5_190a85ed0156);
pub const Level_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x242ac529_cd36_400f_aad9_7876ef3af627);
pub const ListItem_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7b3717f2_44d1_4a58_98a8_f12a9b8f78e2);
pub const List_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x9b149ee1_7cca_4cfc_9af1_cac7bddd3031);
pub const LiveRegionChanged_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x102d5e90_e6a9_41b6_b1c5_a9b1929d9510);
pub const LiveSetting_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc12bcd8e_2a8e_4950_8ae7_3625111d58eb);
pub const LocalizedControlType_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8763404f_a1bd_452a_89c4_3f01d3833806);
pub const LocalizedLandmarkType_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7ac81980_eafb_4fb2_bf91_f485bef5e8e1);
pub const MenuBar_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xcc384250_0e7b_4ae8_95ae_a08f261b52ee);
pub const MenuClosed_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x3cf1266e_1582_4041_acd7_88a35a965297);
pub const MenuItem_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf45225d3_d0a0_49d8_9834_9a000d2aeddc);
pub const MenuModeEnd_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x9ecd4c9f_80dd_47b8_8267_5aec06bb2cff);
pub const MenuModeStart_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x18d7c631_166a_4ac9_ae3b_ef4b5420e681);
pub const MenuOpened_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xebe2e945_66ca_4ed1_9ff8_2ad7df0a1b08);
pub const Menu_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x2e9b1440_0ea8_41fd_b374_c1ea6f503cd1);
pub const MultipleView_CurrentView_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7a81a67a_b94f_4875_918b_65c8d2f998e5);
pub const MultipleView_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x547a6ae4_113f_47c4_850f_db4dfa466b1d);
pub const MultipleView_SupportedViews_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8d5db9fd_ce3c_4ae7_b788_400a3c645547);
pub const Name_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc3a6921b_4a99_44f1_bca6_61187052c431);
pub const NewNativeWindowHandle_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5196b33b_380a_4982_95e1_91f3ef60e024);
pub type NormalizeState = i32;
pub const NormalizeState_Custom: NormalizeState = 2;
pub const NormalizeState_None: NormalizeState = 0;
pub const NormalizeState_View: NormalizeState = 1;
pub const Notification_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x72c5a2f7_9788_480f_b8eb_4dee00f6186f);
pub const ObjectModel_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x3e04acfe_08fc_47ec_96bc_353fa3b34aa7);
pub const OptimizeForVisualContent_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x6a852250_c75a_4e5d_b858_e381b0f78861);
pub const Orientation_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa01eee62_3884_4415_887e_678ec21e39ba);
pub const OutlineColor_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc395d6c0_4b55_4762_a073_fd303a634f52);
pub const OutlineThickness_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x13e67cc7_dac2_4888_bdd3_375c62fa9618);
pub const Pane_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5c2b3f5b_9182_42a3_8dec_8c04c1ee634d);
pub const PositionInSet_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x33d1dc54_641e_4d76_a6b1_13f341c1f896);
pub const ProcessId_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x40499998_9c31_4245_a403_87320e59eaf6);
pub const ProgressBar_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x228c9f86_c36c_47bb_9fb6_a5834bfc53a4);
pub const ProviderDescription_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xdca5708a_c16b_4cd9_b889_beb16a804904);
pub type ProviderType = i32;
pub const ProviderType_BaseHwnd: ProviderType = 0;
pub const ProviderType_NonClientArea: ProviderType = 2;
pub const ProviderType_Proxy: ProviderType = 1;
pub const RadioButton_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x3bdb49db_fe2c_4483_b3e1_e57f219440c6);
pub const RangeValue_IsReadOnly_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x25fa1055_debf_4373_a79e_1f1a1908d3c4);
pub const RangeValue_LargeChange_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa1f96325_3a3d_4b44_8e1f_4a46d9844019);
pub const RangeValue_Maximum_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x19319914_f979_4b35_a1a6_d37e05433473);
pub const RangeValue_Minimum_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x78cbd3b2_684d_4860_af93_d1f95cb022fd);
pub const RangeValue_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x18b00d87_b1c9_476a_bfbd_5f0bdb926f63);
pub const RangeValue_SmallChange_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x81c2c457_3941_4107_9975_139760f7c072);
pub const RangeValue_Value_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x131f5d98_c50c_489d_abe5_ae220898c5f7);
pub const Rotation_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x767cdc7d_aec0_4110_ad32_30edd403492e);
pub const RuntimeId_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa39eebfa_7fba_4c89_b4d4_b99e2de7d160);
pub const SID_ControlElementProvider: windows_core::GUID = windows_core::GUID::from_u128(0xf4791d68_e254_4ba3_9a53_26a5c5497946);
pub const SID_IsUIAutomationObject: windows_core::GUID = windows_core::GUID::from_u128(0xb96fdb85_7204_4724_842b_c7059dedb9d0);
pub const ScrollBar_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xdaf34b36_5065_4946_b22f_92595fc0751a);
pub const ScrollItem_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x4591d005_a803_4d5c_b4d5_8d2800f906a7);
pub const Scroll_HorizontalScrollPercent_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc7c13c0e_eb21_47ff_acc4_b5a3350f5191);
pub const Scroll_HorizontalViewSize_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x70c2e5d4_fcb0_4713_a9aa_af92ff79e4cd);
pub const Scroll_HorizontallyScrollable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8b925147_28cd_49ae_bd63_f44118d2e719);
pub const Scroll_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x895fa4b4_759d_4c50_8e15_03460672003c);
pub const Scroll_VerticalScrollPercent_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x6c8d7099_b2a8_4948_bff7_3cf9058bfefb);
pub const Scroll_VerticalViewSize_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xde6a2e22_d8c7_40c5_83ba_e5f681d53108);
pub const Scroll_VerticallyScrollable_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x89164798_0068_4315_b89a_1e7cfbbc3dfc);
pub const Selection2_CurrentSelectedItem_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x34257c26_83b5_41a6_939c_ae841c136236);
pub const Selection2_FirstSelectedItem_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xcc24ea67_369c_4e55_9ff7_38da69540c29);
pub const Selection2_ItemCount_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbb49eb9f_456d_4048_b591_9c2026b84636);
pub const Selection2_LastSelectedItem_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xcf7bda90_2d83_49f8_860c_9ce394cf89b4);
pub const SelectionItem_ElementAddedToSelectionEvent_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x3c822dd1_c407_4dba_91dd_79d4aed0aec6);
pub const SelectionItem_ElementRemovedFromSelectionEvent_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x097fa8a9_7079_41af_8b9c_0934d8305e5c);
pub const SelectionItem_ElementSelectedEvent_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb9c7dbfb_4ebe_4532_aaf4_008cf647233c);
pub const SelectionItem_IsSelected_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf122835f_cd5f_43df_b79d_4b849e9e6020);
pub const SelectionItem_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x9bc64eeb_87c7_4b28_94bb_4d9fa437b6ef);
pub const SelectionItem_SelectionContainer_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa4365b6e_9c1e_4b63_8b53_c2421dd1e8fb);
pub const Selection_CanSelectMultiple_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x49d73da5_c883_4500_883d_8fcf8daf6cbe);
pub const Selection_InvalidatedEvent_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xcac14904_16b4_4b53_8e47_4cb1df267bb7);
pub const Selection_IsSelectionRequired_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb1ae4422_63fe_44e7_a5a5_a738c829b19a);
pub const Selection_Pattern2_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xfba25cab_ab98_49f7_a7dc_fe539dc15be7);
pub const Selection_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x66e3b7e8_d821_4d25_8761_435d2c8b253f);
pub const Selection_Selection_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xaa6dc2a2_0e2b_4d38_96d5_34e470b81853);
pub const SemanticZoom_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5fd34a43_061e_42c8_b589_9dccf74bc43a);
pub const Separator_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8767eba3_2a63_4ab0_ac8d_aa50e23de978);
pub const SizeOfSet_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x1600d33c_3b9f_4369_9431_aa293f344cf1);
pub const Size_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x2b5f761d_f885_4404_973f_9b1d98e36d8f);
pub const Slider_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb033c24b_3b35_4cea_b609_763682fa660b);
pub const Spinner_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x60cc4b38_3cb1_4161_b442_c6b726c17825);
pub const SplitButton_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7011f01f_4ace_4901_b461_920a6f1ca650);
pub const SpreadsheetItem_AnnotationObjects_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa3194c38_c9bc_4604_9396_ae3f9f457f7b);
pub const SpreadsheetItem_AnnotationTypes_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc70c51d0_d602_4b45_afbc_b4712b96d72b);
pub const SpreadsheetItem_Formula_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe602e47d_1b47_4bea_87cf_3b0b0b5c15b6);
pub const SpreadsheetItem_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x32cf83ff_f1a8_4a8c_8658_d47ba74e20ba);
pub const Spreadsheet_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x6a5b24c9_9d1e_4b85_9e44_c02e3169b10b);
pub const StatusBar_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xd45e7d1b_5873_475f_95a4_0433e1f1b00a);
pub const StructureChanged_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x59977961_3edd_4b11_b13b_676b2a2a6ca9);
pub const StructuredMarkup_CompositionComplete_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc48a3c17_677a_4047_a68d_fc1257528aef);
pub const StructuredMarkup_Deleted_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf9d0a020_e1c1_4ecf_b9aa_52efde7e41e1);
pub const StructuredMarkup_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xabbd0878_8665_4f5c_94fc_36e7d8bb706b);
pub const StructuredMarkup_SelectionChanged_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa7c815f7_ff9f_41c7_a3a7_ab6cbfdb4903);
pub const StyleId_BulletedList_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5963ed64_6426_4632_8caf_a32ad402d91a);
pub const StyleId_Custom_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xef2edd3e_a999_4b7c_a378_09bbd52a3516);
pub const StyleId_Emphasis_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xca6e7dbe_355e_4820_95a0_925f041d3470);
pub const StyleId_Heading1_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7f7e8f69_6866_4621_930c_9a5d0ca5961c);
pub const StyleId_Heading2_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbaa9b241_5c69_469d_85ad_474737b52b14);
pub const StyleId_Heading3_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbf8be9d2_d8b8_4ec5_8c52_9cfb0d035970);
pub const StyleId_Heading4_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8436ffc0_9578_45fc_83a4_ff40053315dd);
pub const StyleId_Heading5_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x909f424d_0dbf_406e_97bb_4e773d9798f7);
pub const StyleId_Heading6_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x89d23459_5d5b_4824_a420_11d3ed82e40f);
pub const StyleId_Heading7_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa3790473_e9ae_422d_b8e3_3b675c6181a4);
pub const StyleId_Heading8_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x2bc14145_a40c_4881_84ae_f2235685380c);
pub const StyleId_Heading9_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc70d9133_bb2a_43d3_8ac6_33657884b0f0);
pub const StyleId_Normal_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xcd14d429_e45e_4475_a1c5_7f9e6be96eba);
pub const StyleId_NumberedList_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x1e96dbd5_64c3_43d0_b1ee_b53b06e3eddf);
pub const StyleId_Quote_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5d1c21ea_8195_4f6c_87ea_5dabece64c1d);
pub const StyleId_Subtitle_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb5d9fc17_5d6f_4420_b439_7cb19ad434e2);
pub const StyleId_Title_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x15d8201a_ffcf_481f_b0a1_30b63be98f07);
pub const Styles_ExtendedProperties_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf451cda0_ba0a_4681_b0b0_0dbdb53e58f3);
pub const Styles_FillColor_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x63eff97a_a1c5_4b1d_84eb_b765f2edd632);
pub const Styles_FillPatternColor_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x939a59fe_8fbd_4e75_a271_ac4595195163);
pub const Styles_FillPatternStyle_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x81cf651f_482b_4451_a30a_e1545e554fb8);
pub const Styles_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x1ae62655_da72_4d60_a153_e5aa6988e3bf);
pub const Styles_Shape_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc71a23f8_778c_400d_8458_3b543e526984);
pub const Styles_StyleId_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xda82852f_3817_4233_82af_02279e72cc77);
pub const Styles_StyleName_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x1c12b035_05d1_4f55_9e8e_1489f3ff550d);
pub const SynchronizedInput_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x05c288a6_c47b_488b_b653_33977a551b8b);
pub const SystemAlert_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xd271545d_7a3a_47a7_8474_81d29a2451c9);
pub const TabItem_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x2c6a634f_921b_4e6e_b26e_08fcb0798f4c);
pub const Tab_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x38cd1f2d_337a_4bd2_a5e3_adb469e30bd3);
pub const TableItem_ColumnHeaderItems_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x967a56a3_74b6_431e_8de6_99c411031c58);
pub const TableItem_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xdf1343bd_1888_4a29_a50c_b92e6de37f6f);
pub const TableItem_RowHeaderItems_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb3f853a0_0574_4cd8_bcd7_ed5923572d97);
pub const Table_ColumnHeaders_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xaff1d72b_968d_42b1_b459_150b299da664);
pub const Table_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x773bfa0e_5bc4_4deb_921b_de7b3206229e);
pub const Table_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc415218e_a028_461e_aa92_8f925cf79351);
pub const Table_RowHeaders_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xd9e35b87_6eb8_4562_aac6_a8a9075236a8);
pub const Table_RowOrColumnMajor_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x83be75c3_29fe_4a30_85e1_2a6277fd106e);
pub const TextChild_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7533cab7_3bfe_41ef_9e85_e2638cbe169e);
pub const TextEdit_ConversionTargetChanged_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x3388c183_ed4f_4c8b_9baa_364d51d8847f);
pub const TextEdit_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x69f3ff89_5af9_4c75_9340_f2de292e4591);
pub const TextEdit_TextChanged_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x120b0308_ec22_4eb8_9c98_9867cda1b165);
pub const Text_AfterParagraphSpacing_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x588cbb38_e62f_497c_b5d1_ccdf0ee823d8);
pub const Text_AfterSpacing_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x588cbb38_e62f_497c_b5d1_ccdf0ee823d8);
pub const Text_AnimationStyle_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x628209f0_7c9a_4d57_be64_1f1836571ff5);
pub const Text_AnnotationObjects_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xff41cf68_e7ab_40b9_8c72_72a8ed94017d);
pub const Text_AnnotationTypes_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xad2eb431_ee4e_4be1_a7ba_5559155a73ef);
pub const Text_BackgroundColor_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xfdc49a07_583d_4f17_ad27_77fc832a3c0b);
pub const Text_BeforeParagraphSpacing_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbe7b0ab1_c822_4a24_85e9_c8f2650fc79c);
pub const Text_BeforeSpacing_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbe7b0ab1_c822_4a24_85e9_c8f2650fc79c);
pub const Text_BulletStyle_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc1097c90_d5c4_4237_9781_3bec8ba54e48);
pub const Text_CapStyle_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xfb059c50_92cc_49a5_ba8f_0aa872bba2f3);
pub const Text_CaretBidiMode_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x929ee7a6_51d3_4715_96dc_b694fa24a168);
pub const Text_CaretPosition_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb227b131_9889_4752_a91b_733efdc5c5a0);
pub const Text_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xae9772dc_d331_4f09_be20_7e6dfaf07b0a);
pub const Text_Culture_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xc2025af9_a42d_4ced_a1fb_c6746315222e);
pub const Text_FontName_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x64e63ba8_f2e5_476e_a477_1734feaaf726);
pub const Text_FontSize_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xdc5eeeff_0506_4673_93f2_377e4a8e01f1);
pub const Text_FontWeight_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x6fc02359_b316_4f5f_b401_f1ce55741853);
pub const Text_ForegroundColor_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x72d1c95d_5e60_471a_96b1_6c1b3b77a436);
pub const Text_HorizontalTextAlignment_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x04ea6161_fba3_477a_952a_bb326d026a5b);
pub const Text_IndentationFirstLine_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x206f9ad5_c1d3_424a_8182_6da9a7f3d632);
pub const Text_IndentationLeading_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5cf66bac_2d45_4a4b_b6c9_f7221d2815b0);
pub const Text_IndentationTrailing_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x97ff6c0f_1ce4_408a_b67b_94d83eb69bf2);
pub const Text_IsActive_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf5a4e533_e1b8_436b_935d_b57aa3f558c4);
pub const Text_IsHidden_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x360182fb_bdd7_47f6_ab69_19e33f8a3344);
pub const Text_IsItalic_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xfce12a56_1336_4a34_9663_1bab47239320);
pub const Text_IsReadOnly_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xa738156b_ca3e_495e_9514_833c440feb11);
pub const Text_IsSubscript_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf0ead858_8f53_413c_873f_1a7d7f5e0de4);
pub const Text_IsSuperscript_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xda706ee4_b3aa_4645_a41f_cd25157dea76);
pub const Text_LineSpacing_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x63ff70ae_d943_4b47_8ab7_a7a033d3214b);
pub const Text_Link_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb38ef51d_9e8d_4e46_9144_56ebe177329b);
pub const Text_MarginBottom_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7ee593c4_72b4_4cac_9271_3ed24b0e4d42);
pub const Text_MarginLeading_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x9e9242d0_5ed0_4900_8e8a_eecc03835afc);
pub const Text_MarginTop_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x683d936f_c9b9_4a9a_b3d9_d20d33311e2a);
pub const Text_MarginTrailing_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xaf522f98_999d_40af_a5b2_0169d0342002);
pub const Text_OutlineStyles_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5b675b27_db89_46fe_970c_614d523bb97d);
pub const Text_OverlineColor_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x83ab383a_fd43_40da_ab3e_ecf8165cbb6d);
pub const Text_OverlineStyle_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x0a234d66_617e_427f_871d_e1ff1e0c213f);
pub const Text_Pattern2_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x498479a2_5b22_448d_b6e4_647490860698);
pub const Text_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8615f05d_7de5_44fd_a679_2ca4b46033a8);
pub const Text_SayAsInterpretAs_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb38ad6ac_eee1_4b6e_88cc_014cefa93fcb);
pub const Text_SelectionActiveEnd_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x1f668cc3_9bbf_416b_b0a2_f89f86f6612c);
pub const Text_StrikethroughColor_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbfe15a18_8c41_4c5a_9a0b_04af0e07f487);
pub const Text_StrikethroughStyle_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x72913ef1_da00_4f01_899c_ac5a8577a307);
pub const Text_StyleId_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x14c300de_c32b_449b_ab7c_b0e0789aea5d);
pub const Text_StyleName_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x22c9e091_4d66_45d8_a828_737bab4c98a7);
pub const Text_Tabs_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x2e68d00b_92fe_42d8_899a_a784aa4454a1);
pub const Text_TextChangedEvent_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x4a342082_f483_48c4_ac11_a84b435e2a84);
pub const Text_TextFlowDirections_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8bdf8739_f420_423e_af77_20a5d973a907);
pub const Text_TextSelectionChangedEvent_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x918edaa1_71b3_49ae_9741_79beb8d358f3);
pub const Text_UnderlineColor_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbfa12c73_fde2_4473_bf64_1036d6aa0f45);
pub const Text_UnderlineStyle_Attribute_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x5f3b21c0_ede4_44bd_9c36_3853038cbfeb);
pub const Thumb_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x701ca877_e310_4dd6_b644_797e4faea213);
pub const TitleBar_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x98aa55bf_3bb0_4b65_836e_2ea30dbc171f);
pub const Toggle_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x0b419760_e2f4_43ff_8c5f_9457c82b56e9);
pub const Toggle_ToggleState_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb23cdc52_22c2_4c6c_9ded_f5c422479ede);
pub const ToolBar_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8f06b751_e182_4e98_8893_2284543a7dce);
pub const ToolTipClosed_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x276d71ef_24a9_49b6_8e97_da98b401bbcd);
pub const ToolTipOpened_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x3f4b97ff_2edc_451d_bca4_95a3188d5b03);
pub const ToolTip_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x05ddc6d1_2137_4768_98ea_73f52f7134f3);
pub const Tranform_Pattern2_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x8afcfd07_a369_44de_988b_2f7ff49fb8a8);
pub const Transform2_CanZoom_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf357e890_a756_4359_9ca6_86702bf8f381);
pub const Transform2_ZoomLevel_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xeee29f1a_f4a2_4b5b_ac65_95cf93283387);
pub const Transform2_ZoomMaximum_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x42ab6b77_ceb0_4eca_b82a_6cfa5fa1fc08);
pub const Transform2_ZoomMinimum_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x742ccc16_4ad1_4e07_96fe_b122c6e6b22b);
pub const Transform_CanMove_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x1b75824d_208b_4fdf_bccd_f1f4e5741f4f);
pub const Transform_CanResize_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xbb98dca5_4c1a_41d4_a4f6_ebc128644180);
pub const Transform_CanRotate_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x10079b48_3849_476f_ac96_44a95c8440d9);
pub const Transform_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x24b46fdb_587e_49f1_9c4a_d8e98b664b7b);
pub const TreeItem_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x62c9feb9_8ffc_4878_a3a4_96b030315c18);
pub const Tree_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x7561349c_d241_43f4_9908_b5f091bee611);
pub const UIA_E_ELEMENTNOTAVAILABLE: u32 = 2147746305;
pub const UIA_E_ELEMENTNOTENABLED: u32 = 2147746304;
pub const UIA_E_INVALIDOPERATION: u32 = 2148734217;
pub const UIA_E_NOCLICKABLEPOINT: u32 = 2147746306;
pub const UIA_E_NOTSUPPORTED: u32 = 2147746308;
pub const UIA_E_PROXYASSEMBLYNOTLOADED: u32 = 2147746307;
pub const UIA_E_TIMEOUT: u32 = 2148734213;
pub const UIA_IAFP_DEFAULT: i32 = 0;
pub const UIA_IAFP_UNWRAP_BRIDGE: i32 = 1;
pub const UIA_PFIA_DEFAULT: i32 = 0;
pub const UIA_PFIA_UNWRAP_BRIDGE: i32 = 1;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiaAndOrCondition {
    pub ConditionType: ConditionType,
    pub ppConditions: *mut *mut UiaCondition,
    pub cConditions: i32,
}
pub const UiaAppendRuntimeId: i32 = 3;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UiaAsyncContentLoadedEventArgs {
    pub Type: EventArgsType,
    pub EventId: i32,
    pub AsyncContentLoadedState: AsyncContentLoadedState,
    pub PercentComplete: f64,
}
#[repr(C)]
#[cfg(all(feature = "shared", feature = "uiautomationclient", feature = "uiautomationcore"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiaCacheRequest {
    pub pViewCondition: *mut UiaCondition,
    pub Scope: super::TreeScope,
    pub pProperties: *mut super::PROPERTYID,
    pub cProperties: i32,
    pub pPatterns: *mut super::PATTERNID,
    pub cPatterns: i32,
    pub automationElementMode: super::AutomationElementMode,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiaChangesEventArgs {
    pub Type: EventArgsType,
    pub EventId: i32,
    pub EventIdCount: i32,
    pub pUiaChanges: *mut super::UiaChangeInfo,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiaCondition {
    pub ConditionType: ConditionType,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiaEventArgs {
    pub Type: EventArgsType,
    pub EventId: i32,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
pub type UiaEventCallback = Option<unsafe extern "system" fn(pargs: *mut UiaEventArgs, prequesteddata: *mut super::SAFEARRAY, ptreestructure: windows_core::BSTR)>;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiaFindParams {
    pub MaxDepth: i32,
    pub FindFirst: windows_core::BOOL,
    pub ExcludeRoot: windows_core::BOOL,
    pub pFindCondition: *mut UiaCondition,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiaNotCondition {
    pub ConditionType: ConditionType,
    pub pCondition: *mut UiaCondition,
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub struct UiaPropertyChangedEventArgs {
    pub Type: EventArgsType,
    pub EventId: i32,
    pub PropertyId: super::PROPERTYID,
    pub OldValue: super::VARIANT,
    pub NewValue: super::VARIANT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for UiaPropertyChangedEventArgs {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for UiaPropertyChangedEventArgs {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationclient", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub struct UiaPropertyCondition {
    pub ConditionType: ConditionType,
    pub PropertyId: super::PROPERTYID,
    pub Value: super::VARIANT,
    pub Flags: super::PropertyConditionFlags,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationclient", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for UiaPropertyCondition {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "uiautomationclient", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Default for UiaPropertyCondition {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "windef", feature = "winnt"))]
pub type UiaProviderCallback = Option<unsafe extern "system" fn(hwnd: super::HWND, providertype: ProviderType) -> *mut super::SAFEARRAY>;
pub const UiaRootObjectId: i32 = -25;
#[repr(C)]
#[cfg(feature = "uiautomationcore")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiaStructureChangedEventArgs {
    pub Type: EventArgsType,
    pub EventId: i32,
    pub StructureChangeType: super::StructureChangeType,
    pub pRuntimeId: *mut i32,
    pub cRuntimeIdLen: i32,
}
#[repr(C)]
#[cfg(all(feature = "oaidl", feature = "shared", feature = "uiautomationcore", feature = "winnt"))]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiaTextEditTextChangedEventArgs {
    pub Type: EventArgsType,
    pub EventId: i32,
    pub TextEditChangeType: super::TextEditChangeType,
    pub pTextChange: *mut super::SAFEARRAY,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiaWindowClosedEventArgs {
    pub Type: EventArgsType,
    pub EventId: i32,
    pub pRuntimeId: *mut i32,
    pub cRuntimeIdLen: i32,
}
pub const Value_IsReadOnly_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xeb090f30_e24c_4799_a705_0d247bc037f8);
pub const Value_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x17faad9e_c877_475b_b933_77332779b637);
pub const Value_Value_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe95f5e64_269f_4a85_ba99_4092c3ea2986);
pub const VirtualizedItem_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xf510173e_2e71_45e9_a6e5_62f6ed8289d5);
pub const VisualEffects_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe61a8565_aad9_46d7_9e70_4e8a8420d420);
pub const Window_CanMaximize_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x64fff53f_635d_41c1_950c_cb5adfbe28e3);
pub const Window_CanMinimize_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xb73b4625_5988_4b97_b4c2_a6fe6e78c8c6);
pub const Window_Control_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xe13a7242_f462_4f4d_aec1_53b28d6c3290);
pub const Window_IsModal_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xff4e6892_37b9_4fca_8532_ffe674ecfeed);
pub const Window_IsTopmost_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xef7d85d3_0937_4962_9241_b62345f24041);
pub const Window_Pattern_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x27901735_c760_4994_ad11_5919e606b110);
pub const Window_WindowClosed_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xedf141f8_fa67_4e22_bbf7_944e05735ee2);
pub const Window_WindowInteractionState_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x4fed26a4_0455_4fa2_b21c_c4da2db1ff9c);
pub const Window_WindowOpened_Event_GUID: windows_core::GUID = windows_core::GUID::from_u128(0xd3e81d06_de45_4f2f_9633_de9e02fb65af);
pub const Window_WindowVisualState_Property_GUID: windows_core::GUID = windows_core::GUID::from_u128(0x4ab7905f_e860_453e_a30a_f6431e5daad5);
