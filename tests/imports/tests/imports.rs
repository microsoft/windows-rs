use windows_gen::*;

fn get_imports(namespace: &'static str, name: &'static str) -> BTreeMap<&'static str, TypeInclude> {
    let reader = TypeReader::get_mut();
    reader.import_type(namespace, name);
    let mut map = BTreeMap::<&'static str, TypeInclude>::new();

    fn walk(tree: &mut TypeTree2, map: &mut BTreeMap<&'static str, TypeInclude>) {
        if tree.include {
            for (name, entry) in &mut tree.types {
                if entry.include != TypeInclude::None {
                    map.insert(name, entry.include);
                    entry.include = TypeInclude::None;
                }
            }

            tree.namespaces
                .values_mut()
                .for_each(|mut tree| walk(&mut tree, map));
        }
    }

    walk(&mut reader.types, &mut map);
    map
}

#[test]
fn test_dependencies() {
    let imports = get_imports("Windows.Win32.System.Com", "CreateUri");
    assert_eq!(imports.len(), 4);
    assert_eq!(imports["CreateUri"], TypeInclude::Full);
    assert_eq!(imports["PWSTR"], TypeInclude::Minimal);
    assert_eq!(imports["URI_CREATE_FLAGS"], TypeInclude::Minimal);
    assert_eq!(imports["IUri"], TypeInclude::Minimal);

    let imports = get_imports("Windows.Win32.Graphics.Direct2D", "ID2D1ImageSource");
    assert_eq!(imports.len(), 5);
    assert_eq!(imports["ID2D1ImageSource"], TypeInclude::Full);
    assert_eq!(imports["ID2D1Image"], TypeInclude::Full); // full because ID2D1ImageSource derives from it
    assert_eq!(imports["ID2D1Resource"], TypeInclude::Full); // full because ID2D1Image derives from it
    assert_eq!(imports["BOOL"], TypeInclude::Minimal); // from ID2D1ImageSource method
    assert_eq!(imports["ID2D1Factory"], TypeInclude::Minimal); // from ID2D1Resource method

    let imports = get_imports("Windows.Win32.Graphics.Direct3D12", "D3D12_INDIRECT_ARGUMENT_DESC");
    assert_eq!(imports.len(), 2);
    assert_eq!(imports["D3D12_INDIRECT_ARGUMENT_DESC"], TypeInclude::Full);
    assert_eq!(imports["D3D12_INDIRECT_ARGUMENT_TYPE"], TypeInclude::Minimal);

    let imports = get_imports("Windows.Win32.UI.ColorSystem", "WhitePoint");
    assert_eq!(imports.len(), 2);
    assert_eq!(imports["WhitePoint"], TypeInclude::Full);
    assert_eq!(imports["XYYPoint"], TypeInclude::Minimal);
}
