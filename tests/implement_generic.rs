// mod windows {
//     pub use winrt::foundation;
// }

// #[test]
// fn implement() -> winrt::Result<()> {

//     Ok(())
// }

// //#[winrt::implement(windows::foundation::collections::{IIterator<i32>})]
// struct TestIterator{
//     values:Vec<i32>,
//     current: u32,
// }

// impl TestIterator {
//     fn current(&self) -> winrt::Result<i32> {
//         Ok(0)
//     }

//     fn has_current(&self) -> winrt::Result<bool> {
//         Ok(false)
//     }

//     fn move_next(&self) -> winrt::Result<()> {
//         Ok(())
//     }

//     fn get_many(&self, values: &mut [i32]) -> winrt::Result<u32> {
//         panic!();
//     }
// }
