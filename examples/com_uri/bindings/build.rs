fn main() {
    windows::build! {
        Windows::Win32::System::Com::{CreateUri, IUri},
    };
}
