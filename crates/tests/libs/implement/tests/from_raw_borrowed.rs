#![cfg(windows)]
#![expect(non_snake_case)]

use windows::{Win32::*, core::*};

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
    fn ProfferService(
        &self,
        _: *const GUID,
        provider: Ref<IServiceProvider>,
        cookie: *mut u32,
    ) -> Result<()> {
        unsafe {
            *cookie = if let Ok(provider) = provider.ok() {
                provider.cast::<IBorrowed>()?.Call()
            } else {
                0
            };
            Ok(())
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

        let mut cookie = u32::MAX;
        service
            .ProfferService(&GUID::zeroed(), None, &mut cookie)
            .ok()?;
        assert_eq!(cookie, 0);

        service
            .ProfferService(
                &GUID::zeroed(),
                &one_two_three.cast::<IServiceProvider>()?,
                &mut cookie,
            )
            .ok()?;
        assert_eq!(cookie, 123);

        service
            .ProfferService(
                &GUID::zeroed(),
                &four_five_six.cast::<IServiceProvider>()?,
                &mut cookie,
            )
            .ok()?;
        assert_eq!(cookie, 456);

        Ok(())
    }
}
