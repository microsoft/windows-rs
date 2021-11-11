#![allow(non_snake_case, non_camel_case_types)]
#[link(name = "windows")]
extern "system" {
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccNotifyTouchInteraction(hwndapp: super::super::Foundation::HWND, hwndtarget: super::super::Foundation::HWND, pttarget: super::super::Foundation::POINT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccSetRunningUtilityState(hwndapp: super::super::Foundation::HWND, dwutilitystatemask: u32, dwutilitystate: ACC_UTILITY_STATE_FLAGS) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn AccessibleChildren(pacccontainer: ::windows::runtime::RawPtr, ichildstart: i32, cchildren: i32, rgvarchildren: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pcobtained: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn AccessibleObjectFromEvent(hwnd: super::super::Foundation::HWND, dwid: u32, dwchildid: u32, ppacc: *mut ::windows::runtime::RawPtr, pvarchild: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn AccessibleObjectFromPoint(ptscreen: super::super::Foundation::POINT, ppacc: *mut ::windows::runtime::RawPtr, pvarchild: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn AccessibleObjectFromWindow(hwnd: super::super::Foundation::HWND, dwid: u32, riid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateStdAccessibleObject(hwnd: super::super::Foundation::HWND, idobject: i32, riid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateStdAccessibleProxyA(hwnd: super::super::Foundation::HWND, pclassname: super::super::Foundation::PSTR, idobject: i32, riid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn CreateStdAccessibleProxyW(hwnd: super::super::Foundation::HWND, pclassname: super::super::Foundation::PWSTR, idobject: i32, riid: *const ::windows::runtime::GUID, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn DockPattern_SetDockPosition(hobj: HUIAPATTERNOBJECT, dockposition: DockPosition) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn ExpandCollapsePattern_Collapse(hobj: HUIAPATTERNOBJECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn ExpandCollapsePattern_Expand(hobj: HUIAPATTERNOBJECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn GetOleaccVersionInfo(pver: *mut u32, pbuild: *mut u32);
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRoleTextA(lrole: u32, lpszrole: super::super::Foundation::PSTR, cchrolemax: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetRoleTextW(lrole: u32, lpszrole: super::super::Foundation::PWSTR, cchrolemax: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStateTextA(lstatebit: u32, lpszstate: super::super::Foundation::PSTR, cchstate: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn GetStateTextW(lstatebit: u32, lpszstate: super::super::Foundation::PWSTR, cchstate: u32) -> u32;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn GridPattern_GetItem(hobj: HUIAPATTERNOBJECT, row: i32, column: i32, presult: *mut HUIANODE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn InvokePattern_Invoke(hobj: HUIAPATTERNOBJECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn IsWinEventHookInstalled(event: u32) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn ItemContainerPattern_FindItemByProperty(hobj: HUIAPATTERNOBJECT, hnodestartafter: HUIANODE, propertyid: i32, value: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pfound: *mut HUIANODE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn LegacyIAccessiblePattern_DoDefaultAction(hobj: HUIAPATTERNOBJECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn LegacyIAccessiblePattern_GetIAccessible(hobj: HUIAPATTERNOBJECT, paccessible: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn LegacyIAccessiblePattern_Select(hobj: HUIAPATTERNOBJECT, flagsselect: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LegacyIAccessiblePattern_SetValue(hobj: HUIAPATTERNOBJECT, szvalue: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn LresultFromObject(riid: *const ::windows::runtime::GUID, wparam: super::super::Foundation::WPARAM, punk: ::windows::runtime::RawPtr) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn MultipleViewPattern_GetViewName(hobj: HUIAPATTERNOBJECT, viewid: i32, ppstr: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn MultipleViewPattern_SetCurrentView(hobj: HUIAPATTERNOBJECT, viewid: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn NotifyWinEvent(event: u32, hwnd: super::super::Foundation::HWND, idobject: i32, idchild: i32);
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ObjectFromLresult(lresult: super::super::Foundation::LRESULT, riid: *const ::windows::runtime::GUID, wparam: super::super::Foundation::WPARAM, ppvobject: *mut *mut ::core::ffi::c_void) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn RangeValuePattern_SetValue(hobj: HUIAPATTERNOBJECT, val: f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn RegisterPointerInputTarget(hwnd: super::super::Foundation::HWND, pointertype: super::WindowsAndMessaging::POINTER_INPUT_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn RegisterPointerInputTargetEx(hwnd: super::super::Foundation::HWND, pointertype: super::WindowsAndMessaging::POINTER_INPUT_TYPE, fobserve: super::super::Foundation::BOOL) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn ScrollItemPattern_ScrollIntoView(hobj: HUIAPATTERNOBJECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn ScrollPattern_Scroll(hobj: HUIAPATTERNOBJECT, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn ScrollPattern_SetScrollPercent(hobj: HUIAPATTERNOBJECT, horizontalpercent: f64, verticalpercent: f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn SelectionItemPattern_AddToSelection(hobj: HUIAPATTERNOBJECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn SelectionItemPattern_RemoveFromSelection(hobj: HUIAPATTERNOBJECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn SelectionItemPattern_Select(hobj: HUIAPATTERNOBJECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn SetWinEventHook(eventmin: u32, eventmax: u32, hmodwineventproc: super::super::Foundation::HINSTANCE, pfnwineventproc: ::windows::runtime::RawPtr, idprocess: u32, idthread: u32, dwflags: u32) -> HWINEVENTHOOK;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn SynchronizedInputPattern_Cancel(hobj: HUIAPATTERNOBJECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn SynchronizedInputPattern_StartListening(hobj: HUIAPATTERNOBJECT, inputtype: SynchronizedInputType) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn TextPattern_GetSelection(hobj: HUIAPATTERNOBJECT, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn TextPattern_GetVisibleRanges(hobj: HUIAPATTERNOBJECT, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextPattern_RangeFromChild(hobj: HUIAPATTERNOBJECT, hnodechild: HUIANODE, pretval: *mut HUIATEXTRANGE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextPattern_RangeFromPoint(hobj: HUIAPATTERNOBJECT, point: UiaPoint, pretval: *mut HUIATEXTRANGE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextPattern_get_DocumentRange(hobj: HUIAPATTERNOBJECT, pretval: *mut HUIATEXTRANGE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextPattern_get_SupportedTextSelection(hobj: HUIAPATTERNOBJECT, pretval: *mut SupportedTextSelection) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextRange_AddToSelection(hobj: HUIATEXTRANGE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextRange_Clone(hobj: HUIATEXTRANGE, pretval: *mut HUIATEXTRANGE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TextRange_Compare(hobj: HUIATEXTRANGE, range: HUIATEXTRANGE, pretval: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextRange_CompareEndpoints(hobj: HUIATEXTRANGE, endpoint: TextPatternRangeEndpoint, targetrange: HUIATEXTRANGE, targetendpoint: TextPatternRangeEndpoint, pretval: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextRange_ExpandToEnclosingUnit(hobj: HUIATEXTRANGE, unit: TextUnit) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn TextRange_FindAttribute(hobj: HUIATEXTRANGE, attributeid: i32, val: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, backward: super::super::Foundation::BOOL, pretval: *mut HUIATEXTRANGE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TextRange_FindText(hobj: HUIATEXTRANGE, text: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, backward: super::super::Foundation::BOOL, ignorecase: super::super::Foundation::BOOL, pretval: *mut HUIATEXTRANGE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn TextRange_GetAttributeValue(hobj: HUIATEXTRANGE, attributeid: i32, pretval: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn TextRange_GetBoundingRectangles(hobj: HUIATEXTRANGE, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn TextRange_GetChildren(hobj: HUIATEXTRANGE, pretval: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextRange_GetEnclosingElement(hobj: HUIATEXTRANGE, pretval: *mut HUIANODE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TextRange_GetText(hobj: HUIATEXTRANGE, maxlength: i32, pretval: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextRange_Move(hobj: HUIATEXTRANGE, unit: TextUnit, count: i32, pretval: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextRange_MoveEndpointByRange(hobj: HUIATEXTRANGE, endpoint: TextPatternRangeEndpoint, targetrange: HUIATEXTRANGE, targetendpoint: TextPatternRangeEndpoint) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextRange_MoveEndpointByUnit(hobj: HUIATEXTRANGE, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32, pretval: *mut i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextRange_RemoveFromSelection(hobj: HUIATEXTRANGE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn TextRange_ScrollIntoView(hobj: HUIATEXTRANGE, aligntotop: super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TextRange_Select(hobj: HUIATEXTRANGE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TogglePattern_Toggle(hobj: HUIAPATTERNOBJECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TransformPattern_Move(hobj: HUIAPATTERNOBJECT, x: f64, y: f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TransformPattern_Resize(hobj: HUIAPATTERNOBJECT, width: f64, height: f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn TransformPattern_Rotate(hobj: HUIAPATTERNOBJECT, degrees: f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn UiaAddEvent(hnode: HUIANODE, eventid: i32, pcallback: *mut ::windows::runtime::RawPtr, scope: TreeScope, pproperties: *mut i32, cproperties: i32, prequest: *mut UiaCacheRequest, phevent: *mut HUIAEVENT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UiaClientsAreListening() -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaDisconnectAllProviders() -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaDisconnectProvider(pprovider: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UiaEventAddWindow(hevent: HUIAEVENT, hwnd: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UiaEventRemoveWindow(hevent: HUIAEVENT, hwnd: super::super::Foundation::HWND) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn UiaFind(hnode: HUIANODE, pparams: *mut UiaFindParams, prequest: *mut UiaCacheRequest, pprequesteddata: *mut *mut super::super::System::Com::SAFEARRAY, ppoffsets: *mut *mut super::super::System::Com::SAFEARRAY, pptreestructures: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UiaGetErrorDescription(pdescription: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaGetPatternProvider(hnode: HUIANODE, patternid: i32, phobj: *mut HUIAPATTERNOBJECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn UiaGetPropertyValue(hnode: HUIANODE, propertyid: i32, pvalue: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaGetReservedMixedAttributeValue(punkmixedattributevalue: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaGetReservedNotSupportedValue(punknotsupportedvalue: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaGetRootNode(phnode: *mut HUIANODE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn UiaGetRuntimeId(hnode: HUIANODE, pruntimeid: *mut *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn UiaGetUpdatedCache(hnode: HUIANODE, prequest: *mut UiaCacheRequest, normalizestate: NormalizeState, pnormalizecondition: *mut UiaCondition, pprequesteddata: *mut *mut super::super::System::Com::SAFEARRAY, pptreestructure: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn UiaHPatternObjectFromVariant(pvar: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, phobj: *mut HUIAPATTERNOBJECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn UiaHTextRangeFromVariant(pvar: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, phtextrange: *mut HUIATEXTRANGE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn UiaHUiaNodeFromVariant(pvar: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, phnode: *mut HUIANODE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UiaHasServerSideProvider(hwnd: super::super::Foundation::HWND) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UiaHostProviderFromHwnd(hwnd: super::super::Foundation::HWND, ppprovider: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn UiaIAccessibleFromProvider(pprovider: ::windows::runtime::RawPtr, dwflags: u32, ppaccessible: *mut ::windows::runtime::RawPtr, pvarchild: *mut ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaLookupId(r#type: AutomationIdentifierType, pguid: *const ::windows::runtime::GUID) -> i32;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn UiaNavigate(hnode: HUIANODE, direction: NavigateDirection, pcondition: *mut UiaCondition, prequest: *mut UiaCacheRequest, pprequesteddata: *mut *mut super::super::System::Com::SAFEARRAY, pptreestructure: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn UiaNodeFromFocus(prequest: *mut UiaCacheRequest, pprequesteddata: *mut *mut super::super::System::Com::SAFEARRAY, pptreestructure: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UiaNodeFromHandle(hwnd: super::super::Foundation::HWND, phnode: *mut HUIANODE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn UiaNodeFromPoint(x: f64, y: f64, prequest: *mut UiaCacheRequest, pprequesteddata: *mut *mut super::super::System::Com::SAFEARRAY, pptreestructure: *mut ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaNodeFromProvider(pprovider: ::windows::runtime::RawPtr, phnode: *mut HUIANODE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UiaNodeRelease(hnode: HUIANODE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UiaPatternRelease(hobj: HUIAPATTERNOBJECT) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UiaProviderForNonClient(hwnd: super::super::Foundation::HWND, idobject: i32, idchild: i32, ppprovider: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaProviderFromIAccessible(paccessible: ::windows::runtime::RawPtr, idchild: i32, dwflags: u32, ppprovider: *mut ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaRaiseActiveTextPositionChangedEvent(provider: ::windows::runtime::RawPtr, textrange: ::windows::runtime::RawPtr) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaRaiseAsyncContentLoadedEvent(pprovider: ::windows::runtime::RawPtr, asynccontentloadedstate: AsyncContentLoadedState, percentcomplete: f64) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaRaiseAutomationEvent(pprovider: ::windows::runtime::RawPtr, id: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn UiaRaiseAutomationPropertyChangedEvent(pprovider: ::windows::runtime::RawPtr, id: i32, oldvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, newvalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`, `Win32_System_Ole`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
    pub fn UiaRaiseChangesEvent(pprovider: ::windows::runtime::RawPtr, eventidcount: i32, puiachanges: *mut ::core::mem::ManuallyDrop<UiaChangeInfo>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UiaRaiseNotificationEvent(provider: ::windows::runtime::RawPtr, notificationkind: NotificationKind, notificationprocessing: NotificationProcessing, displaystring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, activityid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaRaiseStructureChangedEvent(pprovider: ::windows::runtime::RawPtr, structurechangetype: StructureChangeType, pruntimeid: *mut i32, cruntimeidlen: i32) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_System_Com`*"]
    #[cfg(feature = "Win32_System_Com")]
    pub fn UiaRaiseTextEditTextChangedEvent(pprovider: ::windows::runtime::RawPtr, texteditchangetype: TextEditChangeType, pchangeddata: *mut super::super::System::Com::SAFEARRAY) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_System_Com`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com"))]
    pub fn UiaRegisterProviderCallback(pcallback: *mut ::windows::runtime::RawPtr);
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaRemoveEvent(hevent: HUIAEVENT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UiaReturnRawElementProvider(hwnd: super::super::Foundation::HWND, wparam: super::super::Foundation::WPARAM, lparam: super::super::Foundation::LPARAM, el: ::windows::runtime::RawPtr) -> super::super::Foundation::LRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn UiaSetFocus(hnode: HUIANODE) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UiaTextRangeRelease(hobj: HUIATEXTRANGE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn UnhookWinEvent(hwineventhook: HWINEVENTHOOK) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn UnregisterPointerInputTarget(hwnd: super::super::Foundation::HWND, pointertype: super::WindowsAndMessaging::POINTER_INPUT_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`, `Win32_UI_WindowsAndMessaging`*"]
    #[cfg(all(feature = "Win32_Foundation", feature = "Win32_UI_WindowsAndMessaging"))]
    pub fn UnregisterPointerInputTargetEx(hwnd: super::super::Foundation::HWND, pointertype: super::WindowsAndMessaging::POINTER_INPUT_TYPE) -> super::super::Foundation::BOOL;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn ValuePattern_SetValue(hobj: HUIAPATTERNOBJECT, pval: super::super::Foundation::PWSTR) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn VirtualizedItemPattern_Realize(hobj: HUIAPATTERNOBJECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowFromAccessibleObject(param0: ::windows::runtime::RawPtr, phwnd: *mut super::super::Foundation::HWND) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn WindowPattern_Close(hobj: HUIAPATTERNOBJECT) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`*"]
    pub fn WindowPattern_SetWindowVisualState(hobj: HUIAPATTERNOBJECT, state: WindowVisualState) -> ::windows::runtime::HRESULT;
    #[doc = "*Required features: `Win32_UI_Accessibility`, `Win32_Foundation`*"]
    #[cfg(feature = "Win32_Foundation")]
    pub fn WindowPattern_WaitForInputIdle(hobj: HUIAPATTERNOBJECT, milliseconds: i32, presult: *mut super::super::Foundation::BOOL) -> ::windows::runtime::HRESULT;
}
