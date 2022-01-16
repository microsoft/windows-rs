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
