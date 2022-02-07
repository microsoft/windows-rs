#[cfg(feature = "Security_Credentials")]
pub trait IBackgroundTransferBase_Impl: Sized {
    fn SetRequestHeader(&self, headername: &::windows::core::HSTRING, headervalue: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn ServerCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetServerCredential(&self, credential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn ProxyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetProxyCredential(&self, credential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn Method(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMethod(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Group(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetGroup(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn CostPolicy(&self) -> ::windows::core::Result<BackgroundTransferCostPolicy>;
    fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> ::windows::core::Result<()>;
}
#[cfg(feature = "Security_Credentials")]
impl ::windows::core::RuntimeName for IBackgroundTransferBase {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferBase";
}
#[cfg(feature = "Security_Credentials")]
impl IBackgroundTransferBase_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>() -> IBackgroundTransferBase_Vtbl {
        unsafe extern "system" fn SetRequestHeader<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, headername: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, headervalue: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRequestHeader(::core::mem::transmute(&headername), ::core::mem::transmute(&headervalue)).into()
        }
        unsafe extern "system" fn ServerCredential<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ServerCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetServerCredential<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credential: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetServerCredential(::core::mem::transmute(&credential)).into()
        }
        unsafe extern "system" fn ProxyCredential<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ProxyCredential() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProxyCredential<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, credential: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProxyCredential(::core::mem::transmute(&credential)).into()
        }
        unsafe extern "system" fn Method<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Method() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMethod<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMethod(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn Group<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Group() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGroup<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGroup(::core::mem::transmute(&value)).into()
        }
        unsafe extern "system" fn CostPolicy<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BackgroundTransferCostPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CostPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCostPolicy<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferBase_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: BackgroundTransferCostPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCostPolicy(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTransferBase, OFFSET>(),
            SetRequestHeader: SetRequestHeader::<Identity, Impl, OFFSET>,
            ServerCredential: ServerCredential::<Identity, Impl, OFFSET>,
            SetServerCredential: SetServerCredential::<Identity, Impl, OFFSET>,
            ProxyCredential: ProxyCredential::<Identity, Impl, OFFSET>,
            SetProxyCredential: SetProxyCredential::<Identity, Impl, OFFSET>,
            Method: Method::<Identity, Impl, OFFSET>,
            SetMethod: SetMethod::<Identity, Impl, OFFSET>,
            Group: Group::<Identity, Impl, OFFSET>,
            SetGroup: SetGroup::<Identity, Impl, OFFSET>,
            CostPolicy: CostPolicy::<Identity, Impl, OFFSET>,
            SetCostPolicy: SetCostPolicy::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTransferBase as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundTransferContentPartFactory_Impl: Sized {
    fn CreateWithName(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundTransferContentPart>;
    fn CreateWithNameAndFileName(&self, name: &::windows::core::HSTRING, filename: &::windows::core::HSTRING) -> ::windows::core::Result<BackgroundTransferContentPart>;
}
impl ::windows::core::RuntimeName for IBackgroundTransferContentPartFactory {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferContentPartFactory";
}
impl IBackgroundTransferContentPartFactory_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferContentPartFactory_Impl, const OFFSET: isize>() -> IBackgroundTransferContentPartFactory_Vtbl {
        unsafe extern "system" fn CreateWithName<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferContentPartFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateWithName(::core::mem::transmute(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateWithNameAndFileName<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferContentPartFactory_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, filename: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateWithNameAndFileName(::core::mem::transmute(&name), ::core::mem::transmute(&filename)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTransferContentPartFactory, OFFSET>(),
            CreateWithName: CreateWithName::<Identity, Impl, OFFSET>,
            CreateWithNameAndFileName: CreateWithNameAndFileName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTransferContentPartFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
pub trait IBackgroundTransferOperation_Impl: Sized {
    fn Guid(&self) -> ::windows::core::Result<::windows::core::GUID>;
    fn RequestedUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn Method(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn Group(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CostPolicy(&self) -> ::windows::core::Result<BackgroundTransferCostPolicy>;
    fn SetCostPolicy(&self, value: BackgroundTransferCostPolicy) -> ::windows::core::Result<()>;
    fn GetResultStreamAt(&self, position: u64) -> ::windows::core::Result<super::super::Storage::Streams::IInputStream>;
    fn GetResponseInformation(&self) -> ::windows::core::Result<ResponseInformation>;
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl ::windows::core::RuntimeName for IBackgroundTransferOperation {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferOperation";
}
#[cfg(all(feature = "Foundation", feature = "Storage_Streams"))]
impl IBackgroundTransferOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferOperation_Impl, const OFFSET: isize>() -> IBackgroundTransferOperation_Vtbl {
        unsafe extern "system" fn Guid<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::GUID) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Guid() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RequestedUri<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RequestedUri() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Method<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Method() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Group<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Group() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CostPolicy<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BackgroundTransferCostPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CostPolicy() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCostPolicy<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: BackgroundTransferCostPolicy) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetCostPolicy(value).into()
        }
        unsafe extern "system" fn GetResultStreamAt<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, position: u64, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetResultStreamAt(position) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResponseInformation<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetResponseInformation() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTransferOperation, OFFSET>(),
            Guid: Guid::<Identity, Impl, OFFSET>,
            RequestedUri: RequestedUri::<Identity, Impl, OFFSET>,
            Method: Method::<Identity, Impl, OFFSET>,
            Group: Group::<Identity, Impl, OFFSET>,
            CostPolicy: CostPolicy::<Identity, Impl, OFFSET>,
            SetCostPolicy: SetCostPolicy::<Identity, Impl, OFFSET>,
            GetResultStreamAt: GetResultStreamAt::<Identity, Impl, OFFSET>,
            GetResponseInformation: GetResponseInformation::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTransferOperation as ::windows::core::Interface>::IID
    }
}
pub trait IBackgroundTransferOperationPriority_Impl: Sized {
    fn Priority(&self) -> ::windows::core::Result<BackgroundTransferPriority>;
    fn SetPriority(&self, value: BackgroundTransferPriority) -> ::windows::core::Result<()>;
}
impl ::windows::core::RuntimeName for IBackgroundTransferOperationPriority {
    const NAME: &'static str = "Windows.Networking.BackgroundTransfer.IBackgroundTransferOperationPriority";
}
impl IBackgroundTransferOperationPriority_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferOperationPriority_Impl, const OFFSET: isize>() -> IBackgroundTransferOperationPriority_Vtbl {
        unsafe extern "system" fn Priority<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferOperationPriority_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut BackgroundTransferPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Priority() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetPriority<Identity: ::windows::core::IUnknownImpl, Impl: IBackgroundTransferOperationPriority_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, value: BackgroundTransferPriority) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetPriority(value).into()
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IBackgroundTransferOperationPriority, OFFSET>(),
            Priority: Priority::<Identity, Impl, OFFSET>,
            SetPriority: SetPriority::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IBackgroundTransferOperationPriority as ::windows::core::Interface>::IID
    }
}
