windows_core::link!("ole32.dll" "system" fn CoCreateInstance(rclsid : *const windows_core::GUID, punkouter : *mut core::ffi::c_void, dwclscontext : u32, riid : *const windows_core::GUID, ppv : *mut *mut core::ffi::c_void) -> windows_core::HRESULT);
windows_core::link!("ole32.dll" "system" fn CoInitializeEx(pvreserved : *const core::ffi::c_void, dwcoinit : u32) -> windows_core::HRESULT);
windows_core::link!("user32.dll" "system" fn SendMessageW(hwnd : HWND, msg : u32, wparam : WPARAM, lparam : LPARAM) -> LRESULT);
pub type COINIT = i32;
pub const COINIT_MULTITHREADED: COINIT = 0;
pub type CONTROLTYPEID = i32;
pub const CUIAutomation: windows_core::GUID =
    windows_core::GUID::from_u128(0xff48dba4_60ef_4201_aa87_54103eef594e);
pub type HWND = *mut core::ffi::c_void;
windows_core::imp::define_interface!(
    IUIAutomation,
    IUIAutomation_Vtbl,
    0x30cbe57d_d9d0_452a_ab13_7ac5ac4825ee
);
windows_core::imp::interface_hierarchy!(IUIAutomation, windows_core::IUnknown);
impl IUIAutomation {
    pub(crate) unsafe fn GetRootElement(&self) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRootElement)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CreateTrueCondition(
        &self,
    ) -> windows_core::Result<IUIAutomationCondition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateTrueCondition)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IUIAutomation_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    CompareElements: usize,
    CompareRuntimeIds: usize,
    pub GetRootElement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    ElementFromHandle: usize,
    ElementFromPoint: usize,
    GetFocusedElement: usize,
    GetRootElementBuildCache: usize,
    ElementFromHandleBuildCache: usize,
    ElementFromPointBuildCache: usize,
    GetFocusedElementBuildCache: usize,
    CreateTreeWalker: usize,
    ControlViewWalker: usize,
    ContentViewWalker: usize,
    RawViewWalker: usize,
    RawViewCondition: usize,
    ControlViewCondition: usize,
    ContentViewCondition: usize,
    CreateCacheRequest: usize,
    pub CreateTrueCondition: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CreateFalseCondition: usize,
    CreatePropertyCondition: usize,
    CreatePropertyConditionEx: usize,
    CreateAndCondition: usize,
    CreateAndConditionFromArray: usize,
    CreateAndConditionFromNativeArray: usize,
    CreateOrCondition: usize,
    CreateOrConditionFromArray: usize,
    CreateOrConditionFromNativeArray: usize,
    CreateNotCondition: usize,
    AddAutomationEventHandler: usize,
    RemoveAutomationEventHandler: usize,
    AddPropertyChangedEventHandlerNativeArray: usize,
    AddPropertyChangedEventHandler: usize,
    RemovePropertyChangedEventHandler: usize,
    AddStructureChangedEventHandler: usize,
    RemoveStructureChangedEventHandler: usize,
    AddFocusChangedEventHandler: usize,
    RemoveFocusChangedEventHandler: usize,
    RemoveAllEventHandlers: usize,
    IntNativeArrayToSafeArray: usize,
    IntSafeArrayToNativeArray: usize,
    RectToVariant: usize,
    VariantToRect: usize,
    SafeArrayToRectNativeArray: usize,
    CreateProxyFactoryEntry: usize,
    ProxyFactoryMapping: usize,
    GetPropertyProgrammaticName: usize,
    GetPatternProgrammaticName: usize,
    PollForPotentialSupportedPatterns: usize,
    PollForPotentialSupportedProperties: usize,
    CheckNotSupported: usize,
    ReservedNotSupportedValue: usize,
    ReservedMixedAttributeValue: usize,
    ElementFromIAccessible: usize,
    ElementFromIAccessibleBuildCache: usize,
}
impl windows_core::RuntimeName for IUIAutomation {}
windows_core::imp::define_interface!(
    IUIAutomationCondition,
    IUIAutomationCondition_Vtbl,
    0x352ffba8_0973_437c_a61f_f64cafd81df9
);
windows_core::imp::interface_hierarchy!(IUIAutomationCondition, windows_core::IUnknown);
#[repr(C)]
pub struct IUIAutomationCondition_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
}
pub trait IUIAutomationCondition_Impl: windows_core::IUnknownImpl {}
impl IUIAutomationCondition_Vtbl {
    pub const fn new<Identity: IUIAutomationCondition_Impl, const OFFSET: isize>() -> Self {
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationCondition as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationCondition {}
windows_core::imp::define_interface!(
    IUIAutomationElement,
    IUIAutomationElement_Vtbl,
    0xd22108aa_8ac5_49a5_837b_37bbb3d7591e
);
windows_core::imp::interface_hierarchy!(IUIAutomationElement, windows_core::IUnknown);
impl IUIAutomationElement {
    pub(crate) unsafe fn FindAll<P1>(
        &self,
        scope: TreeScope,
        condition: P1,
    ) -> windows_core::Result<IUIAutomationElementArray>
    where
        P1: windows_core::Param<IUIAutomationCondition>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindAll)(
                windows_core::Interface::as_raw(self),
                scope,
                condition.param().abi(),
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn GetCurrentPatternAs<T>(
        &self,
        patternid: PATTERNID,
    ) -> windows_core::Result<T>
    where
        T: windows_core::Interface,
    {
        let mut result__ = core::ptr::null_mut();
        unsafe {
            (windows_core::Interface::vtable(self).GetCurrentPatternAs)(
                windows_core::Interface::as_raw(self),
                patternid,
                &T::IID,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub(crate) unsafe fn CurrentControlType(&self) -> windows_core::Result<CONTROLTYPEID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentControlType)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn CurrentName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentName)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
pub struct IUIAutomationElement_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    SetFocus: usize,
    GetRuntimeId: usize,
    FindFirst: usize,
    pub FindAll: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        TreeScope,
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    FindFirstBuildCache: usize,
    FindAllBuildCache: usize,
    BuildUpdatedCache: usize,
    GetCurrentPropertyValue: usize,
    GetCurrentPropertyValueEx: usize,
    GetCachedPropertyValue: usize,
    GetCachedPropertyValueEx: usize,
    pub GetCurrentPatternAs: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        PATTERNID,
        *const windows_core::GUID,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    GetCachedPatternAs: usize,
    GetCurrentPattern: usize,
    GetCachedPattern: usize,
    GetCachedParent: usize,
    GetCachedChildren: usize,
    CurrentProcessId: usize,
    pub CurrentControlType: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut CONTROLTYPEID,
    ) -> windows_core::HRESULT,
    CurrentLocalizedControlType: usize,
    pub CurrentName: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
    CurrentAcceleratorKey: usize,
    CurrentAccessKey: usize,
    CurrentHasKeyboardFocus: usize,
    CurrentIsKeyboardFocusable: usize,
    CurrentIsEnabled: usize,
    CurrentAutomationId: usize,
    CurrentClassName: usize,
    CurrentHelpText: usize,
    CurrentCulture: usize,
    CurrentIsControlElement: usize,
    CurrentIsContentElement: usize,
    CurrentIsPassword: usize,
    CurrentNativeWindowHandle: usize,
    CurrentItemType: usize,
    CurrentIsOffscreen: usize,
    CurrentOrientation: usize,
    CurrentFrameworkId: usize,
    CurrentIsRequiredForForm: usize,
    CurrentItemStatus: usize,
    CurrentBoundingRectangle: usize,
    CurrentLabeledBy: usize,
    CurrentAriaRole: usize,
    CurrentAriaProperties: usize,
    CurrentIsDataValidForForm: usize,
    CurrentControllerFor: usize,
    CurrentDescribedBy: usize,
    CurrentFlowsTo: usize,
    CurrentProviderDescription: usize,
    CachedProcessId: usize,
    CachedControlType: usize,
    CachedLocalizedControlType: usize,
    CachedName: usize,
    CachedAcceleratorKey: usize,
    CachedAccessKey: usize,
    CachedHasKeyboardFocus: usize,
    CachedIsKeyboardFocusable: usize,
    CachedIsEnabled: usize,
    CachedAutomationId: usize,
    CachedClassName: usize,
    CachedHelpText: usize,
    CachedCulture: usize,
    CachedIsControlElement: usize,
    CachedIsContentElement: usize,
    CachedIsPassword: usize,
    CachedNativeWindowHandle: usize,
    CachedItemType: usize,
    CachedIsOffscreen: usize,
    CachedOrientation: usize,
    CachedFrameworkId: usize,
    CachedIsRequiredForForm: usize,
    CachedItemStatus: usize,
    CachedBoundingRectangle: usize,
    CachedLabeledBy: usize,
    CachedAriaRole: usize,
    CachedAriaProperties: usize,
    CachedIsDataValidForForm: usize,
    CachedControllerFor: usize,
    CachedDescribedBy: usize,
    CachedFlowsTo: usize,
    CachedProviderDescription: usize,
    GetClickablePoint: usize,
}
impl windows_core::RuntimeName for IUIAutomationElement {}
windows_core::imp::define_interface!(
    IUIAutomationElementArray,
    IUIAutomationElementArray_Vtbl,
    0x14314595_b4bc_4055_95f2_58f2e42c9855
);
windows_core::imp::interface_hierarchy!(IUIAutomationElementArray, windows_core::IUnknown);
impl IUIAutomationElementArray {
    pub(crate) unsafe fn Length(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Length)(
                windows_core::Interface::as_raw(self),
                &mut result__,
            )
            .map(|| result__)
        }
    }
    pub(crate) unsafe fn GetElement(
        &self,
        index: i32,
    ) -> windows_core::Result<IUIAutomationElement> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetElement)(
                windows_core::Interface::as_raw(self),
                index,
                &mut result__,
            )
            .and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
