#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreFrameworkInputView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreFrameworkInputView, windows_core::IUnknown, windows_core::IInspectable);
impl CoreFrameworkInputView {
    pub fn PrimaryViewAnimationStarting<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreFrameworkInputView, CoreFrameworkInputViewAnimationStartingEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrimaryViewAnimationStarting)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePrimaryViewAnimationStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemovePrimaryViewAnimationStarting)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn OcclusionsChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreFrameworkInputView, CoreFrameworkInputViewOcclusionsChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OcclusionsChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveOcclusionsChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveOcclusionsChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForUIContext<P0>(context: P0) -> windows_core::Result<CoreFrameworkInputView>
    where
        P0: windows_core::Param<super::super::UIContext>,
    {
        Self::ICoreFrameworkInputViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUIContext)(windows_core::Interface::as_raw(this), context.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetForCurrentView() -> windows_core::Result<CoreFrameworkInputView> {
        Self::ICoreFrameworkInputViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ICoreFrameworkInputViewStatics<R, F: FnOnce(&ICoreFrameworkInputViewStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreFrameworkInputView, ICoreFrameworkInputViewStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CoreFrameworkInputView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreFrameworkInputView>();
}
unsafe impl windows_core::Interface for CoreFrameworkInputView {
    type Vtable = <ICoreFrameworkInputView as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICoreFrameworkInputView as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreFrameworkInputView {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreFrameworkInputView";
}
unsafe impl Send for CoreFrameworkInputView {}
unsafe impl Sync for CoreFrameworkInputView {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreFrameworkInputViewAnimationStartingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreFrameworkInputViewAnimationStartingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CoreFrameworkInputViewAnimationStartingEventArgs {
    pub fn Occlusions(&self) -> windows_core::Result<windows_collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Occlusions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn FrameworkAnimationRecommended(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).FrameworkAnimationRecommended)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn AnimationDuration(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AnimationDuration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CoreFrameworkInputViewAnimationStartingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreFrameworkInputViewAnimationStartingEventArgs>();
}
unsafe impl windows_core::Interface for CoreFrameworkInputViewAnimationStartingEventArgs {
    type Vtable = <ICoreFrameworkInputViewAnimationStartingEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICoreFrameworkInputViewAnimationStartingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreFrameworkInputViewAnimationStartingEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreFrameworkInputViewAnimationStartingEventArgs";
}
unsafe impl Send for CoreFrameworkInputViewAnimationStartingEventArgs {}
unsafe impl Sync for CoreFrameworkInputViewAnimationStartingEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreFrameworkInputViewOcclusionsChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreFrameworkInputViewOcclusionsChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CoreFrameworkInputViewOcclusionsChangedEventArgs {
    pub fn Occlusions(&self) -> windows_core::Result<windows_collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Occlusions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreFrameworkInputViewOcclusionsChangedEventArgs>();
}
unsafe impl windows_core::Interface for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    type Vtable = <ICoreFrameworkInputViewOcclusionsChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICoreFrameworkInputViewOcclusionsChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreFrameworkInputViewOcclusionsChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreFrameworkInputViewOcclusionsChangedEventArgs";
}
unsafe impl Send for CoreFrameworkInputViewOcclusionsChangedEventArgs {}
unsafe impl Sync for CoreFrameworkInputViewOcclusionsChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreInputView(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreInputView, windows_core::IUnknown, windows_core::IInspectable);
impl CoreInputView {
    pub fn OcclusionsChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewOcclusionsChangedEventArgs>>,
    {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OcclusionsChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveOcclusionsChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).RemoveOcclusionsChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetCoreInputViewOcclusions(&self) -> windows_core::Result<windows_collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetCoreInputViewOcclusions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn TryShowPrimaryView(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryShowPrimaryView)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TryHidePrimaryView(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryHidePrimaryView)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn XYFocusTransferringFromPrimaryView<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewTransferringXYFocusEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusTransferringFromPrimaryView)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveXYFocusTransferringFromPrimaryView(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveXYFocusTransferringFromPrimaryView)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn XYFocusTransferredToPrimaryView<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreInputView, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).XYFocusTransferredToPrimaryView)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveXYFocusTransferredToPrimaryView(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveXYFocusTransferredToPrimaryView)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn TryTransferXYFocusToPrimaryView(&self, origin: super::super::super::Foundation::Rect, direction: CoreInputViewXYFocusTransferDirection) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICoreInputView2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryTransferXYFocusToPrimaryView)(windows_core::Interface::as_raw(this), origin, direction, &mut result__).map(|| result__)
        }
    }
    pub fn TryShow(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICoreInputView3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryShow)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn TryShowWithKind(&self, r#type: CoreInputViewKind) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICoreInputView3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryShowWithKind)(windows_core::Interface::as_raw(this), r#type, &mut result__).map(|| result__)
        }
    }
    pub fn TryHide(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICoreInputView3>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryHide)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn PrimaryViewShowing<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewShowingEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICoreInputView4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrimaryViewShowing)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePrimaryViewShowing(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreInputView4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePrimaryViewShowing)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PrimaryViewHiding<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewHidingEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICoreInputView4>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrimaryViewHiding)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePrimaryViewHiding(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreInputView4>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePrimaryViewHiding)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn IsKindSupported(&self, r#type: CoreInputViewKind) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsKindSupported)(windows_core::Interface::as_raw(this), r#type, &mut result__).map(|| result__)
        }
    }
    pub fn SupportedKindsChanged<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreInputView, windows_core::IInspectable>>,
    {
        let this = &windows_core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).SupportedKindsChanged)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemoveSupportedKindsChanged(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemoveSupportedKindsChanged)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn PrimaryViewAnimationStarting<P0>(&self, handler: P0) -> windows_core::Result<i64>
    where
        P0: windows_core::Param<super::super::super::Foundation::TypedEventHandler<CoreInputView, CoreInputViewAnimationStartingEventArgs>>,
    {
        let this = &windows_core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).PrimaryViewAnimationStarting)(windows_core::Interface::as_raw(this), handler.param().abi(), &mut result__).map(|| result__)
        }
    }
    pub fn RemovePrimaryViewAnimationStarting(&self, token: i64) -> windows_core::Result<()> {
        let this = &windows_core::Interface::cast::<ICoreInputView5>(self)?;
        unsafe { (windows_core::Interface::vtable(this).RemovePrimaryViewAnimationStarting)(windows_core::Interface::as_raw(this), token).ok() }
    }
    pub fn GetForCurrentView() -> windows_core::Result<CoreInputView> {
        Self::ICoreInputViewStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForCurrentView)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn GetForUIContext<P0>(context: P0) -> windows_core::Result<CoreInputView>
    where
        P0: windows_core::Param<super::super::UIContext>,
    {
        Self::ICoreInputViewStatics2(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetForUIContext)(windows_core::Interface::as_raw(this), context.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn ICoreInputViewStatics<R, F: FnOnce(&ICoreInputViewStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreInputView, ICoreInputViewStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
    fn ICoreInputViewStatics2<R, F: FnOnce(&ICoreInputViewStatics2) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<CoreInputView, ICoreInputViewStatics2> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for CoreInputView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreInputView>();
}
unsafe impl windows_core::Interface for CoreInputView {
    type Vtable = <ICoreInputView as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICoreInputView as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreInputView {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputView";
}
unsafe impl Send for CoreInputView {}
unsafe impl Sync for CoreInputView {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreInputViewAnimationStartingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreInputViewAnimationStartingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CoreInputViewAnimationStartingEventArgs {
    pub fn Occlusions(&self) -> windows_core::Result<windows_collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Occlusions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHandled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn AnimationDuration(&self) -> windows_core::Result<super::super::super::Foundation::TimeSpan> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).AnimationDuration)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CoreInputViewAnimationStartingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreInputViewAnimationStartingEventArgs>();
}
unsafe impl windows_core::Interface for CoreInputViewAnimationStartingEventArgs {
    type Vtable = <ICoreInputViewAnimationStartingEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICoreInputViewAnimationStartingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreInputViewAnimationStartingEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewAnimationStartingEventArgs";
}
unsafe impl Send for CoreInputViewAnimationStartingEventArgs {}
unsafe impl Sync for CoreInputViewAnimationStartingEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreInputViewHidingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreInputViewHidingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CoreInputViewHidingEventArgs {
    pub fn TryCancel(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCancel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CoreInputViewHidingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreInputViewHidingEventArgs>();
}
unsafe impl windows_core::Interface for CoreInputViewHidingEventArgs {
    type Vtable = <ICoreInputViewHidingEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICoreInputViewHidingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreInputViewHidingEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewHidingEventArgs";
}
unsafe impl Send for CoreInputViewHidingEventArgs {}
unsafe impl Sync for CoreInputViewHidingEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CoreInputViewKind(pub i32);
impl CoreInputViewKind {
    pub const Default: Self = Self(0i32);
    pub const Keyboard: Self = Self(1i32);
    pub const Handwriting: Self = Self(2i32);
    pub const Emoji: Self = Self(3i32);
    pub const Symbols: Self = Self(4i32);
    pub const Clipboard: Self = Self(5i32);
    pub const Dictation: Self = Self(6i32);
    pub const Gamepad: Self = Self(7i32);
}
impl windows_core::TypeKind for CoreInputViewKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CoreInputViewKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.Core.CoreInputViewKind;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreInputViewOcclusion(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreInputViewOcclusion, windows_core::IUnknown, windows_core::IInspectable);
impl CoreInputViewOcclusion {
    pub fn OccludingRect(&self) -> windows_core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OccludingRect)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn OcclusionKind(&self) -> windows_core::Result<CoreInputViewOcclusionKind> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).OcclusionKind)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CoreInputViewOcclusion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreInputViewOcclusion>();
}
unsafe impl windows_core::Interface for CoreInputViewOcclusion {
    type Vtable = <ICoreInputViewOcclusion as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICoreInputViewOcclusion as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreInputViewOcclusion {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewOcclusion";
}
unsafe impl Send for CoreInputViewOcclusion {}
unsafe impl Sync for CoreInputViewOcclusion {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CoreInputViewOcclusionKind(pub i32);
impl CoreInputViewOcclusionKind {
    pub const Docked: Self = Self(0i32);
    pub const Floating: Self = Self(1i32);
    pub const Overlay: Self = Self(2i32);
}
impl windows_core::TypeKind for CoreInputViewOcclusionKind {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CoreInputViewOcclusionKind {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.Core.CoreInputViewOcclusionKind;i4)");
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreInputViewOcclusionsChangedEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreInputViewOcclusionsChangedEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CoreInputViewOcclusionsChangedEventArgs {
    pub fn Occlusions(&self) -> windows_core::Result<windows_collections::IVectorView<CoreInputViewOcclusion>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Occlusions)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Handled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Handled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetHandled)(windows_core::Interface::as_raw(this), value).ok() }
    }
}
impl windows_core::RuntimeType for CoreInputViewOcclusionsChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreInputViewOcclusionsChangedEventArgs>();
}
unsafe impl windows_core::Interface for CoreInputViewOcclusionsChangedEventArgs {
    type Vtable = <ICoreInputViewOcclusionsChangedEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICoreInputViewOcclusionsChangedEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreInputViewOcclusionsChangedEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewOcclusionsChangedEventArgs";
}
unsafe impl Send for CoreInputViewOcclusionsChangedEventArgs {}
unsafe impl Sync for CoreInputViewOcclusionsChangedEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreInputViewShowingEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreInputViewShowingEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CoreInputViewShowingEventArgs {
    pub fn TryCancel(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TryCancel)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CoreInputViewShowingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreInputViewShowingEventArgs>();
}
unsafe impl windows_core::Interface for CoreInputViewShowingEventArgs {
    type Vtable = <ICoreInputViewShowingEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICoreInputViewShowingEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreInputViewShowingEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewShowingEventArgs";
}
unsafe impl Send for CoreInputViewShowingEventArgs {}
unsafe impl Sync for CoreInputViewShowingEventArgs {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CoreInputViewTransferringXYFocusEventArgs(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(CoreInputViewTransferringXYFocusEventArgs, windows_core::IUnknown, windows_core::IInspectable);
impl CoreInputViewTransferringXYFocusEventArgs {
    pub fn Origin(&self) -> windows_core::Result<super::super::super::Foundation::Rect> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Origin)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn Direction(&self) -> windows_core::Result<CoreInputViewXYFocusTransferDirection> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Direction)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetTransferHandled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTransferHandled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn TransferHandled(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).TransferHandled)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn SetKeepPrimaryViewVisible(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetKeepPrimaryViewVisible)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn KeepPrimaryViewVisible(&self) -> windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).KeepPrimaryViewVisible)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
}
impl windows_core::RuntimeType for CoreInputViewTransferringXYFocusEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, ICoreInputViewTransferringXYFocusEventArgs>();
}
unsafe impl windows_core::Interface for CoreInputViewTransferringXYFocusEventArgs {
    type Vtable = <ICoreInputViewTransferringXYFocusEventArgs as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <ICoreInputViewTransferringXYFocusEventArgs as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for CoreInputViewTransferringXYFocusEventArgs {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.CoreInputViewTransferringXYFocusEventArgs";
}
unsafe impl Send for CoreInputViewTransferringXYFocusEventArgs {}
unsafe impl Sync for CoreInputViewTransferringXYFocusEventArgs {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CoreInputViewXYFocusTransferDirection(pub i32);
impl CoreInputViewXYFocusTransferDirection {
    pub const Up: Self = Self(0i32);
    pub const Right: Self = Self(1i32);
    pub const Down: Self = Self(2i32);
    pub const Left: Self = Self(3i32);
}
impl windows_core::TypeKind for CoreInputViewXYFocusTransferDirection {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for CoreInputViewXYFocusTransferDirection {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.ViewManagement.Core.CoreInputViewXYFocusTransferDirection;i4)");
}
windows_core::imp::define_interface!(ICoreFrameworkInputView, ICoreFrameworkInputView_Vtbl, 0xb6c1c06b_08eb_5043_9f5e_d25882a10412);
impl windows_core::RuntimeType for ICoreFrameworkInputView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFrameworkInputView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PrimaryViewAnimationStarting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePrimaryViewAnimationStarting: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub OcclusionsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveOcclusionsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreFrameworkInputViewAnimationStartingEventArgs, ICoreFrameworkInputViewAnimationStartingEventArgs_Vtbl, 0xa8b123ac_0089_53da_8be0_2576d13503b8);
impl windows_core::RuntimeType for ICoreFrameworkInputViewAnimationStartingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFrameworkInputViewAnimationStartingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Occlusions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub FrameworkAnimationRecommended: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub AnimationDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreFrameworkInputViewOcclusionsChangedEventArgs, ICoreFrameworkInputViewOcclusionsChangedEventArgs_Vtbl, 0x3a3151e5_f43f_5039_8978_9ad5488606a6);
impl windows_core::RuntimeType for ICoreFrameworkInputViewOcclusionsChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFrameworkInputViewOcclusionsChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Occlusions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreFrameworkInputViewStatics, ICoreFrameworkInputViewStatics_Vtbl, 0x0514607e_9ca3_510c_8b08_c757d124249f);
impl windows_core::RuntimeType for ICoreFrameworkInputViewStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreFrameworkInputViewStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForUIContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputView, ICoreInputView_Vtbl, 0xc770cd7a_7001_4c32_bf94_25c1f554cbf1);
impl windows_core::RuntimeType for ICoreInputView {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputView_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OcclusionsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveOcclusionsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub GetCoreInputViewOcclusions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub TryShowPrimaryView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub TryHidePrimaryView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputView2, ICoreInputView2_Vtbl, 0xf90bd4ec_e7a6_5ebd_ac19_f4b899c78eef);
impl windows_core::RuntimeType for ICoreInputView2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputView2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub XYFocusTransferringFromPrimaryView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveXYFocusTransferringFromPrimaryView: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub XYFocusTransferredToPrimaryView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveXYFocusTransferredToPrimaryView: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub TryTransferXYFocusToPrimaryView: unsafe extern "system" fn(*mut core::ffi::c_void, super::super::super::Foundation::Rect, CoreInputViewXYFocusTransferDirection, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputView3, ICoreInputView3_Vtbl, 0xbfc7e55a_c203_54be_9d13_4e84a6b6f194);
impl windows_core::RuntimeType for ICoreInputView3 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputView3_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryShow: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub TryShowWithKind: unsafe extern "system" fn(*mut core::ffi::c_void, CoreInputViewKind, *mut bool) -> windows_core::HRESULT,
    pub TryHide: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputView4, ICoreInputView4_Vtbl, 0x2a2fb4df_7876_53bd_baa3_9579c0f960d6);
