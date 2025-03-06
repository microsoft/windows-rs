use windows::{
    core::*, Foundation::*, Win32::Foundation::*, Win32::System::Com::Marshal::*,
    Win32::System::Com::*, Win32::System::WinRT::*,
};

#[implement(IStringable)]
struct Stringable;

impl IStringable_Impl for Stringable_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        todo!()
    }
}

#[test]
fn test() {
    unsafe {
        // This covers the four distinct implementations of QueryInterface.

        let delegate: EventHandler<i32> = EventHandler::<i32>::new(move |_, _| todo!());
        test_query(&delegate);

        let interface: IStringable = Stringable.into();
        test_query(&interface);

        let source: IWeakReferenceSource = interface.cast().unwrap();
        test_query(&source);

        let weak = source.GetWeakReference().unwrap();
        test_query(&weak);
    }
}

fn test_query<I: Interface>(interface: &I) {
    unsafe {
        // Successful query
        {
            let mut unknown: Option<IUnknown> = None;
            let hr = interface.query(&IUnknown::IID, &mut unknown as *mut _ as *mut _);
            assert_eq!(hr, S_OK);
            assert_eq!(interface.cast::<IUnknown>().unwrap(), unknown.unwrap());
        }

        // Unsuccessful query
        {
            let mut closable: Option<IClosable> = None;
            let hr = interface.query(&IClosable::IID, &mut closable as *mut _ as *mut _);
            assert_eq!(hr, E_NOINTERFACE);
            assert_eq!(closable, None);
        }

        // iid param is null
        {
            let mut unknown: Option<IUnknown> = None;
            let hr = interface.query(std::ptr::null(), &mut unknown as *mut _ as *mut _);
            assert_eq!(hr, E_POINTER);
            assert_eq!(unknown, None);
        }

        // interface param is null
        {
            let hr = interface.query(&IUnknown::IID, std::ptr::null_mut());
            assert_eq!(hr, E_POINTER);
        }

        // Implementations support IAgileObject and IMarshal
        {
            interface.cast::<IAgileObject>().expect("IAgileObject");
            interface.cast::<IMarshal>().expect("IMarshal");
        }
    }
}
