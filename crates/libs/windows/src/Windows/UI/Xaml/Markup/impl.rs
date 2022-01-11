pub trait IComponentConnectorImpl: Sized {
    fn Connect(&self, connectionid: i32, target: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IComponentConnector {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IComponentConnector";
}
impl IComponentConnectorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComponentConnectorImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComponentConnectorVtbl {
        unsafe extern "system" fn Connect<Impl: IComponentConnectorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: i32, target: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Connect(connectionid, &*(&target as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComponentConnector>, ::windows::core::GetTrustLevel, Connect::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComponentConnector as ::windows::core::Interface>::IID
    }
}
pub trait IComponentConnector2Impl: Sized {
    fn GetBindingConnector(&self, connectionid: i32, target: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<IComponentConnector>;
}
impl ::windows::core::RuntimeName for IComponentConnector2 {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IComponentConnector2";
}
impl IComponentConnector2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IComponentConnector2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IComponentConnector2Vtbl {
        unsafe extern "system" fn GetBindingConnector<Impl: IComponentConnector2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: i32, target: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IComponentConnector2>, ::windows::core::GetTrustLevel, GetBindingConnector::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IComponentConnector2 as ::windows::core::Interface>::IID
    }
}
pub trait IDataTemplateComponentImpl: Sized {
    fn Recycle(&self) -> ::windows::core::Result<()>;
    fn ProcessBindings(&self, item: &::core::option::Option<::windows::core::IInspectable>, itemindex: i32, phase: i32, nextphase: &mut i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IDataTemplateComponent {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IDataTemplateComponent";
}
impl IDataTemplateComponentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IDataTemplateComponentImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IDataTemplateComponentVtbl {
        unsafe extern "system" fn Recycle<Impl: IDataTemplateComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Recycle().into()
        }
        unsafe extern "system" fn ProcessBindings<Impl: IDataTemplateComponentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, item: *mut ::core::ffi::c_void, itemindex: i32, phase: i32, nextphase: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ProcessBindings(&*(&item as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), itemindex, phase, ::core::mem::transmute_copy(&nextphase)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IDataTemplateComponent>, ::windows::core::GetTrustLevel, Recycle::<Impl, IMPL_OFFSET>, ProcessBindings::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IDataTemplateComponent as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMarkupExtensionImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMarkupExtension {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IMarkupExtension";
}
#[cfg(feature = "implement_exclusive")]
impl IMarkupExtensionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMarkupExtensionImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMarkupExtensionVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMarkupExtension>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMarkupExtension as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMarkupExtensionFactoryImpl: Sized {
    fn CreateInstance(&self, baseinterface: &::core::option::Option<::windows::core::IInspectable>, innerinterface: &mut ::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<MarkupExtension>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMarkupExtensionFactory {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IMarkupExtensionFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IMarkupExtensionFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMarkupExtensionFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMarkupExtensionFactoryVtbl {
        unsafe extern "system" fn CreateInstance<Impl: IMarkupExtensionFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, baseinterface: *mut ::core::ffi::c_void, innerinterface: *mut *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMarkupExtensionFactory>, ::windows::core::GetTrustLevel, CreateInstance::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMarkupExtensionFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IMarkupExtensionOverridesImpl: Sized {
    fn ProvideValue(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IMarkupExtensionOverrides {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IMarkupExtensionOverrides";
}
#[cfg(feature = "implement_exclusive")]
impl IMarkupExtensionOverridesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IMarkupExtensionOverridesImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IMarkupExtensionOverridesVtbl {
        unsafe extern "system" fn ProvideValue<Impl: IMarkupExtensionOverridesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IMarkupExtensionOverrides>, ::windows::core::GetTrustLevel, ProvideValue::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IMarkupExtensionOverrides as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlBinaryWriterImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlBinaryWriter {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlBinaryWriter";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlBinaryWriterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlBinaryWriterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlBinaryWriterVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXamlBinaryWriter>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlBinaryWriter as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
pub trait IXamlBinaryWriterStaticsImpl: Sized {
    fn Write(&self, inputstreams: &::core::option::Option<super::super::super::Foundation::Collections::IVector<super::super::super::Storage::Streams::IRandomAccessStream>>, outputstreams: &::core::option::Option<super::super::super::Foundation::Collections::IVector<super::super::super::Storage::Streams::IRandomAccessStream>>, xamlmetadataprovider: &::core::option::Option<IXamlMetadataProvider>) -> ::windows::core::Result<XamlBinaryWriterErrorInformation>;
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXamlBinaryWriterStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlBinaryWriterStatics";
}
#[cfg(all(feature = "Foundation_Collections", feature = "Storage_Streams", feature = "implement_exclusive"))]
impl IXamlBinaryWriterStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlBinaryWriterStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlBinaryWriterStaticsVtbl {
        unsafe extern "system" fn Write<Impl: IXamlBinaryWriterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, inputstreams: ::windows::core::RawPtr, outputstreams: ::windows::core::RawPtr, xamlmetadataprovider: ::windows::core::RawPtr, result__: *mut XamlBinaryWriterErrorInformation) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXamlBinaryWriterStatics>, ::windows::core::GetTrustLevel, Write::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlBinaryWriterStatics as ::windows::core::Interface>::IID
    }
}
pub trait IXamlBindScopeDiagnosticsImpl: Sized {
    fn Disable(&self, linenumber: i32, columnnumber: i32) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IXamlBindScopeDiagnostics {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlBindScopeDiagnostics";
}
impl IXamlBindScopeDiagnosticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlBindScopeDiagnosticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlBindScopeDiagnosticsVtbl {
        unsafe extern "system" fn Disable<Impl: IXamlBindScopeDiagnosticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, linenumber: i32, columnnumber: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Disable(linenumber, columnnumber).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXamlBindScopeDiagnostics>, ::windows::core::GetTrustLevel, Disable::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlBindScopeDiagnostics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlBindingHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlBindingHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlBindingHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlBindingHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlBindingHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlBindingHelperVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXamlBindingHelper>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlBindingHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
pub trait IXamlBindingHelperStaticsImpl: Sized {
    fn DataTemplateComponentProperty(&self) -> ::windows::core::Result<super::DependencyProperty>;
    fn GetDataTemplateComponent(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<IDataTemplateComponent>;
    fn SetDataTemplateComponent(&self, element: &::core::option::Option<super::DependencyObject>, value: &::core::option::Option<IDataTemplateComponent>) -> ::windows::core::Result<()>;
    fn SuspendRendering(&self, target: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn ResumeRendering(&self, target: &::core::option::Option<super::UIElement>) -> ::windows::core::Result<()>;
    fn ConvertValue(&self, r#type: &super::Interop::TypeName, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetPropertyFromString(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn SetPropertyFromBoolean(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: bool) -> ::windows::core::Result<()>;
    fn SetPropertyFromChar16(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: u16) -> ::windows::core::Result<()>;
    fn SetPropertyFromDateTime(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: &super::super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn SetPropertyFromDouble(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: f64) -> ::windows::core::Result<()>;
    fn SetPropertyFromInt32(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: i32) -> ::windows::core::Result<()>;
    fn SetPropertyFromUInt32(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: u32) -> ::windows::core::Result<()>;
    fn SetPropertyFromInt64(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: i64) -> ::windows::core::Result<()>;
    fn SetPropertyFromUInt64(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: u64) -> ::windows::core::Result<()>;
    fn SetPropertyFromSingle(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: f32) -> ::windows::core::Result<()>;
    fn SetPropertyFromPoint(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: &super::super::super::Foundation::Point) -> ::windows::core::Result<()>;
    fn SetPropertyFromRect(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: &super::super::super::Foundation::Rect) -> ::windows::core::Result<()>;
    fn SetPropertyFromSize(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: &super::super::super::Foundation::Size) -> ::windows::core::Result<()>;
    fn SetPropertyFromTimeSpan(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: &super::super::super::Foundation::TimeSpan) -> ::windows::core::Result<()>;
    fn SetPropertyFromByte(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: u8) -> ::windows::core::Result<()>;
    fn SetPropertyFromUri(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: &::core::option::Option<super::super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn SetPropertyFromObject(&self, dependencyobject: &::core::option::Option<::windows::core::IInspectable>, propertytoset: &::core::option::Option<super::DependencyProperty>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
impl ::windows::core::RuntimeName for IXamlBindingHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlBindingHelperStatics";
}
#[cfg(all(feature = "Foundation", feature = "UI_Xaml_Interop", feature = "implement_exclusive"))]
impl IXamlBindingHelperStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlBindingHelperStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlBindingHelperStaticsVtbl {
        unsafe extern "system" fn DataTemplateComponentProperty<Impl: IXamlBindingHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetDataTemplateComponent<Impl: IXamlBindingHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetDataTemplateComponent<Impl: IXamlBindingHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDataTemplateComponent(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <IDataTemplateComponent as ::windows::core::Abi>::Abi as *const <IDataTemplateComponent as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn SuspendRendering<Impl: IXamlBindingHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SuspendRendering(&*(&target as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ResumeRendering<Impl: IXamlBindingHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, target: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).ResumeRendering(&*(&target as *const <super::UIElement as ::windows::core::Abi>::Abi as *const <super::UIElement as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn ConvertValue<Impl: IXamlBindingHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ::core::mem::ManuallyDrop<super::Interop::TypeName>, value: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetPropertyFromString<Impl: IXamlBindingHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetPropertyFromString(
                    &*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType),
                    &*(&value as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn SetPropertyFromBoolean<Impl: IXamlBindingHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: bool) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyFromBoolean(&*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn SetPropertyFromChar16<Impl: IXamlBindingHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyFromChar16(&*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn SetPropertyFromDateTime<Impl: IXamlBindingHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: super::super::super::Foundation::DateTime) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetPropertyFromDateTime(
                    &*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType),
                    &*(&value as *const <super::super::super::Foundation::DateTime as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::DateTime as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn SetPropertyFromDouble<Impl: IXamlBindingHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: f64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyFromDouble(&*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn SetPropertyFromInt32<Impl: IXamlBindingHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyFromInt32(&*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn SetPropertyFromUInt32<Impl: IXamlBindingHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyFromUInt32(&*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn SetPropertyFromInt64<Impl: IXamlBindingHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: i64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyFromInt64(&*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn SetPropertyFromUInt64<Impl: IXamlBindingHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyFromUInt64(&*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn SetPropertyFromSingle<Impl: IXamlBindingHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: f32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyFromSingle(&*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn SetPropertyFromPoint<Impl: IXamlBindingHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: super::super::super::Foundation::Point) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetPropertyFromPoint(
                    &*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType),
                    &*(&value as *const <super::super::super::Foundation::Point as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Point as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn SetPropertyFromRect<Impl: IXamlBindingHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: super::super::super::Foundation::Rect) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetPropertyFromRect(
                    &*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType),
                    &*(&value as *const <super::super::super::Foundation::Rect as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Rect as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn SetPropertyFromSize<Impl: IXamlBindingHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: super::super::super::Foundation::Size) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetPropertyFromSize(
                    &*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType),
                    &*(&value as *const <super::super::super::Foundation::Size as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Size as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn SetPropertyFromTimeSpan<Impl: IXamlBindingHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: super::super::super::Foundation::TimeSpan) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetPropertyFromTimeSpan(
                    &*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType),
                    &*(&value as *const <super::super::super::Foundation::TimeSpan as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::TimeSpan as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn SetPropertyFromByte<Impl: IXamlBindingHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: u8) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetPropertyFromByte(&*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType), value).into()
        }
        unsafe extern "system" fn SetPropertyFromUri<Impl: IXamlBindingHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetPropertyFromUri(
                    &*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType),
                    &*(&value as *const <super::super::super::Foundation::Uri as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::Uri as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn SetPropertyFromObject<Impl: IXamlBindingHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, dependencyobject: *mut ::core::ffi::c_void, propertytoset: ::windows::core::RawPtr, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .SetPropertyFromObject(
                    &*(&dependencyobject as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&propertytoset as *const <super::DependencyProperty as ::windows::core::Abi>::Abi as *const <super::DependencyProperty as ::windows::core::DefaultType>::DefaultType),
                    &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IXamlBindingHelperStatics>,
            ::windows::core::GetTrustLevel,
            DataTemplateComponentProperty::<Impl, IMPL_OFFSET>,
            GetDataTemplateComponent::<Impl, IMPL_OFFSET>,
            SetDataTemplateComponent::<Impl, IMPL_OFFSET>,
            SuspendRendering::<Impl, IMPL_OFFSET>,
            ResumeRendering::<Impl, IMPL_OFFSET>,
            ConvertValue::<Impl, IMPL_OFFSET>,
            SetPropertyFromString::<Impl, IMPL_OFFSET>,
            SetPropertyFromBoolean::<Impl, IMPL_OFFSET>,
            SetPropertyFromChar16::<Impl, IMPL_OFFSET>,
            SetPropertyFromDateTime::<Impl, IMPL_OFFSET>,
            SetPropertyFromDouble::<Impl, IMPL_OFFSET>,
            SetPropertyFromInt32::<Impl, IMPL_OFFSET>,
            SetPropertyFromUInt32::<Impl, IMPL_OFFSET>,
            SetPropertyFromInt64::<Impl, IMPL_OFFSET>,
            SetPropertyFromUInt64::<Impl, IMPL_OFFSET>,
            SetPropertyFromSingle::<Impl, IMPL_OFFSET>,
            SetPropertyFromPoint::<Impl, IMPL_OFFSET>,
            SetPropertyFromRect::<Impl, IMPL_OFFSET>,
            SetPropertyFromSize::<Impl, IMPL_OFFSET>,
            SetPropertyFromTimeSpan::<Impl, IMPL_OFFSET>,
            SetPropertyFromByte::<Impl, IMPL_OFFSET>,
            SetPropertyFromUri::<Impl, IMPL_OFFSET>,
            SetPropertyFromObject::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlBindingHelperStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlMarkupHelperImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlMarkupHelper {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlMarkupHelper";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlMarkupHelperVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlMarkupHelperImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlMarkupHelperVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXamlMarkupHelper>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlMarkupHelper as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlMarkupHelperStaticsImpl: Sized {
    fn UnloadObject(&self, element: &::core::option::Option<super::DependencyObject>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlMarkupHelperStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlMarkupHelperStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlMarkupHelperStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlMarkupHelperStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlMarkupHelperStaticsVtbl {
        unsafe extern "system" fn UnloadObject<Impl: IXamlMarkupHelperStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, element: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UnloadObject(&*(&element as *const <super::DependencyObject as ::windows::core::Abi>::Abi as *const <super::DependencyObject as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXamlMarkupHelperStatics>, ::windows::core::GetTrustLevel, UnloadObject::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlMarkupHelperStatics as ::windows::core::Interface>::IID
    }
}
pub trait IXamlMemberImpl: Sized {
    fn IsAttachable(&self) -> ::windows::core::Result<bool>;
    fn IsDependencyProperty(&self) -> ::windows::core::Result<bool>;
    fn IsReadOnly(&self) -> ::windows::core::Result<bool>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn TargetType(&self) -> ::windows::core::Result<IXamlType>;
    fn Type(&self) -> ::windows::core::Result<IXamlType>;
    fn GetValue(&self, instance: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn SetValue(&self, instance: &::core::option::Option<::windows::core::IInspectable>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IXamlMember {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlMember";
}
impl IXamlMemberVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlMemberImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlMemberVtbl {
        unsafe extern "system" fn IsAttachable<Impl: IXamlMemberImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsDependencyProperty<Impl: IXamlMemberImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsReadOnly<Impl: IXamlMemberImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Name<Impl: IXamlMemberImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn TargetType<Impl: IXamlMemberImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn Type<Impl: IXamlMemberImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetValue<Impl: IXamlMemberImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instance: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn SetValue<Impl: IXamlMemberImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instance: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetValue(&*(&instance as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IXamlMember>,
            ::windows::core::GetTrustLevel,
            IsAttachable::<Impl, IMPL_OFFSET>,
            IsDependencyProperty::<Impl, IMPL_OFFSET>,
            IsReadOnly::<Impl, IMPL_OFFSET>,
            Name::<Impl, IMPL_OFFSET>,
            TargetType::<Impl, IMPL_OFFSET>,
            Type::<Impl, IMPL_OFFSET>,
            GetValue::<Impl, IMPL_OFFSET>,
            SetValue::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlMember as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Interop")]
pub trait IXamlMetadataProviderImpl: Sized {
    fn GetXamlType(&self, r#type: &super::Interop::TypeName) -> ::windows::core::Result<IXamlType>;
    fn GetXamlTypeByFullName(&self, fullname: &::windows::core::HSTRING) -> ::windows::core::Result<IXamlType>;
    fn GetXmlnsDefinitions(&self) -> ::windows::core::Result<::windows::core::Array<XmlnsDefinition>>;
}
#[cfg(feature = "UI_Xaml_Interop")]
impl ::windows::core::RuntimeName for IXamlMetadataProvider {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlMetadataProvider";
}
#[cfg(feature = "UI_Xaml_Interop")]
impl IXamlMetadataProviderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlMetadataProviderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlMetadataProviderVtbl {
        unsafe extern "system" fn GetXamlType<Impl: IXamlMetadataProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, r#type: ::core::mem::ManuallyDrop<super::Interop::TypeName>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetXamlTypeByFullName<Impl: IXamlMetadataProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fullname: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetXmlnsDefinitions<Impl: IXamlMetadataProviderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result_size__: *mut u32, result__: *mut *mut ::core::mem::ManuallyDrop<XmlnsDefinition>) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXamlMetadataProvider>, ::windows::core::GetTrustLevel, GetXamlType::<Impl, IMPL_OFFSET>, GetXamlTypeByFullName::<Impl, IMPL_OFFSET>, GetXmlnsDefinitions::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlMetadataProvider as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlReaderImpl: Sized {}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlReader {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlReader";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlReaderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlReaderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlReaderVtbl {
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXamlReader>, ::windows::core::GetTrustLevel)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlReader as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IXamlReaderStaticsImpl: Sized {
    fn Load(&self, xaml: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn LoadWithInitialTemplateValidation(&self, xaml: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IXamlReaderStatics {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlReaderStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IXamlReaderStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlReaderStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlReaderStaticsVtbl {
        unsafe extern "system" fn Load<Impl: IXamlReaderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xaml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn LoadWithInitialTemplateValidation<Impl: IXamlReaderStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, xaml: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXamlReaderStatics>, ::windows::core::GetTrustLevel, Load::<Impl, IMPL_OFFSET>, LoadWithInitialTemplateValidation::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlReaderStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Interop")]
pub trait IXamlTypeImpl: Sized {
    fn BaseType(&self) -> ::windows::core::Result<IXamlType>;
    fn ContentProperty(&self) -> ::windows::core::Result<IXamlMember>;
    fn FullName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn IsArray(&self) -> ::windows::core::Result<bool>;
    fn IsCollection(&self) -> ::windows::core::Result<bool>;
    fn IsConstructible(&self) -> ::windows::core::Result<bool>;
    fn IsDictionary(&self) -> ::windows::core::Result<bool>;
    fn IsMarkupExtension(&self) -> ::windows::core::Result<bool>;
    fn IsBindable(&self) -> ::windows::core::Result<bool>;
    fn ItemType(&self) -> ::windows::core::Result<IXamlType>;
    fn KeyType(&self) -> ::windows::core::Result<IXamlType>;
    fn UnderlyingType(&self) -> ::windows::core::Result<super::Interop::TypeName>;
    fn ActivateInstance(&self) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn CreateFromString(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::IInspectable>;
    fn GetMember(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<IXamlMember>;
    fn AddToVector(&self, instance: &::core::option::Option<::windows::core::IInspectable>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn AddToMap(&self, instance: &::core::option::Option<::windows::core::IInspectable>, key: &::core::option::Option<::windows::core::IInspectable>, value: &::core::option::Option<::windows::core::IInspectable>) -> ::windows::core::Result<()>;
    fn RunInitializer(&self) -> ::windows::core::Result<()>;
}
#[cfg(feature = "UI_Xaml_Interop")]
impl ::windows::core::RuntimeName for IXamlType {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlType";
}
#[cfg(feature = "UI_Xaml_Interop")]
impl IXamlTypeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlTypeImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlTypeVtbl {
        unsafe extern "system" fn BaseType<Impl: IXamlTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ContentProperty<Impl: IXamlTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn FullName<Impl: IXamlTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsArray<Impl: IXamlTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsCollection<Impl: IXamlTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsConstructible<Impl: IXamlTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsDictionary<Impl: IXamlTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsMarkupExtension<Impl: IXamlTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn IsBindable<Impl: IXamlTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ItemType<Impl: IXamlTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn KeyType<Impl: IXamlTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn UnderlyingType<Impl: IXamlTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<super::Interop::TypeName>) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn ActivateInstance<Impl: IXamlTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn CreateFromString<Impl: IXamlTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn GetMember<Impl: IXamlTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        unsafe extern "system" fn AddToVector<Impl: IXamlTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instance: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddToVector(&*(&instance as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType)).into()
        }
        unsafe extern "system" fn AddToMap<Impl: IXamlTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, instance: *mut ::core::ffi::c_void, key: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this)
                .AddToMap(
                    &*(&instance as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&key as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                    &*(&value as *const <::windows::core::IInspectable as ::windows::core::Abi>::Abi as *const <::windows::core::IInspectable as ::windows::core::DefaultType>::DefaultType),
                )
                .into()
        }
        unsafe extern "system" fn RunInitializer<Impl: IXamlTypeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RunInitializer().into()
        }
        Self(
            ::windows::core::QueryInterface::<Identity, BASE_OFFSET>,
            ::windows::core::AddRef::<Identity, BASE_OFFSET>,
            ::windows::core::Release::<Identity, BASE_OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IXamlType>,
            ::windows::core::GetTrustLevel,
            BaseType::<Impl, IMPL_OFFSET>,
            ContentProperty::<Impl, IMPL_OFFSET>,
            FullName::<Impl, IMPL_OFFSET>,
            IsArray::<Impl, IMPL_OFFSET>,
            IsCollection::<Impl, IMPL_OFFSET>,
            IsConstructible::<Impl, IMPL_OFFSET>,
            IsDictionary::<Impl, IMPL_OFFSET>,
            IsMarkupExtension::<Impl, IMPL_OFFSET>,
            IsBindable::<Impl, IMPL_OFFSET>,
            ItemType::<Impl, IMPL_OFFSET>,
            KeyType::<Impl, IMPL_OFFSET>,
            UnderlyingType::<Impl, IMPL_OFFSET>,
            ActivateInstance::<Impl, IMPL_OFFSET>,
            CreateFromString::<Impl, IMPL_OFFSET>,
            GetMember::<Impl, IMPL_OFFSET>,
            AddToVector::<Impl, IMPL_OFFSET>,
            AddToMap::<Impl, IMPL_OFFSET>,
            RunInitializer::<Impl, IMPL_OFFSET>,
        )
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlType as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "UI_Xaml_Interop")]
pub trait IXamlType2Impl: Sized + IXamlTypeImpl {
    fn BoxedType(&self) -> ::windows::core::Result<IXamlType>;
}
#[cfg(feature = "UI_Xaml_Interop")]
impl ::windows::core::RuntimeName for IXamlType2 {
    const NAME: &'static str = "Windows.UI.Xaml.Markup.IXamlType2";
}
#[cfg(feature = "UI_Xaml_Interop")]
impl IXamlType2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXamlType2Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXamlType2Vtbl {
        unsafe extern "system" fn BoxedType<Impl: IXamlType2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
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
        Self(::windows::core::QueryInterface::<Identity, BASE_OFFSET>, ::windows::core::AddRef::<Identity, BASE_OFFSET>, ::windows::core::Release::<Identity, BASE_OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXamlType2>, ::windows::core::GetTrustLevel, BoxedType::<Impl, IMPL_OFFSET>)
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXamlType2 as ::windows::core::Interface>::IID
    }
}
