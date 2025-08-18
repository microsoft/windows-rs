#![cfg(target_env = "msvc")]

use std::collections::BTreeMap;
use test_collection_interop::*;
use windows_collections::*;

#[implement(ITest)]
#[derive(Default)]
struct Test;

impl ITest_Impl for Test_Impl {
    fn TestIterable(&self, collection: Ref<IIterable<i32>>, values: &[i32]) -> Result<()> {
        let collection: Vec<i32> = collection.ok()?.into_iter().collect();
        assert_eq!(collection, values);
        Ok(())
    }

    fn GetIterable(&self, values: &[i32]) -> Result<IIterable<i32>> {
        Ok(Vec::from(values).into())
    }

    fn GetMapView(&self, values: &[i32]) -> Result<IMapView<i32, IVectorView<i32>>> {
        let mut map = BTreeMap::new();

        for value in values {
            map.insert(*value, Some(IVectorView::from(Vec::from(values))));
        }

        Ok(IMapView::from(map))
    }
}

fn test_impl(test: &ITest) {
    let rust: ITest = Test.into();
    test.TestIterable(&rust.GetIterable(&[1, 2, 3]).unwrap(), &[1, 2, 3])
        .unwrap();

    let values = test.GetIterable(&[4, 5, 6]).unwrap();
    let values: Vec<i32> = values.into_iter().collect();
    assert_eq!(values, [4, 5, 6]);

    let view = test.GetMapView(&[1, 2, 3]).unwrap();
    assert_eq!(view.Size().unwrap(), 3);

    let mut keys = 0;
    let mut values = 0;

    for pair in view {
        keys += pair.Key().unwrap();

        for value in pair.Value().unwrap() {
            values += value;
        }
    }

    assert_eq!(keys, 6);
    assert_eq!(values, 18);
}

fn test_empty(test: &ITest) {
    let rust: ITest = Test.into();
    test.TestIterable(&rust.GetIterable(&[]).unwrap(), &[])
        .unwrap();

    let values = test.GetIterable(&[]).unwrap();
    let values: Vec<i32> = values.into_iter().collect();
    assert!(values.is_empty());

    let view = test.GetMapView(&[]).unwrap();
    assert_eq!(view.Size().unwrap(), 0);

    let mut keys = 0;

    for pair in view {
        keys += pair.Key().unwrap();
    }

    assert_eq!(keys, 0);
}

#[test]
fn test_rust() {
    let test: ITest = Test.into();
    test_impl(&test);
    test_empty(&test);
}

#[test]
fn test_cpp() {
    let test: ITest = make_cpp().unwrap();
    test_impl(&test);
    test_empty(&test);
}
