use windows::{core::*, Foundation::*, Win32::System::Com::Marshal::*, Win32::System::Com::*};

#[implement(IClassFactory)]
struct Factory;

impl IClassFactory_Impl for Factory_Impl {
    fn CreateInstance(
        &self,
        outer: Ref<IUnknown>,
        iid: *const GUID,
        interface: *mut *mut core::ffi::c_void,
    ) -> Result<()> {
        unsafe {
            assert!(outer.is_null(), "aggregation not supported");
            let unknown: IUnknown = Stringable.into();
            unknown.query(iid, interface).ok()
        }
    }

    fn LockServer(&self, lock: BOOL) -> Result<()> {
        assert!(lock.as_bool());
        Ok(())
    }
}

#[implement(IStringable)]
struct Stringable;

impl IStringable_Impl for Stringable_Impl {
    fn ToString(&self) -> Result<HSTRING> {
        Ok("Stringable".into())
    }
}

const CLSID_SAMPLE: GUID = GUID::from_u128(0x58d39529_4975_4440_b7a6_6d71bfb59559);

#[test]
fn test() {
    unsafe {
        CoIncrementMTAUsage().unwrap();
        let factory: IClassFactory = Factory.into();
        factory.cast::<IAgileObject>().expect("IAgileObject");
        factory.cast::<IMarshal>().expect("IMarshal");

        CoRegisterClassObject(
            &CLSID_SAMPLE,
            &factory,
            CLSCTX_LOCAL_SERVER,
            REGCLS_AGILE | REGCLS_MULTIPLEUSE,
        )
        .expect("CoRegisterClassObject");

        let stringable: IStringable =
            CoCreateInstance(&CLSID_SAMPLE, None, CLSCTX_LOCAL_SERVER).expect("CoCreateInstance");

        assert_eq!(stringable.ToString().unwrap(), "Stringable");

        let stream = CoMarshalInterThreadInterfaceInStream(&IStringable::IID, &stringable).unwrap();
        let copy: IStringable = CoUnmarshalInterface(&stream).unwrap();

        assert_eq!(copy.ToString().unwrap(), "Stringable");
        assert_eq!(stringable, copy);

        assert_eq!(copy, copy.downgrade().unwrap().upgrade().unwrap());
    }
}
