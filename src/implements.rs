pub trait Implements {
    type Type;

    fn make(self) -> Self::Type;
}
