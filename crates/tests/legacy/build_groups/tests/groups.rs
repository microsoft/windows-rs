use windows::Foundation::{
    Collections::{IKeyValuePair, IMap, IVector},
    IStringable,
};

#[test]
fn test() -> windows::core::Result<()> {
    let _ = Option::<IStringable>::None;
    let _ = Option::<IVector<i32>>::None;
    let _ = Option::<IMap<i32, i32>>::None;
    let _ = Option::<IKeyValuePair<i32, i32>>::None;

    Ok(())
}
