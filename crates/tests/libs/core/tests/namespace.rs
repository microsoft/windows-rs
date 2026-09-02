use other::*;
use windows_core::*;

mod other {
    pub struct Type;
}

#[test]
fn type_name_is_not_ambiguous() {
    let _ = Type;
    let _: Option<Ref<'static, IUnknown>> = None;
}