pub struct IUIAutomationElementArray_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Length:
        unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub GetElement: unsafe extern "system" fn(
        *mut core::ffi::c_void,
        i32,
        *mut *mut core::ffi::c_void,
    ) -> windows_core::HRESULT,
}
pub trait IUIAutomationElementArray_Impl: windows_core::IUnknownImpl {
    fn Length(&self) -> windows_core::Result<i32>;
    fn GetElement(&self, index: i32) -> windows_core::Result<IUIAutomationElement>;
}
impl IUIAutomationElementArray_Vtbl {
    pub const fn new<Identity: IUIAutomationElementArray_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Length<
            Identity: IUIAutomationElementArray_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            length: *mut i32,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElementArray_Impl::Length(this) {
                    Ok(ok__) => {
                        length.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetElement<
            Identity: IUIAutomationElementArray_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
            index: i32,
            element: *mut *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationElementArray_Impl::GetElement(
                    this,
                    core::mem::transmute_copy(&index),
                ) {
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
            Length: Length::<Identity, OFFSET>,
            GetElement: GetElement::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationElementArray as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationElementArray {}
windows_core::imp::define_interface!(
    IUIAutomationInvokePattern,
    IUIAutomationInvokePattern_Vtbl,
    0xfb377fbe_8ea6_46d5_9c73_6499642d3059
);
windows_core::imp::interface_hierarchy!(IUIAutomationInvokePattern, windows_core::IUnknown);
impl IUIAutomationInvokePattern {
    pub(crate) unsafe fn Invoke(&self) -> windows_core::HRESULT {
        unsafe {
            (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self))
        }
    }
}
#[repr(C)]
pub struct IUIAutomationInvokePattern_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAutomationInvokePattern_Impl: windows_core::IUnknownImpl {
    fn Invoke(&self) -> windows_core::Result<()>;
}
impl IUIAutomationInvokePattern_Vtbl {
    pub const fn new<Identity: IUIAutomationInvokePattern_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Invoke<
            Identity: IUIAutomationInvokePattern_Impl,
            const OFFSET: isize,
        >(
            this: *mut core::ffi::c_void,
        ) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity =
                    &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationInvokePattern_Impl::Invoke(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Invoke: Invoke::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationInvokePattern as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationInvokePattern {}
pub type LPARAM = isize;
pub type LRESULT = isize;
pub const NIN_SELECT: i32 = 1024;
pub type PATTERNID = i32;
pub type TreeScope = i32;
pub const TreeScope_Descendants: TreeScope = 4;
pub const WM_CONTEXTMENU: i32 = 123;
pub const WM_USER: i32 = 1024;
pub type WPARAM = usize;