impl windows_core::RuntimeType for ICoreInputView4 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputView4_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub PrimaryViewShowing: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePrimaryViewShowing: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PrimaryViewHiding: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePrimaryViewHiding: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputView5, ICoreInputView5_Vtbl, 0x4334d4bf_3650_52dd_bd1f_1e93581ae122);
impl windows_core::RuntimeType for ICoreInputView5 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputView5_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsKindSupported: unsafe extern "system" fn(*mut core::ffi::c_void, CoreInputViewKind, *mut bool) -> windows_core::HRESULT,
    pub SupportedKindsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemoveSupportedKindsChanged: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
    pub PrimaryViewAnimationStarting: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut i64) -> windows_core::HRESULT,
    pub RemovePrimaryViewAnimationStarting: unsafe extern "system" fn(*mut core::ffi::c_void, i64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputViewAnimationStartingEventArgs, ICoreInputViewAnimationStartingEventArgs_Vtbl, 0xa9144af2_b55c_5ea1_b8ab_5340f3e94897);
impl windows_core::RuntimeType for ICoreInputViewAnimationStartingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewAnimationStartingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Occlusions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub AnimationDuration: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::TimeSpan) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputViewHidingEventArgs, ICoreInputViewHidingEventArgs_Vtbl, 0xeada47bd_bac5_5336_848d_41083584daad);
impl windows_core::RuntimeType for ICoreInputViewHidingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewHidingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryCancel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputViewOcclusion, ICoreInputViewOcclusion_Vtbl, 0xcc36ce06_3865_4177_b5f5_8b65e0b9ce84);
impl windows_core::RuntimeType for ICoreInputViewOcclusion {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewOcclusion_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub OccludingRect: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub OcclusionKind: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreInputViewOcclusionKind) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputViewOcclusionsChangedEventArgs, ICoreInputViewOcclusionsChangedEventArgs_Vtbl, 0x6e8b34dc_549f_5f98_8e12_de4c64792ce3);
impl windows_core::RuntimeType for ICoreInputViewOcclusionsChangedEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewOcclusionsChangedEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Occlusions: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Handled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputViewShowingEventArgs, ICoreInputViewShowingEventArgs_Vtbl, 0xc2f2615b_ea85_546d_be8b_ccf7712d678a);
impl windows_core::RuntimeType for ICoreInputViewShowingEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewShowingEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub TryCancel: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputViewStatics, ICoreInputViewStatics_Vtbl, 0x7d9b97cd_edbe_49cf_a54f_337de052907f);
impl windows_core::RuntimeType for ICoreInputViewStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForCurrentView: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputViewStatics2, ICoreInputViewStatics2_Vtbl, 0xa412d927_5106_5050_a60c_3aff2ee30e8f);
impl windows_core::RuntimeType for ICoreInputViewStatics2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewStatics2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub GetForUIContext: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(ICoreInputViewTransferringXYFocusEventArgs, ICoreInputViewTransferringXYFocusEventArgs_Vtbl, 0x4074866e_a536_5c40_a3ea_c8c6ee0edc30);
impl windows_core::RuntimeType for ICoreInputViewTransferringXYFocusEventArgs {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct ICoreInputViewTransferringXYFocusEventArgs_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Origin: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::Rect) -> windows_core::HRESULT,
    pub Direction: unsafe extern "system" fn(*mut core::ffi::c_void, *mut CoreInputViewXYFocusTransferDirection) -> windows_core::HRESULT,
    pub SetTransferHandled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub TransferHandled: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub SetKeepPrimaryViewVisible: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub KeepPrimaryViewVisible: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUISettingsController, IUISettingsController_Vtbl, 0xc086a0a1_b911_54f3_829a_4f5cdab0ed66);
