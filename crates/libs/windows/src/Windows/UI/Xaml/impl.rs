#[cfg(feature = "ApplicationModel_Activation")]
pub trait IApplicationOverrides_Impl: Sized {
    fn OnActivated(&mut self, args: &::core::option::Option<super::super::ApplicationModel::Activation::IActivatedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnLaunched(&mut self, args: &::core::option::Option<super::super::ApplicationModel::Activation::LaunchActivatedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnFileActivated(&mut self, args: &::core::option::Option<super::super::ApplicationModel::Activation::FileActivatedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnSearchActivated(&mut self, args: &::core::option::Option<super::super::ApplicationModel::Activation::SearchActivatedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnShareTargetActivated(&mut self, args: &::core::option::Option<super::super::ApplicationModel::Activation::ShareTargetActivatedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnFileOpenPickerActivated(&mut self, args: &::core::option::Option<super::super::ApplicationModel::Activation::FileOpenPickerActivatedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnFileSavePickerActivated(&mut self, args: &::core::option::Option<super::super::ApplicationModel::Activation::FileSavePickerActivatedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnCachedFileUpdaterActivated(&mut self, args: &::core::option::Option<super::super::ApplicationModel::Activation::CachedFileUpdaterActivatedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnWindowCreated(&mut self, args: &::core::option::Option<WindowCreatedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for IApplicationOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.IApplicationOverrides";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl IApplicationOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationOverrides_Vtbl {
        unsafe extern "system" fn OnActivated<Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnActivated(&*(&args as *const <super::super::ApplicationModel::Activation::IActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::IActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnLaunched<Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnLaunched(&*(&args as *const <super::super::ApplicationModel::Activation::LaunchActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::LaunchActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnFileActivated<Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnFileActivated(&*(&args as *const <super::super::ApplicationModel::Activation::FileActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::FileActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnSearchActivated<Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnSearchActivated(&*(&args as *const <super::super::ApplicationModel::Activation::SearchActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::SearchActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnShareTargetActivated<Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnShareTargetActivated(&*(&args as *const <super::super::ApplicationModel::Activation::ShareTargetActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::ShareTargetActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnFileOpenPickerActivated<Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnFileOpenPickerActivated(&*(&args as *const <super::super::ApplicationModel::Activation::FileOpenPickerActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::FileOpenPickerActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnFileSavePickerActivated<Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnFileSavePickerActivated(&*(&args as *const <super::super::ApplicationModel::Activation::FileSavePickerActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::FileSavePickerActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnCachedFileUpdaterActivated<Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnCachedFileUpdaterActivated(&*(&args as *const <super::super::ApplicationModel::Activation::CachedFileUpdaterActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::CachedFileUpdaterActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnWindowCreated<Impl: IApplicationOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnWindowCreated(&*(&args as *const <WindowCreatedEventArgs as ::windows::core::Abi>::Abi as *const <WindowCreatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationOverrides, BASE_OFFSET>(),
            OnActivated: OnActivated::<Impl, IMPL_OFFSET>,
            OnLaunched: OnLaunched::<Impl, IMPL_OFFSET>,
            OnFileActivated: OnFileActivated::<Impl, IMPL_OFFSET>,
            OnSearchActivated: OnSearchActivated::<Impl, IMPL_OFFSET>,
            OnShareTargetActivated: OnShareTargetActivated::<Impl, IMPL_OFFSET>,
            OnFileOpenPickerActivated: OnFileOpenPickerActivated::<Impl, IMPL_OFFSET>,
            OnFileSavePickerActivated: OnFileSavePickerActivated::<Impl, IMPL_OFFSET>,
            OnCachedFileUpdaterActivated: OnCachedFileUpdaterActivated::<Impl, IMPL_OFFSET>,
            OnWindowCreated: OnWindowCreated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
pub trait IApplicationOverrides2_Impl: Sized {
    fn OnBackgroundActivated(&mut self, args: &::core::option::Option<super::super::ApplicationModel::Activation::BackgroundActivatedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
#[cfg(feature = "ApplicationModel_Activation")]
impl ::windows::core::RuntimeName for IApplicationOverrides2 {
    const NAME: &'static str = "Windows.UI.Xaml.IApplicationOverrides2";
}
#[cfg(feature = "ApplicationModel_Activation")]
impl IApplicationOverrides2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IApplicationOverrides2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IApplicationOverrides2_Vtbl {
        unsafe extern "system" fn OnBackgroundActivated<Impl: IApplicationOverrides2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnBackgroundActivated(&*(&args as *const <super::super::ApplicationModel::Activation::BackgroundActivatedEventArgs as ::windows::core::Abi>::Abi as *const <super::super::ApplicationModel::Activation::BackgroundActivatedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IApplicationOverrides2, BASE_OFFSET>(),
            OnBackgroundActivated: OnBackgroundActivated::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IApplicationOverrides2 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Controls")]
pub trait IDataTemplateExtension_Impl: Sized {
    fn ResetTemplate(&mut self) -> ::windows::core::Result<()>;
    fn ProcessBinding(&mut self, phase: u32) -> ::windows::core::Result<bool>;
    fn ProcessBindings(&mut self, arg: &::core::option::Option<Controls::ContainerContentChangingEventArgs>) -> ::windows::core::Result<i32>;
}
#[cfg(feature = "UI_Xaml_Controls")]
impl ::windows::core::RuntimeName for IDataTemplateExtension {
    const NAME: &'static str = "Windows.UI.Xaml.IDataTemplateExtension";
}
#[cfg(feature = "UI_Xaml_Controls")]
impl IDataTemplateExtension_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataTemplateExtension_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataTemplateExtension_Vtbl {
        unsafe extern "system" fn ResetTemplate<Impl: IDataTemplateExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResetTemplate().into()
        }
        unsafe extern "system" fn ProcessBinding<Impl: IDataTemplateExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, phase: u32, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessBinding(phase) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ProcessBindings<Impl: IDataTemplateExtension_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, arg: ::windows::core::RawPtr, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProcessBindings(&*(&arg as *const <Controls::ContainerContentChangingEventArgs as ::windows::core::Abi>::Abi as *const <Controls::ContainerContentChangingEventArgs as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataTemplateExtension, BASE_OFFSET>(),
            ResetTemplate: ResetTemplate::<Impl, IMPL_OFFSET>,
            ProcessBinding: ProcessBinding::<Impl, IMPL_OFFSET>,
            ProcessBindings: ProcessBindings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataTemplateExtension as ::windows::core::Interface>::IID
    }
}
pub trait IElementFactory_Impl: Sized {
    fn GetElement(&mut self, args: &::core::option::Option<ElementFactoryGetArgs>) -> ::windows::core::Result<UIElement>;
    fn RecycleElement(&mut self, args: &::core::option::Option<ElementFactoryRecycleArgs>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IElementFactory {
    const NAME: &'static str = "Windows.UI.Xaml.IElementFactory";
}
impl IElementFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IElementFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IElementFactory_Vtbl {
        unsafe extern "system" fn GetElement<Impl: IElementFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetElement(&*(&args as *const <ElementFactoryGetArgs as ::windows::core::Abi>::Abi as *const <ElementFactoryGetArgs as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RecycleElement<Impl: IElementFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RecycleElement(&*(&args as *const <ElementFactoryRecycleArgs as ::windows::core::Abi>::Abi as *const <ElementFactoryRecycleArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IElementFactory, BASE_OFFSET>(),
            GetElement: GetElement::<Impl, IMPL_OFFSET>,
            RecycleElement: RecycleElement::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IElementFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "Foundation")]
pub trait IFrameworkElementOverrides_Impl: Sized {
    fn MeasureOverride(&mut self, availablesize: &super::super::Foundation::Size) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn ArrangeOverride(&mut self, finalsize: &super::super::Foundation::Size) -> ::windows::core::Result<super::super::Foundation::Size>;
    fn OnApplyTemplate(&mut self) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
#[cfg(feature = "Foundation")]
impl ::windows::core::RuntimeName for IFrameworkElementOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementOverrides";
}
#[cfg(feature = "Foundation")]
impl IFrameworkElementOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElementOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkElementOverrides_Vtbl {
        unsafe extern "system" fn MeasureOverride<Impl: IFrameworkElementOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, availablesize: super::super::Foundation::Size, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MeasureOverride(&*(&availablesize as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ArrangeOverride<Impl: IFrameworkElementOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, finalsize: super::super::Foundation::Size, result__: *mut super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ArrangeOverride(&*(&finalsize as *const <super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnApplyTemplate<Impl: IFrameworkElementOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnApplyTemplate().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkElementOverrides, BASE_OFFSET>(),
            MeasureOverride: MeasureOverride::<Impl, IMPL_OFFSET>,
            ArrangeOverride: ArrangeOverride::<Impl, IMPL_OFFSET>,
            OnApplyTemplate: OnApplyTemplate::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkElementOverrides as ::windows::core::Interface>::IID
    }
}
pub trait IFrameworkElementOverrides2_Impl: Sized {
    fn GoToElementStateCore(&mut self, statename: &::windows::core::HSTRING, usetransitions: bool) -> ::windows::core::Result<bool>;
}
impl ::windows::core::RuntimeName for IFrameworkElementOverrides2 {
    const NAME: &'static str = "Windows.UI.Xaml.IFrameworkElementOverrides2";
}
impl IFrameworkElementOverrides2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IFrameworkElementOverrides2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IFrameworkElementOverrides2_Vtbl {
        unsafe extern "system" fn GoToElementStateCore<Impl: IFrameworkElementOverrides2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, statename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, usetransitions: bool, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GoToElementStateCore(&*(&statename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), usetransitions) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IFrameworkElementOverrides2, BASE_OFFSET>(),
            GoToElementStateCore: GoToElementStateCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IFrameworkElementOverrides2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Automation_Peers"))]
pub trait IUIElementOverrides_Impl: Sized {
    fn OnCreateAutomationPeer(&mut self) -> ::windows::core::Result<Automation::Peers::AutomationPeer>;
    fn OnDisconnectVisualChildren(&mut self) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn FindSubElementsForTouchTargeting(&mut self, point: &super::super::Foundation::Point, boundingrect: &super::super::Foundation::Rect) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<super::super::Foundation::Collections::IIterable<super::super::Foundation::Point>>>;
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Automation_Peers"))]
impl ::windows::core::RuntimeName for IUIElementOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementOverrides";
}
#[cfg(all(feature = "Foundation", feature = "Foundation_Collections", feature = "UI_Xaml_Automation_Peers"))]
impl IUIElementOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElementOverrides_Vtbl {
        unsafe extern "system" fn OnCreateAutomationPeer<Impl: IUIElementOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OnCreateAutomationPeer() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDisconnectVisualChildren<Impl: IUIElementOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnDisconnectVisualChildren().into()
        }
        unsafe extern "system" fn FindSubElementsForTouchTargeting<Impl: IUIElementOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, point: super::super::Foundation::Point, boundingrect: super::super::Foundation::Rect, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FindSubElementsForTouchTargeting(&*(&point as *const <super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType), &*(&boundingrect as *const <super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElementOverrides, BASE_OFFSET>(),
            OnCreateAutomationPeer: OnCreateAutomationPeer::<Impl, IMPL_OFFSET>,
            OnDisconnectVisualChildren: OnDisconnectVisualChildren::<Impl, IMPL_OFFSET>,
            FindSubElementsForTouchTargeting: FindSubElementsForTouchTargeting::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElementOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Input"))]
pub trait IUIElementOverrides7_Impl: Sized {
    fn GetChildrenInTabFocusOrder(&mut self) -> ::windows::core::Result<super::super::Foundation::Collections::IIterable<DependencyObject>>;
    fn OnProcessKeyboardAccelerators(&mut self, args: &::core::option::Option<Input::ProcessKeyboardAcceleratorEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Input"))]
impl ::windows::core::RuntimeName for IUIElementOverrides7 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementOverrides7";
}
#[cfg(all(feature = "Foundation_Collections", feature = "UI_Xaml_Input"))]
impl IUIElementOverrides7_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementOverrides7_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElementOverrides7_Vtbl {
        unsafe extern "system" fn GetChildrenInTabFocusOrder<Impl: IUIElementOverrides7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetChildrenInTabFocusOrder() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnProcessKeyboardAccelerators<Impl: IUIElementOverrides7_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnProcessKeyboardAccelerators(&*(&args as *const <Input::ProcessKeyboardAcceleratorEventArgs as ::windows::core::Abi>::Abi as *const <Input::ProcessKeyboardAcceleratorEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElementOverrides7, BASE_OFFSET>(),
            GetChildrenInTabFocusOrder: GetChildrenInTabFocusOrder::<Impl, IMPL_OFFSET>,
            OnProcessKeyboardAccelerators: OnProcessKeyboardAccelerators::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElementOverrides7 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Input")]
pub trait IUIElementOverrides8_Impl: Sized {
    fn OnKeyboardAcceleratorInvoked(&mut self, args: &::core::option::Option<Input::KeyboardAcceleratorInvokedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
    fn OnBringIntoViewRequested(&mut self, e: &::core::option::Option<BringIntoViewRequestedEventArgs>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
#[cfg(feature = "UI_Xaml_Input")]
impl ::windows::core::RuntimeName for IUIElementOverrides8 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementOverrides8";
}
#[cfg(feature = "UI_Xaml_Input")]
impl IUIElementOverrides8_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementOverrides8_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElementOverrides8_Vtbl {
        unsafe extern "system" fn OnKeyboardAcceleratorInvoked<Impl: IUIElementOverrides8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, args: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnKeyboardAcceleratorInvoked(&*(&args as *const <Input::KeyboardAcceleratorInvokedEventArgs as ::windows::core::Abi>::Abi as *const <Input::KeyboardAcceleratorInvokedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn OnBringIntoViewRequested<Impl: IUIElementOverrides8_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, e: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).OnBringIntoViewRequested(&*(&e as *const <BringIntoViewRequestedEventArgs as ::windows::core::Abi>::Abi as *const <BringIntoViewRequestedEventArgs as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElementOverrides8, BASE_OFFSET>(),
            OnKeyboardAcceleratorInvoked: OnKeyboardAcceleratorInvoked::<Impl, IMPL_OFFSET>,
            OnBringIntoViewRequested: OnBringIntoViewRequested::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElementOverrides8 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Composition")]
pub trait IUIElementOverrides9_Impl: Sized {
    fn PopulatePropertyInfoOverride(&mut self, propertyname: &::windows::core::HSTRING, animationpropertyinfo: &::core::option::Option<super::Composition::AnimationPropertyInfo>) -> ::windows::core::Result<()> {
        ::core::result::Result::Ok(())
    }
}
#[cfg(feature = "UI_Composition")]
impl ::windows::core::RuntimeName for IUIElementOverrides9 {
    const NAME: &'static str = "Windows.UI.Xaml.IUIElementOverrides9";
}
#[cfg(feature = "UI_Composition")]
impl IUIElementOverrides9_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUIElementOverrides9_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUIElementOverrides9_Vtbl {
        unsafe extern "system" fn PopulatePropertyInfoOverride<Impl: IUIElementOverrides9_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, propertyname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, animationpropertyinfo: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).PopulatePropertyInfoOverride(&*(&propertyname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&animationpropertyinfo as *const <super::Composition::AnimationPropertyInfo as ::windows::core::Abi>::Abi as *const <super::Composition::AnimationPropertyInfo as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IUIElementOverrides9, BASE_OFFSET>(),
            PopulatePropertyInfoOverride: PopulatePropertyInfoOverride::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUIElementOverrides9 as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Controls")]
pub trait IVisualStateManagerOverrides_Impl: Sized {
    fn GoToStateCore(&mut self, control: &::core::option::Option<Controls::Control>, templateroot: &::core::option::Option<FrameworkElement>, statename: &::windows::core::HSTRING, group: &::core::option::Option<VisualStateGroup>, state: &::core::option::Option<VisualState>, usetransitions: bool) -> ::windows::core::Result<bool>;
}
#[cfg(feature = "UI_Xaml_Controls")]
impl ::windows::core::RuntimeName for IVisualStateManagerOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.IVisualStateManagerOverrides";
}
#[cfg(feature = "UI_Xaml_Controls")]
impl IVisualStateManagerOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IVisualStateManagerOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IVisualStateManagerOverrides_Vtbl {
        unsafe extern "system" fn GoToStateCore<Impl: IVisualStateManagerOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, control: ::windows::core::RawPtr, templateroot: ::windows::core::RawPtr, statename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, group: ::windows::core::RawPtr, state: ::windows::core::RawPtr, usetransitions: bool, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GoToStateCore(
                &*(&control as *const <Controls::Control as ::windows::core::Abi>::Abi as *const <Controls::Control as ::windows::core::DefaultType>::DefaultType),
                &*(&templateroot as *const <FrameworkElement as ::windows::core::Abi>::Abi as *const <FrameworkElement as ::windows::core::DefaultType>::DefaultType),
                &*(&statename as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                &*(&group as *const <VisualStateGroup as ::windows::core::Abi>::Abi as *const <VisualStateGroup as ::windows::core::DefaultType>::DefaultType),
                &*(&state as *const <VisualState as ::windows::core::Abi>::Abi as *const <VisualState as ::windows::core::DefaultType>::DefaultType),
                usetransitions,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IVisualStateManagerOverrides, BASE_OFFSET>(),
            GoToStateCore: GoToStateCore::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IVisualStateManagerOverrides as ::windows::core::Interface>::IID
    }
}
