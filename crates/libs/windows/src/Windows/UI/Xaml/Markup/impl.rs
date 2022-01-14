pub trait IComponentConnector_Impl: Sized {
    fn Connect(&mut self, connectionid: i32, target: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IComponentConnector {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IComponentConnector";
}
impl IComponentConnector_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComponentConnector_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComponentConnector_Vtbl {
        unsafe extern "system" fn Connect<Impl: IComponentConnector_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: i32, target: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Connect(connectionid, &*(&target as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IComponentConnector, BASE_OFFSET>(), Connect: Connect::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComponentConnector as ::windows::core::Interface>::IID
    }
}
pub trait IComponentConnector2_Impl: Sized {
    fn GetBindingConnector(&mut self, connectionid: i32, target: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<IComponentConnector>;
}
impl ::windows::core::RuntimeName for IComponentConnector2 {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IComponentConnector2";
}
impl IComponentConnector2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComponentConnector2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComponentConnector2_Vtbl {
        unsafe extern "system" fn GetBindingConnector<Impl: IComponentConnector2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: i32, target: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBindingConnector(connectionid, &*(&target as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IComponentConnector2, BASE_OFFSET>(),
            GetBindingConnector: GetBindingConnector::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComponentConnector2 as ::windows::core::Interface>::IID
    }
}
pub trait IDataTemplateComponent_Impl: Sized {
    fn Recycle(&mut self) -> ::windows::core::Result<()>;
    fn ProcessBindings(&mut self, item: &::core::option::Option<::windows::core::IInspectable>, itemindex: i32, phase: i32, nextphase: &mut i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDataTemplateComponent {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IDataTemplateComponent";
}
impl IDataTemplateComponent_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataTemplateComponent_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataTemplateComponent_Vtbl {
        unsafe extern "system" fn Recycle<Impl: IDataTemplateComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Recycle().into()
        }
        unsafe extern "system" fn ProcessBindings<Impl: IDataTemplateComponent_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, itemindex: i32, phase: i32, nextphase: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessBindings(&*(&item as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), itemindex, phase, ::core::mem::transmute_copy(&nextphase)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IDataTemplateComponent, BASE_OFFSET>(),
            Recycle: Recycle::<Impl, IMPL_OFFSET>,
            ProcessBindings: ProcessBindings::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataTemplateComponent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMarkupExtension_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMarkupExtension {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IMarkupExtension";
}
#[cfg(feature = "implement_exclusive")]
impl IMarkupExtension_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMarkupExtension_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMarkupExtension_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IMarkupExtension, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMarkupExtension as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMarkupExtensionFactory_Impl: Sized {
    fn CreateInstance(&mut self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MarkupExtension>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMarkupExtensionFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IMarkupExtensionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMarkupExtensionFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMarkupExtensionFactory_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMarkupExtensionFactory_Vtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMarkupExtensionFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateInstance(&*(&baseinterface as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&innerinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMarkupExtensionFactory, BASE_OFFSET>(),
            CreateInstance: CreateInstance::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMarkupExtensionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMarkupExtensionOverrides_Impl: Sized {
    fn ProvideValue(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMarkupExtensionOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IMarkupExtensionOverrides";
}
#[cfg(feature = "implement_exclusive")]
impl IMarkupExtensionOverrides_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMarkupExtensionOverrides_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMarkupExtensionOverrides_Vtbl {
        unsafe extern "system" fn ProvideValue<Impl: IMarkupExtensionOverrides_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ProvideValue() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IMarkupExtensionOverrides, BASE_OFFSET>(),
            ProvideValue: ProvideValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMarkupExtensionOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlBinaryWriter_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlBinaryWriter {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlBinaryWriter";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlBinaryWriter_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlBinaryWriter_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlBinaryWriter_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlBinaryWriter, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlBinaryWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IXamlBinaryWriterStatics_Impl: Sized {
    fn Write(&mut self, inputstreams: &::core::option::Option<super::super::super::Foundation::Collections::IVector<super::super::super::Storage::Streams::IRandomAccessStream>>, outputstreams: &::core::option::Option<super::super::super::Foundation::Collections::IVector<super::super::super::Storage::Streams::IRandomAccessStream>>, xamlmetadataprovider: &::core::option::Option<IXamlMetadataProvider>) -> ::windows::core::Result<XamlBinaryWriterErrorInformation>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXamlBinaryWriterStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlBinaryWriterStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IXamlBinaryWriterStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlBinaryWriterStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlBinaryWriterStatics_Vtbl {
        unsafe extern "system" fn Write<Impl: IXamlBinaryWriterStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstreams: ::windows::core::RawPtr, outputstreams: ::windows::core::RawPtr, xamlmetadataprovider: ::windows::core::RawPtr, result__: *mut XamlBinaryWriterErrorInformation) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Write(
                &*(&inputstreams as *const <super::super::super::Foundation::Collections::IVector<super::super::super::Storage::Streams::IRandomAccessStream> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IVector<super::super::super::Storage::Streams::IRandomAccessStream> as ::windows::core::DefaultType>::DefaultType),
                &*(&outputstreams as *const <super::super::super::Foundation::Collections::IVector<super::super::super::Storage::Streams::IRandomAccessStream> as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Collections::IVector<super::super::super::Storage::Streams::IRandomAccessStream> as ::windows::core::DefaultType>::DefaultType),
                &*(&xamlmetadataprovider as *const <IXamlMetadataProvider as ::windows::core::Abi>::Abi as *const <IXamlMetadataProvider as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlBinaryWriterStatics, BASE_OFFSET>(), Write: Write::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlBinaryWriterStatics as ::windows::core::Interface>::IID
    }
}
pub trait IXamlBindScopeDiagnostics_Impl: Sized {
    fn Disable(&mut self, linenumber: i32, columnnumber: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IXamlBindScopeDiagnostics {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlBindScopeDiagnostics";
}
impl IXamlBindScopeDiagnostics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlBindScopeDiagnostics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlBindScopeDiagnostics_Vtbl {
        unsafe extern "system" fn Disable<Impl: IXamlBindScopeDiagnostics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linenumber: i32, columnnumber: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disable(linenumber, columnnumber).into()
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlBindScopeDiagnostics, BASE_OFFSET>(), Disable: Disable::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlBindScopeDiagnostics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlBindingHelper_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlBindingHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlBindingHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlBindingHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlBindingHelper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlBindingHelper_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlBindingHelper, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlBindingHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
pub trait IXamlBindingHelperStatics_Impl: Sized {
    fn DataTemplateComponentProperty(&mut self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetDataTemplateComponent(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<IDataTemplateComponent>;
    fn SetDataTemplateComponent(&mut self, element: &::core::option::Option<super::DependencyObject>, value: &::core::option::Option<IDataTemplateComponent>) -> ::windows::core::Result<()>;
    fn SuspendRendering(&mut self, target: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn ResumeRendering(&mut self, target: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn ConvertValue(&mut self, r#type: &super::Interop::TypeName, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetPropertyFromString(&mut self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetPropertyFromBoolean(&mut self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: bool) -> ::windows::core::Result<()>;
    fn SetPropertyFromChar16(&mut self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: u16) -> ::windows::core::Result<()>;
    fn SetPropertyFromDateTime(&mut self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: &super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn SetPropertyFromDouble(&mut self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: f64) -> ::windows::core::Result<()>;
    fn SetPropertyFromInt32(&mut self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: i32) -> ::windows::core::Result<()>;
    fn SetPropertyFromUInt32(&mut self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: u32) -> ::windows::core::Result<()>;
    fn SetPropertyFromInt64(&mut self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: i64) -> ::windows::core::Result<()>;
    fn SetPropertyFromUInt64(&mut self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: u64) -> ::windows::core::Result<()>;
    fn SetPropertyFromSingle(&mut self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: f32) -> ::windows::core::Result<()>;
    fn SetPropertyFromPoint(&mut self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn SetPropertyFromRect(&mut self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: &super::super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn SetPropertyFromSize(&mut self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: &super::super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn SetPropertyFromTimeSpan(&mut self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SetPropertyFromByte(&mut self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: u8) -> ::windows::core::Result<()>;
    fn SetPropertyFromUri(&mut self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn SetPropertyFromObject(&mut self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXamlBindingHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlBindingHelperStatics";
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
impl IXamlBindingHelperStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlBindingHelperStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlBindingHelperStatics_Vtbl {
        unsafe extern "system" fn DataTemplateComponentProperty<Impl: IXamlBindingHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DataTemplateComponentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataTemplateComponent<Impl: IXamlBindingHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetDataTemplateComponent(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDataTemplateComponent<Impl: IXamlBindingHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataTemplateComponent(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <IDataTemplateComponent as ::windows::core::Abi>::Abi as *const <IDataTemplateComponent as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuspendRendering<Impl: IXamlBindingHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SuspendRendering(&*(&target as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResumeRendering<Impl: IXamlBindingHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResumeRendering(&*(&target as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConvertValue<Impl: IXamlBindingHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ::core::mem::ManuallyDrop<super::Interop::TypeName>, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ConvertValue(&*(&r#type as *const <super::Interop::TypeName as ::windows::core::Abi>::Abi as *const <super::Interop::TypeName as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPropertyFromString<Impl: IXamlBindingHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetPropertyFromString(
                    &*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType),
                    &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn SetPropertyFromBoolean<Impl: IXamlBindingHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyFromBoolean(&*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn SetPropertyFromChar16<Impl: IXamlBindingHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyFromChar16(&*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn SetPropertyFromDateTime<Impl: IXamlBindingHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetPropertyFromDateTime(
                    &*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType),
                    &*(&value as *const <super::super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn SetPropertyFromDouble<Impl: IXamlBindingHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyFromDouble(&*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn SetPropertyFromInt32<Impl: IXamlBindingHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyFromInt32(&*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn SetPropertyFromUInt32<Impl: IXamlBindingHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyFromUInt32(&*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn SetPropertyFromInt64<Impl: IXamlBindingHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyFromInt64(&*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn SetPropertyFromUInt64<Impl: IXamlBindingHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyFromUInt64(&*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn SetPropertyFromSingle<Impl: IXamlBindingHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyFromSingle(&*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn SetPropertyFromPoint<Impl: IXamlBindingHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetPropertyFromPoint(
                    &*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType),
                    &*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn SetPropertyFromRect<Impl: IXamlBindingHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetPropertyFromRect(
                    &*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType),
                    &*(&value as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn SetPropertyFromSize<Impl: IXamlBindingHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetPropertyFromSize(
                    &*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType),
                    &*(&value as *const <super::super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn SetPropertyFromTimeSpan<Impl: IXamlBindingHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetPropertyFromTimeSpan(
                    &*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType),
                    &*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn SetPropertyFromByte<Impl: IXamlBindingHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyFromByte(&*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn SetPropertyFromUri<Impl: IXamlBindingHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetPropertyFromUri(
                    &*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType),
                    &*(&value as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn SetPropertyFromObject<Impl: IXamlBindingHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetPropertyFromObject(
                    &*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType),
                    &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlBindingHelperStatics, BASE_OFFSET>(),
            DataTemplateComponentProperty: DataTemplateComponentProperty::<Impl, IMPL_OFFSET>,
            GetDataTemplateComponent: GetDataTemplateComponent::<Impl, IMPL_OFFSET>,
            SetDataTemplateComponent: SetDataTemplateComponent::<Impl, IMPL_OFFSET>,
            SuspendRendering: SuspendRendering::<Impl, IMPL_OFFSET>,
            ResumeRendering: ResumeRendering::<Impl, IMPL_OFFSET>,
            ConvertValue: ConvertValue::<Impl, IMPL_OFFSET>,
            SetPropertyFromString: SetPropertyFromString::<Impl, IMPL_OFFSET>,
            SetPropertyFromBoolean: SetPropertyFromBoolean::<Impl, IMPL_OFFSET>,
            SetPropertyFromChar16: SetPropertyFromChar16::<Impl, IMPL_OFFSET>,
            SetPropertyFromDateTime: SetPropertyFromDateTime::<Impl, IMPL_OFFSET>,
            SetPropertyFromDouble: SetPropertyFromDouble::<Impl, IMPL_OFFSET>,
            SetPropertyFromInt32: SetPropertyFromInt32::<Impl, IMPL_OFFSET>,
            SetPropertyFromUInt32: SetPropertyFromUInt32::<Impl, IMPL_OFFSET>,
            SetPropertyFromInt64: SetPropertyFromInt64::<Impl, IMPL_OFFSET>,
            SetPropertyFromUInt64: SetPropertyFromUInt64::<Impl, IMPL_OFFSET>,
            SetPropertyFromSingle: SetPropertyFromSingle::<Impl, IMPL_OFFSET>,
            SetPropertyFromPoint: SetPropertyFromPoint::<Impl, IMPL_OFFSET>,
            SetPropertyFromRect: SetPropertyFromRect::<Impl, IMPL_OFFSET>,
            SetPropertyFromSize: SetPropertyFromSize::<Impl, IMPL_OFFSET>,
            SetPropertyFromTimeSpan: SetPropertyFromTimeSpan::<Impl, IMPL_OFFSET>,
            SetPropertyFromByte: SetPropertyFromByte::<Impl, IMPL_OFFSET>,
            SetPropertyFromUri: SetPropertyFromUri::<Impl, IMPL_OFFSET>,
            SetPropertyFromObject: SetPropertyFromObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlBindingHelperStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlMarkupHelper_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlMarkupHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlMarkupHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlMarkupHelper_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlMarkupHelper_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlMarkupHelper_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlMarkupHelper, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlMarkupHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlMarkupHelperStatics_Impl: Sized {
    fn UnloadObject(&mut self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlMarkupHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlMarkupHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlMarkupHelperStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlMarkupHelperStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlMarkupHelperStatics_Vtbl {
        unsafe extern "system" fn UnloadObject<Impl: IXamlMarkupHelperStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnloadObject(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlMarkupHelperStatics, BASE_OFFSET>(),
            UnloadObject: UnloadObject::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlMarkupHelperStatics as ::windows::core::Interface>::IID
    }
}
pub trait IXamlMember_Impl: Sized {
    fn IsAttachable(&mut self) -> ::windows::core::Result<bool>;
    fn IsDependencyProperty(&mut self) -> ::windows::core::Result<bool>;
    fn IsReadOnly(&mut self) -> ::windows::core::Result<bool>;
    fn Name(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TargetType(&mut self) -> ::windows::core::Result<IXamlType>;
    fn Type(&mut self) -> ::windows::core::Result<IXamlType>;
    fn GetValue(&mut self, instance: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetValue(&mut self, instance: &::core::option::Option<::windows::core::IInspectable>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IXamlMember {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlMember";
}
impl IXamlMember_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlMember_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlMember_Vtbl {
        unsafe extern "system" fn IsAttachable<Impl: IXamlMember_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsAttachable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDependencyProperty<Impl: IXamlMember_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDependencyProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsReadOnly<Impl: IXamlMember_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsReadOnly() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IXamlMember_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetType<Impl: IXamlMember_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IXamlMember_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetValue<Impl: IXamlMember_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instance: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetValue(&*(&instance as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetValue<Impl: IXamlMember_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instance: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&instance as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlMember, BASE_OFFSET>(),
            IsAttachable: IsAttachable::<Impl, IMPL_OFFSET>,
            IsDependencyProperty: IsDependencyProperty::<Impl, IMPL_OFFSET>,
            IsReadOnly: IsReadOnly::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
            TargetType: TargetType::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            GetValue: GetValue::<Impl, IMPL_OFFSET>,
            SetValue: SetValue::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlMember as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Interop")]
pub trait IXamlMetadataProvider_Impl: Sized {
    fn GetXamlType(&mut self, r#type: &super::Interop::TypeName) -> ::windows::core::Result<IXamlType>;
    fn GetXamlTypeByFullName(&mut self, fullname: &::windows::core::HSTRING) -> ::windows::core::Result<IXamlType>;
    fn GetXmlnsDefinitions(&mut self) -> ::windows::core::Result<::windows::core::Array<XmlnsDefinition>>;
}
#[cfg(feature = "UI_Xaml_Interop")]
impl ::windows::core::RuntimeName for IXamlMetadataProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlMetadataProvider";
}
#[cfg(feature = "UI_Xaml_Interop")]
impl IXamlMetadataProvider_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlMetadataProvider_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlMetadataProvider_Vtbl {
        unsafe extern "system" fn GetXamlType<Impl: IXamlMetadataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ::core::mem::ManuallyDrop<super::Interop::TypeName>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXamlType(&*(&r#type as *const <super::Interop::TypeName as ::windows::core::Abi>::Abi as *const <super::Interop::TypeName as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXamlTypeByFullName<Impl: IXamlMetadataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXamlTypeByFullName(&*(&fullname as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetXmlnsDefinitions<Impl: IXamlMetadataProvider_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::core::mem::ManuallyDrop<XmlnsDefinition>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetXmlnsDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    let (ok_data__, ok_data_len__) = ok__.into_abi();
                    *result__ = ok_data__;
                    *result_size__ = ok_data_len__;
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlMetadataProvider, BASE_OFFSET>(),
            GetXamlType: GetXamlType::<Impl, IMPL_OFFSET>,
            GetXamlTypeByFullName: GetXamlTypeByFullName::<Impl, IMPL_OFFSET>,
            GetXmlnsDefinitions: GetXmlnsDefinitions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlMetadataProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlReader_Impl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlReader {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlReader";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlReader_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlReader_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlReader_Vtbl {
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlReader, BASE_OFFSET>() }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlReaderStatics_Impl: Sized {
    fn Load(&mut self, xaml: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn LoadWithInitialTemplateValidation(&mut self, xaml: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlReaderStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlReaderStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlReaderStatics_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlReaderStatics_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlReaderStatics_Vtbl {
        unsafe extern "system" fn Load<Impl: IXamlReaderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xaml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Load(&*(&xaml as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LoadWithInitialTemplateValidation<Impl: IXamlReaderStatics_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xaml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LoadWithInitialTemplateValidation(&*(&xaml as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlReaderStatics, BASE_OFFSET>(),
            Load: Load::<Impl, IMPL_OFFSET>,
            LoadWithInitialTemplateValidation: LoadWithInitialTemplateValidation::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlReaderStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Interop")]
pub trait IXamlType_Impl: Sized {
    fn BaseType(&mut self) -> ::windows::core::Result<IXamlType>;
    fn ContentProperty(&mut self) -> ::windows::core::Result<IXamlMember>;
    fn FullName(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsArray(&mut self) -> ::windows::core::Result<bool>;
    fn IsCollection(&mut self) -> ::windows::core::Result<bool>;
    fn IsConstructible(&mut self) -> ::windows::core::Result<bool>;
    fn IsDictionary(&mut self) -> ::windows::core::Result<bool>;
    fn IsMarkupExtension(&mut self) -> ::windows::core::Result<bool>;
    fn IsBindable(&mut self) -> ::windows::core::Result<bool>;
    fn ItemType(&mut self) -> ::windows::core::Result<IXamlType>;
    fn KeyType(&mut self) -> ::windows::core::Result<IXamlType>;
    fn UnderlyingType(&mut self) -> ::windows::core::Result<super::Interop::TypeName>;
    fn ActivateInstance(&mut self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateFromString(&mut self, value: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetMember(&mut self, name: &::windows::core::HSTRING) -> ::windows::core::Result<IXamlMember>;
    fn AddToVector(&mut self, instance: &::core::option::Option<::windows::core::IInspectable>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn AddToMap(&mut self, instance: &::core::option::Option<::windows::core::IInspectable>, key: &::core::option::Option<::windows::core::IInspectable>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn RunInitializer(&mut self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "UI_Xaml_Interop")]
impl ::windows::core::RuntimeName for IXamlType {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlType";
}
#[cfg(feature = "UI_Xaml_Interop")]
impl IXamlType_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlType_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlType_Vtbl {
        unsafe extern "system" fn BaseType<Impl: IXamlType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BaseType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ContentProperty<Impl: IXamlType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ContentProperty() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FullName<Impl: IXamlType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FullName() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsArray<Impl: IXamlType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsArray() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsCollection<Impl: IXamlType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsCollection() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConstructible<Impl: IXamlType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsConstructible() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDictionary<Impl: IXamlType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsDictionary() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsMarkupExtension<Impl: IXamlType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsMarkupExtension() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsBindable<Impl: IXamlType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsBindable() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ItemType<Impl: IXamlType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ItemType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn KeyType<Impl: IXamlType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).KeyType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UnderlyingType<Impl: IXamlType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UnderlyingType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ActivateInstance<Impl: IXamlType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ActivateInstance() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateFromString<Impl: IXamlType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateFromString(&*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetMember<Impl: IXamlType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetMember(&*(&name as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddToVector<Impl: IXamlType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instance: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddToVector(&*(&instance as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddToMap<Impl: IXamlType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instance: *mut ::core::ffi::c_void, key: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .AddToMap(
                    &*(&instance as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&key as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RunInitializer<Impl: IXamlType_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RunInitializer().into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlType, BASE_OFFSET>(),
            BaseType: BaseType::<Impl, IMPL_OFFSET>,
            ContentProperty: ContentProperty::<Impl, IMPL_OFFSET>,
            FullName: FullName::<Impl, IMPL_OFFSET>,
            IsArray: IsArray::<Impl, IMPL_OFFSET>,
            IsCollection: IsCollection::<Impl, IMPL_OFFSET>,
            IsConstructible: IsConstructible::<Impl, IMPL_OFFSET>,
            IsDictionary: IsDictionary::<Impl, IMPL_OFFSET>,
            IsMarkupExtension: IsMarkupExtension::<Impl, IMPL_OFFSET>,
            IsBindable: IsBindable::<Impl, IMPL_OFFSET>,
            ItemType: ItemType::<Impl, IMPL_OFFSET>,
            KeyType: KeyType::<Impl, IMPL_OFFSET>,
            UnderlyingType: UnderlyingType::<Impl, IMPL_OFFSET>,
            ActivateInstance: ActivateInstance::<Impl, IMPL_OFFSET>,
            CreateFromString: CreateFromString::<Impl, IMPL_OFFSET>,
            GetMember: GetMember::<Impl, IMPL_OFFSET>,
            AddToVector: AddToVector::<Impl, IMPL_OFFSET>,
            AddToMap: AddToMap::<Impl, IMPL_OFFSET>,
            RunInitializer: RunInitializer::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlType as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Interop")]
pub trait IXamlType2_Impl: Sized + IXamlType_Impl {
    fn BoxedType(&mut self) -> ::windows::core::Result<IXamlType>;
}
#[cfg(feature = "UI_Xaml_Interop")]
impl ::windows::core::RuntimeName for IXamlType2 {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlType2";
}
#[cfg(feature = "UI_Xaml_Interop")]
impl IXamlType2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlType2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlType2_Vtbl {
        unsafe extern "system" fn BoxedType<Impl: IXamlType2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BoxedType() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IXamlType2, BASE_OFFSET>(), BoxedType: BoxedType::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlType2 as ::windows::core::Interface>::IID
    }
}
