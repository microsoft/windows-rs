// Remove when upstream metadata generator supports other targets
#![cfg(all(windows, target_pointer_width = "64"))]

use test_win32_query::*;
use windows::*;
use Component::Win32::Query::*;
use Windows::Win32::Foundation::E_NOINTERFACE;

#[test]
fn query_function() -> Result<()> {
    unsafe {
        let query: IQueryInt32 = Query()?;
        assert!(query.GetInt32() == 123);

        let query: IQuerySingle = Query()?;
        assert!(query.GetSingle() == 1.23);

        Query::<IUnknown>()?;

        assert!(Query::<IInspectable>().unwrap_err().code() == E_NOINTERFACE);

        Ok(())
    }
}

#[test]
fn query_member() -> Result<()> {
    unsafe {
        let object: IQueryInt32 = Query()?;

        let query: IQueryInt32 = object.Query()?;
        assert!(query.GetInt32() == 123);

        let query: IQuerySingle = object.Query()?;
        assert!(query.GetSingle() == 1.23);

        object.Query::<IUnknown>()?;

        assert!(object.Query::<IInspectable>().unwrap_err().code() == E_NOINTERFACE);

        Ok(())
    }
}

#[test]
fn query_function_with_value() -> Result<()> {
    unsafe {
        // Validates that leading parameters don't confuse signature detection.
        let query: IQueryInt32 = QueryWithValue(456)?;
        assert!(query.GetInt32() == 456);

        Ok(())
    }
}

#[test]
fn query_member_with_value() -> Result<()> {
    unsafe {
        let object: IQueryInt32 = QueryWithValue(456)?;

        // Validates that leading parameters don't confuse signature detection.
        let query: IQueryInt32 = object.QueryWithValue(456)?;
        assert!(query.GetInt32() == 456);

        Ok(())
    }
}

#[test]
fn query_function_optional() -> Result<()> {
    unsafe {
        let mut query: Option<IQueryInt32> = None;
        QueryOptional(&mut query)?;
        assert!(query.unwrap().GetInt32() == 123);

        let mut query = None;
        QueryOptional::<IQueryInt32>(&mut query)?;
        assert!(query.unwrap().GetInt32() == 123);

        // Query test success without returning an object.
        QueryOptional::<IQueryInt32>(std::ptr::null_mut())?;

        // Query test failure without returning an object.
        assert!(QueryOptional::<IInspectable>(std::ptr::null_mut()).unwrap_err().code() == E_NOINTERFACE);

        Ok(())
    }
}

#[test]
fn query_member_optional() -> Result<()> {
    unsafe {
        let object: IQueryInt32 = Query()?;

        let mut query: Option<IQueryInt32> = None;
        object.QueryOptional(&mut query)?;
        assert!(query.unwrap().GetInt32() == 123);

        let mut query = None;
        object.QueryOptional::<IQueryInt32>(&mut query)?;
        assert!(query.unwrap().GetInt32() == 123);

        // Query test success without returning an object.
        object.QueryOptional::<IQueryInt32>(std::ptr::null_mut())?;

        // Query test failure without returning an object.
        assert!(object.QueryOptional::<IInspectable>(std::ptr::null_mut()).unwrap_err().code() == E_NOINTERFACE);

        Ok(())
    }
}
