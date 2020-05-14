/// A WinRT type that can be identified by a name
pub trait RuntimeName {
    // TODO: do the same here to allow generics with a name() function
    const NAME: &'static str;
}
