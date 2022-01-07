#[cfg(feature = "Win32_System_Com")]
pub trait IMXAttributesImpl: Sized + IDispatchImpl {
    fn addAttribute();
    fn addAttributeFromIndex();
    fn clear();
    fn removeAttribute();
    fn setAttribute();
    fn setAttributes();
    fn setLocalName();
    fn setQName();
    fn setType();
    fn setURI();
    fn setValue();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMXAttributes {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IMXAttributes";
}
#[cfg(feature = "Win32_System_Com")]
impl IMXAttributesVtbl {
    pub const fn new<Impl: IMXAttributesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMXAttributesVtbl {
        unsafe extern "system" fn addAttribute<Impl: IMXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, struri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strlocalname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strqname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).addAttribute(
                &*(&struri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strlocalname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strqname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strtype as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strvalue as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addAttributeFromIndex<Impl: IMXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varatts: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, nindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).addAttributeFromIndex(&*(&varatts as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), nindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn clear<Impl: IMXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).clear() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeAttribute<Impl: IMXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).removeAttribute(nindex) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttribute<Impl: IMXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, struri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strlocalname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strqname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).setAttribute(
                nindex,
                &*(&struri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strlocalname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strqname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strtype as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strvalue as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttributes<Impl: IMXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varatts: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).setAttributes(&*(&varatts as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setLocalName<Impl: IMXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, strlocalname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).setLocalName(nindex, &*(&strlocalname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setQName<Impl: IMXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, strqname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).setQName(nindex, &*(&strqname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setType<Impl: IMXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, strtype: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).setType(nindex, &*(&strtype as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setURI<Impl: IMXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, struri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).setURI(nindex, &*(&struri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setValue<Impl: IMXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, strvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).setValue(nindex, &*(&strvalue as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMXAttributes>, base.5, addAttribute::<Impl, OFFSET>, addAttributeFromIndex::<Impl, OFFSET>, clear::<Impl, OFFSET>, removeAttribute::<Impl, OFFSET>, setAttribute::<Impl, OFFSET>, setAttributes::<Impl, OFFSET>, setLocalName::<Impl, OFFSET>, setQName::<Impl, OFFSET>, setType::<Impl, OFFSET>, setURI::<Impl, OFFSET>, setValue::<Impl, OFFSET>)
    }
}
pub trait IMXNamespaceManagerImpl: Sized {
    fn putAllowOverride();
    fn getAllowOverride();
    fn reset();
    fn pushContext();
    fn pushNodeContext();
    fn popContext();
    fn declarePrefix();
    fn getDeclaredPrefix();
    fn getPrefix();
    fn getURI();
}
impl ::windows::core::RuntimeName for IMXNamespaceManager {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IMXNamespaceManager";
}
impl IMXNamespaceManagerVtbl {
    pub const fn new<Impl: IMXNamespaceManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMXNamespaceManagerVtbl {
        unsafe extern "system" fn putAllowOverride<Impl: IMXNamespaceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, foverride: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putAllowOverride(foverride) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAllowOverride<Impl: IMXNamespaceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, foverride: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getAllowOverride(::core::mem::transmute_copy(&foverride)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Impl: IMXNamespaceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn pushContext<Impl: IMXNamespaceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).pushContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn pushNodeContext<Impl: IMXNamespaceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contextnode: ::windows::core::RawPtr, fdeep: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).pushNodeContext(&*(&contextnode as *const <IXMLDOMNode as ::windows::core::Abi>::Abi as *const <IXMLDOMNode as ::windows::core::DefaultType>::DefaultType), fdeep) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn popContext<Impl: IMXNamespaceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).popContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn declarePrefix<Impl: IMXNamespaceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prefix: super::super::super::Foundation::PWSTR, namespaceuri: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).declarePrefix(&*(&prefix as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&namespaceuri as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getDeclaredPrefix<Impl: IMXNamespaceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, pwchprefix: super::super::super::Foundation::PWSTR, pcchprefix: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getDeclaredPrefix(nindex, ::core::mem::transmute_copy(&pwchprefix), pcchprefix) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getPrefix<Impl: IMXNamespaceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwsznamespaceuri: super::super::super::Foundation::PWSTR, nindex: i32, pwchprefix: super::super::super::Foundation::PWSTR, pcchprefix: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getPrefix(&*(&pwsznamespaceuri as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), nindex, ::core::mem::transmute_copy(&pwchprefix), pcchprefix) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getURI<Impl: IMXNamespaceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchprefix: super::super::super::Foundation::PWSTR, pcontextnode: ::windows::core::RawPtr, pwchuri: super::super::super::Foundation::PWSTR, pcchuri: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getURI(&*(&pwchprefix as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pcontextnode as *const <IXMLDOMNode as ::windows::core::Abi>::Abi as *const <IXMLDOMNode as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pwchuri), pcchuri) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMXNamespaceManager>, base.5, putAllowOverride::<Impl, OFFSET>, getAllowOverride::<Impl, OFFSET>, reset::<Impl, OFFSET>, pushContext::<Impl, OFFSET>, pushNodeContext::<Impl, OFFSET>, popContext::<Impl, OFFSET>, declarePrefix::<Impl, OFFSET>, getDeclaredPrefix::<Impl, OFFSET>, getPrefix::<Impl, OFFSET>, getURI::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMXNamespacePrefixesImpl: Sized + IDispatchImpl {
    fn item();
    fn length();
    fn _newEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMXNamespacePrefixes {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IMXNamespacePrefixes";
}
#[cfg(feature = "Win32_System_Com")]
impl IMXNamespacePrefixesVtbl {
    pub const fn new<Impl: IMXNamespacePrefixesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMXNamespacePrefixesVtbl {
        unsafe extern "system" fn item<Impl: IMXNamespacePrefixesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, prefix: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).item(index, ::core::mem::transmute_copy(&prefix)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Impl: IMXNamespacePrefixesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).length(::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Impl: IMXNamespacePrefixesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._newEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMXNamespacePrefixes>, base.5, item::<Impl, OFFSET>, length::<Impl, OFFSET>, _newEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMXReaderControlImpl: Sized + IDispatchImpl {
    fn abort();
    fn resume();
    fn suspend();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMXReaderControl {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IMXReaderControl";
}
#[cfg(feature = "Win32_System_Com")]
impl IMXReaderControlVtbl {
    pub const fn new<Impl: IMXReaderControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMXReaderControlVtbl {
        unsafe extern "system" fn abort<Impl: IMXReaderControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).abort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn resume<Impl: IMXReaderControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).resume() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn suspend<Impl: IMXReaderControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).suspend() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMXReaderControl>, base.5, abort::<Impl, OFFSET>, resume::<Impl, OFFSET>, suspend::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMXSchemaDeclHandlerImpl: Sized + IDispatchImpl {
    fn schemaElementDecl();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMXSchemaDeclHandler {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IMXSchemaDeclHandler";
}
#[cfg(feature = "Win32_System_Com")]
impl IMXSchemaDeclHandlerVtbl {
    pub const fn new<Impl: IMXSchemaDeclHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMXSchemaDeclHandlerVtbl {
        unsafe extern "system" fn schemaElementDecl<Impl: IMXSchemaDeclHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oschemaelement: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).schemaElementDecl(&*(&oschemaelement as *const <ISchemaElement as ::windows::core::Abi>::Abi as *const <ISchemaElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMXSchemaDeclHandler>, base.5, schemaElementDecl::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMXWriterImpl: Sized + IDispatchImpl {
    fn Setoutput();
    fn output();
    fn Setencoding();
    fn encoding();
    fn SetbyteOrderMark();
    fn byteOrderMark();
    fn Setindent();
    fn indent();
    fn Setstandalone();
    fn standalone();
    fn SetomitXMLDeclaration();
    fn omitXMLDeclaration();
    fn Setversion();
    fn version();
    fn SetdisableOutputEscaping();
    fn disableOutputEscaping();
    fn flush();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMXWriter {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IMXWriter";
}
#[cfg(feature = "Win32_System_Com")]
impl IMXWriterVtbl {
    pub const fn new<Impl: IMXWriterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMXWriterVtbl {
        unsafe extern "system" fn Setoutput<Impl: IMXWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vardestination: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Setoutput(&*(&vardestination as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn output<Impl: IMXWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vardestination: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).output(::core::mem::transmute_copy(&vardestination)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setencoding<Impl: IMXWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strencoding: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Setencoding(&*(&strencoding as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn encoding<Impl: IMXWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strencoding: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).encoding(::core::mem::transmute_copy(&strencoding)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetbyteOrderMark<Impl: IMXWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fwritebyteordermark: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetbyteOrderMark(fwritebyteordermark) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn byteOrderMark<Impl: IMXWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fwritebyteordermark: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).byteOrderMark(::core::mem::transmute_copy(&fwritebyteordermark)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setindent<Impl: IMXWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, findentmode: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Setindent(findentmode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn indent<Impl: IMXWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, findentmode: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).indent(::core::mem::transmute_copy(&findentmode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setstandalone<Impl: IMXWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Setstandalone(fvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn standalone<Impl: IMXWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).standalone(::core::mem::transmute_copy(&fvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetomitXMLDeclaration<Impl: IMXWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetomitXMLDeclaration(fvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn omitXMLDeclaration<Impl: IMXWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).omitXMLDeclaration(::core::mem::transmute_copy(&fvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setversion<Impl: IMXWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strversion: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Setversion(&*(&strversion as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn version<Impl: IMXWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strversion: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).version(::core::mem::transmute_copy(&strversion)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetdisableOutputEscaping<Impl: IMXWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetdisableOutputEscaping(fvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn disableOutputEscaping<Impl: IMXWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).disableOutputEscaping(::core::mem::transmute_copy(&fvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn flush<Impl: IMXWriterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).flush() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IMXWriter>,
            base.5,
            Setoutput::<Impl, OFFSET>,
            output::<Impl, OFFSET>,
            Setencoding::<Impl, OFFSET>,
            encoding::<Impl, OFFSET>,
            SetbyteOrderMark::<Impl, OFFSET>,
            byteOrderMark::<Impl, OFFSET>,
            Setindent::<Impl, OFFSET>,
            indent::<Impl, OFFSET>,
            Setstandalone::<Impl, OFFSET>,
            standalone::<Impl, OFFSET>,
            SetomitXMLDeclaration::<Impl, OFFSET>,
            omitXMLDeclaration::<Impl, OFFSET>,
            Setversion::<Impl, OFFSET>,
            version::<Impl, OFFSET>,
            SetdisableOutputEscaping::<Impl, OFFSET>,
            disableOutputEscaping::<Impl, OFFSET>,
            flush::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IMXXMLFilterImpl: Sized + IDispatchImpl {
    fn getFeature();
    fn putFeature();
    fn getProperty();
    fn putProperty();
    fn entityResolver();
    fn putref_entityResolver();
    fn contentHandler();
    fn putref_contentHandler();
    fn dtdHandler();
    fn putref_dtdHandler();
    fn errorHandler();
    fn putref_errorHandler();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IMXXMLFilter {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IMXXMLFilter";
}
#[cfg(feature = "Win32_System_Com")]
impl IMXXMLFilterVtbl {
    pub const fn new<Impl: IMXXMLFilterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IMXXMLFilterVtbl {
        unsafe extern "system" fn getFeature<Impl: IMXXMLFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, fvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getFeature(&*(&strname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&fvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putFeature<Impl: IMXXMLFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, fvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putFeature(&*(&strname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), fvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getProperty<Impl: IMXXMLFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getProperty(&*(&strname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&varvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putProperty<Impl: IMXXMLFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varvalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putProperty(&*(&strname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varvalue as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn entityResolver<Impl: IMXXMLFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oresolver: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).entityResolver(::core::mem::transmute_copy(&oresolver)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_entityResolver<Impl: IMXXMLFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oresolver: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putref_entityResolver(&*(&oresolver as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn contentHandler<Impl: IMXXMLFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).contentHandler(::core::mem::transmute_copy(&ohandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_contentHandler<Impl: IMXXMLFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putref_contentHandler(&*(&ohandler as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn dtdHandler<Impl: IMXXMLFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).dtdHandler(::core::mem::transmute_copy(&ohandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_dtdHandler<Impl: IMXXMLFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putref_dtdHandler(&*(&ohandler as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn errorHandler<Impl: IMXXMLFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ohandler: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).errorHandler(::core::mem::transmute_copy(&ohandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_errorHandler<Impl: IMXXMLFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putref_errorHandler(&*(&ohandler as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IMXXMLFilter>, base.5, getFeature::<Impl, OFFSET>, putFeature::<Impl, OFFSET>, getProperty::<Impl, OFFSET>, putProperty::<Impl, OFFSET>, entityResolver::<Impl, OFFSET>, putref_entityResolver::<Impl, OFFSET>, contentHandler::<Impl, OFFSET>, putref_contentHandler::<Impl, OFFSET>, dtdHandler::<Impl, OFFSET>, putref_dtdHandler::<Impl, OFFSET>, errorHandler::<Impl, OFFSET>, putref_errorHandler::<Impl, OFFSET>)
    }
}
pub trait ISAXAttributesImpl: Sized {
    fn getLength();
    fn getURI();
    fn getLocalName();
    fn getQName();
    fn getName();
    fn getIndexFromName();
    fn getIndexFromQName();
    fn getType();
    fn getTypeFromName();
    fn getTypeFromQName();
    fn getValue();
    fn getValueFromName();
    fn getValueFromQName();
}
impl ::windows::core::RuntimeName for ISAXAttributes {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.ISAXAttributes";
}
impl ISAXAttributesVtbl {
    pub const fn new<Impl: ISAXAttributesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISAXAttributesVtbl {
        unsafe extern "system" fn getLength<Impl: ISAXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getLength(::core::mem::transmute_copy(&pnlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getURI<Impl: ISAXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getURI(nindex, ::core::mem::transmute_copy(&ppwchuri), ::core::mem::transmute_copy(&pcchuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getLocalName<Impl: ISAXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getLocalName(nindex, ::core::mem::transmute_copy(&ppwchlocalname), ::core::mem::transmute_copy(&pcchlocalname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getQName<Impl: ISAXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getQName(nindex, ::core::mem::transmute_copy(&ppwchqname), ::core::mem::transmute_copy(&pcchqname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getName<Impl: ISAXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchuri: *mut *mut u16, pcchuri: *mut i32, ppwchlocalname: *mut *mut u16, pcchlocalname: *mut i32, ppwchqname: *mut *mut u16, pcchqname: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getName(nindex, ::core::mem::transmute_copy(&ppwchuri), ::core::mem::transmute_copy(&pcchuri), ::core::mem::transmute_copy(&ppwchlocalname), ::core::mem::transmute_copy(&pcchlocalname), ::core::mem::transmute_copy(&ppwchqname), ::core::mem::transmute_copy(&pcchqname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getIndexFromName<Impl: ISAXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchuri: super::super::super::Foundation::PWSTR, cchuri: i32, pwchlocalname: super::super::super::Foundation::PWSTR, cchlocalname: i32, pnindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getIndexFromName(&*(&pwchuri as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchuri, &*(&pwchlocalname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchlocalname, ::core::mem::transmute_copy(&pnindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getIndexFromQName<Impl: ISAXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchqname: super::super::super::Foundation::PWSTR, cchqname: i32, pnindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getIndexFromQName(&*(&pwchqname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchqname, ::core::mem::transmute_copy(&pnindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getType<Impl: ISAXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getType(nindex, ::core::mem::transmute_copy(&ppwchtype), ::core::mem::transmute_copy(&pcchtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getTypeFromName<Impl: ISAXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchuri: super::super::super::Foundation::PWSTR, cchuri: i32, pwchlocalname: super::super::super::Foundation::PWSTR, cchlocalname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getTypeFromName(
                &*(&pwchuri as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchuri,
                &*(&pwchlocalname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchlocalname,
                ::core::mem::transmute_copy(&ppwchtype),
                ::core::mem::transmute_copy(&pcchtype),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getTypeFromQName<Impl: ISAXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchqname: super::super::super::Foundation::PWSTR, cchqname: i32, ppwchtype: *mut *mut u16, pcchtype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getTypeFromQName(&*(&pwchqname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchqname, ::core::mem::transmute_copy(&ppwchtype), ::core::mem::transmute_copy(&pcchtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getValue<Impl: ISAXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getValue(nindex, ::core::mem::transmute_copy(&ppwchvalue), ::core::mem::transmute_copy(&pcchvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getValueFromName<Impl: ISAXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchuri: super::super::super::Foundation::PWSTR, cchuri: i32, pwchlocalname: super::super::super::Foundation::PWSTR, cchlocalname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getValueFromName(
                &*(&pwchuri as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchuri,
                &*(&pwchlocalname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchlocalname,
                ::core::mem::transmute_copy(&ppwchvalue),
                ::core::mem::transmute_copy(&pcchvalue),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getValueFromQName<Impl: ISAXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchqname: super::super::super::Foundation::PWSTR, cchqname: i32, ppwchvalue: *mut *mut u16, pcchvalue: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getValueFromQName(&*(&pwchqname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchqname, ::core::mem::transmute_copy(&ppwchvalue), ::core::mem::transmute_copy(&pcchvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISAXAttributes>,
            base.5,
            getLength::<Impl, OFFSET>,
            getURI::<Impl, OFFSET>,
            getLocalName::<Impl, OFFSET>,
            getQName::<Impl, OFFSET>,
            getName::<Impl, OFFSET>,
            getIndexFromName::<Impl, OFFSET>,
            getIndexFromQName::<Impl, OFFSET>,
            getType::<Impl, OFFSET>,
            getTypeFromName::<Impl, OFFSET>,
            getTypeFromQName::<Impl, OFFSET>,
            getValue::<Impl, OFFSET>,
            getValueFromName::<Impl, OFFSET>,
            getValueFromQName::<Impl, OFFSET>,
        )
    }
}
pub trait ISAXContentHandlerImpl: Sized {
    fn putDocumentLocator();
    fn startDocument();
    fn endDocument();
    fn startPrefixMapping();
    fn endPrefixMapping();
    fn startElement();
    fn endElement();
    fn characters();
    fn ignorableWhitespace();
    fn processingInstruction();
    fn skippedEntity();
}
impl ::windows::core::RuntimeName for ISAXContentHandler {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.ISAXContentHandler";
}
impl ISAXContentHandlerVtbl {
    pub const fn new<Impl: ISAXContentHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISAXContentHandlerVtbl {
        unsafe extern "system" fn putDocumentLocator<Impl: ISAXContentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plocator: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putDocumentLocator(&*(&plocator as *const <ISAXLocator as ::windows::core::Abi>::Abi as *const <ISAXLocator as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn startDocument<Impl: ISAXContentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).startDocument() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn endDocument<Impl: ISAXContentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).endDocument() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn startPrefixMapping<Impl: ISAXContentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchprefix: super::super::super::Foundation::PWSTR, cchprefix: i32, pwchuri: super::super::super::Foundation::PWSTR, cchuri: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).startPrefixMapping(&*(&pwchprefix as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchprefix, &*(&pwchuri as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchuri) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn endPrefixMapping<Impl: ISAXContentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchprefix: super::super::super::Foundation::PWSTR, cchprefix: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).endPrefixMapping(&*(&pwchprefix as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchprefix) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn startElement<Impl: ISAXContentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchnamespaceuri: super::super::super::Foundation::PWSTR, cchnamespaceuri: i32, pwchlocalname: super::super::super::Foundation::PWSTR, cchlocalname: i32, pwchqname: super::super::super::Foundation::PWSTR, cchqname: i32, pattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).startElement(
                &*(&pwchnamespaceuri as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchnamespaceuri,
                &*(&pwchlocalname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchlocalname,
                &*(&pwchqname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchqname,
                &*(&pattributes as *const <ISAXAttributes as ::windows::core::Abi>::Abi as *const <ISAXAttributes as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn endElement<Impl: ISAXContentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchnamespaceuri: super::super::super::Foundation::PWSTR, cchnamespaceuri: i32, pwchlocalname: super::super::super::Foundation::PWSTR, cchlocalname: i32, pwchqname: super::super::super::Foundation::PWSTR, cchqname: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).endElement(
                &*(&pwchnamespaceuri as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchnamespaceuri,
                &*(&pwchlocalname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchlocalname,
                &*(&pwchqname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchqname,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn characters<Impl: ISAXContentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchchars: super::super::super::Foundation::PWSTR, cchchars: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).characters(&*(&pwchchars as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchchars) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ignorableWhitespace<Impl: ISAXContentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchchars: super::super::super::Foundation::PWSTR, cchchars: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ignorableWhitespace(&*(&pwchchars as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchchars) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn processingInstruction<Impl: ISAXContentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchtarget: super::super::super::Foundation::PWSTR, cchtarget: i32, pwchdata: super::super::super::Foundation::PWSTR, cchdata: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).processingInstruction(&*(&pwchtarget as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchtarget, &*(&pwchdata as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchdata) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn skippedEntity<Impl: ISAXContentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).skippedEntity(&*(&pwchname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchname) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISAXContentHandler>, base.5, putDocumentLocator::<Impl, OFFSET>, startDocument::<Impl, OFFSET>, endDocument::<Impl, OFFSET>, startPrefixMapping::<Impl, OFFSET>, endPrefixMapping::<Impl, OFFSET>, startElement::<Impl, OFFSET>, endElement::<Impl, OFFSET>, characters::<Impl, OFFSET>, ignorableWhitespace::<Impl, OFFSET>, processingInstruction::<Impl, OFFSET>, skippedEntity::<Impl, OFFSET>)
    }
}
pub trait ISAXDTDHandlerImpl: Sized {
    fn notationDecl();
    fn unparsedEntityDecl();
}
impl ::windows::core::RuntimeName for ISAXDTDHandler {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.ISAXDTDHandler";
}
impl ISAXDTDHandlerVtbl {
    pub const fn new<Impl: ISAXDTDHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISAXDTDHandlerVtbl {
        unsafe extern "system" fn notationDecl<Impl: ISAXDTDHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32, pwchpublicid: super::super::super::Foundation::PWSTR, cchpublicid: i32, pwchsystemid: super::super::super::Foundation::PWSTR, cchsystemid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).notationDecl(
                &*(&pwchname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchname,
                &*(&pwchpublicid as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchpublicid,
                &*(&pwchsystemid as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchsystemid,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn unparsedEntityDecl<Impl: ISAXDTDHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32, pwchpublicid: super::super::super::Foundation::PWSTR, cchpublicid: i32, pwchsystemid: super::super::super::Foundation::PWSTR, cchsystemid: i32, pwchnotationname: super::super::super::Foundation::PWSTR, cchnotationname: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).unparsedEntityDecl(
                &*(&pwchname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchname,
                &*(&pwchpublicid as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchpublicid,
                &*(&pwchsystemid as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchsystemid,
                &*(&pwchnotationname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchnotationname,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISAXDTDHandler>, base.5, notationDecl::<Impl, OFFSET>, unparsedEntityDecl::<Impl, OFFSET>)
    }
}
pub trait ISAXDeclHandlerImpl: Sized {
    fn elementDecl();
    fn attributeDecl();
    fn internalEntityDecl();
    fn externalEntityDecl();
}
impl ::windows::core::RuntimeName for ISAXDeclHandler {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.ISAXDeclHandler";
}
impl ISAXDeclHandlerVtbl {
    pub const fn new<Impl: ISAXDeclHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISAXDeclHandlerVtbl {
        unsafe extern "system" fn elementDecl<Impl: ISAXDeclHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32, pwchmodel: super::super::super::Foundation::PWSTR, cchmodel: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).elementDecl(&*(&pwchname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchname, &*(&pwchmodel as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchmodel) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributeDecl<Impl: ISAXDeclHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchelementname: super::super::super::Foundation::PWSTR, cchelementname: i32, pwchattributename: super::super::super::Foundation::PWSTR, cchattributename: i32, pwchtype: super::super::super::Foundation::PWSTR, cchtype: i32, pwchvaluedefault: super::super::super::Foundation::PWSTR, cchvaluedefault: i32, pwchvalue: super::super::super::Foundation::PWSTR, cchvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).attributeDecl(
                &*(&pwchelementname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchelementname,
                &*(&pwchattributename as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchattributename,
                &*(&pwchtype as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchtype,
                &*(&pwchvaluedefault as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchvaluedefault,
                &*(&pwchvalue as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchvalue,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn internalEntityDecl<Impl: ISAXDeclHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32, pwchvalue: super::super::super::Foundation::PWSTR, cchvalue: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).internalEntityDecl(&*(&pwchname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchname, &*(&pwchvalue as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn externalEntityDecl<Impl: ISAXDeclHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32, pwchpublicid: super::super::super::Foundation::PWSTR, cchpublicid: i32, pwchsystemid: super::super::super::Foundation::PWSTR, cchsystemid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).externalEntityDecl(
                &*(&pwchname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchname,
                &*(&pwchpublicid as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchpublicid,
                &*(&pwchsystemid as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchsystemid,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISAXDeclHandler>, base.5, elementDecl::<Impl, OFFSET>, attributeDecl::<Impl, OFFSET>, internalEntityDecl::<Impl, OFFSET>, externalEntityDecl::<Impl, OFFSET>)
    }
}
pub trait ISAXEntityResolverImpl: Sized {
    fn resolveEntity();
}
impl ::windows::core::RuntimeName for ISAXEntityResolver {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.ISAXEntityResolver";
}
impl ISAXEntityResolverVtbl {
    pub const fn new<Impl: ISAXEntityResolverImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISAXEntityResolverVtbl {
        unsafe extern "system" fn resolveEntity<Impl: ISAXEntityResolverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchpublicid: super::super::super::Foundation::PWSTR, pwchsystemid: super::super::super::Foundation::PWSTR, pvarinput: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).resolveEntity(&*(&pwchpublicid as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pwchsystemid as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarinput)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISAXEntityResolver>, base.5, resolveEntity::<Impl, OFFSET>)
    }
}
pub trait ISAXErrorHandlerImpl: Sized {
    fn error();
    fn fatalError();
    fn ignorableWarning();
}
impl ::windows::core::RuntimeName for ISAXErrorHandler {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.ISAXErrorHandler";
}
impl ISAXErrorHandlerVtbl {
    pub const fn new<Impl: ISAXErrorHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISAXErrorHandlerVtbl {
        unsafe extern "system" fn error<Impl: ISAXErrorHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plocator: ::windows::core::RawPtr, pwcherrormessage: super::super::super::Foundation::PWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).error(&*(&plocator as *const <ISAXLocator as ::windows::core::Abi>::Abi as *const <ISAXLocator as ::windows::core::DefaultType>::DefaultType), &*(&pwcherrormessage as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), hrerrorcode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fatalError<Impl: ISAXErrorHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plocator: ::windows::core::RawPtr, pwcherrormessage: super::super::super::Foundation::PWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).fatalError(&*(&plocator as *const <ISAXLocator as ::windows::core::Abi>::Abi as *const <ISAXLocator as ::windows::core::DefaultType>::DefaultType), &*(&pwcherrormessage as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), hrerrorcode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ignorableWarning<Impl: ISAXErrorHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plocator: ::windows::core::RawPtr, pwcherrormessage: super::super::super::Foundation::PWSTR, hrerrorcode: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ignorableWarning(&*(&plocator as *const <ISAXLocator as ::windows::core::Abi>::Abi as *const <ISAXLocator as ::windows::core::DefaultType>::DefaultType), &*(&pwcherrormessage as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), hrerrorcode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISAXErrorHandler>, base.5, error::<Impl, OFFSET>, fatalError::<Impl, OFFSET>, ignorableWarning::<Impl, OFFSET>)
    }
}
pub trait ISAXLexicalHandlerImpl: Sized {
    fn startDTD();
    fn endDTD();
    fn startEntity();
    fn endEntity();
    fn startCDATA();
    fn endCDATA();
    fn comment();
}
impl ::windows::core::RuntimeName for ISAXLexicalHandler {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.ISAXLexicalHandler";
}
impl ISAXLexicalHandlerVtbl {
    pub const fn new<Impl: ISAXLexicalHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISAXLexicalHandlerVtbl {
        unsafe extern "system" fn startDTD<Impl: ISAXLexicalHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32, pwchpublicid: super::super::super::Foundation::PWSTR, cchpublicid: i32, pwchsystemid: super::super::super::Foundation::PWSTR, cchsystemid: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).startDTD(
                &*(&pwchname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchname,
                &*(&pwchpublicid as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchpublicid,
                &*(&pwchsystemid as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                cchsystemid,
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn endDTD<Impl: ISAXLexicalHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).endDTD() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn startEntity<Impl: ISAXLexicalHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).startEntity(&*(&pwchname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchname) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn endEntity<Impl: ISAXLexicalHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, cchname: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).endEntity(&*(&pwchname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchname) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn startCDATA<Impl: ISAXLexicalHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).startCDATA() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn endCDATA<Impl: ISAXLexicalHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).endCDATA() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn comment<Impl: ISAXLexicalHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchchars: super::super::super::Foundation::PWSTR, cchchars: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).comment(&*(&pwchchars as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), cchchars) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISAXLexicalHandler>, base.5, startDTD::<Impl, OFFSET>, endDTD::<Impl, OFFSET>, startEntity::<Impl, OFFSET>, endEntity::<Impl, OFFSET>, startCDATA::<Impl, OFFSET>, endCDATA::<Impl, OFFSET>, comment::<Impl, OFFSET>)
    }
}
pub trait ISAXLocatorImpl: Sized {
    fn getColumnNumber();
    fn getLineNumber();
    fn getPublicId();
    fn getSystemId();
}
impl ::windows::core::RuntimeName for ISAXLocator {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.ISAXLocator";
}
impl ISAXLocatorVtbl {
    pub const fn new<Impl: ISAXLocatorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISAXLocatorVtbl {
        unsafe extern "system" fn getColumnNumber<Impl: ISAXLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pncolumn: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getColumnNumber(::core::mem::transmute_copy(&pncolumn)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getLineNumber<Impl: ISAXLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnline: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getLineNumber(::core::mem::transmute_copy(&pnline)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getPublicId<Impl: ISAXLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwchpublicid: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getPublicId(::core::mem::transmute_copy(&ppwchpublicid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getSystemId<Impl: ISAXLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwchsystemid: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getSystemId(::core::mem::transmute_copy(&ppwchsystemid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISAXLocator>, base.5, getColumnNumber::<Impl, OFFSET>, getLineNumber::<Impl, OFFSET>, getPublicId::<Impl, OFFSET>, getSystemId::<Impl, OFFSET>)
    }
}
pub trait ISAXXMLFilterImpl: Sized + ISAXXMLReaderImpl {
    fn getParent();
    fn putParent();
}
impl ::windows::core::RuntimeName for ISAXXMLFilter {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.ISAXXMLFilter";
}
impl ISAXXMLFilterVtbl {
    pub const fn new<Impl: ISAXXMLFilterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISAXXMLFilterVtbl {
        unsafe extern "system" fn getParent<Impl: ISAXXMLFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getParent(::core::mem::transmute_copy(&ppreader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putParent<Impl: ISAXXMLFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putParent(&*(&preader as *const <ISAXXMLReader as ::windows::core::Abi>::Abi as *const <ISAXXMLReader as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISAXXMLFilter>, base.5, getParent::<Impl, OFFSET>, putParent::<Impl, OFFSET>)
    }
}
pub trait ISAXXMLReaderImpl: Sized {
    fn getFeature();
    fn putFeature();
    fn getProperty();
    fn putProperty();
    fn getEntityResolver();
    fn putEntityResolver();
    fn getContentHandler();
    fn putContentHandler();
    fn getDTDHandler();
    fn putDTDHandler();
    fn getErrorHandler();
    fn putErrorHandler();
    fn getBaseURL();
    fn putBaseURL();
    fn getSecureBaseURL();
    fn putSecureBaseURL();
    fn parse();
    fn parseURL();
}
impl ::windows::core::RuntimeName for ISAXXMLReader {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.ISAXXMLReader";
}
impl ISAXXMLReaderVtbl {
    pub const fn new<Impl: ISAXXMLReaderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISAXXMLReaderVtbl {
        unsafe extern "system" fn getFeature<Impl: ISAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, pvfvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getFeature(&*(&pwchname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvfvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putFeature<Impl: ISAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, vfvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putFeature(&*(&pwchname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), vfvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getProperty<Impl: ISAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, pvarvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getProperty(&*(&pwchname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pvarvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putProperty<Impl: ISAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchname: super::super::super::Foundation::PWSTR, varvalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putProperty(&*(&pwchname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&varvalue as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getEntityResolver<Impl: ISAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppresolver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getEntityResolver(::core::mem::transmute_copy(&ppresolver)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putEntityResolver<Impl: ISAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, presolver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putEntityResolver(&*(&presolver as *const <ISAXEntityResolver as ::windows::core::Abi>::Abi as *const <ISAXEntityResolver as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getContentHandler<Impl: ISAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getContentHandler(::core::mem::transmute_copy(&pphandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putContentHandler<Impl: ISAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putContentHandler(&*(&phandler as *const <ISAXContentHandler as ::windows::core::Abi>::Abi as *const <ISAXContentHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getDTDHandler<Impl: ISAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getDTDHandler(::core::mem::transmute_copy(&pphandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putDTDHandler<Impl: ISAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putDTDHandler(&*(&phandler as *const <ISAXDTDHandler as ::windows::core::Abi>::Abi as *const <ISAXDTDHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getErrorHandler<Impl: ISAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pphandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getErrorHandler(::core::mem::transmute_copy(&pphandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putErrorHandler<Impl: ISAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, phandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putErrorHandler(&*(&phandler as *const <ISAXErrorHandler as ::windows::core::Abi>::Abi as *const <ISAXErrorHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getBaseURL<Impl: ISAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwchbaseurl: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getBaseURL(::core::mem::transmute_copy(&ppwchbaseurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putBaseURL<Impl: ISAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchbaseurl: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putBaseURL(&*(&pwchbaseurl as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getSecureBaseURL<Impl: ISAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwchsecurebaseurl: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getSecureBaseURL(::core::mem::transmute_copy(&ppwchsecurebaseurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putSecureBaseURL<Impl: ISAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchsecurebaseurl: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putSecureBaseURL(&*(&pwchsecurebaseurl as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn parse<Impl: ISAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varinput: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).parse(&*(&varinput as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn parseURL<Impl: ISAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwchurl: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).parseURL(&*(&pwchurl as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISAXXMLReader>,
            base.5,
            getFeature::<Impl, OFFSET>,
            putFeature::<Impl, OFFSET>,
            getProperty::<Impl, OFFSET>,
            putProperty::<Impl, OFFSET>,
            getEntityResolver::<Impl, OFFSET>,
            putEntityResolver::<Impl, OFFSET>,
            getContentHandler::<Impl, OFFSET>,
            putContentHandler::<Impl, OFFSET>,
            getDTDHandler::<Impl, OFFSET>,
            putDTDHandler::<Impl, OFFSET>,
            getErrorHandler::<Impl, OFFSET>,
            putErrorHandler::<Impl, OFFSET>,
            getBaseURL::<Impl, OFFSET>,
            putBaseURL::<Impl, OFFSET>,
            getSecureBaseURL::<Impl, OFFSET>,
            putSecureBaseURL::<Impl, OFFSET>,
            parse::<Impl, OFFSET>,
            parseURL::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaImpl: Sized + ISchemaItemImpl + IDispatchImpl {
    fn targetNamespace();
    fn version();
    fn types();
    fn elements();
    fn attributes();
    fn attributeGroups();
    fn modelGroups();
    fn notations();
    fn schemaLocations();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISchema {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.ISchema";
}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaVtbl {
    pub const fn new<Impl: ISchemaImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISchemaVtbl {
        unsafe extern "system" fn targetNamespace<Impl: ISchemaImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, targetnamespace: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).targetNamespace(::core::mem::transmute_copy(&targetnamespace)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn version<Impl: ISchemaImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, version: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).version(::core::mem::transmute_copy(&version)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn types<Impl: ISchemaImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, types: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).types(::core::mem::transmute_copy(&types)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn elements<Impl: ISchemaImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, elements: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).elements(::core::mem::transmute_copy(&elements)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributes<Impl: ISchemaImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).attributes(::core::mem::transmute_copy(&attributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributeGroups<Impl: ISchemaImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributegroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).attributeGroups(::core::mem::transmute_copy(&attributegroups)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn modelGroups<Impl: ISchemaImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, modelgroups: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).modelGroups(::core::mem::transmute_copy(&modelgroups)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn notations<Impl: ISchemaImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, notations: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).notations(::core::mem::transmute_copy(&notations)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn schemaLocations<Impl: ISchemaImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, schemalocations: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).schemaLocations(::core::mem::transmute_copy(&schemalocations)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISchema>, base.5, targetNamespace::<Impl, OFFSET>, version::<Impl, OFFSET>, types::<Impl, OFFSET>, elements::<Impl, OFFSET>, attributes::<Impl, OFFSET>, attributeGroups::<Impl, OFFSET>, modelGroups::<Impl, OFFSET>, notations::<Impl, OFFSET>, schemaLocations::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaAnyImpl: Sized + ISchemaParticleImpl + ISchemaItemImpl + IDispatchImpl {
    fn namespaces();
    fn processContents();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISchemaAny {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.ISchemaAny";
}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaAnyVtbl {
    pub const fn new<Impl: ISchemaAnyImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISchemaAnyVtbl {
        unsafe extern "system" fn namespaces<Impl: ISchemaAnyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaces: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).namespaces(::core::mem::transmute_copy(&namespaces)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn processContents<Impl: ISchemaAnyImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, processcontents: *mut SCHEMAPROCESSCONTENTS) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).processContents(::core::mem::transmute_copy(&processcontents)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISchemaAny>, base.5, namespaces::<Impl, OFFSET>, processContents::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaAttributeImpl: Sized + ISchemaItemImpl + IDispatchImpl {
    fn r#type();
    fn scope();
    fn defaultValue();
    fn fixedValue();
    fn r#use();
    fn isReference();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISchemaAttribute {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.ISchemaAttribute";
}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaAttributeVtbl {
    pub const fn new<Impl: ISchemaAttributeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISchemaAttributeVtbl {
        unsafe extern "system" fn r#type<Impl: ISchemaAttributeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).r#type(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn scope<Impl: ISchemaAttributeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).scope(::core::mem::transmute_copy(&scope)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn defaultValue<Impl: ISchemaAttributeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, defaultvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).defaultValue(::core::mem::transmute_copy(&defaultvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fixedValue<Impl: ISchemaAttributeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fixedvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).fixedValue(::core::mem::transmute_copy(&fixedvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#use<Impl: ISchemaAttributeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#use: *mut SCHEMAUSE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).r#use(::core::mem::transmute_copy(&r#use)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isReference<Impl: ISchemaAttributeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reference: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).isReference(::core::mem::transmute_copy(&reference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISchemaAttribute>, base.5, r#type::<Impl, OFFSET>, scope::<Impl, OFFSET>, defaultValue::<Impl, OFFSET>, fixedValue::<Impl, OFFSET>, r#use::<Impl, OFFSET>, isReference::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaAttributeGroupImpl: Sized + ISchemaItemImpl + IDispatchImpl {
    fn anyAttribute();
    fn attributes();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISchemaAttributeGroup {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.ISchemaAttributeGroup";
}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaAttributeGroupVtbl {
    pub const fn new<Impl: ISchemaAttributeGroupImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISchemaAttributeGroupVtbl {
        unsafe extern "system" fn anyAttribute<Impl: ISchemaAttributeGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, anyattribute: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).anyAttribute(::core::mem::transmute_copy(&anyattribute)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributes<Impl: ISchemaAttributeGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).attributes(::core::mem::transmute_copy(&attributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISchemaAttributeGroup>, base.5, anyAttribute::<Impl, OFFSET>, attributes::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaComplexTypeImpl: Sized + ISchemaTypeImpl + ISchemaItemImpl + IDispatchImpl {
    fn isAbstract();
    fn anyAttribute();
    fn attributes();
    fn contentType();
    fn contentModel();
    fn prohibitedSubstitutions();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISchemaComplexType {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.ISchemaComplexType";
}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaComplexTypeVtbl {
    pub const fn new<Impl: ISchemaComplexTypeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISchemaComplexTypeVtbl {
        unsafe extern "system" fn isAbstract<Impl: ISchemaComplexTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#abstract: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).isAbstract(::core::mem::transmute_copy(&r#abstract)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn anyAttribute<Impl: ISchemaComplexTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, anyattribute: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).anyAttribute(::core::mem::transmute_copy(&anyattribute)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributes<Impl: ISchemaComplexTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).attributes(::core::mem::transmute_copy(&attributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn contentType<Impl: ISchemaComplexTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contenttype: *mut SCHEMACONTENTTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).contentType(::core::mem::transmute_copy(&contenttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn contentModel<Impl: ISchemaComplexTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contentmodel: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).contentModel(::core::mem::transmute_copy(&contentmodel)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn prohibitedSubstitutions<Impl: ISchemaComplexTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prohibited: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).prohibitedSubstitutions(::core::mem::transmute_copy(&prohibited)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISchemaComplexType>, base.5, isAbstract::<Impl, OFFSET>, anyAttribute::<Impl, OFFSET>, attributes::<Impl, OFFSET>, contentType::<Impl, OFFSET>, contentModel::<Impl, OFFSET>, prohibitedSubstitutions::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaElementImpl: Sized + ISchemaParticleImpl + ISchemaItemImpl + IDispatchImpl {
    fn r#type();
    fn scope();
    fn defaultValue();
    fn fixedValue();
    fn isNillable();
    fn identityConstraints();
    fn substitutionGroup();
    fn substitutionGroupExclusions();
    fn disallowedSubstitutions();
    fn isAbstract();
    fn isReference();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISchemaElement {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.ISchemaElement";
}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaElementVtbl {
    pub const fn new<Impl: ISchemaElementImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISchemaElementVtbl {
        unsafe extern "system" fn r#type<Impl: ISchemaElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).r#type(::core::mem::transmute_copy(&r#type)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn scope<Impl: ISchemaElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, scope: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).scope(::core::mem::transmute_copy(&scope)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn defaultValue<Impl: ISchemaElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, defaultvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).defaultValue(::core::mem::transmute_copy(&defaultvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fixedValue<Impl: ISchemaElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fixedvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).fixedValue(::core::mem::transmute_copy(&fixedvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isNillable<Impl: ISchemaElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nillable: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).isNillable(::core::mem::transmute_copy(&nillable)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn identityConstraints<Impl: ISchemaElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, constraints: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).identityConstraints(::core::mem::transmute_copy(&constraints)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn substitutionGroup<Impl: ISchemaElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).substitutionGroup(::core::mem::transmute_copy(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn substitutionGroupExclusions<Impl: ISchemaElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, exclusions: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).substitutionGroupExclusions(::core::mem::transmute_copy(&exclusions)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn disallowedSubstitutions<Impl: ISchemaElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, disallowed: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).disallowedSubstitutions(::core::mem::transmute_copy(&disallowed)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isAbstract<Impl: ISchemaElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#abstract: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).isAbstract(::core::mem::transmute_copy(&r#abstract)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isReference<Impl: ISchemaElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reference: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).isReference(::core::mem::transmute_copy(&reference)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISchemaElement>, base.5, r#type::<Impl, OFFSET>, scope::<Impl, OFFSET>, defaultValue::<Impl, OFFSET>, fixedValue::<Impl, OFFSET>, isNillable::<Impl, OFFSET>, identityConstraints::<Impl, OFFSET>, substitutionGroup::<Impl, OFFSET>, substitutionGroupExclusions::<Impl, OFFSET>, disallowedSubstitutions::<Impl, OFFSET>, isAbstract::<Impl, OFFSET>, isReference::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaIdentityConstraintImpl: Sized + ISchemaItemImpl + IDispatchImpl {
    fn selector();
    fn fields();
    fn referencedKey();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISchemaIdentityConstraint {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.ISchemaIdentityConstraint";
}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaIdentityConstraintVtbl {
    pub const fn new<Impl: ISchemaIdentityConstraintImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISchemaIdentityConstraintVtbl {
        unsafe extern "system" fn selector<Impl: ISchemaIdentityConstraintImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, selector: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).selector(::core::mem::transmute_copy(&selector)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fields<Impl: ISchemaIdentityConstraintImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fields: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).fields(::core::mem::transmute_copy(&fields)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn referencedKey<Impl: ISchemaIdentityConstraintImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, key: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).referencedKey(::core::mem::transmute_copy(&key)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISchemaIdentityConstraint>, base.5, selector::<Impl, OFFSET>, fields::<Impl, OFFSET>, referencedKey::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaItemImpl: Sized + IDispatchImpl {
    fn name();
    fn namespaceURI();
    fn schema();
    fn id();
    fn itemType();
    fn unhandledAttributes();
    fn writeAnnotation();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISchemaItem {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.ISchemaItem";
}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaItemVtbl {
    pub const fn new<Impl: ISchemaItemImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISchemaItemVtbl {
        unsafe extern "system" fn name<Impl: ISchemaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).name(::core::mem::transmute_copy(&name)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn namespaceURI<Impl: ISchemaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).namespaceURI(::core::mem::transmute_copy(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn schema<Impl: ISchemaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, schema: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).schema(::core::mem::transmute_copy(&schema)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn id<Impl: ISchemaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, id: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).id(::core::mem::transmute_copy(&id)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn itemType<Impl: ISchemaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, itemtype: *mut SOMITEMTYPE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).itemType(::core::mem::transmute_copy(&itemtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn unhandledAttributes<Impl: ISchemaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).unhandledAttributes(::core::mem::transmute_copy(&attributes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn writeAnnotation<Impl: ISchemaItemImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, annotationsink: *mut ::core::ffi::c_void, iswritten: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).writeAnnotation(&*(&annotationsink as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&iswritten)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISchemaItem>, base.5, name::<Impl, OFFSET>, namespaceURI::<Impl, OFFSET>, schema::<Impl, OFFSET>, id::<Impl, OFFSET>, itemType::<Impl, OFFSET>, unhandledAttributes::<Impl, OFFSET>, writeAnnotation::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaItemCollectionImpl: Sized + IDispatchImpl {
    fn item();
    fn itemByName();
    fn itemByQName();
    fn length();
    fn _newEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISchemaItemCollection {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.ISchemaItemCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaItemCollectionVtbl {
    pub const fn new<Impl: ISchemaItemCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISchemaItemCollectionVtbl {
        unsafe extern "system" fn item<Impl: ISchemaItemCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).item(index, ::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn itemByName<Impl: ISchemaItemCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).itemByName(&*(&name as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn itemByQName<Impl: ISchemaItemCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).itemByQName(&*(&name as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&namespaceuri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Impl: ISchemaItemCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).length(::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Impl: ISchemaItemCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._newEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISchemaItemCollection>, base.5, item::<Impl, OFFSET>, itemByName::<Impl, OFFSET>, itemByQName::<Impl, OFFSET>, length::<Impl, OFFSET>, _newEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaModelGroupImpl: Sized + ISchemaParticleImpl + ISchemaItemImpl + IDispatchImpl {
    fn particles();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISchemaModelGroup {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.ISchemaModelGroup";
}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaModelGroupVtbl {
    pub const fn new<Impl: ISchemaModelGroupImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISchemaModelGroupVtbl {
        unsafe extern "system" fn particles<Impl: ISchemaModelGroupImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, particles: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).particles(::core::mem::transmute_copy(&particles)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISchemaModelGroup>, base.5, particles::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaNotationImpl: Sized + ISchemaItemImpl + IDispatchImpl {
    fn systemIdentifier();
    fn publicIdentifier();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISchemaNotation {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.ISchemaNotation";
}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaNotationVtbl {
    pub const fn new<Impl: ISchemaNotationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISchemaNotationVtbl {
        unsafe extern "system" fn systemIdentifier<Impl: ISchemaNotationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).systemIdentifier(::core::mem::transmute_copy(&uri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn publicIdentifier<Impl: ISchemaNotationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, uri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).publicIdentifier(::core::mem::transmute_copy(&uri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISchemaNotation>, base.5, systemIdentifier::<Impl, OFFSET>, publicIdentifier::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaParticleImpl: Sized + ISchemaItemImpl + IDispatchImpl {
    fn minOccurs();
    fn maxOccurs();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISchemaParticle {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.ISchemaParticle";
}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaParticleVtbl {
    pub const fn new<Impl: ISchemaParticleImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISchemaParticleVtbl {
        unsafe extern "system" fn minOccurs<Impl: ISchemaParticleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minoccurs: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).minOccurs(::core::mem::transmute_copy(&minoccurs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxOccurs<Impl: ISchemaParticleImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxoccurs: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).maxOccurs(::core::mem::transmute_copy(&maxoccurs)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISchemaParticle>, base.5, minOccurs::<Impl, OFFSET>, maxOccurs::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaStringCollectionImpl: Sized + IDispatchImpl {
    fn item();
    fn length();
    fn _newEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISchemaStringCollection {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.ISchemaStringCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaStringCollectionVtbl {
    pub const fn new<Impl: ISchemaStringCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISchemaStringCollectionVtbl {
        unsafe extern "system" fn item<Impl: ISchemaStringCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, bstr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).item(index, ::core::mem::transmute_copy(&bstr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Impl: ISchemaStringCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).length(::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Impl: ISchemaStringCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._newEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<ISchemaStringCollection>, base.5, item::<Impl, OFFSET>, length::<Impl, OFFSET>, _newEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait ISchemaTypeImpl: Sized + ISchemaItemImpl + IDispatchImpl {
    fn baseTypes();
    fn r#final();
    fn variety();
    fn derivedBy();
    fn isValid();
    fn minExclusive();
    fn minInclusive();
    fn maxExclusive();
    fn maxInclusive();
    fn totalDigits();
    fn fractionDigits();
    fn length();
    fn minLength();
    fn maxLength();
    fn enumeration();
    fn whitespace();
    fn patterns();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for ISchemaType {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.ISchemaType";
}
#[cfg(feature = "Win32_System_Com")]
impl ISchemaTypeVtbl {
    pub const fn new<Impl: ISchemaTypeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> ISchemaTypeVtbl {
        unsafe extern "system" fn baseTypes<Impl: ISchemaTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, basetypes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).baseTypes(::core::mem::transmute_copy(&basetypes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#final<Impl: ISchemaTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#final: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).r#final(::core::mem::transmute_copy(&r#final)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn variety<Impl: ISchemaTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, variety: *mut SCHEMATYPEVARIETY) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).variety(::core::mem::transmute_copy(&variety)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn derivedBy<Impl: ISchemaTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, derivedby: *mut SCHEMADERIVATIONMETHOD) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).derivedBy(::core::mem::transmute_copy(&derivedby)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn isValid<Impl: ISchemaTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, valid: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).isValid(&*(&data as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&valid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn minExclusive<Impl: ISchemaTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minexclusive: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).minExclusive(::core::mem::transmute_copy(&minexclusive)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn minInclusive<Impl: ISchemaTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mininclusive: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).minInclusive(::core::mem::transmute_copy(&mininclusive)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxExclusive<Impl: ISchemaTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxexclusive: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).maxExclusive(::core::mem::transmute_copy(&maxexclusive)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxInclusive<Impl: ISchemaTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxinclusive: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).maxInclusive(::core::mem::transmute_copy(&maxinclusive)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn totalDigits<Impl: ISchemaTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, totaldigits: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).totalDigits(::core::mem::transmute_copy(&totaldigits)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fractionDigits<Impl: ISchemaTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fractiondigits: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).fractionDigits(::core::mem::transmute_copy(&fractiondigits)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Impl: ISchemaTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).length(::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn minLength<Impl: ISchemaTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, minlength: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).minLength(::core::mem::transmute_copy(&minlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn maxLength<Impl: ISchemaTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, maxlength: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).maxLength(::core::mem::transmute_copy(&maxlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn enumeration<Impl: ISchemaTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, enumeration: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).enumeration(::core::mem::transmute_copy(&enumeration)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn whitespace<Impl: ISchemaTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, whitespace: *mut SCHEMAWHITESPACE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).whitespace(::core::mem::transmute_copy(&whitespace)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn patterns<Impl: ISchemaTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, patterns: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).patterns(::core::mem::transmute_copy(&patterns)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<ISchemaType>,
            base.5,
            baseTypes::<Impl, OFFSET>,
            r#final::<Impl, OFFSET>,
            variety::<Impl, OFFSET>,
            derivedBy::<Impl, OFFSET>,
            isValid::<Impl, OFFSET>,
            minExclusive::<Impl, OFFSET>,
            minInclusive::<Impl, OFFSET>,
            maxExclusive::<Impl, OFFSET>,
            maxInclusive::<Impl, OFFSET>,
            totalDigits::<Impl, OFFSET>,
            fractionDigits::<Impl, OFFSET>,
            length::<Impl, OFFSET>,
            minLength::<Impl, OFFSET>,
            maxLength::<Impl, OFFSET>,
            enumeration::<Impl, OFFSET>,
            whitespace::<Impl, OFFSET>,
            patterns::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IServerXMLHTTPRequestImpl: Sized + IXMLHTTPRequestImpl + IDispatchImpl {
    fn setTimeouts();
    fn waitForResponse();
    fn getOption();
    fn setOption();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IServerXMLHTTPRequest {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IServerXMLHTTPRequest";
}
#[cfg(feature = "Win32_System_Com")]
impl IServerXMLHTTPRequestVtbl {
    pub const fn new<Impl: IServerXMLHTTPRequestImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IServerXMLHTTPRequestVtbl {
        unsafe extern "system" fn setTimeouts<Impl: IServerXMLHTTPRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, resolvetimeout: i32, connecttimeout: i32, sendtimeout: i32, receivetimeout: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).setTimeouts(resolvetimeout, connecttimeout, sendtimeout, receivetimeout) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn waitForResponse<Impl: IServerXMLHTTPRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, timeoutinseconds: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, issuccessful: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).waitForResponse(&*(&timeoutinseconds as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&issuccessful)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getOption<Impl: IServerXMLHTTPRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, option: SERVERXMLHTTP_OPTION, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getOption(option, ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setOption<Impl: IServerXMLHTTPRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, option: SERVERXMLHTTP_OPTION, value: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).setOption(option, &*(&value as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IServerXMLHTTPRequest>, base.5, setTimeouts::<Impl, OFFSET>, waitForResponse::<Impl, OFFSET>, getOption::<Impl, OFFSET>, setOption::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IServerXMLHTTPRequest2Impl: Sized + IServerXMLHTTPRequestImpl + IXMLHTTPRequestImpl + IDispatchImpl {
    fn setProxy();
    fn setProxyCredentials();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IServerXMLHTTPRequest2 {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IServerXMLHTTPRequest2";
}
#[cfg(feature = "Win32_System_Com")]
impl IServerXMLHTTPRequest2Vtbl {
    pub const fn new<Impl: IServerXMLHTTPRequest2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IServerXMLHTTPRequest2Vtbl {
        unsafe extern "system" fn setProxy<Impl: IServerXMLHTTPRequest2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, proxysetting: SXH_PROXY_SETTING, varproxyserver: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, varbypasslist: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).setProxy(proxysetting, &*(&varproxyserver as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&varbypasslist as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setProxyCredentials<Impl: IServerXMLHTTPRequest2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrusername: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrpassword: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).setProxyCredentials(&*(&bstrusername as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrpassword as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IServerXMLHTTPRequest2>, base.5, setProxy::<Impl, OFFSET>, setProxyCredentials::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBMXNamespaceManagerImpl: Sized + IDispatchImpl {
    fn SetallowOverride();
    fn allowOverride();
    fn reset();
    fn pushContext();
    fn pushNodeContext();
    fn popContext();
    fn declarePrefix();
    fn getDeclaredPrefixes();
    fn getPrefixes();
    fn getURI();
    fn getURIFromNode();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IVBMXNamespaceManager {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IVBMXNamespaceManager";
}
#[cfg(feature = "Win32_System_Com")]
impl IVBMXNamespaceManagerVtbl {
    pub const fn new<Impl: IVBMXNamespaceManagerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVBMXNamespaceManagerVtbl {
        unsafe extern "system" fn SetallowOverride<Impl: IVBMXNamespaceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, foverride: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetallowOverride(foverride) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn allowOverride<Impl: IVBMXNamespaceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, foverride: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).allowOverride(::core::mem::transmute_copy(&foverride)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Impl: IVBMXNamespaceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn pushContext<Impl: IVBMXNamespaceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).pushContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn pushNodeContext<Impl: IVBMXNamespaceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, contextnode: ::windows::core::RawPtr, fdeep: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).pushNodeContext(&*(&contextnode as *const <IXMLDOMNode as ::windows::core::Abi>::Abi as *const <IXMLDOMNode as ::windows::core::DefaultType>::DefaultType), fdeep) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn popContext<Impl: IVBMXNamespaceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).popContext() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn declarePrefix<Impl: IVBMXNamespaceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prefix: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).declarePrefix(&*(&prefix as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&namespaceuri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getDeclaredPrefixes<Impl: IVBMXNamespaceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prefixes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getDeclaredPrefixes(::core::mem::transmute_copy(&prefixes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getPrefixes<Impl: IVBMXNamespaceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, prefixes: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getPrefixes(&*(&namespaceuri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&prefixes)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getURI<Impl: IVBMXNamespaceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prefix: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, uri: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getURI(&*(&prefix as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&uri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getURIFromNode<Impl: IVBMXNamespaceManagerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strprefix: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, contextnode: ::windows::core::RawPtr, uri: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getURIFromNode(&*(&strprefix as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&contextnode as *const <IXMLDOMNode as ::windows::core::Abi>::Abi as *const <IXMLDOMNode as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&uri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVBMXNamespaceManager>, base.5, SetallowOverride::<Impl, OFFSET>, allowOverride::<Impl, OFFSET>, reset::<Impl, OFFSET>, pushContext::<Impl, OFFSET>, pushNodeContext::<Impl, OFFSET>, popContext::<Impl, OFFSET>, declarePrefix::<Impl, OFFSET>, getDeclaredPrefixes::<Impl, OFFSET>, getPrefixes::<Impl, OFFSET>, getURI::<Impl, OFFSET>, getURIFromNode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXAttributesImpl: Sized + IDispatchImpl {
    fn length();
    fn getURI();
    fn getLocalName();
    fn getQName();
    fn getIndexFromName();
    fn getIndexFromQName();
    fn getType();
    fn getTypeFromName();
    fn getTypeFromQName();
    fn getValue();
    fn getValueFromName();
    fn getValueFromQName();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IVBSAXAttributes {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IVBSAXAttributes";
}
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXAttributesVtbl {
    pub const fn new<Impl: IVBSAXAttributesImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVBSAXAttributesVtbl {
        unsafe extern "system" fn length<Impl: IVBSAXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).length(::core::mem::transmute_copy(&nlength)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getURI<Impl: IVBSAXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, struri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getURI(nindex, ::core::mem::transmute_copy(&struri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getLocalName<Impl: IVBSAXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, strlocalname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getLocalName(nindex, ::core::mem::transmute_copy(&strlocalname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getQName<Impl: IVBSAXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, strqname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getQName(nindex, ::core::mem::transmute_copy(&strqname)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getIndexFromName<Impl: IVBSAXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, struri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strlocalname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getIndexFromName(&*(&struri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&strlocalname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getIndexFromQName<Impl: IVBSAXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strqname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nindex: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getIndexFromQName(&*(&strqname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&nindex)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getType<Impl: IVBSAXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, strtype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getType(nindex, ::core::mem::transmute_copy(&strtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getTypeFromName<Impl: IVBSAXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, struri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strlocalname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strtype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getTypeFromName(&*(&struri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&strlocalname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&strtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getTypeFromQName<Impl: IVBSAXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strqname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strtype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getTypeFromQName(&*(&strqname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&strtype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getValue<Impl: IVBSAXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nindex: i32, strvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getValue(nindex, ::core::mem::transmute_copy(&strvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getValueFromName<Impl: IVBSAXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, struri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strlocalname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getValueFromName(&*(&struri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&strlocalname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&strvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getValueFromQName<Impl: IVBSAXAttributesImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strqname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, strvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getValueFromQName(&*(&strqname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&strvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVBSAXAttributes>, base.5, length::<Impl, OFFSET>, getURI::<Impl, OFFSET>, getLocalName::<Impl, OFFSET>, getQName::<Impl, OFFSET>, getIndexFromName::<Impl, OFFSET>, getIndexFromQName::<Impl, OFFSET>, getType::<Impl, OFFSET>, getTypeFromName::<Impl, OFFSET>, getTypeFromQName::<Impl, OFFSET>, getValue::<Impl, OFFSET>, getValueFromName::<Impl, OFFSET>, getValueFromQName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXContentHandlerImpl: Sized + IDispatchImpl {
    fn putref_documentLocator();
    fn startDocument();
    fn endDocument();
    fn startPrefixMapping();
    fn endPrefixMapping();
    fn startElement();
    fn endElement();
    fn characters();
    fn ignorableWhitespace();
    fn processingInstruction();
    fn skippedEntity();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IVBSAXContentHandler {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IVBSAXContentHandler";
}
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXContentHandlerVtbl {
    pub const fn new<Impl: IVBSAXContentHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVBSAXContentHandlerVtbl {
        unsafe extern "system" fn putref_documentLocator<Impl: IVBSAXContentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, olocator: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putref_documentLocator(&*(&olocator as *const <IVBSAXLocator as ::windows::core::Abi>::Abi as *const <IVBSAXLocator as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn startDocument<Impl: IVBSAXContentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).startDocument() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn endDocument<Impl: IVBSAXContentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).endDocument() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn startPrefixMapping<Impl: IVBSAXContentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strprefix: *mut super::super::super::Foundation::BSTR, struri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).startPrefixMapping(&*(&strprefix as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&struri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn endPrefixMapping<Impl: IVBSAXContentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strprefix: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).endPrefixMapping(&*(&strprefix as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn startElement<Impl: IVBSAXContentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strnamespaceuri: *mut super::super::super::Foundation::BSTR, strlocalname: *mut super::super::super::Foundation::BSTR, strqname: *mut super::super::super::Foundation::BSTR, oattributes: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).startElement(
                &*(&strnamespaceuri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strlocalname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strqname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&oattributes as *const <IVBSAXAttributes as ::windows::core::Abi>::Abi as *const <IVBSAXAttributes as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn endElement<Impl: IVBSAXContentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strnamespaceuri: *mut super::super::super::Foundation::BSTR, strlocalname: *mut super::super::super::Foundation::BSTR, strqname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).endElement(
                &*(&strnamespaceuri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strlocalname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strqname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn characters<Impl: IVBSAXContentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strchars: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).characters(&*(&strchars as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ignorableWhitespace<Impl: IVBSAXContentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strchars: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ignorableWhitespace(&*(&strchars as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn processingInstruction<Impl: IVBSAXContentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strtarget: *mut super::super::super::Foundation::BSTR, strdata: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).processingInstruction(&*(&strtarget as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&strdata as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn skippedEntity<Impl: IVBSAXContentHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).skippedEntity(&*(&strname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVBSAXContentHandler>, base.5, putref_documentLocator::<Impl, OFFSET>, startDocument::<Impl, OFFSET>, endDocument::<Impl, OFFSET>, startPrefixMapping::<Impl, OFFSET>, endPrefixMapping::<Impl, OFFSET>, startElement::<Impl, OFFSET>, endElement::<Impl, OFFSET>, characters::<Impl, OFFSET>, ignorableWhitespace::<Impl, OFFSET>, processingInstruction::<Impl, OFFSET>, skippedEntity::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXDTDHandlerImpl: Sized + IDispatchImpl {
    fn notationDecl();
    fn unparsedEntityDecl();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IVBSAXDTDHandler {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IVBSAXDTDHandler";
}
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXDTDHandlerVtbl {
    pub const fn new<Impl: IVBSAXDTDHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVBSAXDTDHandlerVtbl {
        unsafe extern "system" fn notationDecl<Impl: IVBSAXDTDHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).notationDecl(
                &*(&strname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strpublicid as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strsystemid as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn unparsedEntityDecl<Impl: IVBSAXDTDHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR, strnotationname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).unparsedEntityDecl(
                &*(&strname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strpublicid as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strsystemid as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strnotationname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVBSAXDTDHandler>, base.5, notationDecl::<Impl, OFFSET>, unparsedEntityDecl::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXDeclHandlerImpl: Sized + IDispatchImpl {
    fn elementDecl();
    fn attributeDecl();
    fn internalEntityDecl();
    fn externalEntityDecl();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IVBSAXDeclHandler {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IVBSAXDeclHandler";
}
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXDeclHandlerVtbl {
    pub const fn new<Impl: IVBSAXDeclHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVBSAXDeclHandlerVtbl {
        unsafe extern "system" fn elementDecl<Impl: IVBSAXDeclHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR, strmodel: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).elementDecl(&*(&strname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&strmodel as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributeDecl<Impl: IVBSAXDeclHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strelementname: *mut super::super::super::Foundation::BSTR, strattributename: *mut super::super::super::Foundation::BSTR, strtype: *mut super::super::super::Foundation::BSTR, strvaluedefault: *mut super::super::super::Foundation::BSTR, strvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).attributeDecl(
                &*(&strelementname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strattributename as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strtype as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strvaluedefault as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strvalue as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn internalEntityDecl<Impl: IVBSAXDeclHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR, strvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).internalEntityDecl(&*(&strname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&strvalue as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn externalEntityDecl<Impl: IVBSAXDeclHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).externalEntityDecl(
                &*(&strname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strpublicid as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strsystemid as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVBSAXDeclHandler>, base.5, elementDecl::<Impl, OFFSET>, attributeDecl::<Impl, OFFSET>, internalEntityDecl::<Impl, OFFSET>, externalEntityDecl::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXEntityResolverImpl: Sized + IDispatchImpl {
    fn resolveEntity();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IVBSAXEntityResolver {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IVBSAXEntityResolver";
}
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXEntityResolverVtbl {
    pub const fn new<Impl: IVBSAXEntityResolverImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVBSAXEntityResolverVtbl {
        unsafe extern "system" fn resolveEntity<Impl: IVBSAXEntityResolverImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR, varinput: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).resolveEntity(&*(&strpublicid as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&strsystemid as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&varinput)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVBSAXEntityResolver>, base.5, resolveEntity::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXErrorHandlerImpl: Sized + IDispatchImpl {
    fn error();
    fn fatalError();
    fn ignorableWarning();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IVBSAXErrorHandler {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IVBSAXErrorHandler";
}
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXErrorHandlerVtbl {
    pub const fn new<Impl: IVBSAXErrorHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVBSAXErrorHandlerVtbl {
        unsafe extern "system" fn error<Impl: IVBSAXErrorHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, olocator: ::windows::core::RawPtr, strerrormessage: *mut super::super::super::Foundation::BSTR, nerrorcode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).error(&*(&olocator as *const <IVBSAXLocator as ::windows::core::Abi>::Abi as *const <IVBSAXLocator as ::windows::core::DefaultType>::DefaultType), &*(&strerrormessage as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), nerrorcode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fatalError<Impl: IVBSAXErrorHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, olocator: ::windows::core::RawPtr, strerrormessage: *mut super::super::super::Foundation::BSTR, nerrorcode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).fatalError(&*(&olocator as *const <IVBSAXLocator as ::windows::core::Abi>::Abi as *const <IVBSAXLocator as ::windows::core::DefaultType>::DefaultType), &*(&strerrormessage as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), nerrorcode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ignorableWarning<Impl: IVBSAXErrorHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, olocator: ::windows::core::RawPtr, strerrormessage: *mut super::super::super::Foundation::BSTR, nerrorcode: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ignorableWarning(&*(&olocator as *const <IVBSAXLocator as ::windows::core::Abi>::Abi as *const <IVBSAXLocator as ::windows::core::DefaultType>::DefaultType), &*(&strerrormessage as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), nerrorcode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVBSAXErrorHandler>, base.5, error::<Impl, OFFSET>, fatalError::<Impl, OFFSET>, ignorableWarning::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXLexicalHandlerImpl: Sized + IDispatchImpl {
    fn startDTD();
    fn endDTD();
    fn startEntity();
    fn endEntity();
    fn startCDATA();
    fn endCDATA();
    fn comment();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IVBSAXLexicalHandler {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IVBSAXLexicalHandler";
}
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXLexicalHandlerVtbl {
    pub const fn new<Impl: IVBSAXLexicalHandlerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVBSAXLexicalHandlerVtbl {
        unsafe extern "system" fn startDTD<Impl: IVBSAXLexicalHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR, strpublicid: *mut super::super::super::Foundation::BSTR, strsystemid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).startDTD(
                &*(&strname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strpublicid as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&strsystemid as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn endDTD<Impl: IVBSAXLexicalHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).endDTD() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn startEntity<Impl: IVBSAXLexicalHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).startEntity(&*(&strname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn endEntity<Impl: IVBSAXLexicalHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).endEntity(&*(&strname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn startCDATA<Impl: IVBSAXLexicalHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).startCDATA() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn endCDATA<Impl: IVBSAXLexicalHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).endCDATA() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn comment<Impl: IVBSAXLexicalHandlerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strchars: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).comment(&*(&strchars as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVBSAXLexicalHandler>, base.5, startDTD::<Impl, OFFSET>, endDTD::<Impl, OFFSET>, startEntity::<Impl, OFFSET>, endEntity::<Impl, OFFSET>, startCDATA::<Impl, OFFSET>, endCDATA::<Impl, OFFSET>, comment::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXLocatorImpl: Sized + IDispatchImpl {
    fn columnNumber();
    fn lineNumber();
    fn publicId();
    fn systemId();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IVBSAXLocator {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IVBSAXLocator";
}
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXLocatorVtbl {
    pub const fn new<Impl: IVBSAXLocatorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVBSAXLocatorVtbl {
        unsafe extern "system" fn columnNumber<Impl: IVBSAXLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ncolumn: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).columnNumber(::core::mem::transmute_copy(&ncolumn)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lineNumber<Impl: IVBSAXLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nline: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).lineNumber(::core::mem::transmute_copy(&nline)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn publicId<Impl: IVBSAXLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strpublicid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).publicId(::core::mem::transmute_copy(&strpublicid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn systemId<Impl: IVBSAXLocatorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strsystemid: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).systemId(::core::mem::transmute_copy(&strsystemid)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVBSAXLocator>, base.5, columnNumber::<Impl, OFFSET>, lineNumber::<Impl, OFFSET>, publicId::<Impl, OFFSET>, systemId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXXMLFilterImpl: Sized + IDispatchImpl {
    fn parent();
    fn putref_parent();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IVBSAXXMLFilter {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IVBSAXXMLFilter";
}
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXXMLFilterVtbl {
    pub const fn new<Impl: IVBSAXXMLFilterImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVBSAXXMLFilterVtbl {
        unsafe extern "system" fn parent<Impl: IVBSAXXMLFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oreader: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).parent(::core::mem::transmute_copy(&oreader)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_parent<Impl: IVBSAXXMLFilterImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oreader: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putref_parent(&*(&oreader as *const <IVBSAXXMLReader as ::windows::core::Abi>::Abi as *const <IVBSAXXMLReader as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IVBSAXXMLFilter>, base.5, parent::<Impl, OFFSET>, putref_parent::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IVBSAXXMLReaderImpl: Sized + IDispatchImpl {
    fn getFeature();
    fn putFeature();
    fn getProperty();
    fn putProperty();
    fn entityResolver();
    fn putref_entityResolver();
    fn contentHandler();
    fn putref_contentHandler();
    fn dtdHandler();
    fn putref_dtdHandler();
    fn errorHandler();
    fn putref_errorHandler();
    fn baseURL();
    fn SetbaseURL();
    fn secureBaseURL();
    fn SetsecureBaseURL();
    fn parse();
    fn parseURL();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IVBSAXXMLReader {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IVBSAXXMLReader";
}
#[cfg(feature = "Win32_System_Com")]
impl IVBSAXXMLReaderVtbl {
    pub const fn new<Impl: IVBSAXXMLReaderImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IVBSAXXMLReaderVtbl {
        unsafe extern "system" fn getFeature<Impl: IVBSAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, fvalue: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getFeature(&*(&strname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&fvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putFeature<Impl: IVBSAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, fvalue: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putFeature(&*(&strname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), fvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getProperty<Impl: IVBSAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getProperty(&*(&strname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&varvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putProperty<Impl: IVBSAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varvalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putProperty(&*(&strname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&varvalue as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn entityResolver<Impl: IVBSAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oresolver: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).entityResolver(::core::mem::transmute_copy(&oresolver)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_entityResolver<Impl: IVBSAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, oresolver: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putref_entityResolver(&*(&oresolver as *const <IVBSAXEntityResolver as ::windows::core::Abi>::Abi as *const <IVBSAXEntityResolver as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn contentHandler<Impl: IVBSAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).contentHandler(::core::mem::transmute_copy(&ohandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_contentHandler<Impl: IVBSAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ohandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putref_contentHandler(&*(&ohandler as *const <IVBSAXContentHandler as ::windows::core::Abi>::Abi as *const <IVBSAXContentHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn dtdHandler<Impl: IVBSAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).dtdHandler(::core::mem::transmute_copy(&ohandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_dtdHandler<Impl: IVBSAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ohandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putref_dtdHandler(&*(&ohandler as *const <IVBSAXDTDHandler as ::windows::core::Abi>::Abi as *const <IVBSAXDTDHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn errorHandler<Impl: IVBSAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ohandler: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).errorHandler(::core::mem::transmute_copy(&ohandler)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_errorHandler<Impl: IVBSAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ohandler: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putref_errorHandler(&*(&ohandler as *const <IVBSAXErrorHandler as ::windows::core::Abi>::Abi as *const <IVBSAXErrorHandler as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn baseURL<Impl: IVBSAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strbaseurl: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).baseURL(::core::mem::transmute_copy(&strbaseurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetbaseURL<Impl: IVBSAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strbaseurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetbaseURL(&*(&strbaseurl as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn secureBaseURL<Impl: IVBSAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strsecurebaseurl: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).secureBaseURL(::core::mem::transmute_copy(&strsecurebaseurl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetsecureBaseURL<Impl: IVBSAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strsecurebaseurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetsecureBaseURL(&*(&strsecurebaseurl as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn parse<Impl: IVBSAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varinput: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).parse(&*(&varinput as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn parseURL<Impl: IVBSAXXMLReaderImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).parseURL(&*(&strurl as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IVBSAXXMLReader>,
            base.5,
            getFeature::<Impl, OFFSET>,
            putFeature::<Impl, OFFSET>,
            getProperty::<Impl, OFFSET>,
            putProperty::<Impl, OFFSET>,
            entityResolver::<Impl, OFFSET>,
            putref_entityResolver::<Impl, OFFSET>,
            contentHandler::<Impl, OFFSET>,
            putref_contentHandler::<Impl, OFFSET>,
            dtdHandler::<Impl, OFFSET>,
            putref_dtdHandler::<Impl, OFFSET>,
            errorHandler::<Impl, OFFSET>,
            putref_errorHandler::<Impl, OFFSET>,
            baseURL::<Impl, OFFSET>,
            SetbaseURL::<Impl, OFFSET>,
            secureBaseURL::<Impl, OFFSET>,
            SetsecureBaseURL::<Impl, OFFSET>,
            parse::<Impl, OFFSET>,
            parseURL::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLAttributeImpl: Sized + IDispatchImpl {
    fn name();
    fn value();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLAttribute {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLAttribute";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLAttributeVtbl {
    pub const fn new<Impl: IXMLAttributeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLAttributeVtbl {
        unsafe extern "system" fn name<Impl: IXMLAttributeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, n: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).name(::core::mem::transmute_copy(&n)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn value<Impl: IXMLAttributeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, v: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).value(::core::mem::transmute_copy(&v)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLAttribute>, base.5, name::<Impl, OFFSET>, value::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMAttributeImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn name();
    fn value();
    fn Setvalue();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDOMAttribute {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDOMAttribute";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMAttributeVtbl {
    pub const fn new<Impl: IXMLDOMAttributeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDOMAttributeVtbl {
        unsafe extern "system" fn name<Impl: IXMLDOMAttributeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributename: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).name(&*(&attributename as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn value<Impl: IXMLDOMAttributeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributevalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).value(&*(&attributevalue as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setvalue<Impl: IXMLDOMAttributeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributevalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Setvalue(&*(&attributevalue as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLDOMAttribute>, base.5, name::<Impl, OFFSET>, value::<Impl, OFFSET>, Setvalue::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMCDATASectionImpl: Sized + IXMLDOMTextImpl + IXMLDOMCharacterDataImpl + IXMLDOMNodeImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDOMCDATASection {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDOMCDATASection";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMCDATASectionVtbl {
    pub const fn new<Impl: IXMLDOMCDATASectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDOMCDATASectionVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLDOMCDATASection>, base.5)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMCharacterDataImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn data();
    fn Setdata();
    fn length();
    fn substringData();
    fn appendData();
    fn insertData();
    fn deleteData();
    fn replaceData();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDOMCharacterData {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDOMCharacterData";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMCharacterDataVtbl {
    pub const fn new<Impl: IXMLDOMCharacterDataImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDOMCharacterDataVtbl {
        unsafe extern "system" fn data<Impl: IXMLDOMCharacterDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).data(&*(&data as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setdata<Impl: IXMLDOMCharacterDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Setdata(&*(&data as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Impl: IXMLDOMCharacterDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datalength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).length(datalength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn substringData<Impl: IXMLDOMCharacterDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: i32, count: i32, data: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).substringData(offset, count, &*(&data as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn appendData<Impl: IXMLDOMCharacterDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).appendData(&*(&data as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn insertData<Impl: IXMLDOMCharacterDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: i32, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).insertData(offset, &*(&data as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn deleteData<Impl: IXMLDOMCharacterDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: i32, count: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).deleteData(offset, count) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn replaceData<Impl: IXMLDOMCharacterDataImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: i32, count: i32, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).replaceData(offset, count, &*(&data as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLDOMCharacterData>, base.5, data::<Impl, OFFSET>, Setdata::<Impl, OFFSET>, length::<Impl, OFFSET>, substringData::<Impl, OFFSET>, appendData::<Impl, OFFSET>, insertData::<Impl, OFFSET>, deleteData::<Impl, OFFSET>, replaceData::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMCommentImpl: Sized + IXMLDOMCharacterDataImpl + IXMLDOMNodeImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDOMComment {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDOMComment";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMCommentVtbl {
    pub const fn new<Impl: IXMLDOMCommentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDOMCommentVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLDOMComment>, base.5)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMDocumentImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn doctype();
    fn implementation();
    fn documentElement();
    fn putref_documentElement();
    fn createElement();
    fn createDocumentFragment();
    fn createTextNode();
    fn createComment();
    fn createCDATASection();
    fn createProcessingInstruction();
    fn createAttribute();
    fn createEntityReference();
    fn getElementsByTagName();
    fn createNode();
    fn nodeFromID();
    fn load();
    fn readyState();
    fn parseError();
    fn url();
    fn r#async();
    fn Setasync();
    fn abort();
    fn loadXML();
    fn save();
    fn validateOnParse();
    fn SetvalidateOnParse();
    fn resolveExternals();
    fn SetresolveExternals();
    fn preserveWhiteSpace();
    fn SetpreserveWhiteSpace();
    fn Setonreadystatechange();
    fn Setondataavailable();
    fn Setontransformnode();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDOMDocument {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDOMDocument";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMDocumentVtbl {
    pub const fn new<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDOMDocumentVtbl {
        unsafe extern "system" fn doctype<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, documenttype: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).doctype(::core::mem::transmute_copy(&documenttype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn implementation<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#impl: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).implementation(::core::mem::transmute_copy(&r#impl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn documentElement<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, domelement: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).documentElement(::core::mem::transmute_copy(&domelement)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_documentElement<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, domelement: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putref_documentElement(&*(&domelement as *const <IXMLDOMElement as ::windows::core::Abi>::Abi as *const <IXMLDOMElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createElement<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tagname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, element: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).createElement(&*(&tagname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&element)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createDocumentFragment<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, docfrag: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).createDocumentFragment(::core::mem::transmute_copy(&docfrag)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createTextNode<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, text: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).createTextNode(&*(&data as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&text)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createComment<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, comment: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).createComment(&*(&data as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&comment)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createCDATASection<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, cdata: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).createCDATASection(&*(&data as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&cdata)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createProcessingInstruction<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, target: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, data: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pi: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).createProcessingInstruction(&*(&target as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&data as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pi)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createAttribute<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, attribute: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).createAttribute(&*(&name as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&attribute)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createEntityReference<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, entityref: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).createEntityReference(&*(&name as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&entityref)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getElementsByTagName<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tagname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, resultlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getElementsByTagName(&*(&tagname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&resultlist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createNode<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, node: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).createNode(
                &*(&r#type as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&name as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&namespaceuri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                ::core::mem::transmute_copy(&node),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nodeFromID<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, idstring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, node: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).nodeFromID(&*(&idstring as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&node)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn load<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xmlsource: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, issuccessful: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).load(&*(&xmlsource as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), issuccessful) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn readyState<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).readyState(value) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn parseError<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errorobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).parseError(::core::mem::transmute_copy(&errorobj)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn url<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, urlstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).url(&*(&urlstring as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#async<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isasync: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).r#async(isasync) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setasync<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isasync: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Setasync(isasync) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn abort<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).abort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn loadXML<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrxml: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, issuccessful: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).loadXML(&*(&bstrxml as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), issuccessful) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn save<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, destination: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).save(&*(&destination as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn validateOnParse<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isvalidating: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).validateOnParse(isvalidating) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetvalidateOnParse<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isvalidating: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetvalidateOnParse(isvalidating) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn resolveExternals<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isresolving: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).resolveExternals(isresolving) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetresolveExternals<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isresolving: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetresolveExternals(isresolving) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn preserveWhiteSpace<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ispreserving: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).preserveWhiteSpace(ispreserving) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetpreserveWhiteSpace<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ispreserving: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetpreserveWhiteSpace(ispreserving) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setonreadystatechange<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, readystatechangesink: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Setonreadystatechange(&*(&readystatechangesink as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setondataavailable<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ondataavailablesink: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Setondataavailable(&*(&ondataavailablesink as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setontransformnode<Impl: IXMLDOMDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ontransformnodesink: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Setontransformnode(&*(&ontransformnodesink as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IXMLDOMDocument>,
            base.5,
            doctype::<Impl, OFFSET>,
            implementation::<Impl, OFFSET>,
            documentElement::<Impl, OFFSET>,
            putref_documentElement::<Impl, OFFSET>,
            createElement::<Impl, OFFSET>,
            createDocumentFragment::<Impl, OFFSET>,
            createTextNode::<Impl, OFFSET>,
            createComment::<Impl, OFFSET>,
            createCDATASection::<Impl, OFFSET>,
            createProcessingInstruction::<Impl, OFFSET>,
            createAttribute::<Impl, OFFSET>,
            createEntityReference::<Impl, OFFSET>,
            getElementsByTagName::<Impl, OFFSET>,
            createNode::<Impl, OFFSET>,
            nodeFromID::<Impl, OFFSET>,
            load::<Impl, OFFSET>,
            readyState::<Impl, OFFSET>,
            parseError::<Impl, OFFSET>,
            url::<Impl, OFFSET>,
            r#async::<Impl, OFFSET>,
            Setasync::<Impl, OFFSET>,
            abort::<Impl, OFFSET>,
            loadXML::<Impl, OFFSET>,
            save::<Impl, OFFSET>,
            validateOnParse::<Impl, OFFSET>,
            SetvalidateOnParse::<Impl, OFFSET>,
            resolveExternals::<Impl, OFFSET>,
            SetresolveExternals::<Impl, OFFSET>,
            preserveWhiteSpace::<Impl, OFFSET>,
            SetpreserveWhiteSpace::<Impl, OFFSET>,
            Setonreadystatechange::<Impl, OFFSET>,
            Setondataavailable::<Impl, OFFSET>,
            Setontransformnode::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMDocument2Impl: Sized + IXMLDOMDocumentImpl + IXMLDOMNodeImpl + IDispatchImpl {
    fn namespaces();
    fn schemas();
    fn putref_schemas();
    fn validate();
    fn setProperty();
    fn getProperty();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDOMDocument2 {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDOMDocument2";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMDocument2Vtbl {
    pub const fn new<Impl: IXMLDOMDocument2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDOMDocument2Vtbl {
        unsafe extern "system" fn namespaces<Impl: IXMLDOMDocument2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespacecollection: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).namespaces(::core::mem::transmute_copy(&namespacecollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn schemas<Impl: IXMLDOMDocument2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, othercollection: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).schemas(::core::mem::transmute_copy(&othercollection)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_schemas<Impl: IXMLDOMDocument2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, othercollection: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putref_schemas(&*(&othercollection as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn validate<Impl: IXMLDOMDocument2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errorobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).validate(::core::mem::transmute_copy(&errorobj)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setProperty<Impl: IXMLDOMDocument2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).setProperty(&*(&name as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getProperty<Impl: IXMLDOMDocument2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getProperty(&*(&name as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLDOMDocument2>, base.5, namespaces::<Impl, OFFSET>, schemas::<Impl, OFFSET>, putref_schemas::<Impl, OFFSET>, validate::<Impl, OFFSET>, setProperty::<Impl, OFFSET>, getProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMDocument3Impl: Sized + IXMLDOMDocument2Impl + IXMLDOMDocumentImpl + IXMLDOMNodeImpl + IDispatchImpl {
    fn validateNode();
    fn importNode();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDOMDocument3 {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDOMDocument3";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMDocument3Vtbl {
    pub const fn new<Impl: IXMLDOMDocument3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDOMDocument3Vtbl {
        unsafe extern "system" fn validateNode<Impl: IXMLDOMDocument3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, errorobj: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).validateNode(&*(&node as *const <IXMLDOMNode as ::windows::core::Abi>::Abi as *const <IXMLDOMNode as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&errorobj)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn importNode<Impl: IXMLDOMDocument3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, deep: i16, clone: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).importNode(&*(&node as *const <IXMLDOMNode as ::windows::core::Abi>::Abi as *const <IXMLDOMNode as ::windows::core::DefaultType>::DefaultType), deep, ::core::mem::transmute_copy(&clone)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLDOMDocument3>, base.5, validateNode::<Impl, OFFSET>, importNode::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMDocumentFragmentImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDOMDocumentFragment {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDOMDocumentFragment";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMDocumentFragmentVtbl {
    pub const fn new<Impl: IXMLDOMDocumentFragmentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDOMDocumentFragmentVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLDOMDocumentFragment>, base.5)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMDocumentTypeImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn name();
    fn entities();
    fn notations();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDOMDocumentType {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDOMDocumentType";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMDocumentTypeVtbl {
    pub const fn new<Impl: IXMLDOMDocumentTypeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDOMDocumentTypeVtbl {
        unsafe extern "system" fn name<Impl: IXMLDOMDocumentTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, rootname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).name(&*(&rootname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn entities<Impl: IXMLDOMDocumentTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, entitymap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).entities(::core::mem::transmute_copy(&entitymap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn notations<Impl: IXMLDOMDocumentTypeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, notationmap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).notations(::core::mem::transmute_copy(&notationmap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLDOMDocumentType>, base.5, name::<Impl, OFFSET>, entities::<Impl, OFFSET>, notations::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMElementImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn tagName();
    fn getAttribute();
    fn setAttribute();
    fn removeAttribute();
    fn getAttributeNode();
    fn setAttributeNode();
    fn removeAttributeNode();
    fn getElementsByTagName();
    fn normalize();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDOMElement {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDOMElement";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMElementVtbl {
    pub const fn new<Impl: IXMLDOMElementImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDOMElementVtbl {
        unsafe extern "system" fn tagName<Impl: IXMLDOMElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tagname: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).tagName(&*(&tagname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAttribute<Impl: IXMLDOMElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getAttribute(&*(&name as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttribute<Impl: IXMLDOMElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).setAttribute(&*(&name as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeAttribute<Impl: IXMLDOMElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).removeAttribute(&*(&name as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAttributeNode<Impl: IXMLDOMElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, attributenode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getAttributeNode(&*(&name as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&attributenode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttributeNode<Impl: IXMLDOMElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, domattribute: ::windows::core::RawPtr, attributenode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).setAttributeNode(&*(&domattribute as *const <IXMLDOMAttribute as ::windows::core::Abi>::Abi as *const <IXMLDOMAttribute as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&attributenode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeAttributeNode<Impl: IXMLDOMElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, domattribute: ::windows::core::RawPtr, attributenode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).removeAttributeNode(&*(&domattribute as *const <IXMLDOMAttribute as ::windows::core::Abi>::Abi as *const <IXMLDOMAttribute as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&attributenode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getElementsByTagName<Impl: IXMLDOMElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, tagname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, resultlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getElementsByTagName(&*(&tagname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&resultlist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn normalize<Impl: IXMLDOMElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).normalize() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLDOMElement>, base.5, tagName::<Impl, OFFSET>, getAttribute::<Impl, OFFSET>, setAttribute::<Impl, OFFSET>, removeAttribute::<Impl, OFFSET>, getAttributeNode::<Impl, OFFSET>, setAttributeNode::<Impl, OFFSET>, removeAttributeNode::<Impl, OFFSET>, getElementsByTagName::<Impl, OFFSET>, normalize::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMEntityImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn publicId();
    fn systemId();
    fn notationName();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDOMEntity {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDOMEntity";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMEntityVtbl {
    pub const fn new<Impl: IXMLDOMEntityImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDOMEntityVtbl {
        unsafe extern "system" fn publicId<Impl: IXMLDOMEntityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, publicid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).publicId(&*(&publicid as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn systemId<Impl: IXMLDOMEntityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, systemid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).systemId(&*(&systemid as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn notationName<Impl: IXMLDOMEntityImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).notationName(&*(&name as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLDOMEntity>, base.5, publicId::<Impl, OFFSET>, systemId::<Impl, OFFSET>, notationName::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMEntityReferenceImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDOMEntityReference {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDOMEntityReference";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMEntityReferenceVtbl {
    pub const fn new<Impl: IXMLDOMEntityReferenceImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDOMEntityReferenceVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLDOMEntityReference>, base.5)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMImplementationImpl: Sized + IDispatchImpl {
    fn hasFeature();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDOMImplementation {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDOMImplementation";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMImplementationVtbl {
    pub const fn new<Impl: IXMLDOMImplementationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDOMImplementationVtbl {
        unsafe extern "system" fn hasFeature<Impl: IXMLDOMImplementationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, feature: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, version: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, hasfeature: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).hasFeature(&*(&feature as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&version as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), hasfeature) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLDOMImplementation>, base.5, hasFeature::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMNamedNodeMapImpl: Sized + IDispatchImpl {
    fn getNamedItem();
    fn setNamedItem();
    fn removeNamedItem();
    fn item();
    fn length();
    fn getQualifiedItem();
    fn removeQualifiedItem();
    fn nextNode();
    fn reset();
    fn _newEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDOMNamedNodeMap {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDOMNamedNodeMap";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMNamedNodeMapVtbl {
    pub const fn new<Impl: IXMLDOMNamedNodeMapImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDOMNamedNodeMapVtbl {
        unsafe extern "system" fn getNamedItem<Impl: IXMLDOMNamedNodeMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nameditem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getNamedItem(&*(&name as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&nameditem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setNamedItem<Impl: IXMLDOMNamedNodeMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newitem: ::windows::core::RawPtr, nameitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).setNamedItem(&*(&newitem as *const <IXMLDOMNode as ::windows::core::Abi>::Abi as *const <IXMLDOMNode as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&nameitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeNamedItem<Impl: IXMLDOMNamedNodeMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, nameditem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).removeNamedItem(&*(&name as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&nameditem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn item<Impl: IXMLDOMNamedNodeMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, listitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).item(index, ::core::mem::transmute_copy(&listitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Impl: IXMLDOMNamedNodeMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, listlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).length(listlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getQualifiedItem<Impl: IXMLDOMNamedNodeMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, basename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, qualifieditem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getQualifiedItem(&*(&basename as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&namespaceuri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&qualifieditem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeQualifiedItem<Impl: IXMLDOMNamedNodeMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, basename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, qualifieditem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).removeQualifiedItem(&*(&basename as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&namespaceuri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&qualifieditem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nextNode<Impl: IXMLDOMNamedNodeMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nextitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).nextNode(::core::mem::transmute_copy(&nextitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Impl: IXMLDOMNamedNodeMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Impl: IXMLDOMNamedNodeMapImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._newEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLDOMNamedNodeMap>, base.5, getNamedItem::<Impl, OFFSET>, setNamedItem::<Impl, OFFSET>, removeNamedItem::<Impl, OFFSET>, item::<Impl, OFFSET>, length::<Impl, OFFSET>, getQualifiedItem::<Impl, OFFSET>, removeQualifiedItem::<Impl, OFFSET>, nextNode::<Impl, OFFSET>, reset::<Impl, OFFSET>, _newEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMNodeImpl: Sized + IDispatchImpl {
    fn nodeName();
    fn nodeValue();
    fn SetnodeValue();
    fn nodeType();
    fn parentNode();
    fn childNodes();
    fn firstChild();
    fn lastChild();
    fn previousSibling();
    fn nextSibling();
    fn attributes();
    fn insertBefore();
    fn replaceChild();
    fn removeChild();
    fn appendChild();
    fn hasChildNodes();
    fn ownerDocument();
    fn cloneNode();
    fn nodeTypeString();
    fn text();
    fn Settext();
    fn specified();
    fn definition();
    fn nodeTypedValue();
    fn SetnodeTypedValue();
    fn dataType();
    fn SetdataType();
    fn xml();
    fn transformNode();
    fn selectNodes();
    fn selectSingleNode();
    fn parsed();
    fn namespaceURI();
    fn prefix();
    fn baseName();
    fn transformNodeToObject();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDOMNode {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDOMNode";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMNodeVtbl {
    pub const fn new<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDOMNodeVtbl {
        unsafe extern "system" fn nodeName<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).nodeName(&*(&name as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nodeValue<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).nodeValue(&*(&value as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetnodeValue<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetnodeValue(&*(&value as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nodeType<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, r#type: *mut DOMNodeType) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).nodeType(r#type) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn parentNode<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, parent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).parentNode(::core::mem::transmute_copy(&parent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn childNodes<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, childlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).childNodes(::core::mem::transmute_copy(&childlist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn firstChild<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, firstchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).firstChild(::core::mem::transmute_copy(&firstchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn lastChild<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lastchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).lastChild(::core::mem::transmute_copy(&lastchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn previousSibling<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, previoussibling: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).previousSibling(::core::mem::transmute_copy(&previoussibling)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nextSibling<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nextsibling: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).nextSibling(::core::mem::transmute_copy(&nextsibling)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributes<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, attributemap: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).attributes(::core::mem::transmute_copy(&attributemap)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn insertBefore<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr, refchild: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, outnewchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).insertBefore(&*(&newchild as *const <IXMLDOMNode as ::windows::core::Abi>::Abi as *const <IXMLDOMNode as ::windows::core::DefaultType>::DefaultType), &*(&refchild as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&outnewchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn replaceChild<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr, oldchild: ::windows::core::RawPtr, outoldchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).replaceChild(&*(&newchild as *const <IXMLDOMNode as ::windows::core::Abi>::Abi as *const <IXMLDOMNode as ::windows::core::DefaultType>::DefaultType), &*(&oldchild as *const <IXMLDOMNode as ::windows::core::Abi>::Abi as *const <IXMLDOMNode as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&outoldchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeChild<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, childnode: ::windows::core::RawPtr, oldchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).removeChild(&*(&childnode as *const <IXMLDOMNode as ::windows::core::Abi>::Abi as *const <IXMLDOMNode as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&oldchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn appendChild<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, newchild: ::windows::core::RawPtr, outnewchild: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).appendChild(&*(&newchild as *const <IXMLDOMNode as ::windows::core::Abi>::Abi as *const <IXMLDOMNode as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&outnewchild)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn hasChildNodes<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, haschild: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).hasChildNodes(haschild) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ownerDocument<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xmldomdocument: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ownerDocument(::core::mem::transmute_copy(&xmldomdocument)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn cloneNode<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, deep: i16, cloneroot: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).cloneNode(deep, ::core::mem::transmute_copy(&cloneroot)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nodeTypeString<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nodetype: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).nodeTypeString(&*(&nodetype as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn text<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).text(&*(&text as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settext<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, text: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Settext(&*(&text as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn specified<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isspecified: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).specified(isspecified) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn definition<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, definitionnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).definition(::core::mem::transmute_copy(&definitionnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nodeTypedValue<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, typedvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).nodeTypedValue(&*(&typedvalue as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetnodeTypedValue<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, typedvalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetnodeTypedValue(&*(&typedvalue as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn dataType<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datatypename: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).dataType(&*(&datatypename as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetdataType<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, datatypename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetdataType(&*(&datatypename as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn xml<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xmlstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).xml(&*(&xmlstring as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn transformNode<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stylesheet: ::windows::core::RawPtr, xmlstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).transformNode(&*(&stylesheet as *const <IXMLDOMNode as ::windows::core::Abi>::Abi as *const <IXMLDOMNode as ::windows::core::DefaultType>::DefaultType), &*(&xmlstring as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn selectNodes<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, querystring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, resultlist: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).selectNodes(&*(&querystring as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&resultlist)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn selectSingleNode<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, querystring: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, resultnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).selectSingleNode(&*(&querystring as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&resultnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn parsed<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, isparsed: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).parsed(isparsed) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn namespaceURI<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).namespaceURI(&*(&namespaceuri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn prefix<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, prefixstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).prefix(&*(&prefixstring as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn baseName<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namestring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).baseName(&*(&namestring as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn transformNodeToObject<Impl: IXMLDOMNodeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stylesheet: ::windows::core::RawPtr, outputobject: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).transformNodeToObject(&*(&stylesheet as *const <IXMLDOMNode as ::windows::core::Abi>::Abi as *const <IXMLDOMNode as ::windows::core::DefaultType>::DefaultType), &*(&outputobject as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IXMLDOMNode>,
            base.5,
            nodeName::<Impl, OFFSET>,
            nodeValue::<Impl, OFFSET>,
            SetnodeValue::<Impl, OFFSET>,
            nodeType::<Impl, OFFSET>,
            parentNode::<Impl, OFFSET>,
            childNodes::<Impl, OFFSET>,
            firstChild::<Impl, OFFSET>,
            lastChild::<Impl, OFFSET>,
            previousSibling::<Impl, OFFSET>,
            nextSibling::<Impl, OFFSET>,
            attributes::<Impl, OFFSET>,
            insertBefore::<Impl, OFFSET>,
            replaceChild::<Impl, OFFSET>,
            removeChild::<Impl, OFFSET>,
            appendChild::<Impl, OFFSET>,
            hasChildNodes::<Impl, OFFSET>,
            ownerDocument::<Impl, OFFSET>,
            cloneNode::<Impl, OFFSET>,
            nodeTypeString::<Impl, OFFSET>,
            text::<Impl, OFFSET>,
            Settext::<Impl, OFFSET>,
            specified::<Impl, OFFSET>,
            definition::<Impl, OFFSET>,
            nodeTypedValue::<Impl, OFFSET>,
            SetnodeTypedValue::<Impl, OFFSET>,
            dataType::<Impl, OFFSET>,
            SetdataType::<Impl, OFFSET>,
            xml::<Impl, OFFSET>,
            transformNode::<Impl, OFFSET>,
            selectNodes::<Impl, OFFSET>,
            selectSingleNode::<Impl, OFFSET>,
            parsed::<Impl, OFFSET>,
            namespaceURI::<Impl, OFFSET>,
            prefix::<Impl, OFFSET>,
            baseName::<Impl, OFFSET>,
            transformNodeToObject::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMNodeListImpl: Sized + IDispatchImpl {
    fn item();
    fn length();
    fn nextNode();
    fn reset();
    fn _newEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDOMNodeList {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDOMNodeList";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMNodeListVtbl {
    pub const fn new<Impl: IXMLDOMNodeListImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDOMNodeListVtbl {
        unsafe extern "system" fn item<Impl: IXMLDOMNodeListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, listitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).item(index, ::core::mem::transmute_copy(&listitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Impl: IXMLDOMNodeListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, listlength: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).length(listlength) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn nextNode<Impl: IXMLDOMNodeListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, nextitem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).nextNode(::core::mem::transmute_copy(&nextitem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Impl: IXMLDOMNodeListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Impl: IXMLDOMNodeListImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._newEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLDOMNodeList>, base.5, item::<Impl, OFFSET>, length::<Impl, OFFSET>, nextNode::<Impl, OFFSET>, reset::<Impl, OFFSET>, _newEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMNotationImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn publicId();
    fn systemId();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDOMNotation {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDOMNotation";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMNotationVtbl {
    pub const fn new<Impl: IXMLDOMNotationImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDOMNotationVtbl {
        unsafe extern "system" fn publicId<Impl: IXMLDOMNotationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, publicid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).publicId(&*(&publicid as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn systemId<Impl: IXMLDOMNotationImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, systemid: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).systemId(&*(&systemid as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLDOMNotation>, base.5, publicId::<Impl, OFFSET>, systemId::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMParseErrorImpl: Sized + IDispatchImpl {
    fn errorCode();
    fn url();
    fn reason();
    fn srcText();
    fn line();
    fn linepos();
    fn filepos();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDOMParseError {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDOMParseError";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMParseErrorVtbl {
    pub const fn new<Impl: IXMLDOMParseErrorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDOMParseErrorVtbl {
        unsafe extern "system" fn errorCode<Impl: IXMLDOMParseErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, errorcode: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).errorCode(errorcode) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn url<Impl: IXMLDOMParseErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, urlstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).url(&*(&urlstring as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reason<Impl: IXMLDOMParseErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, reasonstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).reason(&*(&reasonstring as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn srcText<Impl: IXMLDOMParseErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, sourcestring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).srcText(&*(&sourcestring as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn line<Impl: IXMLDOMParseErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, linenumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).line(linenumber) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn linepos<Impl: IXMLDOMParseErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lineposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).linepos(lineposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn filepos<Impl: IXMLDOMParseErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fileposition: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).filepos(fileposition) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLDOMParseError>, base.5, errorCode::<Impl, OFFSET>, url::<Impl, OFFSET>, reason::<Impl, OFFSET>, srcText::<Impl, OFFSET>, line::<Impl, OFFSET>, linepos::<Impl, OFFSET>, filepos::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMParseError2Impl: Sized + IXMLDOMParseErrorImpl + IDispatchImpl {
    fn errorXPath();
    fn allErrors();
    fn errorParameters();
    fn errorParametersCount();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDOMParseError2 {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDOMParseError2";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMParseError2Vtbl {
    pub const fn new<Impl: IXMLDOMParseError2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDOMParseError2Vtbl {
        unsafe extern "system" fn errorXPath<Impl: IXMLDOMParseError2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, xpathexpr: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).errorXPath(::core::mem::transmute_copy(&xpathexpr)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn allErrors<Impl: IXMLDOMParseError2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, allerrors: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).allErrors(::core::mem::transmute_copy(&allerrors)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn errorParameters<Impl: IXMLDOMParseError2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, param1: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).errorParameters(index, ::core::mem::transmute_copy(&param1)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn errorParametersCount<Impl: IXMLDOMParseError2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, count: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).errorParametersCount(::core::mem::transmute_copy(&count)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLDOMParseError2>, base.5, errorXPath::<Impl, OFFSET>, allErrors::<Impl, OFFSET>, errorParameters::<Impl, OFFSET>, errorParametersCount::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMParseErrorCollectionImpl: Sized + IDispatchImpl {
    fn item();
    fn length();
    fn next();
    fn reset();
    fn _newEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDOMParseErrorCollection {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDOMParseErrorCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMParseErrorCollectionVtbl {
    pub const fn new<Impl: IXMLDOMParseErrorCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDOMParseErrorCollectionVtbl {
        unsafe extern "system" fn item<Impl: IXMLDOMParseErrorCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, error: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).item(index, ::core::mem::transmute_copy(&error)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Impl: IXMLDOMParseErrorCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).length(::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn next<Impl: IXMLDOMParseErrorCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, error: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).next(::core::mem::transmute_copy(&error)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Impl: IXMLDOMParseErrorCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Impl: IXMLDOMParseErrorCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._newEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLDOMParseErrorCollection>, base.5, item::<Impl, OFFSET>, length::<Impl, OFFSET>, next::<Impl, OFFSET>, reset::<Impl, OFFSET>, _newEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMProcessingInstructionImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn target();
    fn data();
    fn Setdata();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDOMProcessingInstruction {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDOMProcessingInstruction";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMProcessingInstructionVtbl {
    pub const fn new<Impl: IXMLDOMProcessingInstructionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDOMProcessingInstructionVtbl {
        unsafe extern "system" fn target<Impl: IXMLDOMProcessingInstructionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).target(&*(&name as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn data<Impl: IXMLDOMProcessingInstructionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).data(&*(&value as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setdata<Impl: IXMLDOMProcessingInstructionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, value: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Setdata(&*(&value as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLDOMProcessingInstruction>, base.5, target::<Impl, OFFSET>, data::<Impl, OFFSET>, Setdata::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMSchemaCollectionImpl: Sized + IDispatchImpl {
    fn add();
    fn get();
    fn remove();
    fn length();
    fn namespaceURI();
    fn addCollection();
    fn _newEnum();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDOMSchemaCollection {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDOMSchemaCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMSchemaCollectionVtbl {
    pub const fn new<Impl: IXMLDOMSchemaCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDOMSchemaCollectionVtbl {
        unsafe extern "system" fn add<Impl: IXMLDOMSchemaCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, var: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).add(&*(&namespaceuri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&var as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn get<Impl: IXMLDOMSchemaCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, schemanode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).get(&*(&namespaceuri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&schemanode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn remove<Impl: IXMLDOMSchemaCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).remove(&*(&namespaceuri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Impl: IXMLDOMSchemaCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, length: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).length(::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn namespaceURI<Impl: IXMLDOMSchemaCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, index: i32, length: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).namespaceURI(index, ::core::mem::transmute_copy(&length)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addCollection<Impl: IXMLDOMSchemaCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, othercollection: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).addCollection(&*(&othercollection as *const <IXMLDOMSchemaCollection as ::windows::core::Abi>::Abi as *const <IXMLDOMSchemaCollection as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Impl: IXMLDOMSchemaCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._newEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLDOMSchemaCollection>, base.5, add::<Impl, OFFSET>, get::<Impl, OFFSET>, remove::<Impl, OFFSET>, length::<Impl, OFFSET>, namespaceURI::<Impl, OFFSET>, addCollection::<Impl, OFFSET>, _newEnum::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMSchemaCollection2Impl: Sized + IXMLDOMSchemaCollectionImpl + IDispatchImpl {
    fn validate();
    fn SetvalidateOnLoad();
    fn validateOnLoad();
    fn getSchema();
    fn getDeclaration();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDOMSchemaCollection2 {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDOMSchemaCollection2";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMSchemaCollection2Vtbl {
    pub const fn new<Impl: IXMLDOMSchemaCollection2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDOMSchemaCollection2Vtbl {
        unsafe extern "system" fn validate<Impl: IXMLDOMSchemaCollection2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).validate() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetvalidateOnLoad<Impl: IXMLDOMSchemaCollection2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, validateonload: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetvalidateOnLoad(validateonload) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn validateOnLoad<Impl: IXMLDOMSchemaCollection2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, validateonload: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).validateOnLoad(::core::mem::transmute_copy(&validateonload)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getSchema<Impl: IXMLDOMSchemaCollection2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, schema: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getSchema(&*(&namespaceuri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&schema)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getDeclaration<Impl: IXMLDOMSchemaCollection2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, node: ::windows::core::RawPtr, item: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getDeclaration(&*(&node as *const <IXMLDOMNode as ::windows::core::Abi>::Abi as *const <IXMLDOMNode as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&item)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLDOMSchemaCollection2>, base.5, validate::<Impl, OFFSET>, SetvalidateOnLoad::<Impl, OFFSET>, validateOnLoad::<Impl, OFFSET>, getSchema::<Impl, OFFSET>, getDeclaration::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMSelectionImpl: Sized + IXMLDOMNodeListImpl + IDispatchImpl {
    fn expr();
    fn Setexpr();
    fn context();
    fn putref_context();
    fn peekNode();
    fn matches();
    fn removeNext();
    fn removeAll();
    fn clone();
    fn getProperty();
    fn setProperty();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDOMSelection {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDOMSelection";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMSelectionVtbl {
    pub const fn new<Impl: IXMLDOMSelectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDOMSelectionVtbl {
        unsafe extern "system" fn expr<Impl: IXMLDOMSelectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, expression: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).expr(::core::mem::transmute_copy(&expression)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setexpr<Impl: IXMLDOMSelectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, expression: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Setexpr(&*(&expression as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn context<Impl: IXMLDOMSelectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).context(::core::mem::transmute_copy(&ppnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn putref_context<Impl: IXMLDOMSelectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putref_context(&*(&pnode as *const <IXMLDOMNode as ::windows::core::Abi>::Abi as *const <IXMLDOMNode as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn peekNode<Impl: IXMLDOMSelectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).peekNode(::core::mem::transmute_copy(&ppnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn matches<Impl: IXMLDOMSelectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).matches(&*(&pnode as *const <IXMLDOMNode as ::windows::core::Abi>::Abi as *const <IXMLDOMNode as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeNext<Impl: IXMLDOMSelectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).removeNext(::core::mem::transmute_copy(&ppnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeAll<Impl: IXMLDOMSelectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).removeAll() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn clone<Impl: IXMLDOMSelectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).clone(::core::mem::transmute_copy(&ppnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getProperty<Impl: IXMLDOMSelectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getProperty(&*(&name as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&value)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setProperty<Impl: IXMLDOMSelectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, name: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, value: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).setProperty(&*(&name as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&value as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLDOMSelection>, base.5, expr::<Impl, OFFSET>, Setexpr::<Impl, OFFSET>, context::<Impl, OFFSET>, putref_context::<Impl, OFFSET>, peekNode::<Impl, OFFSET>, matches::<Impl, OFFSET>, removeNext::<Impl, OFFSET>, removeAll::<Impl, OFFSET>, clone::<Impl, OFFSET>, getProperty::<Impl, OFFSET>, setProperty::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDOMTextImpl: Sized + IXMLDOMCharacterDataImpl + IXMLDOMNodeImpl + IDispatchImpl {
    fn splitText();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDOMText {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDOMText";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDOMTextVtbl {
    pub const fn new<Impl: IXMLDOMTextImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDOMTextVtbl {
        unsafe extern "system" fn splitText<Impl: IXMLDOMTextImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, offset: i32, righthandtextnode: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).splitText(offset, ::core::mem::transmute_copy(&righthandtextnode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLDOMText>, base.5, splitText::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDSOControlImpl: Sized + IDispatchImpl {
    fn XMLDocument();
    fn SetXMLDocument();
    fn JavaDSOCompatible();
    fn SetJavaDSOCompatible();
    fn readyState();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDSOControl {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDSOControl";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDSOControlVtbl {
    pub const fn new<Impl: IXMLDSOControlImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDSOControlVtbl {
        unsafe extern "system" fn XMLDocument<Impl: IXMLDSOControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdoc: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).XMLDocument(::core::mem::transmute_copy(&ppdoc)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetXMLDocument<Impl: IXMLDSOControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppdoc: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetXMLDocument(&*(&ppdoc as *const <IXMLDOMDocument as ::windows::core::Abi>::Abi as *const <IXMLDOMDocument as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn JavaDSOCompatible<Impl: IXMLDSOControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fjavadsocompatible: *mut super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).JavaDSOCompatible(&*(&fjavadsocompatible as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetJavaDSOCompatible<Impl: IXMLDSOControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fjavadsocompatible: super::super::super::Foundation::BOOL) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetJavaDSOCompatible(&*(&fjavadsocompatible as *const <super::super::super::Foundation::BOOL as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BOOL as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn readyState<Impl: IXMLDSOControlImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, state: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).readyState(state) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLDSOControl>, base.5, XMLDocument::<Impl, OFFSET>, SetXMLDocument::<Impl, OFFSET>, JavaDSOCompatible::<Impl, OFFSET>, SetJavaDSOCompatible::<Impl, OFFSET>, readyState::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDocumentImpl: Sized + IDispatchImpl {
    fn root();
    fn fileSize();
    fn fileModifiedDate();
    fn fileUpdatedDate();
    fn URL();
    fn SetURL();
    fn mimeType();
    fn readyState();
    fn charset();
    fn Setcharset();
    fn version();
    fn doctype();
    fn dtdURL();
    fn createElement();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDocument {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDocument";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDocumentVtbl {
    pub const fn new<Impl: IXMLDocumentImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDocumentVtbl {
        unsafe extern "system" fn root<Impl: IXMLDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).root(::core::mem::transmute_copy(&p)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileSize<Impl: IXMLDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).fileSize(::core::mem::transmute_copy(&p)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileModifiedDate<Impl: IXMLDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).fileModifiedDate(::core::mem::transmute_copy(&p)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileUpdatedDate<Impl: IXMLDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).fileUpdatedDate(::core::mem::transmute_copy(&p)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn URL<Impl: IXMLDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).URL(::core::mem::transmute_copy(&p)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetURL<Impl: IXMLDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetURL(&*(&p as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn mimeType<Impl: IXMLDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).mimeType(::core::mem::transmute_copy(&p)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn readyState<Impl: IXMLDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).readyState(::core::mem::transmute_copy(&pl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn charset<Impl: IXMLDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).charset(::core::mem::transmute_copy(&p)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setcharset<Impl: IXMLDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Setcharset(&*(&p as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn version<Impl: IXMLDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).version(::core::mem::transmute_copy(&p)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn doctype<Impl: IXMLDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).doctype(::core::mem::transmute_copy(&p)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn dtdURL<Impl: IXMLDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).dtdURL(::core::mem::transmute_copy(&p)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createElement<Impl: IXMLDocumentImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vtype: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, var1: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ppelem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).createElement(&*(&vtype as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&var1 as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppelem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLDocument>, base.5, root::<Impl, OFFSET>, fileSize::<Impl, OFFSET>, fileModifiedDate::<Impl, OFFSET>, fileUpdatedDate::<Impl, OFFSET>, URL::<Impl, OFFSET>, SetURL::<Impl, OFFSET>, mimeType::<Impl, OFFSET>, readyState::<Impl, OFFSET>, charset::<Impl, OFFSET>, Setcharset::<Impl, OFFSET>, version::<Impl, OFFSET>, doctype::<Impl, OFFSET>, dtdURL::<Impl, OFFSET>, createElement::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLDocument2Impl: Sized + IDispatchImpl {
    fn root();
    fn fileSize();
    fn fileModifiedDate();
    fn fileUpdatedDate();
    fn URL();
    fn SetURL();
    fn mimeType();
    fn readyState();
    fn charset();
    fn Setcharset();
    fn version();
    fn doctype();
    fn dtdURL();
    fn createElement();
    fn r#async();
    fn Setasync();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLDocument2 {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLDocument2";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLDocument2Vtbl {
    pub const fn new<Impl: IXMLDocument2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLDocument2Vtbl {
        unsafe extern "system" fn root<Impl: IXMLDocument2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).root(::core::mem::transmute_copy(&p)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileSize<Impl: IXMLDocument2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).fileSize(::core::mem::transmute_copy(&p)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileModifiedDate<Impl: IXMLDocument2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).fileModifiedDate(::core::mem::transmute_copy(&p)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn fileUpdatedDate<Impl: IXMLDocument2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).fileUpdatedDate(::core::mem::transmute_copy(&p)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn URL<Impl: IXMLDocument2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).URL(::core::mem::transmute_copy(&p)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetURL<Impl: IXMLDocument2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetURL(&*(&p as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn mimeType<Impl: IXMLDocument2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).mimeType(::core::mem::transmute_copy(&p)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn readyState<Impl: IXMLDocument2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pl: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).readyState(::core::mem::transmute_copy(&pl)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn charset<Impl: IXMLDocument2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).charset(::core::mem::transmute_copy(&p)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setcharset<Impl: IXMLDocument2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Setcharset(&*(&p as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn version<Impl: IXMLDocument2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).version(::core::mem::transmute_copy(&p)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn doctype<Impl: IXMLDocument2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).doctype(::core::mem::transmute_copy(&p)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn dtdURL<Impl: IXMLDocument2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).dtdURL(::core::mem::transmute_copy(&p)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createElement<Impl: IXMLDocument2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vtype: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, var1: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ppelem: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).createElement(&*(&vtype as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&var1 as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppelem)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#async<Impl: IXMLDocument2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pf: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).r#async(::core::mem::transmute_copy(&pf)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setasync<Impl: IXMLDocument2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, f: i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Setasync(f) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IXMLDocument2>,
            base.5,
            root::<Impl, OFFSET>,
            fileSize::<Impl, OFFSET>,
            fileModifiedDate::<Impl, OFFSET>,
            fileUpdatedDate::<Impl, OFFSET>,
            URL::<Impl, OFFSET>,
            SetURL::<Impl, OFFSET>,
            mimeType::<Impl, OFFSET>,
            readyState::<Impl, OFFSET>,
            charset::<Impl, OFFSET>,
            Setcharset::<Impl, OFFSET>,
            version::<Impl, OFFSET>,
            doctype::<Impl, OFFSET>,
            dtdURL::<Impl, OFFSET>,
            createElement::<Impl, OFFSET>,
            r#async::<Impl, OFFSET>,
            Setasync::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLElementImpl: Sized + IDispatchImpl {
    fn tagName();
    fn SettagName();
    fn parent();
    fn setAttribute();
    fn getAttribute();
    fn removeAttribute();
    fn children();
    fn r#type();
    fn text();
    fn Settext();
    fn addChild();
    fn removeChild();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLElement {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLElement";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLElementVtbl {
    pub const fn new<Impl: IXMLElementImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLElementVtbl {
        unsafe extern "system" fn tagName<Impl: IXMLElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).tagName(::core::mem::transmute_copy(&p)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettagName<Impl: IXMLElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SettagName(&*(&p as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn parent<Impl: IXMLElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppparent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).parent(::core::mem::transmute_copy(&ppparent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttribute<Impl: IXMLElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).setAttribute(&*(&strpropertyname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&propertyvalue as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAttribute<Impl: IXMLElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getAttribute(&*(&strpropertyname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&propertyvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeAttribute<Impl: IXMLElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).removeAttribute(&*(&strpropertyname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn children<Impl: IXMLElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).children(::core::mem::transmute_copy(&pp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#type<Impl: IXMLElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).r#type(::core::mem::transmute_copy(&pltype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn text<Impl: IXMLElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).text(::core::mem::transmute_copy(&p)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settext<Impl: IXMLElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Settext(&*(&p as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addChild<Impl: IXMLElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchildelem: ::windows::core::RawPtr, lindex: i32, lreserved: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).addChild(&*(&pchildelem as *const <IXMLElement as ::windows::core::Abi>::Abi as *const <IXMLElement as ::windows::core::DefaultType>::DefaultType), lindex, lreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeChild<Impl: IXMLElementImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchildelem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).removeChild(&*(&pchildelem as *const <IXMLElement as ::windows::core::Abi>::Abi as *const <IXMLElement as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLElement>, base.5, tagName::<Impl, OFFSET>, SettagName::<Impl, OFFSET>, parent::<Impl, OFFSET>, setAttribute::<Impl, OFFSET>, getAttribute::<Impl, OFFSET>, removeAttribute::<Impl, OFFSET>, children::<Impl, OFFSET>, r#type::<Impl, OFFSET>, text::<Impl, OFFSET>, Settext::<Impl, OFFSET>, addChild::<Impl, OFFSET>, removeChild::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLElement2Impl: Sized + IDispatchImpl {
    fn tagName();
    fn SettagName();
    fn parent();
    fn setAttribute();
    fn getAttribute();
    fn removeAttribute();
    fn children();
    fn r#type();
    fn text();
    fn Settext();
    fn addChild();
    fn removeChild();
    fn attributes();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLElement2 {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLElement2";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLElement2Vtbl {
    pub const fn new<Impl: IXMLElement2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLElement2Vtbl {
        unsafe extern "system" fn tagName<Impl: IXMLElement2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).tagName(::core::mem::transmute_copy(&p)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SettagName<Impl: IXMLElement2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SettagName(&*(&p as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn parent<Impl: IXMLElement2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppparent: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).parent(::core::mem::transmute_copy(&ppparent)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setAttribute<Impl: IXMLElement2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).setAttribute(&*(&strpropertyname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&propertyvalue as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAttribute<Impl: IXMLElement2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, propertyvalue: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getAttribute(&*(&strpropertyname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&propertyvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeAttribute<Impl: IXMLElement2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, strpropertyname: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).removeAttribute(&*(&strpropertyname as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn children<Impl: IXMLElement2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).children(::core::mem::transmute_copy(&pp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn r#type<Impl: IXMLElement2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pltype: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).r#type(::core::mem::transmute_copy(&pltype)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn text<Impl: IXMLElement2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).text(::core::mem::transmute_copy(&p)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Settext<Impl: IXMLElement2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Settext(&*(&p as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addChild<Impl: IXMLElement2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchildelem: ::windows::core::RawPtr, lindex: i32, lreserved: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).addChild(&*(&pchildelem as *const <IXMLElement2 as ::windows::core::Abi>::Abi as *const <IXMLElement2 as ::windows::core::DefaultType>::DefaultType), lindex, lreserved) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn removeChild<Impl: IXMLElement2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pchildelem: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).removeChild(&*(&pchildelem as *const <IXMLElement2 as ::windows::core::Abi>::Abi as *const <IXMLElement2 as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn attributes<Impl: IXMLElement2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).attributes(::core::mem::transmute_copy(&pp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLElement2>, base.5, tagName::<Impl, OFFSET>, SettagName::<Impl, OFFSET>, parent::<Impl, OFFSET>, setAttribute::<Impl, OFFSET>, getAttribute::<Impl, OFFSET>, removeAttribute::<Impl, OFFSET>, children::<Impl, OFFSET>, r#type::<Impl, OFFSET>, text::<Impl, OFFSET>, Settext::<Impl, OFFSET>, addChild::<Impl, OFFSET>, removeChild::<Impl, OFFSET>, attributes::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLElementCollectionImpl: Sized + IDispatchImpl {
    fn Setlength();
    fn length();
    fn _newEnum();
    fn item();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLElementCollection {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLElementCollection";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLElementCollectionVtbl {
    pub const fn new<Impl: IXMLElementCollectionImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLElementCollectionVtbl {
        unsafe extern "system" fn Setlength<Impl: IXMLElementCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, v: i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Setlength(v) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn length<Impl: IXMLElementCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, p: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).length(::core::mem::transmute_copy(&p)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn _newEnum<Impl: IXMLElementCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppunk: *mut *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this)._newEnum(::core::mem::transmute_copy(&ppunk)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn item<Impl: IXMLElementCollectionImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, var1: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, var2: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, ppdisp: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).item(&*(&var1 as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), &*(&var2 as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppdisp)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLElementCollection>, base.5, Setlength::<Impl, OFFSET>, length::<Impl, OFFSET>, _newEnum::<Impl, OFFSET>, item::<Impl, OFFSET>)
    }
}
pub trait IXMLErrorImpl: Sized {
    fn GetErrorInfo();
}
impl ::windows::core::RuntimeName for IXMLError {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLError";
}
impl IXMLErrorVtbl {
    pub const fn new<Impl: IXMLErrorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLErrorVtbl {
        unsafe extern "system" fn GetErrorInfo<Impl: IXMLErrorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, perrorreturn: *mut XML_ERROR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetErrorInfo(&*(&perrorreturn as *const <XML_ERROR as ::windows::core::Abi>::Abi as *const <XML_ERROR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLError>, base.5, GetErrorInfo::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLHTTPRequestImpl: Sized + IDispatchImpl {
    fn open();
    fn setRequestHeader();
    fn getResponseHeader();
    fn getAllResponseHeaders();
    fn send();
    fn abort();
    fn status();
    fn statusText();
    fn responseXML();
    fn responseText();
    fn responseBody();
    fn responseStream();
    fn readyState();
    fn Setonreadystatechange();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLHTTPRequest {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLHTTPRequest";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLHTTPRequestVtbl {
    pub const fn new<Impl: IXMLHTTPRequestImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLHTTPRequestVtbl {
        unsafe extern "system" fn open<Impl: IXMLHTTPRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmethod: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varasync: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, bstruser: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, bstrpassword: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).open(
                &*(&bstrmethod as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrurl as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&varasync as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&bstruser as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrpassword as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setRequestHeader<Impl: IXMLHTTPRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrheader: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).setRequestHeader(&*(&bstrheader as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrvalue as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getResponseHeader<Impl: IXMLHTTPRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrheader: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getResponseHeader(&*(&bstrheader as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAllResponseHeaders<Impl: IXMLHTTPRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrheaders: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getAllResponseHeaders(::core::mem::transmute_copy(&pbstrheaders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn send<Impl: IXMLHTTPRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).send(&*(&varbody as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn abort<Impl: IXMLHTTPRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).abort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn status<Impl: IXMLHTTPRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).status(::core::mem::transmute_copy(&plstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn statusText<Impl: IXMLHTTPRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrstatus: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).statusText(::core::mem::transmute_copy(&pbstrstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseXML<Impl: IXMLHTTPRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbody: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).responseXML(::core::mem::transmute_copy(&ppbody)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseText<Impl: IXMLHTTPRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrbody: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).responseText(::core::mem::transmute_copy(&pbstrbody)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseBody<Impl: IXMLHTTPRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).responseBody(::core::mem::transmute_copy(&pvarbody)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseStream<Impl: IXMLHTTPRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).responseStream(::core::mem::transmute_copy(&pvarbody)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn readyState<Impl: IXMLHTTPRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).readyState(::core::mem::transmute_copy(&plstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setonreadystatechange<Impl: IXMLHTTPRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preadystatesink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Setonreadystatechange(&*(&preadystatesink as *const <super::super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IXMLHTTPRequest>,
            base.5,
            open::<Impl, OFFSET>,
            setRequestHeader::<Impl, OFFSET>,
            getResponseHeader::<Impl, OFFSET>,
            getAllResponseHeaders::<Impl, OFFSET>,
            send::<Impl, OFFSET>,
            abort::<Impl, OFFSET>,
            status::<Impl, OFFSET>,
            statusText::<Impl, OFFSET>,
            responseXML::<Impl, OFFSET>,
            responseText::<Impl, OFFSET>,
            responseBody::<Impl, OFFSET>,
            responseStream::<Impl, OFFSET>,
            readyState::<Impl, OFFSET>,
            Setonreadystatechange::<Impl, OFFSET>,
        )
    }
}
pub trait IXMLHTTPRequest2Impl: Sized {
    fn Open();
    fn Send();
    fn Abort();
    fn SetCookie();
    fn SetCustomResponseStream();
    fn SetProperty();
    fn SetRequestHeader();
    fn GetAllResponseHeaders();
    fn GetCookie();
    fn GetResponseHeader();
}
impl ::windows::core::RuntimeName for IXMLHTTPRequest2 {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLHTTPRequest2";
}
impl IXMLHTTPRequest2Vtbl {
    pub const fn new<Impl: IXMLHTTPRequest2Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLHTTPRequest2Vtbl {
        unsafe extern "system" fn Open<Impl: IXMLHTTPRequest2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszmethod: super::super::super::Foundation::PWSTR, pwszurl: super::super::super::Foundation::PWSTR, pstatuscallback: ::windows::core::RawPtr, pwszusername: super::super::super::Foundation::PWSTR, pwszpassword: super::super::super::Foundation::PWSTR, pwszproxyusername: super::super::super::Foundation::PWSTR, pwszproxypassword: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Open(
                &*(&pwszmethod as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pwszurl as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pstatuscallback as *const <IXMLHTTPRequest2Callback as ::windows::core::Abi>::Abi as *const <IXMLHTTPRequest2Callback as ::windows::core::DefaultType>::DefaultType),
                &*(&pwszusername as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pwszpassword as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pwszproxyusername as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&pwszproxypassword as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Send<Impl: IXMLHTTPRequest2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbody: ::windows::core::RawPtr, cbbody: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Send(&*(&pbody as *const <super::super::super::System::Com::ISequentialStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::ISequentialStream as ::windows::core::DefaultType>::DefaultType), cbbody) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Abort<Impl: IXMLHTTPRequest2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Abort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCookie<Impl: IXMLHTTPRequest2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pcookie: *const XHR_COOKIE, pdwcookiestate: *mut u32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCookie(&*(&pcookie as *const <XHR_COOKIE as ::windows::core::Abi>::Abi as *const <XHR_COOKIE as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pdwcookiestate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetCustomResponseStream<Impl: IXMLHTTPRequest2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, psequentialstream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetCustomResponseStream(&*(&psequentialstream as *const <super::super::super::System::Com::ISequentialStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::ISequentialStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetProperty<Impl: IXMLHTTPRequest2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, eproperty: XHR_PROPERTY, ullvalue: u64) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetProperty(eproperty, ullvalue) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn SetRequestHeader<Impl: IXMLHTTPRequest2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszheader: super::super::super::Foundation::PWSTR, pwszvalue: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetRequestHeader(&*(&pwszheader as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pwszvalue as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetAllResponseHeaders<Impl: IXMLHTTPRequest2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppwszheaders: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetAllResponseHeaders(::core::mem::transmute_copy(&ppwszheaders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetCookie<Impl: IXMLHTTPRequest2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszurl: super::super::super::Foundation::PWSTR, pwszname: super::super::super::Foundation::PWSTR, dwflags: u32, pccookies: *mut u32, ppcookies: *mut *mut XHR_COOKIE) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetCookie(&*(&pwszurl as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), &*(&pwszname as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), dwflags, ::core::mem::transmute_copy(&pccookies), ::core::mem::transmute_copy(&ppcookies)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetResponseHeader<Impl: IXMLHTTPRequest2Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pwszheader: super::super::super::Foundation::PWSTR, ppwszvalue: *mut *mut u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).GetResponseHeader(&*(&pwszheader as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&ppwszvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLHTTPRequest2>, base.5, Open::<Impl, OFFSET>, Send::<Impl, OFFSET>, Abort::<Impl, OFFSET>, SetCookie::<Impl, OFFSET>, SetCustomResponseStream::<Impl, OFFSET>, SetProperty::<Impl, OFFSET>, SetRequestHeader::<Impl, OFFSET>, GetAllResponseHeaders::<Impl, OFFSET>, GetCookie::<Impl, OFFSET>, GetResponseHeader::<Impl, OFFSET>)
    }
}
pub trait IXMLHTTPRequest2CallbackImpl: Sized {
    fn OnRedirect();
    fn OnHeadersAvailable();
    fn OnDataAvailable();
    fn OnResponseReceived();
    fn OnError();
}
impl ::windows::core::RuntimeName for IXMLHTTPRequest2Callback {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLHTTPRequest2Callback";
}
impl IXMLHTTPRequest2CallbackVtbl {
    pub const fn new<Impl: IXMLHTTPRequest2CallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLHTTPRequest2CallbackVtbl {
        unsafe extern "system" fn OnRedirect<Impl: IXMLHTTPRequest2CallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, pwszredirecturl: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnRedirect(&*(&pxhr as *const <IXMLHTTPRequest2 as ::windows::core::Abi>::Abi as *const <IXMLHTTPRequest2 as ::windows::core::DefaultType>::DefaultType), &*(&pwszredirecturl as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnHeadersAvailable<Impl: IXMLHTTPRequest2CallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, dwstatus: u32, pwszstatus: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnHeadersAvailable(&*(&pxhr as *const <IXMLHTTPRequest2 as ::windows::core::Abi>::Abi as *const <IXMLHTTPRequest2 as ::windows::core::DefaultType>::DefaultType), dwstatus, &*(&pwszstatus as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnDataAvailable<Impl: IXMLHTTPRequest2CallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, presponsestream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnDataAvailable(&*(&pxhr as *const <IXMLHTTPRequest2 as ::windows::core::Abi>::Abi as *const <IXMLHTTPRequest2 as ::windows::core::DefaultType>::DefaultType), &*(&presponsestream as *const <super::super::super::System::Com::ISequentialStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::ISequentialStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnResponseReceived<Impl: IXMLHTTPRequest2CallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, presponsestream: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnResponseReceived(&*(&pxhr as *const <IXMLHTTPRequest2 as ::windows::core::Abi>::Abi as *const <IXMLHTTPRequest2 as ::windows::core::DefaultType>::DefaultType), &*(&presponsestream as *const <super::super::super::System::Com::ISequentialStream as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::ISequentialStream as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnError<Impl: IXMLHTTPRequest2CallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, hrerror: ::windows::core::HRESULT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnError(&*(&pxhr as *const <IXMLHTTPRequest2 as ::windows::core::Abi>::Abi as *const <IXMLHTTPRequest2 as ::windows::core::DefaultType>::DefaultType), hrerror) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLHTTPRequest2Callback>, base.5, OnRedirect::<Impl, OFFSET>, OnHeadersAvailable::<Impl, OFFSET>, OnDataAvailable::<Impl, OFFSET>, OnResponseReceived::<Impl, OFFSET>, OnError::<Impl, OFFSET>)
    }
}
pub trait IXMLHTTPRequest3Impl: Sized + IXMLHTTPRequest2Impl {
    fn SetClientCertificate();
}
impl ::windows::core::RuntimeName for IXMLHTTPRequest3 {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLHTTPRequest3";
}
impl IXMLHTTPRequest3Vtbl {
    pub const fn new<Impl: IXMLHTTPRequest3Impl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLHTTPRequest3Vtbl {
        unsafe extern "system" fn SetClientCertificate<Impl: IXMLHTTPRequest3Impl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, cbclientcertificatehash: u32, pbclientcertificatehash: *const u8, pwszpin: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).SetClientCertificate(cbclientcertificatehash, pbclientcertificatehash, &*(&pwszpin as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLHTTPRequest3>, base.5, SetClientCertificate::<Impl, OFFSET>)
    }
}
pub trait IXMLHTTPRequest3CallbackImpl: Sized + IXMLHTTPRequest2CallbackImpl {
    fn OnServerCertificateReceived();
    fn OnClientCertificateRequested();
}
impl ::windows::core::RuntimeName for IXMLHTTPRequest3Callback {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLHTTPRequest3Callback";
}
impl IXMLHTTPRequest3CallbackVtbl {
    pub const fn new<Impl: IXMLHTTPRequest3CallbackImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLHTTPRequest3CallbackVtbl {
        unsafe extern "system" fn OnServerCertificateReceived<Impl: IXMLHTTPRequest3CallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, dwcertificateerrors: u32, cservercertificatechain: u32, rgservercertificatechain: *const XHR_CERT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnServerCertificateReceived(&*(&pxhr as *const <IXMLHTTPRequest3 as ::windows::core::Abi>::Abi as *const <IXMLHTTPRequest3 as ::windows::core::DefaultType>::DefaultType), dwcertificateerrors, cservercertificatechain, &*(&rgservercertificatechain as *const <XHR_CERT as ::windows::core::Abi>::Abi as *const <XHR_CERT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn OnClientCertificateRequested<Impl: IXMLHTTPRequest3CallbackImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pxhr: ::windows::core::RawPtr, cissuerlist: u32, rgpwszissuerlist: *const *const u16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).OnClientCertificateRequested(&*(&pxhr as *const <IXMLHTTPRequest3 as ::windows::core::Abi>::Abi as *const <IXMLHTTPRequest3 as ::windows::core::DefaultType>::DefaultType), cissuerlist, rgpwszissuerlist) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXMLHTTPRequest3Callback>, base.5, OnServerCertificateReceived::<Impl, OFFSET>, OnClientCertificateRequested::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXMLHttpRequestImpl: Sized + IDispatchImpl {
    fn open();
    fn setRequestHeader();
    fn getResponseHeader();
    fn getAllResponseHeaders();
    fn send();
    fn abort();
    fn status();
    fn statusText();
    fn responseXML();
    fn responseText();
    fn responseBody();
    fn responseStream();
    fn readyState();
    fn Setonreadystatechange();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXMLHttpRequest {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXMLHttpRequest";
}
#[cfg(feature = "Win32_System_Com")]
impl IXMLHttpRequestVtbl {
    pub const fn new<Impl: IXMLHttpRequestImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXMLHttpRequestVtbl {
        unsafe extern "system" fn open<Impl: IXMLHttpRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrmethod: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrurl: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, varasync: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, bstruser: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, bstrpassword: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).open(
                &*(&bstrmethod as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrurl as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&varasync as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&bstruser as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrpassword as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setRequestHeader<Impl: IXMLHttpRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrheader: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, bstrvalue: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).setRequestHeader(&*(&bstrheader as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&bstrvalue as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getResponseHeader<Impl: IXMLHttpRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrheader: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrvalue: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getResponseHeader(&*(&bstrheader as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&pbstrvalue)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn getAllResponseHeaders<Impl: IXMLHttpRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrheaders: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).getAllResponseHeaders(::core::mem::transmute_copy(&pbstrheaders)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn send<Impl: IXMLHttpRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, varbody: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).send(&*(&varbody as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn abort<Impl: IXMLHttpRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).abort() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn status<Impl: IXMLHttpRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstatus: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).status(::core::mem::transmute_copy(&plstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn statusText<Impl: IXMLHttpRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrstatus: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).statusText(::core::mem::transmute_copy(&pbstrstatus)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseXML<Impl: IXMLHttpRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppbody: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).responseXML(::core::mem::transmute_copy(&ppbody)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseText<Impl: IXMLHttpRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pbstrbody: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).responseText(::core::mem::transmute_copy(&pbstrbody)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseBody<Impl: IXMLHttpRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).responseBody(::core::mem::transmute_copy(&pvarbody)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn responseStream<Impl: IXMLHttpRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvarbody: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).responseStream(::core::mem::transmute_copy(&pvarbody)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn readyState<Impl: IXMLHttpRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, plstate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).readyState(::core::mem::transmute_copy(&plstate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setonreadystatechange<Impl: IXMLHttpRequestImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preadystatesink: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Setonreadystatechange(&*(&preadystatesink as *const <super::super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(
            base.0,
            base.1,
            base.2,
            base.3,
            ::windows::core::GetRuntimeClassName::<IXMLHttpRequest>,
            base.5,
            open::<Impl, OFFSET>,
            setRequestHeader::<Impl, OFFSET>,
            getResponseHeader::<Impl, OFFSET>,
            getAllResponseHeaders::<Impl, OFFSET>,
            send::<Impl, OFFSET>,
            abort::<Impl, OFFSET>,
            status::<Impl, OFFSET>,
            statusText::<Impl, OFFSET>,
            responseXML::<Impl, OFFSET>,
            responseText::<Impl, OFFSET>,
            responseBody::<Impl, OFFSET>,
            responseStream::<Impl, OFFSET>,
            readyState::<Impl, OFFSET>,
            Setonreadystatechange::<Impl, OFFSET>,
        )
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXSLProcessorImpl: Sized + IDispatchImpl {
    fn Setinput();
    fn input();
    fn ownerTemplate();
    fn setStartMode();
    fn startMode();
    fn startModeURI();
    fn Setoutput();
    fn output();
    fn transform();
    fn reset();
    fn readyState();
    fn addParameter();
    fn addObject();
    fn stylesheet();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXSLProcessor {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXSLProcessor";
}
#[cfg(feature = "Win32_System_Com")]
impl IXSLProcessorVtbl {
    pub const fn new<Impl: IXSLProcessorImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXSLProcessorVtbl {
        unsafe extern "system" fn Setinput<Impl: IXSLProcessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, var: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Setinput(&*(&var as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn input<Impl: IXSLProcessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pvar: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).input(::core::mem::transmute_copy(&pvar)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ownerTemplate<Impl: IXSLProcessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pptemplate: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ownerTemplate(::core::mem::transmute_copy(&pptemplate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn setStartMode<Impl: IXSLProcessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).setStartMode(&*(&mode as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&namespaceuri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn startMode<Impl: IXSLProcessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, mode: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).startMode(::core::mem::transmute_copy(&mode)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn startModeURI<Impl: IXSLProcessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, namespaceuri: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).startModeURI(::core::mem::transmute_copy(&namespaceuri)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn Setoutput<Impl: IXSLProcessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, output: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Setoutput(&*(&output as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn output<Impl: IXSLProcessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, poutput: *mut super::super::super::System::Com::VARIANT) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).output(::core::mem::transmute_copy(&poutput)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn transform<Impl: IXSLProcessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pdone: *mut i16) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).transform(::core::mem::transmute_copy(&pdone)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn reset<Impl: IXSLProcessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).reset() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn readyState<Impl: IXSLProcessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, preadystate: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).readyState(::core::mem::transmute_copy(&preadystate)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addParameter<Impl: IXSLProcessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, basename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, parameter: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).addParameter(
                &*(&basename as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&parameter as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&namespaceuri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn addObject<Impl: IXSLProcessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, obj: ::windows::core::RawPtr, namespaceuri: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).addObject(&*(&obj as *const <super::super::super::System::Com::IDispatch as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::IDispatch as ::windows::core::DefaultType>::DefaultType), &*(&namespaceuri as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn stylesheet<Impl: IXSLProcessorImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stylesheet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).stylesheet(::core::mem::transmute_copy(&stylesheet)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXSLProcessor>, base.5, Setinput::<Impl, OFFSET>, input::<Impl, OFFSET>, ownerTemplate::<Impl, OFFSET>, setStartMode::<Impl, OFFSET>, startMode::<Impl, OFFSET>, startModeURI::<Impl, OFFSET>, Setoutput::<Impl, OFFSET>, output::<Impl, OFFSET>, transform::<Impl, OFFSET>, reset::<Impl, OFFSET>, readyState::<Impl, OFFSET>, addParameter::<Impl, OFFSET>, addObject::<Impl, OFFSET>, stylesheet::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXSLTemplateImpl: Sized + IDispatchImpl {
    fn putref_stylesheet();
    fn stylesheet();
    fn createProcessor();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXSLTemplate {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXSLTemplate";
}
#[cfg(feature = "Win32_System_Com")]
impl IXSLTemplateVtbl {
    pub const fn new<Impl: IXSLTemplateImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXSLTemplateVtbl {
        unsafe extern "system" fn putref_stylesheet<Impl: IXSLTemplateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stylesheet: ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).putref_stylesheet(&*(&stylesheet as *const <IXMLDOMNode as ::windows::core::Abi>::Abi as *const <IXMLDOMNode as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn stylesheet<Impl: IXSLTemplateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, stylesheet: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).stylesheet(::core::mem::transmute_copy(&stylesheet)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn createProcessor<Impl: IXSLTemplateImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, ppprocessor: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).createProcessor(::core::mem::transmute_copy(&ppprocessor)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXSLTemplate>, base.5, putref_stylesheet::<Impl, OFFSET>, stylesheet::<Impl, OFFSET>, createProcessor::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait IXTLRuntimeImpl: Sized + IXMLDOMNodeImpl + IDispatchImpl {
    fn uniqueID();
    fn depth();
    fn childNumber();
    fn ancestorChildNumber();
    fn absoluteChildNumber();
    fn formatIndex();
    fn formatNumber();
    fn formatDate();
    fn formatTime();
}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for IXTLRuntime {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.IXTLRuntime";
}
#[cfg(feature = "Win32_System_Com")]
impl IXTLRuntimeVtbl {
    pub const fn new<Impl: IXTLRuntimeImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IXTLRuntimeVtbl {
        unsafe extern "system" fn uniqueID<Impl: IXTLRuntimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr, pid: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).uniqueID(&*(&pnode as *const <IXMLDOMNode as ::windows::core::Abi>::Abi as *const <IXMLDOMNode as ::windows::core::DefaultType>::DefaultType), pid) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn depth<Impl: IXTLRuntimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr, pdepth: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).depth(&*(&pnode as *const <IXMLDOMNode as ::windows::core::Abi>::Abi as *const <IXMLDOMNode as ::windows::core::DefaultType>::DefaultType), pdepth) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn childNumber<Impl: IXTLRuntimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr, pnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).childNumber(&*(&pnode as *const <IXMLDOMNode as ::windows::core::Abi>::Abi as *const <IXMLDOMNode as ::windows::core::DefaultType>::DefaultType), pnumber) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn ancestorChildNumber<Impl: IXTLRuntimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, bstrnodename: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pnode: ::windows::core::RawPtr, pnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).ancestorChildNumber(&*(&bstrnodename as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pnode as *const <IXMLDOMNode as ::windows::core::Abi>::Abi as *const <IXMLDOMNode as ::windows::core::DefaultType>::DefaultType), pnumber) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn absoluteChildNumber<Impl: IXTLRuntimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, pnode: ::windows::core::RawPtr, pnumber: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).absoluteChildNumber(&*(&pnode as *const <IXMLDOMNode as ::windows::core::Abi>::Abi as *const <IXMLDOMNode as ::windows::core::DefaultType>::DefaultType), pnumber) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn formatIndex<Impl: IXTLRuntimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, lindex: i32, bstrformat: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrformattedstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).formatIndex(lindex, &*(&bstrformat as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pbstrformattedstring as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn formatNumber<Impl: IXTLRuntimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, dblnumber: f64, bstrformat: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, pbstrformattedstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).formatNumber(dblnumber, &*(&bstrformat as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType), &*(&pbstrformattedstring as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn formatDate<Impl: IXTLRuntimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vardate: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, bstrformat: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, vardestlocale: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pbstrformattedstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).formatDate(
                &*(&vardate as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrformat as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&vardestlocale as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&pbstrformattedstring as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn formatTime<Impl: IXTLRuntimeImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, vartime: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, bstrformat: ::core::mem::ManuallyDrop<super::super::super::Foundation::BSTR>, vardestlocale: ::core::mem::ManuallyDrop<super::super::super::System::Com::VARIANT>, pbstrformattedstring: *mut super::super::super::Foundation::BSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).formatTime(
                &*(&vartime as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&bstrformat as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&vardestlocale as *const <super::super::super::System::Com::VARIANT as ::windows::core::Abi>::Abi as *const <super::super::super::System::Com::VARIANT as ::windows::core::DefaultType>::DefaultType),
                &*(&pbstrformattedstring as *const <super::super::super::Foundation::BSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::BSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IXTLRuntime>, base.5, uniqueID::<Impl, OFFSET>, depth::<Impl, OFFSET>, childNumber::<Impl, OFFSET>, ancestorChildNumber::<Impl, OFFSET>, absoluteChildNumber::<Impl, OFFSET>, formatIndex::<Impl, OFFSET>, formatNumber::<Impl, OFFSET>, formatDate::<Impl, OFFSET>, formatTime::<Impl, OFFSET>)
    }
}
#[cfg(feature = "Win32_System_Com")]
pub trait XMLDOMDocumentEventsImpl: Sized + IDispatchImpl {}
#[cfg(feature = "Win32_System_Com")]
impl ::windows::core::RuntimeName for XMLDOMDocumentEvents {
    const NAME: &'static str = "Windows.Win32.Data.Xml.MsXml.XMLDOMDocumentEvents";
}
#[cfg(feature = "Win32_System_Com")]
impl XMLDOMDocumentEventsVtbl {
    pub const fn new<Impl: XMLDOMDocumentEventsImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> XMLDOMDocumentEventsVtbl {
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<XMLDOMDocumentEvents>, base.5)
    }
}
