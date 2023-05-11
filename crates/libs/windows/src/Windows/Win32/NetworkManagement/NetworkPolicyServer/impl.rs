#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISdo_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetPropertyInfo(&self, id: i32) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetProperty(&self, id: i32) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn PutProperty(&self, id: i32, pvalue: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn ResetProperty(&self, id: i32) -> ::windows_core::Result<()>;
    fn Apply(&self) -> ::windows_core::Result<()>;
    fn Restore(&self) -> ::windows_core::Result<()>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISdo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISdo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdo_Impl, const OFFSET: isize>() -> ISdo_Vtbl {
        unsafe extern "system" fn GetPropertyInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, pppropertyinfo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetPropertyInfo(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pppropertyinfo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, pvalue: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetProperty(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PutProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32, pvalue: *const super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.PutProperty(::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&pvalue)).into()
        }
        unsafe extern "system" fn ResetProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ResetProperty(::core::mem::transmute_copy(&id)).into()
        }
        unsafe extern "system" fn Apply<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Apply().into()
        }
        unsafe extern "system" fn Restore<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Restore().into()
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISdo as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISdoCollection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Count(&self) -> ::windows_core::Result<i32>;
    fn Add(&self, bstrname: &::windows_core::BSTR, ppitem: *mut ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn Remove(&self, pitem: ::core::option::Option<&super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn RemoveAll(&self) -> ::windows_core::Result<()>;
    fn Reload(&self) -> ::windows_core::Result<()>;
    fn IsNameUnique(&self, bstrname: &::windows_core::BSTR) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn Item(&self, name: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn _NewEnum(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISdoCollection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISdoCollection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: isize>() -> ISdoCollection_Vtbl {
        unsafe extern "system" fn Count<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcount: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Count() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcount, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Add<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Add(::core::mem::transmute(&bstrname), ::core::mem::transmute_copy(&ppitem)).into()
        }
        unsafe extern "system" fn Remove<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pitem: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Remove(::windows_core::from_raw_borrowed(&pitem)).into()
        }
        unsafe extern "system" fn RemoveAll<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.RemoveAll().into()
        }
        unsafe extern "system" fn Reload<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reload().into()
        }
        unsafe extern "system" fn IsNameUnique<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pbool: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsNameUnique(::core::mem::transmute(&bstrname)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbool, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Item<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, name: *const super::super::System::Variant::VARIANT, pitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Item(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pitem, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoCollection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumvariant: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumvariant, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISdoCollection as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISdoDictionaryOld_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn EnumAttributes(&self, id: *mut super::super::System::Variant::VARIANT, pvalues: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn GetAttributeInfo(&self, id: ATTRIBUTEID, pinfoids: *const super::super::System::Variant::VARIANT) -> ::windows_core::Result<super::super::System::Variant::VARIANT>;
    fn EnumAttributeValues(&self, id: ATTRIBUTEID, pvalueids: *mut super::super::System::Variant::VARIANT, pvaluesdesc: *mut super::super::System::Variant::VARIANT) -> ::windows_core::Result<()>;
    fn CreateAttribute(&self, id: ATTRIBUTEID) -> ::windows_core::Result<super::super::System::Com::IDispatch>;
    fn GetAttributeID(&self, bstrattributename: &::windows_core::BSTR) -> ::windows_core::Result<ATTRIBUTEID>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISdoDictionaryOld {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISdoDictionaryOld_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoDictionaryOld_Impl, const OFFSET: isize>() -> ISdoDictionaryOld_Vtbl {
        unsafe extern "system" fn EnumAttributes<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoDictionaryOld_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: *mut super::super::System::Variant::VARIANT, pvalues: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumAttributes(::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&pvalues)).into()
        }
        unsafe extern "system" fn GetAttributeInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoDictionaryOld_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ATTRIBUTEID, pinfoids: *const super::super::System::Variant::VARIANT, pinfovalues: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAttributeInfo(::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&pinfoids)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pinfovalues, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnumAttributeValues<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoDictionaryOld_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ATTRIBUTEID, pvalueids: *mut super::super::System::Variant::VARIANT, pvaluesdesc: *mut super::super::System::Variant::VARIANT) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnumAttributeValues(::core::mem::transmute_copy(&id), ::core::mem::transmute_copy(&pvalueids), ::core::mem::transmute_copy(&pvaluesdesc)).into()
        }
        unsafe extern "system" fn CreateAttribute<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoDictionaryOld_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, id: ATTRIBUTEID, ppattributeobject: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.CreateAttribute(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppattributeobject, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttributeID<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoDictionaryOld_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrattributename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pid: *mut ATTRIBUTEID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAttributeID(::core::mem::transmute(&bstrattributename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISdoDictionaryOld as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISdoMachine_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn Attach(&self, bstrcomputername: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetDictionarySDO(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetServiceSDO(&self, edatastore: IASDATASTORE, bstrservicename: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetUserSDO(&self, edatastore: IASDATASTORE, bstrusername: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn GetOSType(&self) -> ::windows_core::Result<IASOSTYPE>;
    fn GetDomainType(&self) -> ::windows_core::Result<IASDOMAINTYPE>;
    fn IsDirectoryAvailable(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetAttachedComputer(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn GetSDOSchema(&self) -> ::windows_core::Result<::windows_core::IUnknown>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISdoMachine {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISdoMachine_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: isize>() -> ISdoMachine_Vtbl {
        unsafe extern "system" fn Attach<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcomputername: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Attach(::core::mem::transmute(&bstrcomputername)).into()
        }
        unsafe extern "system" fn GetDictionarySDO<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppdictionarysdo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDictionarySDO() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppdictionarysdo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetServiceSDO<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, edatastore: IASDATASTORE, bstrservicename: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppservicesdo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetServiceSDO(::core::mem::transmute_copy(&edatastore), ::core::mem::transmute(&bstrservicename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppservicesdo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetUserSDO<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, edatastore: IASDATASTORE, bstrusername: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppusersdo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetUserSDO(::core::mem::transmute_copy(&edatastore), ::core::mem::transmute(&bstrusername)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppusersdo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetOSType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, eostype: *mut IASOSTYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetOSType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(eostype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDomainType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, edomaintype: *mut IASDOMAINTYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDomainType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(edomaintype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsDirectoryAvailable<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, booldirectoryavailable: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsDirectoryAvailable() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(booldirectoryavailable, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAttachedComputer<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrcomputername: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAttachedComputer() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(bstrcomputername, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetSDOSchema<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppsdoschema: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetSDOSchema() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppsdoschema, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISdoMachine as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISdoMachine2_Impl: Sized + ISdoMachine_Impl {
    fn GetTemplatesSDO(&self, bstrservicename: &::windows_core::BSTR) -> ::windows_core::Result<::windows_core::IUnknown>;
    fn EnableTemplates(&self) -> ::windows_core::Result<()>;
    fn SyncConfigAgainstTemplates(&self, bstrservicename: &::windows_core::BSTR, ppconfigroot: *mut ::core::option::Option<::windows_core::IUnknown>, pptemplatesroot: *mut ::core::option::Option<::windows_core::IUnknown>, bforcedsync: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
    fn ImportRemoteTemplates(&self, plocaltemplatesroot: ::core::option::Option<&::windows_core::IUnknown>, bstrremotemachinename: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn Reload(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISdoMachine2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISdoMachine2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine2_Impl, const OFFSET: isize>() -> ISdoMachine2_Vtbl {
        unsafe extern "system" fn GetTemplatesSDO<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservicename: ::std::mem::MaybeUninit<::windows_core::BSTR>, pptemplatessdo: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetTemplatesSDO(::core::mem::transmute(&bstrservicename)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pptemplatessdo, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn EnableTemplates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.EnableTemplates().into()
        }
        unsafe extern "system" fn SyncConfigAgainstTemplates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrservicename: ::std::mem::MaybeUninit<::windows_core::BSTR>, ppconfigroot: *mut *mut ::core::ffi::c_void, pptemplatesroot: *mut *mut ::core::ffi::c_void, bforcedsync: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SyncConfigAgainstTemplates(::core::mem::transmute(&bstrservicename), ::core::mem::transmute_copy(&ppconfigroot), ::core::mem::transmute_copy(&pptemplatesroot), ::core::mem::transmute_copy(&bforcedsync)).into()
        }
        unsafe extern "system" fn ImportRemoteTemplates<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, plocaltemplatesroot: *mut ::core::ffi::c_void, bstrremotemachinename: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ImportRemoteTemplates(::windows_core::from_raw_borrowed(&plocaltemplatesroot), ::core::mem::transmute(&bstrremotemachinename)).into()
        }
        unsafe extern "system" fn Reload<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoMachine2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISdoMachine2 as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<ISdoMachine as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ISdoServiceControl_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn StartService(&self) -> ::windows_core::Result<()>;
    fn StopService(&self) -> ::windows_core::Result<()>;
    fn GetServiceStatus(&self) -> ::windows_core::Result<i32>;
    fn ResetService(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ISdoServiceControl {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ISdoServiceControl_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoServiceControl_Impl, const OFFSET: isize>() -> ISdoServiceControl_Vtbl {
        unsafe extern "system" fn StartService<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoServiceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StartService().into()
        }
        unsafe extern "system" fn StopService<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoServiceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.StopService().into()
        }
        unsafe extern "system" fn GetServiceStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoServiceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, status: *mut i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetServiceStatus() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(status, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ResetService<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ISdoServiceControl_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
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
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ISdoServiceControl as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_NetworkManagement_NetworkPolicyServer\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait ITemplateSdo_Impl: Sized + ISdo_Impl {
    fn AddToCollection(&self, bstrname: &::windows_core::BSTR, pcollection: ::core::option::Option<&super::super::System::Com::IDispatch>, ppitem: *mut ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn AddToSdo(&self, bstrname: &::windows_core::BSTR, psdotarget: ::core::option::Option<&super::super::System::Com::IDispatch>, ppitem: *mut ::core::option::Option<super::super::System::Com::IDispatch>) -> ::windows_core::Result<()>;
    fn AddToSdoAsProperty(&self, psdotarget: ::core::option::Option<&super::super::System::Com::IDispatch>, id: i32) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for ITemplateSdo {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ITemplateSdo_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITemplateSdo_Impl, const OFFSET: isize>() -> ITemplateSdo_Vtbl {
        unsafe extern "system" fn AddToCollection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITemplateSdo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, pcollection: *mut ::core::ffi::c_void, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddToCollection(::core::mem::transmute(&bstrname), ::windows_core::from_raw_borrowed(&pcollection), ::core::mem::transmute_copy(&ppitem)).into()
        }
        unsafe extern "system" fn AddToSdo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITemplateSdo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, bstrname: ::std::mem::MaybeUninit<::windows_core::BSTR>, psdotarget: *mut ::core::ffi::c_void, ppitem: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddToSdo(::core::mem::transmute(&bstrname), ::windows_core::from_raw_borrowed(&psdotarget), ::core::mem::transmute_copy(&ppitem)).into()
        }
        unsafe extern "system" fn AddToSdoAsProperty<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: ITemplateSdo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psdotarget: *mut ::core::ffi::c_void, id: i32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.AddToSdoAsProperty(::windows_core::from_raw_borrowed(&psdotarget), ::core::mem::transmute_copy(&id)).into()
        }
        Self {
            base__: ISdo_Vtbl::new::<Identity, Impl, OFFSET>(),
            AddToCollection: AddToCollection::<Identity, Impl, OFFSET>,
            AddToSdo: AddToSdo::<Identity, Impl, OFFSET>,
            AddToSdoAsProperty: AddToSdoAsProperty::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<ITemplateSdo as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<ISdo as ::windows_core::ComInterface>::IID
    }
}
