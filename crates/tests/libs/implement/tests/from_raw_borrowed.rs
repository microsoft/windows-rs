#![expect(non_snake_case)]

use windows::{core::*, Win32::System::Com::*, Win32::UI::Shell::*};

#[interface("a563f463-3d23-42cd-a2b5-6d21ee898aae")]
unsafe trait IBorrowed: IUnknown {
    unsafe fn Call(&self) -> u32;
}

#[implement(IServiceProvider, IBorrowed, IProfferService)]
struct Borrowed(u32);

impl IBorrowed_Impl for Borrowed_Impl {
    unsafe fn Call(&self) -> u32 {
        self.0
    }
}

impl IServiceProvider_Impl for Borrowed_Impl {
    fn QueryService(
        &self,
        _service: *const GUID,
        iid: *const GUID,
        object: *mut *mut std::ffi::c_void,
    ) -> Result<()> {
        let unknown = self.as_interface::<IUnknown>();
        unsafe { unknown.query(iid, object).ok() }
    }
}

impl IProfferService_Impl for Borrowed_Impl {
    fn ProfferService(&self, _: *const GUID, provider: Ref<IServiceProvider>) -> Result<u32> {
        unsafe {
            if let Ok(provider) = provider.ok() {
                Ok(provider.cast::<IBorrowed>()?.Call())
            } else {
                Ok(0)
            }
        }
    }

    fn RevokeService(&self, _: u32) -> Result<()> {
        Ok(())
    }
}

#[test]
fn test() -> Result<()> {
    unsafe {
        let one_two_three: IBorrowed = Borrowed(123).into();
        assert_eq!(one_two_three.Call(), 123);

        let four_five_six: IBorrowed = Borrowed(456).into();
        assert_eq!(four_five_six.Call(), 456);

        let service = one_two_three.cast::<IProfferService>()?;

        assert_eq!(service.ProfferService(&GUID::zeroed(), None)?, 0);

        assert_eq!(
            service.ProfferService(&GUID::zeroed(), &one_two_three.cast::<IServiceProvider>()?)?,
            123
        );
        assert_eq!(
            service.ProfferService(&GUID::zeroed(), &four_five_six.cast::<IServiceProvider>()?)?,
            456
        );

        Ok(())
    }
}
