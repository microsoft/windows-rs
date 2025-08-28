#![cfg(target_env = "msvc")]

mod bindings;
pub use bindings::*;
pub use windows_core::*;

pub fn consume(test: &ITest) -> Result<()> {
    unsafe extern "system" {
        fn consume(test: Ref<ITest>) -> HRESULT;
    }

    unsafe { consume(test.into()).ok() }
}

pub fn produce() -> Result<ITest> {
    unsafe extern "system" {
        fn produce(test: OutRef<ITest>) -> HRESULT;
    }

    unsafe {
        let mut test = None;
        produce((&mut test).into()).ok()?;
        Type::from_default(&test)
    }
}
