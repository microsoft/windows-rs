use crate::runtime::*;

pub struct Error {
    code: i32,
    // IErrorInfo*
}

pub type Result<T> = std::result::Result<T, Error>;

// fn ensure_hr(hr: HRESULT) {
//     if hr != 0 {
//         panic!("Failed to load COM");
//     }
// }

// fn check_hr(hr: HRESULT) -> Result<()> {
//     if hr < 0 {
//         Err(Error::code(hr))
//     }
//     else {
//         Ok(())
//     }
// }
