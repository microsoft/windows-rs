use windows_strings::*;

fn main() {
    let literal = h!("Hello");
    let owned = HSTRING::from("Hello");

    assert_eq!(literal, &owned);
    assert_eq!(owned, "Hello");

    println!("text: {}", owned.to_string_lossy());
    println!("len:  {}", owned.len());
}
