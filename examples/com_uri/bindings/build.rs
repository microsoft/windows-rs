fn main() {
    windows::build! {
        Windows::Win32::System::Com::{CreateUri} // TODO: this test should fail to compile without IUri
    };
}
