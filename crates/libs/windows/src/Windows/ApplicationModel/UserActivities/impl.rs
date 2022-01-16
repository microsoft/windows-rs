pub trait IUserActivityContentInfo_Impl: Sized {
    fn ToJson(&mut self) -> ::windows::core::Result<::windows::core::HSTRING>;
}
impl ::windows::core::RuntimeName for IUserActivityContentInfo {
    const NAME: &'static str = "Windows.ApplicationModel.UserActivities.IUserActivityContentInfo";
}
impl IUserActivityContentInfo_Vtbl {
    pub const fn new<Identity: ::windows::core::IUnknownImpl, Impl: IUserActivityContentInfo_Impl, const BASE_OFFSET: isize, const IMPL_OFFSET: isize>() -> IUserActivityContentInfo_Vtbl {
        unsafe extern "system" fn ToJson<Impl: IUserActivityContentInfo_Impl, const OFFSET: isize>(this: *mut ::core::ffi::c_void, result__: *mut ::core::mem::ManuallyDrop<::windows::core::HSTRING>) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).offset(OFFSET) as *mut Impl;
            match (*this).ToJson() {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self { base: ::windows::core::IInspectableVtbl::new::<Identity, IUserActivityContentInfo, BASE_OFFSET>(), ToJson: ToJson::<Impl, IMPL_OFFSET> }
    }
    pub fn matches(iid: &windows::core::GUID) -> bool {
        iid == &<IUserActivityContentInfo as ::windows::core::Interface>::IID
    }
}
