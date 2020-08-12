use tests::test_component::*;
use winrt::*;

#[test]
fn composable() -> Result<()> {
    let c = Composable::new()?;
    assert_eq!(c.value()?, 0);

    let c = Composable::create_with_value(123)?;
    assert_eq!(c.value()?, 123);

    let d = &Derived::new()?;
    assert_eq!(d.value()?, 0);
    d.set_value(456)?;
    assert_eq!(d.value()?, 456);
    assert_eq!(d.one()?, 1);
    assert_eq!(d.two()?, 2);
    assert_eq!(d.three()?, 3);
    assert_eq!(d.four()?, 4);
    assert_eq!(456, Composable::expect_composable(d)?);
    assert_eq!(1, Composable::expect_required_one(d)?);
    assert_eq!(2, Composable::expect_required_two(d)?);
    assert_eq!(3, Composable::expect_required_three(d)?);
    assert_eq!(4, Composable::expect_required_four(d)?);

    // Base class of Derived
    let t: Composable = d.into();
    assert_eq!(t.value()?, 456);
    assert_eq!(t.one()?, 1);
    assert_eq!(t.two()?, 2);
    assert_eq!(t.three()?, 3);
    assert_eq!(t.four()?, 4);
    assert_eq!(456, Composable::expect_composable(&t)?);
    assert_eq!(1, Composable::expect_required_one(&t)?);
    assert_eq!(2, Composable::expect_required_two(&t)?);
    assert_eq!(3, Composable::expect_required_three(&t)?);
    assert_eq!(4, Composable::expect_required_four(&t)?);

    // Default interface of Composable class
    let t: IComposable = d.into();
    assert_eq!(t.value()?, 456);

    let t: IRequiredFour = d.into();
    assert_eq!(t.one()?, 1);
    assert_eq!(t.two()?, 2);
    assert_eq!(t.three()?, 3);
    assert_eq!(t.four()?, 4);
    assert_eq!(1, Composable::expect_required_one(&t)?);
    assert_eq!(2, Composable::expect_required_two(&t)?);
    assert_eq!(3, Composable::expect_required_three(&t)?);
    assert_eq!(4, Composable::expect_required_four(&t)?);

    let t: IRequiredThree = d.into();
    assert_eq!(t.one()?, 1);
    assert_eq!(t.two()?, 2);
    assert_eq!(t.three()?, 3);
    assert_eq!(1, Composable::expect_required_one(&t)?);
    assert_eq!(2, Composable::expect_required_two(&t)?);
    assert_eq!(3, Composable::expect_required_three(&t)?);

    let t: IRequiredTwo = d.into();
    assert_eq!(t.one()?, 1);
    assert_eq!(t.two()?, 2);
    assert_eq!(1, Composable::expect_required_one(&t)?);
    assert_eq!(2, Composable::expect_required_two(&t)?);

    let t: IRequiredOne = d.into();
    assert_eq!(t.one()?, 1);
    assert_eq!(1, Composable::expect_required_one(&t)?);

    Ok(())
}
