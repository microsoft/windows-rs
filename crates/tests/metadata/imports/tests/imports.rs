use reader::*;

fn get_imports(namespace: &'static str, name: &'static str) -> BTreeMap<&'static str, TypeInclude> {
    let reader = TypeReader::get_mut();
    reader.import_type(namespace, name);
    let mut map = BTreeMap::<&'static str, TypeInclude>::new();

    fn walk(tree: &mut TypeTree, map: &mut BTreeMap<&'static str, TypeInclude>) {
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
    assert!(imports["CreateUri"] == TypeInclude::Full);
    assert!(imports["PWSTR"] == TypeInclude::Minimal);
    assert!(imports["URI_CREATE_FLAGS"] == TypeInclude::Minimal);
    assert!(imports["IUri"] == TypeInclude::Minimal);

    let imports = get_imports("Windows.Win32.Graphics.Direct2D", "ID2D1ImageSource");
    assert_eq!(imports.len(), 5);
    assert!(imports["ID2D1ImageSource"] == TypeInclude::Full);
    assert!(imports["ID2D1Image"] == TypeInclude::Full); // full because ID2D1ImageSource derives from it
    assert!(imports["ID2D1Resource"] == TypeInclude::Full); // full because ID2D1Image derives from it
    assert!(imports["BOOL"] == TypeInclude::Minimal); // from ID2D1ImageSource method
    assert!(imports["ID2D1Factory"] == TypeInclude::Minimal); // from ID2D1Resource method

    let imports = get_imports(
        "Windows.Win32.Graphics.Direct3D12",
        "D3D12_INDIRECT_ARGUMENT_DESC",
    );
    assert_eq!(imports.len(), 2);
    assert!(imports["D3D12_INDIRECT_ARGUMENT_DESC"] == TypeInclude::Full);
    assert!(imports["D3D12_INDIRECT_ARGUMENT_TYPE"] == TypeInclude::Minimal);

    let imports = get_imports("Windows.Win32.Graphics.Dxgi", "DXGI_FRAME_STATISTICS_MEDIA");
    assert_eq!(imports.len(), 2);
    assert!(imports["DXGI_FRAME_STATISTICS_MEDIA"] == TypeInclude::Full);
    assert!(imports["DXGI_FRAME_PRESENTATION_MODE"] == TypeInclude::Minimal);

    let imports = get_imports(
        "Windows.Win32.Graphics.Dxgi",
        "DXGI_FRAME_PRESENTATION_MODE",
    );
    assert_eq!(imports.len(), 1);
    assert!(imports["DXGI_FRAME_PRESENTATION_MODE"] == TypeInclude::Full);

    let imports = get_imports("Windows.Foundation", "IAsyncAction");
    assert_eq!(imports.len(), 4);
    assert!(imports["IAsyncAction"] == TypeInclude::Full);
    assert!(imports["IAsyncInfo"] == TypeInclude::Full);
    assert!(imports["AsyncActionCompletedHandler"] == TypeInclude::Full);
    assert!(imports["AsyncStatus"] == TypeInclude::Full);

    let imports = get_imports("Windows.Foundation", "AsyncActionCompletedHandler");
    assert_eq!(imports.len(), 4);
    assert!(imports["AsyncActionCompletedHandler"] == TypeInclude::Full);
    assert!(imports["IAsyncAction"] == TypeInclude::Full);
    assert!(imports["AsyncStatus"] == TypeInclude::Full);
    assert!(imports["IAsyncInfo"] == TypeInclude::Full);

    let imports = get_imports("Windows.Foundation.Collections", "StringMap");
    assert_eq!(imports.len(), 11);
    assert!(imports["StringMap"] == TypeInclude::Full);
    assert!(imports["IMap"] == TypeInclude::Full);
    assert!(imports["IIterable"] == TypeInclude::Full);
    assert!(imports["IIterator"] == TypeInclude::Full);
    assert!(imports["IKeyValuePair"] == TypeInclude::Full);
    assert!(imports["IObservableMap"] == TypeInclude::Full);
    assert!(imports["EventRegistrationToken"] == TypeInclude::Full);
    assert!(imports["IMapChangedEventArgs"] == TypeInclude::Full);
    assert!(imports["IMapView"] == TypeInclude::Full);
    assert!(imports["MapChangedEventHandler"] == TypeInclude::Full);
    assert!(imports["CollectionChange"] == TypeInclude::Full);

    let imports = get_imports(
        "Windows.Win32.Graphics.Direct3D11",
        "D3D11_DEPTH_STENCIL_VIEW_DESC",
    );
    assert_eq!(imports.len(), 9);
    assert!(imports["D3D11_DEPTH_STENCIL_VIEW_DESC"] == TypeInclude::Full);
    assert!(imports["DXGI_FORMAT"] == TypeInclude::Minimal);
    assert!(imports["D3D11_DSV_DIMENSION"] == TypeInclude::Minimal);
    assert!(imports["D3D11_TEX1D_DSV"] == TypeInclude::Minimal);
    assert!(imports["D3D11_TEX1D_ARRAY_DSV"] == TypeInclude::Minimal);
    assert!(imports["D3D11_TEX2D_DSV"] == TypeInclude::Minimal);
    assert!(imports["D3D11_TEX2D_ARRAY_DSV"] == TypeInclude::Minimal);
    assert!(imports["D3D11_TEX2DMS_DSV"] == TypeInclude::Minimal);
    assert!(imports["D3D11_TEX2DMS_ARRAY_DSV"] == TypeInclude::Minimal);

    let imports = get_imports("Component.Interfaces", "Test");
    assert_eq!(imports.len(), 3);
    assert!(imports["Test"] == TypeInclude::Full);
    assert!(imports["IRequires"] == TypeInclude::Minimal);

    let imports = get_imports("Component.Dependencies", "ISimple");
    assert_eq!(imports.len(), 1);
    assert!(imports["ISimple"] == TypeInclude::Full);

    let imports = get_imports("Component.Dependencies", "IRequiredDependencies");
    assert_eq!(imports.len(), 5);
    assert!(imports["IRequiredDependencies"] == TypeInclude::Full);
    assert!(imports["ISimple"] == TypeInclude::Full);
    assert!(imports["IStringable"] == TypeInclude::Full);
    assert!(imports["IIterable"] == TypeInclude::Full);
    assert!(imports["IIterator"] == TypeInclude::Full);

    let imports = get_imports("Component.Dependencies", "IMethodDependencies");
    assert_eq!(imports.len(), 5);
    assert!(imports["IMethodDependencies"] == TypeInclude::Full);
    assert!(imports["ISimple"] == TypeInclude::Minimal);
    assert!(imports["IStringable"] == TypeInclude::Full);
    assert!(imports["IIterable"] == TypeInclude::Full);
    assert!(imports["IIterator"] == TypeInclude::Full);
}
