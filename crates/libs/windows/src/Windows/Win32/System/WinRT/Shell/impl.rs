pub trait IDDEInitializerImpl: Sized {
    fn Initialize();
}
impl ::windows::core::RuntimeName for IDDEInitializer {
    const NAME: &'static str = "Windows.Win32.System.WinRT.Shell.IDDEInitializer";
}
impl IDDEInitializerVtbl {
    pub const fn new<Impl: IDDEInitializerImpl, const OFFSET: usize>(base: &::windows::core::IInspectableVtbl) -> IDDEInitializerVtbl {
        unsafe extern "system" fn Initialize<Impl: IDDEInitializerImpl, const OFFSET: usize>(this: *mut ::core::ffi::c_void, fileextensionorprotocol: super::super::super::Foundation::PWSTR, method: CreateProcessMethod, currentdirectory: super::super::super::Foundation::PWSTR, exectarget: ::windows::core::RawPtr, site: *mut ::core::ffi::c_void, application: super::super::super::Foundation::PWSTR, targetfile: super::super::super::Foundation::PWSTR, arguments: super::super::super::Foundation::PWSTR, verb: super::super::super::Foundation::PWSTR) -> ::windows::core::HRESULT {
            let this = (this as *mut ::windows::core::RawPtr).add(OFFSET) as *mut Impl;
            match (*this).Initialize(
                &*(&fileextensionorprotocol as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                method,
                &*(&currentdirectory as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&exectarget as *const <super::super::super::UI::Shell::IShellItem as ::windows::core::Abi>::Abi as *const <super::super::super::UI::Shell::IShellItem as ::windows::core::DefaultType>::DefaultType),
                &*(&site as *const <::windows::core::IUnknown as ::windows::core::Abi>::Abi as *const <::windows::core::IUnknown as ::windows::core::DefaultType>::DefaultType),
                &*(&application as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&targetfile as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&arguments as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
                &*(&verb as *const <super::super::super::Foundation::PWSTR as ::windows::core::Abi>::Abi as *const <super::super::super::Foundation::PWSTR as ::windows::core::DefaultType>::DefaultType),
            ) {
                ::core::result::Result::Ok(ok__) => {
                    *result__ = ::core::mem::transmute_copy(&ok__);
                    ::core::mem::forget(ok__);
                    ::windows::core::HRESULT(0)
                }
                ::core::result::Result::Err(err) => err.into(),
            }
        }
        Self(base.0, base.1, base.2, base.3, ::windows::core::GetRuntimeClassName::<IDDEInitializer>, base.5, Initialize::<Impl, OFFSET>)
    }
}
