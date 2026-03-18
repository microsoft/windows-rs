#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppTaskContent(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppTaskContent, windows_core::IUnknown, windows_core::IInspectable);
impl AppTaskContent {
    pub fn AddButton<P1>(&self, text: &windows_core::HSTRING, actionuri: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).AddButton)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), actionuri.param().abi()).ok() }
    }
    pub fn SetTextInput(&self, placeholdertext: &windows_core::HSTRING, actionuritemplate: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetTextInput)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(placeholdertext), core::mem::transmute_copy(actionuritemplate)).ok() }
    }
    pub fn SetQuestion(&self, question: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).SetQuestion)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(question)).ok() }
    }
    pub fn CreateSequenceOfSteps(completedsteps: &[windows_core::HSTRING], executingstep: &windows_core::HSTRING) -> windows_core::Result<AppTaskContent> {
        Self::IAppTaskContentStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateSequenceOfSteps)(windows_core::Interface::as_raw(this), completedsteps.len().try_into().unwrap(), core::mem::transmute(completedsteps.as_ptr()), core::mem::transmute_copy(executingstep), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreatePreviewThumbnail<P0>(imageuri: P0, executingstep: &windows_core::HSTRING) -> windows_core::Result<AppTaskContent>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        Self::IAppTaskContentStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreatePreviewThumbnail)(windows_core::Interface::as_raw(this), imageuri.param().abi(), core::mem::transmute_copy(executingstep), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateTextSummaryResult(text: &windows_core::HSTRING) -> windows_core::Result<AppTaskContent> {
        Self::IAppTaskContentStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateTextSummaryResult)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(text), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn CreateGeneratedAssetsResult(assets: &[Option<AppTaskResultAsset>]) -> windows_core::Result<AppTaskContent> {
        Self::IAppTaskContentStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateGeneratedAssetsResult)(windows_core::Interface::as_raw(this), assets.len().try_into().unwrap(), core::mem::transmute(assets.as_ptr()), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    pub fn MaxButtons() -> windows_core::Result<u32> {
        Self::IAppTaskContentStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).MaxButtons)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    fn IAppTaskContentStatics<R, F: FnOnce(&IAppTaskContentStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppTaskContent, IAppTaskContentStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AppTaskContent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppTaskContent>();
}
unsafe impl windows_core::Interface for AppTaskContent {
    type Vtable = <IAppTaskContent as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppTaskContent as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppTaskContent {
    const NAME: &'static str = "Windows.UI.Shell.Tasks.AppTaskContent";
}
unsafe impl Send for AppTaskContent {}
unsafe impl Sync for AppTaskContent {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppTaskInfo(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppTaskInfo, windows_core::IUnknown, windows_core::IInspectable);
impl AppTaskInfo {
    pub fn Remove(&self) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Remove)(windows_core::Interface::as_raw(this)).ok() }
    }
    pub fn Update<P1>(&self, state: AppTaskState, content: P1) -> windows_core::Result<()>
    where
        P1: windows_core::Param<AppTaskContent>,
    {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).Update)(windows_core::Interface::as_raw(this), state, content.param().abi()).ok() }
    }
    pub fn UpdateState(&self, state: AppTaskState) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UpdateState)(windows_core::Interface::as_raw(this), state).ok() }
    }
    pub fn UpdateTitles(&self, title: &windows_core::HSTRING, subtitle: &windows_core::HSTRING) -> windows_core::Result<()> {
        let this = self;
        unsafe { (windows_core::Interface::vtable(this).UpdateTitles)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(title), core::mem::transmute_copy(subtitle)).ok() }
    }
    pub fn GetCompletedSteps(&self) -> windows_core::Result<windows_core::Array<windows_core::HSTRING>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).GetCompletedSteps)(windows_core::Interface::as_raw(this), windows_core::Array::<windows_core::HSTRING>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        }
    }
    pub fn GetExecutingStep(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).GetExecutingStep)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Title)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn Subtitle(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Subtitle)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn DeepLink(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).DeepLink)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn IconUri(&self) -> windows_core::Result<super::super::super::Foundation::Uri> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IconUri)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn State(&self) -> windows_core::Result<AppTaskState> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).State)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn StartTime(&self) -> windows_core::Result<super::super::super::Foundation::DateTime> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).StartTime)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn EndTime(&self) -> windows_core::Result<super::super::super::Foundation::IReference<super::super::super::Foundation::DateTime>> {
        let this = self;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).EndTime)(windows_core::Interface::as_raw(this), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        }
    }
    pub fn Id(&self) -> windows_core::Result<windows_core::HSTRING> {
        let this = &windows_core::Interface::cast::<IAppTaskInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Id)(windows_core::Interface::as_raw(this), &mut result__).map(|| core::mem::transmute(result__))
        }
    }
    pub fn HiddenByUser(&self) -> windows_core::Result<bool> {
        let this = &windows_core::Interface::cast::<IAppTaskInfo2>(self)?;
        unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).HiddenByUser)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        }
    }
    pub fn UpdateDeepLink<P0>(&self, deeplink: P0) -> windows_core::Result<()>
    where
        P0: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        let this = &windows_core::Interface::cast::<IAppTaskInfo2>(self)?;
        unsafe { (windows_core::Interface::vtable(this).UpdateDeepLink)(windows_core::Interface::as_raw(this), deeplink.param().abi()).ok() }
    }
    pub fn IsSupported() -> windows_core::Result<bool> {
        Self::IAppTaskInfoStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).IsSupported)(windows_core::Interface::as_raw(this), &mut result__).map(|| result__)
        })
    }
    pub fn FindAll() -> windows_core::Result<windows_core::Array<AppTaskInfo>> {
        Self::IAppTaskInfoStatics(|this| unsafe {
            let mut result__ = core::mem::MaybeUninit::zeroed();
            (windows_core::Interface::vtable(this).FindAll)(windows_core::Interface::as_raw(this), windows_core::Array::<AppTaskInfo>::set_abi_len(core::mem::transmute(&mut result__)), result__.as_mut_ptr() as *mut _ as _).map(|| result__.assume_init())
        })
    }
    pub fn Create<P2, P3, P4>(title: &windows_core::HSTRING, subtitle: &windows_core::HSTRING, deeplink: P2, iconuri: P3, content: P4) -> windows_core::Result<AppTaskInfo>
    where
        P2: windows_core::Param<super::super::super::Foundation::Uri>,
        P3: windows_core::Param<super::super::super::Foundation::Uri>,
        P4: windows_core::Param<AppTaskContent>,
    {
        Self::IAppTaskInfoStatics(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).Create)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(title), core::mem::transmute_copy(subtitle), deeplink.param().abi(), iconuri.param().abi(), content.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IAppTaskInfoStatics<R, F: FnOnce(&IAppTaskInfoStatics) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppTaskInfo, IAppTaskInfoStatics> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AppTaskInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppTaskInfo>();
}
unsafe impl windows_core::Interface for AppTaskInfo {
    type Vtable = <IAppTaskInfo as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppTaskInfo as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppTaskInfo {
    const NAME: &'static str = "Windows.UI.Shell.Tasks.AppTaskInfo";
}
unsafe impl Send for AppTaskInfo {}
unsafe impl Sync for AppTaskInfo {}
#[repr(transparent)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppTaskResultAsset(windows_core::IUnknown);
windows_core::imp::interface_hierarchy!(AppTaskResultAsset, windows_core::IUnknown, windows_core::IInspectable);
impl AppTaskResultAsset {
    pub fn CreateInstance<P2, P3>(name: &windows_core::HSTRING, context: &windows_core::HSTRING, iconuri: P2, asseturi: P3) -> windows_core::Result<AppTaskResultAsset>
    where
        P2: windows_core::Param<super::super::super::Foundation::Uri>,
        P3: windows_core::Param<super::super::super::Foundation::Uri>,
    {
        Self::IAppTaskResultAssetFactory(|this| unsafe {
            let mut result__ = core::mem::zeroed();
            (windows_core::Interface::vtable(this).CreateInstance)(windows_core::Interface::as_raw(this), core::mem::transmute_copy(name), core::mem::transmute_copy(context), iconuri.param().abi(), asseturi.param().abi(), &mut result__).and_then(|| windows_core::Type::from_abi(result__))
        })
    }
    fn IAppTaskResultAssetFactory<R, F: FnOnce(&IAppTaskResultAssetFactory) -> windows_core::Result<R>>(callback: F) -> windows_core::Result<R> {
        static SHARED: windows_core::imp::FactoryCache<AppTaskResultAsset, IAppTaskResultAssetFactory> = windows_core::imp::FactoryCache::new();
        SHARED.call(callback)
    }
}
impl windows_core::RuntimeType for AppTaskResultAsset {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_class::<Self, IAppTaskResultAsset>();
}
unsafe impl windows_core::Interface for AppTaskResultAsset {
    type Vtable = <IAppTaskResultAsset as windows_core::Interface>::Vtable;
    const IID: windows_core::GUID = <IAppTaskResultAsset as windows_core::Interface>::IID;
}
impl windows_core::RuntimeName for AppTaskResultAsset {
    const NAME: &'static str = "Windows.UI.Shell.Tasks.AppTaskResultAsset";
}
unsafe impl Send for AppTaskResultAsset {}
unsafe impl Sync for AppTaskResultAsset {}
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct AppTaskState(pub i32);
impl AppTaskState {
    pub const Running: Self = Self(0i32);
    pub const Completed: Self = Self(1i32);
    pub const NeedsAttention: Self = Self(2i32);
    pub const Paused: Self = Self(3i32);
    pub const Error: Self = Self(4i32);
}
impl windows_core::TypeKind for AppTaskState {
    type TypeKind = windows_core::CopyType;
}
impl windows_core::RuntimeType for AppTaskState {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::from_slice(b"enum(Windows.UI.Shell.Tasks.AppTaskState;i4)");
}
windows_core::imp::define_interface!(IAppTaskContent, IAppTaskContent_Vtbl, 0x2411bf59_1b2d_5b63_8181_03d6c2248a68);
impl windows_core::RuntimeType for IAppTaskContent {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppTaskContent_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub AddButton: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetTextInput: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub SetQuestion: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppTaskContentStatics, IAppTaskContentStatics_Vtbl, 0xaabd19f6_7afc_5b1b_94f6_5dc9dc9af9e7);
impl windows_core::RuntimeType for IAppTaskContentStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppTaskContentStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateSequenceOfSteps: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const windows_core::HSTRING, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreatePreviewThumbnail: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateTextSummaryResult: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub CreateGeneratedAssetsResult: unsafe extern "system" fn(*mut core::ffi::c_void, u32, *const AppTaskResultAsset, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub MaxButtons: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppTaskInfo, IAppTaskInfo_Vtbl, 0x6720eed6_435b_5db9_be66_9343b70654f7);
impl windows_core::RuntimeType for IAppTaskInfo {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppTaskInfo_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Remove: unsafe extern "system" fn(*mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Update: unsafe extern "system" fn(*mut core::ffi::c_void, AppTaskState, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub UpdateState: unsafe extern "system" fn(*mut core::ffi::c_void, AppTaskState) -> windows_core::HRESULT,
    pub UpdateTitles: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetCompletedSteps: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub GetExecutingStep: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Title: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Subtitle: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub DeepLink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub IconUri: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub State: unsafe extern "system" fn(*mut core::ffi::c_void, *mut AppTaskState) -> windows_core::HRESULT,
    pub StartTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut super::super::super::Foundation::DateTime) -> windows_core::HRESULT,
    pub EndTime: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppTaskInfo2, IAppTaskInfo2_Vtbl, 0xad724d71_f137_51c0_8111_3552436bf447);
impl windows_core::RuntimeType for IAppTaskInfo2 {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppTaskInfo2_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub Id: unsafe extern "system" fn(*mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub HiddenByUser: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub UpdateDeepLink: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppTaskInfoStatics, IAppTaskInfoStatics_Vtbl, 0xa0b0434f_c640_5800_88c9_d7691ac2f48f);
impl windows_core::RuntimeType for IAppTaskInfoStatics {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppTaskInfoStatics_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub IsSupported: unsafe extern "system" fn(*mut core::ffi::c_void, *mut bool) -> windows_core::HRESULT,
    pub FindAll: unsafe extern "system" fn(*mut core::ffi::c_void, *mut u32, *mut *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
    pub Create: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
windows_core::imp::define_interface!(IAppTaskResultAsset, IAppTaskResultAsset_Vtbl, 0x75d0c2b3_8a31_5f8f_bda4_bdca96e95532);
impl windows_core::RuntimeType for IAppTaskResultAsset {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppTaskResultAsset_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
}
windows_core::imp::define_interface!(IAppTaskResultAssetFactory, IAppTaskResultAssetFactory_Vtbl, 0x0334d9df_0e06_5999_ba41_85d72e980085);
impl windows_core::RuntimeType for IAppTaskResultAssetFactory {
    const SIGNATURE: windows_core::imp::ConstBuffer = windows_core::imp::ConstBuffer::for_interface::<Self>();
}
#[repr(C)]
#[doc(hidden)]
pub struct IAppTaskResultAssetFactory_Vtbl {
    pub base__: windows_core::IInspectable_Vtbl,
    pub CreateInstance: unsafe extern "system" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void, *mut *mut core::ffi::c_void) -> windows_core::HRESULT,
}
