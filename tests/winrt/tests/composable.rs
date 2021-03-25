use test_winrt::TestComponent::*;
use windows::*;

#[test]
fn composable() -> Result<()> {
    let c = Composable::new()?;
    assert_eq!(c.Value()?, 0);

    let c = Composable::CreateWithValue(123)?;
    assert_eq!(c.Value()?, 123);

    let d = &Derived::new()?;
    assert_eq!(d.Value()?, 0);
    d.SetValue(456)?;
    assert_eq!(d.Value()?, 456);
    assert_eq!(d.One()?, 1);
    assert_eq!(d.Two()?, 2);
    assert_eq!(d.Three()?, 3);
    assert_eq!(d.Four()?, 4);
    assert_eq!(456, Composable::ExpectComposable(d)?);
    assert_eq!(1, Composable::ExpectRequiredOne(d)?);
    assert_eq!(2, Composable::ExpectRequiredTwo(d)?);
    assert_eq!(3, Composable::ExpectRequiredThree(d)?);
    assert_eq!(4, Composable::ExpectRequiredFour(d)?);

    // Base class of Derived
    let t: Composable = d.into();
    assert_eq!(t.Value()?, 456);
    assert_eq!(d.One()?, 1);
    assert_eq!(d.Two()?, 2);
    assert_eq!(d.Three()?, 3);
    assert_eq!(d.Four()?, 4);
    assert_eq!(456, Composable::ExpectComposable(&t)?);
    assert_eq!(1, Composable::ExpectRequiredOne(&t)?);
    assert_eq!(2, Composable::ExpectRequiredTwo(&t)?);
    assert_eq!(3, Composable::ExpectRequiredThree(&t)?);
    assert_eq!(4, Composable::ExpectRequiredFour(&t)?);

    // Default interface of Composable class
    let _t: IComposable = d.cast()?;

    let t: IRequiredFour = d.into();
    assert_eq!(t.One()?, 1);
    assert_eq!(t.Two()?, 2);
    assert_eq!(t.Three()?, 3);
    assert_eq!(t.Four()?, 4);
    assert_eq!(1, Composable::ExpectRequiredOne(&t)?);
    assert_eq!(2, Composable::ExpectRequiredTwo(&t)?);
    assert_eq!(3, Composable::ExpectRequiredThree(&t)?);
    assert_eq!(4, Composable::ExpectRequiredFour(&t)?);

    let t: IRequiredThree = d.into();
    assert_eq!(t.One()?, 1);
    assert_eq!(t.Two()?, 2);
    assert_eq!(t.Three()?, 3);
    assert_eq!(1, Composable::ExpectRequiredOne(&t)?);
    assert_eq!(2, Composable::ExpectRequiredTwo(&t)?);
    assert_eq!(3, Composable::ExpectRequiredThree(&t)?);

    let t: IRequiredTwo = d.into();
    assert_eq!(t.One()?, 1);
    assert_eq!(t.Two()?, 2);
    assert_eq!(1, Composable::ExpectRequiredOne(&t)?);
    assert_eq!(2, Composable::ExpectRequiredTwo(&t)?);

    let t: IRequiredOne = d.into();
    assert_eq!(t.One()?, 1);
    assert_eq!(1, Composable::ExpectRequiredOne(&t)?);

    Ok(())
}
