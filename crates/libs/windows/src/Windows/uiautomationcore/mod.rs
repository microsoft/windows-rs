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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct CONTROLTYPEID(pub i32);
pub const CUIAutomationClientInfo: windows_core::GUID = windows_core::GUID::from_u128(0xc2d4f567_8a9b_4c3e_9f1a_2b5c7d8e0f3a);
pub const CUIAutomationClientInfoSource: windows_core::GUID = windows_core::GUID::from_u128(0xa8d4f123_7b2c_4e5f_9a1b_3c8d6e9f0a2b);
pub const CUIAutomationRegistrar: windows_core::GUID = windows_core::GUID::from_u128(0x6e29fabf_9977_42d1_8d0e_ca7e61ad87e6);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct EVENTID(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct HEADINGLEVELID(pub i32);
pub type HorizontalTextAlignment = i32;
pub const HorizontalTextAlignment_Centered: HorizontalTextAlignment = 1;
pub const HorizontalTextAlignment_Justified: HorizontalTextAlignment = 3;
pub const HorizontalTextAlignment_Left: HorizontalTextAlignment = 0;
pub const HorizontalTextAlignment_Right: HorizontalTextAlignment = 2;
windows_core::imp::define_interface!(IAccessibleEx, IAccessibleEx_Vtbl, 0xf8b80ada_2c44_48d0_89be_5ff23c9cd875);
windows_core::imp::interface_hierarchy!(IAccessibleEx, windows_core::IUnknown);
impl IAccessibleEx {
    pub unsafe fn GetObjectForChild(&self, idchild: i32) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectForChild)(windows_core::Interface::as_raw(self), idchild, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "oleacc"))]
    pub unsafe fn GetIAccessiblePair(&self, ppacc: *mut Option<super::oleacc::IAccessible>, pidchild: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetIAccessiblePair)(windows_core::Interface::as_raw(self), core::mem::transmute(ppacc), pidchild as _) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetRuntimeId(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRuntimeId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ConvertReturnedElement<P0>(&self, pin: P0) -> windows_core::Result<Self>
    where
        P0: windows_core::Param<IRawElementProviderSimple>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ConvertReturnedElement)(windows_core::Interface::as_raw(self), pin.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessibleEx_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetObjectForChild: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "oleacc"))]
    pub GetIAccessiblePair: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "oleacc")))]
    GetIAccessiblePair: usize,
    #[cfg(feature = "oaidl")]
    pub GetRuntimeId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetRuntimeId: usize,
    pub ConvertReturnedElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "oleacc"))]
pub trait IAccessibleEx_Impl: windows_core::IUnknownImpl {
    fn GetObjectForChild(&self, idchild: i32) -> windows_core::Result<IAccessibleEx>;
    fn GetIAccessiblePair(&self, ppacc: windows_core::OutRef<super::oleacc::IAccessible>, pidchild: *mut i32) -> windows_core::Result<()>;
    fn GetRuntimeId(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn ConvertReturnedElement(&self, pin: windows_core::Ref<IRawElementProviderSimple>) -> windows_core::Result<IAccessibleEx>;
}
#[cfg(all(feature = "oaidl", feature = "oleacc"))]
impl IAccessibleEx_Vtbl {
    pub const fn new<Identity: IAccessibleEx_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetObjectForChild<Identity: IAccessibleEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idchild: i32, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessibleEx_Impl::GetObjectForChild(this, core::mem::transmute_copy(&idchild)) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetIAccessiblePair<Identity: IAccessibleEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppacc: *mut *mut core::ffi::c_void, pidchild: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IAccessibleEx_Impl::GetIAccessiblePair(this, core::mem::transmute_copy(&ppacc), core::mem::transmute_copy(&pidchild)).into()
            }
        }
        unsafe extern "system" fn GetRuntimeId<Identity: IAccessibleEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessibleEx_Impl::GetRuntimeId(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ConvertReturnedElement<Identity: IAccessibleEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pin: *mut core::ffi::c_void, ppretvalout: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessibleEx_Impl::ConvertReturnedElement(this, core::mem::transmute_copy(&pin)) {
                    Ok(ok__) => {
                        ppretvalout.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetObjectForChild: GetObjectForChild::<Identity, OFFSET>,
            GetIAccessiblePair: GetIAccessiblePair::<Identity, OFFSET>,
            GetRuntimeId: GetRuntimeId::<Identity, OFFSET>,
            ConvertReturnedElement: ConvertReturnedElement::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccessibleEx as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "oleacc"))]
impl windows_core::RuntimeName for IAccessibleEx {}
windows_core::imp::define_interface!(IAccessibleHostingElementProviders, IAccessibleHostingElementProviders_Vtbl, 0x33ac331b_943e_4020_b295_db37784974a3);
windows_core::imp::interface_hierarchy!(IAccessibleHostingElementProviders, windows_core::IUnknown);
impl IAccessibleHostingElementProviders {
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetEmbeddedFragmentRoots(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEmbeddedFragmentRoots)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetObjectIdForProvider<P0>(&self, pprovider: P0) -> windows_core::Result<i32>
    where
        P0: windows_core::Param<IRawElementProviderSimple>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectIdForProvider)(windows_core::Interface::as_raw(self), pprovider.param().abi(), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAccessibleHostingElementProviders_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "oaidl")]
    pub GetEmbeddedFragmentRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetEmbeddedFragmentRoots: usize,
    pub GetObjectIdForProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "oaidl")]
