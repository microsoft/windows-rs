pub trait IXMLGraphBuilderImpl: Sized {
    fn BuildFromXML();
    fn SaveToXML();
    fn BuildFromXMLFile();
}
impl ::windows::core::RuntimeName for IXMLGraphBuilder {
    const NAME: &'static str = "Windows.Win32.Media.DirectShow.Xml.IXMLGraphBuilder";
}
impl IXMLGraphBuilderVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IXMLGraphBuilderImpl, const OFFSET: isize>() -> IXMLGraphBuilderVtbl {
        unsafe extern "system" fn BuildFromXML<Impl: IXMLGraphBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgraph: ::windows::core::RawPtr, pxml: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BuildFromXML(&*(&pgraph as *const <super::IGraphBuilder as ::windows::core::Abi>::Abi as *const <super::IGraphBuilder as ::windows::core::DefaultType>::DefaultType), &*(&pxml as *const <super::super::super::Data::Xml::MsXml::IXMLElement as ::windows::core::Abi>::Abi as *const <super::super::super::Data::Xml::MsXml::IXMLElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SaveToXML<Impl: IXMLGraphBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgraph: ::windows::core::RawPtr, pbstrxml: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).SaveToXML(&*(&pgraph as *const <super::IGraphBuilder as ::windows::core::Abi>::Abi as *const <super::IGraphBuilder as ::windows::core::DefaultType>::DefaultType), &*(&pbstrxml as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn BuildFromXMLFile<Impl: IXMLGraphBuilderImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, pgraph: ::windows::core::RawPtr, wszfilename: super::super::super::Foundation::PWSTR, wszbaseurl: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).BuildFromXMLFile(
                &*(&pgraph as *const <super::IGraphBuilder as ::windows::core::Abi>::Abi as *const <super::IGraphBuilder as ::windows::core::DefaultType>::DefaultType),
                &*(&wszfilename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&wszbaseurl as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(::windows::core::QueryInterface::<Identity, OFFSET>, ::windows::core::AddRef::<Identity, OFFSET>, ::windows::core::Release::<Identity, OFFSET>, ::windows::core::GetIids, ::windows::core::GetRuntimeClassName::<IXMLGraphBuilder>, ::windows::core::GetTrustLevel, BuildFromXML::<Impl, OFFSET>, SaveToXML::<Impl, OFFSET>, BuildFromXMLFile::<Impl, OFFSET>)
    }
}
