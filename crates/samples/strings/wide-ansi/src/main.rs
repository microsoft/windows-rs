use windows_strings::*;

const ANSI: PCSTR = s!("ansi");
const WIDE: PCWSTR = w!("wide");

fn main() {
    unsafe {
        println!("ansi: {}", ANSI.to_string().unwrap());
        println!("wide: {}", WIDE.to_string().unwrap());
    }
}
