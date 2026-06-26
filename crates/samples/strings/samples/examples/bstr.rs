use windows_strings::*;

fn main() {
    let bstr = BSTR::from("BSTR value");

    assert_eq!(bstr, "BSTR value");

    println!("text: {}", bstr.display());
    println!("len:  {}", bstr.len());
}
