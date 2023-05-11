#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEnumNetworkConnections_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows_core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<INetworkConnection>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IEnumNetworkConnections>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IEnumNetworkConnections {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IEnumNetworkConnections_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetworkConnections_Impl, const OFFSET: isize>() -> IEnumNetworkConnections_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetworkConnections_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumvar: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumvar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetworkConnections_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetworkConnections_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetworkConnections_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetworkConnections_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumnetwork: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumnetwork, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumNetworkConnections as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait IEnumNetworks_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn _NewEnum(&self) -> ::windows_core::Result<super::super::System::Ole::IEnumVARIANT>;
    fn Next(&self, celt: u32, rgelt: *mut ::core::option::Option<INetwork>, pceltfetched: *mut u32) -> ::windows_core::Result<()>;
    fn Skip(&self, celt: u32) -> ::windows_core::Result<()>;
    fn Reset(&self) -> ::windows_core::Result<()>;
    fn Clone(&self) -> ::windows_core::Result<IEnumNetworks>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for IEnumNetworks {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl IEnumNetworks_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetworks_Impl, const OFFSET: isize>() -> IEnumNetworks_Vtbl {
        unsafe extern "system" fn _NewEnum<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetworks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumvar: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this._NewEnum() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumvar, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Next<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetworks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32, rgelt: *mut *mut ::core::ffi::c_void, pceltfetched: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Next(::core::mem::transmute_copy(&celt), ::core::mem::transmute_copy(&rgelt), ::core::mem::transmute_copy(&pceltfetched)).into()
        }
        unsafe extern "system" fn Skip<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetworks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, celt: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Skip(::core::mem::transmute_copy(&celt)).into()
        }
        unsafe extern "system" fn Reset<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetworks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.Reset().into()
        }
        unsafe extern "system" fn Clone<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IEnumNetworks_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumnetwork: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.Clone() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumnetwork, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            _NewEnum: _NewEnum::<Identity, Impl, OFFSET>,
            Next: Next::<Identity, Impl, OFFSET>,
            Skip: Skip::<Identity, Impl, OFFSET>,
            Reset: Reset::<Identity, Impl, OFFSET>,
            Clone: Clone::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IEnumNetworks as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetwork_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetName(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetName(&self, sznetworknewname: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetDescription(&self) -> ::windows_core::Result<::windows_core::BSTR>;
    fn SetDescription(&self, szdescription: &::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn GetNetworkId(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetDomainType(&self) -> ::windows_core::Result<NLM_DOMAIN_TYPE>;
    fn GetNetworkConnections(&self) -> ::windows_core::Result<IEnumNetworkConnections>;
    fn GetTimeCreatedAndConnected(&self, pdwlowdatetimecreated: *mut u32, pdwhighdatetimecreated: *mut u32, pdwlowdatetimeconnected: *mut u32, pdwhighdatetimeconnected: *mut u32) -> ::windows_core::Result<()>;
    fn IsConnectedToInternet(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsConnected(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetConnectivity(&self) -> ::windows_core::Result<NLM_CONNECTIVITY>;
    fn GetCategory(&self) -> ::windows_core::Result<NLM_NETWORK_CATEGORY>;
    fn SetCategory(&self, newcategory: NLM_NETWORK_CATEGORY) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for INetwork {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetwork_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>() -> INetwork_Vtbl {
        unsafe extern "system" fn GetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psznetworkname: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetName() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(psznetworkname, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetName<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, sznetworknewname: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetName(::core::mem::transmute(&sznetworknewname)).into()
        }
        unsafe extern "system" fn GetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pszdescription: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDescription() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pszdescription, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetDescription<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, szdescription: ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDescription(::core::mem::transmute(&szdescription)).into()
        }
        unsafe extern "system" fn GetNetworkId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgdguidnetworkid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNetworkId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgdguidnetworkid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDomainType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pnetworktype: *mut NLM_DOMAIN_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDomainType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pnetworktype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkConnections<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenumnetworkconnection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNetworkConnections() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumnetworkconnection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetTimeCreatedAndConnected<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdwlowdatetimecreated: *mut u32, pdwhighdatetimecreated: *mut u32, pdwlowdatetimeconnected: *mut u32, pdwhighdatetimeconnected: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetTimeCreatedAndConnected(::core::mem::transmute_copy(&pdwlowdatetimecreated), ::core::mem::transmute_copy(&pdwhighdatetimecreated), ::core::mem::transmute_copy(&pdwlowdatetimeconnected), ::core::mem::transmute_copy(&pdwhighdatetimeconnected)).into()
        }
        unsafe extern "system" fn IsConnectedToInternet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsConnectedToInternet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisconnected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisconnected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectivity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConnectivity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pconnectivity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCategory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcategory: *mut NLM_NETWORK_CATEGORY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCategory() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcategory, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCategory<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcategory: NLM_NETWORK_CATEGORY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetCategory(::core::mem::transmute_copy(&newcategory)).into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetName: GetName::<Identity, Impl, OFFSET>,
            SetName: SetName::<Identity, Impl, OFFSET>,
            GetDescription: GetDescription::<Identity, Impl, OFFSET>,
            SetDescription: SetDescription::<Identity, Impl, OFFSET>,
            GetNetworkId: GetNetworkId::<Identity, Impl, OFFSET>,
            GetDomainType: GetDomainType::<Identity, Impl, OFFSET>,
            GetNetworkConnections: GetNetworkConnections::<Identity, Impl, OFFSET>,
            GetTimeCreatedAndConnected: GetTimeCreatedAndConnected::<Identity, Impl, OFFSET>,
            IsConnectedToInternet: IsConnectedToInternet::<Identity, Impl, OFFSET>,
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
            GetConnectivity: GetConnectivity::<Identity, Impl, OFFSET>,
            GetCategory: GetCategory::<Identity, Impl, OFFSET>,
            SetCategory: SetCategory::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetwork as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetwork2_Impl: Sized + INetwork_Impl {
    fn IsDomainAuthenticatedBy(&self, domainauthenticationkind: NLM_DOMAIN_AUTHENTICATION_KIND) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for INetwork2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetwork2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork2_Impl, const OFFSET: isize>() -> INetwork2_Vtbl {
        unsafe extern "system" fn IsDomainAuthenticatedBy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetwork2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domainauthenticationkind: NLM_DOMAIN_AUTHENTICATION_KIND, pvalue: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsDomainAuthenticatedBy(::core::mem::transmute_copy(&domainauthenticationkind)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: INetwork_Vtbl::new::<Identity, Impl, OFFSET>(), IsDomainAuthenticatedBy: IsDomainAuthenticatedBy::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetwork2 as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<INetwork as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetworkConnection_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetNetwork(&self) -> ::windows_core::Result<INetwork>;
    fn IsConnectedToInternet(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsConnected(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetConnectivity(&self) -> ::windows_core::Result<NLM_CONNECTIVITY>;
    fn GetConnectionId(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetAdapterId(&self) -> ::windows_core::Result<::windows_core::GUID>;
    fn GetDomainType(&self) -> ::windows_core::Result<NLM_DOMAIN_TYPE>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for INetworkConnection {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetworkConnection_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnection_Impl, const OFFSET: isize>() -> INetworkConnection_Vtbl {
        unsafe extern "system" fn GetNetwork<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppnetwork: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNetwork() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnetwork, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnectedToInternet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsConnectedToInternet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisconnected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisconnected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectivity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConnectivity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pconnectivity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectionId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgdconnectionid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConnectionId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgdconnectionid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAdapterId<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgdadapterid: *mut ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetAdapterId() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pgdadapterid, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDomainType<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnection_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdomaintype: *mut NLM_DOMAIN_TYPE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetDomainType() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pdomaintype, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetNetwork: GetNetwork::<Identity, Impl, OFFSET>,
            IsConnectedToInternet: IsConnectedToInternet::<Identity, Impl, OFFSET>,
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
            GetConnectivity: GetConnectivity::<Identity, Impl, OFFSET>,
            GetConnectionId: GetConnectionId::<Identity, Impl, OFFSET>,
            GetAdapterId: GetAdapterId::<Identity, Impl, OFFSET>,
            GetDomainType: GetDomainType::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetworkConnection as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetworkConnection2_Impl: Sized + INetworkConnection_Impl {
    fn IsDomainAuthenticatedBy(&self, domainauthenticationkind: NLM_DOMAIN_AUTHENTICATION_KIND) -> ::windows_core::Result<super::super::Foundation::BOOL>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for INetworkConnection2 {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetworkConnection2_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnection2_Impl, const OFFSET: isize>() -> INetworkConnection2_Vtbl {
        unsafe extern "system" fn IsDomainAuthenticatedBy<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnection2_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, domainauthenticationkind: NLM_DOMAIN_AUTHENTICATION_KIND, pvalue: *mut super::super::Foundation::BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsDomainAuthenticatedBy(::core::mem::transmute_copy(&domainauthenticationkind)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pvalue, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base__: INetworkConnection_Vtbl::new::<Identity, Impl, OFFSET>(), IsDomainAuthenticatedBy: IsDomainAuthenticatedBy::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetworkConnection2 as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID || iid == &<INetworkConnection as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait INetworkConnectionCost_Impl: Sized {
    fn GetCost(&self) -> ::windows_core::Result<u32>;
    fn GetDataPlanStatus(&self, pdataplanstatus: *mut NLM_DATAPLAN_STATUS) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for INetworkConnectionCost {}
#[cfg(feature = "Win32_Foundation")]
impl INetworkConnectionCost_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnectionCost_Impl, const OFFSET: isize>() -> INetworkConnectionCost_Vtbl {
        unsafe extern "system" fn GetCost<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnectionCost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcost: *mut u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetCost() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pcost, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetDataPlanStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnectionCost_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataplanstatus: *mut NLM_DATAPLAN_STATUS) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDataPlanStatus(::core::mem::transmute_copy(&pdataplanstatus)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCost: GetCost::<Identity, Impl, OFFSET>,
            GetDataPlanStatus: GetDataPlanStatus::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetworkConnectionCost as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"implement\"`*"]
pub trait INetworkConnectionCostEvents_Impl: Sized {
    fn ConnectionCostChanged(&self, connectionid: &::windows_core::GUID, newcost: u32) -> ::windows_core::Result<()>;
    fn ConnectionDataPlanStatusChanged(&self, connectionid: &::windows_core::GUID) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for INetworkConnectionCostEvents {}
impl INetworkConnectionCostEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnectionCostEvents_Impl, const OFFSET: isize>() -> INetworkConnectionCostEvents_Vtbl {
        unsafe extern "system" fn ConnectionCostChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnectionCostEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: ::windows_core::GUID, newcost: u32) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConnectionCostChanged(::core::mem::transmute(&connectionid), ::core::mem::transmute_copy(&newcost)).into()
        }
        unsafe extern "system" fn ConnectionDataPlanStatusChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnectionCostEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConnectionDataPlanStatusChanged(::core::mem::transmute(&connectionid)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            ConnectionCostChanged: ConnectionCostChanged::<Identity, Impl, OFFSET>,
            ConnectionDataPlanStatusChanged: ConnectionDataPlanStatusChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetworkConnectionCostEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"implement\"`*"]
pub trait INetworkConnectionEvents_Impl: Sized {
    fn NetworkConnectionConnectivityChanged(&self, connectionid: &::windows_core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows_core::Result<()>;
    fn NetworkConnectionPropertyChanged(&self, connectionid: &::windows_core::GUID, flags: NLM_CONNECTION_PROPERTY_CHANGE) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for INetworkConnectionEvents {}
impl INetworkConnectionEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnectionEvents_Impl, const OFFSET: isize>() -> INetworkConnectionEvents_Vtbl {
        unsafe extern "system" fn NetworkConnectionConnectivityChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnectionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: ::windows_core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NetworkConnectionConnectivityChanged(::core::mem::transmute(&connectionid), ::core::mem::transmute_copy(&newconnectivity)).into()
        }
        unsafe extern "system" fn NetworkConnectionPropertyChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkConnectionEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, connectionid: ::windows_core::GUID, flags: NLM_CONNECTION_PROPERTY_CHANGE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NetworkConnectionPropertyChanged(::core::mem::transmute(&connectionid), ::core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NetworkConnectionConnectivityChanged: NetworkConnectionConnectivityChanged::<Identity, Impl, OFFSET>,
            NetworkConnectionPropertyChanged: NetworkConnectionPropertyChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetworkConnectionEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_Foundation\"`, `\"implement\"`*"]
#[cfg(feature = "Win32_Foundation")]
pub trait INetworkCostManager_Impl: Sized {
    fn GetCost(&self, pcost: *mut u32, pdestipaddr: *const NLM_SOCKADDR) -> ::windows_core::Result<()>;
    fn GetDataPlanStatus(&self, pdataplanstatus: *mut NLM_DATAPLAN_STATUS, pdestipaddr: *const NLM_SOCKADDR) -> ::windows_core::Result<()>;
    fn SetDestinationAddresses(&self, length: u32, pdestipaddrlist: *const NLM_SOCKADDR, bappend: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::Result<()>;
}
#[cfg(feature = "Win32_Foundation")]
impl ::windows_core::RuntimeName for INetworkCostManager {}
#[cfg(feature = "Win32_Foundation")]
impl INetworkCostManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkCostManager_Impl, const OFFSET: isize>() -> INetworkCostManager_Vtbl {
        unsafe extern "system" fn GetCost<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkCostManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pcost: *mut u32, pdestipaddr: *const NLM_SOCKADDR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetCost(::core::mem::transmute_copy(&pcost), ::core::mem::transmute_copy(&pdestipaddr)).into()
        }
        unsafe extern "system" fn GetDataPlanStatus<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkCostManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdataplanstatus: *mut NLM_DATAPLAN_STATUS, pdestipaddr: *const NLM_SOCKADDR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.GetDataPlanStatus(::core::mem::transmute_copy(&pdataplanstatus), ::core::mem::transmute_copy(&pdestipaddr)).into()
        }
        unsafe extern "system" fn SetDestinationAddresses<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkCostManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, length: u32, pdestipaddrlist: *const NLM_SOCKADDR, bappend: super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetDestinationAddresses(::core::mem::transmute_copy(&length), ::core::mem::transmute_copy(&pdestipaddrlist), ::core::mem::transmute_copy(&bappend)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            GetCost: GetCost::<Identity, Impl, OFFSET>,
            GetDataPlanStatus: GetDataPlanStatus::<Identity, Impl, OFFSET>,
            SetDestinationAddresses: SetDestinationAddresses::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetworkCostManager as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"implement\"`*"]
pub trait INetworkCostManagerEvents_Impl: Sized {
    fn CostChanged(&self, newcost: u32, pdestaddr: *const NLM_SOCKADDR) -> ::windows_core::Result<()>;
    fn DataPlanStatusChanged(&self, pdestaddr: *const NLM_SOCKADDR) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for INetworkCostManagerEvents {}
impl INetworkCostManagerEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkCostManagerEvents_Impl, const OFFSET: isize>() -> INetworkCostManagerEvents_Vtbl {
        unsafe extern "system" fn CostChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkCostManagerEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newcost: u32, pdestaddr: *const NLM_SOCKADDR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.CostChanged(::core::mem::transmute_copy(&newcost), ::core::mem::transmute_copy(&pdestaddr)).into()
        }
        unsafe extern "system" fn DataPlanStatusChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkCostManagerEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pdestaddr: *const NLM_SOCKADDR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.DataPlanStatusChanged(::core::mem::transmute_copy(&pdestaddr)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            CostChanged: CostChanged::<Identity, Impl, OFFSET>,
            DataPlanStatusChanged: DataPlanStatusChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetworkCostManagerEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"implement\"`*"]
pub trait INetworkEvents_Impl: Sized {
    fn NetworkAdded(&self, networkid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn NetworkDeleted(&self, networkid: &::windows_core::GUID) -> ::windows_core::Result<()>;
    fn NetworkConnectivityChanged(&self, networkid: &::windows_core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows_core::Result<()>;
    fn NetworkPropertyChanged(&self, networkid: &::windows_core::GUID, flags: NLM_NETWORK_PROPERTY_CHANGE) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for INetworkEvents {}
impl INetworkEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkEvents_Impl, const OFFSET: isize>() -> INetworkEvents_Vtbl {
        unsafe extern "system" fn NetworkAdded<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NetworkAdded(::core::mem::transmute(&networkid)).into()
        }
        unsafe extern "system" fn NetworkDeleted<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkid: ::windows_core::GUID) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NetworkDeleted(::core::mem::transmute(&networkid)).into()
        }
        unsafe extern "system" fn NetworkConnectivityChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkid: ::windows_core::GUID, newconnectivity: NLM_CONNECTIVITY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NetworkConnectivityChanged(::core::mem::transmute(&networkid), ::core::mem::transmute_copy(&newconnectivity)).into()
        }
        unsafe extern "system" fn NetworkPropertyChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, networkid: ::windows_core::GUID, flags: NLM_NETWORK_PROPERTY_CHANGE) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.NetworkPropertyChanged(::core::mem::transmute(&networkid), ::core::mem::transmute_copy(&flags)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            NetworkAdded: NetworkAdded::<Identity, Impl, OFFSET>,
            NetworkDeleted: NetworkDeleted::<Identity, Impl, OFFSET>,
            NetworkConnectivityChanged: NetworkConnectivityChanged::<Identity, Impl, OFFSET>,
            NetworkPropertyChanged: NetworkPropertyChanged::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetworkEvents as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"Win32_Foundation\"`, `\"Win32_System_Com\"`, `\"Win32_System_Ole\"`, `\"Win32_System_Variant\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
pub trait INetworkListManager_Impl: Sized + super::super::System::Com::IDispatch_Impl {
    fn GetNetworks(&self, flags: NLM_ENUM_NETWORK) -> ::windows_core::Result<IEnumNetworks>;
    fn GetNetwork(&self, gdnetworkid: &::windows_core::GUID) -> ::windows_core::Result<INetwork>;
    fn GetNetworkConnections(&self) -> ::windows_core::Result<IEnumNetworkConnections>;
    fn GetNetworkConnection(&self, gdnetworkconnectionid: &::windows_core::GUID) -> ::windows_core::Result<INetworkConnection>;
    fn IsConnectedToInternet(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn IsConnected(&self) -> ::windows_core::Result<super::super::Foundation::VARIANT_BOOL>;
    fn GetConnectivity(&self) -> ::windows_core::Result<NLM_CONNECTIVITY>;
    fn SetSimulatedProfileInfo(&self, psimulatedinfo: *const NLM_SIMULATED_PROFILE_INFO) -> ::windows_core::Result<()>;
    fn ClearSimulatedProfileInfo(&self) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl ::windows_core::RuntimeName for INetworkListManager {}
#[cfg(all(feature = "Win32_Foundation", feature = "Win32_System_Com", feature = "Win32_System_Ole", feature = "Win32_System_Variant"))]
impl INetworkListManager_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: isize>() -> INetworkListManager_Vtbl {
        unsafe extern "system" fn GetNetworks<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, flags: NLM_ENUM_NETWORK, ppenumnetwork: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNetworks(::core::mem::transmute_copy(&flags)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenumnetwork, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetwork<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gdnetworkid: ::windows_core::GUID, ppnetwork: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNetwork(::core::mem::transmute(&gdnetworkid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnetwork, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkConnections<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, ppenum: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNetworkConnections() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppenum, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNetworkConnection<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, gdnetworkconnectionid: ::windows_core::GUID, ppnetworkconnection: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetNetworkConnection(::core::mem::transmute(&gdnetworkconnectionid)) {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(ppnetworkconnection, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnectedToInternet<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsConnectedToInternet() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisconnected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn IsConnected<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pbisconnected: *mut super::super::Foundation::VARIANT_BOOL) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.IsConnected() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pbisconnected, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetConnectivity<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pconnectivity: *mut NLM_CONNECTIVITY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            match this.GetConnectivity() {
                ::core::result::Result::Ok(ok__) => {
                    ::core::ptr::write(pconnectivity, ::core::mem::transmute(ok__));
                    ::windows_core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetSimulatedProfileInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, psimulatedinfo: *const NLM_SIMULATED_PROFILE_INFO) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SetSimulatedProfileInfo(::core::mem::transmute_copy(&psimulatedinfo)).into()
        }
        unsafe extern "system" fn ClearSimulatedProfileInfo<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkListManager_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ClearSimulatedProfileInfo().into()
        }
        Self {
            base__: super::super::System::Com::IDispatch_Vtbl::new::<Identity, Impl, OFFSET>(),
            GetNetworks: GetNetworks::<Identity, Impl, OFFSET>,
            GetNetwork: GetNetwork::<Identity, Impl, OFFSET>,
            GetNetworkConnections: GetNetworkConnections::<Identity, Impl, OFFSET>,
            GetNetworkConnection: GetNetworkConnection::<Identity, Impl, OFFSET>,
            IsConnectedToInternet: IsConnectedToInternet::<Identity, Impl, OFFSET>,
            IsConnected: IsConnected::<Identity, Impl, OFFSET>,
            GetConnectivity: GetConnectivity::<Identity, Impl, OFFSET>,
            SetSimulatedProfileInfo: SetSimulatedProfileInfo::<Identity, Impl, OFFSET>,
            ClearSimulatedProfileInfo: ClearSimulatedProfileInfo::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetworkListManager as ::windows_core::ComInterface>::IID || iid == &<super::super::System::Com::IDispatch as ::windows_core::ComInterface>::IID
    }
}
#[doc = "*Required features: `\"Win32_Networking_NetworkListManager\"`, `\"implement\"`*"]
pub trait INetworkListManagerEvents_Impl: Sized {
    fn ConnectivityChanged(&self, newconnectivity: NLM_CONNECTIVITY) -> ::windows_core::Result<()>;
}
impl ::windows_core::RuntimeName for INetworkListManagerEvents {}
impl INetworkListManagerEvents_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkListManagerEvents_Impl, const OFFSET: isize>() -> INetworkListManagerEvents_Vtbl {
        unsafe extern "system" fn ConnectivityChanged<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: INetworkListManagerEvents_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, newconnectivity: NLM_CONNECTIVITY) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.ConnectivityChanged(::core::mem::transmute_copy(&newconnectivity)).into()
        }
        Self { base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(), ConnectivityChanged: ConnectivityChanged::<Identity, Impl, OFFSET> }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<INetworkListManagerEvents as ::windows_core::ComInterface>::IID
    }
}
