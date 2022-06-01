#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISdo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetPropertyInfo(&self, id: i32) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetProperty(&self, id: i32) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn PutProperty(&self, id: i32, pvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn ResetProperty(&self, id: i32) -> ::windows::core::Result<()>;
    fn Apply(&self) -> ::windows::core::Result<()>;
    fn Restore(&self) -> ::windows::core::Result<()>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISdo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISdo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdo_Impl, const OFFSET: isize>() -> ISdo_Vtbl {
        unsafe extern "system" fn GetPropertyInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, pppropertyinfo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropertyInfo(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertyinfo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, pvalue: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, pvalue: *const super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutProperty(::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn ResetProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResetProperty(::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn Apply<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Apply().into()
        }
        unsafe extern "system" fn Restore<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Restore().into()
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumvariant, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetPropertyInfo: GetPropertyInfo::<Identity, Impl, OFFSET>,
            GetProperty: GetProperty::<Identity, Impl, OFFSET>,
            PutProperty: PutProperty::<Identity, Impl, OFFSET>,
            ResetProperty: ResetProperty::<Identity, Impl, OFFSET>,
            Apply: Apply::<Identity, Impl, OFFSET>,
            Restore: Restore::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISdo as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISdoCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows::core::Result<i32>;
    fn Add(&self, bstrname: &super::super::Foundation::BSTR, ppitem: *mut ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn Remove(&self, pitem: &::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn RemoveAll(&self) -> ::windows::core::Result<()>;
    fn Reload(&self) -> ::windows::core::Result<()>;
    fn IsNameUnique(&self, bstrname: &super::super::Foundation::BSTR) -> ::windows::core::Result<i16>;
    fn Item(&self, name: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn _NewEnum(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISdoCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISdoCollection_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: isize>() -> ISdoCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&ppitem)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::core::mem::transmute(&pitem)).into()
        }
        unsafe extern "system" fn RemoveAll<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAll().into()
        }
        unsafe extern "system" fn Reload<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reload().into()
        }
        unsafe extern "system" fn IsNameUnique<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pbool: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsNameUnique(::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbool, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *const super::super::System::Com::VARIANT, pitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumvariant, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Count: Count::<Identity, Impl, OFFSET>,
            Add: Add::<Identity, Impl, OFFSET>,
            Remove: Remove::<Identity, Impl, OFFSET>,
            RemoveAll: RemoveAll::<Identity, Impl, OFFSET>,
            Reload: Reload::<Identity, Impl, OFFSET>,
            IsNameUnique: IsNameUnique::<Identity, Impl, OFFSET>,
            Item: Item::<Identity, Impl, OFFSET>,
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISdoCollection as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISdoDictionaryOld_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn EnumAttributes(&self, id: *mut super::super::System::Com::VARIANT, pvalues: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn GetAttributeInfo(&self, id: ATTRIBUTEID, pinfoids: *const super::super::System::Com::VARIANT) -> ::windows::core::Result<super::super::System::Com::VARIANT>;
    fn EnumAttributeValues(&self, id: ATTRIBUTEID, pvalueids: *mut super::super::System::Com::VARIANT, pvaluesdesc: *mut super::super::System::Com::VARIANT) -> ::windows::core::Result<()>;
    fn CreateAttribute(&self, id: ATTRIBUTEID) -> ::windows::core::Result<super::super::System::Com::IDispatch>;
    fn GetAttributeID(&self, bstrattributename: &super::super::Foundation::BSTR) -> ::windows::core::Result<ATTRIBUTEID>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISdoDictionaryOld {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISdoDictionaryOld_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoDictionaryOld_Impl, const OFFSET: isize>() -> ISdoDictionaryOld_Vtbl {
        unsafe extern "system" fn EnumAttributes<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoDictionaryOld_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut super::super::System::Com::VARIANT, pvalues: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumAttributes(::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&pvalues)).into()
        }
        unsafe extern "system" fn GetAttributeInfo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoDictionaryOld_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ATTRIBUTEID, pinfoids: *const super::super::System::Com::VARIANT, pinfovalues: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAttributeInfo(::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&pinfoids)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinfovalues, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumAttributeValues<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoDictionaryOld_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ATTRIBUTEID, pvalueids: *mut super::super::System::Com::VARIANT, pvaluesdesc: *mut super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumAttributeValues(::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&pvalueids), ::core::mem::transmute_copy(&pvaluesdesc)).into()
        }
        unsafe extern "system" fn CreateAttribute<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoDictionaryOld_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ATTRIBUTEID, ppattributeobject: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateAttribute(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppattributeobject, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeID<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoDictionaryOld_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattributename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pid: *mut ATTRIBUTEID) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAttributeID(::core::mem::transmute(&bstrattributename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pid, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            EnumAttributes: EnumAttributes::<Identity, Impl, OFFSET>,
            GetAttributeInfo: GetAttributeInfo::<Identity, Impl, OFFSET>,
            EnumAttributeValues: EnumAttributeValues::<Identity, Impl, OFFSET>,
            CreateAttribute: CreateAttribute::<Identity, Impl, OFFSET>,
            GetAttributeID: GetAttributeID::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISdoDictionaryOld as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISdoMachine_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Attach(&self, bstrcomputername: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn GetDictionarySDO(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetServiceSDO(&self, edatastore: IASDATASTORE, bstrservicename: &super::super::Foundation::BSTR) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetUserSDO(&self, edatastore: IASDATASTORE, bstrusername: &super::super::Foundation::BSTR) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn GetOSType(&self) -> ::windows::core::Result<IASOSTYPE>;
    fn GetDomainType(&self) -> ::windows::core::Result<IASDOMAINTYPE>;
    fn IsDirectoryAvailable(&self) -> ::windows::core::Result<i16>;
    fn GetAttachedComputer(&self) -> ::windows::core::Result<super::super::Foundation::BSTR>;
    fn GetSDOSchema(&self) -> ::windows::core::Result<::windows::core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISdoMachine {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISdoMachine_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: isize>() -> ISdoMachine_Vtbl {
        unsafe extern "system" fn Attach<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcomputername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Attach(::core::mem::transmute(&bstrcomputername)).into()
        }
        unsafe extern "system" fn GetDictionarySDO<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdictionarysdo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDictionarySDO() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdictionarysdo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServiceSDO<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, edatastore: IASDATASTORE, bstrservicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppservicesdo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetServiceSDO(::core::mem::transmute_copy(&edatastore), ::core::mem::transmute(&bstrservicename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservicesdo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserSDO<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, edatastore: IASDATASTORE, bstrusername: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppusersdo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUserSDO(::core::mem::transmute_copy(&edatastore), ::core::mem::transmute(&bstrusername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppusersdo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOSType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eostype: *mut IASOSTYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOSType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eostype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDomainType<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, edomaintype: *mut IASDOMAINTYPE) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDomainType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(edomaintype, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDirectoryAvailable<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, booldirectoryavailable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsDirectoryAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(booldirectoryavailable, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttachedComputer<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcomputername: *mut super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAttachedComputer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrcomputername, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSDOSchema<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsdoschema: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSDOSchema() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsdoschema, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            Attach: Attach::<Identity, Impl, OFFSET>,
            GetDictionarySDO: GetDictionarySDO::<Identity, Impl, OFFSET>,
            GetServiceSDO: GetServiceSDO::<Identity, Impl, OFFSET>,
            GetUserSDO: GetUserSDO::<Identity, Impl, OFFSET>,
            GetOSType: GetOSType::<Identity, Impl, OFFSET>,
            GetDomainType: GetDomainType::<Identity, Impl, OFFSET>,
            IsDirectoryAvailable: IsDirectoryAvailable::<Identity, Impl, OFFSET>,
            GetAttachedComputer: GetAttachedComputer::<Identity, Impl, OFFSET>,
            GetSDOSchema: GetSDOSchema::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISdoMachine as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISdoMachine2_Impl: Sized + super::super::System::Com::IDispatch_Impl + ISdoMachine_Impl {
    fn GetTemplatesSDO(&self, bstrservicename: &super::super::Foundation::BSTR) -> ::windows::core::Result<::windows::core::IUnknown>;
    fn EnableTemplates(&self) -> ::windows::core::Result<()>;
    fn SyncConfigAgainstTemplates(&self, bstrservicename: &super::super::Foundation::BSTR, ppconfigroot: *mut ::core::option::Option<::windows::core::IUnknown>, pptemplatesroot: *mut ::core::option::Option<::windows::core::IUnknown>, bforcedsync: i16) -> ::windows::core::Result<()>;
    fn ImportRemoteTemplates(&self, plocaltemplatesroot: &::core::option::Option<::windows::core::IUnknown>, bstrremotemachinename: &super::super::Foundation::BSTR) -> ::windows::core::Result<()>;
    fn Reload(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISdoMachine2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISdoMachine2_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine2_Impl, const OFFSET: isize>() -> ISdoMachine2_Vtbl {
        unsafe extern "system" fn GetTemplatesSDO<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pptemplatessdo: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTemplatesSDO(::core::mem::transmute(&bstrservicename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptemplatessdo, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableTemplates<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableTemplates().into()
        }
        unsafe extern "system" fn SyncConfigAgainstTemplates<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservicename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, ppconfigroot: *mut *mut ::core::ffi::c_void, pptemplatesroot: *mut *mut ::core::ffi::c_void, bforcedsync: i16) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SyncConfigAgainstTemplates(::core::mem::transmute(&bstrservicename), ::core::mem::transmute_copy(&ppconfigroot), ::core::mem::transmute_copy(&pptemplatesroot), ::core::mem::transmute_copy(&bforcedsync)).into()
        }
        unsafe extern "system" fn ImportRemoteTemplates<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocaltemplatesroot: *mut ::core::ffi::c_void, bstrremotemachinename: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ImportRemoteTemplates(::core::mem::transmute(&plocaltemplatesroot), ::core::mem::transmute(&bstrremotemachinename)).into()
        }
        unsafe extern "system" fn Reload<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reload().into()
        }
        Self {
            base__: ISdoMachine_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetTemplatesSDO: GetTemplatesSDO::<Identity, Impl, OFFSET>,
            EnableTemplates: EnableTemplates::<Identity, Impl, OFFSET>,
            SyncConfigAgainstTemplates: SyncConfigAgainstTemplates::<Identity, Impl, OFFSET>,
            ImportRemoteTemplates: ImportRemoteTemplates::<Identity, Impl, OFFSET>,
            Reload: Reload::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISdoMachine2 as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISdoMachine as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ISdoServiceControl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StartService(&self) -> ::windows::core::Result<()>;
    fn StopService(&self) -> ::windows::core::Result<()>;
    fn GetServiceStatus(&self) -> ::windows::core::Result<i32>;
    fn ResetService(&self) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ISdoServiceControl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ISdoServiceControl_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoServiceControl_Impl, const OFFSET: isize>() -> ISdoServiceControl_Vtbl {
        unsafe extern "system" fn StartService<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoServiceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartService().into()
        }
        unsafe extern "system" fn StopService<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoServiceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StopService().into()
        }
        unsafe extern "system" fn GetServiceStatus<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoServiceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetServiceStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetService<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ISdoServiceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResetService().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            StartService: StartService::<Identity, Impl, OFFSET>,
            StopService: StopService::<Identity, Impl, OFFSET>,
            GetServiceStatus: GetServiceStatus::<Identity, Impl, OFFSET>,
            ResetService: ResetService::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ISdoServiceControl as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID
    }
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
pub trait ITemplateSdo_Impl: Sized + super::super::System::Com::IDispatch_Impl + ISdo_Impl {
    fn AddToCollection(&self, bstrname: &super::super::Foundation::BSTR, pcollection: &::core::option::Option<super::super::System::Com::IDispatch>, ppitem: *mut ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn AddToSdo(&self, bstrname: &super::super::Foundation::BSTR, psdotarget: &::core::option::Option<super::super::System::Com::IDispatch>, ppitem: *mut ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows::core::Result<()>;
    fn AddToSdoAsProperty(&self, psdotarget: &::core::option::Option<super::super::System::Com::IDispatch>, id: i32) -> ::windows::core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ::windows::core::RuntimeName for ITemplateSdo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole"))]
impl ITemplateSdo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITemplateSdo_Impl, const OFFSET: isize>() -> ITemplateSdo_Vtbl {
        unsafe extern "system" fn AddToCollection<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITemplateSdo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, pcollection: *mut ::core::ffi::c_void, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddToCollection(::core::mem::transmute(&bstrname), ::core::mem::transmute(&pcollection), ::core::mem::transmute_copy(&ppitem)).into()
        }
        unsafe extern "system" fn AddToSdo<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITemplateSdo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::core::mem::ManuallyDrop<super::super::Foundation::BSTR>, psdotarget: *mut ::core::ffi::c_void, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddToSdo(::core::mem::transmute(&bstrname), ::core::mem::transmute(&psdotarget), ::core::mem::transmute_copy(&ppitem)).into()
        }
        unsafe extern "system" fn AddToSdoAsProperty<Identity: ::windows::core::IUnknownImpl<Impl = Impl>, Impl: ITemplateSdo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psdotarget: *mut ::core::ffi::c_void, id: i32) -> ::windows::core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddToSdoAsProperty(::core::mem::transmute(&psdotarget), ::core::mem::transmute_copy(&id)).into()
        }
        Self {
            base__: ISdo_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddToCollection: AddToCollection::<Identity, Impl, OFFSET>,
            AddToSdo: AddToSdo::<Identity, Impl, OFFSET>,
            AddToSdoAsProperty: AddToSdoAsProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<ITemplateSdo as ::windows::core::Interface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows::core::Interface>::IID || iid == &<ISdo as ::windows::core::Interface>::IID
    }
}
