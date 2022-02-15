use windows::core::*;

#[interface("8CEEB155-2849-4ce5-9448-91FF70E1E4D9")]
unsafe trait IUIAnimationVariable: IUnknown {
    fn GetValue(&self, value: *mut f64) -> HRESULT;
}

#[test]
fn com_works() -> Result<()> {
    Ok(())
}