impl windows_core::RuntimeType for IUISettingsController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettingsController_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub SetAdvancedEffectsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SetAnimationsEnabled: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SetAutoHideScrollBars: unsafe extern "system" fn(*mut core::ffi::c_void, bool) -> windows_core::HRESULT,
    pub SetMessageDuration: unsafe extern "system" fn(*mut core::ffi::c_void, u32) -> windows_core::HRESULT,
    pub SetTextScaleFactor: unsafe extern "system" fn(*mut core::ffi::c_void, f64) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IUISettingsControllerStatics, IUISettingsControllerStatics_Vtbl, 0xeb3c68cc_c220_578c_8119_7db324ed26a6);
impl windows_core::RuntimeType for IUISettingsControllerStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IUISettingsControllerStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub RequestDefaultAsync: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UISettingsController(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(UISettingsController, windows_core::IUnknown, windows_core::IInspectable);
impl UISettingsController {
    pub fn SetAdvancedEffectsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAdvancedEffectsEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetAnimationsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAnimationsEnabled)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetAutoHideScrollBars(&self, value: bool) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetAutoHideScrollBars)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetMessageDuration(&self, value: u32) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetMessageDuration)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn SetTextScaleFactor(&self, value: f64) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTextScaleFactor)(windows_core::Interface::as_raw(this), value).ok() }
    }
    pub fn RequestDefaultAsync() -> windows_core::Result<windows_future::IAsyncOperation<UISettingsController>> {
        Self::IUISettingsControllerStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).RequestDefaultAsync)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IUISettingsControllerStatics<R, F: FnOnce(&IUISettingsControllerStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<UISettingsController, IUISettingsControllerStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for UISettingsController {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IUISettingsController>();
}
unsafe impl windows_core::Interface for UISettingsController {
    type Vtable = <IUISettingsController as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IUISettingsController as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for UISettingsController {
    const NAME: &'static str = "Windows.UI.ViewManagement.Core.UISettingsController";
}
unsafe impl Send for UISettingsController {}
unsafe impl Sync for UISettingsController {}
