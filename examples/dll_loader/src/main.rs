use bindings::Windows::Win32::System::SystemServices::LoadLibraryA;

fn main() {
    let file_name = std::env::args()
        .nth(1)
        .expect("expected a dll filename as program argument");

    let library = unsafe { LoadLibraryA(file_name.as_str()) };

    if library.is_null() {
        println!("Library with filename {} does not exist", file_name)
    }
}
