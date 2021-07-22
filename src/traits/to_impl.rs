use super::*;

pub unsafe trait ToImpl<T: Interface> {
    fn to_impl(from: &T) -> &mut Self;
}
