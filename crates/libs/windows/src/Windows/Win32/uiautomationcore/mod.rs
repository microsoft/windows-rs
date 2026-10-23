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
windows_core::imp::define_interface!(IAccessibleEx, IAccessibleEx_Vtbl, 0xf8b80ada_2c44_48d0_89be_5ff23c9cd875);
windows_core::imp::interface_hierarchy!(IAccessibleEx, windows_core::IUnknown);
impl IAccessibleEx {
    pub unsafe fn GetObjectForChild(&self, idchild: i32) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetObjectForChild)(windows_core::Interface::as_raw(self), idchild, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "oleacc"))]
    pub unsafe fn GetIAccessiblePair(&self, ppacc: *mut Option<super::IAccessible>, pidchild: *mut i32) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetIAccessiblePair)(windows_core::Interface::as_raw(self), core::mem::transmute(ppacc), pidchild as _) }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetRuntimeId(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
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
            (windows_core::Interface::vtable(self).ConvertReturnedElement)(windows_core::Interface::as_raw(self), pin.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetRuntimeId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetRuntimeId: usize,
    pub ConvertReturnedElement: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "shared", feature = "winnt"))]
pub trait IAccessibleEx_Impl: windows_core::IUnknownImpl {
    fn GetObjectForChild(&self, idchild: i32) -> windows_core::Result<IAccessibleEx>;
    fn GetIAccessiblePair(&self, ppacc: windows_core::OutRef<super::IAccessible>, pidchild: *mut i32) -> windows_core::Result<()>;
    fn GetRuntimeId(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn ConvertReturnedElement(&self, pin: windows_core::Ref<IRawElementProviderSimple>) -> windows_core::Result<IAccessibleEx>;
}
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "shared", feature = "winnt"))]
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
        unsafe extern "system" fn GetRuntimeId<Identity: IAccessibleEx_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
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
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IAccessibleEx {}
windows_core::imp::define_interface!(IAccessibleHostingElementProviders, IAccessibleHostingElementProviders_Vtbl, 0x33ac331b_943e_4020_b295_db37784974a3);
windows_core::imp::interface_hierarchy!(IAccessibleHostingElementProviders, windows_core::IUnknown);
impl IAccessibleHostingElementProviders {
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetEmbeddedFragmentRoots(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
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
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetEmbeddedFragmentRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetEmbeddedFragmentRoots: usize,
    pub GetObjectIdForProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
pub trait IAccessibleHostingElementProviders_Impl: windows_core::IUnknownImpl {
    fn GetEmbeddedFragmentRoots(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn GetObjectIdForProvider(&self, pprovider: windows_core::Ref<IRawElementProviderSimple>) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl IAccessibleHostingElementProviders_Vtbl {
    pub const fn new<Identity: IAccessibleHostingElementProviders_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEmbeddedFragmentRoots<Identity: IAccessibleHostingElementProviders_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
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
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IAccessibleHostingElementProviders {}
windows_core::imp::define_interface!(IAnnotationProvider, IAnnotationProvider_Vtbl, 0xf95c7e80_bd63_4601_9782_445ebff011fc);
windows_core::imp::interface_hierarchy!(IAnnotationProvider, windows_core::IUnknown);
impl IAnnotationProvider {
    pub unsafe fn get_AnnotationTypeId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AnnotationTypeId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_AnnotationTypeName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_AnnotationTypeName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Author(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Author)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_DateTime(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DateTime)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Target(&self) -> windows_core::Result<IRawElementProviderSimple> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Target)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IAnnotationProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_AnnotationTypeId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_AnnotationTypeName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Author: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DateTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Target: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IAnnotationProvider_Impl: windows_core::IUnknownImpl {
    fn get_AnnotationTypeId(&self) -> windows_core::Result<i32>;
    fn get_AnnotationTypeName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Author(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_DateTime(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Target(&self) -> windows_core::Result<IRawElementProviderSimple>;
}
impl IAnnotationProvider_Vtbl {
    pub const fn new<Identity: IAnnotationProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_AnnotationTypeId<Identity: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAnnotationProvider_Impl::get_AnnotationTypeId(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_AnnotationTypeName<Identity: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAnnotationProvider_Impl::get_AnnotationTypeName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Author<Identity: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAnnotationProvider_Impl::get_Author(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DateTime<Identity: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAnnotationProvider_Impl::get_DateTime(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Target<Identity: IAnnotationProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IAnnotationProvider_Impl::get_Target(this) {
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
            get_AnnotationTypeId: get_AnnotationTypeId::<Identity, OFFSET>,
            get_AnnotationTypeName: get_AnnotationTypeName::<Identity, OFFSET>,
            get_Author: get_Author::<Identity, OFFSET>,
            get_DateTime: get_DateTime::<Identity, OFFSET>,
            get_Target: get_Target::<Identity, OFFSET>,
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
            (windows_core::Interface::vtable(self).Navigate)(windows_core::Interface::as_raw(self), direction, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
    pub unsafe fn get_DockPosition(&self) -> windows_core::Result<DockPosition> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DockPosition)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDockProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetDockPosition: unsafe extern "system" fn(*mut core::ffi::c_void, DockPosition) -> windows_core::HRESULT,
    pub get_DockPosition: unsafe extern "system" fn(*mut core::ffi::c_void, *mut DockPosition) -> windows_core::HRESULT,
}
pub trait IDockProvider_Impl: windows_core::IUnknownImpl {
    fn SetDockPosition(&self, dockposition: DockPosition) -> windows_core::Result<()>;
    fn get_DockPosition(&self) -> windows_core::Result<DockPosition>;
}
impl IDockProvider_Vtbl {
    pub const fn new<Identity: IDockProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetDockPosition<Identity: IDockProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, dockposition: DockPosition) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IDockProvider_Impl::SetDockPosition(this, core::mem::transmute_copy(&dockposition)).into()
            }
        }
        unsafe extern "system" fn get_DockPosition<Identity: IDockProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut DockPosition) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDockProvider_Impl::get_DockPosition(this) {
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
            get_DockPosition: get_DockPosition::<Identity, OFFSET>,
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
    pub unsafe fn get_IsGrabbed(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsGrabbed)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_DropEffect(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DropEffect)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn get_DropEffects(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DropEffects)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetGrabbedItems(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
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
    pub get_IsGrabbed: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_DropEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub get_DropEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    get_DropEffects: usize,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetGrabbedItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetGrabbedItems: usize,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
pub trait IDragProvider_Impl: windows_core::IUnknownImpl {
    fn get_IsGrabbed(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_DropEffect(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_DropEffects(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn GetGrabbedItems(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl IDragProvider_Vtbl {
    pub const fn new<Identity: IDragProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_IsGrabbed<Identity: IDragProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDragProvider_Impl::get_IsGrabbed(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DropEffect<Identity: IDragProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDragProvider_Impl::get_DropEffect(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DropEffects<Identity: IDragProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDragProvider_Impl::get_DropEffects(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetGrabbedItems<Identity: IDragProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
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
            get_IsGrabbed: get_IsGrabbed::<Identity, OFFSET>,
            get_DropEffect: get_DropEffect::<Identity, OFFSET>,
            get_DropEffects: get_DropEffects::<Identity, OFFSET>,
            GetGrabbedItems: GetGrabbedItems::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDragProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IDragProvider {}
windows_core::imp::define_interface!(IDropTargetProvider, IDropTargetProvider_Vtbl, 0xbae82bfd_358a_481c_85a0_d8b4d90a5d61);
windows_core::imp::interface_hierarchy!(IDropTargetProvider, windows_core::IUnknown);
impl IDropTargetProvider {
    pub unsafe fn get_DropTargetEffect(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DropTargetEffect)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn get_DropTargetEffects(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DropTargetEffects)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IDropTargetProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_DropTargetEffect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub get_DropTargetEffects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    get_DropTargetEffects: usize,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
pub trait IDropTargetProvider_Impl: windows_core::IUnknownImpl {
    fn get_DropTargetEffect(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_DropTargetEffects(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl IDropTargetProvider_Vtbl {
    pub const fn new<Identity: IDropTargetProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_DropTargetEffect<Identity: IDropTargetProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDropTargetProvider_Impl::get_DropTargetEffect(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_DropTargetEffects<Identity: IDropTargetProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IDropTargetProvider_Impl::get_DropTargetEffects(this) {
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
            get_DropTargetEffect: get_DropTargetEffect::<Identity, OFFSET>,
            get_DropTargetEffects: get_DropTargetEffects::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IDropTargetProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
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
    pub unsafe fn get_ExpandCollapseState(&self) -> windows_core::Result<ExpandCollapseState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ExpandCollapseState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IExpandCollapseProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Expand: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Collapse: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ExpandCollapseState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ExpandCollapseState) -> windows_core::HRESULT,
}
pub trait IExpandCollapseProvider_Impl: windows_core::IUnknownImpl {
    fn Expand(&self) -> windows_core::Result<()>;
    fn Collapse(&self) -> windows_core::Result<()>;
    fn get_ExpandCollapseState(&self) -> windows_core::Result<ExpandCollapseState>;
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
        unsafe extern "system" fn get_ExpandCollapseState<Identity: IExpandCollapseProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut ExpandCollapseState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IExpandCollapseProvider_Impl::get_ExpandCollapseState(this) {
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
            get_ExpandCollapseState: get_ExpandCollapseState::<Identity, OFFSET>,
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
    pub unsafe fn get_Row(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Row)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Column(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Column)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_RowSpan(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RowSpan)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ColumnSpan(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ColumnSpan)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ContainingGrid(&self) -> windows_core::Result<IRawElementProviderSimple> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ContainingGrid)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridItemProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_Row: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_Column: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_RowSpan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_ColumnSpan: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_ContainingGrid: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IGridItemProvider_Impl: windows_core::IUnknownImpl {
    fn get_Row(&self) -> windows_core::Result<i32>;
    fn get_Column(&self) -> windows_core::Result<i32>;
    fn get_RowSpan(&self) -> windows_core::Result<i32>;
    fn get_ColumnSpan(&self) -> windows_core::Result<i32>;
    fn get_ContainingGrid(&self) -> windows_core::Result<IRawElementProviderSimple>;
}
impl IGridItemProvider_Vtbl {
    pub const fn new<Identity: IGridItemProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Row<Identity: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGridItemProvider_Impl::get_Row(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Column<Identity: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGridItemProvider_Impl::get_Column(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_RowSpan<Identity: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGridItemProvider_Impl::get_RowSpan(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ColumnSpan<Identity: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGridItemProvider_Impl::get_ColumnSpan(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ContainingGrid<Identity: IGridItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGridItemProvider_Impl::get_ContainingGrid(this) {
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
            get_Row: get_Row::<Identity, OFFSET>,
            get_Column: get_Column::<Identity, OFFSET>,
            get_RowSpan: get_RowSpan::<Identity, OFFSET>,
            get_ColumnSpan: get_ColumnSpan::<Identity, OFFSET>,
            get_ContainingGrid: get_ContainingGrid::<Identity, OFFSET>,
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
            (windows_core::Interface::vtable(self).GetItem)(windows_core::Interface::as_raw(self), row, column, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_RowCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RowCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ColumnCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ColumnCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IGridProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub GetItem: unsafe extern "system" fn(*mut core::ffi::c_void, i32, i32, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_RowCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_ColumnCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
pub trait IGridProvider_Impl: windows_core::IUnknownImpl {
    fn GetItem(&self, row: i32, column: i32) -> windows_core::Result<IRawElementProviderSimple>;
    fn get_RowCount(&self) -> windows_core::Result<i32>;
    fn get_ColumnCount(&self) -> windows_core::Result<i32>;
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
        unsafe extern "system" fn get_RowCount<Identity: IGridProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGridProvider_Impl::get_RowCount(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ColumnCount<Identity: IGridProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IGridProvider_Impl::get_ColumnCount(this) {
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
            get_RowCount: get_RowCount::<Identity, OFFSET>,
            get_ColumnCount: get_ColumnCount::<Identity, OFFSET>,
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
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn FindItemByProperty<P0>(&self, pstartafter: P0, propertyid: super::PROPERTYID, value: &super::VARIANT) -> windows_core::Result<IRawElementProviderSimple>
    where
        P0: windows_core::Param<IRawElementProviderSimple>,
    {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindItemByProperty)(windows_core::Interface::as_raw(self), pstartafter.param().abi(), propertyid, core::mem::transmute_copy(value), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IItemContainerProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub FindItemByProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::PROPERTYID, super::VARIANT, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    FindItemByProperty: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IItemContainerProvider_Impl: windows_core::IUnknownImpl {
    fn FindItemByProperty(&self, pstartafter: windows_core::Ref<IRawElementProviderSimple>, propertyid: super::PROPERTYID, value: &super::VARIANT) -> windows_core::Result<IRawElementProviderSimple>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IItemContainerProvider_Vtbl {
    pub const fn new<Identity: IItemContainerProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn FindItemByProperty<Identity: IItemContainerProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pstartafter: *mut core::ffi::c_void, propertyid: super::PROPERTYID, value: super::VARIANT, pfound: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    pub unsafe fn GetIAccessible(&self) -> windows_core::Result<super::IAccessible> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetIAccessible)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_ChildId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ChildId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Name(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Name)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Value(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Description(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Description)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_Role(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Role)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn get_State(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_State)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Help(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Help)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_KeyboardShortcut(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_KeyboardShortcut)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetSelection(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_DefaultAction(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DefaultAction)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
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
    pub get_ChildId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_Name: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Description: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub get_Role: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_Role: usize,
    #[cfg(feature = "shared")]
    pub get_State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_State: usize,
    pub get_Help: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_KeyboardShortcut: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetSelection: usize,
    pub get_DefaultAction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "shared", feature = "winnt"))]
pub trait ILegacyIAccessibleProvider_Impl: windows_core::IUnknownImpl {
    fn Select(&self, flagsselect: i32) -> windows_core::Result<()>;
    fn DoDefaultAction(&self) -> windows_core::Result<()>;
    fn SetValue(&self, szvalue: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn GetIAccessible(&self) -> windows_core::Result<super::IAccessible>;
    fn get_ChildId(&self) -> windows_core::Result<i32>;
    fn get_Name(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Value(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Description(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Role(&self) -> windows_core::Result<super::DWORD>;
    fn get_State(&self) -> windows_core::Result<super::DWORD>;
    fn get_Help(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_KeyboardShortcut(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetSelection(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn get_DefaultAction(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "shared", feature = "winnt"))]
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
        unsafe extern "system" fn get_ChildId<Identity: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILegacyIAccessibleProvider_Impl::get_ChildId(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Name<Identity: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILegacyIAccessibleProvider_Impl::get_Name(this) {
                    Ok(ok__) => {
                        pszname.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Value<Identity: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszvalue: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILegacyIAccessibleProvider_Impl::get_Value(this) {
                    Ok(ok__) => {
                        pszvalue.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Description<Identity: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdescription: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILegacyIAccessibleProvider_Impl::get_Description(this) {
                    Ok(ok__) => {
                        pszdescription.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Role<Identity: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwrole: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILegacyIAccessibleProvider_Impl::get_Role(this) {
                    Ok(ok__) => {
                        pdwrole.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_State<Identity: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pdwstate: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILegacyIAccessibleProvider_Impl::get_State(this) {
                    Ok(ok__) => {
                        pdwstate.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Help<Identity: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszhelp: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILegacyIAccessibleProvider_Impl::get_Help(this) {
                    Ok(ok__) => {
                        pszhelp.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_KeyboardShortcut<Identity: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszkeyboardshortcut: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILegacyIAccessibleProvider_Impl::get_KeyboardShortcut(this) {
                    Ok(ok__) => {
                        pszkeyboardshortcut.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSelection<Identity: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pvarselectedchildren: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
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
        unsafe extern "system" fn get_DefaultAction<Identity: ILegacyIAccessibleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pszdefaultaction: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ILegacyIAccessibleProvider_Impl::get_DefaultAction(this) {
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
            get_ChildId: get_ChildId::<Identity, OFFSET>,
            get_Name: get_Name::<Identity, OFFSET>,
            get_Value: get_Value::<Identity, OFFSET>,
            get_Description: get_Description::<Identity, OFFSET>,
            get_Role: get_Role::<Identity, OFFSET>,
            get_State: get_State::<Identity, OFFSET>,
            get_Help: get_Help::<Identity, OFFSET>,
            get_KeyboardShortcut: get_KeyboardShortcut::<Identity, OFFSET>,
            GetSelection: GetSelection::<Identity, OFFSET>,
            get_DefaultAction: get_DefaultAction::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ILegacyIAccessibleProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "oleacc", feature = "shared", feature = "winnt"))]
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
    pub unsafe fn get_CurrentView(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentView)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetSupportedViews(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
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
    pub get_CurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetSupportedViews: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetSupportedViews: usize,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
pub trait IMultipleViewProvider_Impl: windows_core::IUnknownImpl {
    fn GetViewName(&self, viewid: i32) -> windows_core::Result<windows_core::BSTR>;
    fn SetCurrentView(&self, viewid: i32) -> windows_core::Result<()>;
    fn get_CurrentView(&self) -> windows_core::Result<i32>;
    fn GetSupportedViews(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
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
        unsafe extern "system" fn get_CurrentView<Identity: IMultipleViewProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IMultipleViewProvider_Impl::get_CurrentView(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetSupportedViews<Identity: IMultipleViewProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
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
            get_CurrentView: get_CurrentView::<Identity, OFFSET>,
            GetSupportedViews: GetSupportedViews::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IMultipleViewProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IMultipleViewProvider {}
windows_core::imp::define_interface!(IObjectModelProvider, IObjectModelProvider_Vtbl, 0x3ad86ebd_f5ef_483d_bb18_b1042a475d64);
windows_core::imp::interface_hierarchy!(IObjectModelProvider, windows_core::IUnknown);
impl IObjectModelProvider {
    pub unsafe fn GetUnderlyingObjectModel(&self) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetUnderlyingObjectModel)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub unsafe fn RespondToWinEvent<P4>(&self, idwinevent: super::DWORD, hwnd: super::HWND, idobject: super::LONG, idchild: super::LONG, psink: P4) -> windows_core::HRESULT
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
    #[cfg(all(feature = "shared", feature = "windef"))]
    pub RespondToWinEvent: unsafe extern "system" fn(*mut core::ffi::c_void, super::DWORD, super::HWND, super::LONG, super::LONG, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "shared", feature = "windef")))]
    RespondToWinEvent: usize,
}
#[cfg(all(feature = "shared", feature = "windef"))]
pub trait IProxyProviderWinEventHandler_Impl: windows_core::IUnknownImpl {
    fn RespondToWinEvent(&self, idwinevent: super::DWORD, hwnd: super::HWND, idobject: super::LONG, idchild: super::LONG, psink: windows_core::Ref<IProxyProviderWinEventSink>) -> windows_core::Result<()>;
}
#[cfg(all(feature = "shared", feature = "windef"))]
impl IProxyProviderWinEventHandler_Vtbl {
    pub const fn new<Identity: IProxyProviderWinEventHandler_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RespondToWinEvent<Identity: IProxyProviderWinEventHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, idwinevent: super::DWORD, hwnd: super::HWND, idobject: super::LONG, idchild: super::LONG, psink: *mut core::ffi::c_void) -> windows_core::HRESULT {
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
#[cfg(all(feature = "shared", feature = "windef"))]
impl windows_core::RuntimeName for IProxyProviderWinEventHandler {}
windows_core::imp::define_interface!(IProxyProviderWinEventSink, IProxyProviderWinEventSink_Vtbl, 0x4fd82b78_a43e_46ac_9803_0a6969c7c183);
windows_core::imp::interface_hierarchy!(IProxyProviderWinEventSink, windows_core::IUnknown);
impl IProxyProviderWinEventSink {
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn AddAutomationPropertyChangedEvent<P0>(&self, pprovider: P0, id: super::PROPERTYID, newvalue: &super::VARIANT) -> windows_core::HRESULT
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
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn AddStructureChangedEvent<P0>(&self, pprovider: P0, structurechangetype: StructureChangeType, runtimeid: *const super::SAFEARRAY) -> windows_core::HRESULT
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
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub AddAutomationPropertyChangedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::PROPERTYID, super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    AddAutomationPropertyChangedEvent: usize,
    pub AddAutomationEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, EVENTID) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub AddStructureChangedEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, StructureChangeType, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    AddStructureChangedEvent: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IProxyProviderWinEventSink_Impl: windows_core::IUnknownImpl {
    fn AddAutomationPropertyChangedEvent(&self, pprovider: windows_core::Ref<IRawElementProviderSimple>, id: super::PROPERTYID, newvalue: &super::VARIANT) -> windows_core::Result<()>;
    fn AddAutomationEvent(&self, pprovider: windows_core::Ref<IRawElementProviderSimple>, id: EVENTID) -> windows_core::Result<()>;
    fn AddStructureChangedEvent(&self, pprovider: windows_core::Ref<IRawElementProviderSimple>, structurechangetype: StructureChangeType, runtimeid: *const super::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IProxyProviderWinEventSink_Vtbl {
    pub const fn new<Identity: IProxyProviderWinEventSink_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AddAutomationPropertyChangedEvent<Identity: IProxyProviderWinEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprovider: *mut core::ffi::c_void, id: super::PROPERTYID, newvalue: super::VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn AddStructureChangedEvent<Identity: IProxyProviderWinEventSink_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pprovider: *mut core::ffi::c_void, structurechangetype: StructureChangeType, runtimeid: *const super::SAFEARRAY) -> windows_core::HRESULT {
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
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IProxyProviderWinEventSink {}
windows_core::imp::define_interface!(IRangeValueProvider, IRangeValueProvider_Vtbl, 0x36dc7aef_33e6_4691_afe1_2be7274b3d33);
windows_core::imp::interface_hierarchy!(IRangeValueProvider, windows_core::IUnknown);
impl IRangeValueProvider {
    pub unsafe fn SetValue(&self, val: f64) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetValue)(windows_core::Interface::as_raw(self), val) }
    }
    pub unsafe fn get_Value(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_IsReadOnly(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Maximum(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Maximum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_Minimum(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Minimum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_LargeChange(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LargeChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_SmallChange(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SmallChange)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRangeValueProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
    pub get_Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_IsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_Maximum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_Minimum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_LargeChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_SmallChange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
}
pub trait IRangeValueProvider_Impl: windows_core::IUnknownImpl {
    fn SetValue(&self, val: f64) -> windows_core::Result<()>;
    fn get_Value(&self) -> windows_core::Result<f64>;
    fn get_IsReadOnly(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_Maximum(&self) -> windows_core::Result<f64>;
    fn get_Minimum(&self) -> windows_core::Result<f64>;
    fn get_LargeChange(&self) -> windows_core::Result<f64>;
    fn get_SmallChange(&self) -> windows_core::Result<f64>;
}
impl IRangeValueProvider_Vtbl {
    pub const fn new<Identity: IRangeValueProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetValue<Identity: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRangeValueProvider_Impl::SetValue(this, core::mem::transmute_copy(&val)).into()
            }
        }
        unsafe extern "system" fn get_Value<Identity: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRangeValueProvider_Impl::get_Value(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsReadOnly<Identity: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRangeValueProvider_Impl::get_IsReadOnly(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Maximum<Identity: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRangeValueProvider_Impl::get_Maximum(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Minimum<Identity: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRangeValueProvider_Impl::get_Minimum(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_LargeChange<Identity: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRangeValueProvider_Impl::get_LargeChange(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SmallChange<Identity: IRangeValueProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRangeValueProvider_Impl::get_SmallChange(this) {
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
            get_Value: get_Value::<Identity, OFFSET>,
            get_IsReadOnly: get_IsReadOnly::<Identity, OFFSET>,
            get_Maximum: get_Maximum::<Identity, OFFSET>,
            get_Minimum: get_Minimum::<Identity, OFFSET>,
            get_LargeChange: get_LargeChange::<Identity, OFFSET>,
            get_SmallChange: get_SmallChange::<Identity, OFFSET>,
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
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn AdviseEventAdded(&self, eventid: EVENTID, propertyids: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AdviseEventAdded)(windows_core::Interface::as_raw(self), eventid, propertyids) }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn AdviseEventRemoved(&self, eventid: EVENTID, propertyids: *const super::SAFEARRAY) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).AdviseEventRemoved)(windows_core::Interface::as_raw(self), eventid, propertyids) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawElementProviderAdviseEvents_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub AdviseEventAdded: unsafe extern "system" fn(*mut core::ffi::c_void, EVENTID, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    AdviseEventAdded: usize,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub AdviseEventRemoved: unsafe extern "system" fn(*mut core::ffi::c_void, EVENTID, *const super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    AdviseEventRemoved: usize,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
pub trait IRawElementProviderAdviseEvents_Impl: windows_core::IUnknownImpl {
    fn AdviseEventAdded(&self, eventid: EVENTID, propertyids: *const super::SAFEARRAY) -> windows_core::Result<()>;
    fn AdviseEventRemoved(&self, eventid: EVENTID, propertyids: *const super::SAFEARRAY) -> windows_core::Result<()>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl IRawElementProviderAdviseEvents_Vtbl {
    pub const fn new<Identity: IRawElementProviderAdviseEvents_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn AdviseEventAdded<Identity: IRawElementProviderAdviseEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: EVENTID, propertyids: *const super::SAFEARRAY) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IRawElementProviderAdviseEvents_Impl::AdviseEventAdded(this, core::mem::transmute_copy(&eventid), core::mem::transmute_copy(&propertyids)).into()
            }
        }
        unsafe extern "system" fn AdviseEventRemoved<Identity: IRawElementProviderAdviseEvents_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, eventid: EVENTID, propertyids: *const super::SAFEARRAY) -> windows_core::HRESULT {
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
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IRawElementProviderAdviseEvents {}
windows_core::imp::define_interface!(IRawElementProviderFragment, IRawElementProviderFragment_Vtbl, 0xf7063da8_8359_439c_9297_bbc5299a7d87);
windows_core::imp::interface_hierarchy!(IRawElementProviderFragment, windows_core::IUnknown);
impl IRawElementProviderFragment {
    pub unsafe fn Navigate(&self, direction: NavigateDirection) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).Navigate)(windows_core::Interface::as_raw(self), direction, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetRuntimeId(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
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
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetEmbeddedFragmentRoots(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetEmbeddedFragmentRoots)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn SetFocus(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).SetFocus)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn get_FragmentRoot(&self) -> windows_core::Result<IRawElementProviderFragmentRoot> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FragmentRoot)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawElementProviderFragment_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Navigate: unsafe extern "system" fn(*mut core::ffi::c_void, NavigateDirection, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetRuntimeId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetRuntimeId: usize,
    pub get_BoundingRectangle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut UiaRect) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetEmbeddedFragmentRoots: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetEmbeddedFragmentRoots: usize,
    pub SetFocus: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_FragmentRoot: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
pub trait IRawElementProviderFragment_Impl: windows_core::IUnknownImpl {
    fn Navigate(&self, direction: NavigateDirection) -> windows_core::Result<IRawElementProviderFragment>;
    fn GetRuntimeId(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn get_BoundingRectangle(&self) -> windows_core::Result<UiaRect>;
    fn GetEmbeddedFragmentRoots(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn SetFocus(&self) -> windows_core::Result<()>;
    fn get_FragmentRoot(&self) -> windows_core::Result<IRawElementProviderFragmentRoot>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
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
        unsafe extern "system" fn GetRuntimeId<Identity: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetEmbeddedFragmentRoots<Identity: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
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
        unsafe extern "system" fn get_FragmentRoot<Identity: IRawElementProviderFragment_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawElementProviderFragment_Impl::get_FragmentRoot(this) {
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
            get_FragmentRoot: get_FragmentRoot::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRawElementProviderFragment as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IRawElementProviderFragment {}
windows_core::imp::define_interface!(IRawElementProviderFragmentRoot, IRawElementProviderFragmentRoot_Vtbl, 0x620ce2a5_ab8f_40a9_86cb_de3c75599b58);
windows_core::imp::interface_hierarchy!(IRawElementProviderFragmentRoot, windows_core::IUnknown);
impl IRawElementProviderFragmentRoot {
    pub unsafe fn ElementProviderFromPoint(&self, x: f64, y: f64) -> windows_core::Result<IRawElementProviderFragment> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).ElementProviderFromPoint)(windows_core::Interface::as_raw(self), x, y, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetFocus(&self) -> windows_core::Result<IRawElementProviderFragment> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetFocus)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetEmbeddedAccessibles(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
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
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetEmbeddedAccessibles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetEmbeddedAccessibles: usize,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
pub trait IRawElementProviderHostingAccessibles_Impl: windows_core::IUnknownImpl {
    fn GetEmbeddedAccessibles(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl IRawElementProviderHostingAccessibles_Vtbl {
    pub const fn new<Identity: IRawElementProviderHostingAccessibles_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetEmbeddedAccessibles<Identity: IRawElementProviderHostingAccessibles_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
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
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for IRawElementProviderHostingAccessibles {}
windows_core::imp::define_interface!(IRawElementProviderHwndOverride, IRawElementProviderHwndOverride_Vtbl, 0x1d5df27c_8947_4425_b8d9_79787bb460b8);
windows_core::imp::interface_hierarchy!(IRawElementProviderHwndOverride, windows_core::IUnknown);
impl IRawElementProviderHwndOverride {
    #[cfg(feature = "windef")]
    pub unsafe fn GetOverrideProviderForHwnd(&self, hwnd: super::HWND) -> windows_core::Result<IRawElementProviderSimple> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetOverrideProviderForHwnd)(windows_core::Interface::as_raw(self), hwnd, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawElementProviderHwndOverride_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "windef")]
    pub GetOverrideProviderForHwnd: unsafe extern "system" fn(*mut core::ffi::c_void, super::HWND, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "windef"))]
    GetOverrideProviderForHwnd: usize,
}
#[cfg(feature = "windef")]
pub trait IRawElementProviderHwndOverride_Impl: windows_core::IUnknownImpl {
    fn GetOverrideProviderForHwnd(&self, hwnd: super::HWND) -> windows_core::Result<IRawElementProviderSimple>;
}
#[cfg(feature = "windef")]
impl IRawElementProviderHwndOverride_Vtbl {
    pub const fn new<Identity: IRawElementProviderHwndOverride_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetOverrideProviderForHwnd<Identity: IRawElementProviderHwndOverride_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, hwnd: super::HWND, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
    pub unsafe fn get_ProviderOptions(&self) -> windows_core::Result<ProviderOptions> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ProviderOptions)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetPatternProvider(&self, patternid: PATTERNID) -> windows_core::Result<windows_core::IUnknown> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPatternProvider)(windows_core::Interface::as_raw(self), patternid, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetPropertyValue(&self, propertyid: super::PROPERTYID) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetPropertyValue)(windows_core::Interface::as_raw(self), propertyid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_HostRawElementProvider(&self) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HostRawElementProvider)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IRawElementProviderSimple_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_ProviderOptions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ProviderOptions) -> windows_core::HRESULT,
    pub GetPatternProvider: unsafe extern "system" fn(*mut core::ffi::c_void, PATTERNID, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetPropertyValue: unsafe extern "system" fn(*mut core::ffi::c_void, super::PROPERTYID, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetPropertyValue: usize,
    pub get_HostRawElementProvider: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IRawElementProviderSimple_Impl: windows_core::IUnknownImpl {
    fn get_ProviderOptions(&self) -> windows_core::Result<ProviderOptions>;
    fn GetPatternProvider(&self, patternid: PATTERNID) -> windows_core::Result<windows_core::IUnknown>;
    fn GetPropertyValue(&self, propertyid: super::PROPERTYID) -> windows_core::Result<super::VARIANT>;
    fn get_HostRawElementProvider(&self) -> windows_core::Result<IRawElementProviderSimple>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IRawElementProviderSimple_Vtbl {
    pub const fn new<Identity: IRawElementProviderSimple_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ProviderOptions<Identity: IRawElementProviderSimple_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut ProviderOptions) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawElementProviderSimple_Impl::get_ProviderOptions(this) {
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
        unsafe extern "system" fn GetPropertyValue<Identity: IRawElementProviderSimple_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, propertyid: super::PROPERTYID, pretval: *mut super::VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn get_HostRawElementProvider<Identity: IRawElementProviderSimple_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IRawElementProviderSimple_Impl::get_HostRawElementProvider(this) {
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
            get_ProviderOptions: get_ProviderOptions::<Identity, OFFSET>,
            GetPatternProvider: GetPatternProvider::<Identity, OFFSET>,
            GetPropertyValue: GetPropertyValue::<Identity, OFFSET>,
            get_HostRawElementProvider: get_HostRawElementProvider::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IRawElementProviderSimple as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IRawElementProviderSimple2_Impl: IRawElementProviderSimple_Impl {
    fn ShowContextMenu(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetMetadataValue(&self, targetid: i32, metadataid: METADATAID) -> windows_core::Result<super::VARIANT> {
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
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetMetadataValue: unsafe extern "system" fn(*mut core::ffi::c_void, i32, METADATAID, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetMetadataValue: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait IRawElementProviderSimple3_Impl: IRawElementProviderSimple2_Impl {
    fn GetMetadataValue(&self, targetid: i32, metadataid: METADATAID) -> windows_core::Result<super::VARIANT>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl IRawElementProviderSimple3_Vtbl {
    pub const fn new<Identity: IRawElementProviderSimple3_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetMetadataValue<Identity: IRawElementProviderSimple3_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, targetid: i32, metadataid: METADATAID, returnval: *mut super::VARIANT) -> windows_core::HRESULT {
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
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for IRawElementProviderSimple3 {}
windows_core::imp::define_interface!(IRawElementProviderWindowlessSite, IRawElementProviderWindowlessSite_Vtbl, 0x0a2a93cc_bfad_42ac_9b2e_0991fb0d3ea0);
windows_core::imp::interface_hierarchy!(IRawElementProviderWindowlessSite, windows_core::IUnknown);
impl IRawElementProviderWindowlessSite {
    pub unsafe fn GetAdjacentFragment(&self, direction: NavigateDirection) -> windows_core::Result<IRawElementProviderFragment> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAdjacentFragment)(windows_core::Interface::as_raw(self), direction, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetRuntimeIdPrefix(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
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
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetRuntimeIdPrefix: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetRuntimeIdPrefix: usize,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
pub trait IRawElementProviderWindowlessSite_Impl: windows_core::IUnknownImpl {
    fn GetAdjacentFragment(&self, direction: NavigateDirection) -> windows_core::Result<IRawElementProviderFragment>;
    fn GetRuntimeIdPrefix(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
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
        unsafe extern "system" fn GetRuntimeIdPrefix<Identity: IRawElementProviderWindowlessSite_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
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
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
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
    pub unsafe fn get_HorizontalScrollPercent(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HorizontalScrollPercent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_VerticalScrollPercent(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_VerticalScrollPercent)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_HorizontalViewSize(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HorizontalViewSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_VerticalViewSize(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_VerticalViewSize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_HorizontallyScrollable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_HorizontallyScrollable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_VerticallyScrollable(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_VerticallyScrollable)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IScrollProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Scroll: unsafe extern "system" fn(*mut core::ffi::c_void, ScrollAmount, ScrollAmount) -> windows_core::HRESULT,
    pub SetScrollPercent: unsafe extern "system" fn(*mut core::ffi::c_void, f64, f64) -> windows_core::HRESULT,
    pub get_HorizontalScrollPercent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_VerticalScrollPercent: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_HorizontalViewSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_VerticalViewSize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_HorizontallyScrollable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_VerticallyScrollable: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IScrollProvider_Impl: windows_core::IUnknownImpl {
    fn Scroll(&self, horizontalamount: ScrollAmount, verticalamount: ScrollAmount) -> windows_core::Result<()>;
    fn SetScrollPercent(&self, horizontalpercent: f64, verticalpercent: f64) -> windows_core::Result<()>;
    fn get_HorizontalScrollPercent(&self) -> windows_core::Result<f64>;
    fn get_VerticalScrollPercent(&self) -> windows_core::Result<f64>;
    fn get_HorizontalViewSize(&self) -> windows_core::Result<f64>;
    fn get_VerticalViewSize(&self) -> windows_core::Result<f64>;
    fn get_HorizontallyScrollable(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_VerticallyScrollable(&self) -> windows_core::Result<windows_core::BOOL>;
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
        unsafe extern "system" fn get_HorizontalScrollPercent<Identity: IScrollProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScrollProvider_Impl::get_HorizontalScrollPercent(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_VerticalScrollPercent<Identity: IScrollProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScrollProvider_Impl::get_VerticalScrollPercent(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_HorizontalViewSize<Identity: IScrollProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScrollProvider_Impl::get_HorizontalViewSize(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_VerticalViewSize<Identity: IScrollProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScrollProvider_Impl::get_VerticalViewSize(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_HorizontallyScrollable<Identity: IScrollProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScrollProvider_Impl::get_HorizontallyScrollable(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_VerticallyScrollable<Identity: IScrollProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IScrollProvider_Impl::get_VerticallyScrollable(this) {
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
            get_HorizontalScrollPercent: get_HorizontalScrollPercent::<Identity, OFFSET>,
            get_VerticalScrollPercent: get_VerticalScrollPercent::<Identity, OFFSET>,
            get_HorizontalViewSize: get_HorizontalViewSize::<Identity, OFFSET>,
            get_VerticalViewSize: get_VerticalViewSize::<Identity, OFFSET>,
            get_HorizontallyScrollable: get_HorizontallyScrollable::<Identity, OFFSET>,
            get_VerticallyScrollable: get_VerticallyScrollable::<Identity, OFFSET>,
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
    pub unsafe fn get_IsSelected(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsSelected)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_SelectionContainer(&self) -> windows_core::Result<IRawElementProviderSimple> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SelectionContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
    pub get_IsSelected: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_SelectionContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ISelectionItemProvider_Impl: windows_core::IUnknownImpl {
    fn Select(&self) -> windows_core::Result<()>;
    fn AddToSelection(&self) -> windows_core::Result<()>;
    fn RemoveFromSelection(&self) -> windows_core::Result<()>;
    fn get_IsSelected(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_SelectionContainer(&self) -> windows_core::Result<IRawElementProviderSimple>;
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
        unsafe extern "system" fn get_IsSelected<Identity: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISelectionItemProvider_Impl::get_IsSelected(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SelectionContainer<Identity: ISelectionItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISelectionItemProvider_Impl::get_SelectionContainer(this) {
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
            get_IsSelected: get_IsSelected::<Identity, OFFSET>,
            get_SelectionContainer: get_SelectionContainer::<Identity, OFFSET>,
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
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetSelection(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CanSelectMultiple(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CanSelectMultiple)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_IsSelectionRequired(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsSelectionRequired)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetSelection: usize,
    pub get_CanSelectMultiple: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_IsSelectionRequired: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
pub trait ISelectionProvider_Impl: windows_core::IUnknownImpl {
    fn GetSelection(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn get_CanSelectMultiple(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_IsSelectionRequired(&self) -> windows_core::Result<windows_core::BOOL>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl ISelectionProvider_Vtbl {
    pub const fn new<Identity: ISelectionProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSelection<Identity: ISelectionProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
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
        unsafe extern "system" fn get_CanSelectMultiple<Identity: ISelectionProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISelectionProvider_Impl::get_CanSelectMultiple(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsSelectionRequired<Identity: ISelectionProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISelectionProvider_Impl::get_IsSelectionRequired(this) {
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
            get_CanSelectMultiple: get_CanSelectMultiple::<Identity, OFFSET>,
            get_IsSelectionRequired: get_IsSelectionRequired::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISelectionProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
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
    pub unsafe fn get_FirstSelectedItem(&self) -> windows_core::Result<IRawElementProviderSimple> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FirstSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_LastSelectedItem(&self) -> windows_core::Result<IRawElementProviderSimple> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_LastSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_CurrentSelectedItem(&self) -> windows_core::Result<IRawElementProviderSimple> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CurrentSelectedItem)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_ItemCount(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ItemCount)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ISelectionProvider2_Vtbl {
    pub base__: ISelectionProvider_Vtbl,
    pub get_FirstSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_LastSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_CurrentSelectedItem: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ItemCount: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
pub trait ISelectionProvider2_Impl: ISelectionProvider_Impl {
    fn get_FirstSelectedItem(&self) -> windows_core::Result<IRawElementProviderSimple>;
    fn get_LastSelectedItem(&self) -> windows_core::Result<IRawElementProviderSimple>;
    fn get_CurrentSelectedItem(&self) -> windows_core::Result<IRawElementProviderSimple>;
    fn get_ItemCount(&self) -> windows_core::Result<i32>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl ISelectionProvider2_Vtbl {
    pub const fn new<Identity: ISelectionProvider2_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_FirstSelectedItem<Identity: ISelectionProvider2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISelectionProvider2_Impl::get_FirstSelectedItem(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_LastSelectedItem<Identity: ISelectionProvider2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISelectionProvider2_Impl::get_LastSelectedItem(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CurrentSelectedItem<Identity: ISelectionProvider2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISelectionProvider2_Impl::get_CurrentSelectedItem(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ItemCount<Identity: ISelectionProvider2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISelectionProvider2_Impl::get_ItemCount(this) {
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
            get_FirstSelectedItem: get_FirstSelectedItem::<Identity, OFFSET>,
            get_LastSelectedItem: get_LastSelectedItem::<Identity, OFFSET>,
            get_CurrentSelectedItem: get_CurrentSelectedItem::<Identity, OFFSET>,
            get_ItemCount: get_ItemCount::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISelectionProvider2 as windows_core::Interface>::IID || iid == &<ISelectionProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for ISelectionProvider2 {}
windows_core::imp::define_interface!(ISpreadsheetItemProvider, ISpreadsheetItemProvider_Vtbl, 0xeaed4660_7b3d_4879_a2e6_365ce603f3d0);
windows_core::imp::interface_hierarchy!(ISpreadsheetItemProvider, windows_core::IUnknown);
impl ISpreadsheetItemProvider {
    pub unsafe fn get_Formula(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Formula)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetAnnotationObjects(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAnnotationObjects)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetAnnotationTypes(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
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
    pub get_Formula: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetAnnotationObjects: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetAnnotationObjects: usize,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetAnnotationTypes: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetAnnotationTypes: usize,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
pub trait ISpreadsheetItemProvider_Impl: windows_core::IUnknownImpl {
    fn get_Formula(&self) -> windows_core::Result<windows_core::BSTR>;
    fn GetAnnotationObjects(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn GetAnnotationTypes(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl ISpreadsheetItemProvider_Vtbl {
    pub const fn new<Identity: ISpreadsheetItemProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_Formula<Identity: ISpreadsheetItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ISpreadsheetItemProvider_Impl::get_Formula(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAnnotationObjects<Identity: ISpreadsheetItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetAnnotationTypes<Identity: ISpreadsheetItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
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
            get_Formula: get_Formula::<Identity, OFFSET>,
            GetAnnotationObjects: GetAnnotationObjects::<Identity, OFFSET>,
            GetAnnotationTypes: GetAnnotationTypes::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ISpreadsheetItemProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
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
            (windows_core::Interface::vtable(self).GetItemByName)(windows_core::Interface::as_raw(self), name.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
    pub unsafe fn get_StyleId(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StyleId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_StyleName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_StyleName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_FillColor(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FillColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_FillPatternStyle(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FillPatternStyle)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_Shape(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Shape)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_FillPatternColor(&self) -> windows_core::Result<i32> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_FillPatternColor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ExtendedProperties(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ExtendedProperties)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IStylesProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_StyleId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_StyleName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_FillColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_FillPatternStyle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_Shape: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_FillPatternColor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut i32) -> windows_core::HRESULT,
    pub get_ExtendedProperties: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait IStylesProvider_Impl: windows_core::IUnknownImpl {
    fn get_StyleId(&self) -> windows_core::Result<i32>;
    fn get_StyleName(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_FillColor(&self) -> windows_core::Result<i32>;
    fn get_FillPatternStyle(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_Shape(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_FillPatternColor(&self) -> windows_core::Result<i32>;
    fn get_ExtendedProperties(&self) -> windows_core::Result<windows_core::BSTR>;
}
impl IStylesProvider_Vtbl {
    pub const fn new<Identity: IStylesProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_StyleId<Identity: IStylesProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStylesProvider_Impl::get_StyleId(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_StyleName<Identity: IStylesProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStylesProvider_Impl::get_StyleName(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_FillColor<Identity: IStylesProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStylesProvider_Impl::get_FillColor(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_FillPatternStyle<Identity: IStylesProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStylesProvider_Impl::get_FillPatternStyle(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_Shape<Identity: IStylesProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStylesProvider_Impl::get_Shape(this) {
                    Ok(ok__) => {
                        retval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_FillPatternColor<Identity: IStylesProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut i32) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStylesProvider_Impl::get_FillPatternColor(this) {
                    Ok(ok__) => {
                        retval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ExtendedProperties<Identity: IStylesProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, retval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IStylesProvider_Impl::get_ExtendedProperties(this) {
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
            get_StyleId: get_StyleId::<Identity, OFFSET>,
            get_StyleName: get_StyleName::<Identity, OFFSET>,
            get_FillColor: get_FillColor::<Identity, OFFSET>,
            get_FillPatternStyle: get_FillPatternStyle::<Identity, OFFSET>,
            get_Shape: get_Shape::<Identity, OFFSET>,
            get_FillPatternColor: get_FillPatternColor::<Identity, OFFSET>,
            get_ExtendedProperties: get_ExtendedProperties::<Identity, OFFSET>,
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
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetRowHeaderItems(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRowHeaderItems)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetColumnHeaderItems(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
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
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetRowHeaderItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetRowHeaderItems: usize,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetColumnHeaderItems: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetColumnHeaderItems: usize,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
pub trait ITableItemProvider_Impl: windows_core::IUnknownImpl {
    fn GetRowHeaderItems(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn GetColumnHeaderItems(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl ITableItemProvider_Vtbl {
    pub const fn new<Identity: ITableItemProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRowHeaderItems<Identity: ITableItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetColumnHeaderItems<Identity: ITableItemProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
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
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for ITableItemProvider {}
windows_core::imp::define_interface!(ITableProvider, ITableProvider_Vtbl, 0x9c860395_97b3_490a_b52a_858cc22af166);
windows_core::imp::interface_hierarchy!(ITableProvider, windows_core::IUnknown);
impl ITableProvider {
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetRowHeaders(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetRowHeaders)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetColumnHeaders(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetColumnHeaders)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_RowOrColumnMajor(&self) -> windows_core::Result<RowOrColumnMajor> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_RowOrColumnMajor)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITableProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetRowHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetRowHeaders: usize,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetColumnHeaders: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetColumnHeaders: usize,
    pub get_RowOrColumnMajor: unsafe extern "system" fn(*mut core::ffi::c_void, *mut RowOrColumnMajor) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
pub trait ITableProvider_Impl: windows_core::IUnknownImpl {
    fn GetRowHeaders(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn GetColumnHeaders(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn get_RowOrColumnMajor(&self) -> windows_core::Result<RowOrColumnMajor>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl ITableProvider_Vtbl {
    pub const fn new<Identity: ITableProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetRowHeaders<Identity: ITableProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetColumnHeaders<Identity: ITableProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
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
        unsafe extern "system" fn get_RowOrColumnMajor<Identity: ITableProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut RowOrColumnMajor) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITableProvider_Impl::get_RowOrColumnMajor(this) {
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
            get_RowOrColumnMajor: get_RowOrColumnMajor::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITableProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for ITableProvider {}
windows_core::imp::define_interface!(ITextChildProvider, ITextChildProvider_Vtbl, 0x4c2de2b9_c88f_4f88_a111_f1d336b7d1a9);
windows_core::imp::interface_hierarchy!(ITextChildProvider, windows_core::IUnknown);
impl ITextChildProvider {
    pub unsafe fn get_TextContainer(&self) -> windows_core::Result<IRawElementProviderSimple> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TextContainer)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_TextRange(&self) -> windows_core::Result<ITextRangeProvider> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_TextRange)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextChildProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub get_TextContainer: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_TextRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
pub trait ITextChildProvider_Impl: windows_core::IUnknownImpl {
    fn get_TextContainer(&self) -> windows_core::Result<IRawElementProviderSimple>;
    fn get_TextRange(&self) -> windows_core::Result<ITextRangeProvider>;
}
impl ITextChildProvider_Vtbl {
    pub const fn new<Identity: ITextChildProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_TextContainer<Identity: ITextChildProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextChildProvider_Impl::get_TextContainer(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_TextRange<Identity: ITextChildProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextChildProvider_Impl::get_TextRange(this) {
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
            get_TextContainer: get_TextContainer::<Identity, OFFSET>,
            get_TextRange: get_TextRange::<Identity, OFFSET>,
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
            (windows_core::Interface::vtable(self).GetActiveComposition)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetConversionTarget(&self) -> windows_core::Result<ITextRangeProvider> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetConversionTarget)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
pub trait ITextEditProvider_Impl: ITextProvider_Impl {
    fn GetActiveComposition(&self) -> windows_core::Result<ITextRangeProvider>;
    fn GetConversionTarget(&self) -> windows_core::Result<ITextRangeProvider>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
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
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for ITextEditProvider {}
windows_core::imp::define_interface!(ITextProvider, ITextProvider_Vtbl, 0x3589c92c_63f3_4367_99bb_ada653b77cf2);
windows_core::imp::interface_hierarchy!(ITextProvider, windows_core::IUnknown);
impl ITextProvider {
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetSelection(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetVisibleRanges(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
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
            (windows_core::Interface::vtable(self).RangeFromChild)(windows_core::Interface::as_raw(self), childelement.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn RangeFromPoint(&self, point: UiaPoint) -> windows_core::Result<ITextRangeProvider> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).RangeFromPoint)(windows_core::Interface::as_raw(self), point, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_DocumentRange(&self) -> windows_core::Result<ITextRangeProvider> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_DocumentRange)(windows_core::Interface::as_raw(self), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn get_SupportedTextSelection(&self) -> windows_core::Result<SupportedTextSelection> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_SupportedTextSelection)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct ITextProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetSelection: usize,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetVisibleRanges: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetVisibleRanges: usize,
    pub RangeFromChild: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub RangeFromPoint: unsafe extern "system" fn(*mut core::ffi::c_void, UiaPoint, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_DocumentRange: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_SupportedTextSelection: unsafe extern "system" fn(*mut core::ffi::c_void, *mut SupportedTextSelection) -> windows_core::HRESULT,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
pub trait ITextProvider_Impl: windows_core::IUnknownImpl {
    fn GetSelection(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn GetVisibleRanges(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn RangeFromChild(&self, childelement: windows_core::Ref<IRawElementProviderSimple>) -> windows_core::Result<ITextRangeProvider>;
    fn RangeFromPoint(&self, point: &UiaPoint) -> windows_core::Result<ITextRangeProvider>;
    fn get_DocumentRange(&self) -> windows_core::Result<ITextRangeProvider>;
    fn get_SupportedTextSelection(&self) -> windows_core::Result<SupportedTextSelection>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl ITextProvider_Vtbl {
    pub const fn new<Identity: ITextProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetSelection<Identity: ITextProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetVisibleRanges<Identity: ITextProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
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
        unsafe extern "system" fn get_DocumentRange<Identity: ITextProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextProvider_Impl::get_DocumentRange(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_SupportedTextSelection<Identity: ITextProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut SupportedTextSelection) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextProvider_Impl::get_SupportedTextSelection(this) {
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
            get_DocumentRange: get_DocumentRange::<Identity, OFFSET>,
            get_SupportedTextSelection: get_SupportedTextSelection::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<ITextProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
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
            (windows_core::Interface::vtable(self).RangeFromAnnotation)(windows_core::Interface::as_raw(self), annotationelement.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn GetCaretRange(&self, isactive: *mut windows_core::BOOL) -> windows_core::Result<ITextRangeProvider> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetCaretRange)(windows_core::Interface::as_raw(self), isactive as _, &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
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
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
pub trait ITextProvider2_Impl: ITextProvider_Impl {
    fn RangeFromAnnotation(&self, annotationelement: windows_core::Ref<IRawElementProviderSimple>) -> windows_core::Result<ITextRangeProvider>;
    fn GetCaretRange(&self, isactive: *mut windows_core::BOOL) -> windows_core::Result<ITextRangeProvider>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
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
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
impl windows_core::RuntimeName for ITextProvider2 {}
windows_core::imp::define_interface!(ITextRangeProvider, ITextRangeProvider_Vtbl, 0x5347ad7b_c355_46f8_aff5_909033582f63);
windows_core::imp::interface_hierarchy!(ITextRangeProvider, windows_core::IUnknown);
impl ITextRangeProvider {
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
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn FindAttribute(&self, attributeid: TEXTATTRIBUTEID, val: &super::VARIANT, backward: bool) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindAttribute)(windows_core::Interface::as_raw(self), attributeid, core::mem::transmute_copy(val), backward.into(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    pub unsafe fn FindTextA(&self, text: &windows_core::BSTR, backward: bool, ignorecase: bool) -> windows_core::Result<Self> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).FindTextA)(windows_core::Interface::as_raw(self), core::mem::transmute_copy(text), backward.into(), ignorecase.into(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub unsafe fn GetAttributeValue(&self, attributeid: TEXTATTRIBUTEID) -> windows_core::Result<super::VARIANT> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetAttributeValue)(windows_core::Interface::as_raw(self), attributeid, &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetBoundingRectangles(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).GetBoundingRectangles)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn GetEnclosingElement(&self) -> windows_core::Result<IRawElementProviderSimple> {
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
pub struct ITextRangeProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Clone: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Compare: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub CompareEndpoints: unsafe extern "system" fn(*mut core::ffi::c_void, TextPatternRangeEndpoint, *mut core::ffi::c_void, TextPatternRangeEndpoint, *mut i32) -> windows_core::HRESULT,
    pub ExpandToEnclosingUnit: unsafe extern "system" fn(*mut core::ffi::c_void, TextUnit) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub FindAttribute: unsafe extern "system" fn(*mut core::ffi::c_void, TEXTATTRIBUTEID, super::VARIANT, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    FindAttribute: usize,
    pub FindTextA: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, windows_core::BOOL, windows_core::BOOL, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
    pub GetAttributeValue: unsafe extern "system" fn(*mut core::ffi::c_void, TEXTATTRIBUTEID, *mut super::VARIANT) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase")))]
    GetAttributeValue: usize,
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetBoundingRectangles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
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
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetChildren: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetChildren: usize,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITextRangeProvider_Impl: windows_core::IUnknownImpl {
    fn Clone(&self) -> windows_core::Result<ITextRangeProvider>;
    fn Compare(&self, range: windows_core::Ref<ITextRangeProvider>) -> windows_core::Result<windows_core::BOOL>;
    fn CompareEndpoints(&self, endpoint: TextPatternRangeEndpoint, targetrange: windows_core::Ref<ITextRangeProvider>, targetendpoint: TextPatternRangeEndpoint) -> windows_core::Result<i32>;
    fn ExpandToEnclosingUnit(&self, unit: TextUnit) -> windows_core::Result<()>;
    fn FindAttribute(&self, attributeid: TEXTATTRIBUTEID, val: &super::VARIANT, backward: windows_core::BOOL) -> windows_core::Result<ITextRangeProvider>;
    fn FindTextA(&self, text: &windows_core::BSTR, backward: windows_core::BOOL, ignorecase: windows_core::BOOL) -> windows_core::Result<ITextRangeProvider>;
    fn GetAttributeValue(&self, attributeid: TEXTATTRIBUTEID) -> windows_core::Result<super::VARIANT>;
    fn GetBoundingRectangles(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
    fn GetEnclosingElement(&self) -> windows_core::Result<IRawElementProviderSimple>;
    fn GetText(&self, maxlength: i32) -> windows_core::Result<windows_core::BSTR>;
    fn Move(&self, unit: TextUnit, count: i32) -> windows_core::Result<i32>;
    fn MoveEndpointByUnit(&self, endpoint: TextPatternRangeEndpoint, unit: TextUnit, count: i32) -> windows_core::Result<i32>;
    fn MoveEndpointByRange(&self, endpoint: TextPatternRangeEndpoint, targetrange: windows_core::Ref<ITextRangeProvider>, targetendpoint: TextPatternRangeEndpoint) -> windows_core::Result<()>;
    fn Select(&self) -> windows_core::Result<()>;
    fn AddToSelection(&self) -> windows_core::Result<()>;
    fn RemoveFromSelection(&self) -> windows_core::Result<()>;
    fn ScrollIntoView(&self, aligntotop: windows_core::BOOL) -> windows_core::Result<()>;
    fn GetChildren(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
        unsafe extern "system" fn FindAttribute<Identity: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributeid: TEXTATTRIBUTEID, val: super::VARIANT, backward: windows_core::BOOL, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
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
        unsafe extern "system" fn FindTextA<Identity: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, text: *mut core::ffi::c_void, backward: windows_core::BOOL, ignorecase: windows_core::BOOL, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITextRangeProvider_Impl::FindTextA(this, core::mem::transmute(&text), core::mem::transmute_copy(&backward), core::mem::transmute_copy(&ignorecase)) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn GetAttributeValue<Identity: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, attributeid: TEXTATTRIBUTEID, pretval: *mut super::VARIANT) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetBoundingRectangles<Identity: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
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
        unsafe extern "system" fn GetChildren<Identity: ITextRangeProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
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
        iid == &<ITextRangeProvider as windows_core::Interface>::IID
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub trait ITextRangeProvider2_Impl: ITextRangeProvider_Impl {
    fn ShowContextMenu(&self) -> windows_core::Result<()>;
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl windows_core::RuntimeName for ITextRangeProvider2 {}
windows_core::imp::define_interface!(IToggleProvider, IToggleProvider_Vtbl, 0x56d00bd0_c4f4_433c_a836_1a52a57e0892);
windows_core::imp::interface_hierarchy!(IToggleProvider, windows_core::IUnknown);
impl IToggleProvider {
    pub unsafe fn Toggle(&self) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).Toggle)(windows_core::Interface::as_raw(self)) }
    }
    pub unsafe fn get_ToggleState(&self) -> windows_core::Result<ToggleState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ToggleState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IToggleProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub Toggle: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_ToggleState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut ToggleState) -> windows_core::HRESULT,
}
pub trait IToggleProvider_Impl: windows_core::IUnknownImpl {
    fn Toggle(&self) -> windows_core::Result<()>;
    fn get_ToggleState(&self) -> windows_core::Result<ToggleState>;
}
impl IToggleProvider_Vtbl {
    pub const fn new<Identity: IToggleProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn Toggle<Identity: IToggleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IToggleProvider_Impl::Toggle(this).into()
            }
        }
        unsafe extern "system" fn get_ToggleState<Identity: IToggleProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut ToggleState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IToggleProvider_Impl::get_ToggleState(this) {
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
            Toggle: Toggle::<Identity, OFFSET>,
            get_ToggleState: get_ToggleState::<Identity, OFFSET>,
        }
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
    pub unsafe fn get_CanMove(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CanMove)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CanResize(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CanResize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CanRotate(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CanRotate)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    pub get_CanMove: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CanResize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CanRotate: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait ITransformProvider_Impl: windows_core::IUnknownImpl {
    fn Move(&self, x: f64, y: f64) -> windows_core::Result<()>;
    fn Resize(&self, width: f64, height: f64) -> windows_core::Result<()>;
    fn Rotate(&self, degrees: f64) -> windows_core::Result<()>;
    fn get_CanMove(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CanResize(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CanRotate(&self) -> windows_core::Result<windows_core::BOOL>;
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
        unsafe extern "system" fn get_CanMove<Identity: ITransformProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransformProvider_Impl::get_CanMove(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CanResize<Identity: ITransformProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransformProvider_Impl::get_CanResize(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CanRotate<Identity: ITransformProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransformProvider_Impl::get_CanRotate(this) {
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
            get_CanMove: get_CanMove::<Identity, OFFSET>,
            get_CanResize: get_CanResize::<Identity, OFFSET>,
            get_CanRotate: get_CanRotate::<Identity, OFFSET>,
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
    pub unsafe fn get_CanZoom(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CanZoom)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ZoomLevel(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ZoomLevel)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ZoomMinimum(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ZoomMinimum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ZoomMaximum(&self) -> windows_core::Result<f64> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ZoomMaximum)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    pub get_CanZoom: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_ZoomLevel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_ZoomMinimum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub get_ZoomMaximum: unsafe extern "system" fn(*mut core::ffi::c_void, *mut f64) -> windows_core::HRESULT,
    pub ZoomByUnit: unsafe extern "system" fn(*mut core::ffi::c_void, ZoomUnit) -> windows_core::HRESULT,
}
pub trait ITransformProvider2_Impl: ITransformProvider_Impl {
    fn Zoom(&self, zoom: f64) -> windows_core::Result<()>;
    fn get_CanZoom(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_ZoomLevel(&self) -> windows_core::Result<f64>;
    fn get_ZoomMinimum(&self) -> windows_core::Result<f64>;
    fn get_ZoomMaximum(&self) -> windows_core::Result<f64>;
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
        unsafe extern "system" fn get_CanZoom<Identity: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransformProvider2_Impl::get_CanZoom(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ZoomLevel<Identity: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransformProvider2_Impl::get_ZoomLevel(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ZoomMinimum<Identity: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransformProvider2_Impl::get_ZoomMinimum(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ZoomMaximum<Identity: ITransformProvider2_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut f64) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match ITransformProvider2_Impl::get_ZoomMaximum(this) {
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
            get_CanZoom: get_CanZoom::<Identity, OFFSET>,
            get_ZoomLevel: get_ZoomLevel::<Identity, OFFSET>,
            get_ZoomMinimum: get_ZoomMinimum::<Identity, OFFSET>,
            get_ZoomMaximum: get_ZoomMaximum::<Identity, OFFSET>,
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
    #[cfg(feature = "shared")]
    pub unsafe fn get_ProcessId(&self) -> windows_core::Result<super::DWORD> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ProcessId)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_ProcessName(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_ProcessName)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationClientInfo_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub get_ProcessId: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::DWORD) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    get_ProcessId: usize,
    pub get_ProcessName: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[cfg(feature = "shared")]
pub trait IUIAutomationClientInfo_Impl: windows_core::IUnknownImpl {
    fn get_ProcessId(&self) -> windows_core::Result<super::DWORD>;
    fn get_ProcessName(&self) -> windows_core::Result<windows_core::BSTR>;
}
#[cfg(feature = "shared")]
impl IUIAutomationClientInfo_Vtbl {
    pub const fn new<Identity: IUIAutomationClientInfo_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn get_ProcessId<Identity: IUIAutomationClientInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, processid: *mut super::DWORD) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationClientInfo_Impl::get_ProcessId(this) {
                    Ok(ok__) => {
                        processid.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_ProcessName<Identity: IUIAutomationClientInfo_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, processname: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IUIAutomationClientInfo_Impl::get_ProcessName(this) {
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
            get_ProcessId: get_ProcessId::<Identity, OFFSET>,
            get_ProcessName: get_ProcessName::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IUIAutomationClientInfo as windows_core::Interface>::IID
    }
}
#[cfg(feature = "shared")]
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
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub unsafe fn GetConnectedClients(&self) -> windows_core::Result<*mut super::SAFEARRAY> {
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
    #[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
    pub GetConnectedClients: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut super::SAFEARRAY) -> windows_core::HRESULT,
    #[cfg(not(all(feature = "oaidl", feature = "shared", feature = "winnt")))]
    GetConnectedClients: usize,
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
pub trait IUIAutomationClientInfoSource_Impl: windows_core::IUnknownImpl {
    fn RegisterClientConnectionCallback(&self, callback: windows_core::Ref<IUIAutomationClientConnectionCallback>) -> windows_core::Result<u64>;
    fn UnregisterClientConnectionCallback(&self, handle: u64) -> windows_core::Result<()>;
    fn GetConnectedClients(&self) -> windows_core::Result<*mut super::SAFEARRAY>;
}
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
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
        unsafe extern "system" fn GetConnectedClients<Identity: IUIAutomationClientInfoSource_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, clients: *mut *mut super::SAFEARRAY) -> windows_core::HRESULT {
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
#[cfg(all(feature = "oaidl", feature = "shared", feature = "winnt"))]
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
            (windows_core::Interface::vtable(self).CreateClientWrapper)(windows_core::Interface::as_raw(self), ppatterninstance.param().abi(), &mut result__).and_then(|| windows_core::imp::Type::from_abi(result__))
        }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn Dispatch<P0>(&self, ptarget: P0, index: super::UINT, pparams: *const UIAutomationParameter, cparams: super::UINT) -> windows_core::HRESULT
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
    #[cfg(feature = "shared")]
    pub Dispatch: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, super::UINT, *const UIAutomationParameter, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    Dispatch: usize,
}
#[cfg(feature = "shared")]
pub trait IUIAutomationPatternHandler_Impl: windows_core::IUnknownImpl {
    fn CreateClientWrapper(&self, ppatterninstance: windows_core::Ref<IUIAutomationPatternInstance>) -> windows_core::Result<windows_core::IUnknown>;
    fn Dispatch(&self, ptarget: windows_core::Ref<windows_core::IUnknown>, index: super::UINT, pparams: *const UIAutomationParameter, cparams: super::UINT) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
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
        unsafe extern "system" fn Dispatch<Identity: IUIAutomationPatternHandler_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, ptarget: *mut core::ffi::c_void, index: super::UINT, pparams: *const UIAutomationParameter, cparams: super::UINT) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IUIAutomationPatternHandler {}
windows_core::imp::define_interface!(IUIAutomationPatternInstance, IUIAutomationPatternInstance_Vtbl, 0xc03a7fe4_9431_409f_bed8_ae7c2299bc8d);
windows_core::imp::interface_hierarchy!(IUIAutomationPatternInstance, windows_core::IUnknown);
impl IUIAutomationPatternInstance {
    #[cfg(feature = "shared")]
    pub unsafe fn GetProperty(&self, index: super::UINT, cached: bool, r#type: UIAutomationType, pptr: *mut core::ffi::c_void) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).GetProperty)(windows_core::Interface::as_raw(self), index, cached.into(), r#type, pptr as _) }
    }
    #[cfg(feature = "shared")]
    pub unsafe fn CallMethod(&self, index: super::UINT, pparams: *const UIAutomationParameter, cparams: super::UINT) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).CallMethod)(windows_core::Interface::as_raw(self), index, pparams, cparams) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationPatternInstance_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub GetProperty: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, windows_core::BOOL, UIAutomationType, *mut core::ffi::c_void) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    GetProperty: usize,
    #[cfg(feature = "shared")]
    pub CallMethod: unsafe extern "system" fn(*mut core::ffi::c_void, super::UINT, *const UIAutomationParameter, super::UINT) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    CallMethod: usize,
}
#[cfg(feature = "shared")]
pub trait IUIAutomationPatternInstance_Impl: windows_core::IUnknownImpl {
    fn GetProperty(&self, index: super::UINT, cached: windows_core::BOOL, r#type: UIAutomationType, pptr: *mut core::ffi::c_void) -> windows_core::Result<()>;
    fn CallMethod(&self, index: super::UINT, pparams: *const UIAutomationParameter, cparams: super::UINT) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IUIAutomationPatternInstance_Vtbl {
    pub const fn new<Identity: IUIAutomationPatternInstance_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn GetProperty<Identity: IUIAutomationPatternInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, cached: windows_core::BOOL, r#type: UIAutomationType, pptr: *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IUIAutomationPatternInstance_Impl::GetProperty(this, core::mem::transmute_copy(&index), core::mem::transmute_copy(&cached), core::mem::transmute_copy(&r#type), core::mem::transmute_copy(&pptr)).into()
            }
        }
        unsafe extern "system" fn CallMethod<Identity: IUIAutomationPatternInstance_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, index: super::UINT, pparams: *const UIAutomationParameter, cparams: super::UINT) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
impl windows_core::RuntimeName for IUIAutomationPatternInstance {}
windows_core::imp::define_interface!(IUIAutomationRegistrar, IUIAutomationRegistrar_Vtbl, 0x8609c4ec_4a1a_4d88_a357_5a66e060e1cf);
windows_core::imp::interface_hierarchy!(IUIAutomationRegistrar, windows_core::IUnknown);
impl IUIAutomationRegistrar {
    #[cfg(feature = "shared")]
    pub unsafe fn RegisterProperty(&self, property: *const UIAutomationPropertyInfo) -> windows_core::Result<super::PROPERTYID> {
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
    #[cfg(feature = "shared")]
    pub unsafe fn RegisterPattern(&self, pattern: *const UIAutomationPatternInfo, ppatternid: *mut PATTERNID, ppatternavailablepropertyid: *mut super::PROPERTYID, propertyidcount: super::UINT, ppropertyids: *mut super::PROPERTYID, eventidcount: super::UINT, peventids: *mut EVENTID) -> windows_core::HRESULT {
        unsafe { (windows_core::Interface::vtable(self).RegisterPattern)(windows_core::Interface::as_raw(self), pattern, ppatternid as _, ppatternavailablepropertyid as _, propertyidcount, ppropertyids as _, eventidcount, peventids as _) }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IUIAutomationRegistrar_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    #[cfg(feature = "shared")]
    pub RegisterProperty: unsafe extern "system" fn(*mut core::ffi::c_void, *const UIAutomationPropertyInfo, *mut super::PROPERTYID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RegisterProperty: usize,
    pub RegisterEvent: unsafe extern "system" fn(*mut core::ffi::c_void, *const UIAutomationEventInfo, *mut EVENTID) -> windows_core::HRESULT,
    #[cfg(feature = "shared")]
    pub RegisterPattern: unsafe extern "system" fn(*mut core::ffi::c_void, *const UIAutomationPatternInfo, *mut PATTERNID, *mut super::PROPERTYID, super::UINT, *mut super::PROPERTYID, super::UINT, *mut EVENTID) -> windows_core::HRESULT,
    #[cfg(not(feature = "shared"))]
    RegisterPattern: usize,
}
#[cfg(feature = "shared")]
pub trait IUIAutomationRegistrar_Impl: windows_core::IUnknownImpl {
    fn RegisterProperty(&self, property: *const UIAutomationPropertyInfo) -> windows_core::Result<super::PROPERTYID>;
    fn RegisterEvent(&self, event: *const UIAutomationEventInfo) -> windows_core::Result<EVENTID>;
    fn RegisterPattern(&self, pattern: *const UIAutomationPatternInfo, ppatternid: *mut PATTERNID, ppatternavailablepropertyid: *mut super::PROPERTYID, propertyidcount: super::UINT, ppropertyids: *mut super::PROPERTYID, eventidcount: super::UINT, peventids: *mut EVENTID) -> windows_core::Result<()>;
}
#[cfg(feature = "shared")]
impl IUIAutomationRegistrar_Vtbl {
    pub const fn new<Identity: IUIAutomationRegistrar_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn RegisterProperty<Identity: IUIAutomationRegistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, property: *const UIAutomationPropertyInfo, propertyid: *mut super::PROPERTYID) -> windows_core::HRESULT {
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
        unsafe extern "system" fn RegisterPattern<Identity: IUIAutomationRegistrar_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pattern: *const UIAutomationPatternInfo, ppatternid: *mut PATTERNID, ppatternavailablepropertyid: *mut super::PROPERTYID, propertyidcount: super::UINT, ppropertyids: *mut super::PROPERTYID, eventidcount: super::UINT, peventids: *mut EVENTID) -> windows_core::HRESULT {
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
#[cfg(feature = "shared")]
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
    pub unsafe fn get_Value(&self) -> windows_core::Result<windows_core::BSTR> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_Value)(windows_core::Interface::as_raw(self), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub unsafe fn get_IsReadOnly(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsReadOnly)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
}
#[repr(C)]
#[doc(hidden)]
pub struct IValueProvider_Vtbl {
    pub base__: windows_core::IUnknown_Vtbl,
    pub SetValue: unsafe extern "system" fn(*mut core::ffi::c_void, windows_core::PCWSTR) -> windows_core::HRESULT,
    pub get_Value: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub get_IsReadOnly: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IValueProvider_Impl: windows_core::IUnknownImpl {
    fn SetValue(&self, val: &windows_core::PCWSTR) -> windows_core::Result<()>;
    fn get_Value(&self) -> windows_core::Result<windows_core::BSTR>;
    fn get_IsReadOnly(&self) -> windows_core::Result<windows_core::BOOL>;
}
impl IValueProvider_Vtbl {
    pub const fn new<Identity: IValueProvider_Impl, const OFFSET: isize>() -> Self {
        unsafe extern "system" fn SetValue<Identity: IValueProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, val: windows_core::PCWSTR) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                IValueProvider_Impl::SetValue(this, core::mem::transmute(&val)).into()
            }
        }
        unsafe extern "system" fn get_Value<Identity: IValueProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut *mut core::ffi::c_void) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueProvider_Impl::get_Value(this) {
                    Ok(ok__) => {
                        pretval.write(core::mem::transmute(ok__));
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsReadOnly<Identity: IValueProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IValueProvider_Impl::get_IsReadOnly(this) {
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
            get_Value: get_Value::<Identity, OFFSET>,
            get_IsReadOnly: get_IsReadOnly::<Identity, OFFSET>,
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
    pub unsafe fn get_CanMaximize(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CanMaximize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_CanMinimize(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_CanMinimize)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_IsModal(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsModal)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_WindowVisualState(&self) -> windows_core::Result<WindowVisualState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WindowVisualState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_WindowInteractionState(&self) -> windows_core::Result<WindowInteractionState> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_WindowInteractionState)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
        }
    }
    pub unsafe fn get_IsTopmost(&self) -> windows_core::Result<windows_core::BOOL> {
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(self).get_IsTopmost)(windows_core::Interface::as_raw(self), &mut result__).map(|| result__)
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
    pub get_CanMaximize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_CanMinimize: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_IsModal: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
    pub get_WindowVisualState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowVisualState) -> windows_core::HRESULT,
    pub get_WindowInteractionState: unsafe extern "system" fn(*mut core::ffi::c_void, *mut WindowInteractionState) -> windows_core::HRESULT,
    pub get_IsTopmost: unsafe extern "system" fn(*mut core::ffi::c_void, *mut windows_core::BOOL) -> windows_core::HRESULT,
}
pub trait IWindowProvider_Impl: windows_core::IUnknownImpl {
    fn SetVisualState(&self, state: WindowVisualState) -> windows_core::Result<()>;
    fn Close(&self) -> windows_core::Result<()>;
    fn WaitForInputIdle(&self, milliseconds: i32) -> windows_core::Result<windows_core::BOOL>;
    fn get_CanMaximize(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_CanMinimize(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_IsModal(&self) -> windows_core::Result<windows_core::BOOL>;
    fn get_WindowVisualState(&self) -> windows_core::Result<WindowVisualState>;
    fn get_WindowInteractionState(&self) -> windows_core::Result<WindowInteractionState>;
    fn get_IsTopmost(&self) -> windows_core::Result<windows_core::BOOL>;
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
        unsafe extern "system" fn get_CanMaximize<Identity: IWindowProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowProvider_Impl::get_CanMaximize(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_CanMinimize<Identity: IWindowProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowProvider_Impl::get_CanMinimize(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsModal<Identity: IWindowProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowProvider_Impl::get_IsModal(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_WindowVisualState<Identity: IWindowProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut WindowVisualState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowProvider_Impl::get_WindowVisualState(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_WindowInteractionState<Identity: IWindowProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut WindowInteractionState) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowProvider_Impl::get_WindowInteractionState(this) {
                    Ok(ok__) => {
                        pretval.write(ok__);
                        windows_core::HRESULT(0)
                    }
                    Err(err) => err.into(),
                }
            }
        }
        unsafe extern "system" fn get_IsTopmost<Identity: IWindowProvider_Impl, const OFFSET: isize>(this: *mut core::ffi::c_void, pretval: *mut windows_core::BOOL) -> windows_core::HRESULT {
            unsafe {
                let this: &Identity = &*((this as *const *const ()).offset(OFFSET) as *const Identity);
                match IWindowProvider_Impl::get_IsTopmost(this) {
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
            get_CanMaximize: get_CanMaximize::<Identity, OFFSET>,
            get_CanMinimize: get_CanMinimize::<Identity, OFFSET>,
            get_IsModal: get_IsModal::<Identity, OFFSET>,
            get_WindowVisualState: get_WindowVisualState::<Identity, OFFSET>,
            get_WindowInteractionState: get_WindowInteractionState::<Identity, OFFSET>,
            get_IsTopmost: get_IsTopmost::<Identity, OFFSET>,
        }
    }
    pub fn matches(iid: &windows_core::GUID) -> bool {
        iid == &<IWindowProvider as windows_core::Interface>::IID
    }
}
impl windows_core::RuntimeName for IWindowProvider {}
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
pub const Polite: LiveSetting = 1;
pub type ProviderOptions = i32;
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
pub type SynchronizedInputType = i32;
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
pub const UIA_ScrollPatternNoScroll: f64 = -1.0;
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UIAutomationEventInfo {
    pub guid: windows_core::GUID,
    pub pProgrammaticName: windows_core::PCWSTR,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UIAutomationMethodInfo {
    pub pProgrammaticName: windows_core::PCWSTR,
    pub doSetFocus: windows_core::BOOL,
    pub cInParameters: super::UINT,
    pub cOutParameters: super::UINT,
    pub pParameterTypes: *mut UIAutomationType,
    pub pParameterNames: *mut windows_core::PCWSTR,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UIAutomationParameter {
    pub r#type: UIAutomationType,
    pub pData: *mut core::ffi::c_void,
}
#[repr(C)]
#[cfg(feature = "shared")]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct UIAutomationPatternInfo {
    pub guid: windows_core::GUID,
    pub pProgrammaticName: windows_core::PCWSTR,
    pub providerInterfaceId: windows_core::GUID,
    pub clientInterfaceId: windows_core::GUID,
    pub cProperties: super::UINT,
    pub pProperties: *mut UIAutomationPropertyInfo,
    pub cMethods: super::UINT,
    pub pMethods: *mut UIAutomationMethodInfo,
    pub cEvents: super::UINT,
    pub pEvents: *mut UIAutomationEventInfo,
    pub pPatternHandler: core::mem::ManuallyDrop<Option<IUIAutomationPatternHandler>>,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UIAutomationPropertyInfo {
    pub guid: windows_core::GUID,
    pub pProgrammaticName: windows_core::PCWSTR,
    pub r#type: UIAutomationType,
}
pub type UIAutomationType = i32;
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
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
pub struct UiaChangeInfo {
    pub uiaId: i32,
    pub payload: super::VARIANT,
    pub extraInfo: super::VARIANT,
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
impl Clone for UiaChangeInfo {
    fn clone(&self) -> Self {
        unsafe { core::mem::transmute_copy(self) }
    }
}
#[cfg(all(feature = "minwindef", feature = "oaidl", feature = "shared", feature = "winnt", feature = "wtypes", feature = "wtypesbase"))]
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
