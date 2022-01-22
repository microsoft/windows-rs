#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzApplication_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ApplicationData(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetApplicationData(&mut self, bstrapplicationdata: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AuthzInterfaceClsid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetAuthzInterfaceClsid(&mut self, bstrprop: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Version(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetVersion(&mut self, bstrprop: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GenerateAudits(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetGenerateAudits(&mut self, bprop: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ApplyStoreSacl(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetApplyStoreSacl(&mut self, bprop: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Writable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&mut self, lpropid: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&mut self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PolicyAdministrators(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PolicyReaders(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddPolicyAdministrator(&mut self, bstradmin: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyAdministrator(&mut self, bstradmin: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPolicyReader(&mut self, bstrreader: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyReader(&mut self, bstrreader: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Scopes(&mut self) -> ::windows::core::Result<IAzScopes>;
    fn OpenScope(&mut self, bstrscopename: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzScope>;
    fn CreateScope(&mut self, bstrscopename: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzScope>;
    fn DeleteScope(&mut self, bstrscopename: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Operations(&mut self) -> ::windows::core::Result<IAzOperations>;
    fn OpenOperation(&mut self, bstroperationname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzOperation>;
    fn CreateOperation(&mut self, bstroperationname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzOperation>;
    fn DeleteOperation(&mut self, bstroperationname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Tasks(&mut self) -> ::windows::core::Result<IAzTasks>;
    fn OpenTask(&mut self, bstrtaskname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzTask>;
    fn CreateTask(&mut self, bstrtaskname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzTask>;
    fn DeleteTask(&mut self, bstrtaskname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ApplicationGroups(&mut self) -> ::windows::core::Result<IAzApplicationGroups>;
    fn OpenApplicationGroup(&mut self, bstrgroupname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>;
    fn CreateApplicationGroup(&mut self, bstrgroupname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>;
    fn DeleteApplicationGroup(&mut self, bstrgroupname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Roles(&mut self) -> ::windows::core::Result<IAzRoles>;
    fn OpenRole(&mut self, bstrrolename: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzRole>;
    fn CreateRole(&mut self, bstrrolename: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzRole>;
    fn DeleteRole(&mut self, bstrrolename: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn InitializeClientContextFromToken(&mut self, ulltokenhandle: u64, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext>;
    fn AddPropertyItem(&mut self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePropertyItem(&mut self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Submit(&mut self, lflags: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn InitializeClientContextFromName(&mut self, clientname: &super::super::Foundation::BSTR, domainname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext>;
    fn DelegatedPolicyUsers(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddDelegatedPolicyUser(&mut self, bstrdelegatedpolicyuser: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteDelegatedPolicyUser(&mut self, bstrdelegatedpolicyuser: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn InitializeClientContextFromStringSid(&mut self, sidstring: &super::super::Foundation::BSTR, loptions: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext>;
    fn PolicyAdministratorsName(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PolicyReadersName(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddPolicyAdministratorName(&mut self, bstradmin: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyAdministratorName(&mut self, bstradmin: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPolicyReaderName(&mut self, bstrreader: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyReaderName(&mut self, bstrreader: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DelegatedPolicyUsersName(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddDelegatedPolicyUserName(&mut self, bstrdelegatedpolicyuser: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteDelegatedPolicyUserName(&mut self, bstrdelegatedpolicyuser: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzApplication_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>() -> IAzApplication_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn ApplicationData<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ApplicationData() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrapplicationdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetApplicationData(::core::mem::transmute_copy(&bstrapplicationdata)).into()
        }
        unsafe extern "system" fn AuthzInterfaceClsid<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AuthzInterfaceClsid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthzInterfaceClsid<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetAuthzInterfaceClsid(::core::mem::transmute_copy(&bstrprop)).into()
        }
        unsafe extern "system" fn Version<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Version() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVersion<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetVersion(::core::mem::transmute_copy(&bstrprop)).into()
        }
        unsafe extern "system" fn GenerateAudits<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GenerateAudits() {
                ::core::result::Result::Ok(ok__) => {
                    *pbprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGenerateAudits<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bprop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGenerateAudits(::core::mem::transmute_copy(&bprop)).into()
        }
        unsafe extern "system" fn ApplyStoreSacl<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ApplyStoreSacl() {
                ::core::result::Result::Ok(ok__) => {
                    *pbprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplyStoreSacl<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bprop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetApplyStoreSacl(::core::mem::transmute_copy(&bprop)).into()
        }
        unsafe extern "system" fn Writable<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Writable() {
                ::core::result::Result::Ok(ok__) => {
                    *pfprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn PolicyAdministrators<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PolicyAdministrators() {
                ::core::result::Result::Ok(ok__) => {
                    *pvaradmins = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReaders<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PolicyReaders() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarreaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministrator<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPolicyAdministrator(::core::mem::transmute_copy(&bstradmin), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyAdministrator<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeletePolicyAdministrator(::core::mem::transmute_copy(&bstradmin), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddPolicyReader<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPolicyReader(::core::mem::transmute_copy(&bstrreader), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyReader<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeletePolicyReader(::core::mem::transmute_copy(&bstrreader), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Scopes<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppscopecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Scopes() {
                ::core::result::Result::Ok(ok__) => {
                    *ppscopecollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenScope<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppscope: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenScope(::core::mem::transmute_copy(&bstrscopename), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppscope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScope<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppscope: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateScope(::core::mem::transmute_copy(&bstrscopename), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppscope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteScope<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteScope(::core::mem::transmute_copy(&bstrscopename), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Operations<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoperationcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Operations() {
                ::core::result::Result::Ok(ok__) => {
                    *ppoperationcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenOperation<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroperationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppoperation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenOperation(::core::mem::transmute_copy(&bstroperationname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoperation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOperation<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroperationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppoperation: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateOperation(::core::mem::transmute_copy(&bstroperationname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoperation = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteOperation<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroperationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteOperation(::core::mem::transmute_copy(&bstroperationname), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Tasks<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptaskcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Tasks() {
                ::core::result::Result::Ok(ok__) => {
                    *pptaskcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTask<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pptask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenTask(::core::mem::transmute_copy(&bstrtaskname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTask<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pptask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTask(::core::mem::transmute_copy(&bstrtaskname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteTask<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteTask(::core::mem::transmute_copy(&bstrtaskname), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn ApplicationGroups<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroupcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ApplicationGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroupcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenApplicationGroup<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenApplicationGroup(::core::mem::transmute_copy(&bstrgroupname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateApplicationGroup<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateApplicationGroup(::core::mem::transmute_copy(&bstrgroupname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteApplicationGroup<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteApplicationGroup(::core::mem::transmute_copy(&bstrgroupname), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Roles<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprolecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Roles() {
                ::core::result::Result::Ok(ok__) => {
                    *pprolecollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRole<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pprole: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenRole(::core::mem::transmute_copy(&bstrrolename), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprole = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRole<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pprole: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateRole(::core::mem::transmute_copy(&bstrrolename), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprole = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRole<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteRole(::core::mem::transmute_copy(&bstrrolename), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn InitializeClientContextFromToken<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulltokenhandle: u64, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclientcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InitializeClientContextFromToken(::core::mem::transmute_copy(&ulltokenhandle), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclientcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPropertyItem<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeletePropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn InitializeClientContextFromName<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, domainname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclientcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InitializeClientContextFromName(::core::mem::transmute_copy(&clientname), ::core::mem::transmute_copy(&domainname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclientcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DelegatedPolicyUsers<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DelegatedPolicyUsers() {
                ::core::result::Result::Ok(ok__) => {
                    *pvardelegatedpolicyusers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDelegatedPolicyUser<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddDelegatedPolicyUser(::core::mem::transmute_copy(&bstrdelegatedpolicyuser), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUser<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteDelegatedPolicyUser(::core::mem::transmute_copy(&bstrdelegatedpolicyuser), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn InitializeClientContextFromStringSid<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sidstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclientcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InitializeClientContextFromStringSid(::core::mem::transmute_copy(&sidstring), ::core::mem::transmute_copy(&loptions), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclientcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyAdministratorsName<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PolicyAdministratorsName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvaradmins = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReadersName<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PolicyReadersName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarreaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministratorName<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPolicyAdministratorName(::core::mem::transmute_copy(&bstradmin), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyAdministratorName<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeletePolicyAdministratorName(::core::mem::transmute_copy(&bstradmin), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddPolicyReaderName<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPolicyReaderName(::core::mem::transmute_copy(&bstrreader), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyReaderName<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeletePolicyReaderName(::core::mem::transmute_copy(&bstrreader), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DelegatedPolicyUsersName<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DelegatedPolicyUsersName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvardelegatedpolicyusers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDelegatedPolicyUserName<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddDelegatedPolicyUserName(::core::mem::transmute_copy(&bstrdelegatedpolicyuser), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUserName<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteDelegatedPolicyUserName(::core::mem::transmute_copy(&bstrdelegatedpolicyuser), ::core::mem::transmute_copy(&varreserved)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            ApplicationData: ApplicationData::<Identity, Impl, OFFSET>,
            SetApplicationData: SetApplicationData::<Identity, Impl, OFFSET>,
            AuthzInterfaceClsid: AuthzInterfaceClsid::<Identity, Impl, OFFSET>,
            SetAuthzInterfaceClsid: SetAuthzInterfaceClsid::<Identity, Impl, OFFSET>,
            Version: Version::<Identity, Impl, OFFSET>,
            SetVersion: SetVersion::<Identity, Impl, OFFSET>,
            GenerateAudits: GenerateAudits::<Identity, Impl, OFFSET>,
            SetGenerateAudits: SetGenerateAudits::<Identity, Impl, OFFSET>,
            ApplyStoreSacl: ApplyStoreSacl::<Identity, Impl, OFFSET>,
            SetApplyStoreSacl: SetApplyStoreSacl::<Identity, Impl, OFFSET>,
            Writable: Writable::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            PolicyAdministrators: PolicyAdministrators::<Identity, Impl, OFFSET>,
            PolicyReaders: PolicyReaders::<Identity, Impl, OFFSET>,
            AddPolicyAdministrator: AddPolicyAdministrator::<Identity, Impl, OFFSET>,
            DeletePolicyAdministrator: DeletePolicyAdministrator::<Identity, Impl, OFFSET>,
            AddPolicyReader: AddPolicyReader::<Identity, Impl, OFFSET>,
            DeletePolicyReader: DeletePolicyReader::<Identity, Impl, OFFSET>,
            Scopes: Scopes::<Identity, Impl, OFFSET>,
            OpenScope: OpenScope::<Identity, Impl, OFFSET>,
            CreateScope: CreateScope::<Identity, Impl, OFFSET>,
            DeleteScope: DeleteScope::<Identity, Impl, OFFSET>,
            Operations: Operations::<Identity, Impl, OFFSET>,
            OpenOperation: OpenOperation::<Identity, Impl, OFFSET>,
            CreateOperation: CreateOperation::<Identity, Impl, OFFSET>,
            DeleteOperation: DeleteOperation::<Identity, Impl, OFFSET>,
            Tasks: Tasks::<Identity, Impl, OFFSET>,
            OpenTask: OpenTask::<Identity, Impl, OFFSET>,
            CreateTask: CreateTask::<Identity, Impl, OFFSET>,
            DeleteTask: DeleteTask::<Identity, Impl, OFFSET>,
            ApplicationGroups: ApplicationGroups::<Identity, Impl, OFFSET>,
            OpenApplicationGroup: OpenApplicationGroup::<Identity, Impl, OFFSET>,
            CreateApplicationGroup: CreateApplicationGroup::<Identity, Impl, OFFSET>,
            DeleteApplicationGroup: DeleteApplicationGroup::<Identity, Impl, OFFSET>,
            Roles: Roles::<Identity, Impl, OFFSET>,
            OpenRole: OpenRole::<Identity, Impl, OFFSET>,
            CreateRole: CreateRole::<Identity, Impl, OFFSET>,
            DeleteRole: DeleteRole::<Identity, Impl, OFFSET>,
            InitializeClientContextFromToken: InitializeClientContextFromToken::<Identity, Impl, OFFSET>,
            AddPropertyItem: AddPropertyItem::<Identity, Impl, OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Identity, Impl, OFFSET>,
            Submit: Submit::<Identity, Impl, OFFSET>,
            InitializeClientContextFromName: InitializeClientContextFromName::<Identity, Impl, OFFSET>,
            DelegatedPolicyUsers: DelegatedPolicyUsers::<Identity, Impl, OFFSET>,
            AddDelegatedPolicyUser: AddDelegatedPolicyUser::<Identity, Impl, OFFSET>,
            DeleteDelegatedPolicyUser: DeleteDelegatedPolicyUser::<Identity, Impl, OFFSET>,
            InitializeClientContextFromStringSid: InitializeClientContextFromStringSid::<Identity, Impl, OFFSET>,
            PolicyAdministratorsName: PolicyAdministratorsName::<Identity, Impl, OFFSET>,
            PolicyReadersName: PolicyReadersName::<Identity, Impl, OFFSET>,
            AddPolicyAdministratorName: AddPolicyAdministratorName::<Identity, Impl, OFFSET>,
            DeletePolicyAdministratorName: DeletePolicyAdministratorName::<Identity, Impl, OFFSET>,
            AddPolicyReaderName: AddPolicyReaderName::<Identity, Impl, OFFSET>,
            DeletePolicyReaderName: DeletePolicyReaderName::<Identity, Impl, OFFSET>,
            DelegatedPolicyUsersName: DelegatedPolicyUsersName::<Identity, Impl, OFFSET>,
            AddDelegatedPolicyUserName: AddDelegatedPolicyUserName::<Identity, Impl, OFFSET>,
            DeleteDelegatedPolicyUserName: DeleteDelegatedPolicyUserName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzApplication as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzApplication2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IAzApplication_Impl {
    fn InitializeClientContextFromToken2(&mut self, ultokenhandlelowpart: u32, ultokenhandlehighpart: u32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext2>;
    fn InitializeClientContext2(&mut self, identifyingstring: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzApplication2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication2_Impl, const OFFSET: isize>() -> IAzApplication2_Vtbl {
        unsafe extern "system" fn InitializeClientContextFromToken2<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ultokenhandlelowpart: u32, ultokenhandlehighpart: u32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclientcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InitializeClientContextFromToken2(::core::mem::transmute_copy(&ultokenhandlelowpart), ::core::mem::transmute_copy(&ultokenhandlehighpart), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclientcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeClientContext2<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifyingstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclientcontext: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).InitializeClientContext2(::core::mem::transmute_copy(&identifyingstring), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppclientcontext = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IAzApplication_Vtbl::new::<Identity, Impl, OFFSET>(),
            InitializeClientContextFromToken2: InitializeClientContextFromToken2::<Identity, Impl, OFFSET>,
            InitializeClientContext2: InitializeClientContext2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzApplication2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IAzApplication as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzApplication3_Impl: Sized + super::super::System::Com::IDispatch_Impl + IAzApplication_Impl + IAzApplication2_Impl {
    fn ScopeExists(&mut self, bstrscopename: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn OpenScope2(&mut self, bstrscopename: &super::super::Foundation::BSTR) -> ::windows::core::Result<IAzScope2>;
    fn CreateScope2(&mut self, bstrscopename: &super::super::Foundation::BSTR) -> ::windows::core::Result<IAzScope2>;
    fn DeleteScope2(&mut self, bstrscopename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RoleDefinitions(&mut self) -> ::windows::core::Result<IAzRoleDefinitions>;
    fn CreateRoleDefinition(&mut self, bstrroledefinitionname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IAzRoleDefinition>;
    fn OpenRoleDefinition(&mut self, bstrroledefinitionname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IAzRoleDefinition>;
    fn DeleteRoleDefinition(&mut self, bstrroledefinitionname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RoleAssignments(&mut self) -> ::windows::core::Result<IAzRoleAssignments>;
    fn CreateRoleAssignment(&mut self, bstrroleassignmentname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IAzRoleAssignment>;
    fn OpenRoleAssignment(&mut self, bstrroleassignmentname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IAzRoleAssignment>;
    fn DeleteRoleAssignment(&mut self, bstrroleassignmentname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BizRulesEnabled(&mut self) -> ::windows::core::Result<i16>;
    fn SetBizRulesEnabled(&mut self, benabled: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzApplication3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication3_Impl, const OFFSET: isize>() -> IAzApplication3_Vtbl {
        unsafe extern "system" fn ScopeExists<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbexist: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ScopeExists(::core::mem::transmute_copy(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbexist = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenScope2<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppscope2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenScope2(::core::mem::transmute_copy(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppscope2 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScope2<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppscope2: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateScope2(::core::mem::transmute_copy(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppscope2 = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteScope2<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteScope2(::core::mem::transmute_copy(&bstrscopename)).into()
        }
        unsafe extern "system" fn RoleDefinitions<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RoleDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    *pproledefinitions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRoleDefinition<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateRoleDefinition(::core::mem::transmute_copy(&bstrroledefinitionname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproledefinitions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRoleDefinition<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenRoleDefinition(::core::mem::transmute_copy(&bstrroledefinitionname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproledefinitions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRoleDefinition<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteRoleDefinition(::core::mem::transmute_copy(&bstrroledefinitionname)).into()
        }
        unsafe extern "system" fn RoleAssignments<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproleassignments: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RoleAssignments() {
                ::core::result::Result::Ok(ok__) => {
                    *pproleassignments = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRoleAssignment<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproleassignment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateRoleAssignment(::core::mem::transmute_copy(&bstrroleassignmentname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproleassignment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRoleAssignment<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproleassignment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenRoleAssignment(::core::mem::transmute_copy(&bstrroleassignmentname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproleassignment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRoleAssignment<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteRoleAssignment(::core::mem::transmute_copy(&bstrroleassignmentname)).into()
        }
        unsafe extern "system" fn BizRulesEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BizRulesEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    *pbenabled = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRulesEnabled<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBizRulesEnabled(::core::mem::transmute_copy(&benabled)).into()
        }
        Self {
            base: IAzApplication2_Vtbl::new::<Identity, Impl, OFFSET>(),
            ScopeExists: ScopeExists::<Identity, Impl, OFFSET>,
            OpenScope2: OpenScope2::<Identity, Impl, OFFSET>,
            CreateScope2: CreateScope2::<Identity, Impl, OFFSET>,
            DeleteScope2: DeleteScope2::<Identity, Impl, OFFSET>,
            RoleDefinitions: RoleDefinitions::<Identity, Impl, OFFSET>,
            CreateRoleDefinition: CreateRoleDefinition::<Identity, Impl, OFFSET>,
            OpenRoleDefinition: OpenRoleDefinition::<Identity, Impl, OFFSET>,
            DeleteRoleDefinition: DeleteRoleDefinition::<Identity, Impl, OFFSET>,
            RoleAssignments: RoleAssignments::<Identity, Impl, OFFSET>,
            CreateRoleAssignment: CreateRoleAssignment::<Identity, Impl, OFFSET>,
            OpenRoleAssignment: OpenRoleAssignment::<Identity, Impl, OFFSET>,
            DeleteRoleAssignment: DeleteRoleAssignment::<Identity, Impl, OFFSET>,
            BizRulesEnabled: BizRulesEnabled::<Identity, Impl, OFFSET>,
            SetBizRulesEnabled: SetBizRulesEnabled::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzApplication3 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IAzApplication as ::windows::core::Interface>::IID || iid == &<IAzApplication2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzApplicationGroup_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Type(&mut self) -> ::windows::core::Result<i32>;
    fn SetType(&mut self, lprop: i32) -> ::windows::core::Result<()>;
    fn LdapQuery(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLdapQuery(&mut self, bstrprop: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AppMembers(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AppNonMembers(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Members(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn NonMembers(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddAppMember(&mut self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteAppMember(&mut self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddAppNonMember(&mut self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteAppNonMember(&mut self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddMember(&mut self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteMember(&mut self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddNonMember(&mut self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteNonMember(&mut self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Writable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&mut self, lpropid: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&mut self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPropertyItem(&mut self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePropertyItem(&mut self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Submit(&mut self, lflags: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddMemberName(&mut self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteMemberName(&mut self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddNonMemberName(&mut self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteNonMemberName(&mut self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MembersName(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn NonMembersName(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzApplicationGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>() -> IAzApplicationGroup_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Type() {
                ::core::result::Result::Ok(ok__) => {
                    *plprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetType(::core::mem::transmute_copy(&lprop)).into()
        }
        unsafe extern "system" fn LdapQuery<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LdapQuery() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLdapQuery<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLdapQuery(::core::mem::transmute_copy(&bstrprop)).into()
        }
        unsafe extern "system" fn AppMembers<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AppMembers() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppNonMembers<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AppNonMembers() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Members<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Members() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NonMembers<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NonMembers() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn AddAppMember<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddAppMember(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteAppMember<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteAppMember(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddAppNonMember<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddAppNonMember(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteAppNonMember<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteAppNonMember(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddMember<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddMember(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteMember<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteMember(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddNonMember<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddNonMember(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteNonMember<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteNonMember(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Writable<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Writable() {
                ::core::result::Result::Ok(ok__) => {
                    *pfprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddPropertyItem<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeletePropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddMemberName<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddMemberName(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteMemberName<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteMemberName(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddNonMemberName<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddNonMemberName(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteNonMemberName<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteNonMemberName(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn MembersName<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MembersName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NonMembersName<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NonMembersName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Type: Type::<Identity, Impl, OFFSET>,
            SetType: SetType::<Identity, Impl, OFFSET>,
            LdapQuery: LdapQuery::<Identity, Impl, OFFSET>,
            SetLdapQuery: SetLdapQuery::<Identity, Impl, OFFSET>,
            AppMembers: AppMembers::<Identity, Impl, OFFSET>,
            AppNonMembers: AppNonMembers::<Identity, Impl, OFFSET>,
            Members: Members::<Identity, Impl, OFFSET>,
            NonMembers: NonMembers::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            AddAppMember: AddAppMember::<Identity, Impl, OFFSET>,
            DeleteAppMember: DeleteAppMember::<Identity, Impl, OFFSET>,
            AddAppNonMember: AddAppNonMember::<Identity, Impl, OFFSET>,
            DeleteAppNonMember: DeleteAppNonMember::<Identity, Impl, OFFSET>,
            AddMember: AddMember::<Identity, Impl, OFFSET>,
            DeleteMember: DeleteMember::<Identity, Impl, OFFSET>,
            AddNonMember: AddNonMember::<Identity, Impl, OFFSET>,
            DeleteNonMember: DeleteNonMember::<Identity, Impl, OFFSET>,
            Writable: Writable::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            AddPropertyItem: AddPropertyItem::<Identity, Impl, OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Identity, Impl, OFFSET>,
            Submit: Submit::<Identity, Impl, OFFSET>,
            AddMemberName: AddMemberName::<Identity, Impl, OFFSET>,
            DeleteMemberName: DeleteMemberName::<Identity, Impl, OFFSET>,
            AddNonMemberName: AddNonMemberName::<Identity, Impl, OFFSET>,
            DeleteNonMemberName: DeleteNonMemberName::<Identity, Impl, OFFSET>,
            MembersName: MembersName::<Identity, Impl, OFFSET>,
            NonMembersName: NonMembersName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzApplicationGroup as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzApplicationGroup2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IAzApplicationGroup_Impl {
    fn BizRule(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetBizRule(&mut self, bstrprop: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BizRuleLanguage(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetBizRuleLanguage(&mut self, bstrprop: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BizRuleImportedPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetBizRuleImportedPath(&mut self, bstrprop: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RoleAssignments(&mut self, bstrscopename: &super::super::Foundation::BSTR, brecursive: i16) -> ::windows::core::Result<IAzRoleAssignments>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzApplicationGroup2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>() -> IAzApplicationGroup2_Vtbl {
        unsafe extern "system" fn BizRule<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BizRule() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRule<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBizRule(::core::mem::transmute_copy(&bstrprop)).into()
        }
        unsafe extern "system" fn BizRuleLanguage<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BizRuleLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRuleLanguage<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBizRuleLanguage(::core::mem::transmute_copy(&bstrprop)).into()
        }
        unsafe extern "system" fn BizRuleImportedPath<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BizRuleImportedPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRuleImportedPath<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBizRuleImportedPath(::core::mem::transmute_copy(&bstrprop)).into()
        }
        unsafe extern "system" fn RoleAssignments<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, brecursive: i16, pproleassignments: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RoleAssignments(::core::mem::transmute_copy(&bstrscopename), ::core::mem::transmute_copy(&brecursive)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproleassignments = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IAzApplicationGroup_Vtbl::new::<Identity, Impl, OFFSET>(),
            BizRule: BizRule::<Identity, Impl, OFFSET>,
            SetBizRule: SetBizRule::<Identity, Impl, OFFSET>,
            BizRuleLanguage: BizRuleLanguage::<Identity, Impl, OFFSET>,
            SetBizRuleLanguage: SetBizRuleLanguage::<Identity, Impl, OFFSET>,
            BizRuleImportedPath: BizRuleImportedPath::<Identity, Impl, OFFSET>,
            SetBizRuleImportedPath: SetBizRuleImportedPath::<Identity, Impl, OFFSET>,
            RoleAssignments: RoleAssignments::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzApplicationGroup2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IAzApplicationGroup as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroups_Impl, const OFFSET: isize>() -> IAzApplicationGroups_Vtbl {
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarobtptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplicationGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzApplicationGroups as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplications_Impl, const OFFSET: isize>() -> IAzApplications_Vtbl {
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarobtptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IAzApplications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzApplications as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzAuthorizationStore_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ApplicationData(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetApplicationData(&mut self, bstrapplicationdata: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DomainTimeout(&mut self) -> ::windows::core::Result<i32>;
    fn SetDomainTimeout(&mut self, lprop: i32) -> ::windows::core::Result<()>;
    fn ScriptEngineTimeout(&mut self) -> ::windows::core::Result<i32>;
    fn SetScriptEngineTimeout(&mut self, lprop: i32) -> ::windows::core::Result<()>;
    fn MaxScriptEngines(&mut self) -> ::windows::core::Result<i32>;
    fn SetMaxScriptEngines(&mut self, lprop: i32) -> ::windows::core::Result<()>;
    fn GenerateAudits(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetGenerateAudits(&mut self, bprop: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Writable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&mut self, lpropid: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&mut self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPropertyItem(&mut self, lpropid: AZ_PROP_CONSTANTS, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePropertyItem(&mut self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PolicyAdministrators(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PolicyReaders(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddPolicyAdministrator(&mut self, bstradmin: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyAdministrator(&mut self, bstradmin: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPolicyReader(&mut self, bstrreader: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyReader(&mut self, bstrreader: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Initialize(&mut self, lflags: AZ_PROP_CONSTANTS, bstrpolicyurl: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn UpdateCache(&mut self, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Delete(&mut self, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Applications(&mut self) -> ::windows::core::Result<IAzApplications>;
    fn OpenApplication(&mut self, bstrapplicationname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplication>;
    fn CreateApplication(&mut self, bstrapplicationname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplication>;
    fn DeleteApplication(&mut self, bstrapplicationname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ApplicationGroups(&mut self) -> ::windows::core::Result<IAzApplicationGroups>;
    fn CreateApplicationGroup(&mut self, bstrgroupname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>;
    fn OpenApplicationGroup(&mut self, bstrgroupname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>;
    fn DeleteApplicationGroup(&mut self, bstrgroupname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Submit(&mut self, lflags: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DelegatedPolicyUsers(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddDelegatedPolicyUser(&mut self, bstrdelegatedpolicyuser: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteDelegatedPolicyUser(&mut self, bstrdelegatedpolicyuser: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn TargetMachine(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ApplyStoreSacl(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetApplyStoreSacl(&mut self, bapplystoresacl: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn PolicyAdministratorsName(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PolicyReadersName(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddPolicyAdministratorName(&mut self, bstradmin: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyAdministratorName(&mut self, bstradmin: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPolicyReaderName(&mut self, bstrreader: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyReaderName(&mut self, bstrreader: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DelegatedPolicyUsersName(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddDelegatedPolicyUserName(&mut self, bstrdelegatedpolicyuser: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteDelegatedPolicyUserName(&mut self, bstrdelegatedpolicyuser: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn CloseApplication(&mut self, bstrapplicationname: &super::super::Foundation::BSTR, lflag: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzAuthorizationStore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>() -> IAzAuthorizationStore_Vtbl {
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn ApplicationData<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ApplicationData() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrapplicationdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetApplicationData(::core::mem::transmute_copy(&bstrapplicationdata)).into()
        }
        unsafe extern "system" fn DomainTimeout<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DomainTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *plprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDomainTimeout<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDomainTimeout(::core::mem::transmute_copy(&lprop)).into()
        }
        unsafe extern "system" fn ScriptEngineTimeout<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ScriptEngineTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    *plprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScriptEngineTimeout<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetScriptEngineTimeout(::core::mem::transmute_copy(&lprop)).into()
        }
        unsafe extern "system" fn MaxScriptEngines<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MaxScriptEngines() {
                ::core::result::Result::Ok(ok__) => {
                    *plprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxScriptEngines<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetMaxScriptEngines(::core::mem::transmute_copy(&lprop)).into()
        }
        unsafe extern "system" fn GenerateAudits<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GenerateAudits() {
                ::core::result::Result::Ok(ok__) => {
                    *pbprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGenerateAudits<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bprop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetGenerateAudits(::core::mem::transmute_copy(&bprop)).into()
        }
        unsafe extern "system" fn Writable<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Writable() {
                ::core::result::Result::Ok(ok__) => {
                    *pfprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddPropertyItem<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: AZ_PROP_CONSTANTS, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeletePropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn PolicyAdministrators<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PolicyAdministrators() {
                ::core::result::Result::Ok(ok__) => {
                    *pvaradmins = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReaders<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PolicyReaders() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarreaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministrator<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPolicyAdministrator(::core::mem::transmute_copy(&bstradmin), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyAdministrator<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeletePolicyAdministrator(::core::mem::transmute_copy(&bstradmin), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddPolicyReader<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPolicyReader(::core::mem::transmute_copy(&bstrreader), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyReader<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeletePolicyReader(::core::mem::transmute_copy(&bstrreader), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: AZ_PROP_CONSTANTS, bstrpolicyurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Initialize(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&bstrpolicyurl), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn UpdateCache<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpdateCache(::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Delete(::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Applications<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppappcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Applications() {
                ::core::result::Result::Ok(ok__) => {
                    *ppappcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenApplication<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppapplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenApplication(::core::mem::transmute_copy(&bstrapplicationname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppapplication = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateApplication<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppapplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateApplication(::core::mem::transmute_copy(&bstrapplicationname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppapplication = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteApplication<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteApplication(::core::mem::transmute_copy(&bstrapplicationname), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn ApplicationGroups<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroupcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ApplicationGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroupcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateApplicationGroup<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateApplicationGroup(::core::mem::transmute_copy(&bstrgroupname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenApplicationGroup<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenApplicationGroup(::core::mem::transmute_copy(&bstrgroupname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteApplicationGroup<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteApplicationGroup(::core::mem::transmute_copy(&bstrgroupname), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DelegatedPolicyUsers<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DelegatedPolicyUsers() {
                ::core::result::Result::Ok(ok__) => {
                    *pvardelegatedpolicyusers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDelegatedPolicyUser<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddDelegatedPolicyUser(::core::mem::transmute_copy(&bstrdelegatedpolicyuser), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUser<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteDelegatedPolicyUser(::core::mem::transmute_copy(&bstrdelegatedpolicyuser), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn TargetMachine<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtargetmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).TargetMachine() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrtargetmachine = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyStoreSacl<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbapplystoresacl: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ApplyStoreSacl() {
                ::core::result::Result::Ok(ok__) => {
                    *pbapplystoresacl = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplyStoreSacl<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bapplystoresacl: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetApplyStoreSacl(::core::mem::transmute_copy(&bapplystoresacl)).into()
        }
        unsafe extern "system" fn PolicyAdministratorsName<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PolicyAdministratorsName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvaradmins = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReadersName<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PolicyReadersName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarreaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministratorName<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPolicyAdministratorName(::core::mem::transmute_copy(&bstradmin), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyAdministratorName<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeletePolicyAdministratorName(::core::mem::transmute_copy(&bstradmin), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddPolicyReaderName<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPolicyReaderName(::core::mem::transmute_copy(&bstrreader), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyReaderName<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeletePolicyReaderName(::core::mem::transmute_copy(&bstrreader), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DelegatedPolicyUsersName<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).DelegatedPolicyUsersName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvardelegatedpolicyusers = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDelegatedPolicyUserName<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddDelegatedPolicyUserName(::core::mem::transmute_copy(&bstrdelegatedpolicyuser), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUserName<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteDelegatedPolicyUserName(::core::mem::transmute_copy(&bstrdelegatedpolicyuser), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn CloseApplication<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflag: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).CloseApplication(::core::mem::transmute_copy(&bstrapplicationname), ::core::mem::transmute_copy(&lflag)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            ApplicationData: ApplicationData::<Identity, Impl, OFFSET>,
            SetApplicationData: SetApplicationData::<Identity, Impl, OFFSET>,
            DomainTimeout: DomainTimeout::<Identity, Impl, OFFSET>,
            SetDomainTimeout: SetDomainTimeout::<Identity, Impl, OFFSET>,
            ScriptEngineTimeout: ScriptEngineTimeout::<Identity, Impl, OFFSET>,
            SetScriptEngineTimeout: SetScriptEngineTimeout::<Identity, Impl, OFFSET>,
            MaxScriptEngines: MaxScriptEngines::<Identity, Impl, OFFSET>,
            SetMaxScriptEngines: SetMaxScriptEngines::<Identity, Impl, OFFSET>,
            GenerateAudits: GenerateAudits::<Identity, Impl, OFFSET>,
            SetGenerateAudits: SetGenerateAudits::<Identity, Impl, OFFSET>,
            Writable: Writable::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            AddPropertyItem: AddPropertyItem::<Identity, Impl, OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Identity, Impl, OFFSET>,
            PolicyAdministrators: PolicyAdministrators::<Identity, Impl, OFFSET>,
            PolicyReaders: PolicyReaders::<Identity, Impl, OFFSET>,
            AddPolicyAdministrator: AddPolicyAdministrator::<Identity, Impl, OFFSET>,
            DeletePolicyAdministrator: DeletePolicyAdministrator::<Identity, Impl, OFFSET>,
            AddPolicyReader: AddPolicyReader::<Identity, Impl, OFFSET>,
            DeletePolicyReader: DeletePolicyReader::<Identity, Impl, OFFSET>,
            Initialize: Initialize::<Identity, Impl, OFFSET>,
            UpdateCache: UpdateCache::<Identity, Impl, OFFSET>,
            Delete: Delete::<Identity, Impl, OFFSET>,
            Applications: Applications::<Identity, Impl, OFFSET>,
            OpenApplication: OpenApplication::<Identity, Impl, OFFSET>,
            CreateApplication: CreateApplication::<Identity, Impl, OFFSET>,
            DeleteApplication: DeleteApplication::<Identity, Impl, OFFSET>,
            ApplicationGroups: ApplicationGroups::<Identity, Impl, OFFSET>,
            CreateApplicationGroup: CreateApplicationGroup::<Identity, Impl, OFFSET>,
            OpenApplicationGroup: OpenApplicationGroup::<Identity, Impl, OFFSET>,
            DeleteApplicationGroup: DeleteApplicationGroup::<Identity, Impl, OFFSET>,
            Submit: Submit::<Identity, Impl, OFFSET>,
            DelegatedPolicyUsers: DelegatedPolicyUsers::<Identity, Impl, OFFSET>,
            AddDelegatedPolicyUser: AddDelegatedPolicyUser::<Identity, Impl, OFFSET>,
            DeleteDelegatedPolicyUser: DeleteDelegatedPolicyUser::<Identity, Impl, OFFSET>,
            TargetMachine: TargetMachine::<Identity, Impl, OFFSET>,
            ApplyStoreSacl: ApplyStoreSacl::<Identity, Impl, OFFSET>,
            SetApplyStoreSacl: SetApplyStoreSacl::<Identity, Impl, OFFSET>,
            PolicyAdministratorsName: PolicyAdministratorsName::<Identity, Impl, OFFSET>,
            PolicyReadersName: PolicyReadersName::<Identity, Impl, OFFSET>,
            AddPolicyAdministratorName: AddPolicyAdministratorName::<Identity, Impl, OFFSET>,
            DeletePolicyAdministratorName: DeletePolicyAdministratorName::<Identity, Impl, OFFSET>,
            AddPolicyReaderName: AddPolicyReaderName::<Identity, Impl, OFFSET>,
            DeletePolicyReaderName: DeletePolicyReaderName::<Identity, Impl, OFFSET>,
            DelegatedPolicyUsersName: DelegatedPolicyUsersName::<Identity, Impl, OFFSET>,
            AddDelegatedPolicyUserName: AddDelegatedPolicyUserName::<Identity, Impl, OFFSET>,
            DeleteDelegatedPolicyUserName: DeleteDelegatedPolicyUserName::<Identity, Impl, OFFSET>,
            CloseApplication: CloseApplication::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzAuthorizationStore as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzAuthorizationStore2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IAzAuthorizationStore_Impl {
    fn OpenApplication2(&mut self, bstrapplicationname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplication2>;
    fn CreateApplication2(&mut self, bstrapplicationname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplication2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzAuthorizationStore2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore2_Impl, const OFFSET: isize>() -> IAzAuthorizationStore2_Vtbl {
        unsafe extern "system" fn OpenApplication2<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppapplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenApplication2(::core::mem::transmute_copy(&bstrapplicationname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppapplication = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateApplication2<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppapplication: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateApplication2(::core::mem::transmute_copy(&bstrapplicationname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppapplication = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IAzAuthorizationStore_Vtbl::new::<Identity, Impl, OFFSET>(),
            OpenApplication2: OpenApplication2::<Identity, Impl, OFFSET>,
            CreateApplication2: CreateApplication2::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzAuthorizationStore2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IAzAuthorizationStore as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore3_Impl, const OFFSET: isize>() -> IAzAuthorizationStore3_Vtbl {
        unsafe extern "system" fn IsUpdateNeeded<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisupdateneeded: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsUpdateNeeded() {
                ::core::result::Result::Ok(ok__) => {
                    *pbisupdateneeded = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BizruleGroupSupported<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsupported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BizruleGroupSupported() {
                ::core::result::Result::Ok(ok__) => {
                    *pbsupported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpgradeStoresFunctionalLevel<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfunctionallevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).UpgradeStoresFunctionalLevel(::core::mem::transmute_copy(&lfunctionallevel)).into()
        }
        unsafe extern "system" fn IsFunctionalLevelUpgradeSupported<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfunctionallevel: i32, pbsupported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsFunctionalLevelUpgradeSupported(::core::mem::transmute_copy(&lfunctionallevel)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbsupported = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSchemaVersion<Identity: ::windows::core::IUnknownImpl, Impl: IAzAuthorizationStore3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetSchemaVersion(::core::mem::transmute_copy(&plmajorversion), ::core::mem::transmute_copy(&plminorversion)).into()
        }
        Self {
            base: IAzAuthorizationStore2_Vtbl::new::<Identity, Impl, OFFSET>(),
            IsUpdateNeeded: IsUpdateNeeded::<Identity, Impl, OFFSET>,
            BizruleGroupSupported: BizruleGroupSupported::<Identity, Impl, OFFSET>,
            UpgradeStoresFunctionalLevel: UpgradeStoresFunctionalLevel::<Identity, Impl, OFFSET>,
            IsFunctionalLevelUpgradeSupported: IsFunctionalLevelUpgradeSupported::<Identity, Impl, OFFSET>,
            GetSchemaVersion: GetSchemaVersion::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzAuthorizationStore3 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IAzAuthorizationStore as ::windows::core::Interface>::IID || iid == &<IAzAuthorizationStore2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzBizRuleContext_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn SetBusinessRuleResult(&mut self, bresult: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetBusinessRuleString(&mut self, bstrbusinessrulestring: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BusinessRuleString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetParameter(&mut self, bstrparametername: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzBizRuleContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzBizRuleContext_Impl, const OFFSET: isize>() -> IAzBizRuleContext_Vtbl {
        unsafe extern "system" fn SetBusinessRuleResult<Identity: ::windows::core::IUnknownImpl, Impl: IAzBizRuleContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bresult: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBusinessRuleResult(::core::mem::transmute_copy(&bresult)).into()
        }
        unsafe extern "system" fn SetBusinessRuleString<Identity: ::windows::core::IUnknownImpl, Impl: IAzBizRuleContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbusinessrulestring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBusinessRuleString(::core::mem::transmute_copy(&bstrbusinessrulestring)).into()
        }
        unsafe extern "system" fn BusinessRuleString<Identity: ::windows::core::IUnknownImpl, Impl: IAzBizRuleContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbusinessrulestring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BusinessRuleString() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrbusinessrulestring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParameter<Identity: ::windows::core::IUnknownImpl, Impl: IAzBizRuleContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrparametername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarparametervalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetParameter(::core::mem::transmute_copy(&bstrparametername)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarparametervalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            SetBusinessRuleResult: SetBusinessRuleResult::<Identity, Impl, OFFSET>,
            SetBusinessRuleString: SetBusinessRuleString::<Identity, Impl, OFFSET>,
            BusinessRuleString: BusinessRuleString::<Identity, Impl, OFFSET>,
            GetParameter: GetParameter::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzBizRuleContext as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzBizRuleInterfaces_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AddInterface(&mut self, bstrinterfacename: &super::super::Foundation::BSTR, linterfaceflag: i32, varinterface: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddInterfaces(&mut self, varinterfacenames: &super::super::System::Com::VARIANT, varinterfaceflags: &super::super::System::Com::VARIANT, varinterfaces: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetInterfaceValue(&mut self, bstrinterfacename: &super::super::Foundation::BSTR, linterfaceflag: *mut i32, varinterface: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Remove(&mut self, bstrinterfacename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoveAll(&mut self) -> ::windows::core::Result<()>;
    fn Count(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzBizRuleInterfaces_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>() -> IAzBizRuleInterfaces_Vtbl {
        unsafe extern "system" fn AddInterface<Identity: ::windows::core::IUnknownImpl, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinterfacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linterfaceflag: i32, varinterface: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddInterface(::core::mem::transmute_copy(&bstrinterfacename), ::core::mem::transmute_copy(&linterfaceflag), ::core::mem::transmute_copy(&varinterface)).into()
        }
        unsafe extern "system" fn AddInterfaces<Identity: ::windows::core::IUnknownImpl, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varinterfacenames: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varinterfaceflags: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varinterfaces: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddInterfaces(::core::mem::transmute_copy(&varinterfacenames), ::core::mem::transmute_copy(&varinterfaceflags), ::core::mem::transmute_copy(&varinterfaces)).into()
        }
        unsafe extern "system" fn GetInterfaceValue<Identity: ::windows::core::IUnknownImpl, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinterfacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linterfaceflag: *mut i32, varinterface: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetInterfaceValue(::core::mem::transmute_copy(&bstrinterfacename), ::core::mem::transmute_copy(&linterfaceflag), ::core::mem::transmute_copy(&varinterface)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinterfacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&bstrinterfacename)).into()
        }
        unsafe extern "system" fn RemoveAll<Identity: ::windows::core::IUnknownImpl, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveAll().into()
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddInterface: AddInterface::<Identity, Impl, OFFSET>,
            AddInterfaces: AddInterfaces::<Identity, Impl, OFFSET>,
            GetInterfaceValue: GetInterfaceValue::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            RemoveAll: RemoveAll::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzBizRuleInterfaces as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzBizRuleParameters_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AddParameter(&mut self, bstrparametername: &super::super::Foundation::BSTR, varparametervalue: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddParameters(&mut self, varparameternames: &super::super::System::Com::VARIANT, varparametervalues: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetParameterValue(&mut self, bstrparametername: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Remove(&mut self, varparametername: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoveAll(&mut self) -> ::windows::core::Result<()>;
    fn Count(&mut self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzBizRuleParameters_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>() -> IAzBizRuleParameters_Vtbl {
        unsafe extern "system" fn AddParameter<Identity: ::windows::core::IUnknownImpl, Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrparametername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varparametervalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddParameter(::core::mem::transmute_copy(&bstrparametername), ::core::mem::transmute_copy(&varparametervalue)).into()
        }
        unsafe extern "system" fn AddParameters<Identity: ::windows::core::IUnknownImpl, Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varparameternames: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varparametervalues: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddParameters(::core::mem::transmute_copy(&varparameternames), ::core::mem::transmute_copy(&varparametervalues)).into()
        }
        unsafe extern "system" fn GetParameterValue<Identity: ::windows::core::IUnknownImpl, Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrparametername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarparametervalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetParameterValue(::core::mem::transmute_copy(&bstrparametername)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarparametervalue = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl, Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varparametername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Remove(::core::mem::transmute_copy(&varparametername)).into()
        }
        unsafe extern "system" fn RemoveAll<Identity: ::windows::core::IUnknownImpl, Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).RemoveAll().into()
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddParameter: AddParameter::<Identity, Impl, OFFSET>,
            AddParameters: AddParameters::<Identity, Impl, OFFSET>,
            GetParameterValue: GetParameterValue::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            RemoveAll: RemoveAll::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzBizRuleParameters as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzClientContext_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn AccessCheck(&mut self, bstrobjectname: &super::super::Foundation::BSTR, varscopenames: &super::super::System::Com::VARIANT, varoperations: &super::super::System::Com::VARIANT, varparameternames: &super::super::System::Com::VARIANT, varparametervalues: &super::super::System::Com::VARIANT, varinterfacenames: &super::super::System::Com::VARIANT, varinterfaceflags: &super::super::System::Com::VARIANT, varinterfaces: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetBusinessRuleString(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserDn(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserSamCompat(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserDisplay(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserGuid(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserCanonical(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserUpn(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserDnsSamCompat(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetProperty(&mut self, lpropid: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetRoles(&mut self, bstrscopename: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn RoleForAccessCheck(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRoleForAccessCheck(&mut self, bstrprop: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzClientContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext_Impl, const OFFSET: isize>() -> IAzClientContext_Vtbl {
        unsafe extern "system" fn AccessCheck<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext_Impl, const OFFSET: isize>(
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
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AccessCheck(::core::mem::transmute_copy(&bstrobjectname), ::core::mem::transmute_copy(&varscopenames), ::core::mem::transmute_copy(&varoperations), ::core::mem::transmute_copy(&varparameternames), ::core::mem::transmute_copy(&varparametervalues), ::core::mem::transmute_copy(&varinterfacenames), ::core::mem::transmute_copy(&varinterfaceflags), ::core::mem::transmute_copy(&varinterfaces)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarresults = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBusinessRuleString<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbusinessrulestring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetBusinessRuleString() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrbusinessrulestring = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserDn<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UserDn() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserSamCompat<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UserSamCompat() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserDisplay<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UserDisplay() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserGuid<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UserGuid() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserCanonical<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UserCanonical() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserUpn<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UserUpn() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserDnsSamCompat<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).UserDnsSamCompat() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRoles<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarrolenames: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetRoles(::core::mem::transmute_copy(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarrolenames = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoleForAccessCheck<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RoleForAccessCheck() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoleForAccessCheck<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetRoleForAccessCheck(::core::mem::transmute_copy(&bstrprop)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            AccessCheck: AccessCheck::<Identity, Impl, OFFSET>,
            GetBusinessRuleString: GetBusinessRuleString::<Identity, Impl, OFFSET>,
            UserDn: UserDn::<Identity, Impl, OFFSET>,
            UserSamCompat: UserSamCompat::<Identity, Impl, OFFSET>,
            UserDisplay: UserDisplay::<Identity, Impl, OFFSET>,
            UserGuid: UserGuid::<Identity, Impl, OFFSET>,
            UserCanonical: UserCanonical::<Identity, Impl, OFFSET>,
            UserUpn: UserUpn::<Identity, Impl, OFFSET>,
            UserDnsSamCompat: UserDnsSamCompat::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            GetRoles: GetRoles::<Identity, Impl, OFFSET>,
            RoleForAccessCheck: RoleForAccessCheck::<Identity, Impl, OFFSET>,
            SetRoleForAccessCheck: SetRoleForAccessCheck::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzClientContext as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzClientContext2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IAzClientContext_Impl {
    fn GetAssignedScopesPage(&mut self, loptions: i32, pagesize: i32, pvarcursor: *mut super::super::System::Com::VARIANT, pvarscopenames: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddRoles(&mut self, varroles: &super::super::System::Com::VARIANT, bstrscopename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddApplicationGroups(&mut self, varapplicationgroups: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddStringSids(&mut self, varstringsids: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetLDAPQueryDN(&mut self, bstrldapquerydn: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LDAPQueryDN(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzClientContext2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext2_Impl, const OFFSET: isize>() -> IAzClientContext2_Vtbl {
        unsafe extern "system" fn GetAssignedScopesPage<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loptions: i32, pagesize: i32, pvarcursor: *mut super::super::System::Com::VARIANT, pvarscopenames: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetAssignedScopesPage(::core::mem::transmute_copy(&loptions), ::core::mem::transmute_copy(&pagesize), ::core::mem::transmute_copy(&pvarcursor), ::core::mem::transmute_copy(&pvarscopenames)).into()
        }
        unsafe extern "system" fn AddRoles<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varroles: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRoles(::core::mem::transmute_copy(&varroles), ::core::mem::transmute_copy(&bstrscopename)).into()
        }
        unsafe extern "system" fn AddApplicationGroups<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varapplicationgroups: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddApplicationGroups(::core::mem::transmute_copy(&varapplicationgroups)).into()
        }
        unsafe extern "system" fn AddStringSids<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varstringsids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddStringSids(::core::mem::transmute_copy(&varstringsids)).into()
        }
        unsafe extern "system" fn SetLDAPQueryDN<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrldapquerydn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetLDAPQueryDN(::core::mem::transmute_copy(&bstrldapquerydn)).into()
        }
        unsafe extern "system" fn LDAPQueryDN<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrldapquerydn: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).LDAPQueryDN() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrldapquerydn = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IAzClientContext_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetAssignedScopesPage: GetAssignedScopesPage::<Identity, Impl, OFFSET>,
            AddRoles: AddRoles::<Identity, Impl, OFFSET>,
            AddApplicationGroups: AddApplicationGroups::<Identity, Impl, OFFSET>,
            AddStringSids: AddStringSids::<Identity, Impl, OFFSET>,
            SetLDAPQueryDN: SetLDAPQueryDN::<Identity, Impl, OFFSET>,
            LDAPQueryDN: LDAPQueryDN::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzClientContext2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IAzClientContext as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzClientContext3_Impl: Sized + super::super::System::Com::IDispatch_Impl + IAzClientContext_Impl + IAzClientContext2_Impl {
    fn AccessCheck2(&mut self, bstrobjectname: &super::super::Foundation::BSTR, bstrscopename: &super::super::Foundation::BSTR, loperation: i32) -> ::windows::core::Result<u32>;
    fn IsInRoleAssignment(&mut self, bstrscopename: &super::super::Foundation::BSTR, bstrrolename: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn GetOperations(&mut self, bstrscopename: &super::super::Foundation::BSTR) -> ::windows::core::Result<IAzOperations>;
    fn GetTasks(&mut self, bstrscopename: &super::super::Foundation::BSTR) -> ::windows::core::Result<IAzTasks>;
    fn BizRuleParameters(&mut self) -> ::windows::core::Result<IAzBizRuleParameters>;
    fn BizRuleInterfaces(&mut self) -> ::windows::core::Result<IAzBizRuleInterfaces>;
    fn GetGroups(&mut self, bstrscopename: &super::super::Foundation::BSTR, uloptions: AZ_PROP_CONSTANTS) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Sids(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzClientContext3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext3_Impl, const OFFSET: isize>() -> IAzClientContext3_Vtbl {
        unsafe extern "system" fn AccessCheck2<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrobjectname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loperation: i32, plresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AccessCheck2(::core::mem::transmute_copy(&bstrobjectname), ::core::mem::transmute_copy(&bstrscopename), ::core::mem::transmute_copy(&loperation)) {
                ::core::result::Result::Ok(ok__) => {
                    *plresult = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInRoleAssignment<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbisinrole: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsInRoleAssignment(::core::mem::transmute_copy(&bstrscopename), ::core::mem::transmute_copy(&bstrrolename)) {
                ::core::result::Result::Ok(ok__) => {
                    *pbisinrole = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOperations<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppoperationcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetOperations(::core::mem::transmute_copy(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppoperationcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTasks<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptaskcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetTasks(::core::mem::transmute_copy(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptaskcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BizRuleParameters<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbizruleparam: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BizRuleParameters() {
                ::core::result::Result::Ok(ok__) => {
                    *ppbizruleparam = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BizRuleInterfaces<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbizruleinterfaces: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BizRuleInterfaces() {
                ::core::result::Result::Ok(ok__) => {
                    *ppbizruleinterfaces = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGroups<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, uloptions: AZ_PROP_CONSTANTS, pgrouparray: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetGroups(::core::mem::transmute_copy(&bstrscopename), ::core::mem::transmute_copy(&uloptions)) {
                ::core::result::Result::Ok(ok__) => {
                    *pgrouparray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sids<Identity: ::windows::core::IUnknownImpl, Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstringsidarray: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Sids() {
                ::core::result::Result::Ok(ok__) => {
                    *pstringsidarray = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IAzClientContext2_Vtbl::new::<Identity, Impl, OFFSET>(),
            AccessCheck2: AccessCheck2::<Identity, Impl, OFFSET>,
            IsInRoleAssignment: IsInRoleAssignment::<Identity, Impl, OFFSET>,
            GetOperations: GetOperations::<Identity, Impl, OFFSET>,
            GetTasks: GetTasks::<Identity, Impl, OFFSET>,
            BizRuleParameters: BizRuleParameters::<Identity, Impl, OFFSET>,
            BizRuleInterfaces: BizRuleInterfaces::<Identity, Impl, OFFSET>,
            GetGroups: GetGroups::<Identity, Impl, OFFSET>,
            Sids: Sids::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzClientContext3 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IAzClientContext as ::windows::core::Interface>::IID || iid == &<IAzClientContext2 as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzNameResolver_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn NameFromSid(&mut self, bstrsid: &super::super::Foundation::BSTR, psidtype: *mut i32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NamesFromSids(&mut self, vsids: &super::super::System::Com::VARIANT, pvsidtypes: *mut super::super::System::Com::VARIANT, pvnames: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzNameResolver_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzNameResolver_Impl, const OFFSET: isize>() -> IAzNameResolver_Vtbl {
        unsafe extern "system" fn NameFromSid<Identity: ::windows::core::IUnknownImpl, Impl: IAzNameResolver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psidtype: *mut i32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NameFromSid(::core::mem::transmute_copy(&bstrsid), ::core::mem::transmute_copy(&psidtype), ::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn NamesFromSids<Identity: ::windows::core::IUnknownImpl, Impl: IAzNameResolver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vsids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvsidtypes: *mut super::super::System::Com::VARIANT, pvnames: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).NamesFromSids(::core::mem::transmute_copy(&vsids), ::core::mem::transmute_copy(&pvsidtypes), ::core::mem::transmute_copy(&pvnames)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            NameFromSid: NameFromSid::<Identity, Impl, OFFSET>,
            NamesFromSids: NamesFromSids::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzNameResolver as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzObjectPicker_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetPrincipals(&mut self, hparentwnd: super::super::Foundation::HWND, bstrtitle: &super::super::Foundation::BSTR, pvsidtypes: *mut super::super::System::Com::VARIANT, pvnames: *mut super::super::System::Com::VARIANT, pvsids: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzObjectPicker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzObjectPicker_Impl, const OFFSET: isize>() -> IAzObjectPicker_Vtbl {
        unsafe extern "system" fn GetPrincipals<Identity: ::windows::core::IUnknownImpl, Impl: IAzObjectPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hparentwnd: super::super::Foundation::HWND, bstrtitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvsidtypes: *mut super::super::System::Com::VARIANT, pvnames: *mut super::super::System::Com::VARIANT, pvsids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).GetPrincipals(::core::mem::transmute_copy(&hparentwnd), ::core::mem::transmute_copy(&bstrtitle), ::core::mem::transmute_copy(&pvsidtypes), ::core::mem::transmute_copy(&pvnames), ::core::mem::transmute_copy(&pvsids)).into()
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IAzObjectPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPrincipals: GetPrincipals::<Identity, Impl, OFFSET>,
            Name: Name::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzObjectPicker as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzOperation_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ApplicationData(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetApplicationData(&mut self, bstrapplicationdata: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OperationID(&mut self) -> ::windows::core::Result<i32>;
    fn SetOperationID(&mut self, lprop: i32) -> ::windows::core::Result<()>;
    fn Writable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&mut self, lpropid: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&mut self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Submit(&mut self, lflags: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzOperation_Impl, const OFFSET: isize>() -> IAzOperation_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn ApplicationData<Identity: ::windows::core::IUnknownImpl, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ApplicationData() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrapplicationdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Identity: ::windows::core::IUnknownImpl, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetApplicationData(::core::mem::transmute_copy(&bstrapplicationdata)).into()
        }
        unsafe extern "system" fn OperationID<Identity: ::windows::core::IUnknownImpl, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OperationID() {
                ::core::result::Result::Ok(ok__) => {
                    *plprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOperationID<Identity: ::windows::core::IUnknownImpl, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetOperationID(::core::mem::transmute_copy(&lprop)).into()
        }
        unsafe extern "system" fn Writable<Identity: ::windows::core::IUnknownImpl, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Writable() {
                ::core::result::Result::Ok(ok__) => {
                    *pfprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Identity: ::windows::core::IUnknownImpl, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&varreserved)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            ApplicationData: ApplicationData::<Identity, Impl, OFFSET>,
            SetApplicationData: SetApplicationData::<Identity, Impl, OFFSET>,
            OperationID: OperationID::<Identity, Impl, OFFSET>,
            SetOperationID: SetOperationID::<Identity, Impl, OFFSET>,
            Writable: Writable::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            Submit: Submit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzOperation as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzOperation2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IAzOperation_Impl {
    fn RoleAssignments(&mut self, bstrscopename: &super::super::Foundation::BSTR, brecursive: i16) -> ::windows::core::Result<IAzRoleAssignments>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzOperation2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzOperation2_Impl, const OFFSET: isize>() -> IAzOperation2_Vtbl {
        unsafe extern "system" fn RoleAssignments<Identity: ::windows::core::IUnknownImpl, Impl: IAzOperation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, brecursive: i16, pproleassignments: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RoleAssignments(::core::mem::transmute_copy(&bstrscopename), ::core::mem::transmute_copy(&brecursive)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproleassignments = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IAzOperation_Vtbl::new::<Identity, Impl, OFFSET>(), RoleAssignments: RoleAssignments::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzOperation2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IAzOperation as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzOperations_Impl, const OFFSET: isize>() -> IAzOperations_Vtbl {
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IAzOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarobtptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IAzOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IAzOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzOperations as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzPrincipalLocator_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn NameResolver(&mut self) -> ::windows::core::Result<IAzNameResolver>;
    fn ObjectPicker(&mut self) -> ::windows::core::Result<IAzObjectPicker>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzPrincipalLocator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzPrincipalLocator_Impl, const OFFSET: isize>() -> IAzPrincipalLocator_Vtbl {
        unsafe extern "system" fn NameResolver<Identity: ::windows::core::IUnknownImpl, Impl: IAzPrincipalLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnameresolver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).NameResolver() {
                ::core::result::Result::Ok(ok__) => {
                    *ppnameresolver = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ObjectPicker<Identity: ::windows::core::IUnknownImpl, Impl: IAzPrincipalLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobjectpicker: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ObjectPicker() {
                ::core::result::Result::Ok(ok__) => {
                    *ppobjectpicker = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            NameResolver: NameResolver::<Identity, Impl, OFFSET>,
            ObjectPicker: ObjectPicker::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzPrincipalLocator as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzRole_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ApplicationData(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetApplicationData(&mut self, bstrapplicationdata: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddAppMember(&mut self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteAppMember(&mut self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddTask(&mut self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteTask(&mut self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddOperation(&mut self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteOperation(&mut self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddMember(&mut self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteMember(&mut self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Writable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&mut self, lpropid: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&mut self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AppMembers(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Members(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Operations(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Tasks(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddPropertyItem(&mut self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePropertyItem(&mut self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Submit(&mut self, lflags: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddMemberName(&mut self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteMemberName(&mut self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MembersName(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzRole_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>() -> IAzRole_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn ApplicationData<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ApplicationData() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrapplicationdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetApplicationData(::core::mem::transmute_copy(&bstrapplicationdata)).into()
        }
        unsafe extern "system" fn AddAppMember<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddAppMember(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteAppMember<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteAppMember(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddTask<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddTask(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteTask<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteTask(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddOperation<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddOperation(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteOperation<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteOperation(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddMember<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddMember(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteMember<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteMember(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Writable<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Writable() {
                ::core::result::Result::Ok(ok__) => {
                    *pfprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AppMembers<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).AppMembers() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Members<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Members() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Operations<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Operations() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tasks<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Tasks() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPropertyItem<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeletePropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddMemberName<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddMemberName(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteMemberName<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteMemberName(::core::mem::transmute_copy(&bstrprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn MembersName<Identity: ::windows::core::IUnknownImpl, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).MembersName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            ApplicationData: ApplicationData::<Identity, Impl, OFFSET>,
            SetApplicationData: SetApplicationData::<Identity, Impl, OFFSET>,
            AddAppMember: AddAppMember::<Identity, Impl, OFFSET>,
            DeleteAppMember: DeleteAppMember::<Identity, Impl, OFFSET>,
            AddTask: AddTask::<Identity, Impl, OFFSET>,
            DeleteTask: DeleteTask::<Identity, Impl, OFFSET>,
            AddOperation: AddOperation::<Identity, Impl, OFFSET>,
            DeleteOperation: DeleteOperation::<Identity, Impl, OFFSET>,
            AddMember: AddMember::<Identity, Impl, OFFSET>,
            DeleteMember: DeleteMember::<Identity, Impl, OFFSET>,
            Writable: Writable::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            AppMembers: AppMembers::<Identity, Impl, OFFSET>,
            Members: Members::<Identity, Impl, OFFSET>,
            Operations: Operations::<Identity, Impl, OFFSET>,
            Tasks: Tasks::<Identity, Impl, OFFSET>,
            AddPropertyItem: AddPropertyItem::<Identity, Impl, OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Identity, Impl, OFFSET>,
            Submit: Submit::<Identity, Impl, OFFSET>,
            AddMemberName: AddMemberName::<Identity, Impl, OFFSET>,
            DeleteMemberName: DeleteMemberName::<Identity, Impl, OFFSET>,
            MembersName: MembersName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzRole as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzRoleAssignment_Impl: Sized + super::super::System::Com::IDispatch_Impl + IAzRole_Impl {
    fn AddRoleDefinition(&mut self, bstrroledefinition: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DeleteRoleDefinition(&mut self, bstrroledefinition: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RoleDefinitions(&mut self) -> ::windows::core::Result<IAzRoleDefinitions>;
    fn Scope(&mut self) -> ::windows::core::Result<IAzScope>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzRoleAssignment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleAssignment_Impl, const OFFSET: isize>() -> IAzRoleAssignment_Vtbl {
        unsafe extern "system" fn AddRoleDefinition<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinition: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRoleDefinition(::core::mem::transmute_copy(&bstrroledefinition)).into()
        }
        unsafe extern "system" fn DeleteRoleDefinition<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinition: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteRoleDefinition(::core::mem::transmute_copy(&bstrroledefinition)).into()
        }
        unsafe extern "system" fn RoleDefinitions<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RoleDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    *pproledefinitions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scope<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppscope: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Scope() {
                ::core::result::Result::Ok(ok__) => {
                    *ppscope = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IAzRole_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddRoleDefinition: AddRoleDefinition::<Identity, Impl, OFFSET>,
            DeleteRoleDefinition: DeleteRoleDefinition::<Identity, Impl, OFFSET>,
            RoleDefinitions: RoleDefinitions::<Identity, Impl, OFFSET>,
            Scope: Scope::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzRoleAssignment as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IAzRole as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleAssignments_Impl, const OFFSET: isize>() -> IAzRoleAssignments_Vtbl {
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleAssignments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarobtptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleAssignments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleAssignments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzRoleAssignments as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzRoleDefinition_Impl: Sized + super::super::System::Com::IDispatch_Impl + IAzTask_Impl {
    fn RoleAssignments(&mut self, bstrscopename: &super::super::Foundation::BSTR, brecursive: i16) -> ::windows::core::Result<IAzRoleAssignments>;
    fn AddRoleDefinition(&mut self, bstrroledefinition: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DeleteRoleDefinition(&mut self, bstrroledefinition: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RoleDefinitions(&mut self) -> ::windows::core::Result<IAzRoleDefinitions>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzRoleDefinition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleDefinition_Impl, const OFFSET: isize>() -> IAzRoleDefinition_Vtbl {
        unsafe extern "system" fn RoleAssignments<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, brecursive: i16, pproleassignments: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RoleAssignments(::core::mem::transmute_copy(&bstrscopename), ::core::mem::transmute_copy(&brecursive)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproleassignments = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRoleDefinition<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinition: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddRoleDefinition(::core::mem::transmute_copy(&bstrroledefinition)).into()
        }
        unsafe extern "system" fn DeleteRoleDefinition<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinition: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteRoleDefinition(::core::mem::transmute_copy(&bstrroledefinition)).into()
        }
        unsafe extern "system" fn RoleDefinitions<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RoleDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    *pproledefinitions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: IAzTask_Vtbl::new::<Identity, Impl, OFFSET>(),
            RoleAssignments: RoleAssignments::<Identity, Impl, OFFSET>,
            AddRoleDefinition: AddRoleDefinition::<Identity, Impl, OFFSET>,
            DeleteRoleDefinition: DeleteRoleDefinition::<Identity, Impl, OFFSET>,
            RoleDefinitions: RoleDefinitions::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzRoleDefinition as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IAzTask as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleDefinitions_Impl, const OFFSET: isize>() -> IAzRoleDefinitions_Vtbl {
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleDefinitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarobtptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleDefinitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoleDefinitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzRoleDefinitions as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoles_Impl, const OFFSET: isize>() -> IAzRoles_Vtbl {
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarobtptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IAzRoles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzRoles as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzScope_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ApplicationData(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetApplicationData(&mut self, bstrapplicationdata: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Writable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&mut self, lpropid: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&mut self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPropertyItem(&mut self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePropertyItem(&mut self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PolicyAdministrators(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PolicyReaders(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddPolicyAdministrator(&mut self, bstradmin: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyAdministrator(&mut self, bstradmin: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPolicyReader(&mut self, bstrreader: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyReader(&mut self, bstrreader: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ApplicationGroups(&mut self) -> ::windows::core::Result<IAzApplicationGroups>;
    fn OpenApplicationGroup(&mut self, bstrgroupname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>;
    fn CreateApplicationGroup(&mut self, bstrgroupname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>;
    fn DeleteApplicationGroup(&mut self, bstrgroupname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Roles(&mut self) -> ::windows::core::Result<IAzRoles>;
    fn OpenRole(&mut self, bstrrolename: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzRole>;
    fn CreateRole(&mut self, bstrrolename: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzRole>;
    fn DeleteRole(&mut self, bstrrolename: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Tasks(&mut self) -> ::windows::core::Result<IAzTasks>;
    fn OpenTask(&mut self, bstrtaskname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzTask>;
    fn CreateTask(&mut self, bstrtaskname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzTask>;
    fn DeleteTask(&mut self, bstrtaskname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Submit(&mut self, lflags: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn CanBeDelegated(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn BizrulesWritable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn PolicyAdministratorsName(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PolicyReadersName(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddPolicyAdministratorName(&mut self, bstradmin: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyAdministratorName(&mut self, bstradmin: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPolicyReaderName(&mut self, bstrreader: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyReaderName(&mut self, bstrreader: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzScope_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>() -> IAzScope_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn ApplicationData<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ApplicationData() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrapplicationdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetApplicationData(::core::mem::transmute_copy(&bstrapplicationdata)).into()
        }
        unsafe extern "system" fn Writable<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Writable() {
                ::core::result::Result::Ok(ok__) => {
                    *pfprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddPropertyItem<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeletePropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn PolicyAdministrators<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PolicyAdministrators() {
                ::core::result::Result::Ok(ok__) => {
                    *pvaradmins = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReaders<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PolicyReaders() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarreaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministrator<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPolicyAdministrator(::core::mem::transmute_copy(&bstradmin), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyAdministrator<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeletePolicyAdministrator(::core::mem::transmute_copy(&bstradmin), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddPolicyReader<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPolicyReader(::core::mem::transmute_copy(&bstrreader), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyReader<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeletePolicyReader(::core::mem::transmute_copy(&bstrreader), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn ApplicationGroups<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroupcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ApplicationGroups() {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroupcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenApplicationGroup<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenApplicationGroup(::core::mem::transmute_copy(&bstrgroupname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateApplicationGroup<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateApplicationGroup(::core::mem::transmute_copy(&bstrgroupname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *ppgroup = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteApplicationGroup<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteApplicationGroup(::core::mem::transmute_copy(&bstrgroupname), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Roles<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprolecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Roles() {
                ::core::result::Result::Ok(ok__) => {
                    *pprolecollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRole<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pprole: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenRole(::core::mem::transmute_copy(&bstrrolename), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprole = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRole<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pprole: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateRole(::core::mem::transmute_copy(&bstrrolename), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pprole = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRole<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteRole(::core::mem::transmute_copy(&bstrrolename), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Tasks<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptaskcollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Tasks() {
                ::core::result::Result::Ok(ok__) => {
                    *pptaskcollection = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTask<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pptask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenTask(::core::mem::transmute_copy(&bstrtaskname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTask<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pptask: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateTask(::core::mem::transmute_copy(&bstrtaskname), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pptask = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteTask<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteTask(::core::mem::transmute_copy(&bstrtaskname), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn CanBeDelegated<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CanBeDelegated() {
                ::core::result::Result::Ok(ok__) => {
                    *pfprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BizrulesWritable<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BizrulesWritable() {
                ::core::result::Result::Ok(ok__) => {
                    *pfprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyAdministratorsName<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PolicyAdministratorsName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvaradmins = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReadersName<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).PolicyReadersName() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarreaders = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministratorName<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPolicyAdministratorName(::core::mem::transmute_copy(&bstradmin), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyAdministratorName<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeletePolicyAdministratorName(::core::mem::transmute_copy(&bstradmin), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddPolicyReaderName<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPolicyReaderName(::core::mem::transmute_copy(&bstrreader), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyReaderName<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeletePolicyReaderName(::core::mem::transmute_copy(&bstrreader), ::core::mem::transmute_copy(&varreserved)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            ApplicationData: ApplicationData::<Identity, Impl, OFFSET>,
            SetApplicationData: SetApplicationData::<Identity, Impl, OFFSET>,
            Writable: Writable::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            AddPropertyItem: AddPropertyItem::<Identity, Impl, OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Identity, Impl, OFFSET>,
            PolicyAdministrators: PolicyAdministrators::<Identity, Impl, OFFSET>,
            PolicyReaders: PolicyReaders::<Identity, Impl, OFFSET>,
            AddPolicyAdministrator: AddPolicyAdministrator::<Identity, Impl, OFFSET>,
            DeletePolicyAdministrator: DeletePolicyAdministrator::<Identity, Impl, OFFSET>,
            AddPolicyReader: AddPolicyReader::<Identity, Impl, OFFSET>,
            DeletePolicyReader: DeletePolicyReader::<Identity, Impl, OFFSET>,
            ApplicationGroups: ApplicationGroups::<Identity, Impl, OFFSET>,
            OpenApplicationGroup: OpenApplicationGroup::<Identity, Impl, OFFSET>,
            CreateApplicationGroup: CreateApplicationGroup::<Identity, Impl, OFFSET>,
            DeleteApplicationGroup: DeleteApplicationGroup::<Identity, Impl, OFFSET>,
            Roles: Roles::<Identity, Impl, OFFSET>,
            OpenRole: OpenRole::<Identity, Impl, OFFSET>,
            CreateRole: CreateRole::<Identity, Impl, OFFSET>,
            DeleteRole: DeleteRole::<Identity, Impl, OFFSET>,
            Tasks: Tasks::<Identity, Impl, OFFSET>,
            OpenTask: OpenTask::<Identity, Impl, OFFSET>,
            CreateTask: CreateTask::<Identity, Impl, OFFSET>,
            DeleteTask: DeleteTask::<Identity, Impl, OFFSET>,
            Submit: Submit::<Identity, Impl, OFFSET>,
            CanBeDelegated: CanBeDelegated::<Identity, Impl, OFFSET>,
            BizrulesWritable: BizrulesWritable::<Identity, Impl, OFFSET>,
            PolicyAdministratorsName: PolicyAdministratorsName::<Identity, Impl, OFFSET>,
            PolicyReadersName: PolicyReadersName::<Identity, Impl, OFFSET>,
            AddPolicyAdministratorName: AddPolicyAdministratorName::<Identity, Impl, OFFSET>,
            DeletePolicyAdministratorName: DeletePolicyAdministratorName::<Identity, Impl, OFFSET>,
            AddPolicyReaderName: AddPolicyReaderName::<Identity, Impl, OFFSET>,
            DeletePolicyReaderName: DeletePolicyReaderName::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzScope as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzScope2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IAzScope_Impl {
    fn RoleDefinitions(&mut self) -> ::windows::core::Result<IAzRoleDefinitions>;
    fn CreateRoleDefinition(&mut self, bstrroledefinitionname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IAzRoleDefinition>;
    fn OpenRoleDefinition(&mut self, bstrroledefinitionname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IAzRoleDefinition>;
    fn DeleteRoleDefinition(&mut self, bstrroledefinitionname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RoleAssignments(&mut self) -> ::windows::core::Result<IAzRoleAssignments>;
    fn CreateRoleAssignment(&mut self, bstrroleassignmentname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IAzRoleAssignment>;
    fn OpenRoleAssignment(&mut self, bstrroleassignmentname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IAzRoleAssignment>;
    fn DeleteRoleAssignment(&mut self, bstrroleassignmentname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzScope2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope2_Impl, const OFFSET: isize>() -> IAzScope2_Vtbl {
        unsafe extern "system" fn RoleDefinitions<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RoleDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    *pproledefinitions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRoleDefinition<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateRoleDefinition(::core::mem::transmute_copy(&bstrroledefinitionname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproledefinitions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRoleDefinition<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproledefinitions: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenRoleDefinition(::core::mem::transmute_copy(&bstrroledefinitionname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproledefinitions = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRoleDefinition<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteRoleDefinition(::core::mem::transmute_copy(&bstrroledefinitionname)).into()
        }
        unsafe extern "system" fn RoleAssignments<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproleassignments: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RoleAssignments() {
                ::core::result::Result::Ok(ok__) => {
                    *pproleassignments = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRoleAssignment<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproleassignment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).CreateRoleAssignment(::core::mem::transmute_copy(&bstrroleassignmentname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproleassignment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRoleAssignment<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproleassignment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).OpenRoleAssignment(::core::mem::transmute_copy(&bstrroleassignmentname)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproleassignment = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRoleAssignment<Identity: ::windows::core::IUnknownImpl, Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteRoleAssignment(::core::mem::transmute_copy(&bstrroleassignmentname)).into()
        }
        Self {
            base: IAzScope_Vtbl::new::<Identity, Impl, OFFSET>(),
            RoleDefinitions: RoleDefinitions::<Identity, Impl, OFFSET>,
            CreateRoleDefinition: CreateRoleDefinition::<Identity, Impl, OFFSET>,
            OpenRoleDefinition: OpenRoleDefinition::<Identity, Impl, OFFSET>,
            DeleteRoleDefinition: DeleteRoleDefinition::<Identity, Impl, OFFSET>,
            RoleAssignments: RoleAssignments::<Identity, Impl, OFFSET>,
            CreateRoleAssignment: CreateRoleAssignment::<Identity, Impl, OFFSET>,
            OpenRoleAssignment: OpenRoleAssignment::<Identity, Impl, OFFSET>,
            DeleteRoleAssignment: DeleteRoleAssignment::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzScope2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IAzScope as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzScopes_Impl, const OFFSET: isize>() -> IAzScopes_Vtbl {
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IAzScopes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarobtptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IAzScopes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IAzScopes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzScopes as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzTask_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&mut self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&mut self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ApplicationData(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetApplicationData(&mut self, bstrapplicationdata: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BizRule(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetBizRule(&mut self, bstrprop: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BizRuleLanguage(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetBizRuleLanguage(&mut self, bstrprop: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BizRuleImportedPath(&mut self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetBizRuleImportedPath(&mut self, bstrprop: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsRoleDefinition(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetIsRoleDefinition(&mut self, fprop: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Operations(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Tasks(&mut self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddOperation(&mut self, bstrop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteOperation(&mut self, bstrop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddTask(&mut self, bstrtask: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteTask(&mut self, bstrtask: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Writable(&mut self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&mut self, lpropid: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&mut self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPropertyItem(&mut self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePropertyItem(&mut self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Submit(&mut self, lflags: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzTask_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>() -> IAzTask_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Name() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrname = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetName(::core::mem::transmute_copy(&bstrname)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Description() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrdescription = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetDescription(::core::mem::transmute_copy(&bstrdescription)).into()
        }
        unsafe extern "system" fn ApplicationData<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).ApplicationData() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrapplicationdata = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetApplicationData(::core::mem::transmute_copy(&bstrapplicationdata)).into()
        }
        unsafe extern "system" fn BizRule<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BizRule() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRule<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBizRule(::core::mem::transmute_copy(&bstrprop)).into()
        }
        unsafe extern "system" fn BizRuleLanguage<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BizRuleLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRuleLanguage<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBizRuleLanguage(::core::mem::transmute_copy(&bstrprop)).into()
        }
        unsafe extern "system" fn BizRuleImportedPath<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).BizRuleImportedPath() {
                ::core::result::Result::Ok(ok__) => {
                    *pbstrprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRuleImportedPath<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetBizRuleImportedPath(::core::mem::transmute_copy(&bstrprop)).into()
        }
        unsafe extern "system" fn IsRoleDefinition<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).IsRoleDefinition() {
                ::core::result::Result::Ok(ok__) => {
                    *pfprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRoleDefinition<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fprop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetIsRoleDefinition(::core::mem::transmute_copy(&fprop)).into()
        }
        unsafe extern "system" fn Operations<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Operations() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tasks<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Tasks() {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddOperation<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddOperation(::core::mem::transmute_copy(&bstrop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteOperation<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteOperation(::core::mem::transmute_copy(&bstrop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddTask<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddTask(::core::mem::transmute_copy(&bstrtask), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteTask<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeleteTask(::core::mem::transmute_copy(&bstrtask), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Writable<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Writable() {
                ::core::result::Result::Ok(ok__) => {
                    *pfprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarprop = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn AddPropertyItem<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).AddPropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).DeletePropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute_copy(&varprop), ::core::mem::transmute_copy(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            (*this).Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute_copy(&varreserved)).into()
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Name: Name::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            Description: Description::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            ApplicationData: ApplicationData::<Identity, Impl, OFFSET>,
            SetApplicationData: SetApplicationData::<Identity, Impl, OFFSET>,
            BizRule: BizRule::<Identity, Impl, OFFSET>,
            SetBizRule: SetBizRule::<Identity, Impl, OFFSET>,
            BizRuleLanguage: BizRuleLanguage::<Identity, Impl, OFFSET>,
            SetBizRuleLanguage: SetBizRuleLanguage::<Identity, Impl, OFFSET>,
            BizRuleImportedPath: BizRuleImportedPath::<Identity, Impl, OFFSET>,
            SetBizRuleImportedPath: SetBizRuleImportedPath::<Identity, Impl, OFFSET>,
            IsRoleDefinition: IsRoleDefinition::<Identity, Impl, OFFSET>,
            SetIsRoleDefinition: SetIsRoleDefinition::<Identity, Impl, OFFSET>,
            Operations: Operations::<Identity, Impl, OFFSET>,
            Tasks: Tasks::<Identity, Impl, OFFSET>,
            AddOperation: AddOperation::<Identity, Impl, OFFSET>,
            DeleteOperation: DeleteOperation::<Identity, Impl, OFFSET>,
            AddTask: AddTask::<Identity, Impl, OFFSET>,
            DeleteTask: DeleteTask::<Identity, Impl, OFFSET>,
            Writable: Writable::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            SetProperty: SetProperty::<Identity, Impl, OFFSET>,
            AddPropertyItem: AddPropertyItem::<Identity, Impl, OFFSET>,
            DeletePropertyItem: DeletePropertyItem::<Identity, Impl, OFFSET>,
            Submit: Submit::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzTask as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzTask2_Impl: Sized + super::super::System::Com::IDispatch_Impl + IAzTask_Impl {
    fn RoleAssignments(&mut self, bstrscopename: &super::super::Foundation::BSTR, brecursive: i16) -> ::windows::core::Result<IAzRoleAssignments>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzTask2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask2_Impl, const OFFSET: isize>() -> IAzTask2_Vtbl {
        unsafe extern "system" fn RoleAssignments<Identity: ::windows::core::IUnknownImpl, Impl: IAzTask2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, brecursive: i16, pproleassignments: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).RoleAssignments(::core::mem::transmute_copy(&bstrscopename), ::core::mem::transmute_copy(&brecursive)) {
                ::core::result::Result::Ok(ok__) => {
                    *pproleassignments = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: IAzTask_Vtbl::new::<Identity, Impl, OFFSET>(), RoleAssignments: RoleAssignments::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzTask2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IAzTask as ::windows::core::Interface>::IID
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
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IAzTasks_Impl, const OFFSET: isize>() -> IAzTasks_Vtbl {
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl, Impl: IAzTasks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    *pvarobtptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl, Impl: IAzTasks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this).Count() {
                ::core::result::Result::Ok(ok__) => {
                    *plcount = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl, Impl: IAzTasks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Identity;
            let this = (*this).get_impl() as *mut Impl;
            match (*this)._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    *ppenumptr = ::core::mem::transmute(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Item: Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzTasks as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