pub trait IAccessibleHostingElementProviders_Impl: windows_core::IUnknownImpl {
    fn GetEmbeddedFragmentRoots(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn GetObjectIdForProvider(&self, pprovider: windows_core::Ref<IRawElementProviderSimple>) -> windows_core::Result<i32>;
}
#[cfg(feature = "oaidl")]
impl IAccessibleHostingElementProviders_Vtbl {
    pub const fn new<Identity: IAccessibleHostingElementProviders_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEmbeddedFragmentRoots<Identity: IAccessibleHostingElementProviders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessibleHostingElementProviders_Impl::GetEmbeddedFragmentRoots(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetObjectIdForProvider<Identity: IAccessibleHostingElementProviders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprovider: *mut core::ffi::c_void, pidobject: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAccessibleHostingElementProviders_Impl::GetObjectIdForProvider(this, core::mem::transmute_copy(&pprovider)) {
                    Ok(ok__) => {
                        pidobject.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetEmbeddedFragmentRoots: GetEmbeddedFragmentRoots::<Identity, OFFSET>,
            GetObjectIdForProvider: GetObjectIdForProvider::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAccessibleHostingElementProviders as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IAccessibleHostingElementProviders {}
windows_core::imp::define_interface!(IAnnotationProvider, IAnnotationProvider_Vtbl, 0xf95c7e80_bd63_4601_9782_445ebff011fc);
windows_core::imp::interface_hierarchy!(IAnnotationProvider, windows_core::IUnknown);
impl IAnnotationProvider {
    pub unsafe fn AnnotationTypeId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AnnotationTypeId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn AnnotationTypeName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).AnnotationTypeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Author(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Author)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn DateTime(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DateTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Target(&self) -> windows_core::Result<IRawElementProviderSimple> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Target)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnnotationProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub AnnotationTypeId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub AnnotationTypeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Author: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Target: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAnnotationProvider_Impl: windows_core::IUnknownImpl {
    fn AnnotationTypeId(&self) -> windows_core::Result<i32>;
    fn AnnotationTypeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Author(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DateTime(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Target(&self) -> windows_core::Result<IRawElementProviderSimple>;
}
impl IAnnotationProvider_Vtbl {
    pub const fn new<Identity: IAnnotationProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AnnotationTypeId<Identity: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAnnotationProvider_Impl::AnnotationTypeId(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn AnnotationTypeName<Identity: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAnnotationProvider_Impl::AnnotationTypeName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Author<Identity: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAnnotationProvider_Impl::Author(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DateTime<Identity: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAnnotationProvider_Impl::DateTime(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Target<Identity: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAnnotationProvider_Impl::Target(this) {
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
            AnnotationTypeId: AnnotationTypeId::<Identity, OFFSET>,
            AnnotationTypeName: AnnotationTypeName::<Identity, OFFSET>,
            Author: Author::<Identity, OFFSET>,
            DateTime: DateTime::<Identity, OFFSET>,
            Target: Target::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IAnnotationProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IAnnotationProvider {}
windows_core::imp::define_interface!(ICustomNavigationProvider, ICustomNavigationProvider_Vtbl, 0x2062a28a_8c07_4b94_8e12_7037c622aeb8);
windows_core::imp::interface_hierarchy!(ICustomNavigationProvider, windows_core::IUnknown);
impl ICustomNavigationProvider {
    pub unsafe fn Navigate(&self, direction: NavigateDirection) -> windows_core::Result<IRawElementProviderSimple> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Navigate)(windows_core::Interface::as_raw(self), direction, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ICustomNavigationProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Navigate: unsafe extern "system" fn(*mut core::ffi::c_void, NavigateDirection, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ICustomNavigationProvider_Impl: windows_core::IUnknownImpl {
    fn Navigate(&self, direction: NavigateDirection) -> windows_core::Result<IRawElementProviderSimple>;
}
impl ICustomNavigationProvider_Vtbl {
    pub const fn new<Identity: ICustomNavigationProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Navigate<Identity: ICustomNavigationProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, direction: NavigateDirection, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ICustomNavigationProvider_Impl::Navigate(this, core::mem::transmute_copy(&direction)) {
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
        iid == &<ICustomNavigationProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ICustomNavigationProvider {}
windows_core::imp::define_interface!(IDockProvider, IDockProvider_Vtbl, 0x159bc72c_4ad3_485e_9637_d7052edf0146);
windows_core::imp::interface_hierarchy!(IDockProvider, windows_core::IUnknown);
impl IDockProvider {
    pub unsafe fn SetDockPosition(&self, dockposition: DockPosition) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetDockPosition)(windows_core::Interface::as_raw(self), dockposition) }
    }
    pub unsafe fn DockPosition(&self) -> windows_core::Result<DockPosition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DockPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDockProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDockPosition: unsafe extern "system" fn(*mut core::ffi::c_void, DockPosition) -> windows_core::HRESULT,
    pub DockPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DockPosition) -> windows_core::HRESULT,
}
pub trait IDockProvider_Impl: windows_core::IUnknownImpl {
    fn SetDockPosition(&self, dockposition: DockPosition) -> windows_core::Result<()>;
    fn DockPosition(&self) -> windows_core::Result<DockPosition>;
}
impl IDockProvider_Vtbl {
    pub const fn new<Identity: IDockProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDockPosition<Identity: IDockProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dockposition: DockPosition) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDockProvider_Impl::SetDockPosition(this, core::mem::transmute_copy(&dockposition)).into()
            }
        }
        unsafe extern "system" fn DockPosition<Identity: IDockProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut DockPosition) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDockProvider_Impl::DockPosition(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetDockPosition: SetDockPosition::<Identity, OFFSET>,
            DockPosition: DockPosition::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDockProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IDockProvider {}
windows_core::imp::define_interface!(IDragProvider, IDragProvider_Vtbl, 0x6aa7bbbb_7ff9_497d_904f_d20b897929d8);
windows_core::imp::interface_hierarchy!(IDragProvider, windows_core::IUnknown);
impl IDragProvider {
    pub unsafe fn IsGrabbed(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsGrabbed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DropEffect(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DropEffect)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn DropEffects(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DropEffects)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetGrabbedItems(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetGrabbedItems)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDragProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub IsGrabbed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub DropEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub DropEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    DropEffects: usize,
    #[cfg(feature = "oaidl")]
    pub GetGrabbedItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetGrabbedItems: usize,
}
#[cfg(feature = "oaidl")]
pub trait IDragProvider_Impl: windows_core::IUnknownImpl {
    fn IsGrabbed(&self) -> windows_core::Result<windows_core::BOOL>;
    fn DropEffect(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DropEffects(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn GetGrabbedItems(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
}
#[cfg(feature = "oaidl")]
impl IDragProvider_Vtbl {
    pub const fn new<Identity: IDragProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn IsGrabbed<Identity: IDragProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDragProvider_Impl::IsGrabbed(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DropEffect<Identity: IDragProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDragProvider_Impl::DropEffect(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DropEffects<Identity: IDragProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDragProvider_Impl::DropEffects(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGrabbedItems<Identity: IDragProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDragProvider_Impl::GetGrabbedItems(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            IsGrabbed: IsGrabbed::<Identity, OFFSET>,
            DropEffect: DropEffect::<Identity, OFFSET>,
            DropEffects: DropEffects::<Identity, OFFSET>,
            GetGrabbedItems: GetGrabbedItems::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDragProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IDragProvider {}
windows_core::imp::define_interface!(IDropTargetProvider, IDropTargetProvider_Vtbl, 0xbae82bfd_358a_481c_85a0_d8b4d90a5d61);
windows_core::imp::interface_hierarchy!(IDropTargetProvider, windows_core::IUnknown);
impl IDropTargetProvider {
    pub unsafe fn DropTargetEffect(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DropTargetEffect)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn DropTargetEffects(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DropTargetEffects)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTargetProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub DropTargetEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub DropTargetEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    DropTargetEffects: usize,
}
#[cfg(feature = "oaidl")]
pub trait IDropTargetProvider_Impl: windows_core::IUnknownImpl {
    fn DropTargetEffect(&self) -> windows_core::Result<windows_core::BSTR>;
    fn DropTargetEffects(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
}
#[cfg(feature = "oaidl")]
impl IDropTargetProvider_Vtbl {
    pub const fn new<Identity: IDropTargetProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn DropTargetEffect<Identity: IDropTargetProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDropTargetProvider_Impl::DropTargetEffect(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DropTargetEffects<Identity: IDropTargetProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDropTargetProvider_Impl::DropTargetEffects(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            DropTargetEffect: DropTargetEffect::<Identity, OFFSET>,
            DropTargetEffects: DropTargetEffects::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDropTargetProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IDropTargetProvider {}
windows_core::imp::define_interface!(IExpandCollapseProvider, IExpandCollapseProvider_Vtbl, 0xd847d3a5_cab0_4a98_8c32_ecb45c59ad24);
windows_core::imp::interface_hierarchy!(IExpandCollapseProvider, windows_core::IUnknown);
impl IExpandCollapseProvider {
    pub unsafe fn Expand(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Expand)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn Collapse(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Collapse)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ExpandCollapseState(&self) -> windows_core::Result<ExpandCollapseState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExpandCollapseState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpandCollapseProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Expand: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Collapse: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ExpandCollapseState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ExpandCollapseState) -> windows_core::HRESULT,
}
pub trait IExpandCollapseProvider_Impl: windows_core::IUnknownImpl {
    fn Expand(&self) -> windows_core::Result<()>;
    fn Collapse(&self) -> windows_core::Result<()>;
    fn ExpandCollapseState(&self) -> windows_core::Result<ExpandCollapseState>;
}
impl IExpandCollapseProvider_Vtbl {
    pub const fn new<Identity: IExpandCollapseProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Expand<Identity: IExpandCollapseProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExpandCollapseProvider_Impl::Expand(this).into()
            }
        }
        unsafe extern "system" fn Collapse<Identity: IExpandCollapseProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IExpandCollapseProvider_Impl::Collapse(this).into()
            }
        }
        unsafe extern "system" fn ExpandCollapseState<Identity: IExpandCollapseProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut ExpandCollapseState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IExpandCollapseProvider_Impl::ExpandCollapseState(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
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
            ExpandCollapseState: ExpandCollapseState::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IExpandCollapseProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IExpandCollapseProvider {}
windows_core::imp::define_interface!(IGridItemProvider, IGridItemProvider_Vtbl, 0xd02541f1_fb81_4d64_ae32_f520f8a6dbd1);
windows_core::imp::interface_hierarchy!(IGridItemProvider, windows_core::IUnknown);
impl IGridItemProvider {
    pub unsafe fn Row(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Row)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Column(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Column)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RowSpan(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RowSpan)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ColumnSpan(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ColumnSpan)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ContainingGrid(&self) -> windows_core::Result<IRawElementProviderSimple> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ContainingGrid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridItemProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Row: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Column: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub RowSpan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ColumnSpan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ContainingGrid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IGridItemProvider_Impl: windows_core::IUnknownImpl {
    fn Row(&self) -> windows_core::Result<i32>;
    fn Column(&self) -> windows_core::Result<i32>;
    fn RowSpan(&self) -> windows_core::Result<i32>;
    fn ColumnSpan(&self) -> windows_core::Result<i32>;
    fn ContainingGrid(&self) -> windows_core::Result<IRawElementProviderSimple>;
}
impl IGridItemProvider_Vtbl {
    pub const fn new<Identity: IGridItemProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Row<Identity: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGridItemProvider_Impl::Row(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Column<Identity: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGridItemProvider_Impl::Column(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RowSpan<Identity: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGridItemProvider_Impl::RowSpan(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ColumnSpan<Identity: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGridItemProvider_Impl::ColumnSpan(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ContainingGrid<Identity: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGridItemProvider_Impl::ContainingGrid(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Row: Row::<Identity, OFFSET>,
            Column: Column::<Identity, OFFSET>,
            RowSpan: RowSpan::<Identity, OFFSET>,
            ColumnSpan: ColumnSpan::<Identity, OFFSET>,
            ContainingGrid: ContainingGrid::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGridItemProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGridItemProvider {}
windows_core::imp::define_interface!(IGridProvider, IGridProvider_Vtbl, 0xb17d6187_0907_464b_a168_0ef17a1572b1);
windows_core::imp::interface_hierarchy!(IGridProvider, windows_core::IUnknown);
impl IGridProvider {
    pub unsafe fn GetItem(&self, row: i32, column: i32) -> windows_core::Result<IRawElementProviderSimple> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItem)(windows_core::Interface::as_raw(self), row, column, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RowCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RowCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ColumnCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ColumnCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RowCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ColumnCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
pub trait IGridProvider_Impl: windows_core::IUnknownImpl {
    fn GetItem(&self, row: i32, column: i32) -> windows_core::Result<IRawElementProviderSimple>;
    fn RowCount(&self) -> windows_core::Result<i32>;
    fn ColumnCount(&self) -> windows_core::Result<i32>;
}
impl IGridProvider_Vtbl {
    pub const fn new<Identity: IGridProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetItem<Identity: IGridProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, row: i32, column: i32, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGridProvider_Impl::GetItem(this, core::mem::transmute_copy(&row), core::mem::transmute_copy(&column)) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RowCount<Identity: IGridProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGridProvider_Impl::RowCount(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ColumnCount<Identity: IGridProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGridProvider_Impl::ColumnCount(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetItem: GetItem::<Identity, OFFSET>,
            RowCount: RowCount::<Identity, OFFSET>,
            ColumnCount: ColumnCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IGridProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IGridProvider {}
windows_core::imp::define_interface!(IInvokeProvider, IInvokeProvider_Vtbl, 0x54fcb24b_e18e_47a2_b4d3_eccbe77599a2);
windows_core::imp::interface_hierarchy!(IInvokeProvider, windows_core::IUnknown);
impl IInvokeProvider {
    pub unsafe fn Invoke(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Invoke)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IInvokeProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Invoke: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IInvokeProvider_Impl: windows_core::IUnknownImpl {
    fn Invoke(&self) -> windows_core::Result<()>;
}
impl IInvokeProvider_Vtbl {
    pub const fn new<Identity: IInvokeProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Invoke<Identity: IInvokeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IInvokeProvider_Impl::Invoke(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Invoke: Invoke::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IInvokeProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IInvokeProvider {}
windows_core::imp::define_interface!(IItemContainerProvider, IItemContainerProvider_Vtbl, 0xe747770b_39ce_4382_ab30_d8fb3f336f24);
windows_core::imp::interface_hierarchy!(IItemContainerProvider, windows_core::IUnknown);
impl IItemContainerProvider {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn FindItemByProperty<P0>(&self, pstartafter: P0, propertyid: PROPERTYID, value: &super::oaidl::VARIANT) -> windows_core::Result<IRawElementProviderSimple>
    where
        P0: windows_core::Param<IRawElementProviderSimple>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindItemByProperty)(windows_core::Interface::as_raw(self), pstartafter.param().abi(), propertyid, core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemContainerProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub FindItemByProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, PROPERTYID, super::oaidl::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    FindItemByProperty: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IItemContainerProvider_Impl: windows_core::IUnknownImpl {
    fn FindItemByProperty(&self, pstartafter: windows_core::Ref<IRawElementProviderSimple>, propertyid: PROPERTYID, value: &super::oaidl::VARIANT) -> windows_core::Result<IRawElementProviderSimple>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IItemContainerProvider_Vtbl {
    pub const fn new<Identity: IItemContainerProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindItemByProperty<Identity: IItemContainerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstartafter: *mut core::ffi::c_void, propertyid: PROPERTYID, value: super::oaidl::VARIANT, pfound: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IItemContainerProvider_Impl::FindItemByProperty(this, core::mem::transmute_copy(&pstartafter), core::mem::transmute_copy(&propertyid), core::mem::transmute(&value)) {
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
        iid == &<IItemContainerProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IItemContainerProvider {}
windows_core::imp::define_interface!(ILegacyIAccessibleProvider, ILegacyIAccessibleProvider_Vtbl, 0xe44c3566_915d_4070_99c6_047bff5a08f5);
windows_core::imp::interface_hierarchy!(ILegacyIAccessibleProvider, windows_core::IUnknown);
impl ILegacyIAccessibleProvider {
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
    #[cfg(all(feature = "oaidl", feature = "oleacc"))]
    pub unsafe fn GetIAccessible(&self) -> windows_core::Result<super::oleacc::IAccessible> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIAccessible)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ChildId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ChildId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Role(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Role)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn State(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Help(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Help)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn KeyboardShortcut(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).KeyboardShortcut)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetSelection(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn DefaultAction(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DefaultAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ILegacyIAccessibleProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Select: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub DoDefaultAction: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "oleacc"))]
    pub GetIAccessible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "oleacc")))]
    GetIAccessible: usize,
    pub ChildId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Role: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub Help: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub KeyboardShortcut: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetSelection: usize,
    pub DefaultAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "oleacc"))]
pub trait ILegacyIAccessibleProvider_Impl: windows_core::IUnknownImpl {
    fn Select(&self, flagsselect: i32) -> windows_core::Result<()>;
    fn DoDefaultAction(&self) -> windows_core::Result<()>;
    fn SetValue(&self, szvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetIAccessible(&self) -> windows_core::Result<super::oleacc::IAccessible>;
    fn ChildId(&self) -> windows_core::Result<i32>;
    fn Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Value(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Role(&self) -> windows_core::Result<u32>;
    fn State(&self) -> windows_core::Result<u32>;
    fn Help(&self) -> windows_core::Result<windows_core::BSTR>;
    fn KeyboardShortcut(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetSelection(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn DefaultAction(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "oleacc"))]
impl ILegacyIAccessibleProvider_Vtbl {
    pub const fn new<Identity: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Select<Identity: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, flagsselect: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILegacyIAccessibleProvider_Impl::Select(this, core::mem::transmute_copy(&flagsselect)).into()
            }
        }
        unsafe extern "system" fn DoDefaultAction<Identity: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILegacyIAccessibleProvider_Impl::DoDefaultAction(this).into()
            }
        }
        unsafe extern "system" fn SetValue<Identity: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, szvalue: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ILegacyIAccessibleProvider_Impl::SetValue(this, core::mem::transmute(&szvalue)).into()
            }
        }
        unsafe extern "system" fn GetIAccessible<Identity: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppaccessible: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILegacyIAccessibleProvider_Impl::GetIAccessible(this) {
                    Ok(ok__) => {
                        ppaccessible.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ChildId<Identity: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILegacyIAccessibleProvider_Impl::ChildId(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Name<Identity: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILegacyIAccessibleProvider_Impl::Name(this) {
                    Ok(ok__) => {
                        pszname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Value<Identity: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILegacyIAccessibleProvider_Impl::Value(this) {
                    Ok(ok__) => {
                        pszvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Description<Identity: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILegacyIAccessibleProvider_Impl::Description(this) {
                    Ok(ok__) => {
                        pszdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Role<Identity: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwrole: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILegacyIAccessibleProvider_Impl::Role(this) {
                    Ok(ok__) => {
                        pdwrole.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn State<Identity: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstate: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILegacyIAccessibleProvider_Impl::State(this) {
                    Ok(ok__) => {
                        pdwstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Help<Identity: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszhelp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILegacyIAccessibleProvider_Impl::Help(this) {
                    Ok(ok__) => {
                        pszhelp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn KeyboardShortcut<Identity: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszkeyboardshortcut: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILegacyIAccessibleProvider_Impl::KeyboardShortcut(this) {
                    Ok(ok__) => {
                        pszkeyboardshortcut.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSelection<Identity: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarselectedchildren: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILegacyIAccessibleProvider_Impl::GetSelection(this) {
                    Ok(ok__) => {
                        pvarselectedchildren.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DefaultAction<Identity: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdefaultaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILegacyIAccessibleProvider_Impl::DefaultAction(this) {
                    Ok(ok__) => {
                        pszdefaultaction.write(core::mem::transmute(ok__));
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
            GetIAccessible: GetIAccessible::<Identity, OFFSET>,
            ChildId: ChildId::<Identity, OFFSET>,
            Name: Name::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            Description: Description::<Identity, OFFSET>,
            Role: Role::<Identity, OFFSET>,
            State: State::<Identity, OFFSET>,
            Help: Help::<Identity, OFFSET>,
            KeyboardShortcut: KeyboardShortcut::<Identity, OFFSET>,
            GetSelection: GetSelection::<Identity, OFFSET>,
            DefaultAction: DefaultAction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILegacyIAccessibleProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "oleacc"))]
impl windows_core::RuntimeName for ILegacyIAccessibleProvider {}
windows_core::imp::define_interface!(IMultipleViewProvider, IMultipleViewProvider_Vtbl, 0x6278cab1_b556_4a1a_b4e0_418acc523201);
windows_core::imp::interface_hierarchy!(IMultipleViewProvider, windows_core::IUnknown);
impl IMultipleViewProvider {
    pub unsafe fn GetViewName(&self, viewid: i32) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetViewName)(windows_core::Interface::as_raw(self), viewid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn SetCurrentView(&self, viewid: i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetCurrentView)(windows_core::Interface::as_raw(self), viewid) }
    }
    pub unsafe fn CurrentView(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentView)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetSupportedViews(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSupportedViews)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IMultipleViewProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetViewName: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, i32) -> windows_core::HRESULT,
    pub CurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub GetSupportedViews: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetSupportedViews: usize,
}
#[cfg(feature = "oaidl")]
pub trait IMultipleViewProvider_Impl: windows_core::IUnknownImpl {
    fn GetViewName(&self, viewid: i32) -> windows_core::Result<windows_core::BSTR>;
    fn SetCurrentView(&self, viewid: i32) -> windows_core::Result<()>;
    fn CurrentView(&self) -> windows_core::Result<i32>;
    fn GetSupportedViews(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
}
#[cfg(feature = "oaidl")]
impl IMultipleViewProvider_Vtbl {
    pub const fn new<Identity: IMultipleViewProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetViewName<Identity: IMultipleViewProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewid: i32, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultipleViewProvider_Impl::GetViewName(this, core::mem::transmute_copy(&viewid)) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetCurrentView<Identity: IMultipleViewProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, viewid: i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IMultipleViewProvider_Impl::SetCurrentView(this, core::mem::transmute_copy(&viewid)).into()
            }
        }
        unsafe extern "system" fn CurrentView<Identity: IMultipleViewProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultipleViewProvider_Impl::CurrentView(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedViews<Identity: IMultipleViewProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultipleViewProvider_Impl::GetSupportedViews(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
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
            CurrentView: CurrentView::<Identity, OFFSET>,
            GetSupportedViews: GetSupportedViews::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultipleViewProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IMultipleViewProvider {}
windows_core::imp::define_interface!(IObjectModelProvider, IObjectModelProvider_Vtbl, 0x3ad86ebd_f5ef_483d_bb18_b1042a475d64);
windows_core::imp::interface_hierarchy!(IObjectModelProvider, windows_core::IUnknown);
impl IObjectModelProvider {
    pub unsafe fn GetUnderlyingObjectModel(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUnderlyingObjectModel)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IObjectModelProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetUnderlyingObjectModel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IObjectModelProvider_Impl: windows_core::IUnknownImpl {
    fn GetUnderlyingObjectModel(&self) -> windows_core::Result<windows_core::IUnknown>;
}
impl IObjectModelProvider_Vtbl {
    pub const fn new<Identity: IObjectModelProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetUnderlyingObjectModel<Identity: IObjectModelProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppunknown: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IObjectModelProvider_Impl::GetUnderlyingObjectModel(this) {
                    Ok(ok__) => {
                        ppunknown.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetUnderlyingObjectModel: GetUnderlyingObjectModel::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IObjectModelProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IObjectModelProvider {}
windows_core::imp::define_interface!(IProxyProviderWinEventHandler, IProxyProviderWinEventHandler_Vtbl, 0x89592ad4_f4e0_43d5_a3b6_bad7e111b435);
windows_core::imp::interface_hierarchy!(IProxyProviderWinEventHandler, windows_core::IUnknown);
impl IProxyProviderWinEventHandler {
    #[cfg(feature = "windef")]
    pub unsafe fn RespondToWinEvent<P4>(&self, idwinevent: u32, hwnd: super::windef::HWND, idobject: i32, idchild: i32, psink: P4) -> windows_core::HRESULT
    where
        P4: windows_core::Param<IProxyProviderWinEventSink>,
    {
        unsafe { (windows_core::Interface::vtable(self).RespondToWinEvent)(windows_core::Interface::as_raw(self), idwinevent, hwnd, idobject, idchild, psink.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProxyProviderWinEventHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub RespondToWinEvent: unsafe extern "system" fn(*mut core::ffi::c_void, u32, super::windef::HWND, i32, i32, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    RespondToWinEvent: usize,
}
#[cfg(feature = "windef")]
pub trait IProxyProviderWinEventHandler_Impl: windows_core::IUnknownImpl {
    fn RespondToWinEvent(&self, idwinevent: u32, hwnd: super::windef::HWND, idobject: i32, idchild: i32, psink: windows_core::Ref<IProxyProviderWinEventSink>) -> windows_core::Result<()>;
}
#[cfg(feature = "windef")]
impl IProxyProviderWinEventHandler_Vtbl {
    pub const fn new<Identity: IProxyProviderWinEventHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RespondToWinEvent<Identity: IProxyProviderWinEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idwinevent: u32, hwnd: super::windef::HWND, idobject: i32, idchild: i32, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProxyProviderWinEventHandler_Impl::RespondToWinEvent(this, core::mem::transmute_copy(&idwinevent), core::mem::transmute_copy(&hwnd), core::mem::transmute_copy(&idobject), core::mem::transmute_copy(&idchild), core::mem::transmute_copy(&psink)).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), RespondToWinEvent: RespondToWinEvent::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProxyProviderWinEventHandler as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IProxyProviderWinEventHandler {}
windows_core::imp::define_interface!(IProxyProviderWinEventSink, IProxyProviderWinEventSink_Vtbl, 0x4fd82b78_a43e_46ac_9803_0a6969c7c183);
windows_core::imp::interface_hierarchy!(IProxyProviderWinEventSink, windows_core::IUnknown);
impl IProxyProviderWinEventSink {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AddAutomationPropertyChangedEvent<P0>(&self, pprovider: P0, id: PROPERTYID, newvalue: &super::oaidl::VARIANT) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IRawElementProviderSimple>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddAutomationPropertyChangedEvent)(windows_core::Interface::as_raw(self), pprovider.param().abi(), id, core::mem::transmute_copy(newvalue)) }
    }
    pub unsafe fn AddAutomationEvent<P0>(&self, pprovider: P0, id: EVENTID) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IRawElementProviderSimple>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddAutomationEvent)(windows_core::Interface::as_raw(self), pprovider.param().abi(), id) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn AddStructureChangedEvent<P0>(&self, pprovider: P0, structurechangetype: StructureChangeType, runtimeid: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IRawElementProviderSimple>,
    {
        unsafe { (windows_core::Interface::vtable(self).AddStructureChangedEvent)(windows_core::Interface::as_raw(self), pprovider.param().abi(), structurechangetype, runtimeid) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IProxyProviderWinEventSink_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub AddAutomationPropertyChangedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, PROPERTYID, super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    AddAutomationPropertyChangedEvent: usize,
    pub AddAutomationEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, EVENTID) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub AddStructureChangedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, StructureChangeType, *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    AddStructureChangedEvent: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IProxyProviderWinEventSink_Impl: windows_core::IUnknownImpl {
    fn AddAutomationPropertyChangedEvent(&self, pprovider: windows_core::Ref<IRawElementProviderSimple>, id: PROPERTYID, newvalue: &super::oaidl::VARIANT) -> windows_core::Result<()>;
    fn AddAutomationEvent(&self, pprovider: windows_core::Ref<IRawElementProviderSimple>, id: EVENTID) -> windows_core::Result<()>;
    fn AddStructureChangedEvent(&self, pprovider: windows_core::Ref<IRawElementProviderSimple>, structurechangetype: StructureChangeType, runtimeid: *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IProxyProviderWinEventSink_Vtbl {
    pub const fn new<Identity: IProxyProviderWinEventSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddAutomationPropertyChangedEvent<Identity: IProxyProviderWinEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprovider: *mut core::ffi::c_void, id: PROPERTYID, newvalue: super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProxyProviderWinEventSink_Impl::AddAutomationPropertyChangedEvent(this, core::mem::transmute_copy(&pprovider), core::mem::transmute_copy(&id), core::mem::transmute(&newvalue)).into()
            }
        }
        unsafe extern "system" fn AddAutomationEvent<Identity: IProxyProviderWinEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprovider: *mut core::ffi::c_void, id: EVENTID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProxyProviderWinEventSink_Impl::AddAutomationEvent(this, core::mem::transmute_copy(&pprovider), core::mem::transmute_copy(&id)).into()
            }
        }
        unsafe extern "system" fn AddStructureChangedEvent<Identity: IProxyProviderWinEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprovider: *mut core::ffi::c_void, structurechangetype: StructureChangeType, runtimeid: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IProxyProviderWinEventSink_Impl::AddStructureChangedEvent(this, core::mem::transmute_copy(&pprovider), core::mem::transmute_copy(&structurechangetype), core::mem::transmute_copy(&runtimeid)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AddAutomationPropertyChangedEvent: AddAutomationPropertyChangedEvent::<Identity, OFFSET>,
            AddAutomationEvent: AddAutomationEvent::<Identity, OFFSET>,
            AddStructureChangedEvent: AddStructureChangedEvent::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IProxyProviderWinEventSink as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IProxyProviderWinEventSink {}
windows_core::imp::define_interface!(IRangeValueProvider, IRangeValueProvider_Vtbl, 0x36dc7aef_33e6_4691_afe1_2be7274b3d33);
windows_core::imp::interface_hierarchy!(IRangeValueProvider, windows_core::IUnknown);
impl IRangeValueProvider {
    pub unsafe fn SetValue(&self, val: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), val) }
    }
    pub unsafe fn Value(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsReadOnly(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Maximum(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Maximum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn Minimum(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Minimum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn LargeChange(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LargeChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SmallChange(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SmallChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeValueProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub Maximum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub Minimum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub LargeChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub SmallChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
pub trait IRangeValueProvider_Impl: windows_core::IUnknownImpl {
    fn SetValue(&self, val: f64) -> windows_core::Result<()>;
    fn Value(&self) -> windows_core::Result<f64>;
    fn IsReadOnly(&self) -> windows_core::Result<windows_core::BOOL>;
    fn Maximum(&self) -> windows_core::Result<f64>;
    fn Minimum(&self) -> windows_core::Result<f64>;
    fn LargeChange(&self) -> windows_core::Result<f64>;
    fn SmallChange(&self) -> windows_core::Result<f64>;
}
impl IRangeValueProvider_Vtbl {
    pub const fn new<Identity: IRangeValueProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetValue<Identity: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRangeValueProvider_Impl::SetValue(this, core::mem::transmute_copy(&val)).into()
            }
        }
        unsafe extern "system" fn Value<Identity: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRangeValueProvider_Impl::Value(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsReadOnly<Identity: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRangeValueProvider_Impl::IsReadOnly(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Maximum<Identity: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRangeValueProvider_Impl::Maximum(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Minimum<Identity: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRangeValueProvider_Impl::Minimum(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LargeChange<Identity: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRangeValueProvider_Impl::LargeChange(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SmallChange<Identity: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRangeValueProvider_Impl::SmallChange(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetValue: SetValue::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            IsReadOnly: IsReadOnly::<Identity, OFFSET>,
            Maximum: Maximum::<Identity, OFFSET>,
            Minimum: Minimum::<Identity, OFFSET>,
            LargeChange: LargeChange::<Identity, OFFSET>,
            SmallChange: SmallChange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRangeValueProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRangeValueProvider {}
windows_core::imp::define_interface!(IRawElementProviderAdviseEvents, IRawElementProviderAdviseEvents_Vtbl, 0xa407b27b_0f6d_4427_9292_473c7bf93258);
windows_core::imp::interface_hierarchy!(IRawElementProviderAdviseEvents, windows_core::IUnknown);
impl IRawElementProviderAdviseEvents {
    #[cfg(feature = "oaidl")]
    pub unsafe fn AdviseEventAdded(&self, eventid: EVENTID, propertyids: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AdviseEventAdded)(windows_core::Interface::as_raw(self), eventid, propertyids) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn AdviseEventRemoved(&self, eventid: EVENTID, propertyids: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AdviseEventRemoved)(windows_core::Interface::as_raw(self), eventid, propertyids) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawElementProviderAdviseEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "oaidl")]
    pub AdviseEventAdded: unsafe extern "system" fn(*mut core::ffi::c_void, EVENTID, *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    AdviseEventAdded: usize,
    #[cfg(feature = "oaidl")]
    pub AdviseEventRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, EVENTID, *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    AdviseEventRemoved: usize,
}
#[cfg(feature = "oaidl")]
pub trait IRawElementProviderAdviseEvents_Impl: windows_core::IUnknownImpl {
    fn AdviseEventAdded(&self, eventid: EVENTID, propertyids: *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
    fn AdviseEventRemoved(&self, eventid: EVENTID, propertyids: *const super::oaidl::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(feature = "oaidl")]
impl IRawElementProviderAdviseEvents_Vtbl {
    pub const fn new<Identity: IRawElementProviderAdviseEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdviseEventAdded<Identity: IRawElementProviderAdviseEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: EVENTID, propertyids: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawElementProviderAdviseEvents_Impl::AdviseEventAdded(this, core::mem::transmute_copy(&eventid), core::mem::transmute_copy(&propertyids)).into()
            }
        }
        unsafe extern "system" fn AdviseEventRemoved<Identity: IRawElementProviderAdviseEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: EVENTID, propertyids: *const super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawElementProviderAdviseEvents_Impl::AdviseEventRemoved(this, core::mem::transmute_copy(&eventid), core::mem::transmute_copy(&propertyids)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            AdviseEventAdded: AdviseEventAdded::<Identity, OFFSET>,
            AdviseEventRemoved: AdviseEventRemoved::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRawElementProviderAdviseEvents as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IRawElementProviderAdviseEvents {}
windows_core::imp::define_interface!(IRawElementProviderFragment, IRawElementProviderFragment_Vtbl, 0xf7063da8_8359_439c_9297_bbc5299a7d87);
windows_core::imp::interface_hierarchy!(IRawElementProviderFragment, windows_core::IUnknown);
impl IRawElementProviderFragment {
    pub unsafe fn Navigate(&self, direction: NavigateDirection) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Navigate)(windows_core::Interface::as_raw(self), direction, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetRuntimeId(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRuntimeId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_BoundingRectangle(&self) -> windows_core::Result<UiaRect> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_BoundingRectangle)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetEmbeddedFragmentRoots(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEmbeddedFragmentRoots)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFocus(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFocus)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn FragmentRoot(&self) -> windows_core::Result<IRawElementProviderFragmentRoot> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FragmentRoot)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawElementProviderFragment_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Navigate: unsafe extern "system" fn(*mut core::ffi::c_void, NavigateDirection, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub GetRuntimeId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetRuntimeId: usize,
    pub get_BoundingRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UiaRect) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub GetEmbeddedFragmentRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetEmbeddedFragmentRoots: usize,
    pub SetFocus: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FragmentRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "oaidl")]
pub trait IRawElementProviderFragment_Impl: windows_core::IUnknownImpl {
    fn Navigate(&self, direction: NavigateDirection) -> windows_core::Result<IRawElementProviderFragment>;
    fn GetRuntimeId(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn get_BoundingRectangle(&self) -> windows_core::Result<UiaRect>;
    fn GetEmbeddedFragmentRoots(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn SetFocus(&self) -> windows_core::Result<()>;
    fn FragmentRoot(&self) -> windows_core::Result<IRawElementProviderFragmentRoot>;
}
#[cfg(feature = "oaidl")]
impl IRawElementProviderFragment_Vtbl {
    pub const fn new<Identity: IRawElementProviderFragment_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Navigate<Identity: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, direction: NavigateDirection, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawElementProviderFragment_Impl::Navigate(this, core::mem::transmute_copy(&direction)) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRuntimeId<Identity: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawElementProviderFragment_Impl::GetRuntimeId(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_BoundingRectangle<Identity: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut UiaRect) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawElementProviderFragment_Impl::get_BoundingRectangle(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEmbeddedFragmentRoots<Identity: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawElementProviderFragment_Impl::GetEmbeddedFragmentRoots(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SetFocus<Identity: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawElementProviderFragment_Impl::SetFocus(this).into()
            }
        }
        unsafe extern "system" fn FragmentRoot<Identity: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawElementProviderFragment_Impl::FragmentRoot(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Navigate: Navigate::<Identity, OFFSET>,
            GetRuntimeId: GetRuntimeId::<Identity, OFFSET>,
            get_BoundingRectangle: get_BoundingRectangle::<Identity, OFFSET>,
            GetEmbeddedFragmentRoots: GetEmbeddedFragmentRoots::<Identity, OFFSET>,
            SetFocus: SetFocus::<Identity, OFFSET>,
            FragmentRoot: FragmentRoot::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRawElementProviderFragment as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IRawElementProviderFragment {}
windows_core::imp::define_interface!(IRawElementProviderFragmentRoot, IRawElementProviderFragmentRoot_Vtbl, 0x620ce2a5_ab8f_40a9_86cb_de3c75599b58);
windows_core::imp::interface_hierarchy!(IRawElementProviderFragmentRoot, windows_core::IUnknown);
impl IRawElementProviderFragmentRoot {
    pub unsafe fn ElementProviderFromPoint(&self, x: f64, y: f64) -> windows_core::Result<IRawElementProviderFragment> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElementProviderFromPoint)(windows_core::Interface::as_raw(self), x, y, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFocus(&self) -> windows_core::Result<IRawElementProviderFragment> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFocus)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawElementProviderFragmentRoot_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ElementProviderFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetFocus: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IRawElementProviderFragmentRoot_Impl: windows_core::IUnknownImpl {
    fn ElementProviderFromPoint(&self, x: f64, y: f64) -> windows_core::Result<IRawElementProviderFragment>;
    fn GetFocus(&self) -> windows_core::Result<IRawElementProviderFragment>;
}
impl IRawElementProviderFragmentRoot_Vtbl {
    pub const fn new<Identity: IRawElementProviderFragmentRoot_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ElementProviderFromPoint<Identity: IRawElementProviderFragmentRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f64, y: f64, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawElementProviderFragmentRoot_Impl::ElementProviderFromPoint(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetFocus<Identity: IRawElementProviderFragmentRoot_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawElementProviderFragmentRoot_Impl::GetFocus(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ElementProviderFromPoint: ElementProviderFromPoint::<Identity, OFFSET>,
            GetFocus: GetFocus::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRawElementProviderFragmentRoot as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IRawElementProviderFragmentRoot {}
windows_core::imp::define_interface!(IRawElementProviderHostingAccessibles, IRawElementProviderHostingAccessibles_Vtbl, 0x24be0b07_d37d_487a_98cf_a13ed465e9b3);
windows_core::imp::interface_hierarchy!(IRawElementProviderHostingAccessibles, windows_core::IUnknown);
impl IRawElementProviderHostingAccessibles {
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetEmbeddedAccessibles(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEmbeddedAccessibles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawElementProviderHostingAccessibles_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "oaidl")]
    pub GetEmbeddedAccessibles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetEmbeddedAccessibles: usize,
}
#[cfg(feature = "oaidl")]
pub trait IRawElementProviderHostingAccessibles_Impl: windows_core::IUnknownImpl {
    fn GetEmbeddedAccessibles(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
}
#[cfg(feature = "oaidl")]
impl IRawElementProviderHostingAccessibles_Vtbl {
    pub const fn new<Identity: IRawElementProviderHostingAccessibles_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEmbeddedAccessibles<Identity: IRawElementProviderHostingAccessibles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawElementProviderHostingAccessibles_Impl::GetEmbeddedAccessibles(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetEmbeddedAccessibles: GetEmbeddedAccessibles::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRawElementProviderHostingAccessibles as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IRawElementProviderHostingAccessibles {}
windows_core::imp::define_interface!(IRawElementProviderHwndOverride, IRawElementProviderHwndOverride_Vtbl, 0x1d5df27c_8947_4425_b8d9_79787bb460b8);
windows_core::imp::interface_hierarchy!(IRawElementProviderHwndOverride, windows_core::IUnknown);
impl IRawElementProviderHwndOverride {
    #[cfg(feature = "windef")]
    pub unsafe fn GetOverrideProviderForHwnd(&self, hwnd: super::windef::HWND) -> windows_core::Result<IRawElementProviderSimple> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOverrideProviderForHwnd)(windows_core::Interface::as_raw(self), hwnd, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawElementProviderHwndOverride_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub GetOverrideProviderForHwnd: unsafe extern "system" fn(*mut core::ffi::c_void, super::windef::HWND, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetOverrideProviderForHwnd: usize,
}
#[cfg(feature = "windef")]
pub trait IRawElementProviderHwndOverride_Impl: windows_core::IUnknownImpl {
    fn GetOverrideProviderForHwnd(&self, hwnd: super::windef::HWND) -> windows_core::Result<IRawElementProviderSimple>;
}
#[cfg(feature = "windef")]
impl IRawElementProviderHwndOverride_Vtbl {
    pub const fn new<Identity: IRawElementProviderHwndOverride_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOverrideProviderForHwnd<Identity: IRawElementProviderHwndOverride_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::windef::HWND, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawElementProviderHwndOverride_Impl::GetOverrideProviderForHwnd(this, core::mem::transmute_copy(&hwnd)) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetOverrideProviderForHwnd: GetOverrideProviderForHwnd::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRawElementProviderHwndOverride as windows_core::Interface>::IID
    }
}
#[cfg(feature = "windef")]
impl windows_core::RuntimeName for IRawElementProviderHwndOverride {}
windows_core::imp::define_interface!(IRawElementProviderSimple, IRawElementProviderSimple_Vtbl, 0xd6dd68d1_86fd_4332_8666_9abedea2d24c);
windows_core::imp::interface_hierarchy!(IRawElementProviderSimple, windows_core::IUnknown);
impl IRawElementProviderSimple {
    pub unsafe fn ProviderOptions(&self) -> windows_core::Result<ProviderOptions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProviderOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPatternProvider(&self, patternid: PATTERNID) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPatternProvider)(windows_core::Interface::as_raw(self), patternid, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetPropertyValue(&self, propertyid: PROPERTYID) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyValue)(windows_core::Interface::as_raw(self), propertyid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn HostRawElementProvider(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HostRawElementProvider)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawElementProviderSimple_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ProviderOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ProviderOptions) -> windows_core::HRESULT,
    pub GetPatternProvider: unsafe extern "system" fn(*mut core::ffi::c_void, PATTERNID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetPropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, PROPERTYID, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    GetPropertyValue: usize,
    pub HostRawElementProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IRawElementProviderSimple_Impl: windows_core::IUnknownImpl {
    fn ProviderOptions(&self) -> windows_core::Result<ProviderOptions>;
    fn GetPatternProvider(&self, patternid: PATTERNID) -> windows_core::Result<windows_core::IUnknown>;
    fn GetPropertyValue(&self, propertyid: PROPERTYID) -> windows_core::Result<super::oaidl::VARIANT>;
    fn HostRawElementProvider(&self) -> windows_core::Result<IRawElementProviderSimple>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IRawElementProviderSimple_Vtbl {
    pub const fn new<Identity: IRawElementProviderSimple_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProviderOptions<Identity: IRawElementProviderSimple_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut ProviderOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawElementProviderSimple_Impl::ProviderOptions(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPatternProvider<Identity: IRawElementProviderSimple_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, patternid: PATTERNID, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawElementProviderSimple_Impl::GetPatternProvider(this, core::mem::transmute_copy(&patternid)) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetPropertyValue<Identity: IRawElementProviderSimple_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: PROPERTYID, pretval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawElementProviderSimple_Impl::GetPropertyValue(this, core::mem::transmute_copy(&propertyid)) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HostRawElementProvider<Identity: IRawElementProviderSimple_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawElementProviderSimple_Impl::HostRawElementProvider(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProviderOptions: ProviderOptions::<Identity, OFFSET>,
            GetPatternProvider: GetPatternProvider::<Identity, OFFSET>,
            GetPropertyValue: GetPropertyValue::<Identity, OFFSET>,
            HostRawElementProvider: HostRawElementProvider::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRawElementProviderSimple as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IRawElementProviderSimple {}
windows_core::imp::define_interface!(IRawElementProviderSimple2, IRawElementProviderSimple2_Vtbl, 0xa0a839a9_8da1_4a82_806a_8e0d44e79f56);
impl core::ops::Deref for IRawElementProviderSimple2 {
    type Target = IRawElementProviderSimple;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRawElementProviderSimple2, windows_core::IUnknown, IRawElementProviderSimple);
impl IRawElementProviderSimple2 {
    pub unsafe fn ShowContextMenu(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShowContextMenu)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawElementProviderSimple2_Vtbl {
    pub base__: IRawElementProviderSimple_Vtbl,
    pub ShowContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IRawElementProviderSimple2_Impl: IRawElementProviderSimple_Impl {
    fn ShowContextMenu(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IRawElementProviderSimple2_Vtbl {
    pub const fn new<Identity: IRawElementProviderSimple2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ShowContextMenu<Identity: IRawElementProviderSimple2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawElementProviderSimple2_Impl::ShowContextMenu(this).into()
            }
        }
        Self { base__: IRawElementProviderSimple_Vtbl::new::<Identity, OFFSET>(), ShowContextMenu: ShowContextMenu::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRawElementProviderSimple2 as windows_core::Interface>::IID || iid == &<IRawElementProviderSimple as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IRawElementProviderSimple2 {}
windows_core::imp::define_interface!(IRawElementProviderSimple3, IRawElementProviderSimple3_Vtbl, 0xfcf5d820_d7ec_4613_bdf6_42a84ce7daaf);
impl core::ops::Deref for IRawElementProviderSimple3 {
    type Target = IRawElementProviderSimple2;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(IRawElementProviderSimple3, windows_core::IUnknown, IRawElementProviderSimple, IRawElementProviderSimple2);
impl IRawElementProviderSimple3 {
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetMetadataValue(&self, targetid: i32, metadataid: METADATAID) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetMetadataValue)(windows_core::Interface::as_raw(self), targetid, metadataid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawElementProviderSimple3_Vtbl {
    pub base__: IRawElementProviderSimple2_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetMetadataValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, METADATAID, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    GetMetadataValue: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait IRawElementProviderSimple3_Impl: IRawElementProviderSimple2_Impl {
    fn GetMetadataValue(&self, targetid: i32, metadataid: METADATAID) -> windows_core::Result<super::oaidl::VARIANT>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl IRawElementProviderSimple3_Vtbl {
    pub const fn new<Identity: IRawElementProviderSimple3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMetadataValue<Identity: IRawElementProviderSimple3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetid: i32, metadataid: METADATAID, returnval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawElementProviderSimple3_Impl::GetMetadataValue(this, core::mem::transmute_copy(&targetid), core::mem::transmute_copy(&metadataid)) {
                    Ok(ok__) => {
                        returnval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: IRawElementProviderSimple2_Vtbl::new::<Identity, OFFSET>(), GetMetadataValue: GetMetadataValue::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRawElementProviderSimple3 as windows_core::Interface>::IID || iid == &<IRawElementProviderSimple as windows_core::Interface>::IID || iid == &<IRawElementProviderSimple2 as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IRawElementProviderSimple3 {}
windows_core::imp::define_interface!(IRawElementProviderWindowlessSite, IRawElementProviderWindowlessSite_Vtbl, 0x0a2a93cc_bfad_42ac_9b2e_0991fb0d3ea0);
windows_core::imp::interface_hierarchy!(IRawElementProviderWindowlessSite, windows_core::IUnknown);
impl IRawElementProviderWindowlessSite {
    pub unsafe fn GetAdjacentFragment(&self, direction: NavigateDirection) -> windows_core::Result<IRawElementProviderFragment> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAdjacentFragment)(windows_core::Interface::as_raw(self), direction, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetRuntimeIdPrefix(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRuntimeIdPrefix)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawElementProviderWindowlessSite_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetAdjacentFragment: unsafe extern "system" fn(*mut core::ffi::c_void, NavigateDirection, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub GetRuntimeIdPrefix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetRuntimeIdPrefix: usize,
}
#[cfg(feature = "oaidl")]
pub trait IRawElementProviderWindowlessSite_Impl: windows_core::IUnknownImpl {
    fn GetAdjacentFragment(&self, direction: NavigateDirection) -> windows_core::Result<IRawElementProviderFragment>;
    fn GetRuntimeIdPrefix(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
}
#[cfg(feature = "oaidl")]
impl IRawElementProviderWindowlessSite_Vtbl {
    pub const fn new<Identity: IRawElementProviderWindowlessSite_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetAdjacentFragment<Identity: IRawElementProviderWindowlessSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, direction: NavigateDirection, ppparent: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawElementProviderWindowlessSite_Impl::GetAdjacentFragment(this, core::mem::transmute_copy(&direction)) {
                    Ok(ok__) => {
                        ppparent.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetRuntimeIdPrefix<Identity: IRawElementProviderWindowlessSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawElementProviderWindowlessSite_Impl::GetRuntimeIdPrefix(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetAdjacentFragment: GetAdjacentFragment::<Identity, OFFSET>,
            GetRuntimeIdPrefix: GetRuntimeIdPrefix::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRawElementProviderWindowlessSite as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IRawElementProviderWindowlessSite {}
windows_core::imp::define_interface!(IScrollItemProvider, IScrollItemProvider_Vtbl, 0x2360c714_4bf1_4b26_ba65_9b21316127eb);
windows_core::imp::interface_hierarchy!(IScrollItemProvider, windows_core::IUnknown);
impl IScrollItemProvider {
    pub unsafe fn ScrollIntoView(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ScrollIntoView)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollItemProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ScrollIntoView: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IScrollItemProvider_Impl: windows_core::IUnknownImpl {
    fn ScrollIntoView(&self) -> windows_core::Result<()>;
}
impl IScrollItemProvider_Vtbl {
    pub const fn new<Identity: IScrollItemProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ScrollIntoView<Identity: IScrollItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScrollItemProvider_Impl::ScrollIntoView(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ScrollIntoView: ScrollIntoView::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IScrollItemProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IScrollItemProvider {}
windows_core::imp::define_interface!(IScrollProvider, IScrollProvider_Vtbl, 0xb38b8077_1fc3_42a5_8cae_d40c2215055a);
windows_core::imp::interface_hierarchy!(IScrollProvider, windows_core::IUnknown);
impl IScrollProvider {
    pub unsafe fn Scroll(&self, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Scroll)(windows_core::Interface::as_raw(self), horizontalamount, verticalamount) }
    }
    pub unsafe fn SetScrollPercent(&self, horizontalpercent: f64, verticalpercent: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetScrollPercent)(windows_core::Interface::as_raw(self), horizontalpercent, verticalpercent) }
    }
    pub unsafe fn HorizontalScrollPercent(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HorizontalScrollPercent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn VerticalScrollPercent(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VerticalScrollPercent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn HorizontalViewSize(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HorizontalViewSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn VerticalViewSize(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VerticalViewSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn HorizontallyScrollable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).HorizontallyScrollable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn VerticallyScrollable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).VerticallyScrollable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Scroll: unsafe extern "system" fn(*mut core::ffi::c_void, ScrollAmount, ScrollAmount) -> windows_core::HRESULT,
    pub SetScrollPercent: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub HorizontalScrollPercent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub VerticalScrollPercent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub HorizontalViewSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub VerticalViewSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub HorizontallyScrollable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub VerticallyScrollable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IScrollProvider_Impl: windows_core::IUnknownImpl {
    fn Scroll(&self, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> windows_core::Result<()>;
    fn SetScrollPercent(&self, horizontalpercent: f64, verticalpercent: f64) -> windows_core::Result<()>;
    fn HorizontalScrollPercent(&self) -> windows_core::Result<f64>;
    fn VerticalScrollPercent(&self) -> windows_core::Result<f64>;
    fn HorizontalViewSize(&self) -> windows_core::Result<f64>;
    fn VerticalViewSize(&self) -> windows_core::Result<f64>;
    fn HorizontallyScrollable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn VerticallyScrollable(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IScrollProvider_Vtbl {
    pub const fn new<Identity: IScrollProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Scroll<Identity: IScrollProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScrollProvider_Impl::Scroll(this, core::mem::transmute_copy(&horizontalamount), core::mem::transmute_copy(&verticalamount)).into()
            }
        }
        unsafe extern "system" fn SetScrollPercent<Identity: IScrollProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, horizontalpercent: f64, verticalpercent: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IScrollProvider_Impl::SetScrollPercent(this, core::mem::transmute_copy(&horizontalpercent), core::mem::transmute_copy(&verticalpercent)).into()
            }
        }
        unsafe extern "system" fn HorizontalScrollPercent<Identity: IScrollProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScrollProvider_Impl::HorizontalScrollPercent(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VerticalScrollPercent<Identity: IScrollProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScrollProvider_Impl::VerticalScrollPercent(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HorizontalViewSize<Identity: IScrollProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScrollProvider_Impl::HorizontalViewSize(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VerticalViewSize<Identity: IScrollProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScrollProvider_Impl::VerticalViewSize(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn HorizontallyScrollable<Identity: IScrollProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScrollProvider_Impl::HorizontallyScrollable(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn VerticallyScrollable<Identity: IScrollProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScrollProvider_Impl::VerticallyScrollable(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
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
            HorizontalScrollPercent: HorizontalScrollPercent::<Identity, OFFSET>,
            VerticalScrollPercent: VerticalScrollPercent::<Identity, OFFSET>,
            HorizontalViewSize: HorizontalViewSize::<Identity, OFFSET>,
            VerticalViewSize: VerticalViewSize::<Identity, OFFSET>,
            HorizontallyScrollable: HorizontallyScrollable::<Identity, OFFSET>,
            VerticallyScrollable: VerticallyScrollable::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IScrollProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IScrollProvider {}
windows_core::imp::define_interface!(ISelectionItemProvider, ISelectionItemProvider_Vtbl, 0x2acad808_b2d4_452d_a407_91ff1ad167b2);
windows_core::imp::interface_hierarchy!(ISelectionItemProvider, windows_core::IUnknown);
impl ISelectionItemProvider {
    pub unsafe fn Select(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Select)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn AddToSelection(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AddToSelection)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn RemoveFromSelection(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RemoveFromSelection)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn IsSelected(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSelected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SelectionContainer(&self) -> windows_core::Result<IRawElementProviderSimple> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SelectionContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionItemProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Select: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddToSelection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveFromSelection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsSelected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub SelectionContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISelectionItemProvider_Impl: windows_core::IUnknownImpl {
    fn Select(&self) -> windows_core::Result<()>;
    fn AddToSelection(&self) -> windows_core::Result<()>;
    fn RemoveFromSelection(&self) -> windows_core::Result<()>;
    fn IsSelected(&self) -> windows_core::Result<windows_core::BOOL>;
    fn SelectionContainer(&self) -> windows_core::Result<IRawElementProviderSimple>;
}
impl ISelectionItemProvider_Vtbl {
    pub const fn new<Identity: ISelectionItemProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Select<Identity: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISelectionItemProvider_Impl::Select(this).into()
            }
        }
        unsafe extern "system" fn AddToSelection<Identity: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISelectionItemProvider_Impl::AddToSelection(this).into()
            }
        }
        unsafe extern "system" fn RemoveFromSelection<Identity: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISelectionItemProvider_Impl::RemoveFromSelection(this).into()
            }
        }
        unsafe extern "system" fn IsSelected<Identity: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISelectionItemProvider_Impl::IsSelected(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SelectionContainer<Identity: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISelectionItemProvider_Impl::SelectionContainer(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
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
            IsSelected: IsSelected::<Identity, OFFSET>,
            SelectionContainer: SelectionContainer::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISelectionItemProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISelectionItemProvider {}
windows_core::imp::define_interface!(ISelectionProvider, ISelectionProvider_Vtbl, 0xfb8b03af_3bdf_48d4_bd36_1a65793be168);
windows_core::imp::interface_hierarchy!(ISelectionProvider, windows_core::IUnknown);
impl ISelectionProvider {
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetSelection(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CanSelectMultiple(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanSelectMultiple)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsSelectionRequired(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsSelectionRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "oaidl")]
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetSelection: usize,
    pub CanSelectMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsSelectionRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(feature = "oaidl")]
pub trait ISelectionProvider_Impl: windows_core::IUnknownImpl {
    fn GetSelection(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn CanSelectMultiple(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsSelectionRequired(&self) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(feature = "oaidl")]
impl ISelectionProvider_Vtbl {
    pub const fn new<Identity: ISelectionProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSelection<Identity: ISelectionProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISelectionProvider_Impl::GetSelection(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanSelectMultiple<Identity: ISelectionProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISelectionProvider_Impl::CanSelectMultiple(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsSelectionRequired<Identity: ISelectionProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISelectionProvider_Impl::IsSelectionRequired(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSelection: GetSelection::<Identity, OFFSET>,
            CanSelectMultiple: CanSelectMultiple::<Identity, OFFSET>,
            IsSelectionRequired: IsSelectionRequired::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISelectionProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for ISelectionProvider {}
windows_core::imp::define_interface!(ISelectionProvider2, ISelectionProvider2_Vtbl, 0x14f68475_ee1c_44f6_a869_d239381f0fe7);
impl core::ops::Deref for ISelectionProvider2 {
    type Target = ISelectionProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ISelectionProvider2, windows_core::IUnknown, ISelectionProvider);
impl ISelectionProvider2 {
    pub unsafe fn FirstSelectedItem(&self) -> windows_core::Result<IRawElementProviderSimple> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FirstSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn LastSelectedItem(&self) -> windows_core::Result<IRawElementProviderSimple> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).LastSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn CurrentSelectedItem(&self) -> windows_core::Result<IRawElementProviderSimple> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CurrentSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn ItemCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ItemCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionProvider2_Vtbl {
    pub base__: ISelectionProvider_Vtbl,
    pub FirstSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub LastSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CurrentSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ItemCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(feature = "oaidl")]
pub trait ISelectionProvider2_Impl: ISelectionProvider_Impl {
    fn FirstSelectedItem(&self) -> windows_core::Result<IRawElementProviderSimple>;
    fn LastSelectedItem(&self) -> windows_core::Result<IRawElementProviderSimple>;
    fn CurrentSelectedItem(&self) -> windows_core::Result<IRawElementProviderSimple>;
    fn ItemCount(&self) -> windows_core::Result<i32>;
}
#[cfg(feature = "oaidl")]
impl ISelectionProvider2_Vtbl {
    pub const fn new<Identity: ISelectionProvider2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FirstSelectedItem<Identity: ISelectionProvider2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISelectionProvider2_Impl::FirstSelectedItem(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn LastSelectedItem<Identity: ISelectionProvider2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISelectionProvider2_Impl::LastSelectedItem(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CurrentSelectedItem<Identity: ISelectionProvider2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISelectionProvider2_Impl::CurrentSelectedItem(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ItemCount<Identity: ISelectionProvider2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISelectionProvider2_Impl::ItemCount(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ISelectionProvider_Vtbl::new::<Identity, OFFSET>(),
            FirstSelectedItem: FirstSelectedItem::<Identity, OFFSET>,
            LastSelectedItem: LastSelectedItem::<Identity, OFFSET>,
            CurrentSelectedItem: CurrentSelectedItem::<Identity, OFFSET>,
            ItemCount: ItemCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISelectionProvider2 as windows_core::Interface>::IID || iid == &<ISelectionProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for ISelectionProvider2 {}
windows_core::imp::define_interface!(ISpreadsheetItemProvider, ISpreadsheetItemProvider_Vtbl, 0xeaed4660_7b3d_4879_a2e6_365ce603f3d0);
windows_core::imp::interface_hierarchy!(ISpreadsheetItemProvider, windows_core::IUnknown);
impl ISpreadsheetItemProvider {
    pub unsafe fn Formula(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Formula)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetAnnotationObjects(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAnnotationObjects)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetAnnotationTypes(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAnnotationTypes)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpreadsheetItemProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Formula: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub GetAnnotationObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetAnnotationObjects: usize,
    #[cfg(feature = "oaidl")]
    pub GetAnnotationTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetAnnotationTypes: usize,
}
#[cfg(feature = "oaidl")]
pub trait ISpreadsheetItemProvider_Impl: windows_core::IUnknownImpl {
    fn Formula(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetAnnotationObjects(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn GetAnnotationTypes(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
}
#[cfg(feature = "oaidl")]
impl ISpreadsheetItemProvider_Vtbl {
    pub const fn new<Identity: ISpreadsheetItemProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Formula<Identity: ISpreadsheetItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpreadsheetItemProvider_Impl::Formula(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAnnotationObjects<Identity: ISpreadsheetItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpreadsheetItemProvider_Impl::GetAnnotationObjects(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAnnotationTypes<Identity: ISpreadsheetItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpreadsheetItemProvider_Impl::GetAnnotationTypes(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            Formula: Formula::<Identity, OFFSET>,
            GetAnnotationObjects: GetAnnotationObjects::<Identity, OFFSET>,
            GetAnnotationTypes: GetAnnotationTypes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpreadsheetItemProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for ISpreadsheetItemProvider {}
windows_core::imp::define_interface!(ISpreadsheetProvider, ISpreadsheetProvider_Vtbl, 0x6f6b5d35_5525_4f80_b758_85473832ffc7);
windows_core::imp::interface_hierarchy!(ISpreadsheetProvider, windows_core::IUnknown);
impl ISpreadsheetProvider {
    pub unsafe fn GetItemByName<P0>(&self, name: P0) -> windows_core::Result<IRawElementProviderSimple>
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetItemByName)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISpreadsheetProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetItemByName: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISpreadsheetProvider_Impl: windows_core::IUnknownImpl {
    fn GetItemByName(&self, name: &windows_core::PCWSTR) -> windows_core::Result<IRawElementProviderSimple>;
}
impl ISpreadsheetProvider_Vtbl {
    pub const fn new<Identity: ISpreadsheetProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetItemByName<Identity: ISpreadsheetProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, name: windows_core::PCWSTR, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpreadsheetProvider_Impl::GetItemByName(this, core::mem::transmute(&name)) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), GetItemByName: GetItemByName::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpreadsheetProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISpreadsheetProvider {}
windows_core::imp::define_interface!(IStylesProvider, IStylesProvider_Vtbl, 0x19b6b649_f5d7_4a6d_bdcb_129252be588a);
windows_core::imp::interface_hierarchy!(IStylesProvider, windows_core::IUnknown);
impl IStylesProvider {
    pub unsafe fn StyleId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StyleId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn StyleName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).StyleName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn FillColor(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FillColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn FillPatternStyle(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FillPatternStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn Shape(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Shape)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn FillPatternColor(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FillPatternColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ExtendedProperties(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ExtendedProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStylesProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StyleId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub StyleName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FillColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub FillPatternStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Shape: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FillPatternColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub ExtendedProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IStylesProvider_Impl: windows_core::IUnknownImpl {
    fn StyleId(&self) -> windows_core::Result<i32>;
    fn StyleName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn FillColor(&self) -> windows_core::Result<i32>;
    fn FillPatternStyle(&self) -> windows_core::Result<windows_core::BSTR>;
    fn Shape(&self) -> windows_core::Result<windows_core::BSTR>;
    fn FillPatternColor(&self) -> windows_core::Result<i32>;
    fn ExtendedProperties(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl IStylesProvider_Vtbl {
    pub const fn new<Identity: IStylesProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StyleId<Identity: IStylesProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStylesProvider_Impl::StyleId(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn StyleName<Identity: IStylesProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStylesProvider_Impl::StyleName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FillColor<Identity: IStylesProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStylesProvider_Impl::FillColor(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FillPatternStyle<Identity: IStylesProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStylesProvider_Impl::FillPatternStyle(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Shape<Identity: IStylesProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStylesProvider_Impl::Shape(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FillPatternColor<Identity: IStylesProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStylesProvider_Impl::FillPatternColor(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExtendedProperties<Identity: IStylesProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStylesProvider_Impl::ExtendedProperties(this) {
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
            StyleId: StyleId::<Identity, OFFSET>,
            StyleName: StyleName::<Identity, OFFSET>,
            FillColor: FillColor::<Identity, OFFSET>,
            FillPatternStyle: FillPatternStyle::<Identity, OFFSET>,
            Shape: Shape::<Identity, OFFSET>,
            FillPatternColor: FillPatternColor::<Identity, OFFSET>,
            ExtendedProperties: ExtendedProperties::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IStylesProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IStylesProvider {}
windows_core::imp::define_interface!(ISynchronizedInputProvider, ISynchronizedInputProvider_Vtbl, 0x29db1a06_02ce_4cf7_9b42_565d4fab20ee);
windows_core::imp::interface_hierarchy!(ISynchronizedInputProvider, windows_core::IUnknown);
impl ISynchronizedInputProvider {
    pub unsafe fn StartListening(&self, inputtype: SynchronizedInputType) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).StartListening)(windows_core::Interface::as_raw(self), inputtype) }
    }
    pub unsafe fn Cancel(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Cancel)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISynchronizedInputProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub StartListening: unsafe extern "system" fn(*mut core::ffi::c_void, SynchronizedInputType) -> windows_core::HRESULT,
    pub Cancel: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISynchronizedInputProvider_Impl: windows_core::IUnknownImpl {
    fn StartListening(&self, inputtype: SynchronizedInputType) -> windows_core::Result<()>;
    fn Cancel(&self) -> windows_core::Result<()>;
}
impl ISynchronizedInputProvider_Vtbl {
    pub const fn new<Identity: ISynchronizedInputProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn StartListening<Identity: ISynchronizedInputProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, inputtype: SynchronizedInputType) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISynchronizedInputProvider_Impl::StartListening(this, core::mem::transmute_copy(&inputtype)).into()
            }
        }
        unsafe extern "system" fn Cancel<Identity: ISynchronizedInputProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ISynchronizedInputProvider_Impl::Cancel(this).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            StartListening: StartListening::<Identity, OFFSET>,
            Cancel: Cancel::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISynchronizedInputProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ISynchronizedInputProvider {}
windows_core::imp::define_interface!(ITableItemProvider, ITableItemProvider_Vtbl, 0xb9734fa6_771f_4d78_9c90_2517999349cd);
windows_core::imp::interface_hierarchy!(ITableItemProvider, windows_core::IUnknown);
impl ITableItemProvider {
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetRowHeaderItems(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRowHeaderItems)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetColumnHeaderItems(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColumnHeaderItems)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableItemProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "oaidl")]
    pub GetRowHeaderItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetRowHeaderItems: usize,
    #[cfg(feature = "oaidl")]
    pub GetColumnHeaderItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetColumnHeaderItems: usize,
}
#[cfg(feature = "oaidl")]
pub trait ITableItemProvider_Impl: windows_core::IUnknownImpl {
    fn GetRowHeaderItems(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn GetColumnHeaderItems(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
}
#[cfg(feature = "oaidl")]
impl ITableItemProvider_Vtbl {
    pub const fn new<Identity: ITableItemProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRowHeaderItems<Identity: ITableItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITableItemProvider_Impl::GetRowHeaderItems(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetColumnHeaderItems<Identity: ITableItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITableItemProvider_Impl::GetColumnHeaderItems(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRowHeaderItems: GetRowHeaderItems::<Identity, OFFSET>,
            GetColumnHeaderItems: GetColumnHeaderItems::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITableItemProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for ITableItemProvider {}
windows_core::imp::define_interface!(ITableProvider, ITableProvider_Vtbl, 0x9c860395_97b3_490a_b52a_858cc22af166);
windows_core::imp::interface_hierarchy!(ITableProvider, windows_core::IUnknown);
impl ITableProvider {
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetRowHeaders(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRowHeaders)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetColumnHeaders(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColumnHeaders)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RowOrColumnMajor(&self) -> windows_core::Result<RowOrColumnMajor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RowOrColumnMajor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "oaidl")]
    pub GetRowHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetRowHeaders: usize,
    #[cfg(feature = "oaidl")]
    pub GetColumnHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetColumnHeaders: usize,
    pub RowOrColumnMajor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RowOrColumnMajor) -> windows_core::HRESULT,
}
#[cfg(feature = "oaidl")]
pub trait ITableProvider_Impl: windows_core::IUnknownImpl {
    fn GetRowHeaders(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn GetColumnHeaders(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn RowOrColumnMajor(&self) -> windows_core::Result<RowOrColumnMajor>;
}
#[cfg(feature = "oaidl")]
impl ITableProvider_Vtbl {
    pub const fn new<Identity: ITableProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRowHeaders<Identity: ITableProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITableProvider_Impl::GetRowHeaders(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetColumnHeaders<Identity: ITableProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITableProvider_Impl::GetColumnHeaders(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RowOrColumnMajor<Identity: ITableProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut RowOrColumnMajor) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITableProvider_Impl::RowOrColumnMajor(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetRowHeaders: GetRowHeaders::<Identity, OFFSET>,
            GetColumnHeaders: GetColumnHeaders::<Identity, OFFSET>,
            RowOrColumnMajor: RowOrColumnMajor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITableProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for ITableProvider {}
windows_core::imp::define_interface!(ITextChildProvider, ITextChildProvider_Vtbl, 0x4c2de2b9_c88f_4f88_a111_f1d336b7d1a9);
windows_core::imp::interface_hierarchy!(ITextChildProvider, windows_core::IUnknown);
impl ITextChildProvider {
    pub unsafe fn TextContainer(&self) -> windows_core::Result<IRawElementProviderSimple> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TextContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn TextRange(&self) -> windows_core::Result<ITextRangeProvider> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).TextRange)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextChildProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub TextContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TextRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITextChildProvider_Impl: windows_core::IUnknownImpl {
    fn TextContainer(&self) -> windows_core::Result<IRawElementProviderSimple>;
    fn TextRange(&self) -> windows_core::Result<ITextRangeProvider>;
}
impl ITextChildProvider_Vtbl {
    pub const fn new<Identity: ITextChildProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn TextContainer<Identity: ITextChildProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextChildProvider_Impl::TextContainer(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn TextRange<Identity: ITextChildProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextChildProvider_Impl::TextRange(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
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
        iid == &<ITextChildProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITextChildProvider {}
windows_core::imp::define_interface!(ITextEditProvider, ITextEditProvider_Vtbl, 0xea3605b4_3a05_400e_b5f9_4e91b40f6176);
impl core::ops::Deref for ITextEditProvider {
    type Target = ITextProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITextEditProvider, windows_core::IUnknown, ITextProvider);
impl ITextEditProvider {
    pub unsafe fn GetActiveComposition(&self) -> windows_core::Result<ITextRangeProvider> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetActiveComposition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetConversionTarget(&self) -> windows_core::Result<ITextRangeProvider> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConversionTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextEditProvider_Vtbl {
    pub base__: ITextProvider_Vtbl,
    pub GetActiveComposition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetConversionTarget: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "oaidl")]
pub trait ITextEditProvider_Impl: ITextProvider_Impl {
    fn GetActiveComposition(&self) -> windows_core::Result<ITextRangeProvider>;
    fn GetConversionTarget(&self) -> windows_core::Result<ITextRangeProvider>;
}
#[cfg(feature = "oaidl")]
impl ITextEditProvider_Vtbl {
    pub const fn new<Identity: ITextEditProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetActiveComposition<Identity: ITextEditProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextEditProvider_Impl::GetActiveComposition(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetConversionTarget<Identity: ITextEditProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextEditProvider_Impl::GetConversionTarget(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ITextProvider_Vtbl::new::<Identity, OFFSET>(),
            GetActiveComposition: GetActiveComposition::<Identity, OFFSET>,
            GetConversionTarget: GetConversionTarget::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextEditProvider as windows_core::Interface>::IID || iid == &<ITextProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for ITextEditProvider {}
windows_core::imp::define_interface!(ITextProvider, ITextProvider_Vtbl, 0x3589c92c_63f3_4367_99bb_ada653b77cf2);
windows_core::imp::interface_hierarchy!(ITextProvider, windows_core::IUnknown);
impl ITextProvider {
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetSelection(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetVisibleRanges(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetVisibleRanges)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RangeFromChild<P0>(&self, childelement: P0) -> windows_core::Result<ITextRangeProvider>
    where
        P0: windows_core::Param<IRawElementProviderSimple>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RangeFromChild)(windows_core::Interface::as_raw(self), childelement.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn RangeFromPoint(&self, point: UiaPoint) -> windows_core::Result<ITextRangeProvider> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RangeFromPoint)(windows_core::Interface::as_raw(self), point, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn DocumentRange(&self) -> windows_core::Result<ITextRangeProvider> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).DocumentRange)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn SupportedTextSelection(&self) -> windows_core::Result<SupportedTextSelection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).SupportedTextSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "oaidl")]
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetSelection: usize,
    #[cfg(feature = "oaidl")]
    pub GetVisibleRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetVisibleRanges: usize,
    pub RangeFromChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RangeFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, UiaPoint, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DocumentRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SupportedTextSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SupportedTextSelection) -> windows_core::HRESULT,
}
#[cfg(feature = "oaidl")]
pub trait ITextProvider_Impl: windows_core::IUnknownImpl {
    fn GetSelection(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn GetVisibleRanges(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn RangeFromChild(&self, childelement: windows_core::Ref<IRawElementProviderSimple>) -> windows_core::Result<ITextRangeProvider>;
    fn RangeFromPoint(&self, point: &UiaPoint) -> windows_core::Result<ITextRangeProvider>;
    fn DocumentRange(&self) -> windows_core::Result<ITextRangeProvider>;
    fn SupportedTextSelection(&self) -> windows_core::Result<SupportedTextSelection>;
}
#[cfg(feature = "oaidl")]
impl ITextProvider_Vtbl {
    pub const fn new<Identity: ITextProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSelection<Identity: ITextProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextProvider_Impl::GetSelection(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetVisibleRanges<Identity: ITextProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextProvider_Impl::GetVisibleRanges(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RangeFromChild<Identity: ITextProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, childelement: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextProvider_Impl::RangeFromChild(this, core::mem::transmute_copy(&childelement)) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RangeFromPoint<Identity: ITextProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, point: UiaPoint, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextProvider_Impl::RangeFromPoint(this, core::mem::transmute(&point)) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn DocumentRange<Identity: ITextProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextProvider_Impl::DocumentRange(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn SupportedTextSelection<Identity: ITextProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut SupportedTextSelection) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextProvider_Impl::SupportedTextSelection(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetSelection: GetSelection::<Identity, OFFSET>,
            GetVisibleRanges: GetVisibleRanges::<Identity, OFFSET>,
            RangeFromChild: RangeFromChild::<Identity, OFFSET>,
            RangeFromPoint: RangeFromPoint::<Identity, OFFSET>,
            DocumentRange: DocumentRange::<Identity, OFFSET>,
            SupportedTextSelection: SupportedTextSelection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for ITextProvider {}
windows_core::imp::define_interface!(ITextProvider2, ITextProvider2_Vtbl, 0x0dc5e6ed_3e16_4bf1_8f9a_a979878bc195);
impl core::ops::Deref for ITextProvider2 {
    type Target = ITextProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITextProvider2, windows_core::IUnknown, ITextProvider);
impl ITextProvider2 {
    pub unsafe fn RangeFromAnnotation<P0>(&self, annotationelement: P0) -> windows_core::Result<ITextRangeProvider>
    where
        P0: windows_core::Param<IRawElementProviderSimple>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RangeFromAnnotation)(windows_core::Interface::as_raw(self), annotationelement.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCaretRange(&self, isactive: *mut windows_core::BOOL) -> windows_core::Result<ITextRangeProvider> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCaretRange)(windows_core::Interface::as_raw(self), isactive as _, &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextProvider2_Vtbl {
    pub base__: ITextProvider_Vtbl,
    pub RangeFromAnnotation: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCaretRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "oaidl")]
pub trait ITextProvider2_Impl: ITextProvider_Impl {
    fn RangeFromAnnotation(&self, annotationelement: windows_core::Ref<IRawElementProviderSimple>) -> windows_core::Result<ITextRangeProvider>;
    fn GetCaretRange(&self, isactive: *mut windows_core::BOOL) -> windows_core::Result<ITextRangeProvider>;
}
#[cfg(feature = "oaidl")]
impl ITextProvider2_Vtbl {
    pub const fn new<Identity: ITextProvider2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RangeFromAnnotation<Identity: ITextProvider2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, annotationelement: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextProvider2_Impl::RangeFromAnnotation(this, core::mem::transmute_copy(&annotationelement)) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetCaretRange<Identity: ITextProvider2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, isactive: *mut windows_core::BOOL, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextProvider2_Impl::GetCaretRange(this, core::mem::transmute_copy(&isactive)) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: ITextProvider_Vtbl::new::<Identity, OFFSET>(),
            RangeFromAnnotation: RangeFromAnnotation::<Identity, OFFSET>,
            GetCaretRange: GetCaretRange::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextProvider2 as windows_core::Interface>::IID || iid == &<ITextProvider as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for ITextProvider2 {}
windows_core::imp::define_interface!(ITextRangeProvider, ITextRangeProvider_Vtbl, 0x5347ad7b_c355_46f8_aff5_909033582f63);
windows_core::imp::interface_hierarchy!(ITextRangeProvider, windows_core::IUnknown);
impl ITextRangeProvider {
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
    pub unsafe fn CompareEndpoints<P1>(&self, endpoint: TextPatternRangeEndpoint, targetrange: P1, targetendpoint: TextPatternRangeEndpoint) -> windows_core::Result<i32>
    where
        P1: windows_core::Param<Self>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CompareEndpoints)(windows_core::Interface::as_raw(self), endpoint, targetrange.param().abi(), targetendpoint, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ExpandToEnclosingUnit(&self, unit: TextUnit) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ExpandToEnclosingUnit)(windows_core::Interface::as_raw(self), unit) }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn FindAttribute(&self, attributeid: TEXTATTRIBUTEID, val: &super::oaidl::VARIANT, backward: bool) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindAttribute)(windows_core::Interface::as_raw(self), attributeid, core::mem::transmute_copy(val), backward.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindText(&self, text: &windows_core::BSTR, backward: bool, ignorecase: bool) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindText)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(text), backward.into(), ignorecase.into(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetAttributeValue(&self, attributeid: TEXTATTRIBUTEID) -> windows_core::Result<super::oaidl::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttributeValue)(windows_core::Interface::as_raw(self), attributeid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetBoundingRectangles(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBoundingRectangles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetEnclosingElement(&self) -> windows_core::Result<IRawElementProviderSimple> {
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
    pub unsafe fn Move(&self, unit: TextUnit, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Move)(windows_core::Interface::as_raw(self), unit, count, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveEndpointByUnit(&self, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).MoveEndpointByUnit)(windows_core::Interface::as_raw(self), endpoint, unit, count, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn MoveEndpointByRange<P1>(&self, endpoint: TextPatternRangeEndpoint, targetrange: P1, targetendpoint: TextPatternRangeEndpoint) -> windows_core::HRESULT
    where
        P1: windows_core::Param<Self>,
    {
        unsafe { (windows_core::Interface::vtable(self).MoveEndpointByRange)(windows_core::Interface::as_raw(self), endpoint, targetrange.param().abi(), targetendpoint) }
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
pub struct ITextRangeProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Compare: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CompareEndpoints: unsafe extern "system" fn(*mut core::ffi::c_void, TextPatternRangeEndpoint, *mut core::ffi::c_void, TextPatternRangeEndpoint, *mut i32) -> windows_core::HRESULT,
    pub ExpandToEnclosingUnit: unsafe extern "system" fn(*mut core::ffi::c_void, TextUnit) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub FindAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, TEXTATTRIBUTEID, super::oaidl::VARIANT, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    FindAttribute: usize,
    pub FindText: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
    pub GetAttributeValue: unsafe extern "system" fn(*mut core::ffi::c_void, TEXTATTRIBUTEID, *mut super::oaidl::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase")))]
    GetAttributeValue: usize,
    #[cfg(feature = "oaidl")]
    pub GetBoundingRectangles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetBoundingRectangles: usize,
    pub GetEnclosingElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetText: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Move: unsafe extern "system" fn(*mut core::ffi::c_void, TextUnit, i32, *mut i32) -> windows_core::HRESULT,
    pub MoveEndpointByUnit: unsafe extern "system" fn(*mut core::ffi::c_void, TextPatternRangeEndpoint, TextUnit, i32, *mut i32) -> windows_core::HRESULT,
    pub MoveEndpointByRange: unsafe extern "system" fn(*mut core::ffi::c_void, TextPatternRangeEndpoint, *mut core::ffi::c_void, TextPatternRangeEndpoint) -> windows_core::HRESULT,
    pub Select: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub AddToSelection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RemoveFromSelection: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ScrollIntoView: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::BOOL) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub GetChildren: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetChildren: usize,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITextRangeProvider_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<ITextRangeProvider>;
    fn Compare(&self, range: windows_core::Ref<ITextRangeProvider>) -> windows_core::Result<windows_core::BOOL>;
    fn CompareEndpoints(&self, endpoint: TextPatternRangeEndpoint, targetrange: windows_core::Ref<ITextRangeProvider>, targetendpoint: TextPatternRangeEndpoint) -> windows_core::Result<i32>;
    fn ExpandToEnclosingUnit(&self, unit: TextUnit) -> windows_core::Result<()>;
    fn FindAttribute(&self, attributeid: TEXTATTRIBUTEID, val: &super::oaidl::VARIANT, backward: windows_core::BOOL) -> windows_core::Result<ITextRangeProvider>;
    fn FindText(&self, text: &windows_core::BSTR, backward: windows_core::BOOL, ignorecase: windows_core::BOOL) -> windows_core::Result<ITextRangeProvider>;
    fn GetAttributeValue(&self, attributeid: TEXTATTRIBUTEID) -> windows_core::Result<super::oaidl::VARIANT>;
    fn GetBoundingRectangles(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
    fn GetEnclosingElement(&self) -> windows_core::Result<IRawElementProviderSimple>;
    fn GetText(&self, maxlength: i32) -> windows_core::Result<windows_core::BSTR>;
    fn Move(&self, unit: TextUnit, count: i32) -> windows_core::Result<i32>;
    fn MoveEndpointByUnit(&self, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32) -> windows_core::Result<i32>;
    fn MoveEndpointByRange(&self, endpoint: TextPatternRangeEndpoint, targetrange: windows_core::Ref<ITextRangeProvider>, targetendpoint: TextPatternRangeEndpoint) -> windows_core::Result<()>;
    fn Select(&self) -> windows_core::Result<()>;
    fn AddToSelection(&self) -> windows_core::Result<()>;
    fn RemoveFromSelection(&self) -> windows_core::Result<()>;
    fn ScrollIntoView(&self, aligntotop: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetChildren(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl ITextRangeProvider_Vtbl {
    pub const fn new<Identity: ITextRangeProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Clone<Identity: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRangeProvider_Impl::Clone(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Compare<Identity: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, range: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRangeProvider_Impl::Compare(this, core::mem::transmute_copy(&range)) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CompareEndpoints<Identity: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endpoint: TextPatternRangeEndpoint, targetrange: *mut core::ffi::c_void, targetendpoint: TextPatternRangeEndpoint, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRangeProvider_Impl::CompareEndpoints(this, core::mem::transmute_copy(&endpoint), core::mem::transmute_copy(&targetrange), core::mem::transmute_copy(&targetendpoint)) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ExpandToEnclosingUnit<Identity: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextUnit) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRangeProvider_Impl::ExpandToEnclosingUnit(this, core::mem::transmute_copy(&unit)).into()
            }
        }
        unsafe extern "system" fn FindAttribute<Identity: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributeid: TEXTATTRIBUTEID, val: super::oaidl::VARIANT, backward: windows_core::BOOL, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRangeProvider_Impl::FindAttribute(this, core::mem::transmute_copy(&attributeid), core::mem::transmute(&val), core::mem::transmute_copy(&backward)) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn FindText<Identity: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut core::ffi::c_void, backward: windows_core::BOOL, ignorecase: windows_core::BOOL, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRangeProvider_Impl::FindText(this, core::mem::transmute(&text), core::mem::transmute_copy(&backward), core::mem::transmute_copy(&ignorecase)) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAttributeValue<Identity: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributeid: TEXTATTRIBUTEID, pretval: *mut super::oaidl::VARIANT) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRangeProvider_Impl::GetAttributeValue(this, core::mem::transmute_copy(&attributeid)) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetBoundingRectangles<Identity: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRangeProvider_Impl::GetBoundingRectangles(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetEnclosingElement<Identity: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRangeProvider_Impl::GetEnclosingElement(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetText<Identity: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, maxlength: i32, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRangeProvider_Impl::GetText(this, core::mem::transmute_copy(&maxlength)) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Move<Identity: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, unit: TextUnit, count: i32, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRangeProvider_Impl::Move(this, core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveEndpointByUnit<Identity: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRangeProvider_Impl::MoveEndpointByUnit(this, core::mem::transmute_copy(&endpoint), core::mem::transmute_copy(&unit), core::mem::transmute_copy(&count)) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn MoveEndpointByRange<Identity: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, endpoint: TextPatternRangeEndpoint, targetrange: *mut core::ffi::c_void, targetendpoint: TextPatternRangeEndpoint) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRangeProvider_Impl::MoveEndpointByRange(this, core::mem::transmute_copy(&endpoint), core::mem::transmute_copy(&targetrange), core::mem::transmute_copy(&targetendpoint)).into()
            }
        }
        unsafe extern "system" fn Select<Identity: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRangeProvider_Impl::Select(this).into()
            }
        }
        unsafe extern "system" fn AddToSelection<Identity: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRangeProvider_Impl::AddToSelection(this).into()
            }
        }
        unsafe extern "system" fn RemoveFromSelection<Identity: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRangeProvider_Impl::RemoveFromSelection(this).into()
            }
        }
        unsafe extern "system" fn ScrollIntoView<Identity: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, aligntotop: windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRangeProvider_Impl::ScrollIntoView(this, core::mem::transmute_copy(&aligntotop)).into()
            }
        }
        unsafe extern "system" fn GetChildren<Identity: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRangeProvider_Impl::GetChildren(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
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
        iid == &<ITextRangeProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITextRangeProvider {}
windows_core::imp::define_interface!(ITextRangeProvider2, ITextRangeProvider2_Vtbl, 0x9bbce42c_1921_4f18_89ca_dba1910a0386);
impl core::ops::Deref for ITextRangeProvider2 {
    type Target = ITextRangeProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITextRangeProvider2, windows_core::IUnknown, ITextRangeProvider);
impl ITextRangeProvider2 {
    pub unsafe fn ShowContextMenu(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ShowContextMenu)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextRangeProvider2_Vtbl {
    pub base__: ITextRangeProvider_Vtbl,
    pub ShowContextMenu: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITextRangeProvider2_Impl: ITextRangeProvider_Impl {
    fn ShowContextMenu(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl ITextRangeProvider2_Vtbl {
    pub const fn new<Identity: ITextRangeProvider2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ShowContextMenu<Identity: ITextRangeProvider2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITextRangeProvider2_Impl::ShowContextMenu(this).into()
            }
        }
        Self { base__: ITextRangeProvider_Vtbl::new::<Identity, OFFSET>(), ShowContextMenu: ShowContextMenu::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextRangeProvider2 as windows_core::Interface>::IID || iid == &<ITextRangeProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITextRangeProvider2 {}
windows_core::imp::define_interface!(IToggleProvider, IToggleProvider_Vtbl, 0x56d00bd0_c4f4_433c_a836_1a52a57e0892);
windows_core::imp::interface_hierarchy!(IToggleProvider, windows_core::IUnknown);
impl IToggleProvider {
    pub unsafe fn Toggle(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Toggle)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn ToggleState(&self) -> windows_core::Result<ToggleState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ToggleState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Toggle: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub ToggleState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ToggleState) -> windows_core::HRESULT,
}
pub trait IToggleProvider_Impl: windows_core::IUnknownImpl {
    fn Toggle(&self) -> windows_core::Result<()>;
    fn ToggleState(&self) -> windows_core::Result<ToggleState>;
}
impl IToggleProvider_Vtbl {
    pub const fn new<Identity: IToggleProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Toggle<Identity: IToggleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IToggleProvider_Impl::Toggle(this).into()
            }
        }
        unsafe extern "system" fn ToggleState<Identity: IToggleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut ToggleState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IToggleProvider_Impl::ToggleState(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Toggle: Toggle::<Identity, OFFSET>, ToggleState: ToggleState::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IToggleProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IToggleProvider {}
windows_core::imp::define_interface!(ITransformProvider, ITransformProvider_Vtbl, 0x6829ddc4_4f91_4ffa_b86f_bd3e2987cb4c);
windows_core::imp::interface_hierarchy!(ITransformProvider, windows_core::IUnknown);
impl ITransformProvider {
    pub unsafe fn Move(&self, x: f64, y: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Move)(windows_core::Interface::as_raw(self), x, y) }
    }
    pub unsafe fn Resize(&self, width: f64, height: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Resize)(windows_core::Interface::as_raw(self), width, height) }
    }
    pub unsafe fn Rotate(&self, degrees: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Rotate)(windows_core::Interface::as_raw(self), degrees) }
    }
    pub unsafe fn CanMove(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanMove)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CanResize(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanResize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CanRotate(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanRotate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Move: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub Resize: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub Rotate: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub CanMove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CanResize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CanRotate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait ITransformProvider_Impl: windows_core::IUnknownImpl {
    fn Move(&self, x: f64, y: f64) -> windows_core::Result<()>;
    fn Resize(&self, width: f64, height: f64) -> windows_core::Result<()>;
    fn Rotate(&self, degrees: f64) -> windows_core::Result<()>;
    fn CanMove(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CanResize(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CanRotate(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl ITransformProvider_Vtbl {
    pub const fn new<Identity: ITransformProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Move<Identity: ITransformProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, x: f64, y: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransformProvider_Impl::Move(this, core::mem::transmute_copy(&x), core::mem::transmute_copy(&y)).into()
            }
        }
        unsafe extern "system" fn Resize<Identity: ITransformProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, width: f64, height: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransformProvider_Impl::Resize(this, core::mem::transmute_copy(&width), core::mem::transmute_copy(&height)).into()
            }
        }
        unsafe extern "system" fn Rotate<Identity: ITransformProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, degrees: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransformProvider_Impl::Rotate(this, core::mem::transmute_copy(&degrees)).into()
            }
        }
        unsafe extern "system" fn CanMove<Identity: ITransformProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransformProvider_Impl::CanMove(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanResize<Identity: ITransformProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransformProvider_Impl::CanResize(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanRotate<Identity: ITransformProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransformProvider_Impl::CanRotate(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
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
            CanMove: CanMove::<Identity, OFFSET>,
            CanResize: CanResize::<Identity, OFFSET>,
            CanRotate: CanRotate::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransformProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransformProvider {}
windows_core::imp::define_interface!(ITransformProvider2, ITransformProvider2_Vtbl, 0x4758742f_7ac2_460c_bc48_09fc09308a93);
impl core::ops::Deref for ITransformProvider2 {
    type Target = ITransformProvider;
    fn deref(&self) -> &Self::Target {
        unsafe { core::mem::transmute(self) }
    }
}
windows_core::imp::interface_hierarchy!(ITransformProvider2, windows_core::IUnknown, ITransformProvider);
impl ITransformProvider2 {
    pub unsafe fn Zoom(&self, zoom: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Zoom)(windows_core::Interface::as_raw(self), zoom) }
    }
    pub unsafe fn CanZoom(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanZoom)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ZoomLevel(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ZoomLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ZoomMinimum(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ZoomMinimum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ZoomMaximum(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ZoomMaximum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ZoomByUnit(&self, zoomunit: ZoomUnit) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).ZoomByUnit)(windows_core::Interface::as_raw(self), zoomunit) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITransformProvider2_Vtbl {
    pub base__: ITransformProvider_Vtbl,
    pub Zoom: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub CanZoom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub ZoomLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ZoomMinimum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ZoomMaximum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ZoomByUnit: unsafe extern "system" fn(*mut core::ffi::c_void, ZoomUnit) -> windows_core::HRESULT,
}
pub trait ITransformProvider2_Impl: ITransformProvider_Impl {
    fn Zoom(&self, zoom: f64) -> windows_core::Result<()>;
    fn CanZoom(&self) -> windows_core::Result<windows_core::BOOL>;
    fn ZoomLevel(&self) -> windows_core::Result<f64>;
    fn ZoomMinimum(&self) -> windows_core::Result<f64>;
    fn ZoomMaximum(&self) -> windows_core::Result<f64>;
    fn ZoomByUnit(&self, zoomunit: ZoomUnit) -> windows_core::Result<()>;
}
impl ITransformProvider2_Vtbl {
    pub const fn new<Identity: ITransformProvider2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Zoom<Identity: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, zoom: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransformProvider2_Impl::Zoom(this, core::mem::transmute_copy(&zoom)).into()
            }
        }
        unsafe extern "system" fn CanZoom<Identity: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransformProvider2_Impl::CanZoom(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ZoomLevel<Identity: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransformProvider2_Impl::ZoomLevel(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ZoomMinimum<Identity: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransformProvider2_Impl::ZoomMinimum(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ZoomMaximum<Identity: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransformProvider2_Impl::ZoomMaximum(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ZoomByUnit<Identity: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, zoomunit: ZoomUnit) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                ITransformProvider2_Impl::ZoomByUnit(this, core::mem::transmute_copy(&zoomunit)).into()
            }
        }
        Self {
            base__: ITransformProvider_Vtbl::new::<Identity, OFFSET>(),
            Zoom: Zoom::<Identity, OFFSET>,
            CanZoom: CanZoom::<Identity, OFFSET>,
            ZoomLevel: ZoomLevel::<Identity, OFFSET>,
            ZoomMinimum: ZoomMinimum::<Identity, OFFSET>,
            ZoomMaximum: ZoomMaximum::<Identity, OFFSET>,
            ZoomByUnit: ZoomByUnit::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITransformProvider2 as windows_core::Interface>::IID || iid == &<ITransformProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for ITransformProvider2 {}
windows_core::imp::define_interface!(IUIAutomationClientConnectionCallback, IUIAutomationClientConnectionCallback_Vtbl, 0x5b8e8f2a_9c7d_4f3e_a1b2_8d6e9f4c0a1b);
windows_core::imp::interface_hierarchy!(IUIAutomationClientConnectionCallback, windows_core::IUnknown);
impl IUIAutomationClientConnectionCallback {
    pub unsafe fn OnConnected<P0>(&self, clientinfo: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationClientInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnConnected)(windows_core::Interface::as_raw(self), clientinfo.param().abi()) }
    }
    pub unsafe fn OnDisconnected<P0>(&self, clientinfo: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<IUIAutomationClientInfo>,
    {
        unsafe { (windows_core::Interface::vtable(self).OnDisconnected)(windows_core::Interface::as_raw(self), clientinfo.param().abi()) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationClientConnectionCallback_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub OnConnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub OnDisconnected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAutomationClientConnectionCallback_Impl: windows_core::IUnknownImpl {
    fn OnConnected(&self, clientinfo: windows_core::Ref<IUIAutomationClientInfo>) -> windows_core::Result<()>;
    fn OnDisconnected(&self, clientinfo: windows_core::Ref<IUIAutomationClientInfo>) -> windows_core::Result<()>;
}
impl IUIAutomationClientConnectionCallback_Vtbl {
    pub const fn new<Identity: IUIAutomationClientConnectionCallback_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn OnConnected<Identity: IUIAutomationClientConnectionCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientinfo: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationClientConnectionCallback_Impl::OnConnected(this, core::mem::transmute_copy(&clientinfo)).into()
            }
        }
        unsafe extern "system" fn OnDisconnected<Identity: IUIAutomationClientConnectionCallback_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clientinfo: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationClientConnectionCallback_Impl::OnDisconnected(this, core::mem::transmute_copy(&clientinfo)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            OnConnected: OnConnected::<Identity, OFFSET>,
            OnDisconnected: OnDisconnected::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationClientConnectionCallback as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationClientConnectionCallback {}
windows_core::imp::define_interface!(IUIAutomationClientInfo, IUIAutomationClientInfo_Vtbl, 0xb2e8a3f1_4c5d_4e7a_8f6b_3d2e1c9a0b8f);
windows_core::imp::interface_hierarchy!(IUIAutomationClientInfo, windows_core::IUnknown);
impl IUIAutomationClientInfo {
    pub unsafe fn ProcessId(&self) -> windows_core::Result<u32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProcessId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn ProcessName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ProcessName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationClientInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub ProcessId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
    pub ProcessName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IUIAutomationClientInfo_Impl: windows_core::IUnknownImpl {
    fn ProcessId(&self) -> windows_core::Result<u32>;
    fn ProcessName(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl IUIAutomationClientInfo_Vtbl {
    pub const fn new<Identity: IUIAutomationClientInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn ProcessId<Identity: IUIAutomationClientInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, processid: *mut u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationClientInfo_Impl::ProcessId(this) {
                    Ok(ok__) => {
                        processid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn ProcessName<Identity: IUIAutomationClientInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, processname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationClientInfo_Impl::ProcessName(this) {
                    Ok(ok__) => {
                        processname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ProcessId: ProcessId::<Identity, OFFSET>,
            ProcessName: ProcessName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationClientInfo as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationClientInfo {}
windows_core::imp::define_interface!(IUIAutomationClientInfoSource, IUIAutomationClientInfoSource_Vtbl, 0xf4b8a2e1_9c3d_4a7e_8f6b_2d5e4c1a9b8f);
windows_core::imp::interface_hierarchy!(IUIAutomationClientInfoSource, windows_core::IUnknown);
impl IUIAutomationClientInfoSource {
    pub unsafe fn RegisterClientConnectionCallback<P0>(&self, callback: P0) -> windows_core::Result<u64>
    where
        P0: windows_core::Param<IUIAutomationClientConnectionCallback>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterClientConnectionCallback)(windows_core::Interface::as_raw(self), callback.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn UnregisterClientConnectionCallback(&self, handle: u64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).UnregisterClientConnectionCallback)(windows_core::Interface::as_raw(self), handle) }
    }
    #[cfg(feature = "oaidl")]
    pub unsafe fn GetConnectedClients(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConnectedClients)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationClientInfoSource_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterClientConnectionCallback: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut u64) -> windows_core::HRESULT,
    pub UnregisterClientConnectionCallback: unsafe extern "system" fn(*mut core::ffi::c_void, u64) -> windows_core::HRESULT,
    #[cfg(feature = "oaidl")]
    pub GetConnectedClients: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(feature = "oaidl"))]
    GetConnectedClients: usize,
}
#[cfg(feature = "oaidl")]
pub trait IUIAutomationClientInfoSource_Impl: windows_core::IUnknownImpl {
    fn RegisterClientConnectionCallback(&self, callback: windows_core::Ref<IUIAutomationClientConnectionCallback>) -> windows_core::Result<u64>;
    fn UnregisterClientConnectionCallback(&self, handle: u64) -> windows_core::Result<()>;
    fn GetConnectedClients(&self) -> windows_core::Result<*mut super::oaidl::SAFEARRAY>;
}
#[cfg(feature = "oaidl")]
impl IUIAutomationClientInfoSource_Vtbl {
    pub const fn new<Identity: IUIAutomationClientInfoSource_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterClientConnectionCallback<Identity: IUIAutomationClientInfoSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, callback: *mut core::ffi::c_void, handle: *mut u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationClientInfoSource_Impl::RegisterClientConnectionCallback(this, core::mem::transmute_copy(&callback)) {
                    Ok(ok__) => {
                        handle.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn UnregisterClientConnectionCallback<Identity: IUIAutomationClientInfoSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, handle: u64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationClientInfoSource_Impl::UnregisterClientConnectionCallback(this, core::mem::transmute_copy(&handle)).into()
            }
        }
        unsafe extern "system" fn GetConnectedClients<Identity: IUIAutomationClientInfoSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clients: *mut *mut super::oaidl::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationClientInfoSource_Impl::GetConnectedClients(this) {
                    Ok(ok__) => {
                        clients.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterClientConnectionCallback: RegisterClientConnectionCallback::<Identity, OFFSET>,
            UnregisterClientConnectionCallback: UnregisterClientConnectionCallback::<Identity, OFFSET>,
            GetConnectedClients: GetConnectedClients::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationClientInfoSource as windows_core::Interface>::IID
    }
}
#[cfg(feature = "oaidl")]
impl windows_core::RuntimeName for IUIAutomationClientInfoSource {}
windows_core::imp::define_interface!(IUIAutomationPatternHandler, IUIAutomationPatternHandler_Vtbl, 0xd97022f3_a947_465e_8b2a_ac4315fa54e8);
windows_core::imp::interface_hierarchy!(IUIAutomationPatternHandler, windows_core::IUnknown);
impl IUIAutomationPatternHandler {
    pub unsafe fn CreateClientWrapper<P0>(&self, ppatterninstance: P0) -> windows_core::Result<windows_core::IUnknown>
    where
        P0: windows_core::Param<IUIAutomationPatternInstance>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CreateClientWrapper)(windows_core::Interface::as_raw(self), ppatterninstance.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub unsafe fn Dispatch<P0>(&self, ptarget: P0, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::IUnknown>,
    {
        unsafe { (windows_core::Interface::vtable(self).Dispatch)(windows_core::Interface::as_raw(self), ptarget.param().abi(), index, pparams, cparams) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationPatternHandler_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub CreateClientWrapper: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Dispatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, u32, *const UIAutomationParameter, u32) -> windows_core::HRESULT,
}
pub trait IUIAutomationPatternHandler_Impl: windows_core::IUnknownImpl {
    fn CreateClientWrapper(&self, ppatterninstance: windows_core::Ref<IUIAutomationPatternInstance>) -> windows_core::Result<windows_core::IUnknown>;
    fn Dispatch(&self, ptarget: windows_core::Ref<windows_core::IUnknown>, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> windows_core::Result<()>;
}
impl IUIAutomationPatternHandler_Vtbl {
    pub const fn new<Identity: IUIAutomationPatternHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn CreateClientWrapper<Identity: IUIAutomationPatternHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ppatterninstance: *mut core::ffi::c_void, pclientwrapper: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationPatternHandler_Impl::CreateClientWrapper(this, core::mem::transmute_copy(&ppatterninstance)) {
                    Ok(ok__) => {
                        pclientwrapper.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn Dispatch<Identity: IUIAutomationPatternHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptarget: *mut core::ffi::c_void, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationPatternHandler_Impl::Dispatch(this, core::mem::transmute_copy(&ptarget), core::mem::transmute_copy(&index), core::mem::transmute_copy(&pparams), core::mem::transmute_copy(&cparams)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CreateClientWrapper: CreateClientWrapper::<Identity, OFFSET>,
            Dispatch: Dispatch::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationPatternHandler as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationPatternHandler {}
windows_core::imp::define_interface!(IUIAutomationPatternInstance, IUIAutomationPatternInstance_Vtbl, 0xc03a7fe4_9431_409f_bed8_ae7c2299bc8d);
windows_core::imp::interface_hierarchy!(IUIAutomationPatternInstance, windows_core::IUnknown);
impl IUIAutomationPatternInstance {
    pub unsafe fn GetProperty(&self, index: u32, cached: bool, r#type: UIAutomationType, pptr: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), index, cached.into(), r#type, pptr as _) }
    }
    pub unsafe fn CallMethod(&self, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CallMethod)(windows_core::Interface::as_raw(self), index, pparams, cparams) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationPatternInstance_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, u32, windows_core::BOOL, UIAutomationType, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CallMethod: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const UIAutomationParameter, u32) -> windows_core::HRESULT,
}
pub trait IUIAutomationPatternInstance_Impl: windows_core::IUnknownImpl {
    fn GetProperty(&self, index: u32, cached: windows_core::BOOL, r#type: UIAutomationType, pptr: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CallMethod(&self, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> windows_core::Result<()>;
}
impl IUIAutomationPatternInstance_Vtbl {
    pub const fn new<Identity: IUIAutomationPatternInstance_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProperty<Identity: IUIAutomationPatternInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, cached: windows_core::BOOL, r#type: UIAutomationType, pptr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationPatternInstance_Impl::GetProperty(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&cached), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pptr)).into()
            }
        }
        unsafe extern "system" fn CallMethod<Identity: IUIAutomationPatternInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: u32, pparams: *const UIAutomationParameter, cparams: u32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationPatternInstance_Impl::CallMethod(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&pparams), core::mem::transmute_copy(&cparams)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetProperty: GetProperty::<Identity, OFFSET>,
            CallMethod: CallMethod::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationPatternInstance as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationPatternInstance {}
windows_core::imp::define_interface!(IUIAutomationRegistrar, IUIAutomationRegistrar_Vtbl, 0x8609c4ec_4a1a_4d88_a357_5a66e060e1cf);
windows_core::imp::interface_hierarchy!(IUIAutomationRegistrar, windows_core::IUnknown);
impl IUIAutomationRegistrar {
    pub unsafe fn RegisterProperty(&self, property: *const UIAutomationPropertyInfo) -> windows_core::Result<PROPERTYID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterProperty)(windows_core::Interface::as_raw(self), property, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RegisterEvent(&self, event: *const UIAutomationEventInfo) -> windows_core::Result<EVENTID> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RegisterEvent)(windows_core::Interface::as_raw(self), event, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn RegisterPattern(&self, pattern: *const UIAutomationPatternInfo, ppatternid: *mut PATTERNID, ppatternavailablepropertyid: *mut PROPERTYID, propertyidcount: u32, ppropertyids: *mut PROPERTYID, eventidcount: u32, peventids: *mut EVENTID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterPattern)(windows_core::Interface::as_raw(self), pattern, ppatternid as _, ppatternavailablepropertyid as _, propertyidcount, ppropertyids as _, eventidcount, peventids as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationRegistrar_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub RegisterProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const UIAutomationPropertyInfo, *mut PROPERTYID) -> windows_core::HRESULT,
    pub RegisterEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *const UIAutomationEventInfo, *mut EVENTID) -> windows_core::HRESULT,
    pub RegisterPattern: unsafe extern "system" fn(*mut core::ffi::c_void, *const UIAutomationPatternInfo, *mut PATTERNID, *mut PROPERTYID, u32, *mut PROPERTYID, u32, *mut EVENTID) -> windows_core::HRESULT,
}
pub trait IUIAutomationRegistrar_Impl: windows_core::IUnknownImpl {
    fn RegisterProperty(&self, property: *const UIAutomationPropertyInfo) -> windows_core::Result<PROPERTYID>;
    fn RegisterEvent(&self, event: *const UIAutomationEventInfo) -> windows_core::Result<EVENTID>;
    fn RegisterPattern(&self, pattern: *const UIAutomationPatternInfo, ppatternid: *mut PATTERNID, ppatternavailablepropertyid: *mut PROPERTYID, propertyidcount: u32, ppropertyids: *mut PROPERTYID, eventidcount: u32, peventids: *mut EVENTID) -> windows_core::Result<()>;
}
impl IUIAutomationRegistrar_Vtbl {
    pub const fn new<Identity: IUIAutomationRegistrar_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterProperty<Identity: IUIAutomationRegistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: *const UIAutomationPropertyInfo, propertyid: *mut PROPERTYID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationRegistrar_Impl::RegisterProperty(this, core::mem::transmute_copy(&property)) {
                    Ok(ok__) => {
                        propertyid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterEvent<Identity: IUIAutomationRegistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, event: *const UIAutomationEventInfo, eventid: *mut EVENTID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationRegistrar_Impl::RegisterEvent(this, core::mem::transmute_copy(&event)) {
                    Ok(ok__) => {
                        eventid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn RegisterPattern<Identity: IUIAutomationRegistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattern: *const UIAutomationPatternInfo, ppatternid: *mut PATTERNID, ppatternavailablepropertyid: *mut PROPERTYID, propertyidcount: u32, ppropertyids: *mut PROPERTYID, eventidcount: u32, peventids: *mut EVENTID) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationRegistrar_Impl::RegisterPattern(this, core::mem::transmute_copy(&pattern), core::mem::transmute_copy(&ppatternid), core::mem::transmute_copy(&ppatternavailablepropertyid), core::mem::transmute_copy(&propertyidcount), core::mem::transmute_copy(&ppropertyids), core::mem::transmute_copy(&eventidcount), core::mem::transmute_copy(&peventids)).into()
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            RegisterProperty: RegisterProperty::<Identity, OFFSET>,
            RegisterEvent: RegisterEvent::<Identity, OFFSET>,
            RegisterPattern: RegisterPattern::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationRegistrar as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IUIAutomationRegistrar {}
windows_core::imp::define_interface!(IValueProvider, IValueProvider_Vtbl, 0xc7935180_6fb3_4201_b174_7df73adbf64a);
windows_core::imp::interface_hierarchy!(IValueProvider, windows_core::IUnknown);
impl IValueProvider {
    pub unsafe fn SetValue<P0>(&self, val: P0) -> windows_core::HRESULT
    where
        P0: windows_core::Param<windows_core::PCWSTR>,
    {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), val.param().abi()) }
    }
    pub unsafe fn Value(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn IsReadOnly(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IValueProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IValueProvider_Impl: windows_core::IUnknownImpl {
    fn SetValue(&self, val: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn Value(&self) -> windows_core::Result<windows_core::BSTR>;
    fn IsReadOnly(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IValueProvider_Vtbl {
    pub const fn new<Identity: IValueProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetValue<Identity: IValueProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueProvider_Impl::SetValue(this, core::mem::transmute(&val)).into()
            }
        }
        unsafe extern "system" fn Value<Identity: IValueProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueProvider_Impl::Value(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsReadOnly<Identity: IValueProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueProvider_Impl::IsReadOnly(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetValue: SetValue::<Identity, OFFSET>,
            Value: Value::<Identity, OFFSET>,
            IsReadOnly: IsReadOnly::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IValueProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IValueProvider {}
windows_core::imp::define_interface!(IVirtualizedItemProvider, IVirtualizedItemProvider_Vtbl, 0xcb98b665_2d35_4fac_ad35_f3c60d0c0b8b);
windows_core::imp::interface_hierarchy!(IVirtualizedItemProvider, windows_core::IUnknown);
impl IVirtualizedItemProvider {
    pub unsafe fn Realize(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Realize)(windows_core::Interface::as_raw(self)) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IVirtualizedItemProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Realize: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IVirtualizedItemProvider_Impl: windows_core::IUnknownImpl {
    fn Realize(&self) -> windows_core::Result<()>;
}
impl IVirtualizedItemProvider_Vtbl {
    pub const fn new<Identity: IVirtualizedItemProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Realize<Identity: IVirtualizedItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IVirtualizedItemProvider_Impl::Realize(this).into()
            }
        }
        Self { base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), Realize: Realize::<Identity, OFFSET> }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IVirtualizedItemProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IVirtualizedItemProvider {}
windows_core::imp::define_interface!(IWindowProvider, IWindowProvider_Vtbl, 0x987df77b_db06_4d77_8f8a_86a9c3bb90b9);
windows_core::imp::interface_hierarchy!(IWindowProvider, windows_core::IUnknown);
impl IWindowProvider {
    pub unsafe fn SetVisualState(&self, state: WindowVisualState) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetVisualState)(windows_core::Interface::as_raw(self), state) }
    }
    pub unsafe fn Close(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Close)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn WaitForInputIdle(&self, milliseconds: i32) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WaitForInputIdle)(windows_core::Interface::as_raw(self), milliseconds, &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CanMaximize(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanMaximize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn CanMinimize(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).CanMinimize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsModal(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsModal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn WindowVisualState(&self) -> windows_core::Result<WindowVisualState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WindowVisualState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn WindowInteractionState(&self) -> windows_core::Result<WindowInteractionState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).WindowInteractionState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn IsTopmost(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).IsTopmost)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IWindowProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetVisualState: unsafe extern "system" fn(*mut core::ffi::c_void, WindowVisualState) -> windows_core::HRESULT,
    pub Close: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub WaitForInputIdle: unsafe extern "system" fn(*mut core::ffi::c_void, i32, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CanMaximize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CanMinimize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub IsModal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub WindowVisualState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowVisualState) -> windows_core::HRESULT,
    pub WindowInteractionState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowInteractionState) -> windows_core::HRESULT,
    pub IsTopmost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IWindowProvider_Impl: windows_core::IUnknownImpl {
    fn SetVisualState(&self, state: WindowVisualState) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn WaitForInputIdle(&self, milliseconds: i32) -> windows_core::Result<windows_core::BOOL>;
    fn CanMaximize(&self) -> windows_core::Result<windows_core::BOOL>;
    fn CanMinimize(&self) -> windows_core::Result<windows_core::BOOL>;
    fn IsModal(&self) -> windows_core::Result<windows_core::BOOL>;
    fn WindowVisualState(&self) -> windows_core::Result<WindowVisualState>;
    fn WindowInteractionState(&self) -> windows_core::Result<WindowInteractionState>;
    fn IsTopmost(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IWindowProvider_Vtbl {
    pub const fn new<Identity: IWindowProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetVisualState<Identity: IWindowProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, state: WindowVisualState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWindowProvider_Impl::SetVisualState(this, core::mem::transmute_copy(&state)).into()
            }
        }
        unsafe extern "system" fn Close<Identity: IWindowProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IWindowProvider_Impl::Close(this).into()
            }
        }
        unsafe extern "system" fn WaitForInputIdle<Identity: IWindowProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, milliseconds: i32, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowProvider_Impl::WaitForInputIdle(this, core::mem::transmute_copy(&milliseconds)) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanMaximize<Identity: IWindowProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowProvider_Impl::CanMaximize(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn CanMinimize<Identity: IWindowProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowProvider_Impl::CanMinimize(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsModal<Identity: IWindowProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowProvider_Impl::IsModal(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WindowVisualState<Identity: IWindowProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut WindowVisualState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowProvider_Impl::WindowVisualState(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn WindowInteractionState<Identity: IWindowProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut WindowInteractionState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowProvider_Impl::WindowInteractionState(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn IsTopmost<Identity: IWindowProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowProvider_Impl::IsTopmost(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        Self {
            base__: windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            SetVisualState: SetVisualState::<Identity, OFFSET>,
            Close: Close::<Identity, OFFSET>,
            WaitForInputIdle: WaitForInputIdle::<Identity, OFFSET>,
            CanMaximize: CanMaximize::<Identity, OFFSET>,
            CanMinimize: CanMinimize::<Identity, OFFSET>,
            IsModal: IsModal::<Identity, OFFSET>,
            WindowVisualState: WindowVisualState::<Identity, OFFSET>,
            WindowInteractionState: WindowInteractionState::<Identity, OFFSET>,
            IsTopmost: IsTopmost::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWindowProvider {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct LANDMARKTYPEID(pub i32);
pub type LiveSetting = i32;
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct METADATAID(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PATTERNID(pub i32);
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct PROPERTYID(pub i32);
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
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct TEXTATTRIBUTEID(pub i32);
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
pub const UIA_ScrollPatternNoScroll: f64 = -1.0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UIAutomationEventInfo {
    pub guid: windows_core::GUID,
    pub pProgrammaticName: windows_core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UIAutomationMethodInfo {
    pub pProgrammaticName: windows_core::PCWSTR,
    pub doSetFocus: windows_core::BOOL,
    pub cInParameters: u32,
    pub cOutParameters: u32,
    pub pParameterTypes: *mut UIAutomationType,
    pub pParameterNames: *mut windows_core::PCWSTR,
}
impl Default for UIAutomationMethodInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
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
#[derive(Clone, Debug, PartialEq)]
pub struct UIAutomationPatternInfo {
    pub guid: windows_core::GUID,
    pub pProgrammaticName: windows_core::PCWSTR,
    pub providerInterfaceId: windows_core::GUID,
    pub clientInterfaceId: windows_core::GUID,
    pub cProperties: u32,
    pub pProperties: *mut UIAutomationPropertyInfo,
    pub cMethods: u32,
    pub pMethods: *mut UIAutomationMethodInfo,
    pub cEvents: u32,
    pub pEvents: *mut UIAutomationEventInfo,
    pub pPatternHandler: core::mem::ManuallyDrop<Option<IUIAutomationPatternHandler>>,
}
impl Default for UIAutomationPatternInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UIAutomationPropertyInfo {
    pub guid: windows_core::GUID,
    pub pProgrammaticName: windows_core::PCWSTR,
    pub r#type: UIAutomationType,
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
pub struct UiaChangeInfo {
    pub uiaId: i32,
    pub payload: super::oaidl::VARIANT,
    pub extraInfo: super::oaidl::VARIANT,
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for UiaChangeInfo {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "oaidl", feature = "wtypes", feature = "wtypesbase"))]
impl Default for UiaChangeInfo {
    fn default() -> Self {
        unsafe { core::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct UiaPoint {
    pub x: f64,
    pub y: f64,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
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
