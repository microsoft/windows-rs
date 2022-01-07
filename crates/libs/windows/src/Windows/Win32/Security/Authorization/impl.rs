#[cfg(feature = "Win32_System_Com")]
pub trait IAzApplicationImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetName();
    fn Description();
    fn SetDescription();
    fn ApplicationData();
    fn SetApplicationData();
    fn AuthzInterfaceClsid();
    fn SetAuthzInterfaceClsid();
    fn Version();
    fn SetVersion();
    fn GenerateAudits();
    fn SetGenerateAudits();
    fn ApplyStoreSacl();
    fn SetApplyStoreSacl();
    fn Writable();
    fn GetProperty();
    fn SetProperty();
    fn PolicyAdministrators();
    fn PolicyReaders();
    fn AddPolicyAdministrator();
    fn DeletePolicyAdministrator();
    fn AddPolicyReader();
    fn DeletePolicyReader();
    fn Scopes();
    fn OpenScope();
    fn CreateScope();
    fn DeleteScope();
    fn Operations();
    fn OpenOperation();
    fn CreateOperation();
    fn DeleteOperation();
    fn Tasks();
    fn OpenTask();
    fn CreateTask();
    fn DeleteTask();
    fn ApplicationGroups();
    fn OpenApplicationGroup();
    fn CreateApplicationGroup();
    fn DeleteApplicationGroup();
    fn Roles();
    fn OpenRole();
    fn CreateRole();
    fn DeleteRole();
    fn InitializeClientContextFromToken();
    fn AddPropertyItem();
    fn DeletePropertyItem();
    fn Submit();
    fn InitializeClientContextFromName();
    fn DelegatedPolicyUsers();
    fn AddDelegatedPolicyUser();
    fn DeleteDelegatedPolicyUser();
    fn InitializeClientContextFromStringSid();
    fn PolicyAdministratorsName();
    fn PolicyReadersName();
    fn AddPolicyAdministratorName();
    fn DeletePolicyAdministratorName();
    fn AddPolicyReaderName();
    fn DeletePolicyReaderName();
    fn DelegatedPolicyUsersName();
    fn AddDelegatedPolicyUserName();
    fn DeleteDelegatedPolicyUserName();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzApplication {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzApplication";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzApplicationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationImpl, const OFFSET: isize>() -> IAzApplicationVtbl {
        unsafe extern "system" fn Name<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&pbstrdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDescription(&*(&bstrdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationData<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationData(::core::mem::transmute_copy(&pbstrapplicationdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetApplicationData(&*(&bstrapplicationdata as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AuthzInterfaceClsid<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthzInterfaceClsid(::core::mem::transmute_copy(&pbstrprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthzInterfaceClsid<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetAuthzInterfaceClsid(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Version<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Version(::core::mem::transmute_copy(&pbstrprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVersion<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetVersion(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateAudits<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerateAudits(::core::mem::transmute_copy(&pbprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGenerateAudits<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bprop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGenerateAudits(&*(&bprop as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyStoreSacl<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplyStoreSacl(::core::mem::transmute_copy(&pbprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplyStoreSacl<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bprop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetApplyStoreSacl(&*(&bprop as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Writable<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Writable(::core::mem::transmute_copy(&pfprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(lpropid, &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProperty(lpropid, &*(&varprop as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyAdministrators<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyAdministrators(::core::mem::transmute_copy(&pvaradmins)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReaders<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyReaders(::core::mem::transmute_copy(&pvarreaders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministrator<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPolicyAdministrator(&*(&bstradmin as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePolicyAdministrator<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeletePolicyAdministrator(&*(&bstradmin as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyReader<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPolicyReader(&*(&bstrreader as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePolicyReader<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeletePolicyReader(&*(&bstrreader as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scopes<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppscopecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scopes(::core::mem::transmute_copy(&ppscopecollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenScope<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppscope: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenScope(&*(&bstrscopename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppscope)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScope<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppscope: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateScope(&*(&bstrscopename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppscope)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteScope<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteScope(&*(&bstrscopename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Operations<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoperationcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Operations(::core::mem::transmute_copy(&ppoperationcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenOperation<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroperationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppoperation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenOperation(&*(&bstroperationname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppoperation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOperation<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroperationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppoperation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateOperation(&*(&bstroperationname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppoperation)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteOperation<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroperationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteOperation(&*(&bstroperationname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tasks<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptaskcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tasks(::core::mem::transmute_copy(&pptaskcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTask<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pptask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenTask(&*(&bstrtaskname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pptask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTask<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pptask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTask(&*(&bstrtaskname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pptask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteTask<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteTask(&*(&bstrtaskname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationGroups<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroupcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationGroups(::core::mem::transmute_copy(&ppgroupcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenApplicationGroup<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenApplicationGroup(&*(&bstrgroupname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateApplicationGroup<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateApplicationGroup(&*(&bstrgroupname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteApplicationGroup<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteApplicationGroup(&*(&bstrgroupname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Roles<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprolecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Roles(::core::mem::transmute_copy(&pprolecollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRole<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pprole: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenRole(&*(&bstrrolename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprole)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRole<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pprole: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRole(&*(&bstrrolename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprole)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRole<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteRole(&*(&bstrrolename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeClientContextFromToken<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulltokenhandle: u64, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclientcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeClientContextFromToken(ulltokenhandle, &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppclientcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPropertyItem<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPropertyItem(lpropid, &*(&varprop as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePropertyItem<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeletePropertyItem(lpropid, &*(&varprop as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Submit<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Submit(lflags, &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeClientContextFromName<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, domainname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclientcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeClientContextFromName(
                &*(&clientname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&domainname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&ppclientcontext),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DelegatedPolicyUsers<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DelegatedPolicyUsers(::core::mem::transmute_copy(&pvardelegatedpolicyusers)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDelegatedPolicyUser<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddDelegatedPolicyUser(&*(&bstrdelegatedpolicyuser as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUser<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteDelegatedPolicyUser(&*(&bstrdelegatedpolicyuser as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeClientContextFromStringSid<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sidstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclientcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeClientContextFromStringSid(&*(&sidstring as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), loptions, &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppclientcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyAdministratorsName<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyAdministratorsName(::core::mem::transmute_copy(&pvaradmins)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReadersName<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyReadersName(::core::mem::transmute_copy(&pvarreaders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministratorName<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPolicyAdministratorName(&*(&bstradmin as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePolicyAdministratorName<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeletePolicyAdministratorName(&*(&bstradmin as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyReaderName<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPolicyReaderName(&*(&bstrreader as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePolicyReaderName<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeletePolicyReaderName(&*(&bstrreader as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DelegatedPolicyUsersName<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DelegatedPolicyUsersName(::core::mem::transmute_copy(&pvardelegatedpolicyusers)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDelegatedPolicyUserName<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddDelegatedPolicyUserName(&*(&bstrdelegatedpolicyuser as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUserName<Impl: IAzApplicationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteDelegatedPolicyUserName(&*(&bstrdelegatedpolicyuser as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAzApplication>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            SetDescription::<Impl, OFFSET>,
            ApplicationData::<Impl, OFFSET>,
            SetApplicationData::<Impl, OFFSET>,
            AuthzInterfaceClsid::<Impl, OFFSET>,
            SetAuthzInterfaceClsid::<Impl, OFFSET>,
            Version::<Impl, OFFSET>,
            SetVersion::<Impl, OFFSET>,
            GenerateAudits::<Impl, OFFSET>,
            SetGenerateAudits::<Impl, OFFSET>,
            ApplyStoreSacl::<Impl, OFFSET>,
            SetApplyStoreSacl::<Impl, OFFSET>,
            Writable::<Impl, OFFSET>,
            GetProperty::<Impl, OFFSET>,
            SetProperty::<Impl, OFFSET>,
            PolicyAdministrators::<Impl, OFFSET>,
            PolicyReaders::<Impl, OFFSET>,
            AddPolicyAdministrator::<Impl, OFFSET>,
            DeletePolicyAdministrator::<Impl, OFFSET>,
            AddPolicyReader::<Impl, OFFSET>,
            DeletePolicyReader::<Impl, OFFSET>,
            Scopes::<Impl, OFFSET>,
            OpenScope::<Impl, OFFSET>,
            CreateScope::<Impl, OFFSET>,
            DeleteScope::<Impl, OFFSET>,
            Operations::<Impl, OFFSET>,
            OpenOperation::<Impl, OFFSET>,
            CreateOperation::<Impl, OFFSET>,
            DeleteOperation::<Impl, OFFSET>,
            Tasks::<Impl, OFFSET>,
            OpenTask::<Impl, OFFSET>,
            CreateTask::<Impl, OFFSET>,
            DeleteTask::<Impl, OFFSET>,
            ApplicationGroups::<Impl, OFFSET>,
            OpenApplicationGroup::<Impl, OFFSET>,
            CreateApplicationGroup::<Impl, OFFSET>,
            DeleteApplicationGroup::<Impl, OFFSET>,
            Roles::<Impl, OFFSET>,
            OpenRole::<Impl, OFFSET>,
            CreateRole::<Impl, OFFSET>,
            DeleteRole::<Impl, OFFSET>,
            InitializeClientContextFromToken::<Impl, OFFSET>,
            AddPropertyItem::<Impl, OFFSET>,
            DeletePropertyItem::<Impl, OFFSET>,
            Submit::<Impl, OFFSET>,
            InitializeClientContextFromName::<Impl, OFFSET>,
            DelegatedPolicyUsers::<Impl, OFFSET>,
            AddDelegatedPolicyUser::<Impl, OFFSET>,
            DeleteDelegatedPolicyUser::<Impl, OFFSET>,
            InitializeClientContextFromStringSid::<Impl, OFFSET>,
            PolicyAdministratorsName::<Impl, OFFSET>,
            PolicyReadersName::<Impl, OFFSET>,
            AddPolicyAdministratorName::<Impl, OFFSET>,
            DeletePolicyAdministratorName::<Impl, OFFSET>,
            AddPolicyReaderName::<Impl, OFFSET>,
            DeletePolicyReaderName::<Impl, OFFSET>,
            DelegatedPolicyUsersName::<Impl, OFFSET>,
            AddDelegatedPolicyUserName::<Impl, OFFSET>,
            DeleteDelegatedPolicyUserName::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzApplication2Impl: Sized + IAzApplicationImpl + IDispatchImpl {
    fn InitializeClientContextFromToken2();
    fn InitializeClientContext2();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzApplication2 {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzApplication2";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzApplication2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication2Impl, const OFFSET: isize>() -> IAzApplication2Vtbl {
        unsafe extern "system" fn InitializeClientContextFromToken2<Impl: IAzApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ultokenhandlelowpart: u32, ultokenhandlehighpart: u32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclientcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeClientContextFromToken2(ultokenhandlelowpart, ultokenhandlehighpart, &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppclientcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeClientContext2<Impl: IAzApplication2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifyingstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclientcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeClientContext2(&*(&identifyingstring as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppclientcontext)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAzApplication2>, ::windows::core::GetTrustLevel, InitializeClientContextFromToken2::<Impl, OFFSET>, InitializeClientContext2::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzApplication3Impl: Sized + IAzApplication2Impl + IAzApplicationImpl + IDispatchImpl {
    fn ScopeExists();
    fn OpenScope2();
    fn CreateScope2();
    fn DeleteScope2();
    fn RoleDefinitions();
    fn CreateRoleDefinition();
    fn OpenRoleDefinition();
    fn DeleteRoleDefinition();
    fn RoleAssignments();
    fn CreateRoleAssignment();
    fn OpenRoleAssignment();
    fn DeleteRoleAssignment();
    fn BizRulesEnabled();
    fn SetBizRulesEnabled();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzApplication3 {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzApplication3";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzApplication3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication3Impl, const OFFSET: isize>() -> IAzApplication3Vtbl {
        unsafe extern "system" fn ScopeExists<Impl: IAzApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbexist: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScopeExists(&*(&bstrscopename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbexist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenScope2<Impl: IAzApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppscope2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenScope2(&*(&bstrscopename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppscope2)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScope2<Impl: IAzApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppscope2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateScope2(&*(&bstrscopename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppscope2)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteScope2<Impl: IAzApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteScope2(&*(&bstrscopename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoleDefinitions<Impl: IAzApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoleDefinitions(::core::mem::transmute_copy(&pproledefinitions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRoleDefinition<Impl: IAzApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRoleDefinition(&*(&bstrroledefinitionname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pproledefinitions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRoleDefinition<Impl: IAzApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenRoleDefinition(&*(&bstrroledefinitionname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pproledefinitions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRoleDefinition<Impl: IAzApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteRoleDefinition(&*(&bstrroledefinitionname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoleAssignments<Impl: IAzApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproleassignments: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoleAssignments(::core::mem::transmute_copy(&pproleassignments)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRoleAssignment<Impl: IAzApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproleassignment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRoleAssignment(&*(&bstrroleassignmentname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pproleassignment)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRoleAssignment<Impl: IAzApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproleassignment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenRoleAssignment(&*(&bstrroleassignmentname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pproleassignment)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRoleAssignment<Impl: IAzApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteRoleAssignment(&*(&bstrroleassignmentname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BizRulesEnabled<Impl: IAzApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BizRulesEnabled(::core::mem::transmute_copy(&pbenabled)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRulesEnabled<Impl: IAzApplication3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBizRulesEnabled(benabled) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAzApplication3>,
            ::windows::core::GetTrustLevel,
            ScopeExists::<Impl, OFFSET>,
            OpenScope2::<Impl, OFFSET>,
            CreateScope2::<Impl, OFFSET>,
            DeleteScope2::<Impl, OFFSET>,
            RoleDefinitions::<Impl, OFFSET>,
            CreateRoleDefinition::<Impl, OFFSET>,
            OpenRoleDefinition::<Impl, OFFSET>,
            DeleteRoleDefinition::<Impl, OFFSET>,
            RoleAssignments::<Impl, OFFSET>,
            CreateRoleAssignment::<Impl, OFFSET>,
            OpenRoleAssignment::<Impl, OFFSET>,
            DeleteRoleAssignment::<Impl, OFFSET>,
            BizRulesEnabled::<Impl, OFFSET>,
            SetBizRulesEnabled::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzApplicationGroupImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetName();
    fn Type();
    fn SetType();
    fn LdapQuery();
    fn SetLdapQuery();
    fn AppMembers();
    fn AppNonMembers();
    fn Members();
    fn NonMembers();
    fn Description();
    fn SetDescription();
    fn AddAppMember();
    fn DeleteAppMember();
    fn AddAppNonMember();
    fn DeleteAppNonMember();
    fn AddMember();
    fn DeleteMember();
    fn AddNonMember();
    fn DeleteNonMember();
    fn Writable();
    fn GetProperty();
    fn SetProperty();
    fn AddPropertyItem();
    fn DeletePropertyItem();
    fn Submit();
    fn AddMemberName();
    fn DeleteMemberName();
    fn AddNonMemberName();
    fn DeleteNonMemberName();
    fn MembersName();
    fn NonMembersName();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzApplicationGroup {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzApplicationGroup";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzApplicationGroupVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroupImpl, const OFFSET: isize>() -> IAzApplicationGroupVtbl {
        unsafe extern "system" fn Name<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Type<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type(::core::mem::transmute_copy(&plprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetType(lprop) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LdapQuery<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LdapQuery(::core::mem::transmute_copy(&pbstrprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLdapQuery<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLdapQuery(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppMembers<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppMembers(::core::mem::transmute_copy(&pvarprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppNonMembers<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppNonMembers(::core::mem::transmute_copy(&pvarprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Members<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Members(::core::mem::transmute_copy(&pvarprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NonMembers<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NonMembers(::core::mem::transmute_copy(&pvarprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&pbstrdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDescription(&*(&bstrdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAppMember<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddAppMember(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAppMember<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteAppMember(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAppNonMember<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddAppNonMember(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAppNonMember<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteAppNonMember(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddMember<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddMember(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteMember<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteMember(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddNonMember<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddNonMember(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteNonMember<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteNonMember(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Writable<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Writable(::core::mem::transmute_copy(&pfprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(lpropid, &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProperty(lpropid, &*(&varprop as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPropertyItem<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPropertyItem(lpropid, &*(&varprop as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePropertyItem<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeletePropertyItem(lpropid, &*(&varprop as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Submit<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Submit(lflags, &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddMemberName<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddMemberName(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteMemberName<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteMemberName(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddNonMemberName<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddNonMemberName(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteNonMemberName<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteNonMemberName(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MembersName<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MembersName(::core::mem::transmute_copy(&pvarprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NonMembersName<Impl: IAzApplicationGroupImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NonMembersName(::core::mem::transmute_copy(&pvarprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAzApplicationGroup>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            Type::<Impl, OFFSET>,
            SetType::<Impl, OFFSET>,
            LdapQuery::<Impl, OFFSET>,
            SetLdapQuery::<Impl, OFFSET>,
            AppMembers::<Impl, OFFSET>,
            AppNonMembers::<Impl, OFFSET>,
            Members::<Impl, OFFSET>,
            NonMembers::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            SetDescription::<Impl, OFFSET>,
            AddAppMember::<Impl, OFFSET>,
            DeleteAppMember::<Impl, OFFSET>,
            AddAppNonMember::<Impl, OFFSET>,
            DeleteAppNonMember::<Impl, OFFSET>,
            AddMember::<Impl, OFFSET>,
            DeleteMember::<Impl, OFFSET>,
            AddNonMember::<Impl, OFFSET>,
            DeleteNonMember::<Impl, OFFSET>,
            Writable::<Impl, OFFSET>,
            GetProperty::<Impl, OFFSET>,
            SetProperty::<Impl, OFFSET>,
            AddPropertyItem::<Impl, OFFSET>,
            DeletePropertyItem::<Impl, OFFSET>,
            Submit::<Impl, OFFSET>,
            AddMemberName::<Impl, OFFSET>,
            DeleteMemberName::<Impl, OFFSET>,
            AddNonMemberName::<Impl, OFFSET>,
            DeleteNonMemberName::<Impl, OFFSET>,
            MembersName::<Impl, OFFSET>,
            NonMembersName::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzApplicationGroup2Impl: Sized + IAzApplicationGroupImpl + IDispatchImpl {
    fn BizRule();
    fn SetBizRule();
    fn BizRuleLanguage();
    fn SetBizRuleLanguage();
    fn BizRuleImportedPath();
    fn SetBizRuleImportedPath();
    fn RoleAssignments();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzApplicationGroup2 {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzApplicationGroup2";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzApplicationGroup2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup2Impl, const OFFSET: isize>() -> IAzApplicationGroup2Vtbl {
        unsafe extern "system" fn BizRule<Impl: IAzApplicationGroup2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BizRule(::core::mem::transmute_copy(&pbstrprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRule<Impl: IAzApplicationGroup2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBizRule(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BizRuleLanguage<Impl: IAzApplicationGroup2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BizRuleLanguage(::core::mem::transmute_copy(&pbstrprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRuleLanguage<Impl: IAzApplicationGroup2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBizRuleLanguage(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BizRuleImportedPath<Impl: IAzApplicationGroup2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BizRuleImportedPath(::core::mem::transmute_copy(&pbstrprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRuleImportedPath<Impl: IAzApplicationGroup2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBizRuleImportedPath(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoleAssignments<Impl: IAzApplicationGroup2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, brecursive: i16, pproleassignments: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoleAssignments(&*(&bstrscopename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), brecursive, ::core::mem::transmute_copy(&pproleassignments)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAzApplicationGroup2>,
            ::windows::core::GetTrustLevel,
            BizRule::<Impl, OFFSET>,
            SetBizRule::<Impl, OFFSET>,
            BizRuleLanguage::<Impl, OFFSET>,
            SetBizRuleLanguage::<Impl, OFFSET>,
            BizRuleImportedPath::<Impl, OFFSET>,
            SetBizRuleImportedPath::<Impl, OFFSET>,
            RoleAssignments::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzApplicationGroupsImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzApplicationGroups {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzApplicationGroups";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzApplicationGroupsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroupsImpl, const OFFSET: isize>() -> IAzApplicationGroupsVtbl {
        unsafe extern "system" fn Item<Impl: IAzApplicationGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&pvarobtptr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IAzApplicationGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IAzApplicationGroupsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppenumptr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAzApplicationGroups>, ::windows::core::GetTrustLevel, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzApplicationsImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzApplications {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzApplications";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzApplicationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationsImpl, const OFFSET: isize>() -> IAzApplicationsVtbl {
        unsafe extern "system" fn Item<Impl: IAzApplicationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&pvarobtptr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IAzApplicationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IAzApplicationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppenumptr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAzApplications>, ::windows::core::GetTrustLevel, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzAuthorizationStoreImpl: Sized + IDispatchImpl {
    fn Description();
    fn SetDescription();
    fn ApplicationData();
    fn SetApplicationData();
    fn DomainTimeout();
    fn SetDomainTimeout();
    fn ScriptEngineTimeout();
    fn SetScriptEngineTimeout();
    fn MaxScriptEngines();
    fn SetMaxScriptEngines();
    fn GenerateAudits();
    fn SetGenerateAudits();
    fn Writable();
    fn GetProperty();
    fn SetProperty();
    fn AddPropertyItem();
    fn DeletePropertyItem();
    fn PolicyAdministrators();
    fn PolicyReaders();
    fn AddPolicyAdministrator();
    fn DeletePolicyAdministrator();
    fn AddPolicyReader();
    fn DeletePolicyReader();
    fn Initialize();
    fn UpdateCache();
    fn Delete();
    fn Applications();
    fn OpenApplication();
    fn CreateApplication();
    fn DeleteApplication();
    fn ApplicationGroups();
    fn CreateApplicationGroup();
    fn OpenApplicationGroup();
    fn DeleteApplicationGroup();
    fn Submit();
    fn DelegatedPolicyUsers();
    fn AddDelegatedPolicyUser();
    fn DeleteDelegatedPolicyUser();
    fn TargetMachine();
    fn ApplyStoreSacl();
    fn SetApplyStoreSacl();
    fn PolicyAdministratorsName();
    fn PolicyReadersName();
    fn AddPolicyAdministratorName();
    fn DeletePolicyAdministratorName();
    fn AddPolicyReaderName();
    fn DeletePolicyReaderName();
    fn DelegatedPolicyUsersName();
    fn AddDelegatedPolicyUserName();
    fn DeleteDelegatedPolicyUserName();
    fn CloseApplication();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzAuthorizationStore {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzAuthorizationStore";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzAuthorizationStoreVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>() -> IAzAuthorizationStoreVtbl {
        unsafe extern "system" fn Description<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&pbstrdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDescription(&*(&bstrdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationData<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationData(::core::mem::transmute_copy(&pbstrapplicationdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetApplicationData(&*(&bstrapplicationdata as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DomainTimeout<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DomainTimeout(::core::mem::transmute_copy(&plprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDomainTimeout<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDomainTimeout(lprop) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ScriptEngineTimeout<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScriptEngineTimeout(::core::mem::transmute_copy(&plprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScriptEngineTimeout<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetScriptEngineTimeout(lprop) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MaxScriptEngines<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxScriptEngines(::core::mem::transmute_copy(&plprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxScriptEngines<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetMaxScriptEngines(lprop) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GenerateAudits<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerateAudits(::core::mem::transmute_copy(&pbprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGenerateAudits<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bprop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetGenerateAudits(&*(&bprop as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Writable<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Writable(::core::mem::transmute_copy(&pfprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(lpropid, &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProperty(lpropid, &*(&varprop as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPropertyItem<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: AZ_PROP_CONSTANTS, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPropertyItem(lpropid, &*(&varprop as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePropertyItem<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeletePropertyItem(lpropid, &*(&varprop as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyAdministrators<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyAdministrators(::core::mem::transmute_copy(&pvaradmins)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReaders<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyReaders(::core::mem::transmute_copy(&pvarreaders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministrator<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPolicyAdministrator(&*(&bstradmin as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePolicyAdministrator<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeletePolicyAdministrator(&*(&bstradmin as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyReader<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPolicyReader(&*(&bstrreader as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePolicyReader<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeletePolicyReader(&*(&bstrreader as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Initialize<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: AZ_PROP_CONSTANTS, bstrpolicyurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Initialize(lflags, &*(&bstrpolicyurl as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpdateCache<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpdateCache(&*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Delete<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Delete(&*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Applications<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppappcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Applications(::core::mem::transmute_copy(&ppappcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenApplication<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppapplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenApplication(&*(&bstrapplicationname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppapplication)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateApplication<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppapplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateApplication(&*(&bstrapplicationname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppapplication)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteApplication<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteApplication(&*(&bstrapplicationname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationGroups<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroupcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationGroups(::core::mem::transmute_copy(&ppgroupcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateApplicationGroup<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateApplicationGroup(&*(&bstrgroupname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenApplicationGroup<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenApplicationGroup(&*(&bstrgroupname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteApplicationGroup<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteApplicationGroup(&*(&bstrgroupname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Submit<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Submit(lflags, &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DelegatedPolicyUsers<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DelegatedPolicyUsers(::core::mem::transmute_copy(&pvardelegatedpolicyusers)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDelegatedPolicyUser<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddDelegatedPolicyUser(&*(&bstrdelegatedpolicyuser as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUser<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteDelegatedPolicyUser(&*(&bstrdelegatedpolicyuser as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TargetMachine<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtargetmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetMachine(::core::mem::transmute_copy(&pbstrtargetmachine)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyStoreSacl<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbapplystoresacl: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplyStoreSacl(::core::mem::transmute_copy(&pbapplystoresacl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplyStoreSacl<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bapplystoresacl: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetApplyStoreSacl(&*(&bapplystoresacl as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyAdministratorsName<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyAdministratorsName(::core::mem::transmute_copy(&pvaradmins)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReadersName<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyReadersName(::core::mem::transmute_copy(&pvarreaders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministratorName<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPolicyAdministratorName(&*(&bstradmin as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePolicyAdministratorName<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeletePolicyAdministratorName(&*(&bstradmin as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyReaderName<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPolicyReaderName(&*(&bstrreader as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePolicyReaderName<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeletePolicyReaderName(&*(&bstrreader as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DelegatedPolicyUsersName<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DelegatedPolicyUsersName(::core::mem::transmute_copy(&pvardelegatedpolicyusers)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDelegatedPolicyUserName<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddDelegatedPolicyUserName(&*(&bstrdelegatedpolicyuser as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUserName<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteDelegatedPolicyUserName(&*(&bstrdelegatedpolicyuser as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CloseApplication<Impl: IAzAuthorizationStoreImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflag: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CloseApplication(&*(&bstrapplicationname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), lflag) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAzAuthorizationStore>,
            ::windows::core::GetTrustLevel,
            Description::<Impl, OFFSET>,
            SetDescription::<Impl, OFFSET>,
            ApplicationData::<Impl, OFFSET>,
            SetApplicationData::<Impl, OFFSET>,
            DomainTimeout::<Impl, OFFSET>,
            SetDomainTimeout::<Impl, OFFSET>,
            ScriptEngineTimeout::<Impl, OFFSET>,
            SetScriptEngineTimeout::<Impl, OFFSET>,
            MaxScriptEngines::<Impl, OFFSET>,
            SetMaxScriptEngines::<Impl, OFFSET>,
            GenerateAudits::<Impl, OFFSET>,
            SetGenerateAudits::<Impl, OFFSET>,
            Writable::<Impl, OFFSET>,
            GetProperty::<Impl, OFFSET>,
            SetProperty::<Impl, OFFSET>,
            AddPropertyItem::<Impl, OFFSET>,
            DeletePropertyItem::<Impl, OFFSET>,
            PolicyAdministrators::<Impl, OFFSET>,
            PolicyReaders::<Impl, OFFSET>,
            AddPolicyAdministrator::<Impl, OFFSET>,
            DeletePolicyAdministrator::<Impl, OFFSET>,
            AddPolicyReader::<Impl, OFFSET>,
            DeletePolicyReader::<Impl, OFFSET>,
            Initialize::<Impl, OFFSET>,
            UpdateCache::<Impl, OFFSET>,
            Delete::<Impl, OFFSET>,
            Applications::<Impl, OFFSET>,
            OpenApplication::<Impl, OFFSET>,
            CreateApplication::<Impl, OFFSET>,
            DeleteApplication::<Impl, OFFSET>,
            ApplicationGroups::<Impl, OFFSET>,
            CreateApplicationGroup::<Impl, OFFSET>,
            OpenApplicationGroup::<Impl, OFFSET>,
            DeleteApplicationGroup::<Impl, OFFSET>,
            Submit::<Impl, OFFSET>,
            DelegatedPolicyUsers::<Impl, OFFSET>,
            AddDelegatedPolicyUser::<Impl, OFFSET>,
            DeleteDelegatedPolicyUser::<Impl, OFFSET>,
            TargetMachine::<Impl, OFFSET>,
            ApplyStoreSacl::<Impl, OFFSET>,
            SetApplyStoreSacl::<Impl, OFFSET>,
            PolicyAdministratorsName::<Impl, OFFSET>,
            PolicyReadersName::<Impl, OFFSET>,
            AddPolicyAdministratorName::<Impl, OFFSET>,
            DeletePolicyAdministratorName::<Impl, OFFSET>,
            AddPolicyReaderName::<Impl, OFFSET>,
            DeletePolicyReaderName::<Impl, OFFSET>,
            DelegatedPolicyUsersName::<Impl, OFFSET>,
            AddDelegatedPolicyUserName::<Impl, OFFSET>,
            DeleteDelegatedPolicyUserName::<Impl, OFFSET>,
            CloseApplication::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzAuthorizationStore2Impl: Sized + IAzAuthorizationStoreImpl + IDispatchImpl {
    fn OpenApplication2();
    fn CreateApplication2();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzAuthorizationStore2 {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzAuthorizationStore2";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzAuthorizationStore2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore2Impl, const OFFSET: isize>() -> IAzAuthorizationStore2Vtbl {
        unsafe extern "system" fn OpenApplication2<Impl: IAzAuthorizationStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppapplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenApplication2(&*(&bstrapplicationname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppapplication)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateApplication2<Impl: IAzAuthorizationStore2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppapplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateApplication2(&*(&bstrapplicationname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppapplication)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAzAuthorizationStore2>, ::windows::core::GetTrustLevel, OpenApplication2::<Impl, OFFSET>, CreateApplication2::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzAuthorizationStore3Impl: Sized + IAzAuthorizationStore2Impl + IAzAuthorizationStoreImpl + IDispatchImpl {
    fn IsUpdateNeeded();
    fn BizruleGroupSupported();
    fn UpgradeStoresFunctionalLevel();
    fn IsFunctionalLevelUpgradeSupported();
    fn GetSchemaVersion();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzAuthorizationStore3 {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzAuthorizationStore3";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzAuthorizationStore3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore3Impl, const OFFSET: isize>() -> IAzAuthorizationStore3Vtbl {
        unsafe extern "system" fn IsUpdateNeeded<Impl: IAzAuthorizationStore3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisupdateneeded: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUpdateNeeded(::core::mem::transmute_copy(&pbisupdateneeded)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BizruleGroupSupported<Impl: IAzAuthorizationStore3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsupported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BizruleGroupSupported(::core::mem::transmute_copy(&pbsupported)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpgradeStoresFunctionalLevel<Impl: IAzAuthorizationStore3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfunctionallevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UpgradeStoresFunctionalLevel(lfunctionallevel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsFunctionalLevelUpgradeSupported<Impl: IAzAuthorizationStore3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfunctionallevel: i32, pbsupported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFunctionalLevelUpgradeSupported(lfunctionallevel, ::core::mem::transmute_copy(&pbsupported)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSchemaVersion<Impl: IAzAuthorizationStore3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetSchemaVersion(::core::mem::transmute_copy(&plmajorversion), ::core::mem::transmute_copy(&plminorversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAzAuthorizationStore3>, ::windows::core::GetTrustLevel, IsUpdateNeeded::<Impl, OFFSET>, BizruleGroupSupported::<Impl, OFFSET>, UpgradeStoresFunctionalLevel::<Impl, OFFSET>, IsFunctionalLevelUpgradeSupported::<Impl, OFFSET>, GetSchemaVersion::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzBizRuleContextImpl: Sized + IDispatchImpl {
    fn SetBusinessRuleResult();
    fn SetBusinessRuleString();
    fn BusinessRuleString();
    fn GetParameter();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzBizRuleContext {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzBizRuleContext";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzBizRuleContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzBizRuleContextImpl, const OFFSET: isize>() -> IAzBizRuleContextVtbl {
        unsafe extern "system" fn SetBusinessRuleResult<Impl: IAzBizRuleContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bresult: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBusinessRuleResult(&*(&bresult as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBusinessRuleString<Impl: IAzBizRuleContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbusinessrulestring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBusinessRuleString(&*(&bstrbusinessrulestring as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BusinessRuleString<Impl: IAzBizRuleContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbusinessrulestring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BusinessRuleString(::core::mem::transmute_copy(&pbstrbusinessrulestring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParameter<Impl: IAzBizRuleContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrparametername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarparametervalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParameter(&*(&bstrparametername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarparametervalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAzBizRuleContext>, ::windows::core::GetTrustLevel, SetBusinessRuleResult::<Impl, OFFSET>, SetBusinessRuleString::<Impl, OFFSET>, BusinessRuleString::<Impl, OFFSET>, GetParameter::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzBizRuleInterfacesImpl: Sized + IDispatchImpl {
    fn AddInterface();
    fn AddInterfaces();
    fn GetInterfaceValue();
    fn Remove();
    fn RemoveAll();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzBizRuleInterfaces {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzBizRuleInterfaces";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzBizRuleInterfacesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzBizRuleInterfacesImpl, const OFFSET: isize>() -> IAzBizRuleInterfacesVtbl {
        unsafe extern "system" fn AddInterface<Impl: IAzBizRuleInterfacesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinterfacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linterfaceflag: i32, varinterface: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddInterface(&*(&bstrinterfacename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), linterfaceflag, &*(&varinterface as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddInterfaces<Impl: IAzBizRuleInterfacesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varinterfacenames: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varinterfaceflags: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varinterfaces: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddInterfaces(
                &*(&varinterfacenames as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&varinterfaceflags as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&varinterfaces as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetInterfaceValue<Impl: IAzBizRuleInterfacesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinterfacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linterfaceflag: *mut i32, varinterface: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetInterfaceValue(&*(&bstrinterfacename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&linterfaceflag), ::core::mem::transmute_copy(&varinterface)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IAzBizRuleInterfacesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinterfacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(&*(&bstrinterfacename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAll<Impl: IAzBizRuleInterfacesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveAll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IAzBizRuleInterfacesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAzBizRuleInterfaces>, ::windows::core::GetTrustLevel, AddInterface::<Impl, OFFSET>, AddInterfaces::<Impl, OFFSET>, GetInterfaceValue::<Impl, OFFSET>, Remove::<Impl, OFFSET>, RemoveAll::<Impl, OFFSET>, Count::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzBizRuleParametersImpl: Sized + IDispatchImpl {
    fn AddParameter();
    fn AddParameters();
    fn GetParameterValue();
    fn Remove();
    fn RemoveAll();
    fn Count();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzBizRuleParameters {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzBizRuleParameters";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzBizRuleParametersVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzBizRuleParametersImpl, const OFFSET: isize>() -> IAzBizRuleParametersVtbl {
        unsafe extern "system" fn AddParameter<Impl: IAzBizRuleParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrparametername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varparametervalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddParameter(&*(&bstrparametername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varparametervalue as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddParameters<Impl: IAzBizRuleParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varparameternames: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varparametervalues: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddParameters(&*(&varparameternames as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&varparametervalues as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParameterValue<Impl: IAzBizRuleParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrparametername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarparametervalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParameterValue(&*(&bstrparametername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarparametervalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IAzBizRuleParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varparametername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Remove(&*(&varparametername as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RemoveAll<Impl: IAzBizRuleParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RemoveAll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IAzBizRuleParametersImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAzBizRuleParameters>, ::windows::core::GetTrustLevel, AddParameter::<Impl, OFFSET>, AddParameters::<Impl, OFFSET>, GetParameterValue::<Impl, OFFSET>, Remove::<Impl, OFFSET>, RemoveAll::<Impl, OFFSET>, Count::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzClientContextImpl: Sized + IDispatchImpl {
    fn AccessCheck();
    fn GetBusinessRuleString();
    fn UserDn();
    fn UserSamCompat();
    fn UserDisplay();
    fn UserGuid();
    fn UserCanonical();
    fn UserUpn();
    fn UserDnsSamCompat();
    fn GetProperty();
    fn GetRoles();
    fn RoleForAccessCheck();
    fn SetRoleForAccessCheck();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzClientContext {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzClientContext";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzClientContextVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContextImpl, const OFFSET: isize>() -> IAzClientContextVtbl {
        unsafe extern "system" fn AccessCheck<Impl: IAzClientContextImpl, const OFFSET: isize>(
            this: *mut ::core::ffi::c_void,
            bstrobjectname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>,
            varscopenames: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>,
            varoperations: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>,
            varparameternames: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>,
            varparametervalues: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>,
            varinterfacenames: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>,
            varinterfaceflags: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>,
            varinterfaces: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>,
            pvarresults: *mut super::super::System::Com::VARIANT,
        ) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessCheck(
                &*(&bstrobjectname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&varscopenames as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&varoperations as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&varparameternames as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&varparametervalues as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&varinterfacenames as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&varinterfaceflags as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&varinterfaces as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&pvarresults),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBusinessRuleString<Impl: IAzClientContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbusinessrulestring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBusinessRuleString(::core::mem::transmute_copy(&pbstrbusinessrulestring)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserDn<Impl: IAzClientContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserDn(::core::mem::transmute_copy(&pbstrprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserSamCompat<Impl: IAzClientContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserSamCompat(::core::mem::transmute_copy(&pbstrprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserDisplay<Impl: IAzClientContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserDisplay(::core::mem::transmute_copy(&pbstrprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserGuid<Impl: IAzClientContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserGuid(::core::mem::transmute_copy(&pbstrprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserCanonical<Impl: IAzClientContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserCanonical(::core::mem::transmute_copy(&pbstrprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserUpn<Impl: IAzClientContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserUpn(::core::mem::transmute_copy(&pbstrprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserDnsSamCompat<Impl: IAzClientContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserDnsSamCompat(::core::mem::transmute_copy(&pbstrprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IAzClientContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(lpropid, &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRoles<Impl: IAzClientContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarrolenames: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRoles(&*(&bstrscopename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarrolenames)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoleForAccessCheck<Impl: IAzClientContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoleForAccessCheck(::core::mem::transmute_copy(&pbstrprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoleForAccessCheck<Impl: IAzClientContextImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetRoleForAccessCheck(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAzClientContext>,
            ::windows::core::GetTrustLevel,
            AccessCheck::<Impl, OFFSET>,
            GetBusinessRuleString::<Impl, OFFSET>,
            UserDn::<Impl, OFFSET>,
            UserSamCompat::<Impl, OFFSET>,
            UserDisplay::<Impl, OFFSET>,
            UserGuid::<Impl, OFFSET>,
            UserCanonical::<Impl, OFFSET>,
            UserUpn::<Impl, OFFSET>,
            UserDnsSamCompat::<Impl, OFFSET>,
            GetProperty::<Impl, OFFSET>,
            GetRoles::<Impl, OFFSET>,
            RoleForAccessCheck::<Impl, OFFSET>,
            SetRoleForAccessCheck::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzClientContext2Impl: Sized + IAzClientContextImpl + IDispatchImpl {
    fn GetAssignedScopesPage();
    fn AddRoles();
    fn AddApplicationGroups();
    fn AddStringSids();
    fn SetLDAPQueryDN();
    fn LDAPQueryDN();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzClientContext2 {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzClientContext2";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzClientContext2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext2Impl, const OFFSET: isize>() -> IAzClientContext2Vtbl {
        unsafe extern "system" fn GetAssignedScopesPage<Impl: IAzClientContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loptions: i32, pagesize: i32, pvarcursor: *mut super::super::System::Com::VARIANT, pvarscopenames: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetAssignedScopesPage(loptions, pagesize, &*(&pvarcursor as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarscopenames)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRoles<Impl: IAzClientContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varroles: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddRoles(&*(&varroles as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&bstrscopename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddApplicationGroups<Impl: IAzClientContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varapplicationgroups: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddApplicationGroups(&*(&varapplicationgroups as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddStringSids<Impl: IAzClientContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varstringsids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddStringSids(&*(&varstringsids as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLDAPQueryDN<Impl: IAzClientContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrldapquerydn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetLDAPQueryDN(&*(&bstrldapquerydn as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn LDAPQueryDN<Impl: IAzClientContext2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrldapquerydn: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LDAPQueryDN(::core::mem::transmute_copy(&pbstrldapquerydn)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAzClientContext2>, ::windows::core::GetTrustLevel, GetAssignedScopesPage::<Impl, OFFSET>, AddRoles::<Impl, OFFSET>, AddApplicationGroups::<Impl, OFFSET>, AddStringSids::<Impl, OFFSET>, SetLDAPQueryDN::<Impl, OFFSET>, LDAPQueryDN::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzClientContext3Impl: Sized + IAzClientContext2Impl + IAzClientContextImpl + IDispatchImpl {
    fn AccessCheck2();
    fn IsInRoleAssignment();
    fn GetOperations();
    fn GetTasks();
    fn BizRuleParameters();
    fn BizRuleInterfaces();
    fn GetGroups();
    fn Sids();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzClientContext3 {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzClientContext3";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzClientContext3Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext3Impl, const OFFSET: isize>() -> IAzClientContext3Vtbl {
        unsafe extern "system" fn AccessCheck2<Impl: IAzClientContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrobjectname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loperation: i32, plresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessCheck2(&*(&bstrobjectname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrscopename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), loperation, ::core::mem::transmute_copy(&plresult)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInRoleAssignment<Impl: IAzClientContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbisinrole: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInRoleAssignment(&*(&bstrscopename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrrolename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbisinrole)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOperations<Impl: IAzClientContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppoperationcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOperations(&*(&bstrscopename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppoperationcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTasks<Impl: IAzClientContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptaskcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTasks(&*(&bstrscopename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pptaskcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BizRuleParameters<Impl: IAzClientContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbizruleparam: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BizRuleParameters(::core::mem::transmute_copy(&ppbizruleparam)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BizRuleInterfaces<Impl: IAzClientContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbizruleinterfaces: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BizRuleInterfaces(::core::mem::transmute_copy(&ppbizruleinterfaces)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGroups<Impl: IAzClientContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, uloptions: AZ_PROP_CONSTANTS, pgrouparray: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGroups(&*(&bstrscopename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), uloptions, ::core::mem::transmute_copy(&pgrouparray)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sids<Impl: IAzClientContext3Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstringsidarray: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sids(::core::mem::transmute_copy(&pstringsidarray)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAzClientContext3>,
            ::windows::core::GetTrustLevel,
            AccessCheck2::<Impl, OFFSET>,
            IsInRoleAssignment::<Impl, OFFSET>,
            GetOperations::<Impl, OFFSET>,
            GetTasks::<Impl, OFFSET>,
            BizRuleParameters::<Impl, OFFSET>,
            BizRuleInterfaces::<Impl, OFFSET>,
            GetGroups::<Impl, OFFSET>,
            Sids::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzNameResolverImpl: Sized + IDispatchImpl {
    fn NameFromSid();
    fn NamesFromSids();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzNameResolver {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzNameResolver";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzNameResolverVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzNameResolverImpl, const OFFSET: isize>() -> IAzNameResolverVtbl {
        unsafe extern "system" fn NameFromSid<Impl: IAzNameResolverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psidtype: *mut i32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NameFromSid(&*(&bstrsid as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&psidtype), ::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NamesFromSids<Impl: IAzNameResolverImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vsids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvsidtypes: *mut super::super::System::Com::VARIANT, pvnames: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NamesFromSids(&*(&vsids as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvsidtypes), ::core::mem::transmute_copy(&pvnames)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAzNameResolver>, ::windows::core::GetTrustLevel, NameFromSid::<Impl, OFFSET>, NamesFromSids::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzObjectPickerImpl: Sized + IDispatchImpl {
    fn GetPrincipals();
    fn Name();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzObjectPicker {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzObjectPicker";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzObjectPickerVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzObjectPickerImpl, const OFFSET: isize>() -> IAzObjectPickerVtbl {
        unsafe extern "system" fn GetPrincipals<Impl: IAzObjectPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hparentwnd: super::super::Foundation::HWND, bstrtitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvsidtypes: *mut super::super::System::Com::VARIANT, pvnames: *mut super::super::System::Com::VARIANT, pvsids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetPrincipals(&*(&hparentwnd as *const <super::super::Foundation::HWND as ::windows::core::Abi>::Abi as *const <super::super::Foundation::HWND as ::windows::core::DefaultType>::DefaultType), &*(&bstrtitle as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvsidtypes), ::core::mem::transmute_copy(&pvnames), ::core::mem::transmute_copy(&pvsids)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Name<Impl: IAzObjectPickerImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAzObjectPicker>, ::windows::core::GetTrustLevel, GetPrincipals::<Impl, OFFSET>, Name::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzOperationImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetName();
    fn Description();
    fn SetDescription();
    fn ApplicationData();
    fn SetApplicationData();
    fn OperationID();
    fn SetOperationID();
    fn Writable();
    fn GetProperty();
    fn SetProperty();
    fn Submit();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzOperation {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzOperation";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzOperationVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzOperationImpl, const OFFSET: isize>() -> IAzOperationVtbl {
        unsafe extern "system" fn Name<Impl: IAzOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IAzOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IAzOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&pbstrdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IAzOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDescription(&*(&bstrdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationData<Impl: IAzOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationData(::core::mem::transmute_copy(&pbstrapplicationdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Impl: IAzOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetApplicationData(&*(&bstrapplicationdata as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OperationID<Impl: IAzOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OperationID(::core::mem::transmute_copy(&plprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOperationID<Impl: IAzOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetOperationID(lprop) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Writable<Impl: IAzOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Writable(::core::mem::transmute_copy(&pfprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IAzOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(lpropid, &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IAzOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProperty(lpropid, &*(&varprop as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Submit<Impl: IAzOperationImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Submit(lflags, &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAzOperation>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            SetDescription::<Impl, OFFSET>,
            ApplicationData::<Impl, OFFSET>,
            SetApplicationData::<Impl, OFFSET>,
            OperationID::<Impl, OFFSET>,
            SetOperationID::<Impl, OFFSET>,
            Writable::<Impl, OFFSET>,
            GetProperty::<Impl, OFFSET>,
            SetProperty::<Impl, OFFSET>,
            Submit::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzOperation2Impl: Sized + IAzOperationImpl + IDispatchImpl {
    fn RoleAssignments();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzOperation2 {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzOperation2";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzOperation2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzOperation2Impl, const OFFSET: isize>() -> IAzOperation2Vtbl {
        unsafe extern "system" fn RoleAssignments<Impl: IAzOperation2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, brecursive: i16, pproleassignments: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoleAssignments(&*(&bstrscopename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), brecursive, ::core::mem::transmute_copy(&pproleassignments)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAzOperation2>, ::windows::core::GetTrustLevel, RoleAssignments::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzOperationsImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzOperations {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzOperations";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzOperationsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzOperationsImpl, const OFFSET: isize>() -> IAzOperationsVtbl {
        unsafe extern "system" fn Item<Impl: IAzOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&pvarobtptr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IAzOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IAzOperationsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppenumptr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAzOperations>, ::windows::core::GetTrustLevel, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzPrincipalLocatorImpl: Sized + IDispatchImpl {
    fn NameResolver();
    fn ObjectPicker();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzPrincipalLocator {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzPrincipalLocator";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzPrincipalLocatorVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzPrincipalLocatorImpl, const OFFSET: isize>() -> IAzPrincipalLocatorVtbl {
        unsafe extern "system" fn NameResolver<Impl: IAzPrincipalLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnameresolver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NameResolver(::core::mem::transmute_copy(&ppnameresolver)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ObjectPicker<Impl: IAzPrincipalLocatorImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobjectpicker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ObjectPicker(::core::mem::transmute_copy(&ppobjectpicker)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAzPrincipalLocator>, ::windows::core::GetTrustLevel, NameResolver::<Impl, OFFSET>, ObjectPicker::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzRoleImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetName();
    fn Description();
    fn SetDescription();
    fn ApplicationData();
    fn SetApplicationData();
    fn AddAppMember();
    fn DeleteAppMember();
    fn AddTask();
    fn DeleteTask();
    fn AddOperation();
    fn DeleteOperation();
    fn AddMember();
    fn DeleteMember();
    fn Writable();
    fn GetProperty();
    fn SetProperty();
    fn AppMembers();
    fn Members();
    fn Operations();
    fn Tasks();
    fn AddPropertyItem();
    fn DeletePropertyItem();
    fn Submit();
    fn AddMemberName();
    fn DeleteMemberName();
    fn MembersName();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzRole {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzRole";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzRoleVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleImpl, const OFFSET: isize>() -> IAzRoleVtbl {
        unsafe extern "system" fn Name<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&pbstrdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDescription(&*(&bstrdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationData<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationData(::core::mem::transmute_copy(&pbstrapplicationdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetApplicationData(&*(&bstrapplicationdata as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddAppMember<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddAppMember(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteAppMember<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteAppMember(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTask<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddTask(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteTask<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteTask(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddOperation<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddOperation(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteOperation<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteOperation(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddMember<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddMember(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteMember<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteMember(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Writable<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Writable(::core::mem::transmute_copy(&pfprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(lpropid, &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProperty(lpropid, &*(&varprop as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppMembers<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppMembers(::core::mem::transmute_copy(&pvarprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Members<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Members(::core::mem::transmute_copy(&pvarprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Operations<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Operations(::core::mem::transmute_copy(&pvarprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tasks<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tasks(::core::mem::transmute_copy(&pvarprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPropertyItem<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPropertyItem(lpropid, &*(&varprop as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePropertyItem<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeletePropertyItem(lpropid, &*(&varprop as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Submit<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Submit(lflags, &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddMemberName<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddMemberName(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteMemberName<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteMemberName(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn MembersName<Impl: IAzRoleImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MembersName(::core::mem::transmute_copy(&pvarprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAzRole>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            SetDescription::<Impl, OFFSET>,
            ApplicationData::<Impl, OFFSET>,
            SetApplicationData::<Impl, OFFSET>,
            AddAppMember::<Impl, OFFSET>,
            DeleteAppMember::<Impl, OFFSET>,
            AddTask::<Impl, OFFSET>,
            DeleteTask::<Impl, OFFSET>,
            AddOperation::<Impl, OFFSET>,
            DeleteOperation::<Impl, OFFSET>,
            AddMember::<Impl, OFFSET>,
            DeleteMember::<Impl, OFFSET>,
            Writable::<Impl, OFFSET>,
            GetProperty::<Impl, OFFSET>,
            SetProperty::<Impl, OFFSET>,
            AppMembers::<Impl, OFFSET>,
            Members::<Impl, OFFSET>,
            Operations::<Impl, OFFSET>,
            Tasks::<Impl, OFFSET>,
            AddPropertyItem::<Impl, OFFSET>,
            DeletePropertyItem::<Impl, OFFSET>,
            Submit::<Impl, OFFSET>,
            AddMemberName::<Impl, OFFSET>,
            DeleteMemberName::<Impl, OFFSET>,
            MembersName::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzRoleAssignmentImpl: Sized + IAzRoleImpl + IDispatchImpl {
    fn AddRoleDefinition();
    fn DeleteRoleDefinition();
    fn RoleDefinitions();
    fn Scope();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzRoleAssignment {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzRoleAssignment";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzRoleAssignmentVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleAssignmentImpl, const OFFSET: isize>() -> IAzRoleAssignmentVtbl {
        unsafe extern "system" fn AddRoleDefinition<Impl: IAzRoleAssignmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinition: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddRoleDefinition(&*(&bstrroledefinition as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRoleDefinition<Impl: IAzRoleAssignmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinition: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteRoleDefinition(&*(&bstrroledefinition as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoleDefinitions<Impl: IAzRoleAssignmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoleDefinitions(::core::mem::transmute_copy(&pproledefinitions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scope<Impl: IAzRoleAssignmentImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppscope: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scope(::core::mem::transmute_copy(&ppscope)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAzRoleAssignment>, ::windows::core::GetTrustLevel, AddRoleDefinition::<Impl, OFFSET>, DeleteRoleDefinition::<Impl, OFFSET>, RoleDefinitions::<Impl, OFFSET>, Scope::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzRoleAssignmentsImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzRoleAssignments {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzRoleAssignments";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzRoleAssignmentsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleAssignmentsImpl, const OFFSET: isize>() -> IAzRoleAssignmentsVtbl {
        unsafe extern "system" fn Item<Impl: IAzRoleAssignmentsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&pvarobtptr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IAzRoleAssignmentsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IAzRoleAssignmentsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppenumptr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAzRoleAssignments>, ::windows::core::GetTrustLevel, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzRoleDefinitionImpl: Sized + IAzTaskImpl + IDispatchImpl {
    fn RoleAssignments();
    fn AddRoleDefinition();
    fn DeleteRoleDefinition();
    fn RoleDefinitions();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzRoleDefinition {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzRoleDefinition";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzRoleDefinitionVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleDefinitionImpl, const OFFSET: isize>() -> IAzRoleDefinitionVtbl {
        unsafe extern "system" fn RoleAssignments<Impl: IAzRoleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, brecursive: i16, pproleassignments: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoleAssignments(&*(&bstrscopename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), brecursive, ::core::mem::transmute_copy(&pproleassignments)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRoleDefinition<Impl: IAzRoleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinition: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddRoleDefinition(&*(&bstrroledefinition as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRoleDefinition<Impl: IAzRoleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinition: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteRoleDefinition(&*(&bstrroledefinition as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoleDefinitions<Impl: IAzRoleDefinitionImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoleDefinitions(::core::mem::transmute_copy(&pproledefinitions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAzRoleDefinition>, ::windows::core::GetTrustLevel, RoleAssignments::<Impl, OFFSET>, AddRoleDefinition::<Impl, OFFSET>, DeleteRoleDefinition::<Impl, OFFSET>, RoleDefinitions::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzRoleDefinitionsImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzRoleDefinitions {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzRoleDefinitions";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzRoleDefinitionsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleDefinitionsImpl, const OFFSET: isize>() -> IAzRoleDefinitionsVtbl {
        unsafe extern "system" fn Item<Impl: IAzRoleDefinitionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&pvarobtptr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IAzRoleDefinitionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IAzRoleDefinitionsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppenumptr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAzRoleDefinitions>, ::windows::core::GetTrustLevel, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzRolesImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzRoles {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzRoles";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzRolesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzRolesImpl, const OFFSET: isize>() -> IAzRolesVtbl {
        unsafe extern "system" fn Item<Impl: IAzRolesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&pvarobtptr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IAzRolesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IAzRolesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppenumptr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAzRoles>, ::windows::core::GetTrustLevel, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzScopeImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetName();
    fn Description();
    fn SetDescription();
    fn ApplicationData();
    fn SetApplicationData();
    fn Writable();
    fn GetProperty();
    fn SetProperty();
    fn AddPropertyItem();
    fn DeletePropertyItem();
    fn PolicyAdministrators();
    fn PolicyReaders();
    fn AddPolicyAdministrator();
    fn DeletePolicyAdministrator();
    fn AddPolicyReader();
    fn DeletePolicyReader();
    fn ApplicationGroups();
    fn OpenApplicationGroup();
    fn CreateApplicationGroup();
    fn DeleteApplicationGroup();
    fn Roles();
    fn OpenRole();
    fn CreateRole();
    fn DeleteRole();
    fn Tasks();
    fn OpenTask();
    fn CreateTask();
    fn DeleteTask();
    fn Submit();
    fn CanBeDelegated();
    fn BizrulesWritable();
    fn PolicyAdministratorsName();
    fn PolicyReadersName();
    fn AddPolicyAdministratorName();
    fn DeletePolicyAdministratorName();
    fn AddPolicyReaderName();
    fn DeletePolicyReaderName();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzScope {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzScope";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzScopeVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzScopeImpl, const OFFSET: isize>() -> IAzScopeVtbl {
        unsafe extern "system" fn Name<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&pbstrdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDescription(&*(&bstrdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationData<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationData(::core::mem::transmute_copy(&pbstrapplicationdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetApplicationData(&*(&bstrapplicationdata as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Writable<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Writable(::core::mem::transmute_copy(&pfprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(lpropid, &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProperty(lpropid, &*(&varprop as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPropertyItem<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPropertyItem(lpropid, &*(&varprop as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePropertyItem<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeletePropertyItem(lpropid, &*(&varprop as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyAdministrators<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyAdministrators(::core::mem::transmute_copy(&pvaradmins)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReaders<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyReaders(::core::mem::transmute_copy(&pvarreaders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministrator<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPolicyAdministrator(&*(&bstradmin as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePolicyAdministrator<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeletePolicyAdministrator(&*(&bstradmin as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyReader<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPolicyReader(&*(&bstrreader as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePolicyReader<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeletePolicyReader(&*(&bstrreader as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationGroups<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroupcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationGroups(::core::mem::transmute_copy(&ppgroupcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenApplicationGroup<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenApplicationGroup(&*(&bstrgroupname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateApplicationGroup<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateApplicationGroup(&*(&bstrgroupname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppgroup)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteApplicationGroup<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteApplicationGroup(&*(&bstrgroupname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Roles<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprolecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Roles(::core::mem::transmute_copy(&pprolecollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRole<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pprole: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenRole(&*(&bstrrolename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprole)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRole<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pprole: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRole(&*(&bstrrolename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pprole)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRole<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteRole(&*(&bstrrolename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tasks<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptaskcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tasks(::core::mem::transmute_copy(&pptaskcollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTask<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pptask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenTask(&*(&bstrtaskname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pptask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTask<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pptask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTask(&*(&bstrtaskname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pptask)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteTask<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteTask(&*(&bstrtaskname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Submit<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Submit(lflags, &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CanBeDelegated<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanBeDelegated(::core::mem::transmute_copy(&pfprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BizrulesWritable<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BizrulesWritable(::core::mem::transmute_copy(&pfprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyAdministratorsName<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyAdministratorsName(::core::mem::transmute_copy(&pvaradmins)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReadersName<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyReadersName(::core::mem::transmute_copy(&pvarreaders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministratorName<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPolicyAdministratorName(&*(&bstradmin as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePolicyAdministratorName<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeletePolicyAdministratorName(&*(&bstradmin as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyReaderName<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPolicyReaderName(&*(&bstrreader as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePolicyReaderName<Impl: IAzScopeImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeletePolicyReaderName(&*(&bstrreader as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAzScope>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            SetDescription::<Impl, OFFSET>,
            ApplicationData::<Impl, OFFSET>,
            SetApplicationData::<Impl, OFFSET>,
            Writable::<Impl, OFFSET>,
            GetProperty::<Impl, OFFSET>,
            SetProperty::<Impl, OFFSET>,
            AddPropertyItem::<Impl, OFFSET>,
            DeletePropertyItem::<Impl, OFFSET>,
            PolicyAdministrators::<Impl, OFFSET>,
            PolicyReaders::<Impl, OFFSET>,
            AddPolicyAdministrator::<Impl, OFFSET>,
            DeletePolicyAdministrator::<Impl, OFFSET>,
            AddPolicyReader::<Impl, OFFSET>,
            DeletePolicyReader::<Impl, OFFSET>,
            ApplicationGroups::<Impl, OFFSET>,
            OpenApplicationGroup::<Impl, OFFSET>,
            CreateApplicationGroup::<Impl, OFFSET>,
            DeleteApplicationGroup::<Impl, OFFSET>,
            Roles::<Impl, OFFSET>,
            OpenRole::<Impl, OFFSET>,
            CreateRole::<Impl, OFFSET>,
            DeleteRole::<Impl, OFFSET>,
            Tasks::<Impl, OFFSET>,
            OpenTask::<Impl, OFFSET>,
            CreateTask::<Impl, OFFSET>,
            DeleteTask::<Impl, OFFSET>,
            Submit::<Impl, OFFSET>,
            CanBeDelegated::<Impl, OFFSET>,
            BizrulesWritable::<Impl, OFFSET>,
            PolicyAdministratorsName::<Impl, OFFSET>,
            PolicyReadersName::<Impl, OFFSET>,
            AddPolicyAdministratorName::<Impl, OFFSET>,
            DeletePolicyAdministratorName::<Impl, OFFSET>,
            AddPolicyReaderName::<Impl, OFFSET>,
            DeletePolicyReaderName::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzScope2Impl: Sized + IAzScopeImpl + IDispatchImpl {
    fn RoleDefinitions();
    fn CreateRoleDefinition();
    fn OpenRoleDefinition();
    fn DeleteRoleDefinition();
    fn RoleAssignments();
    fn CreateRoleAssignment();
    fn OpenRoleAssignment();
    fn DeleteRoleAssignment();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzScope2 {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzScope2";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzScope2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope2Impl, const OFFSET: isize>() -> IAzScope2Vtbl {
        unsafe extern "system" fn RoleDefinitions<Impl: IAzScope2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoleDefinitions(::core::mem::transmute_copy(&pproledefinitions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRoleDefinition<Impl: IAzScope2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRoleDefinition(&*(&bstrroledefinitionname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pproledefinitions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRoleDefinition<Impl: IAzScope2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenRoleDefinition(&*(&bstrroledefinitionname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pproledefinitions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRoleDefinition<Impl: IAzScope2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteRoleDefinition(&*(&bstrroledefinitionname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoleAssignments<Impl: IAzScope2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproleassignments: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoleAssignments(::core::mem::transmute_copy(&pproleassignments)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRoleAssignment<Impl: IAzScope2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproleassignment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRoleAssignment(&*(&bstrroleassignmentname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pproleassignment)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRoleAssignment<Impl: IAzScope2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproleassignment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenRoleAssignment(&*(&bstrroleassignmentname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pproleassignment)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRoleAssignment<Impl: IAzScope2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteRoleAssignment(&*(&bstrroleassignmentname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAzScope2>,
            ::windows::core::GetTrustLevel,
            RoleDefinitions::<Impl, OFFSET>,
            CreateRoleDefinition::<Impl, OFFSET>,
            OpenRoleDefinition::<Impl, OFFSET>,
            DeleteRoleDefinition::<Impl, OFFSET>,
            RoleAssignments::<Impl, OFFSET>,
            CreateRoleAssignment::<Impl, OFFSET>,
            OpenRoleAssignment::<Impl, OFFSET>,
            DeleteRoleAssignment::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzScopesImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzScopes {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzScopes";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzScopesVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzScopesImpl, const OFFSET: isize>() -> IAzScopesVtbl {
        unsafe extern "system" fn Item<Impl: IAzScopesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&pvarobtptr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IAzScopesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IAzScopesImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppenumptr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAzScopes>, ::windows::core::GetTrustLevel, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzTaskImpl: Sized + IDispatchImpl {
    fn Name();
    fn SetName();
    fn Description();
    fn SetDescription();
    fn ApplicationData();
    fn SetApplicationData();
    fn BizRule();
    fn SetBizRule();
    fn BizRuleLanguage();
    fn SetBizRuleLanguage();
    fn BizRuleImportedPath();
    fn SetBizRuleImportedPath();
    fn IsRoleDefinition();
    fn SetIsRoleDefinition();
    fn Operations();
    fn Tasks();
    fn AddOperation();
    fn DeleteOperation();
    fn AddTask();
    fn DeleteTask();
    fn Writable();
    fn GetProperty();
    fn SetProperty();
    fn AddPropertyItem();
    fn DeletePropertyItem();
    fn Submit();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzTask {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzTask";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzTaskVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzTaskImpl, const OFFSET: isize>() -> IAzTaskVtbl {
        unsafe extern "system" fn Name<Impl: IAzTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name(::core::mem::transmute_copy(&pbstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IAzTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetName(&*(&bstrname as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IAzTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description(::core::mem::transmute_copy(&pbstrdescription)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IAzTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetDescription(&*(&bstrdescription as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplicationData<Impl: IAzTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationData(::core::mem::transmute_copy(&pbstrapplicationdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Impl: IAzTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetApplicationData(&*(&bstrapplicationdata as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BizRule<Impl: IAzTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BizRule(::core::mem::transmute_copy(&pbstrprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRule<Impl: IAzTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBizRule(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BizRuleLanguage<Impl: IAzTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BizRuleLanguage(::core::mem::transmute_copy(&pbstrprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRuleLanguage<Impl: IAzTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBizRuleLanguage(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BizRuleImportedPath<Impl: IAzTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BizRuleImportedPath(::core::mem::transmute_copy(&pbstrprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRuleImportedPath<Impl: IAzTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetBizRuleImportedPath(&*(&bstrprop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsRoleDefinition<Impl: IAzTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRoleDefinition(::core::mem::transmute_copy(&pfprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRoleDefinition<Impl: IAzTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fprop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetIsRoleDefinition(&*(&fprop as *const <super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Operations<Impl: IAzTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Operations(::core::mem::transmute_copy(&pvarprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tasks<Impl: IAzTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tasks(::core::mem::transmute_copy(&pvarprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddOperation<Impl: IAzTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddOperation(&*(&bstrop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteOperation<Impl: IAzTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteOperation(&*(&bstrop as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddTask<Impl: IAzTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddTask(&*(&bstrtask as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteTask<Impl: IAzTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeleteTask(&*(&bstrtask as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Writable<Impl: IAzTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Writable(::core::mem::transmute_copy(&pfprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IAzTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(lpropid, &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarprop)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IAzTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SetProperty(lpropid, &*(&varprop as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPropertyItem<Impl: IAzTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AddPropertyItem(lpropid, &*(&varprop as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeletePropertyItem<Impl: IAzTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DeletePropertyItem(lpropid, &*(&varprop as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Submit<Impl: IAzTaskImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Submit(lflags, &*(&varreserved as *const <super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            ::windows::core::QueryInterface::<Identity, OFFSET>,
            ::windows::core::AddRef::<Identity, OFFSET>,
            ::windows::core::Release::<Identity, OFFSET>,
            ::windows::core::GetIids,
            ::windows::core::GetRuntimeClassName::<IAzTask>,
            ::windows::core::GetTrustLevel,
            Name::<Impl, OFFSET>,
            SetName::<Impl, OFFSET>,
            Description::<Impl, OFFSET>,
            SetDescription::<Impl, OFFSET>,
            ApplicationData::<Impl, OFFSET>,
            SetApplicationData::<Impl, OFFSET>,
            BizRule::<Impl, OFFSET>,
            SetBizRule::<Impl, OFFSET>,
            BizRuleLanguage::<Impl, OFFSET>,
            SetBizRuleLanguage::<Impl, OFFSET>,
            BizRuleImportedPath::<Impl, OFFSET>,
            SetBizRuleImportedPath::<Impl, OFFSET>,
            IsRoleDefinition::<Impl, OFFSET>,
            SetIsRoleDefinition::<Impl, OFFSET>,
            Operations::<Impl, OFFSET>,
            Tasks::<Impl, OFFSET>,
            AddOperation::<Impl, OFFSET>,
            DeleteOperation::<Impl, OFFSET>,
            AddTask::<Impl, OFFSET>,
            DeleteTask::<Impl, OFFSET>,
            Writable::<Impl, OFFSET>,
            GetProperty::<Impl, OFFSET>,
            SetProperty::<Impl, OFFSET>,
            AddPropertyItem::<Impl, OFFSET>,
            DeletePropertyItem::<Impl, OFFSET>,
            Submit::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzTask2Impl: Sized + IAzTaskImpl + IDispatchImpl {
    fn RoleAssignments();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzTask2 {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzTask2";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzTask2Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask2Impl, const OFFSET: isize>() -> IAzTask2Vtbl {
        unsafe extern "system" fn RoleAssignments<Impl: IAzTask2Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, brecursive: i16, pproleassignments: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoleAssignments(&*(&bstrscopename as *const <super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), brecursive, ::core::mem::transmute_copy(&pproleassignments)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAzTask2>, ::windows::core::GetTrustLevel, RoleAssignments::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IAzTasksImpl: Sized + IDispatchImpl {
    fn Item();
    fn Count();
    fn _NewEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IAzTasks {
    const NAME: &'static str = "Windows.Win32.Security.Authorization.IAzTasks";
}
#[cfg(feature = "Win32_System_Com")]
impl IAzTasksVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzTasksImpl, const OFFSET: isize>() -> IAzTasksVtbl {
        unsafe extern "system" fn Item<Impl: IAzTasksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(index, ::core::mem::transmute_copy(&pvarobtptr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IAzTasksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count(::core::mem::transmute_copy(&plcount)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IAzTasksImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum(::core::mem::transmute_copy(&ppenumptr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IAzTasks>, ::windows::core::GetTrustLevel, Item::<Impl, OFFSET>, Count::<Impl, OFFSET>, _NewEnum::<Impl, OFFSET>)
    }
}
