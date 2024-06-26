use crate::*;

#[test]
fn show_sizes() {
    use core::mem::size_of;

    macro_rules! show_size {
        ($t:ty) => {
            println!("size_of {} = {}", stringify!($t), size_of::<$t>());
        };
    }

    println!("sizes:");
    show_size!(Error);
    show_size!(Result<()>);
    show_size!(Result<u32>);
    show_size!(Result<String>);
}
