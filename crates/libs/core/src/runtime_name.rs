use super::*;

#[doc(hidden)]
pub trait RuntimeName {
    const NAME: &'static str = "";
    const RUNTIME_CLASS_NAME: imp::ConstBuffer =
        imp::ConstBuffer::from_slice(Self::NAME.as_bytes());
}
