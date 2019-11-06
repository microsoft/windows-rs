use winrt::Stringable;

struct Pancakes;

impl Stringable for Pancakes {
    fn to_string() {
        println!("Hello Pancakes!");
    }
}

fn main() {
    Pancakes::to_string();
}
