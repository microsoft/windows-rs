#[cfg(feature = "implement_exclusive")]
pub trait IPhoneNumberFormatterImpl: Sized {
    fn Format(&self, number: &::core::option::Option<PhoneNumberInfo>) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormatWithOutputFormat(&self, number: &::core::option::Option<PhoneNumberInfo>, numberformat: PhoneNumberFormat) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormatPartialString(&self, number: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormatString(&self, number: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn FormatStringWithLeftToRightMarkers(&self, number: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneNumberFormatter {
    const NAME: &'static str = "Windows.Globalization.PhoneNumberFormatting.IPhoneNumberFormatter";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneNumberFormatterVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneNumberFormatterImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneNumberFormatterVtbl {
        unsafe extern "system" fn Format<Impl: IPhoneNumberFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, number: ::windows::core::RawPtr, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Format(&*(&number as *const <PhoneNumberInfo as ::windows::core::Abi>::Abi as *const <PhoneNumberInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatWithOutputFormat<Impl: IPhoneNumberFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, number: ::windows::core::RawPtr, numberformat: PhoneNumberFormat, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatWithOutputFormat(&*(&number as *const <PhoneNumberInfo as ::windows::core::Abi>::Abi as *const <PhoneNumberInfo as ::windows::core::DefaultType>::DefaultType), numberformat) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatPartialString<Impl: IPhoneNumberFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, number: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatPartialString(&*(&number as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatString<Impl: IPhoneNumberFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, number: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatString(&*(&number as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn FormatStringWithLeftToRightMarkers<Impl: IPhoneNumberFormatterImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, number: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).FormatStringWithLeftToRightMarkers(&*(&number as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneNumberFormatter, BASE_OFFSET>(),
            Format: Format::<Impl, IMPL_OFFSET>,
            FormatWithOutputFormat: FormatWithOutputFormat::<Impl, IMPL_OFFSET>,
            FormatPartialString: FormatPartialString::<Impl, IMPL_OFFSET>,
            FormatString: FormatString::<Impl, IMPL_OFFSET>,
            FormatStringWithLeftToRightMarkers: FormatStringWithLeftToRightMarkers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneNumberFormatter as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneNumberFormatterStaticsImpl: Sized {
    fn TryCreate(&self, regioncode: &::windows::core::HSTRING, phonenumber: &mut ::core::option::Option<PhoneNumberFormatter>) -> ::windows::core::Result<()>;
    fn GetCountryCodeForRegion(&self, regioncode: &::windows::core::HSTRING) -> ::windows::core::Result<i32>;
    fn GetNationalDirectDialingPrefixForRegion(&self, regioncode: &::windows::core::HSTRING, stripnondigit: bool) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn WrapWithLeftToRightMarkers(&self, number: &::windows::core::HSTRING) -> ::windows::core::Result<::windows::core::HSTRING>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneNumberFormatterStatics {
    const NAME: &'static str = "Windows.Globalization.PhoneNumberFormatting.IPhoneNumberFormatterStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneNumberFormatterStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneNumberFormatterStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneNumberFormatterStaticsVtbl {
        unsafe extern "system" fn TryCreate<Impl: IPhoneNumberFormatterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, regioncode: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, phonenumber: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            (*this).TryCreate(&*(&regioncode as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phonenumber)).into()
        }
        unsafe extern "system" fn GetCountryCodeForRegion<Impl: IPhoneNumberFormatterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, regioncode: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetCountryCodeForRegion(&*(&regioncode as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNationalDirectDialingPrefixForRegion<Impl: IPhoneNumberFormatterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, regioncode: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, stripnondigit: bool, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNationalDirectDialingPrefixForRegion(&*(&regioncode as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), stripnondigit) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn WrapWithLeftToRightMarkers<Impl: IPhoneNumberFormatterStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, number: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).WrapWithLeftToRightMarkers(&*(&number as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneNumberFormatterStatics, BASE_OFFSET>(),
            TryCreate: TryCreate::<Impl, IMPL_OFFSET>,
            GetCountryCodeForRegion: GetCountryCodeForRegion::<Impl, IMPL_OFFSET>,
            GetNationalDirectDialingPrefixForRegion: GetNationalDirectDialingPrefixForRegion::<Impl, IMPL_OFFSET>,
            WrapWithLeftToRightMarkers: WrapWithLeftToRightMarkers::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneNumberFormatterStatics as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneNumberInfoImpl: Sized {
    fn CountryCode(&self) -> ::windows::core::Result<i32>;
    fn PhoneNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetLengthOfGeographicalAreaCode(&self) -> ::windows::core::Result<i32>;
    fn GetNationalSignificantNumber(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn GetLengthOfNationalDestinationCode(&self) -> ::windows::core::Result<i32>;
    fn PredictNumberKind(&self) -> ::windows::core::Result<PredictedPhoneNumberKind>;
    fn GetGeographicRegionCode(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn CheckNumberMatch(&self, othernumber: &::core::option::Option<PhoneNumberInfo>) -> ::windows::core::Result<PhoneNumberMatchResult>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneNumberInfo {
    const NAME: &'static str = "Windows.Globalization.PhoneNumberFormatting.IPhoneNumberInfo";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneNumberInfoVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneNumberInfoImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneNumberInfoVtbl {
        unsafe extern "system" fn CountryCode<Impl: IPhoneNumberInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CountryCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PhoneNumber<Impl: IPhoneNumberInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PhoneNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLengthOfGeographicalAreaCode<Impl: IPhoneNumberInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLengthOfGeographicalAreaCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetNationalSignificantNumber<Impl: IPhoneNumberInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetNationalSignificantNumber() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetLengthOfNationalDestinationCode<Impl: IPhoneNumberInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut i32) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetLengthOfNationalDestinationCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn PredictNumberKind<Impl: IPhoneNumberInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut PredictedPhoneNumberKind) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).PredictNumberKind() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn GetGeographicRegionCode<Impl: IPhoneNumberInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).GetGeographicRegionCode() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn CheckNumberMatch<Impl: IPhoneNumberInfoImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, othernumber: ::windows::core::RawPtr, result__: *mut PhoneNumberMatchResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).CheckNumberMatch(&*(&othernumber as *const <PhoneNumberInfo as ::windows::core::Abi>::Abi as *const <PhoneNumberInfo as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneNumberInfo, BASE_OFFSET>(),
            CountryCode: CountryCode::<Impl, IMPL_OFFSET>,
            PhoneNumber: PhoneNumber::<Impl, IMPL_OFFSET>,
            GetLengthOfGeographicalAreaCode: GetLengthOfGeographicalAreaCode::<Impl, IMPL_OFFSET>,
            GetNationalSignificantNumber: GetNationalSignificantNumber::<Impl, IMPL_OFFSET>,
            GetLengthOfNationalDestinationCode: GetLengthOfNationalDestinationCode::<Impl, IMPL_OFFSET>,
            PredictNumberKind: PredictNumberKind::<Impl, IMPL_OFFSET>,
            GetGeographicRegionCode: GetGeographicRegionCode::<Impl, IMPL_OFFSET>,
            CheckNumberMatch: CheckNumberMatch::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneNumberInfo as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneNumberInfoFactoryImpl: Sized {
    fn Create(&self, number: &::windows::core::HSTRING) -> ::windows::core::Result<PhoneNumberInfo>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneNumberInfoFactory {
    const NAME: &'static str = "Windows.Globalization.PhoneNumberFormatting.IPhoneNumberInfoFactory";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneNumberInfoFactoryVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneNumberInfoFactoryImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneNumberInfoFactoryVtbl {
        unsafe extern "system" fn Create<Impl: IPhoneNumberInfoFactoryImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, number: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, result__: *mut ::windows::core::RawPtr) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).Create(&*(&number as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneNumberInfoFactory, BASE_OFFSET>(), Create: Create::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneNumberInfoFactory as ::windows::core::Interface>::IID
    }
}
#[cfg(feature = "implement_exclusive")]
pub trait IPhoneNumberInfoStaticsImpl: Sized {
    fn TryParse(&self, input: &::windows::core::HSTRING, phonenumber: &mut ::core::option::Option<PhoneNumberInfo>) -> ::windows::core::Result<PhoneNumberParseResult>;
    fn TryParseWithRegion(&self, input: &::windows::core::HSTRING, regioncode: &::windows::core::HSTRING, phonenumber: &mut ::core::option::Option<PhoneNumberInfo>) -> ::windows::core::Result<PhoneNumberParseResult>;
}
#[cfg(feature = "implement_exclusive")]
impl ::windows::core::RuntimeName for IPhoneNumberInfoStatics {
    const NAME: &'static str = "Windows.Globalization.PhoneNumberFormatting.IPhoneNumberInfoStatics";
}
#[cfg(feature = "implement_exclusive")]
impl IPhoneNumberInfoStaticsVtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IPhoneNumberInfoStaticsImpl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IPhoneNumberInfoStaticsVtbl {
        unsafe extern "system" fn TryParse<Impl: IPhoneNumberInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, phonenumber: *mut ::windows::core::RawPtr, result__: *mut PhoneNumberParseResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParse(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phonenumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        unsafe extern "system" fn TryParseWithRegion<Impl: IPhoneNumberInfoStaticsImpl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, input: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, regioncode: ::core::mem::ManuallyDrop<::windows::core::HSTRING>, phonenumber: *mut ::windows::core::RawPtr, result__: *mut PhoneNumberParseResult) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).TryParseWithRegion(&*(&input as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), &*(&regioncode as *const <::windows::core::HSTRING as ::windows::core::Abi>::Abi as *const <::windows::core::HSTRING as ::windows::core::DefaultType>::DefaultType), ::core::mem::transmute_copy(&phonenumber)) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self {
            base: ::windows::core::IInspectableVtbl::new::<Identity, IPhoneNumberInfoStatics, BASE_OFFSET>(),
            TryParse: TryParse::<Impl, IMPL_OFFSET>,
            TryParseWithRegion: TryParseWithRegion::<Impl, IMPL_OFFSET>,
        }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IPhoneNumberInfoStatics as ::windows::core::Interface>::IID
    }
}
