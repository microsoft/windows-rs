#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzApplication_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ApplicationData(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetApplicationData(&mut self, bstrapplicationdata: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AuthzInterfaceClsid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetAuthzInterfaceClsid(&mut self, bstrprop: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Version(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetVersion(&mut self, bstrprop: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GenerateAudits(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetGenerateAudits(&mut self, bprop: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ApplyStoreSacl(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetApplyStoreSacl(&mut self, bprop: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Writable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&mut self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&mut self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PolicyAdministrators(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PolicyReaders(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddPolicyAdministrator(&mut self, bstradmin: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyAdministrator(&mut self, bstradmin: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPolicyReader(&mut self, bstrreader: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyReader(&mut self, bstrreader: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Scopes(&mut self) -> ::windows::core::Result<IAzScopes>;
    fn OpenScope(&mut self, bstrscopename: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzScope>;
    fn CreateScope(&mut self, bstrscopename: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzScope>;
    fn DeleteScope(&mut self, bstrscopename: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Operations(&mut self) -> ::windows::core::Result<IAzOperations>;
    fn OpenOperation(&mut self, bstroperationname: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzOperation>;
    fn CreateOperation(&mut self, bstroperationname: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzOperation>;
    fn DeleteOperation(&mut self, bstroperationname: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Tasks(&mut self) -> ::windows::core::Result<IAzTasks>;
    fn OpenTask(&mut self, bstrtaskname: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzTask>;
    fn CreateTask(&mut self, bstrtaskname: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzTask>;
    fn DeleteTask(&mut self, bstrtaskname: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ApplicationGroups(&mut self) -> ::windows::core::Result<IAzApplicationGroups>;
    fn OpenApplicationGroup(&mut self, bstrgroupname: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>;
    fn CreateApplicationGroup(&mut self, bstrgroupname: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>;
    fn DeleteApplicationGroup(&mut self, bstrgroupname: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Roles(&mut self) -> ::windows::core::Result<IAzRoles>;
    fn OpenRole(&mut self, bstrrolename: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzRole>;
    fn CreateRole(&mut self, bstrrolename: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzRole>;
    fn DeleteRole(&mut self, bstrrolename: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn InitializeClientContextFromToken(&mut self, ulltokenhandle: u64, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext>;
    fn AddPropertyItem(&mut self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePropertyItem(&mut self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Submit(&mut self, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn InitializeClientContextFromName(&mut self, clientname: super::super::Foundation::BSTR, domainname: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext>;
    fn DelegatedPolicyUsers(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddDelegatedPolicyUser(&mut self, bstrdelegatedpolicyuser: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteDelegatedPolicyUser(&mut self, bstrdelegatedpolicyuser: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn InitializeClientContextFromStringSid(&mut self, sidstring: super::super::Foundation::BSTR, loptions: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext>;
    fn PolicyAdministratorsName(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PolicyReadersName(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddPolicyAdministratorName(&mut self, bstradmin: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyAdministratorName(&mut self, bstradmin: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPolicyReaderName(&mut self, bstrreader: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyReaderName(&mut self, bstrreader: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DelegatedPolicyUsersName(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddDelegatedPolicyUserName(&mut self, bstrdelegatedpolicyuser: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteDelegatedPolicyUserName(&mut self, bstrdelegatedpolicyuser: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzApplication_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzApplication_Vtbl {
        unsafe extern "system" fn Name<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn Description<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn ApplicationData<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationData() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrapplicationdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetApplicationData(::core::mem::transmute_copy(&bstrapplicationdata)).into()
        }
        unsafe extern "system" fn AuthzInterfaceClsid<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AuthzInterfaceClsid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthzInterfaceClsid<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetAuthzInterfaceClsid(::core::mem::transmute_copy(&bstrprop)).into()
        }
        unsafe extern "system" fn Version<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Version() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVersion<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetVersion(::core::mem::transmute_copy(&bstrprop)).into()
        }
        unsafe extern "system" fn GenerateAudits<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerateAudits() {
                ::core::result::Result::Ok(ok__) => {
                    *pbprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGenerateAudits<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bprop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGenerateAudits(::core::mem::transmute_copy(&bprop)).into()
        }
        unsafe extern "system" fn ApplyStoreSacl<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplyStoreSacl() {
                ::core::result::Result::Ok(ok__) => {
                    *pbprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplyStoreSacl<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bprop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetApplyStoreSacl(::core::mem::transmute_copy(&bprop)).into()
        }
        unsafe extern "system" fn Writable<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Writable() {
                ::core::result::Result::Ok(ok__) => {
                    *pfprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn PolicyAdministrators<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyAdministrators() {
                ::core::result::Result::Ok(ok__) => {
                    *pvaradmins = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReaders<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyReaders() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarreaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministrator<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPolicyAdministrator(::core::mem::transmute_copy(&bstradmin), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyAdministrator<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeletePolicyAdministrator(::core::mem::transmute_copy(&bstradmin), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddPolicyReader<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPolicyReader(::core::mem::transmute_copy(&bstrreader), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyReader<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeletePolicyReader(::core::mem::transmute_copy(&bstrreader), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Scopes<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppscopecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scopes() {
                ::core::result::Result::Ok(ok__) => {
                    *ppscopecollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenScope<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppscope: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenScope(::core::mem::transmute_copy(&bstrscopename), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppscope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScope<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppscope: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateScope(::core::mem::transmute_copy(&bstrscopename), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppscope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteScope<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteScope(::core::mem::transmute_copy(&bstrscopename), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Operations<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoperationcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Operations() {
                ::core::result::Result::Ok(ok__) => {
                    *ppoperationcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenOperation<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroperationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppoperation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenOperation(::core::mem::transmute_copy(&bstroperationname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoperation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOperation<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroperationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppoperation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateOperation(::core::mem::transmute_copy(&bstroperationname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoperation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteOperation<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroperationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteOperation(::core::mem::transmute_copy(&bstroperationname), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Tasks<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptaskcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tasks() {
                ::core::result::Result::Ok(ok__) => {
                    *pptaskcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTask<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pptask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenTask(::core::mem::transmute_copy(&bstrtaskname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTask<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pptask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTask(::core::mem::transmute_copy(&bstrtaskname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteTask<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteTask(::core::mem::transmute_copy(&bstrtaskname), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn ApplicationGroups<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroupcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroupcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenApplicationGroup<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenApplicationGroup(::core::mem::transmute_copy(&bstrgroupname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateApplicationGroup<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateApplicationGroup(::core::mem::transmute_copy(&bstrgroupname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteApplicationGroup<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteApplicationGroup(::core::mem::transmute_copy(&bstrgroupname), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Roles<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprolecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Roles() {
                ::core::result::Result::Ok(ok__) => {
                    *pprolecollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRole<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pprole: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenRole(::core::mem::transmute_copy(&bstrrolename), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprole = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRole<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pprole: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRole(::core::mem::transmute_copy(&bstrrolename), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprole = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRole<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteRole(::core::mem::transmute_copy(&bstrrolename), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn InitializeClientContextFromToken<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulltokenhandle: u64, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclientcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeClientContextFromToken(::core::mem::transmute_copy(&ulltokenhandle), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclientcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPropertyItem<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePropertyItem<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeletePropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn InitializeClientContextFromName<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, domainname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclientcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeClientContextFromName(::core::mem::transmute_copy(&clientname), ::core::mem::transmute_copy(&domainname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclientcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DelegatedPolicyUsers<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DelegatedPolicyUsers() {
                ::core::result::Result::Ok(ok__) => {
                    *pvardelegatedpolicyusers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDelegatedPolicyUser<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDelegatedPolicyUser(::core::mem::transmute_copy(&bstrdelegatedpolicyuser), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUser<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteDelegatedPolicyUser(::core::mem::transmute_copy(&bstrdelegatedpolicyuser), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn InitializeClientContextFromStringSid<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sidstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclientcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeClientContextFromStringSid(::core::mem::transmute_copy(&sidstring), ::core::mem::transmute_copy(&loptions), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclientcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyAdministratorsName<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyAdministratorsName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvaradmins = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReadersName<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyReadersName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarreaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministratorName<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPolicyAdministratorName(::core::mem::transmute_copy(&bstradmin), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyAdministratorName<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeletePolicyAdministratorName(::core::mem::transmute_copy(&bstradmin), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddPolicyReaderName<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPolicyReaderName(::core::mem::transmute_copy(&bstrreader), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyReaderName<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeletePolicyReaderName(::core::mem::transmute_copy(&bstrreader), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DelegatedPolicyUsersName<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DelegatedPolicyUsersName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvardelegatedpolicyusers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDelegatedPolicyUserName<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDelegatedPolicyUserName(::core::mem::transmute_copy(&bstrdelegatedpolicyuser), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUserName<Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteDelegatedPolicyUserName(::core::mem::transmute_copy(&bstrdelegatedpolicyuser), ::core::mem::transmute_copy(&varreserved)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            ApplicationData: ApplicationData::<Impl, IMPL_OFFSET>,
            SetApplicationData: SetApplicationData::<Impl, IMPL_OFFSET>,
            AuthzInterfaceClsid: AuthzInterfaceClsid::<Impl, IMPL_OFFSET>,
            SetAuthzInterfaceClsid: SetAuthzInterfaceClsid::<Impl, IMPL_OFFSET>,
            Version: Version::<Impl, IMPL_OFFSET>,
            SetVersion: SetVersion::<Impl, IMPL_OFFSET>,
            GenerateAudits: GenerateAudits::<Impl, IMPL_OFFSET>,
            SetGenerateAudits: SetGenerateAudits::<Impl, IMPL_OFFSET>,
            ApplyStoreSacl: ApplyStoreSacl::<Impl, IMPL_OFFSET>,
            SetApplyStoreSacl: SetApplyStoreSacl::<Impl, IMPL_OFFSET>,
            Writable: Writable::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            PolicyAdministrators: PolicyAdministrators::<Impl, IMPL_OFFSET>,
            PolicyReaders: PolicyReaders::<Impl, IMPL_OFFSET>,
            AddPolicyAdministrator: AddPolicyAdministrator::<Impl, IMPL_OFFSET>,
            DeletePolicyAdministrator: DeletePolicyAdministrator::<Impl, IMPL_OFFSET>,
            AddPolicyReader: AddPolicyReader::<Impl, IMPL_OFFSET>,
            DeletePolicyReader: DeletePolicyReader::<Impl, IMPL_OFFSET>,
            Scopes: Scopes::<Impl, IMPL_OFFSET>,
            OpenScope: OpenScope::<Impl, IMPL_OFFSET>,
            CreateScope: CreateScope::<Impl, IMPL_OFFSET>,
            DeleteScope: DeleteScope::<Impl, IMPL_OFFSET>,
            Operations: Operations::<Impl, IMPL_OFFSET>,
            OpenOperation: OpenOperation::<Impl, IMPL_OFFSET>,
            CreateOperation: CreateOperation::<Impl, IMPL_OFFSET>,
            DeleteOperation: DeleteOperation::<Impl, IMPL_OFFSET>,
            Tasks: Tasks::<Impl, IMPL_OFFSET>,
            OpenTask: OpenTask::<Impl, IMPL_OFFSET>,
            CreateTask: CreateTask::<Impl, IMPL_OFFSET>,
            DeleteTask: DeleteTask::<Impl, IMPL_OFFSET>,
            ApplicationGroups: ApplicationGroups::<Impl, IMPL_OFFSET>,
            OpenApplicationGroup: OpenApplicationGroup::<Impl, IMPL_OFFSET>,
            CreateApplicationGroup: CreateApplicationGroup::<Impl, IMPL_OFFSET>,
            DeleteApplicationGroup: DeleteApplicationGroup::<Impl, IMPL_OFFSET>,
            Roles: Roles::<Impl, IMPL_OFFSET>,
            OpenRole: OpenRole::<Impl, IMPL_OFFSET>,
            CreateRole: CreateRole::<Impl, IMPL_OFFSET>,
            DeleteRole: DeleteRole::<Impl, IMPL_OFFSET>,
            InitializeClientContextFromToken: InitializeClientContextFromToken::<Impl, IMPL_OFFSET>,
            AddPropertyItem: AddPropertyItem::<Impl, IMPL_OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Impl, IMPL_OFFSET>,
            Submit: Submit::<Impl, IMPL_OFFSET>,
            InitializeClientContextFromName: InitializeClientContextFromName::<Impl, IMPL_OFFSET>,
            DelegatedPolicyUsers: DelegatedPolicyUsers::<Impl, IMPL_OFFSET>,
            AddDelegatedPolicyUser: AddDelegatedPolicyUser::<Impl, IMPL_OFFSET>,
            DeleteDelegatedPolicyUser: DeleteDelegatedPolicyUser::<Impl, IMPL_OFFSET>,
            InitializeClientContextFromStringSid: InitializeClientContextFromStringSid::<Impl, IMPL_OFFSET>,
            PolicyAdministratorsName: PolicyAdministratorsName::<Impl, IMPL_OFFSET>,
            PolicyReadersName: PolicyReadersName::<Impl, IMPL_OFFSET>,
            AddPolicyAdministratorName: AddPolicyAdministratorName::<Impl, IMPL_OFFSET>,
            DeletePolicyAdministratorName: DeletePolicyAdministratorName::<Impl, IMPL_OFFSET>,
            AddPolicyReaderName: AddPolicyReaderName::<Impl, IMPL_OFFSET>,
            DeletePolicyReaderName: DeletePolicyReaderName::<Impl, IMPL_OFFSET>,
            DelegatedPolicyUsersName: DelegatedPolicyUsersName::<Impl, IMPL_OFFSET>,
            AddDelegatedPolicyUserName: AddDelegatedPolicyUserName::<Impl, IMPL_OFFSET>,
            DeleteDelegatedPolicyUserName: DeleteDelegatedPolicyUserName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzApplication as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzApplication2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IAzApplication_Impl {
    fn InitializeClientContextFromToken2(&mut self, ultokenhandlelowpart: u32, ultokenhandlehighpart: u32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext2>;
    fn InitializeClientContext2(&mut self, identifyingstring: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzApplication2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzApplication2_Vtbl {
        unsafe extern "system" fn InitializeClientContextFromToken2<Impl: IAzApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ultokenhandlelowpart: u32, ultokenhandlehighpart: u32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclientcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeClientContextFromToken2(::core::mem::transmute_copy(&ultokenhandlelowpart), ::core::mem::transmute_copy(&ultokenhandlehighpart), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclientcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeClientContext2<Impl: IAzApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifyingstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclientcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).InitializeClientContext2(::core::mem::transmute_copy(&identifyingstring), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclientcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IAzApplication_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            InitializeClientContextFromToken2: InitializeClientContextFromToken2::<Impl, IMPL_OFFSET>,
            InitializeClientContext2: InitializeClientContext2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzApplication2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzApplication3_Impl: Sized + super::super::System::Com::IDispatch_Impl + IAzApplication_Impl + IAzApplication2_Impl {
    fn ScopeExists(&mut self, bstrscopename: super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn OpenScope2(&mut self, bstrscopename: super::super::Foundation::BSTR) -> ::windows::core::Result<IAzScope2>;
    fn CreateScope2(&mut self, bstrscopename: super::super::Foundation::BSTR) -> ::windows::core::Result<IAzScope2>;
    fn DeleteScope2(&mut self, bstrscopename: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RoleDefinitions(&mut self) -> ::windows::core::Result<IAzRoleDefinitions>;
    fn CreateRoleDefinition(&mut self, bstrroledefinitionname: super::super::Foundation::BSTR) -> ::windows::core::Result<IAzRoleDefinition>;
    fn OpenRoleDefinition(&mut self, bstrroledefinitionname: super::super::Foundation::BSTR) -> ::windows::core::Result<IAzRoleDefinition>;
    fn DeleteRoleDefinition(&mut self, bstrroledefinitionname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RoleAssignments(&mut self) -> ::windows::core::Result<IAzRoleAssignments>;
    fn CreateRoleAssignment(&mut self, bstrroleassignmentname: super::super::Foundation::BSTR) -> ::windows::core::Result<IAzRoleAssignment>;
    fn OpenRoleAssignment(&mut self, bstrroleassignmentname: super::super::Foundation::BSTR) -> ::windows::core::Result<IAzRoleAssignment>;
    fn DeleteRoleAssignment(&mut self, bstrroleassignmentname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BizRulesEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetBizRulesEnabled(&mut self, benabled: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzApplication3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzApplication3_Vtbl {
        unsafe extern "system" fn ScopeExists<Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbexist: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScopeExists(::core::mem::transmute_copy(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbexist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenScope2<Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppscope2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenScope2(::core::mem::transmute_copy(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppscope2 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScope2<Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppscope2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateScope2(::core::mem::transmute_copy(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppscope2 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteScope2<Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteScope2(::core::mem::transmute_copy(&bstrscopename)).into()
        }
        unsafe extern "system" fn RoleDefinitions<Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoleDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    *pproledefinitions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRoleDefinition<Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRoleDefinition(::core::mem::transmute_copy(&bstrroledefinitionname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproledefinitions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRoleDefinition<Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenRoleDefinition(::core::mem::transmute_copy(&bstrroledefinitionname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproledefinitions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRoleDefinition<Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteRoleDefinition(::core::mem::transmute_copy(&bstrroledefinitionname)).into()
        }
        unsafe extern "system" fn RoleAssignments<Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproleassignments: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoleAssignments() {
                ::core::result::Result::Ok(ok__) => {
                    *pproleassignments = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRoleAssignment<Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproleassignment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRoleAssignment(::core::mem::transmute_copy(&bstrroleassignmentname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproleassignment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRoleAssignment<Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproleassignment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenRoleAssignment(::core::mem::transmute_copy(&bstrroleassignmentname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproleassignment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRoleAssignment<Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteRoleAssignment(::core::mem::transmute_copy(&bstrroleassignmentname)).into()
        }
        unsafe extern "system" fn BizRulesEnabled<Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BizRulesEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pbenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRulesEnabled<Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBizRulesEnabled(::core::mem::transmute_copy(&benabled)).into()
        }
        Self {
            base: IAzApplication2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            ScopeExists: ScopeExists::<Impl, IMPL_OFFSET>,
            OpenScope2: OpenScope2::<Impl, IMPL_OFFSET>,
            CreateScope2: CreateScope2::<Impl, IMPL_OFFSET>,
            DeleteScope2: DeleteScope2::<Impl, IMPL_OFFSET>,
            RoleDefinitions: RoleDefinitions::<Impl, IMPL_OFFSET>,
            CreateRoleDefinition: CreateRoleDefinition::<Impl, IMPL_OFFSET>,
            OpenRoleDefinition: OpenRoleDefinition::<Impl, IMPL_OFFSET>,
            DeleteRoleDefinition: DeleteRoleDefinition::<Impl, IMPL_OFFSET>,
            RoleAssignments: RoleAssignments::<Impl, IMPL_OFFSET>,
            CreateRoleAssignment: CreateRoleAssignment::<Impl, IMPL_OFFSET>,
            OpenRoleAssignment: OpenRoleAssignment::<Impl, IMPL_OFFSET>,
            DeleteRoleAssignment: DeleteRoleAssignment::<Impl, IMPL_OFFSET>,
            BizRulesEnabled: BizRulesEnabled::<Impl, IMPL_OFFSET>,
            SetBizRulesEnabled: SetBizRulesEnabled::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzApplication3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzApplicationGroup_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Type(&mut self) -> ::windows::core::Result<i32>;
    fn SetType(&mut self, lprop: i32) -> ::windows::core::Result<()>;
    fn LdapQuery(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLdapQuery(&mut self, bstrprop: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AppMembers(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AppNonMembers(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Members(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn NonMembers(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddAppMember(&mut self, bstrprop: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteAppMember(&mut self, bstrprop: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddAppNonMember(&mut self, bstrprop: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteAppNonMember(&mut self, bstrprop: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddMember(&mut self, bstrprop: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteMember(&mut self, bstrprop: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddNonMember(&mut self, bstrprop: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteNonMember(&mut self, bstrprop: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Writable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&mut self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&mut self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPropertyItem(&mut self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePropertyItem(&mut self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Submit(&mut self, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddMemberName(&mut self, bstrprop: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteMemberName(&mut self, bstrprop: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddNonMemberName(&mut self, bstrprop: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteNonMemberName(&mut self, bstrprop: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MembersName(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn NonMembersName(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzApplicationGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzApplicationGroup_Vtbl {
        unsafe extern "system" fn Name<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn Type<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *plprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetType(::core::mem::transmute_copy(&lprop)).into()
        }
        unsafe extern "system" fn LdapQuery<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LdapQuery() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLdapQuery<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLdapQuery(::core::mem::transmute_copy(&bstrprop)).into()
        }
        unsafe extern "system" fn AppMembers<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppMembers() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppNonMembers<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppNonMembers() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Members<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Members() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NonMembers<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NonMembers() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn AddAppMember<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAppMember(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteAppMember<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteAppMember(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddAppNonMember<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAppNonMember(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteAppNonMember<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteAppNonMember(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddMember<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddMember(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteMember<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteMember(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddNonMember<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddNonMember(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteNonMember<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteNonMember(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Writable<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Writable() {
                ::core::result::Result::Ok(ok__) => {
                    *pfprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddPropertyItem<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePropertyItem<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeletePropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddMemberName<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddMemberName(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteMemberName<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteMemberName(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddNonMemberName<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddNonMemberName(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteNonMemberName<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteNonMemberName(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn MembersName<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MembersName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NonMembersName<Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NonMembersName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Type: Type::<Impl, IMPL_OFFSET>,
            SetType: SetType::<Impl, IMPL_OFFSET>,
            LdapQuery: LdapQuery::<Impl, IMPL_OFFSET>,
            SetLdapQuery: SetLdapQuery::<Impl, IMPL_OFFSET>,
            AppMembers: AppMembers::<Impl, IMPL_OFFSET>,
            AppNonMembers: AppNonMembers::<Impl, IMPL_OFFSET>,
            Members: Members::<Impl, IMPL_OFFSET>,
            NonMembers: NonMembers::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            AddAppMember: AddAppMember::<Impl, IMPL_OFFSET>,
            DeleteAppMember: DeleteAppMember::<Impl, IMPL_OFFSET>,
            AddAppNonMember: AddAppNonMember::<Impl, IMPL_OFFSET>,
            DeleteAppNonMember: DeleteAppNonMember::<Impl, IMPL_OFFSET>,
            AddMember: AddMember::<Impl, IMPL_OFFSET>,
            DeleteMember: DeleteMember::<Impl, IMPL_OFFSET>,
            AddNonMember: AddNonMember::<Impl, IMPL_OFFSET>,
            DeleteNonMember: DeleteNonMember::<Impl, IMPL_OFFSET>,
            Writable: Writable::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            AddPropertyItem: AddPropertyItem::<Impl, IMPL_OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Impl, IMPL_OFFSET>,
            Submit: Submit::<Impl, IMPL_OFFSET>,
            AddMemberName: AddMemberName::<Impl, IMPL_OFFSET>,
            DeleteMemberName: DeleteMemberName::<Impl, IMPL_OFFSET>,
            AddNonMemberName: AddNonMemberName::<Impl, IMPL_OFFSET>,
            DeleteNonMemberName: DeleteNonMemberName::<Impl, IMPL_OFFSET>,
            MembersName: MembersName::<Impl, IMPL_OFFSET>,
            NonMembersName: NonMembersName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzApplicationGroup as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzApplicationGroup2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IAzApplicationGroup_Impl {
    fn BizRule(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetBizRule(&mut self, bstrprop: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BizRuleLanguage(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetBizRuleLanguage(&mut self, bstrprop: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BizRuleImportedPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetBizRuleImportedPath(&mut self, bstrprop: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RoleAssignments(&mut self, bstrscopename: super::super::Foundation::BSTR, brecursive: i16) -> ::windows::core::Result<IAzRoleAssignments>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzApplicationGroup2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzApplicationGroup2_Vtbl {
        unsafe extern "system" fn BizRule<Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BizRule() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRule<Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBizRule(::core::mem::transmute_copy(&bstrprop)).into()
        }
        unsafe extern "system" fn BizRuleLanguage<Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BizRuleLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRuleLanguage<Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBizRuleLanguage(::core::mem::transmute_copy(&bstrprop)).into()
        }
        unsafe extern "system" fn BizRuleImportedPath<Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BizRuleImportedPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRuleImportedPath<Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBizRuleImportedPath(::core::mem::transmute_copy(&bstrprop)).into()
        }
        unsafe extern "system" fn RoleAssignments<Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, brecursive: i16, pproleassignments: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoleAssignments(::core::mem::transmute_copy(&bstrscopename), ::core::mem::transmute_copy(&brecursive)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproleassignments = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IAzApplicationGroup_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            BizRule: BizRule::<Impl, IMPL_OFFSET>,
            SetBizRule: SetBizRule::<Impl, IMPL_OFFSET>,
            BizRuleLanguage: BizRuleLanguage::<Impl, IMPL_OFFSET>,
            SetBizRuleLanguage: SetBizRuleLanguage::<Impl, IMPL_OFFSET>,
            BizRuleImportedPath: BizRuleImportedPath::<Impl, IMPL_OFFSET>,
            SetBizRuleImportedPath: SetBizRuleImportedPath::<Impl, IMPL_OFFSET>,
            RoleAssignments: RoleAssignments::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzApplicationGroup2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzApplicationGroups_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Item(&mut self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzApplicationGroups_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroups_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzApplicationGroups_Vtbl {
        unsafe extern "system" fn Item<Impl: IAzApplicationGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarobtptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IAzApplicationGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IAzApplicationGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzApplicationGroups as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzApplications_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Item(&mut self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzApplications_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplications_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzApplications_Vtbl {
        unsafe extern "system" fn Item<Impl: IAzApplications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarobtptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IAzApplications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IAzApplications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzApplications as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzAuthorizationStore_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ApplicationData(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetApplicationData(&mut self, bstrapplicationdata: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DomainTimeout(&mut self) -> ::windows::core::Result<i32>;
    fn SetDomainTimeout(&mut self, lprop: i32) -> ::windows::core::Result<()>;
    fn ScriptEngineTimeout(&mut self) -> ::windows::core::Result<i32>;
    fn SetScriptEngineTimeout(&mut self, lprop: i32) -> ::windows::core::Result<()>;
    fn MaxScriptEngines(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxScriptEngines(&mut self, lprop: i32) -> ::windows::core::Result<()>;
    fn GenerateAudits(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetGenerateAudits(&mut self, bprop: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Writable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&mut self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&mut self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPropertyItem(&mut self, lpropid: AZ_PROP_CONSTANTS, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePropertyItem(&mut self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PolicyAdministrators(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PolicyReaders(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddPolicyAdministrator(&mut self, bstradmin: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyAdministrator(&mut self, bstradmin: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPolicyReader(&mut self, bstrreader: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyReader(&mut self, bstrreader: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, lflags: AZ_PROP_CONSTANTS, bstrpolicyurl: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn UpdateCache(&mut self, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Delete(&mut self, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Applications(&mut self) -> ::windows::core::Result<IAzApplications>;
    fn OpenApplication(&mut self, bstrapplicationname: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplication>;
    fn CreateApplication(&mut self, bstrapplicationname: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplication>;
    fn DeleteApplication(&mut self, bstrapplicationname: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ApplicationGroups(&mut self) -> ::windows::core::Result<IAzApplicationGroups>;
    fn CreateApplicationGroup(&mut self, bstrgroupname: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>;
    fn OpenApplicationGroup(&mut self, bstrgroupname: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>;
    fn DeleteApplicationGroup(&mut self, bstrgroupname: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Submit(&mut self, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DelegatedPolicyUsers(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddDelegatedPolicyUser(&mut self, bstrdelegatedpolicyuser: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteDelegatedPolicyUser(&mut self, bstrdelegatedpolicyuser: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn TargetMachine(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ApplyStoreSacl(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetApplyStoreSacl(&mut self, bapplystoresacl: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn PolicyAdministratorsName(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PolicyReadersName(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddPolicyAdministratorName(&mut self, bstradmin: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyAdministratorName(&mut self, bstradmin: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPolicyReaderName(&mut self, bstrreader: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyReaderName(&mut self, bstrreader: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DelegatedPolicyUsersName(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddDelegatedPolicyUserName(&mut self, bstrdelegatedpolicyuser: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteDelegatedPolicyUserName(&mut self, bstrdelegatedpolicyuser: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn CloseApplication(&mut self, bstrapplicationname: super::super::Foundation::BSTR, lflag: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzAuthorizationStore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzAuthorizationStore_Vtbl {
        unsafe extern "system" fn Description<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn ApplicationData<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationData() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrapplicationdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetApplicationData(::core::mem::transmute_copy(&bstrapplicationdata)).into()
        }
        unsafe extern "system" fn DomainTimeout<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DomainTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *plprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDomainTimeout<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDomainTimeout(::core::mem::transmute_copy(&lprop)).into()
        }
        unsafe extern "system" fn ScriptEngineTimeout<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ScriptEngineTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *plprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScriptEngineTimeout<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetScriptEngineTimeout(::core::mem::transmute_copy(&lprop)).into()
        }
        unsafe extern "system" fn MaxScriptEngines<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MaxScriptEngines() {
                ::core::result::Result::Ok(ok__) => {
                    *plprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxScriptEngines<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetMaxScriptEngines(::core::mem::transmute_copy(&lprop)).into()
        }
        unsafe extern "system" fn GenerateAudits<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GenerateAudits() {
                ::core::result::Result::Ok(ok__) => {
                    *pbprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGenerateAudits<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bprop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetGenerateAudits(::core::mem::transmute_copy(&bprop)).into()
        }
        unsafe extern "system" fn Writable<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Writable() {
                ::core::result::Result::Ok(ok__) => {
                    *pfprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddPropertyItem<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: AZ_PROP_CONSTANTS, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePropertyItem<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeletePropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn PolicyAdministrators<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyAdministrators() {
                ::core::result::Result::Ok(ok__) => {
                    *pvaradmins = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReaders<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyReaders() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarreaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministrator<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPolicyAdministrator(::core::mem::transmute_copy(&bstradmin), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyAdministrator<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeletePolicyAdministrator(::core::mem::transmute_copy(&bstradmin), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddPolicyReader<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPolicyReader(::core::mem::transmute_copy(&bstrreader), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyReader<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeletePolicyReader(::core::mem::transmute_copy(&bstrreader), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Initialize<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: AZ_PROP_CONSTANTS, bstrpolicyurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&bstrpolicyurl), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn UpdateCache<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpdateCache(::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Delete<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Applications<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppappcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Applications() {
                ::core::result::Result::Ok(ok__) => {
                    *ppappcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenApplication<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppapplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenApplication(::core::mem::transmute_copy(&bstrapplicationname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppapplication = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateApplication<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppapplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateApplication(::core::mem::transmute_copy(&bstrapplicationname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppapplication = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteApplication<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteApplication(::core::mem::transmute_copy(&bstrapplicationname), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn ApplicationGroups<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroupcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroupcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateApplicationGroup<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateApplicationGroup(::core::mem::transmute_copy(&bstrgroupname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenApplicationGroup<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenApplicationGroup(::core::mem::transmute_copy(&bstrgroupname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteApplicationGroup<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteApplicationGroup(::core::mem::transmute_copy(&bstrgroupname), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DelegatedPolicyUsers<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DelegatedPolicyUsers() {
                ::core::result::Result::Ok(ok__) => {
                    *pvardelegatedpolicyusers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDelegatedPolicyUser<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDelegatedPolicyUser(::core::mem::transmute_copy(&bstrdelegatedpolicyuser), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUser<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteDelegatedPolicyUser(::core::mem::transmute_copy(&bstrdelegatedpolicyuser), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn TargetMachine<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtargetmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TargetMachine() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtargetmachine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyStoreSacl<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbapplystoresacl: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplyStoreSacl() {
                ::core::result::Result::Ok(ok__) => {
                    *pbapplystoresacl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplyStoreSacl<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bapplystoresacl: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetApplyStoreSacl(::core::mem::transmute_copy(&bapplystoresacl)).into()
        }
        unsafe extern "system" fn PolicyAdministratorsName<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyAdministratorsName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvaradmins = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReadersName<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyReadersName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarreaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministratorName<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPolicyAdministratorName(::core::mem::transmute_copy(&bstradmin), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyAdministratorName<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeletePolicyAdministratorName(::core::mem::transmute_copy(&bstradmin), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddPolicyReaderName<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPolicyReaderName(::core::mem::transmute_copy(&bstrreader), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyReaderName<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeletePolicyReaderName(::core::mem::transmute_copy(&bstrreader), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DelegatedPolicyUsersName<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).DelegatedPolicyUsersName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvardelegatedpolicyusers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDelegatedPolicyUserName<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddDelegatedPolicyUserName(::core::mem::transmute_copy(&bstrdelegatedpolicyuser), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUserName<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteDelegatedPolicyUserName(::core::mem::transmute_copy(&bstrdelegatedpolicyuser), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn CloseApplication<Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflag: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).CloseApplication(::core::mem::transmute_copy(&bstrapplicationname), ::core::mem::transmute_copy(&lflag)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            ApplicationData: ApplicationData::<Impl, IMPL_OFFSET>,
            SetApplicationData: SetApplicationData::<Impl, IMPL_OFFSET>,
            DomainTimeout: DomainTimeout::<Impl, IMPL_OFFSET>,
            SetDomainTimeout: SetDomainTimeout::<Impl, IMPL_OFFSET>,
            ScriptEngineTimeout: ScriptEngineTimeout::<Impl, IMPL_OFFSET>,
            SetScriptEngineTimeout: SetScriptEngineTimeout::<Impl, IMPL_OFFSET>,
            MaxScriptEngines: MaxScriptEngines::<Impl, IMPL_OFFSET>,
            SetMaxScriptEngines: SetMaxScriptEngines::<Impl, IMPL_OFFSET>,
            GenerateAudits: GenerateAudits::<Impl, IMPL_OFFSET>,
            SetGenerateAudits: SetGenerateAudits::<Impl, IMPL_OFFSET>,
            Writable: Writable::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            AddPropertyItem: AddPropertyItem::<Impl, IMPL_OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Impl, IMPL_OFFSET>,
            PolicyAdministrators: PolicyAdministrators::<Impl, IMPL_OFFSET>,
            PolicyReaders: PolicyReaders::<Impl, IMPL_OFFSET>,
            AddPolicyAdministrator: AddPolicyAdministrator::<Impl, IMPL_OFFSET>,
            DeletePolicyAdministrator: DeletePolicyAdministrator::<Impl, IMPL_OFFSET>,
            AddPolicyReader: AddPolicyReader::<Impl, IMPL_OFFSET>,
            DeletePolicyReader: DeletePolicyReader::<Impl, IMPL_OFFSET>,
            Initialize: Initialize::<Impl, IMPL_OFFSET>,
            UpdateCache: UpdateCache::<Impl, IMPL_OFFSET>,
            Delete: Delete::<Impl, IMPL_OFFSET>,
            Applications: Applications::<Impl, IMPL_OFFSET>,
            OpenApplication: OpenApplication::<Impl, IMPL_OFFSET>,
            CreateApplication: CreateApplication::<Impl, IMPL_OFFSET>,
            DeleteApplication: DeleteApplication::<Impl, IMPL_OFFSET>,
            ApplicationGroups: ApplicationGroups::<Impl, IMPL_OFFSET>,
            CreateApplicationGroup: CreateApplicationGroup::<Impl, IMPL_OFFSET>,
            OpenApplicationGroup: OpenApplicationGroup::<Impl, IMPL_OFFSET>,
            DeleteApplicationGroup: DeleteApplicationGroup::<Impl, IMPL_OFFSET>,
            Submit: Submit::<Impl, IMPL_OFFSET>,
            DelegatedPolicyUsers: DelegatedPolicyUsers::<Impl, IMPL_OFFSET>,
            AddDelegatedPolicyUser: AddDelegatedPolicyUser::<Impl, IMPL_OFFSET>,
            DeleteDelegatedPolicyUser: DeleteDelegatedPolicyUser::<Impl, IMPL_OFFSET>,
            TargetMachine: TargetMachine::<Impl, IMPL_OFFSET>,
            ApplyStoreSacl: ApplyStoreSacl::<Impl, IMPL_OFFSET>,
            SetApplyStoreSacl: SetApplyStoreSacl::<Impl, IMPL_OFFSET>,
            PolicyAdministratorsName: PolicyAdministratorsName::<Impl, IMPL_OFFSET>,
            PolicyReadersName: PolicyReadersName::<Impl, IMPL_OFFSET>,
            AddPolicyAdministratorName: AddPolicyAdministratorName::<Impl, IMPL_OFFSET>,
            DeletePolicyAdministratorName: DeletePolicyAdministratorName::<Impl, IMPL_OFFSET>,
            AddPolicyReaderName: AddPolicyReaderName::<Impl, IMPL_OFFSET>,
            DeletePolicyReaderName: DeletePolicyReaderName::<Impl, IMPL_OFFSET>,
            DelegatedPolicyUsersName: DelegatedPolicyUsersName::<Impl, IMPL_OFFSET>,
            AddDelegatedPolicyUserName: AddDelegatedPolicyUserName::<Impl, IMPL_OFFSET>,
            DeleteDelegatedPolicyUserName: DeleteDelegatedPolicyUserName::<Impl, IMPL_OFFSET>,
            CloseApplication: CloseApplication::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzAuthorizationStore as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzAuthorizationStore2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IAzAuthorizationStore_Impl {
    fn OpenApplication2(&mut self, bstrapplicationname: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplication2>;
    fn CreateApplication2(&mut self, bstrapplicationname: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplication2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzAuthorizationStore2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzAuthorizationStore2_Vtbl {
        unsafe extern "system" fn OpenApplication2<Impl: IAzAuthorizationStore2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppapplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenApplication2(::core::mem::transmute_copy(&bstrapplicationname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppapplication = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateApplication2<Impl: IAzAuthorizationStore2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppapplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateApplication2(::core::mem::transmute_copy(&bstrapplicationname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppapplication = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IAzAuthorizationStore_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            OpenApplication2: OpenApplication2::<Impl, IMPL_OFFSET>,
            CreateApplication2: CreateApplication2::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzAuthorizationStore2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzAuthorizationStore3_Impl: Sized + super::super::System::Com::IDispatch_Impl + IAzAuthorizationStore_Impl + IAzAuthorizationStore2_Impl {
    fn IsUpdateNeeded(&mut self) -> ::windows::core::Result<i16>;
    fn BizruleGroupSupported(&mut self) -> ::windows::core::Result<i16>;
    fn UpgradeStoresFunctionalLevel(&mut self, lfunctionallevel: i32) -> ::windows::core::Result<()>;
    fn IsFunctionalLevelUpgradeSupported(&mut self, lfunctionallevel: i32) -> ::windows::core::Result<i16>;
    fn GetSchemaVersion(&mut self, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzAuthorizationStore3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzAuthorizationStore3_Vtbl {
        unsafe extern "system" fn IsUpdateNeeded<Impl: IAzAuthorizationStore3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisupdateneeded: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsUpdateNeeded() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisupdateneeded = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BizruleGroupSupported<Impl: IAzAuthorizationStore3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsupported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BizruleGroupSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *pbsupported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpgradeStoresFunctionalLevel<Impl: IAzAuthorizationStore3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfunctionallevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).UpgradeStoresFunctionalLevel(::core::mem::transmute_copy(&lfunctionallevel)).into()
        }
        unsafe extern "system" fn IsFunctionalLevelUpgradeSupported<Impl: IAzAuthorizationStore3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfunctionallevel: i32, pbsupported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsFunctionalLevelUpgradeSupported(::core::mem::transmute_copy(&lfunctionallevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbsupported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSchemaVersion<Impl: IAzAuthorizationStore3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetSchemaVersion(::core::mem::transmute_copy(&plmajorversion), ::core::mem::transmute_copy(&plminorversion)).into()
        }
        Self {
            base: IAzAuthorizationStore2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            IsUpdateNeeded: IsUpdateNeeded::<Impl, IMPL_OFFSET>,
            BizruleGroupSupported: BizruleGroupSupported::<Impl, IMPL_OFFSET>,
            UpgradeStoresFunctionalLevel: UpgradeStoresFunctionalLevel::<Impl, IMPL_OFFSET>,
            IsFunctionalLevelUpgradeSupported: IsFunctionalLevelUpgradeSupported::<Impl, IMPL_OFFSET>,
            GetSchemaVersion: GetSchemaVersion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzAuthorizationStore3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzBizRuleContext_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetBusinessRuleResult(&mut self, bresult: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetBusinessRuleString(&mut self, bstrbusinessrulestring: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BusinessRuleString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetParameter(&mut self, bstrparametername: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzBizRuleContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzBizRuleContext_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzBizRuleContext_Vtbl {
        unsafe extern "system" fn SetBusinessRuleResult<Impl: IAzBizRuleContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bresult: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBusinessRuleResult(::core::mem::transmute_copy(&bresult)).into()
        }
        unsafe extern "system" fn SetBusinessRuleString<Impl: IAzBizRuleContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbusinessrulestring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBusinessRuleString(::core::mem::transmute_copy(&bstrbusinessrulestring)).into()
        }
        unsafe extern "system" fn BusinessRuleString<Impl: IAzBizRuleContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbusinessrulestring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BusinessRuleString() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrbusinessrulestring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParameter<Impl: IAzBizRuleContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrparametername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarparametervalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParameter(::core::mem::transmute_copy(&bstrparametername)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarparametervalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            SetBusinessRuleResult: SetBusinessRuleResult::<Impl, IMPL_OFFSET>,
            SetBusinessRuleString: SetBusinessRuleString::<Impl, IMPL_OFFSET>,
            BusinessRuleString: BusinessRuleString::<Impl, IMPL_OFFSET>,
            GetParameter: GetParameter::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzBizRuleContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzBizRuleInterfaces_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AddInterface(&mut self, bstrinterfacename: super::super::Foundation::BSTR, linterfaceflag: i32, varinterface: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddInterfaces(&mut self, varinterfacenames: super::super::System::Com::VARIANT, varinterfaceflags: super::super::System::Com::VARIANT, varinterfaces: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetInterfaceValue(&mut self, bstrinterfacename: super::super::Foundation::BSTR, linterfaceflag: *mut i32, varinterface: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Remove(&mut self, bstrinterfacename: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoveAll(&mut self) -> ::windows::core::Result<()>;
    fn Count(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzBizRuleInterfaces_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzBizRuleInterfaces_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzBizRuleInterfaces_Vtbl {
        unsafe extern "system" fn AddInterface<Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinterfacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linterfaceflag: i32, varinterface: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInterface(::core::mem::transmute_copy(&bstrinterfacename), ::core::mem::transmute_copy(&linterfaceflag), ::core::mem::transmute_copy(&varinterface)).into()
        }
        unsafe extern "system" fn AddInterfaces<Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varinterfacenames: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varinterfaceflags: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varinterfaces: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddInterfaces(::core::mem::transmute_copy(&varinterfacenames), ::core::mem::transmute_copy(&varinterfaceflags), ::core::mem::transmute_copy(&varinterfaces)).into()
        }
        unsafe extern "system" fn GetInterfaceValue<Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinterfacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linterfaceflag: *mut i32, varinterface: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetInterfaceValue(::core::mem::transmute_copy(&bstrinterfacename), ::core::mem::transmute_copy(&linterfaceflag), ::core::mem::transmute_copy(&varinterface)).into()
        }
        unsafe extern "system" fn Remove<Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinterfacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&bstrinterfacename)).into()
        }
        unsafe extern "system" fn RemoveAll<Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAll().into()
        }
        unsafe extern "system" fn Count<Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddInterface: AddInterface::<Impl, IMPL_OFFSET>,
            AddInterfaces: AddInterfaces::<Impl, IMPL_OFFSET>,
            GetInterfaceValue: GetInterfaceValue::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            RemoveAll: RemoveAll::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzBizRuleInterfaces as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzBizRuleParameters_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AddParameter(&mut self, bstrparametername: super::super::Foundation::BSTR, varparametervalue: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddParameters(&mut self, varparameternames: super::super::System::Com::VARIANT, varparametervalues: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetParameterValue(&mut self, bstrparametername: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Remove(&mut self, varparametername: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoveAll(&mut self) -> ::windows::core::Result<()>;
    fn Count(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzBizRuleParameters_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzBizRuleParameters_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzBizRuleParameters_Vtbl {
        unsafe extern "system" fn AddParameter<Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrparametername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varparametervalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddParameter(::core::mem::transmute_copy(&bstrparametername), ::core::mem::transmute_copy(&varparametervalue)).into()
        }
        unsafe extern "system" fn AddParameters<Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varparameternames: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varparametervalues: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddParameters(::core::mem::transmute_copy(&varparameternames), ::core::mem::transmute_copy(&varparametervalues)).into()
        }
        unsafe extern "system" fn GetParameterValue<Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrparametername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarparametervalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetParameterValue(::core::mem::transmute_copy(&bstrparametername)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarparametervalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varparametername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&varparametername)).into()
        }
        unsafe extern "system" fn RemoveAll<Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).RemoveAll().into()
        }
        unsafe extern "system" fn Count<Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddParameter: AddParameter::<Impl, IMPL_OFFSET>,
            AddParameters: AddParameters::<Impl, IMPL_OFFSET>,
            GetParameterValue: GetParameterValue::<Impl, IMPL_OFFSET>,
            Remove: Remove::<Impl, IMPL_OFFSET>,
            RemoveAll: RemoveAll::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzBizRuleParameters as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzClientContext_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AccessCheck(&mut self, bstrobjectname: super::super::Foundation::BSTR, varscopenames: super::super::System::Com::VARIANT, varoperations: super::super::System::Com::VARIANT, varparameternames: super::super::System::Com::VARIANT, varparametervalues: super::super::System::Com::VARIANT, varinterfacenames: super::super::System::Com::VARIANT, varinterfaceflags: super::super::System::Com::VARIANT, varinterfaces: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetBusinessRuleString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserDn(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserSamCompat(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserDisplay(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserCanonical(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserUpn(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserDnsSamCompat(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetProperty(&mut self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetRoles(&mut self, bstrscopename: super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn RoleForAccessCheck(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRoleForAccessCheck(&mut self, bstrprop: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzClientContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzClientContext_Vtbl {
        unsafe extern "system" fn AccessCheck<Impl: IAzClientContext_Impl, const OFFSET: isize>(
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
            match (*this).AccessCheck(::core::mem::transmute_copy(&bstrobjectname), ::core::mem::transmute_copy(&varscopenames), ::core::mem::transmute_copy(&varoperations), ::core::mem::transmute_copy(&varparameternames), ::core::mem::transmute_copy(&varparametervalues), ::core::mem::transmute_copy(&varinterfacenames), ::core::mem::transmute_copy(&varinterfaceflags), ::core::mem::transmute_copy(&varinterfaces)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarresults = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBusinessRuleString<Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbusinessrulestring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetBusinessRuleString() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrbusinessrulestring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserDn<Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserDn() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserSamCompat<Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserSamCompat() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserDisplay<Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserDisplay() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserGuid<Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserCanonical<Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserCanonical() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserUpn<Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserUpn() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserDnsSamCompat<Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).UserDnsSamCompat() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRoles<Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarrolenames: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetRoles(::core::mem::transmute_copy(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarrolenames = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoleForAccessCheck<Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoleForAccessCheck() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoleForAccessCheck<Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetRoleForAccessCheck(::core::mem::transmute_copy(&bstrprop)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AccessCheck: AccessCheck::<Impl, IMPL_OFFSET>,
            GetBusinessRuleString: GetBusinessRuleString::<Impl, IMPL_OFFSET>,
            UserDn: UserDn::<Impl, IMPL_OFFSET>,
            UserSamCompat: UserSamCompat::<Impl, IMPL_OFFSET>,
            UserDisplay: UserDisplay::<Impl, IMPL_OFFSET>,
            UserGuid: UserGuid::<Impl, IMPL_OFFSET>,
            UserCanonical: UserCanonical::<Impl, IMPL_OFFSET>,
            UserUpn: UserUpn::<Impl, IMPL_OFFSET>,
            UserDnsSamCompat: UserDnsSamCompat::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            GetRoles: GetRoles::<Impl, IMPL_OFFSET>,
            RoleForAccessCheck: RoleForAccessCheck::<Impl, IMPL_OFFSET>,
            SetRoleForAccessCheck: SetRoleForAccessCheck::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzClientContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzClientContext2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IAzClientContext_Impl {
    fn GetAssignedScopesPage(&mut self, loptions: i32, pagesize: i32, pvarcursor: *mut super::super::System::Com::VARIANT, pvarscopenames: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddRoles(&mut self, varroles: super::super::System::Com::VARIANT, bstrscopename: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddApplicationGroups(&mut self, varapplicationgroups: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddStringSids(&mut self, varstringsids: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetLDAPQueryDN(&mut self, bstrldapquerydn: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LDAPQueryDN(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzClientContext2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzClientContext2_Vtbl {
        unsafe extern "system" fn GetAssignedScopesPage<Impl: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loptions: i32, pagesize: i32, pvarcursor: *mut super::super::System::Com::VARIANT, pvarscopenames: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetAssignedScopesPage(::core::mem::transmute_copy(&loptions), ::core::mem::transmute_copy(&pagesize), ::core::mem::transmute_copy(&pvarcursor), ::core::mem::transmute_copy(&pvarscopenames)).into()
        }
        unsafe extern "system" fn AddRoles<Impl: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varroles: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRoles(::core::mem::transmute_copy(&varroles), ::core::mem::transmute_copy(&bstrscopename)).into()
        }
        unsafe extern "system" fn AddApplicationGroups<Impl: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varapplicationgroups: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddApplicationGroups(::core::mem::transmute_copy(&varapplicationgroups)).into()
        }
        unsafe extern "system" fn AddStringSids<Impl: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varstringsids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddStringSids(::core::mem::transmute_copy(&varstringsids)).into()
        }
        unsafe extern "system" fn SetLDAPQueryDN<Impl: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrldapquerydn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetLDAPQueryDN(::core::mem::transmute_copy(&bstrldapquerydn)).into()
        }
        unsafe extern "system" fn LDAPQueryDN<Impl: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrldapquerydn: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).LDAPQueryDN() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrldapquerydn = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IAzClientContext_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetAssignedScopesPage: GetAssignedScopesPage::<Impl, IMPL_OFFSET>,
            AddRoles: AddRoles::<Impl, IMPL_OFFSET>,
            AddApplicationGroups: AddApplicationGroups::<Impl, IMPL_OFFSET>,
            AddStringSids: AddStringSids::<Impl, IMPL_OFFSET>,
            SetLDAPQueryDN: SetLDAPQueryDN::<Impl, IMPL_OFFSET>,
            LDAPQueryDN: LDAPQueryDN::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzClientContext2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzClientContext3_Impl: Sized + super::super::System::Com::IDispatch_Impl + IAzClientContext_Impl + IAzClientContext2_Impl {
    fn AccessCheck2(&mut self, bstrobjectname: super::super::Foundation::BSTR, bstrscopename: super::super::Foundation::BSTR, loperation: i32) -> ::windows::core::Result<u32>;
    fn IsInRoleAssignment(&mut self, bstrscopename: super::super::Foundation::BSTR, bstrrolename: super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn GetOperations(&mut self, bstrscopename: super::super::Foundation::BSTR) -> ::windows::core::Result<IAzOperations>;
    fn GetTasks(&mut self, bstrscopename: super::super::Foundation::BSTR) -> ::windows::core::Result<IAzTasks>;
    fn BizRuleParameters(&mut self) -> ::windows::core::Result<IAzBizRuleParameters>;
    fn BizRuleInterfaces(&mut self) -> ::windows::core::Result<IAzBizRuleInterfaces>;
    fn GetGroups(&mut self, bstrscopename: super::super::Foundation::BSTR, uloptions: AZ_PROP_CONSTANTS) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Sids(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzClientContext3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext3_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzClientContext3_Vtbl {
        unsafe extern "system" fn AccessCheck2<Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrobjectname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loperation: i32, plresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AccessCheck2(::core::mem::transmute_copy(&bstrobjectname), ::core::mem::transmute_copy(&bstrscopename), ::core::mem::transmute_copy(&loperation)) {
                ::core::result::Result::Ok(ok__) => {
                    *plresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInRoleAssignment<Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbisinrole: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsInRoleAssignment(::core::mem::transmute_copy(&bstrscopename), ::core::mem::transmute_copy(&bstrrolename)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbisinrole = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOperations<Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppoperationcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetOperations(::core::mem::transmute_copy(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoperationcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTasks<Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptaskcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetTasks(::core::mem::transmute_copy(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptaskcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BizRuleParameters<Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbizruleparam: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BizRuleParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *ppbizruleparam = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BizRuleInterfaces<Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbizruleinterfaces: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BizRuleInterfaces() {
                ::core::result::Result::Ok(ok__) => {
                    *ppbizruleinterfaces = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGroups<Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, uloptions: AZ_PROP_CONSTANTS, pgrouparray: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGroups(::core::mem::transmute_copy(&bstrscopename), ::core::mem::transmute_copy(&uloptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *pgrouparray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sids<Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstringsidarray: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Sids() {
                ::core::result::Result::Ok(ok__) => {
                    *pstringsidarray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IAzClientContext2_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AccessCheck2: AccessCheck2::<Impl, IMPL_OFFSET>,
            IsInRoleAssignment: IsInRoleAssignment::<Impl, IMPL_OFFSET>,
            GetOperations: GetOperations::<Impl, IMPL_OFFSET>,
            GetTasks: GetTasks::<Impl, IMPL_OFFSET>,
            BizRuleParameters: BizRuleParameters::<Impl, IMPL_OFFSET>,
            BizRuleInterfaces: BizRuleInterfaces::<Impl, IMPL_OFFSET>,
            GetGroups: GetGroups::<Impl, IMPL_OFFSET>,
            Sids: Sids::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzClientContext3 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzNameResolver_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn NameFromSid(&mut self, bstrsid: super::super::Foundation::BSTR, psidtype: *mut i32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NamesFromSids(&mut self, vsids: super::super::System::Com::VARIANT, pvsidtypes: *mut super::super::System::Com::VARIANT, pvnames: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzNameResolver_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzNameResolver_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzNameResolver_Vtbl {
        unsafe extern "system" fn NameFromSid<Impl: IAzNameResolver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psidtype: *mut i32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NameFromSid(::core::mem::transmute_copy(&bstrsid), ::core::mem::transmute_copy(&psidtype), ::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn NamesFromSids<Impl: IAzNameResolver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vsids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvsidtypes: *mut super::super::System::Com::VARIANT, pvnames: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).NamesFromSids(::core::mem::transmute_copy(&vsids), ::core::mem::transmute_copy(&pvsidtypes), ::core::mem::transmute_copy(&pvnames)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            NameFromSid: NameFromSid::<Impl, IMPL_OFFSET>,
            NamesFromSids: NamesFromSids::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzNameResolver as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzObjectPicker_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetPrincipals(&mut self, hparentwnd: super::super::Foundation::HWND, bstrtitle: super::super::Foundation::BSTR, pvsidtypes: *mut super::super::System::Com::VARIANT, pvnames: *mut super::super::System::Com::VARIANT, pvsids: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzObjectPicker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzObjectPicker_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzObjectPicker_Vtbl {
        unsafe extern "system" fn GetPrincipals<Impl: IAzObjectPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hparentwnd: super::super::Foundation::HWND, bstrtitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvsidtypes: *mut super::super::System::Com::VARIANT, pvnames: *mut super::super::System::Com::VARIANT, pvsids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).GetPrincipals(::core::mem::transmute_copy(&hparentwnd), ::core::mem::transmute_copy(&bstrtitle), ::core::mem::transmute_copy(&pvsidtypes), ::core::mem::transmute_copy(&pvnames), ::core::mem::transmute_copy(&pvsids)).into()
        }
        unsafe extern "system" fn Name<Impl: IAzObjectPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            GetPrincipals: GetPrincipals::<Impl, IMPL_OFFSET>,
            Name: Name::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzObjectPicker as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzOperation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ApplicationData(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetApplicationData(&mut self, bstrapplicationdata: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OperationID(&mut self) -> ::windows::core::Result<i32>;
    fn SetOperationID(&mut self, lprop: i32) -> ::windows::core::Result<()>;
    fn Writable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&mut self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&mut self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Submit(&mut self, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzOperation_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzOperation_Vtbl {
        unsafe extern "system" fn Name<Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn Description<Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn ApplicationData<Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationData() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrapplicationdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetApplicationData(::core::mem::transmute_copy(&bstrapplicationdata)).into()
        }
        unsafe extern "system" fn OperationID<Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OperationID() {
                ::core::result::Result::Ok(ok__) => {
                    *plprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOperationID<Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetOperationID(::core::mem::transmute_copy(&lprop)).into()
        }
        unsafe extern "system" fn Writable<Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Writable() {
                ::core::result::Result::Ok(ok__) => {
                    *pfprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&varreserved)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            ApplicationData: ApplicationData::<Impl, IMPL_OFFSET>,
            SetApplicationData: SetApplicationData::<Impl, IMPL_OFFSET>,
            OperationID: OperationID::<Impl, IMPL_OFFSET>,
            SetOperationID: SetOperationID::<Impl, IMPL_OFFSET>,
            Writable: Writable::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            Submit: Submit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzOperation2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IAzOperation_Impl {
    fn RoleAssignments(&mut self, bstrscopename: super::super::Foundation::BSTR, brecursive: i16) -> ::windows::core::Result<IAzRoleAssignments>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzOperation2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzOperation2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzOperation2_Vtbl {
        unsafe extern "system" fn RoleAssignments<Impl: IAzOperation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, brecursive: i16, pproleassignments: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoleAssignments(::core::mem::transmute_copy(&bstrscopename), ::core::mem::transmute_copy(&brecursive)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproleassignments = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IAzOperation_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), RoleAssignments: RoleAssignments::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzOperation2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzOperations_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Item(&mut self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzOperations_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzOperations_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzOperations_Vtbl {
        unsafe extern "system" fn Item<Impl: IAzOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarobtptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IAzOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IAzOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzOperations as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzPrincipalLocator_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn NameResolver(&mut self) -> ::windows::core::Result<IAzNameResolver>;
    fn ObjectPicker(&mut self) -> ::windows::core::Result<IAzObjectPicker>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzPrincipalLocator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzPrincipalLocator_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzPrincipalLocator_Vtbl {
        unsafe extern "system" fn NameResolver<Impl: IAzPrincipalLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnameresolver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).NameResolver() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnameresolver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ObjectPicker<Impl: IAzPrincipalLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobjectpicker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ObjectPicker() {
                ::core::result::Result::Ok(ok__) => {
                    *ppobjectpicker = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            NameResolver: NameResolver::<Impl, IMPL_OFFSET>,
            ObjectPicker: ObjectPicker::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzPrincipalLocator as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzRole_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ApplicationData(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetApplicationData(&mut self, bstrapplicationdata: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddAppMember(&mut self, bstrprop: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteAppMember(&mut self, bstrprop: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddTask(&mut self, bstrprop: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteTask(&mut self, bstrprop: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddOperation(&mut self, bstrprop: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteOperation(&mut self, bstrprop: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddMember(&mut self, bstrprop: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteMember(&mut self, bstrprop: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Writable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&mut self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&mut self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AppMembers(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Members(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Operations(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Tasks(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddPropertyItem(&mut self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePropertyItem(&mut self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Submit(&mut self, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddMemberName(&mut self, bstrprop: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteMemberName(&mut self, bstrprop: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MembersName(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzRole_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzRole_Vtbl {
        unsafe extern "system" fn Name<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn Description<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn ApplicationData<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationData() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrapplicationdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetApplicationData(::core::mem::transmute_copy(&bstrapplicationdata)).into()
        }
        unsafe extern "system" fn AddAppMember<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddAppMember(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteAppMember<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteAppMember(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddTask<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTask(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteTask<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteTask(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddOperation<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddOperation(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteOperation<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteOperation(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddMember<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddMember(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteMember<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteMember(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Writable<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Writable() {
                ::core::result::Result::Ok(ok__) => {
                    *pfprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AppMembers<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).AppMembers() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Members<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Members() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Operations<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Operations() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tasks<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tasks() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPropertyItem<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePropertyItem<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeletePropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddMemberName<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddMemberName(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteMemberName<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteMemberName(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn MembersName<Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).MembersName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            ApplicationData: ApplicationData::<Impl, IMPL_OFFSET>,
            SetApplicationData: SetApplicationData::<Impl, IMPL_OFFSET>,
            AddAppMember: AddAppMember::<Impl, IMPL_OFFSET>,
            DeleteAppMember: DeleteAppMember::<Impl, IMPL_OFFSET>,
            AddTask: AddTask::<Impl, IMPL_OFFSET>,
            DeleteTask: DeleteTask::<Impl, IMPL_OFFSET>,
            AddOperation: AddOperation::<Impl, IMPL_OFFSET>,
            DeleteOperation: DeleteOperation::<Impl, IMPL_OFFSET>,
            AddMember: AddMember::<Impl, IMPL_OFFSET>,
            DeleteMember: DeleteMember::<Impl, IMPL_OFFSET>,
            Writable: Writable::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            AppMembers: AppMembers::<Impl, IMPL_OFFSET>,
            Members: Members::<Impl, IMPL_OFFSET>,
            Operations: Operations::<Impl, IMPL_OFFSET>,
            Tasks: Tasks::<Impl, IMPL_OFFSET>,
            AddPropertyItem: AddPropertyItem::<Impl, IMPL_OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Impl, IMPL_OFFSET>,
            Submit: Submit::<Impl, IMPL_OFFSET>,
            AddMemberName: AddMemberName::<Impl, IMPL_OFFSET>,
            DeleteMemberName: DeleteMemberName::<Impl, IMPL_OFFSET>,
            MembersName: MembersName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzRole as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzRoleAssignment_Impl: Sized + super::super::System::Com::IDispatch_Impl + IAzRole_Impl {
    fn AddRoleDefinition(&mut self, bstrroledefinition: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DeleteRoleDefinition(&mut self, bstrroledefinition: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RoleDefinitions(&mut self) -> ::windows::core::Result<IAzRoleDefinitions>;
    fn Scope(&mut self) -> ::windows::core::Result<IAzScope>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzRoleAssignment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleAssignment_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzRoleAssignment_Vtbl {
        unsafe extern "system" fn AddRoleDefinition<Impl: IAzRoleAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinition: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRoleDefinition(::core::mem::transmute_copy(&bstrroledefinition)).into()
        }
        unsafe extern "system" fn DeleteRoleDefinition<Impl: IAzRoleAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinition: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteRoleDefinition(::core::mem::transmute_copy(&bstrroledefinition)).into()
        }
        unsafe extern "system" fn RoleDefinitions<Impl: IAzRoleAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoleDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    *pproledefinitions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scope<Impl: IAzRoleAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppscope: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Scope() {
                ::core::result::Result::Ok(ok__) => {
                    *ppscope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IAzRole_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            AddRoleDefinition: AddRoleDefinition::<Impl, IMPL_OFFSET>,
            DeleteRoleDefinition: DeleteRoleDefinition::<Impl, IMPL_OFFSET>,
            RoleDefinitions: RoleDefinitions::<Impl, IMPL_OFFSET>,
            Scope: Scope::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzRoleAssignment as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzRoleAssignments_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Item(&mut self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzRoleAssignments_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleAssignments_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzRoleAssignments_Vtbl {
        unsafe extern "system" fn Item<Impl: IAzRoleAssignments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarobtptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IAzRoleAssignments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IAzRoleAssignments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzRoleAssignments as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzRoleDefinition_Impl: Sized + super::super::System::Com::IDispatch_Impl + IAzTask_Impl {
    fn RoleAssignments(&mut self, bstrscopename: super::super::Foundation::BSTR, brecursive: i16) -> ::windows::core::Result<IAzRoleAssignments>;
    fn AddRoleDefinition(&mut self, bstrroledefinition: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DeleteRoleDefinition(&mut self, bstrroledefinition: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RoleDefinitions(&mut self) -> ::windows::core::Result<IAzRoleDefinitions>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzRoleDefinition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleDefinition_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzRoleDefinition_Vtbl {
        unsafe extern "system" fn RoleAssignments<Impl: IAzRoleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, brecursive: i16, pproleassignments: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoleAssignments(::core::mem::transmute_copy(&bstrscopename), ::core::mem::transmute_copy(&brecursive)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproleassignments = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRoleDefinition<Impl: IAzRoleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinition: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddRoleDefinition(::core::mem::transmute_copy(&bstrroledefinition)).into()
        }
        unsafe extern "system" fn DeleteRoleDefinition<Impl: IAzRoleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinition: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteRoleDefinition(::core::mem::transmute_copy(&bstrroledefinition)).into()
        }
        unsafe extern "system" fn RoleDefinitions<Impl: IAzRoleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoleDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    *pproledefinitions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IAzTask_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RoleAssignments: RoleAssignments::<Impl, IMPL_OFFSET>,
            AddRoleDefinition: AddRoleDefinition::<Impl, IMPL_OFFSET>,
            DeleteRoleDefinition: DeleteRoleDefinition::<Impl, IMPL_OFFSET>,
            RoleDefinitions: RoleDefinitions::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzRoleDefinition as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzRoleDefinitions_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Item(&mut self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzRoleDefinitions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleDefinitions_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzRoleDefinitions_Vtbl {
        unsafe extern "system" fn Item<Impl: IAzRoleDefinitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarobtptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IAzRoleDefinitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IAzRoleDefinitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzRoleDefinitions as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzRoles_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Item(&mut self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzRoles_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoles_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzRoles_Vtbl {
        unsafe extern "system" fn Item<Impl: IAzRoles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarobtptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IAzRoles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IAzRoles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzRoles as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzScope_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ApplicationData(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetApplicationData(&mut self, bstrapplicationdata: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Writable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&mut self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&mut self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPropertyItem(&mut self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePropertyItem(&mut self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PolicyAdministrators(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PolicyReaders(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddPolicyAdministrator(&mut self, bstradmin: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyAdministrator(&mut self, bstradmin: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPolicyReader(&mut self, bstrreader: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyReader(&mut self, bstrreader: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ApplicationGroups(&mut self) -> ::windows::core::Result<IAzApplicationGroups>;
    fn OpenApplicationGroup(&mut self, bstrgroupname: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>;
    fn CreateApplicationGroup(&mut self, bstrgroupname: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>;
    fn DeleteApplicationGroup(&mut self, bstrgroupname: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Roles(&mut self) -> ::windows::core::Result<IAzRoles>;
    fn OpenRole(&mut self, bstrrolename: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzRole>;
    fn CreateRole(&mut self, bstrrolename: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzRole>;
    fn DeleteRole(&mut self, bstrrolename: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Tasks(&mut self) -> ::windows::core::Result<IAzTasks>;
    fn OpenTask(&mut self, bstrtaskname: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzTask>;
    fn CreateTask(&mut self, bstrtaskname: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzTask>;
    fn DeleteTask(&mut self, bstrtaskname: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Submit(&mut self, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn CanBeDelegated(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn BizrulesWritable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn PolicyAdministratorsName(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PolicyReadersName(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddPolicyAdministratorName(&mut self, bstradmin: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyAdministratorName(&mut self, bstradmin: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPolicyReaderName(&mut self, bstrreader: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyReaderName(&mut self, bstrreader: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzScope_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzScope_Vtbl {
        unsafe extern "system" fn Name<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn Description<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn ApplicationData<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationData() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrapplicationdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetApplicationData(::core::mem::transmute_copy(&bstrapplicationdata)).into()
        }
        unsafe extern "system" fn Writable<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Writable() {
                ::core::result::Result::Ok(ok__) => {
                    *pfprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddPropertyItem<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePropertyItem<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeletePropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn PolicyAdministrators<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyAdministrators() {
                ::core::result::Result::Ok(ok__) => {
                    *pvaradmins = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReaders<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyReaders() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarreaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministrator<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPolicyAdministrator(::core::mem::transmute_copy(&bstradmin), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyAdministrator<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeletePolicyAdministrator(::core::mem::transmute_copy(&bstradmin), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddPolicyReader<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPolicyReader(::core::mem::transmute_copy(&bstrreader), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyReader<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeletePolicyReader(::core::mem::transmute_copy(&bstrreader), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn ApplicationGroups<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroupcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroupcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenApplicationGroup<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenApplicationGroup(::core::mem::transmute_copy(&bstrgroupname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateApplicationGroup<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateApplicationGroup(::core::mem::transmute_copy(&bstrgroupname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteApplicationGroup<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteApplicationGroup(::core::mem::transmute_copy(&bstrgroupname), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Roles<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprolecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Roles() {
                ::core::result::Result::Ok(ok__) => {
                    *pprolecollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRole<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pprole: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenRole(::core::mem::transmute_copy(&bstrrolename), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprole = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRole<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pprole: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRole(::core::mem::transmute_copy(&bstrrolename), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprole = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRole<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteRole(::core::mem::transmute_copy(&bstrrolename), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Tasks<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptaskcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tasks() {
                ::core::result::Result::Ok(ok__) => {
                    *pptaskcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTask<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pptask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenTask(::core::mem::transmute_copy(&bstrtaskname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTask<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pptask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateTask(::core::mem::transmute_copy(&bstrtaskname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteTask<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteTask(::core::mem::transmute_copy(&bstrtaskname), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn CanBeDelegated<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CanBeDelegated() {
                ::core::result::Result::Ok(ok__) => {
                    *pfprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BizrulesWritable<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BizrulesWritable() {
                ::core::result::Result::Ok(ok__) => {
                    *pfprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyAdministratorsName<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyAdministratorsName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvaradmins = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReadersName<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PolicyReadersName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarreaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministratorName<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPolicyAdministratorName(::core::mem::transmute_copy(&bstradmin), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyAdministratorName<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeletePolicyAdministratorName(::core::mem::transmute_copy(&bstradmin), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddPolicyReaderName<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPolicyReaderName(::core::mem::transmute_copy(&bstrreader), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyReaderName<Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeletePolicyReaderName(::core::mem::transmute_copy(&bstrreader), ::core::mem::transmute_copy(&varreserved)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            ApplicationData: ApplicationData::<Impl, IMPL_OFFSET>,
            SetApplicationData: SetApplicationData::<Impl, IMPL_OFFSET>,
            Writable: Writable::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            AddPropertyItem: AddPropertyItem::<Impl, IMPL_OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Impl, IMPL_OFFSET>,
            PolicyAdministrators: PolicyAdministrators::<Impl, IMPL_OFFSET>,
            PolicyReaders: PolicyReaders::<Impl, IMPL_OFFSET>,
            AddPolicyAdministrator: AddPolicyAdministrator::<Impl, IMPL_OFFSET>,
            DeletePolicyAdministrator: DeletePolicyAdministrator::<Impl, IMPL_OFFSET>,
            AddPolicyReader: AddPolicyReader::<Impl, IMPL_OFFSET>,
            DeletePolicyReader: DeletePolicyReader::<Impl, IMPL_OFFSET>,
            ApplicationGroups: ApplicationGroups::<Impl, IMPL_OFFSET>,
            OpenApplicationGroup: OpenApplicationGroup::<Impl, IMPL_OFFSET>,
            CreateApplicationGroup: CreateApplicationGroup::<Impl, IMPL_OFFSET>,
            DeleteApplicationGroup: DeleteApplicationGroup::<Impl, IMPL_OFFSET>,
            Roles: Roles::<Impl, IMPL_OFFSET>,
            OpenRole: OpenRole::<Impl, IMPL_OFFSET>,
            CreateRole: CreateRole::<Impl, IMPL_OFFSET>,
            DeleteRole: DeleteRole::<Impl, IMPL_OFFSET>,
            Tasks: Tasks::<Impl, IMPL_OFFSET>,
            OpenTask: OpenTask::<Impl, IMPL_OFFSET>,
            CreateTask: CreateTask::<Impl, IMPL_OFFSET>,
            DeleteTask: DeleteTask::<Impl, IMPL_OFFSET>,
            Submit: Submit::<Impl, IMPL_OFFSET>,
            CanBeDelegated: CanBeDelegated::<Impl, IMPL_OFFSET>,
            BizrulesWritable: BizrulesWritable::<Impl, IMPL_OFFSET>,
            PolicyAdministratorsName: PolicyAdministratorsName::<Impl, IMPL_OFFSET>,
            PolicyReadersName: PolicyReadersName::<Impl, IMPL_OFFSET>,
            AddPolicyAdministratorName: AddPolicyAdministratorName::<Impl, IMPL_OFFSET>,
            DeletePolicyAdministratorName: DeletePolicyAdministratorName::<Impl, IMPL_OFFSET>,
            AddPolicyReaderName: AddPolicyReaderName::<Impl, IMPL_OFFSET>,
            DeletePolicyReaderName: DeletePolicyReaderName::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzScope as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzScope2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IAzScope_Impl {
    fn RoleDefinitions(&mut self) -> ::windows::core::Result<IAzRoleDefinitions>;
    fn CreateRoleDefinition(&mut self, bstrroledefinitionname: super::super::Foundation::BSTR) -> ::windows::core::Result<IAzRoleDefinition>;
    fn OpenRoleDefinition(&mut self, bstrroledefinitionname: super::super::Foundation::BSTR) -> ::windows::core::Result<IAzRoleDefinition>;
    fn DeleteRoleDefinition(&mut self, bstrroledefinitionname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RoleAssignments(&mut self) -> ::windows::core::Result<IAzRoleAssignments>;
    fn CreateRoleAssignment(&mut self, bstrroleassignmentname: super::super::Foundation::BSTR) -> ::windows::core::Result<IAzRoleAssignment>;
    fn OpenRoleAssignment(&mut self, bstrroleassignmentname: super::super::Foundation::BSTR) -> ::windows::core::Result<IAzRoleAssignment>;
    fn DeleteRoleAssignment(&mut self, bstrroleassignmentname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzScope2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzScope2_Vtbl {
        unsafe extern "system" fn RoleDefinitions<Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoleDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    *pproledefinitions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRoleDefinition<Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRoleDefinition(::core::mem::transmute_copy(&bstrroledefinitionname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproledefinitions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRoleDefinition<Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenRoleDefinition(::core::mem::transmute_copy(&bstrroledefinitionname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproledefinitions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRoleDefinition<Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteRoleDefinition(::core::mem::transmute_copy(&bstrroledefinitionname)).into()
        }
        unsafe extern "system" fn RoleAssignments<Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproleassignments: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoleAssignments() {
                ::core::result::Result::Ok(ok__) => {
                    *pproleassignments = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRoleAssignment<Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproleassignment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CreateRoleAssignment(::core::mem::transmute_copy(&bstrroleassignmentname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproleassignment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRoleAssignment<Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproleassignment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).OpenRoleAssignment(::core::mem::transmute_copy(&bstrroleassignmentname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproleassignment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRoleAssignment<Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteRoleAssignment(::core::mem::transmute_copy(&bstrroleassignmentname)).into()
        }
        Self {
            base: IAzScope_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            RoleDefinitions: RoleDefinitions::<Impl, IMPL_OFFSET>,
            CreateRoleDefinition: CreateRoleDefinition::<Impl, IMPL_OFFSET>,
            OpenRoleDefinition: OpenRoleDefinition::<Impl, IMPL_OFFSET>,
            DeleteRoleDefinition: DeleteRoleDefinition::<Impl, IMPL_OFFSET>,
            RoleAssignments: RoleAssignments::<Impl, IMPL_OFFSET>,
            CreateRoleAssignment: CreateRoleAssignment::<Impl, IMPL_OFFSET>,
            OpenRoleAssignment: OpenRoleAssignment::<Impl, IMPL_OFFSET>,
            DeleteRoleAssignment: DeleteRoleAssignment::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzScope2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzScopes_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Item(&mut self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzScopes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzScopes_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzScopes_Vtbl {
        unsafe extern "system" fn Item<Impl: IAzScopes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarobtptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IAzScopes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IAzScopes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzScopes as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzTask_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, bstrname: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ApplicationData(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetApplicationData(&mut self, bstrapplicationdata: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BizRule(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetBizRule(&mut self, bstrprop: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BizRuleLanguage(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetBizRuleLanguage(&mut self, bstrprop: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BizRuleImportedPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetBizRuleImportedPath(&mut self, bstrprop: super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsRoleDefinition(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetIsRoleDefinition(&mut self, fprop: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Operations(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Tasks(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddOperation(&mut self, bstrop: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteOperation(&mut self, bstrop: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddTask(&mut self, bstrtask: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteTask(&mut self, bstrtask: super::super::Foundation::BSTR, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Writable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&mut self, lpropid: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&mut self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPropertyItem(&mut self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePropertyItem(&mut self, lpropid: i32, varprop: super::super::System::Com::VARIANT, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Submit(&mut self, lflags: i32, varreserved: super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzTask_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzTask_Vtbl {
        unsafe extern "system" fn Name<Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn Description<Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn ApplicationData<Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ApplicationData() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrapplicationdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetApplicationData(::core::mem::transmute_copy(&bstrapplicationdata)).into()
        }
        unsafe extern "system" fn BizRule<Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BizRule() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRule<Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBizRule(::core::mem::transmute_copy(&bstrprop)).into()
        }
        unsafe extern "system" fn BizRuleLanguage<Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BizRuleLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRuleLanguage<Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBizRuleLanguage(::core::mem::transmute_copy(&bstrprop)).into()
        }
        unsafe extern "system" fn BizRuleImportedPath<Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BizRuleImportedPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRuleImportedPath<Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetBizRuleImportedPath(::core::mem::transmute_copy(&bstrprop)).into()
        }
        unsafe extern "system" fn IsRoleDefinition<Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).IsRoleDefinition() {
                ::core::result::Result::Ok(ok__) => {
                    *pfprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRoleDefinition<Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fprop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetIsRoleDefinition(::core::mem::transmute_copy(&fprop)).into()
        }
        unsafe extern "system" fn Operations<Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Operations() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tasks<Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Tasks() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddOperation<Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddOperation(::core::mem::transmute_copy(&bstrop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteOperation<Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteOperation(::core::mem::transmute_copy(&bstrop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddTask<Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddTask(::core::mem::transmute_copy(&bstrtask), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteTask<Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeleteTask(::core::mem::transmute_copy(&bstrtask), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Writable<Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Writable() {
                ::core::result::Result::Ok(ok__) => {
                    *pfprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddPropertyItem<Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).AddPropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePropertyItem<Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).DeletePropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&varreserved)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Name: Name::<Impl, IMPL_OFFSET>,
            SetName: SetName::<Impl, IMPL_OFFSET>,
            Description: Description::<Impl, IMPL_OFFSET>,
            SetDescription: SetDescription::<Impl, IMPL_OFFSET>,
            ApplicationData: ApplicationData::<Impl, IMPL_OFFSET>,
            SetApplicationData: SetApplicationData::<Impl, IMPL_OFFSET>,
            BizRule: BizRule::<Impl, IMPL_OFFSET>,
            SetBizRule: SetBizRule::<Impl, IMPL_OFFSET>,
            BizRuleLanguage: BizRuleLanguage::<Impl, IMPL_OFFSET>,
            SetBizRuleLanguage: SetBizRuleLanguage::<Impl, IMPL_OFFSET>,
            BizRuleImportedPath: BizRuleImportedPath::<Impl, IMPL_OFFSET>,
            SetBizRuleImportedPath: SetBizRuleImportedPath::<Impl, IMPL_OFFSET>,
            IsRoleDefinition: IsRoleDefinition::<Impl, IMPL_OFFSET>,
            SetIsRoleDefinition: SetIsRoleDefinition::<Impl, IMPL_OFFSET>,
            Operations: Operations::<Impl, IMPL_OFFSET>,
            Tasks: Tasks::<Impl, IMPL_OFFSET>,
            AddOperation: AddOperation::<Impl, IMPL_OFFSET>,
            DeleteOperation: DeleteOperation::<Impl, IMPL_OFFSET>,
            AddTask: AddTask::<Impl, IMPL_OFFSET>,
            DeleteTask: DeleteTask::<Impl, IMPL_OFFSET>,
            Writable: Writable::<Impl, IMPL_OFFSET>,
            GetProperty: GetProperty::<Impl, IMPL_OFFSET>,
            SetProperty: SetProperty::<Impl, IMPL_OFFSET>,
            AddPropertyItem: AddPropertyItem::<Impl, IMPL_OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Impl, IMPL_OFFSET>,
            Submit: Submit::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzTask as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzTask2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IAzTask_Impl {
    fn RoleAssignments(&mut self, bstrscopename: super::super::Foundation::BSTR, brecursive: i16) -> ::windows::core::Result<IAzRoleAssignments>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzTask2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask2_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzTask2_Vtbl {
        unsafe extern "system" fn RoleAssignments<Impl: IAzTask2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, brecursive: i16, pproleassignments: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).RoleAssignments(::core::mem::transmute_copy(&bstrscopename), ::core::mem::transmute_copy(&brecursive)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproleassignments = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IAzTask_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(), RoleAssignments: RoleAssignments::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzTask2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzTasks_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Item(&mut self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Count(&mut self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&mut self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzTasks_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzTasks_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IAzTasks_Vtbl {
        unsafe extern "system" fn Item<Impl: IAzTasks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarobtptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Impl: IAzTasks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Impl: IAzTasks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, BASE_OFFSET, IMPL_OFFSET>(),
            Item: Item::<Impl, IMPL_OFFSET>,
            Count: Count::<Impl, IMPL_OFFSET>,
            _NewEnum: _NewEnum::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzTasks as ::windows::core::Interface>::IID
    }
}
