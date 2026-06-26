use windows_result::*;

fn main() {
    let cancelled = WIN32_ERROR(1223).to_hresult();
    let error = Error::new(cancelled, "operation cancelled");

    println!("code:    {:#010x}", error.code().0);
    println!("message: {}", error.message());
}
