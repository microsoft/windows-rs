use windows_registry::*;

#[test]
fn hash() {
    let mut set = std::collections::HashSet::<Type>::new();
    set.insert(Type::U32);
}
