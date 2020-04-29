/// A WinRT type that can be identified by a name
pub trait RuntimeName {
    const NAME: &'static str;
}
