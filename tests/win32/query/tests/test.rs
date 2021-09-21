// Remove when upstream metadata generator supports other targets
#![cfg(all(windows, target_pointer_width = "64"))]

use test_win32_query::*;
use windows::*;
use Component::Win32::Query::*;
use Windows::Win32::Foundation::E_NOINTERFACE;

#[test]
fn query_interface() -> Result<()> {
    unsafe {
        let query: IQueryInt32 = QueryInterface()?;
        assert!(query.GetInt32() == 123);

        let query: IQuerySingle = QueryInterface()?;
        assert!(query.GetSingle() == 1.23);

        QueryInterface::<IUnknown>()?;

        assert!(QueryInterface::<IInspectable>().unwrap_err().code() == E_NOINTERFACE);

        Ok(())
    }
}

#[test]
fn query_with_value() -> Result<()> {
    unsafe {
        let query: IQueryInt32 = QueryWithValue(456)?;
        assert!(query.GetInt32() == 456);

        Ok(())
    }
}
