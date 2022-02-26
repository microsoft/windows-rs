#[cfg(feature = "ApplicationModel_Activation")]
pub trait IApplicationOverrides_Impl: Sized {
    fn OnActivated(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::IActivatedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnLaunched(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::LaunchActivatedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnFileActivated(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::FileActivatedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnSearchActivated(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::SearchActivatedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnShareTargetActivated(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::ShareTargetActivatedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnFileOpenPickerActivated(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::FileOpenPickerActivatedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnFileSavePickerActivated(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::FileSavePickerActivatedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnCachedFileUpdaterActivated(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::CachedFileUpdaterActivatedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnWindowCreated(&self, args: &::core::option::Option<WindowCreatedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for IApplicationOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.IApplicationOverrides";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl IApplicationOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationOverrides_Impl, const OFFSET: isize>() -> IApplicationOverrides_Vtbl {
        unsafe extern "system" fn OnActivated<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnActivated(::core::mem::transmute(&args)).into()
        }
        unsafe extern "system" fn OnLaunched<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnLaunched(::core::mem::transmute(&args)).into()
        }
        unsafe extern "system" fn OnFileActivated<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnFileActivated(::core::mem::transmute(&args)).into()
        }
        unsafe extern "system" fn OnSearchActivated<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnSearchActivated(::core::mem::transmute(&args)).into()
        }
        unsafe extern "system" fn OnShareTargetActivated<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnShareTargetActivated(::core::mem::transmute(&args)).into()
        }
        unsafe extern "system" fn OnFileOpenPickerActivated<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnFileOpenPickerActivated(::core::mem::transmute(&args)).into()
        }
        unsafe extern "system" fn OnFileSavePickerActivated<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnFileSavePickerActivated(::core::mem::transmute(&args)).into()
        }
        unsafe extern "system" fn OnCachedFileUpdaterActivated<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnCachedFileUpdaterActivated(::core::mem::transmute(&args)).into()
        }
        unsafe extern "system" fn OnWindowCreated<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnWindowCreated(::core::mem::transmute(&args)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationOverrides, OFFSET>(),
            OnActivated: OnActivated::<Identity, Impl, OFFSET>,
            OnLaunched: OnLaunched::<Identity, Impl, OFFSET>,
            OnFileActivated: OnFileActivated::<Identity, Impl, OFFSET>,
            OnSearchActivated: OnSearchActivated::<Identity, Impl, OFFSET>,
            OnShareTargetActivated: OnShareTargetActivated::<Identity, Impl, OFFSET>,
            OnFileOpenPickerActivated: OnFileOpenPickerActivated::<Identity, Impl, OFFSET>,
            OnFileSavePickerActivated: OnFileSavePickerActivated::<Identity, Impl, OFFSET>,
            OnCachedFileUpdaterActivated: OnCachedFileUpdaterActivated::<Identity, Impl, OFFSET>,
            OnWindowCreated: OnWindowCreated::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
pub trait IApplicationOverrides2_Impl: Sized {
    fn OnBackgroundActivated(&self, args: &::core::option::Option<super::super::ApplicationModel::Activation::BackgroundActivatedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for IApplicationOverrides2 {
    const NAME: &'static str = "Windows.UI.Xaml.IApplicationOverrides2";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl IApplicationOverrides2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationOverrides2_Impl, const OFFSET: isize>() -> IApplicationOverrides2_Vtbl {
        unsafe extern "system" fn OnBackgroundActivated<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationOverrides2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnBackgroundActivated(::core::mem::transmute(&args)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationOverrides2, OFFSET>(),
            OnBackgroundActivated: OnBackgroundActivated::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationOverrides2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Controls")]
pub trait IDataTemplateExtension_Impl: Sized {
    fn ResetTemplate(&self) -> ::windows::core::Result<()>;
    fn ProcessBinding(&self, phase: u32) -> ::windows::core::Result<bool>;
    fn ProcessBindings(&self, arg: &::core::option::Option<Controls::ContainerContentChangingEventArgs>) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "UI_Xaml_Controls")]
impl ::windows::core::RuntimeName for IDataTemplateExtension {
    const NAME: &'static str = "Windows.UI.Xaml.IDataTemplateExtension";
}
#[cfg(feature = "UI_Xaml_Controls")]
impl IDataTemplateExtension_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataTemplateExtension_Impl, const OFFSET: isize>() -> IDataTemplateExtension_Vtbl {
        unsafe extern "system" fn ResetTemplate<Identity: ::windows::core::IUnknownImpl, Impl: IDataTemplateExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).ResetTemplate().into()
        }
        unsafe extern "system" fn ProcessBinding<Identity: ::windows::core::IUnknownImpl, Impl: IDataTemplateExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phase: u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProcessBinding(phase) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessBindings<Identity: ::windows::core::IUnknownImpl, Impl: IDataTemplateExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, arg: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProcessBindings(::core::mem::transmute(&arg)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataTemplateExtension, OFFSET>(),
            ResetTemplate: ResetTemplate::<Identity, Impl, OFFSET>,
            ProcessBinding: ProcessBinding::<Identity, Impl, OFFSET>,
            ProcessBindings: ProcessBindings::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataTemplateExtension as ::windows::core::Interface>::IID
    }
}
pub trait IElementFactory_Impl: Sized {
    fn GetElement(&self, args: &::core::option::Option<ElementFactoryGetArgs>) -> ::windows::core::Result<UIElement>;
    fn RecycleElement(&self, args: &::core::option::Option<ElementFactoryRecycleArgs>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IElementFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IElementFactory";
}
impl IElementFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IElementFactory_Impl, const OFFSET: isize>() -> IElementFactory_Vtbl {
        unsafe extern "system" fn GetElement<Identity: ::windows::core::IUnknownImpl, Impl: IElementFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetElement(::core::mem::transmute(&args)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecycleElement<Identity: ::windows::core::IUnknownImpl, Impl: IElementFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RecycleElement(::core::mem::transmute(&args)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IElementFactory, OFFSET>(),
            GetElement: GetElement::<Identity, Impl, OFFSET>,
            RecycleElement: RecycleElement::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IElementFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IFrameworkElementOverrides_Impl: Sized {
    fn MeasureOverride(&self, availablesize: &super::super::Foundation::Size) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn ArrangeOverride(&self, finalsize: &super::super::Foundation::Size) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn OnApplyTemplate(&self) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IFrameworkElementOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementOverrides";
}
#[cfg(feature = "Foundation")]
impl IFrameworkElementOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElementOverrides_Impl, const OFFSET: isize>() -> IFrameworkElementOverrides_Vtbl {
        unsafe extern "system" fn MeasureOverride<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElementOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, availablesize: super::super::Foundation::Size, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MeasureOverride(::core::mem::transmute(&availablesize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrangeOverride<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElementOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalsize: super::super::Foundation::Size, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ArrangeOverride(::core::mem::transmute(&finalsize)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnApplyTemplate<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElementOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnApplyTemplate().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkElementOverrides, OFFSET>(),
            MeasureOverride: MeasureOverride::<Identity, Impl, OFFSET>,
            ArrangeOverride: ArrangeOverride::<Identity, Impl, OFFSET>,
            OnApplyTemplate: OnApplyTemplate::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkElementOverrides as ::windows::core::Interface>::IID
    }
}
pub trait IFrameworkElementOverrides2_Impl: Sized {
    fn GoToElementStateCore(&self, statename: &::windows::core::HSTRING, usetransitions: bool) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IFrameworkElementOverrides2 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementOverrides2";
}
impl IFrameworkElementOverrides2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElementOverrides2_Impl, const OFFSET: isize>() -> IFrameworkElementOverrides2_Vtbl {
        unsafe extern "system" fn GoToElementStateCore<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElementOverrides2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, usetransitions: bool, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GoToElementStateCore(::core::mem::transmute(&statename), usetransitions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkElementOverrides2, OFFSET>(),
            GoToElementStateCore: GoToElementStateCore::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkElementOverrides2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Automation_Peers"))]
pub trait IUIElementOverrides_Impl: Sized {
    fn OnCreateAutomationPeer(&self) -> ::windows::core::Result<Automation::Peers::AutomationPeer>;
    fn OnDisconnectVisualChildren(&self) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn FindSubElementsForTouchTargeting(&self, point: &super::super::Foundation::Point, boundingrect: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IIterable<super::super::Foundation::Point>>>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Automation_Peers"))]
impl ::windows::core::RuntimeName for IUIElementOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementOverrides";
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Automation_Peers"))]
impl IUIElementOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementOverrides_Impl, const OFFSET: isize>() -> IUIElementOverrides_Vtbl {
        unsafe extern "system" fn OnCreateAutomationPeer<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OnCreateAutomationPeer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDisconnectVisualChildren<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnDisconnectVisualChildren().into()
        }
        unsafe extern "system" fn FindSubElementsForTouchTargeting<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, point: super::super::Foundation::Point, boundingrect: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).FindSubElementsForTouchTargeting(::core::mem::transmute(&point), ::core::mem::transmute(&boundingrect)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElementOverrides, OFFSET>(),
            OnCreateAutomationPeer: OnCreateAutomationPeer::<Identity, Impl, OFFSET>,
            OnDisconnectVisualChildren: OnDisconnectVisualChildren::<Identity, Impl, OFFSET>,
            FindSubElementsForTouchTargeting: FindSubElementsForTouchTargeting::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElementOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Input"))]
pub trait IUIElementOverrides7_Impl: Sized {
    fn GetChildrenInTabFocusOrder(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<DependencyObject>>;
    fn OnProcessKeyboardAccelerators(&self, args: &::core::option::Option<Input::ProcessKeyboardAcceleratorEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Input"))]
impl ::windows::core::RuntimeName for IUIElementOverrides7 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementOverrides7";
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Input"))]
impl IUIElementOverrides7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementOverrides7_Impl, const OFFSET: isize>() -> IUIElementOverrides7_Vtbl {
        unsafe extern "system" fn GetChildrenInTabFocusOrder<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementOverrides7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetChildrenInTabFocusOrder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnProcessKeyboardAccelerators<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementOverrides7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnProcessKeyboardAccelerators(::core::mem::transmute(&args)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElementOverrides7, OFFSET>(),
            GetChildrenInTabFocusOrder: GetChildrenInTabFocusOrder::<Identity, Impl, OFFSET>,
            OnProcessKeyboardAccelerators: OnProcessKeyboardAccelerators::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElementOverrides7 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Input")]
pub trait IUIElementOverrides8_Impl: Sized {
    fn OnKeyboardAcceleratorInvoked(&self, args: &::core::option::Option<Input::KeyboardAcceleratorInvokedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnBringIntoViewRequested(&self, e: &::core::option::Option<BringIntoViewRequestedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
#[cfg(feature = "UI_Xaml_Input")]
impl ::windows::core::RuntimeName for IUIElementOverrides8 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementOverrides8";
}
#[cfg(feature = "UI_Xaml_Input")]
impl IUIElementOverrides8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementOverrides8_Impl, const OFFSET: isize>() -> IUIElementOverrides8_Vtbl {
        unsafe extern "system" fn OnKeyboardAcceleratorInvoked<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementOverrides8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnKeyboardAcceleratorInvoked(::core::mem::transmute(&args)).into()
        }
        unsafe extern "system" fn OnBringIntoViewRequested<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementOverrides8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).OnBringIntoViewRequested(::core::mem::transmute(&e)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElementOverrides8, OFFSET>(),
            OnKeyboardAcceleratorInvoked: OnKeyboardAcceleratorInvoked::<Identity, Impl, OFFSET>,
            OnBringIntoViewRequested: OnBringIntoViewRequested::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElementOverrides8 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Composition")]
pub trait IUIElementOverrides9_Impl: Sized {
    fn PopulatePropertyInfoOverride(&self, propertyname: &::windows::core::HSTRING, animationpropertyinfo: &::core::option::Option<super::Composition::AnimationPropertyInfo>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
#[cfg(feature = "UI_Composition")]
impl ::windows::core::RuntimeName for IUIElementOverrides9 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementOverrides9";
}
#[cfg(feature = "UI_Composition")]
impl IUIElementOverrides9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementOverrides9_Impl, const OFFSET: isize>() -> IUIElementOverrides9_Vtbl {
        unsafe extern "system" fn PopulatePropertyInfoOverride<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementOverrides9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, animationpropertyinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).PopulatePropertyInfoOverride(::core::mem::transmute(&propertyname), ::core::mem::transmute(&animationpropertyinfo)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElementOverrides9, OFFSET>(),
            PopulatePropertyInfoOverride: PopulatePropertyInfoOverride::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElementOverrides9 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Controls")]
pub trait IVisualStateManagerOverrides_Impl: Sized {
    fn GoToStateCore(&self, control: &::core::option::Option<Controls::Control>, templateroot: &::core::option::Option<FrameworkElement>, statename: &::windows::core::HSTRING, group: &::core::option::Option<VisualStateGroup>, state: &::core::option::Option<VisualState>, usetransitions: bool) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "UI_Xaml_Controls")]
impl ::windows::core::RuntimeName for IVisualStateManagerOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.IVisualStateManagerOverrides";
}
#[cfg(feature = "UI_Xaml_Controls")]
impl IVisualStateManagerOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualStateManagerOverrides_Impl, const OFFSET: isize>() -> IVisualStateManagerOverrides_Vtbl {
        unsafe extern "system" fn GoToStateCore<Identity: ::windows::core::IUnknownImpl, Impl: IVisualStateManagerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, control: ::windows::core::RawPtr, templateroot: ::windows::core::RawPtr, statename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, group: ::windows::core::RawPtr, state: ::windows::core::RawPtr, usetransitions: bool, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GoToStateCore(::core::mem::transmute(&control), ::core::mem::transmute(&templateroot), ::core::mem::transmute(&statename), ::core::mem::transmute(&group), ::core::mem::transmute(&state), usetransitions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualStateManagerOverrides, OFFSET>(),
            GoToStateCore: GoToStateCore::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualStateManagerOverrides as ::windows::core::Interface>::IID
    }
}
