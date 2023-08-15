#[doc = "*Required features: `\"Win32_Media_DirectShow_Xml\"`, `\"Win32_Data_Xml_MsXml\"`, `\"Win32_System_Com\"`, `\"implement\"`*"]
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
pub trait IXMLGraphBuilder_Impl: Sized {
    fn BuildFromXML(&self, pgraph: ::core::option::Option<&super::IGraphBuilder>, pxml: ::core::option::Option<&super::super::super::Data::Xml::MsXml::IXMLElement>) -> ::windows_core::Result<()>;
    fn SaveToXML(&self, pgraph: ::core::option::Option<&super::IGraphBuilder>, pbstrxml: *mut ::windows_core::BSTR) -> ::windows_core::Result<()>;
    fn BuildFromXMLFile(&self, pgraph: ::core::option::Option<&super::IGraphBuilder>, wszfilename: &::windows_core::PCWSTR, wszbaseurl: &::windows_core::PCWSTR) -> ::windows_core::Result<()>;
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl ::windows_core::RuntimeName for IXMLGraphBuilder {}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_System_Com"))]
impl IXMLGraphBuilder_Vtbl {
    pub const fn new<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLGraphBuilder_Impl, const OFFSET: isize>() -> IXMLGraphBuilder_Vtbl {
        unsafe extern "system" fn BuildFromXML<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLGraphBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgraph: *mut ::core::ffi::c_void, pxml: *mut ::core::ffi::c_void) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BuildFromXML(::windows_core::from_raw_borrowed(&pgraph), ::windows_core::from_raw_borrowed(&pxml)).into()
        }
        unsafe extern "system" fn SaveToXML<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLGraphBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgraph: *mut ::core::ffi::c_void, pbstrxml: *mut ::std::mem::MaybeUninit<::windows_core::BSTR>) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.SaveToXML(::windows_core::from_raw_borrowed(&pgraph), ::core::mem::transmute_copy(&pbstrxml)).into()
        }
        unsafe extern "system" fn BuildFromXMLFile<Identity: ::windows_core::IUnknownImpl<Impl = Impl>, Impl: IXMLGraphBuilder_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgraph: *mut ::core::ffi::c_void, wszfilename: ::windows_core::PCWSTR, wszbaseurl: ::windows_core::PCWSTR) -> ::windows_core::HRESULT {
            let this = (this as *const *const ()).offset(OFFSET) as *const Identity;
            let this = (*this).get_impl();
            this.BuildFromXMLFile(::windows_core::from_raw_borrowed(&pgraph), ::core::mem::transmute(&wszfilename), ::core::mem::transmute(&wszbaseurl)).into()
        }
        Self {
            base__: ::windows_core::IUnknown_Vtbl::new::<Identity, OFFSET>(),
            BuildFromXML: BuildFromXML::<Identity, Impl, OFFSET>,
            SaveToXML: SaveToXML::<Identity, Impl, OFFSET>,
            BuildFromXMLFile: BuildFromXMLFile::<Identity, Impl, OFFSET>,
        }
    }
    pub fn matches(iid: &::windows_core::GUID) -> bool {
        iid == &<IXMLGraphBuilder as ::windows_core::ComInterface>::IID
    }
}
