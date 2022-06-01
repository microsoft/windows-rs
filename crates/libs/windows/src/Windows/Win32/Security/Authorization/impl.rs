#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzApplication_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ApplicationData(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetApplicationData(&self, bstrapplicationdata: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AuthzInterfaceClsid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetAuthzInterfaceClsid(&self, bstrprop: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Version(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetVersion(&self, bstrprop: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GenerateAudits(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetGenerateAudits(&self, bprop: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn ApplyStoreSacl(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetApplyStoreSacl(&self, bprop: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Writable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&self, lpropid: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PolicyAdministrators(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PolicyReaders(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddPolicyAdministrator(&self, bstradmin: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyAdministrator(&self, bstradmin: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPolicyReader(&self, bstrreader: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyReader(&self, bstrreader: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Scopes(&self) -> ::windows::core::Result<IAzScopes>;
    fn OpenScope(&self, bstrscopename: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzScope>;
    fn CreateScope(&self, bstrscopename: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzScope>;
    fn DeleteScope(&self, bstrscopename: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Operations(&self) -> ::windows::core::Result<IAzOperations>;
    fn OpenOperation(&self, bstroperationname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzOperation>;
    fn CreateOperation(&self, bstroperationname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzOperation>;
    fn DeleteOperation(&self, bstroperationname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Tasks(&self) -> ::windows::core::Result<IAzTasks>;
    fn OpenTask(&self, bstrtaskname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzTask>;
    fn CreateTask(&self, bstrtaskname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzTask>;
    fn DeleteTask(&self, bstrtaskname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ApplicationGroups(&self) -> ::windows::core::Result<IAzApplicationGroups>;
    fn OpenApplicationGroup(&self, bstrgroupname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>;
    fn CreateApplicationGroup(&self, bstrgroupname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>;
    fn DeleteApplicationGroup(&self, bstrgroupname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Roles(&self) -> ::windows::core::Result<IAzRoles>;
    fn OpenRole(&self, bstrrolename: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzRole>;
    fn CreateRole(&self, bstrrolename: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzRole>;
    fn DeleteRole(&self, bstrrolename: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn InitializeClientContextFromToken(&self, ulltokenhandle: u64, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext>;
    fn AddPropertyItem(&self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePropertyItem(&self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Submit(&self, lflags: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn InitializeClientContextFromName(&self, clientname: &super::super::Foundation::BSTR, domainname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext>;
    fn DelegatedPolicyUsers(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddDelegatedPolicyUser(&self, bstrdelegatedpolicyuser: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteDelegatedPolicyUser(&self, bstrdelegatedpolicyuser: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn InitializeClientContextFromStringSid(&self, sidstring: &super::super::Foundation::BSTR, loptions: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext>;
    fn PolicyAdministratorsName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PolicyReadersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddPolicyAdministratorName(&self, bstradmin: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyAdministratorName(&self, bstradmin: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPolicyReaderName(&self, bstrreader: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyReaderName(&self, bstrreader: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DelegatedPolicyUsersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddDelegatedPolicyUserName(&self, bstrdelegatedpolicyuser: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteDelegatedPolicyUserName(&self, bstrdelegatedpolicyuser: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzApplication {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzApplication_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>() -> IAzApplication_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn ApplicationData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ApplicationData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrapplicationdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetApplicationData(::core::mem::transmute(&bstrapplicationdata)).into()
        }
        unsafe extern "system" fn AuthzInterfaceClsid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AuthzInterfaceClsid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetAuthzInterfaceClsid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetAuthzInterfaceClsid(::core::mem::transmute(&bstrprop)).into()
        }
        unsafe extern "system" fn Version<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Version() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetVersion(::core::mem::transmute(&bstrprop)).into()
        }
        unsafe extern "system" fn GenerateAudits<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GenerateAudits() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGenerateAudits<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bprop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGenerateAudits(::core::mem::transmute_copy(&bprop)).into()
        }
        unsafe extern "system" fn ApplyStoreSacl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ApplyStoreSacl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplyStoreSacl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bprop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetApplyStoreSacl(::core::mem::transmute_copy(&bprop)).into()
        }
        unsafe extern "system" fn Writable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Writable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn PolicyAdministrators<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PolicyAdministrators() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvaradmins, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReaders<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PolicyReaders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarreaders, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministrator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPolicyAdministrator(::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyAdministrator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePolicyAdministrator(::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddPolicyReader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPolicyReader(::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyReader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePolicyReader(::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Scopes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppscopecollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Scopes() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscopecollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenScope<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppscope: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenScope(::core::mem::transmute(&bstrscopename), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscope, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScope<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppscope: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateScope(::core::mem::transmute(&bstrscopename), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscope, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteScope<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteScope(::core::mem::transmute(&bstrscopename), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Operations<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppoperationcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Operations() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoperationcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenOperation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroperationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenOperation(::core::mem::transmute(&bstroperationname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoperation, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateOperation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroperationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppoperation: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateOperation(::core::mem::transmute(&bstroperationname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoperation, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteOperation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstroperationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteOperation(::core::mem::transmute(&bstroperationname), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Tasks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptaskcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Tasks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptaskcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pptask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenTask(::core::mem::transmute(&bstrtaskname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptask, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pptask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTask(::core::mem::transmute(&bstrtaskname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptask, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteTask(::core::mem::transmute(&bstrtaskname), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn ApplicationGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroupcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ApplicationGroups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroupcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenApplicationGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenApplicationGroup(::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateApplicationGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateApplicationGroup(::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteApplicationGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteApplicationGroup(::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Roles<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprolecollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Roles() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprolecollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRole<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pprole: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenRole(::core::mem::transmute(&bstrrolename), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprole, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRole<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pprole: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRole(::core::mem::transmute(&bstrrolename), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprole, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRole<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteRole(::core::mem::transmute(&bstrrolename), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn InitializeClientContextFromToken<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ulltokenhandle: u64, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclientcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InitializeClientContextFromToken(::core::mem::transmute_copy(&ulltokenhandle), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclientcontext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPropertyItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn InitializeClientContextFromName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, clientname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, domainname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclientcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InitializeClientContextFromName(::core::mem::transmute(&clientname), ::core::mem::transmute(&domainname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclientcontext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DelegatedPolicyUsers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DelegatedPolicyUsers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvardelegatedpolicyusers, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDelegatedPolicyUser<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddDelegatedPolicyUser(::core::mem::transmute(&bstrdelegatedpolicyuser), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUser<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteDelegatedPolicyUser(::core::mem::transmute(&bstrdelegatedpolicyuser), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn InitializeClientContextFromStringSid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sidstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loptions: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclientcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InitializeClientContextFromStringSid(::core::mem::transmute(&sidstring), ::core::mem::transmute_copy(&loptions), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclientcontext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyAdministratorsName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PolicyAdministratorsName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvaradmins, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReadersName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PolicyReadersName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarreaders, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministratorName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPolicyAdministratorName(::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyAdministratorName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePolicyAdministratorName(::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddPolicyReaderName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPolicyReaderName(::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyReaderName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePolicyReaderName(::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DelegatedPolicyUsersName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DelegatedPolicyUsersName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvardelegatedpolicyusers, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDelegatedPolicyUserName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddDelegatedPolicyUserName(::core::mem::transmute(&bstrdelegatedpolicyuser), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUserName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteDelegatedPolicyUserName(::core::mem::transmute(&bstrdelegatedpolicyuser), ::core::mem::transmute(&varreserved)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn InitializeClientContextFromToken2(&self, ultokenhandlelowpart: u32, ultokenhandlehighpart: u32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext2>;
    fn InitializeClientContext2(&self, identifyingstring: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzClientContext2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzApplication2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzApplication2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication2_Impl, const OFFSET: isize>() -> IAzApplication2_Vtbl {
        unsafe extern "system" fn InitializeClientContextFromToken2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ultokenhandlelowpart: u32, ultokenhandlehighpart: u32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclientcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InitializeClientContextFromToken2(::core::mem::transmute_copy(&ultokenhandlelowpart), ::core::mem::transmute_copy(&ultokenhandlehighpart), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclientcontext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn InitializeClientContext2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, identifyingstring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppclientcontext: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.InitializeClientContext2(::core::mem::transmute(&identifyingstring), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppclientcontext, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IAzApplication_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn ScopeExists(&self, bstrscopename: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn OpenScope2(&self, bstrscopename: &super::super::Foundation::BSTR) -> ::windows::core::Result<IAzScope2>;
    fn CreateScope2(&self, bstrscopename: &super::super::Foundation::BSTR) -> ::windows::core::Result<IAzScope2>;
    fn DeleteScope2(&self, bstrscopename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RoleDefinitions(&self) -> ::windows::core::Result<IAzRoleDefinitions>;
    fn CreateRoleDefinition(&self, bstrroledefinitionname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IAzRoleDefinition>;
    fn OpenRoleDefinition(&self, bstrroledefinitionname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IAzRoleDefinition>;
    fn DeleteRoleDefinition(&self, bstrroledefinitionname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RoleAssignments(&self) -> ::windows::core::Result<IAzRoleAssignments>;
    fn CreateRoleAssignment(&self, bstrroleassignmentname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IAzRoleAssignment>;
    fn OpenRoleAssignment(&self, bstrroleassignmentname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IAzRoleAssignment>;
    fn DeleteRoleAssignment(&self, bstrroleassignmentname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BizRulesEnabled(&self) -> ::windows::core::Result<i16>;
    fn SetBizRulesEnabled(&self, benabled: i16) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzApplication3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzApplication3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>() -> IAzApplication3_Vtbl {
        unsafe extern "system" fn ScopeExists<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbexist: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ScopeExists(::core::mem::transmute(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbexist, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenScope2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppscope2: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenScope2(::core::mem::transmute(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscope2, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateScope2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppscope2: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateScope2(::core::mem::transmute(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscope2, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteScope2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteScope2(::core::mem::transmute(&bstrscopename)).into()
        }
        unsafe extern "system" fn RoleDefinitions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoleDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproledefinitions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRoleDefinition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRoleDefinition(::core::mem::transmute(&bstrroledefinitionname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproledefinitions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRoleDefinition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenRoleDefinition(::core::mem::transmute(&bstrroledefinitionname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproledefinitions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRoleDefinition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteRoleDefinition(::core::mem::transmute(&bstrroledefinitionname)).into()
        }
        unsafe extern "system" fn RoleAssignments<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoleAssignments() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignments, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRoleAssignment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproleassignment: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRoleAssignment(::core::mem::transmute(&bstrroleassignmentname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignment, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRoleAssignment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproleassignment: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenRoleAssignment(::core::mem::transmute(&bstrroleassignmentname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignment, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRoleAssignment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteRoleAssignment(::core::mem::transmute(&bstrroleassignmentname)).into()
        }
        unsafe extern "system" fn BizRulesEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbenabled: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BizRulesEnabled() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbenabled, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRulesEnabled<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplication3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, benabled: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBizRulesEnabled(::core::mem::transmute_copy(&benabled)).into()
        }
        Self {
            base__: IAzApplication2_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Type(&self) -> ::windows::core::Result<i32>;
    fn SetType(&self, lprop: i32) -> ::windows::core::Result<()>;
    fn LdapQuery(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetLdapQuery(&self, bstrprop: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AppMembers(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AppNonMembers(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Members(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn NonMembers(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddAppMember(&self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteAppMember(&self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddAppNonMember(&self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteAppNonMember(&self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddMember(&self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteMember(&self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddNonMember(&self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteNonMember(&self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Writable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&self, lpropid: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPropertyItem(&self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePropertyItem(&self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Submit(&self, lflags: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddMemberName(&self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteMemberName(&self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddNonMemberName(&self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteNonMemberName(&self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MembersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn NonMembersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzApplicationGroup {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzApplicationGroup_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>() -> IAzApplicationGroup_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn Type<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Type() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetType(::core::mem::transmute_copy(&lprop)).into()
        }
        unsafe extern "system" fn LdapQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LdapQuery() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetLdapQuery<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLdapQuery(::core::mem::transmute(&bstrprop)).into()
        }
        unsafe extern "system" fn AppMembers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AppMembers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AppNonMembers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AppNonMembers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Members<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Members() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NonMembers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NonMembers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn AddAppMember<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddAppMember(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteAppMember<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteAppMember(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddAppNonMember<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddAppNonMember(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteAppNonMember<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteAppNonMember(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddMember<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddMember(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteMember<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteMember(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddNonMember<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddNonMember(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteNonMember<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteNonMember(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Writable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Writable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddPropertyItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddMemberName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddMemberName(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteMemberName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteMemberName(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddNonMemberName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddNonMemberName(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteNonMemberName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteNonMemberName(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn MembersName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MembersName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn NonMembersName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NonMembersName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn BizRule(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetBizRule(&self, bstrprop: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BizRuleLanguage(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetBizRuleLanguage(&self, bstrprop: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BizRuleImportedPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetBizRuleImportedPath(&self, bstrprop: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RoleAssignments(&self, bstrscopename: &super::super::Foundation::BSTR, brecursive: i16) -> ::windows::core::Result<IAzRoleAssignments>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzApplicationGroup2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzApplicationGroup2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>() -> IAzApplicationGroup2_Vtbl {
        unsafe extern "system" fn BizRule<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BizRule() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRule<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBizRule(::core::mem::transmute(&bstrprop)).into()
        }
        unsafe extern "system" fn BizRuleLanguage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BizRuleLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRuleLanguage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBizRuleLanguage(::core::mem::transmute(&bstrprop)).into()
        }
        unsafe extern "system" fn BizRuleImportedPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BizRuleImportedPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRuleImportedPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBizRuleImportedPath(::core::mem::transmute(&bstrprop)).into()
        }
        unsafe extern "system" fn RoleAssignments<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroup2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, brecursive: i16, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoleAssignments(::core::mem::transmute(&bstrscopename), ::core::mem::transmute_copy(&brecursive)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignments, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IAzApplicationGroup_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn get_Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzApplicationGroups {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzApplicationGroups_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroups_Impl, const OFFSET: isize>() -> IAzApplicationGroups_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarobtptr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplicationGroups_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumptr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
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
    fn get_Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzApplications {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzApplications_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplications_Impl, const OFFSET: isize>() -> IAzApplications_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarobtptr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzApplications_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumptr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
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
    fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ApplicationData(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetApplicationData(&self, bstrapplicationdata: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DomainTimeout(&self) -> ::windows::core::Result<i32>;
    fn SetDomainTimeout(&self, lprop: i32) -> ::windows::core::Result<()>;
    fn ScriptEngineTimeout(&self) -> ::windows::core::Result<i32>;
    fn SetScriptEngineTimeout(&self, lprop: i32) -> ::windows::core::Result<()>;
    fn MaxScriptEngines(&self) -> ::windows::core::Result<i32>;
    fn SetMaxScriptEngines(&self, lprop: i32) -> ::windows::core::Result<()>;
    fn GenerateAudits(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetGenerateAudits(&self, bprop: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Writable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&self, lpropid: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPropertyItem(&self, lpropid: AZ_PROP_CONSTANTS, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePropertyItem(&self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PolicyAdministrators(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PolicyReaders(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddPolicyAdministrator(&self, bstradmin: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyAdministrator(&self, bstradmin: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPolicyReader(&self, bstrreader: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyReader(&self, bstrreader: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Initialize(&self, lflags: AZ_PROP_CONSTANTS, bstrpolicyurl: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn UpdateCache(&self, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Delete(&self, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Applications(&self) -> ::windows::core::Result<IAzApplications>;
    fn OpenApplication(&self, bstrapplicationname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplication>;
    fn CreateApplication(&self, bstrapplicationname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplication>;
    fn DeleteApplication(&self, bstrapplicationname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ApplicationGroups(&self) -> ::windows::core::Result<IAzApplicationGroups>;
    fn CreateApplicationGroup(&self, bstrgroupname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>;
    fn OpenApplicationGroup(&self, bstrgroupname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>;
    fn DeleteApplicationGroup(&self, bstrgroupname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Submit(&self, lflags: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DelegatedPolicyUsers(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddDelegatedPolicyUser(&self, bstrdelegatedpolicyuser: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteDelegatedPolicyUser(&self, bstrdelegatedpolicyuser: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn TargetMachine(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn ApplyStoreSacl(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetApplyStoreSacl(&self, bapplystoresacl: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn PolicyAdministratorsName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PolicyReadersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddPolicyAdministratorName(&self, bstradmin: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyAdministratorName(&self, bstradmin: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPolicyReaderName(&self, bstrreader: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyReaderName(&self, bstrreader: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DelegatedPolicyUsersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddDelegatedPolicyUserName(&self, bstrdelegatedpolicyuser: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteDelegatedPolicyUserName(&self, bstrdelegatedpolicyuser: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn CloseApplication(&self, bstrapplicationname: &super::super::Foundation::BSTR, lflag: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzAuthorizationStore {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzAuthorizationStore_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>() -> IAzAuthorizationStore_Vtbl {
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn ApplicationData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ApplicationData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrapplicationdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetApplicationData(::core::mem::transmute(&bstrapplicationdata)).into()
        }
        unsafe extern "system" fn DomainTimeout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DomainTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDomainTimeout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDomainTimeout(::core::mem::transmute_copy(&lprop)).into()
        }
        unsafe extern "system" fn ScriptEngineTimeout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ScriptEngineTimeout() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetScriptEngineTimeout<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetScriptEngineTimeout(::core::mem::transmute_copy(&lprop)).into()
        }
        unsafe extern "system" fn MaxScriptEngines<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MaxScriptEngines() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetMaxScriptEngines<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetMaxScriptEngines(::core::mem::transmute_copy(&lprop)).into()
        }
        unsafe extern "system" fn GenerateAudits<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GenerateAudits() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetGenerateAudits<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bprop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetGenerateAudits(::core::mem::transmute_copy(&bprop)).into()
        }
        unsafe extern "system" fn Writable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Writable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddPropertyItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: AZ_PROP_CONSTANTS, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn PolicyAdministrators<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PolicyAdministrators() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvaradmins, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReaders<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PolicyReaders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarreaders, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministrator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPolicyAdministrator(::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyAdministrator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePolicyAdministrator(::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddPolicyReader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPolicyReader(::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyReader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePolicyReader(::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Initialize<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: AZ_PROP_CONSTANTS, bstrpolicyurl: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Initialize(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&bstrpolicyurl), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn UpdateCache<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpdateCache(::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Delete<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Delete(::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Applications<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppappcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Applications() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppappcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenApplication<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppapplication: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenApplication(::core::mem::transmute(&bstrapplicationname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppapplication, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateApplication<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppapplication: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateApplication(::core::mem::transmute(&bstrapplicationname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppapplication, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteApplication<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteApplication(::core::mem::transmute(&bstrapplicationname), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn ApplicationGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroupcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ApplicationGroups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroupcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateApplicationGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateApplicationGroup(::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenApplicationGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenApplicationGroup(::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteApplicationGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteApplicationGroup(::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DelegatedPolicyUsers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DelegatedPolicyUsers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvardelegatedpolicyusers, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDelegatedPolicyUser<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddDelegatedPolicyUser(::core::mem::transmute(&bstrdelegatedpolicyuser), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUser<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteDelegatedPolicyUser(::core::mem::transmute(&bstrdelegatedpolicyuser), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn TargetMachine<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrtargetmachine: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.TargetMachine() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrtargetmachine, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ApplyStoreSacl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbapplystoresacl: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ApplyStoreSacl() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbapplystoresacl, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplyStoreSacl<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bapplystoresacl: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetApplyStoreSacl(::core::mem::transmute_copy(&bapplystoresacl)).into()
        }
        unsafe extern "system" fn PolicyAdministratorsName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PolicyAdministratorsName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvaradmins, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReadersName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PolicyReadersName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarreaders, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministratorName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPolicyAdministratorName(::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyAdministratorName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePolicyAdministratorName(::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddPolicyReaderName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPolicyReaderName(::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyReaderName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePolicyReaderName(::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DelegatedPolicyUsersName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvardelegatedpolicyusers: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.DelegatedPolicyUsersName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvardelegatedpolicyusers, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddDelegatedPolicyUserName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddDelegatedPolicyUserName(::core::mem::transmute(&bstrdelegatedpolicyuser), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteDelegatedPolicyUserName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdelegatedpolicyuser: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteDelegatedPolicyUserName(::core::mem::transmute(&bstrdelegatedpolicyuser), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn CloseApplication<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, lflag: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CloseApplication(::core::mem::transmute(&bstrapplicationname), ::core::mem::transmute_copy(&lflag)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn OpenApplication2(&self, bstrapplicationname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplication2>;
    fn CreateApplication2(&self, bstrapplicationname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplication2>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzAuthorizationStore2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzAuthorizationStore2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore2_Impl, const OFFSET: isize>() -> IAzAuthorizationStore2_Vtbl {
        unsafe extern "system" fn OpenApplication2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppapplication: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenApplication2(::core::mem::transmute(&bstrapplicationname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppapplication, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateApplication2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppapplication: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateApplication2(::core::mem::transmute(&bstrapplicationname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppapplication, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IAzAuthorizationStore_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn IsUpdateNeeded(&self) -> ::windows::core::Result<i16>;
    fn BizruleGroupSupported(&self) -> ::windows::core::Result<i16>;
    fn UpgradeStoresFunctionalLevel(&self, lfunctionallevel: i32) -> ::windows::core::Result<()>;
    fn IsFunctionalLevelUpgradeSupported(&self, lfunctionallevel: i32) -> ::windows::core::Result<i16>;
    fn GetSchemaVersion(&self, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzAuthorizationStore3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzAuthorizationStore3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore3_Impl, const OFFSET: isize>() -> IAzAuthorizationStore3_Vtbl {
        unsafe extern "system" fn IsUpdateNeeded<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisupdateneeded: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsUpdateNeeded() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisupdateneeded, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BizruleGroupSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbsupported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BizruleGroupSupported() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsupported, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UpgradeStoresFunctionalLevel<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfunctionallevel: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.UpgradeStoresFunctionalLevel(::core::mem::transmute_copy(&lfunctionallevel)).into()
        }
        unsafe extern "system" fn IsFunctionalLevelUpgradeSupported<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lfunctionallevel: i32, pbsupported: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsFunctionalLevelUpgradeSupported(::core::mem::transmute_copy(&lfunctionallevel)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbsupported, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSchemaVersion<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzAuthorizationStore3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plmajorversion: *mut i32, plminorversion: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetSchemaVersion(::core::mem::transmute_copy(&plmajorversion), ::core::mem::transmute_copy(&plminorversion)).into()
        }
        Self {
            base__: IAzAuthorizationStore2_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn SetBusinessRuleResult(&self, bresult: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn SetBusinessRuleString(&self, bstrbusinessrulestring: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BusinessRuleString(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetParameter(&self, bstrparametername: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzBizRuleContext {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzBizRuleContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleContext_Impl, const OFFSET: isize>() -> IAzBizRuleContext_Vtbl {
        unsafe extern "system" fn SetBusinessRuleResult<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bresult: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBusinessRuleResult(::core::mem::transmute_copy(&bresult)).into()
        }
        unsafe extern "system" fn SetBusinessRuleString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrbusinessrulestring: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBusinessRuleString(::core::mem::transmute(&bstrbusinessrulestring)).into()
        }
        unsafe extern "system" fn BusinessRuleString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbusinessrulestring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BusinessRuleString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrbusinessrulestring, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetParameter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrparametername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarparametervalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParameter(::core::mem::transmute(&bstrparametername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarparametervalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn AddInterface(&self, bstrinterfacename: &super::super::Foundation::BSTR, linterfaceflag: i32, varinterface: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddInterfaces(&self, varinterfacenames: &super::super::System::Com::VARIANT, varinterfaceflags: &super::super::System::Com::VARIANT, varinterfaces: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetInterfaceValue(&self, bstrinterfacename: &super::super::Foundation::BSTR, linterfaceflag: *mut i32, varinterface: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Remove(&self, bstrinterfacename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoveAll(&self) -> ::windows::core::Result<()>;
    fn Count(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzBizRuleInterfaces {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzBizRuleInterfaces_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>() -> IAzBizRuleInterfaces_Vtbl {
        unsafe extern "system" fn AddInterface<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinterfacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linterfaceflag: i32, varinterface: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddInterface(::core::mem::transmute(&bstrinterfacename), ::core::mem::transmute_copy(&linterfaceflag), ::core::mem::transmute(&varinterface)).into()
        }
        unsafe extern "system" fn AddInterfaces<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varinterfacenames: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varinterfaceflags: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varinterfaces: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddInterfaces(::core::mem::transmute(&varinterfacenames), ::core::mem::transmute(&varinterfaceflags), ::core::mem::transmute(&varinterfaces)).into()
        }
        unsafe extern "system" fn GetInterfaceValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinterfacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, linterfaceflag: *mut i32, varinterface: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetInterfaceValue(::core::mem::transmute(&bstrinterfacename), ::core::mem::transmute_copy(&linterfaceflag), ::core::mem::transmute_copy(&varinterface)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrinterfacename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute(&bstrinterfacename)).into()
        }
        unsafe extern "system" fn RemoveAll<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAll().into()
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleInterfaces_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn AddParameter(&self, bstrparametername: &super::super::Foundation::BSTR, varparametervalue: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddParameters(&self, varparameternames: &super::super::System::Com::VARIANT, varparametervalues: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetParameterValue(&self, bstrparametername: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Remove(&self, varparametername: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RemoveAll(&self) -> ::windows::core::Result<()>;
    fn Count(&self) -> ::windows::core::Result<u32>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzBizRuleParameters {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzBizRuleParameters_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>() -> IAzBizRuleParameters_Vtbl {
        unsafe extern "system" fn AddParameter<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrparametername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varparametervalue: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddParameter(::core::mem::transmute(&bstrparametername), ::core::mem::transmute(&varparametervalue)).into()
        }
        unsafe extern "system" fn AddParameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varparameternames: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varparametervalues: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddParameters(::core::mem::transmute(&varparameternames), ::core::mem::transmute(&varparametervalues)).into()
        }
        unsafe extern "system" fn GetParameterValue<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrparametername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarparametervalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetParameterValue(::core::mem::transmute(&bstrparametername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarparametervalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varparametername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute(&varparametername)).into()
        }
        unsafe extern "system" fn RemoveAll<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAll().into()
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzBizRuleParameters_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn AccessCheck(&self, bstrobjectname: &super::super::Foundation::BSTR, varscopenames: &super::super::System::Com::VARIANT, varoperations: &super::super::System::Com::VARIANT, varparameternames: &super::super::System::Com::VARIANT, varparametervalues: &super::super::System::Com::VARIANT, varinterfacenames: &super::super::System::Com::VARIANT, varinterfaceflags: &super::super::System::Com::VARIANT, varinterfaces: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetBusinessRuleString(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserDn(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserSamCompat(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserDisplay(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserGuid(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserCanonical(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserUpn(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn UserDnsSamCompat(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetProperty(&self, lpropid: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn GetRoles(&self, bstrscopename: &super::super::Foundation::BSTR) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn RoleForAccessCheck(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetRoleForAccessCheck(&self, bstrprop: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzClientContext {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzClientContext_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>() -> IAzClientContext_Vtbl {
        unsafe extern "system" fn AccessCheck<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>(
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
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AccessCheck(::core::mem::transmute(&bstrobjectname), ::core::mem::transmute(&varscopenames), ::core::mem::transmute(&varoperations), ::core::mem::transmute(&varparameternames), ::core::mem::transmute(&varparametervalues), ::core::mem::transmute(&varinterfacenames), ::core::mem::transmute(&varinterfaceflags), ::core::mem::transmute(&varinterfaces)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarresults, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetBusinessRuleString<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrbusinessrulestring: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetBusinessRuleString() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrbusinessrulestring, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserDn<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserDn() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserSamCompat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserSamCompat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserDisplay<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserDisplay() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserGuid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserGuid() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserCanonical<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserCanonical() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserUpn<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserUpn() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn UserDnsSamCompat<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.UserDnsSamCompat() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetRoles<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvarrolenames: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetRoles(::core::mem::transmute(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarrolenames, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn RoleForAccessCheck<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoleForAccessCheck() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRoleForAccessCheck<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetRoleForAccessCheck(::core::mem::transmute(&bstrprop)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn GetAssignedScopesPage(&self, loptions: i32, pagesize: i32, pvarcursor: *mut super::super::System::Com::VARIANT, pvarscopenames: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddRoles(&self, varroles: &super::super::System::Com::VARIANT, bstrscopename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddApplicationGroups(&self, varapplicationgroups: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddStringSids(&self, varstringsids: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn SetLDAPQueryDN(&self, bstrldapquerydn: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn LDAPQueryDN(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzClientContext2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzClientContext2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext2_Impl, const OFFSET: isize>() -> IAzClientContext2_Vtbl {
        unsafe extern "system" fn GetAssignedScopesPage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, loptions: i32, pagesize: i32, pvarcursor: *mut super::super::System::Com::VARIANT, pvarscopenames: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetAssignedScopesPage(::core::mem::transmute_copy(&loptions), ::core::mem::transmute_copy(&pagesize), ::core::mem::transmute_copy(&pvarcursor), ::core::mem::transmute_copy(&pvarscopenames)).into()
        }
        unsafe extern "system" fn AddRoles<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varroles: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRoles(::core::mem::transmute(&varroles), ::core::mem::transmute(&bstrscopename)).into()
        }
        unsafe extern "system" fn AddApplicationGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varapplicationgroups: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddApplicationGroups(::core::mem::transmute(&varapplicationgroups)).into()
        }
        unsafe extern "system" fn AddStringSids<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, varstringsids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddStringSids(::core::mem::transmute(&varstringsids)).into()
        }
        unsafe extern "system" fn SetLDAPQueryDN<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrldapquerydn: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetLDAPQueryDN(::core::mem::transmute(&bstrldapquerydn)).into()
        }
        unsafe extern "system" fn LDAPQueryDN<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrldapquerydn: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.LDAPQueryDN() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrldapquerydn, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IAzClientContext_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn AccessCheck2(&self, bstrobjectname: &super::super::Foundation::BSTR, bstrscopename: &super::super::Foundation::BSTR, loperation: i32) -> ::windows::core::Result<u32>;
    fn IsInRoleAssignment(&self, bstrscopename: &super::super::Foundation::BSTR, bstrrolename: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn GetOperations(&self, bstrscopename: &super::super::Foundation::BSTR) -> ::windows::core::Result<IAzOperations>;
    fn GetTasks(&self, bstrscopename: &super::super::Foundation::BSTR) -> ::windows::core::Result<IAzTasks>;
    fn BizRuleParameters(&self) -> ::windows::core::Result<IAzBizRuleParameters>;
    fn BizRuleInterfaces(&self) -> ::windows::core::Result<IAzBizRuleInterfaces>;
    fn GetGroups(&self, bstrscopename: &super::super::Foundation::BSTR, uloptions: AZ_PROP_CONSTANTS) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Sids(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzClientContext3 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzClientContext3_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: isize>() -> IAzClientContext3_Vtbl {
        unsafe extern "system" fn AccessCheck2<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrobjectname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, loperation: i32, plresult: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AccessCheck2(::core::mem::transmute(&bstrobjectname), ::core::mem::transmute(&bstrscopename), ::core::mem::transmute_copy(&loperation)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plresult, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsInRoleAssignment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbisinrole: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsInRoleAssignment(::core::mem::transmute(&bstrscopename), ::core::mem::transmute(&bstrrolename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisinrole, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOperations<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppoperationcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOperations(::core::mem::transmute(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppoperationcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTasks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptaskcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTasks(::core::mem::transmute(&bstrscopename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptaskcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BizRuleParameters<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbizruleparam: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BizRuleParameters() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbizruleparam, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BizRuleInterfaces<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppbizruleinterfaces: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BizRuleInterfaces() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppbizruleinterfaces, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, uloptions: AZ_PROP_CONSTANTS, pgrouparray: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetGroups(::core::mem::transmute(&bstrscopename), ::core::mem::transmute_copy(&uloptions)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgrouparray, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Sids<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzClientContext3_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pstringsidarray: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Sids() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pstringsidarray, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IAzClientContext2_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn NameFromSid(&self, bstrsid: &super::super::Foundation::BSTR, psidtype: *mut i32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn NamesFromSids(&self, vsids: &super::super::System::Com::VARIANT, pvsidtypes: *mut super::super::System::Com::VARIANT, pvnames: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzNameResolver {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzNameResolver_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzNameResolver_Impl, const OFFSET: isize>() -> IAzNameResolver_Vtbl {
        unsafe extern "system" fn NameFromSid<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzNameResolver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrsid: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psidtype: *mut i32, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NameFromSid(::core::mem::transmute(&bstrsid), ::core::mem::transmute_copy(&psidtype), ::core::mem::transmute_copy(&pbstrname)).into()
        }
        unsafe extern "system" fn NamesFromSids<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzNameResolver_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, vsids: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvsidtypes: *mut super::super::System::Com::VARIANT, pvnames: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NamesFromSids(::core::mem::transmute(&vsids), ::core::mem::transmute_copy(&pvsidtypes), ::core::mem::transmute_copy(&pvnames)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn GetPrincipals(&self, hparentwnd: super::super::Foundation::HWND, bstrtitle: &super::super::Foundation::BSTR, pvsidtypes: *mut super::super::System::Com::VARIANT, pvnames: *mut super::super::System::Com::VARIANT, pvsids: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzObjectPicker {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzObjectPicker_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzObjectPicker_Impl, const OFFSET: isize>() -> IAzObjectPicker_Vtbl {
        unsafe extern "system" fn GetPrincipals<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzObjectPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, hparentwnd: super::super::Foundation::HWND, bstrtitle: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pvsidtypes: *mut super::super::System::Com::VARIANT, pvnames: *mut super::super::System::Com::VARIANT, pvsids: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetPrincipals(::core::mem::transmute_copy(&hparentwnd), ::core::mem::transmute(&bstrtitle), ::core::mem::transmute_copy(&pvsidtypes), ::core::mem::transmute_copy(&pvnames), ::core::mem::transmute_copy(&pvsids)).into()
        }
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzObjectPicker_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ApplicationData(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetApplicationData(&self, bstrapplicationdata: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn OperationID(&self) -> ::windows::core::Result<i32>;
    fn SetOperationID(&self, lprop: i32) -> ::windows::core::Result<()>;
    fn Writable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&self, lpropid: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Submit(&self, lflags: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzOperation {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzOperation_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: isize>() -> IAzOperation_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn ApplicationData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ApplicationData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrapplicationdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetApplicationData(::core::mem::transmute(&bstrapplicationdata)).into()
        }
        unsafe extern "system" fn OperationID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plprop: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OperationID() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetOperationID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lprop: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetOperationID(::core::mem::transmute_copy(&lprop)).into()
        }
        unsafe extern "system" fn Writable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Writable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&varreserved)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn RoleAssignments(&self, bstrscopename: &super::super::Foundation::BSTR, brecursive: i16) -> ::windows::core::Result<IAzRoleAssignments>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzOperation2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzOperation2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation2_Impl, const OFFSET: isize>() -> IAzOperation2_Vtbl {
        unsafe extern "system" fn RoleAssignments<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzOperation2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, brecursive: i16, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoleAssignments(::core::mem::transmute(&bstrscopename), ::core::mem::transmute_copy(&brecursive)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignments, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IAzOperation_Vtbl::new::<Identity, Impl, OFFSET>(), RoleAssignments: RoleAssignments::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzOperation2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IAzOperation as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzOperations_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzOperations {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzOperations_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzOperations_Impl, const OFFSET: isize>() -> IAzOperations_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarobtptr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzOperations_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumptr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
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
    fn NameResolver(&self) -> ::windows::core::Result<IAzNameResolver>;
    fn ObjectPicker(&self) -> ::windows::core::Result<IAzObjectPicker>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzPrincipalLocator {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzPrincipalLocator_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzPrincipalLocator_Impl, const OFFSET: isize>() -> IAzPrincipalLocator_Vtbl {
        unsafe extern "system" fn NameResolver<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzPrincipalLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnameresolver: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.NameResolver() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnameresolver, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ObjectPicker<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzPrincipalLocator_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppobjectpicker: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ObjectPicker() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppobjectpicker, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ApplicationData(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetApplicationData(&self, bstrapplicationdata: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn AddAppMember(&self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteAppMember(&self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddTask(&self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteTask(&self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddOperation(&self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteOperation(&self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddMember(&self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteMember(&self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Writable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&self, lpropid: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AppMembers(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Members(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Operations(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Tasks(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddPropertyItem(&self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePropertyItem(&self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Submit(&self, lflags: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddMemberName(&self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteMemberName(&self, bstrprop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn MembersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzRole {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzRole_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>() -> IAzRole_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn ApplicationData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ApplicationData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrapplicationdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetApplicationData(::core::mem::transmute(&bstrapplicationdata)).into()
        }
        unsafe extern "system" fn AddAppMember<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddAppMember(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteAppMember<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteAppMember(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddTask(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteTask(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddOperation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddOperation(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteOperation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteOperation(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddMember<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddMember(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteMember<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteMember(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Writable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Writable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AppMembers<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.AppMembers() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Members<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Members() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Operations<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Operations() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tasks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Tasks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPropertyItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddMemberName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddMemberName(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteMemberName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteMemberName(::core::mem::transmute(&bstrprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn MembersName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRole_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.MembersName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn AddRoleDefinition(&self, bstrroledefinition: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DeleteRoleDefinition(&self, bstrroledefinition: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RoleDefinitions(&self) -> ::windows::core::Result<IAzRoleDefinitions>;
    fn Scope(&self) -> ::windows::core::Result<IAzScope>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzRoleAssignment {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzRoleAssignment_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleAssignment_Impl, const OFFSET: isize>() -> IAzRoleAssignment_Vtbl {
        unsafe extern "system" fn AddRoleDefinition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinition: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRoleDefinition(::core::mem::transmute(&bstrroledefinition)).into()
        }
        unsafe extern "system" fn DeleteRoleDefinition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinition: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteRoleDefinition(::core::mem::transmute(&bstrroledefinition)).into()
        }
        unsafe extern "system" fn RoleDefinitions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoleDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproledefinitions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Scope<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleAssignment_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppscope: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Scope() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppscope, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IAzRole_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn get_Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzRoleAssignments {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzRoleAssignments_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleAssignments_Impl, const OFFSET: isize>() -> IAzRoleAssignments_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleAssignments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarobtptr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleAssignments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleAssignments_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumptr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
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
    fn RoleAssignments(&self, bstrscopename: &super::super::Foundation::BSTR, brecursive: i16) -> ::windows::core::Result<IAzRoleAssignments>;
    fn AddRoleDefinition(&self, bstrroledefinition: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn DeleteRoleDefinition(&self, bstrroledefinition: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RoleDefinitions(&self) -> ::windows::core::Result<IAzRoleDefinitions>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzRoleDefinition {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzRoleDefinition_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleDefinition_Impl, const OFFSET: isize>() -> IAzRoleDefinition_Vtbl {
        unsafe extern "system" fn RoleAssignments<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, brecursive: i16, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoleAssignments(::core::mem::transmute(&bstrscopename), ::core::mem::transmute_copy(&brecursive)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignments, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddRoleDefinition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinition: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddRoleDefinition(::core::mem::transmute(&bstrroledefinition)).into()
        }
        unsafe extern "system" fn DeleteRoleDefinition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinition: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteRoleDefinition(::core::mem::transmute(&bstrroledefinition)).into()
        }
        unsafe extern "system" fn RoleDefinitions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleDefinition_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoleDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproledefinitions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: IAzTask_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn get_Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzRoleDefinitions {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzRoleDefinitions_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleDefinitions_Impl, const OFFSET: isize>() -> IAzRoleDefinitions_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleDefinitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarobtptr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleDefinitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRoleDefinitions_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumptr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
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
    fn get_Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzRoles {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzRoles_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRoles_Impl, const OFFSET: isize>() -> IAzRoles_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRoles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarobtptr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRoles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzRoles_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumptr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
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
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ApplicationData(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetApplicationData(&self, bstrapplicationdata: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Writable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&self, lpropid: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPropertyItem(&self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePropertyItem(&self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn PolicyAdministrators(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PolicyReaders(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddPolicyAdministrator(&self, bstradmin: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyAdministrator(&self, bstradmin: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPolicyReader(&self, bstrreader: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyReader(&self, bstrreader: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ApplicationGroups(&self) -> ::windows::core::Result<IAzApplicationGroups>;
    fn OpenApplicationGroup(&self, bstrgroupname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>;
    fn CreateApplicationGroup(&self, bstrgroupname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzApplicationGroup>;
    fn DeleteApplicationGroup(&self, bstrgroupname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Roles(&self) -> ::windows::core::Result<IAzRoles>;
    fn OpenRole(&self, bstrrolename: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzRole>;
    fn CreateRole(&self, bstrrolename: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzRole>;
    fn DeleteRole(&self, bstrrolename: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Tasks(&self) -> ::windows::core::Result<IAzTasks>;
    fn OpenTask(&self, bstrtaskname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzTask>;
    fn CreateTask(&self, bstrtaskname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<IAzTask>;
    fn DeleteTask(&self, bstrtaskname: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Submit(&self, lflags: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn CanBeDelegated(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn BizrulesWritable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn PolicyAdministratorsName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PolicyReadersName(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddPolicyAdministratorName(&self, bstradmin: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyAdministratorName(&self, bstradmin: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPolicyReaderName(&self, bstrreader: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePolicyReaderName(&self, bstrreader: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzScope {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzScope_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>() -> IAzScope_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn ApplicationData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ApplicationData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrapplicationdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetApplicationData(::core::mem::transmute(&bstrapplicationdata)).into()
        }
        unsafe extern "system" fn Writable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Writable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddPropertyItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn PolicyAdministrators<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PolicyAdministrators() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvaradmins, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReaders<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PolicyReaders() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarreaders, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministrator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPolicyAdministrator(::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyAdministrator<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePolicyAdministrator(::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddPolicyReader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPolicyReader(::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyReader<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePolicyReader(::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn ApplicationGroups<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppgroupcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ApplicationGroups() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroupcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenApplicationGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenApplicationGroup(::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateApplicationGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, ppgroup: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateApplicationGroup(::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppgroup, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteApplicationGroup<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrgroupname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteApplicationGroup(::core::mem::transmute(&bstrgroupname), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Roles<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pprolecollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Roles() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprolecollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRole<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pprole: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenRole(::core::mem::transmute(&bstrrolename), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprole, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRole<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pprole: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRole(::core::mem::transmute(&bstrrolename), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pprole, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRole<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrrolename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteRole(::core::mem::transmute(&bstrrolename), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Tasks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pptaskcollection: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Tasks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptaskcollection, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pptask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenTask(::core::mem::transmute(&bstrtaskname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptask, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pptask: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateTask(::core::mem::transmute(&bstrtaskname), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptask, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtaskname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteTask(::core::mem::transmute(&bstrtaskname), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn CanBeDelegated<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CanBeDelegated() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BizrulesWritable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BizrulesWritable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyAdministratorsName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvaradmins: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PolicyAdministratorsName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvaradmins, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PolicyReadersName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarreaders: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.PolicyReadersName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarreaders, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddPolicyAdministratorName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPolicyAdministratorName(::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyAdministratorName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstradmin: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePolicyAdministratorName(::core::mem::transmute(&bstradmin), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddPolicyReaderName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPolicyReaderName(::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePolicyReaderName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrreader: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePolicyReaderName(::core::mem::transmute(&bstrreader), ::core::mem::transmute(&varreserved)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn RoleDefinitions(&self) -> ::windows::core::Result<IAzRoleDefinitions>;
    fn CreateRoleDefinition(&self, bstrroledefinitionname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IAzRoleDefinition>;
    fn OpenRoleDefinition(&self, bstrroledefinitionname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IAzRoleDefinition>;
    fn DeleteRoleDefinition(&self, bstrroledefinitionname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn RoleAssignments(&self) -> ::windows::core::Result<IAzRoleAssignments>;
    fn CreateRoleAssignment(&self, bstrroleassignmentname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IAzRoleAssignment>;
    fn OpenRoleAssignment(&self, bstrroleassignmentname: &super::super::Foundation::BSTR) -> ::windows::core::Result<IAzRoleAssignment>;
    fn DeleteRoleAssignment(&self, bstrroleassignmentname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzScope2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzScope2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: isize>() -> IAzScope2_Vtbl {
        unsafe extern "system" fn RoleDefinitions<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoleDefinitions() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproledefinitions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRoleDefinition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRoleDefinition(::core::mem::transmute(&bstrroledefinitionname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproledefinitions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRoleDefinition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproledefinitions: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenRoleDefinition(::core::mem::transmute(&bstrroledefinitionname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproledefinitions, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRoleDefinition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroledefinitionname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteRoleDefinition(::core::mem::transmute(&bstrroledefinitionname)).into()
        }
        unsafe extern "system" fn RoleAssignments<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoleAssignments() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignments, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CreateRoleAssignment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproleassignment: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateRoleAssignment(::core::mem::transmute(&bstrroleassignmentname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignment, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OpenRoleAssignment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pproleassignment: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.OpenRoleAssignment(::core::mem::transmute(&bstrroleassignmentname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignment, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn DeleteRoleAssignment<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScope2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrroleassignmentname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteRoleAssignment(::core::mem::transmute(&bstrroleassignmentname)).into()
        }
        Self {
            base__: IAzScope_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn get_Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzScopes {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzScopes_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScopes_Impl, const OFFSET: isize>() -> IAzScopes_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScopes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarobtptr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScopes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzScopes_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumptr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
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
    fn Name(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetName(&self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Description(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetDescription(&self, bstrdescription: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn ApplicationData(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetApplicationData(&self, bstrapplicationdata: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BizRule(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetBizRule(&self, bstrprop: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BizRuleLanguage(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetBizRuleLanguage(&self, bstrprop: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn BizRuleImportedPath(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn SetBizRuleImportedPath(&self, bstrprop: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn IsRoleDefinition(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn SetIsRoleDefinition(&self, fprop: super::super::Foundation::BOOL) -> ::windows::core::Result<()>;
    fn Operations(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Tasks(&self) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn AddOperation(&self, bstrop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteOperation(&self, bstrop: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddTask(&self, bstrtask: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeleteTask(&self, bstrtask: &super::super::Foundation::BSTR, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Writable(&self) -> ::windows::core::Result<super::super::Foundation::BOOL>;
    fn GetProperty(&self, lpropid: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn SetProperty(&self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn AddPropertyItem(&self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn DeletePropertyItem(&self, lpropid: i32, varprop: &super::super::System::Com::VARIANT, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn Submit(&self, lflags: i32, varreserved: &super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzTask {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzTask_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>() -> IAzTask_Vtbl {
        unsafe extern "system" fn Name<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrname: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Name() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrname, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&bstrname)).into()
        }
        unsafe extern "system" fn Description<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrdescription: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Description() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrdescription, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrdescription: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&bstrdescription)).into()
        }
        unsafe extern "system" fn ApplicationData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrapplicationdata: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.ApplicationData() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrapplicationdata, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetApplicationData<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrapplicationdata: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetApplicationData(::core::mem::transmute(&bstrapplicationdata)).into()
        }
        unsafe extern "system" fn BizRule<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BizRule() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRule<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBizRule(::core::mem::transmute(&bstrprop)).into()
        }
        unsafe extern "system" fn BizRuleLanguage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BizRuleLanguage() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRuleLanguage<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBizRuleLanguage(::core::mem::transmute(&bstrprop)).into()
        }
        unsafe extern "system" fn BizRuleImportedPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbstrprop: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.BizRuleImportedPath() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbstrprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetBizRuleImportedPath<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrprop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetBizRuleImportedPath(::core::mem::transmute(&bstrprop)).into()
        }
        unsafe extern "system" fn IsRoleDefinition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsRoleDefinition() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetIsRoleDefinition<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, fprop: super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetIsRoleDefinition(::core::mem::transmute_copy(&fprop)).into()
        }
        unsafe extern "system" fn Operations<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Operations() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Tasks<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Tasks() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn AddOperation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddOperation(::core::mem::transmute(&bstrop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteOperation<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrop: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteOperation(::core::mem::transmute(&bstrop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddTask(::core::mem::transmute(&bstrtask), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeleteTask<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrtask: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeleteTask(::core::mem::transmute(&bstrtask), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Writable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pfprop: *mut super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Writable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pfprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, pvarprop: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varreserved)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarprop, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetProperty(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn AddPropertyItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddPropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn DeletePropertyItem<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lpropid: i32, varprop: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DeletePropertyItem(::core::mem::transmute_copy(&lpropid), ::core::mem::transmute(&varprop), ::core::mem::transmute(&varreserved)).into()
        }
        unsafe extern "system" fn Submit<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, lflags: i32, varreserved: ::core::mem::ManuallyDrop<super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Submit(::core::mem::transmute_copy(&lflags), ::core::mem::transmute(&varreserved)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
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
    fn RoleAssignments(&self, bstrscopename: &super::super::Foundation::BSTR, brecursive: i16) -> ::windows::core::Result<IAzRoleAssignments>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzTask2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzTask2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask2_Impl, const OFFSET: isize>() -> IAzTask2_Vtbl {
        unsafe extern "system" fn RoleAssignments<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTask2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrscopename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, brecursive: i16, pproleassignments: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.RoleAssignments(::core::mem::transmute(&bstrscopename), ::core::mem::transmute_copy(&brecursive)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pproleassignments, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: IAzTask_Vtbl::new::<Identity, Impl, OFFSET>(), RoleAssignments: RoleAssignments::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzTask2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<IAzTask as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait IAzTasks_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn get_Item(&self, index: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for IAzTasks {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl IAzTasks_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTasks_Impl, const OFFSET: isize>() -> IAzTasks_Vtbl {
        unsafe extern "system" fn get_Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTasks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, index: i32, pvarobtptr: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.get_Item(::core::mem::transmute_copy(&index)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvarobtptr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTasks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(plcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: IAzTasks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumptr: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumptr, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            get_Item: get_Item::<Identity, Impl, OFFSET>,
            Count: Count::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IAzTasks as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
