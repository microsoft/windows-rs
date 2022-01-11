#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com"))]
pub trait IXMLGraphBuilderImpl: Sized {
    fn BuildFromXML();
    fn SaveToXML();
    fn BuildFromXMLFile();
}
#[cfg(all(feature = "Win32_Data_Xml_MsXml", feature = "Win32_Foundation", feature = "Win32_System_Com"))]
impl IXMLGraphBuilderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLGraphBuilderImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IXMLGraphBuilderVtbl {
        unsafe extern "system" fn BuildFromXML<Impl: IXMLGraphBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgraph: ::windows::core::RawPtr, pxml: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn SaveToXML<Impl: IXMLGraphBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgraph: ::windows::core::RawPtr, pbstrxml: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        unsafe extern "system" fn BuildFromXMLFile<Impl: IXMLGraphBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgraph: ::windows::core::RawPtr, wszfilename: super::super::super::Foundation::PWSTR, wszbaseurl: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            panic!()
        }
        Self {
            base: ::windows::core::IUnknownVtbl::new::<Identity, BASE_OFFSET>(),
            BuildFromXML: BuildFromXML::<Impl, IMPL_OFFSET>,
            SaveToXML: SaveToXML::<Impl, IMPL_OFFSET>,
            BuildFromXMLFile: BuildFromXMLFile::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IXMLGraphBuilder as ::windows::core::Interface>::IID
    }
}
