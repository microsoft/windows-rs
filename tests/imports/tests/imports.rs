use winmd_reader::*;

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

    let imports = get_imports("Windows.Win32.UI.ColorSystem", "WhitePoint");
    assert_eq!(imports.len(), 2);
    assert!(imports["WhitePoint"] == TypeInclude::Full);
    assert!(imports["XYYPoint"] == TypeInclude::Minimal);

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

    let imports = get_imports("Windows.Foundation", "IUriRuntimeClassFactory");
    assert_eq!(imports.len(), 3);
    assert!(imports["IUriRuntimeClassFactory"] == TypeInclude::Full);
    assert!(imports["Uri"] == TypeInclude::Minimal);
    assert!(imports["IUriRuntimeClass"] == TypeInclude::Minimal);

    let imports = get_imports("Windows.Foundation", "IAsyncAction");
    assert_eq!(imports.len(), 4);
    assert!(imports["IAsyncAction"] == TypeInclude::Full);
    assert!(imports["IAsyncInfo"] == TypeInclude::Full);
    assert!(imports["AsyncActionCompletedHandler"] == TypeInclude::Minimal);
    assert!(imports["AsyncStatus"] == TypeInclude::Minimal);

    let imports = get_imports("Windows.Foundation", "AsyncActionCompletedHandler");
    assert_eq!(imports.len(), 3);
    assert!(imports["AsyncActionCompletedHandler"] == TypeInclude::Full);
    assert!(imports["IAsyncAction"] == TypeInclude::Minimal);
    assert!(imports["AsyncStatus"] == TypeInclude::Minimal);

    let imports = get_imports("Windows.Foundation.Collections", "StringMap");
    assert_eq!(imports.len(), 10);
    assert!(imports["StringMap"] == TypeInclude::Full);
    assert!(imports["IMap"] == TypeInclude::Full);
    assert!(imports["IIterable"] == TypeInclude::Full);
    assert!(imports["IIterator"] == TypeInclude::Full);
    assert!(imports["IKeyValuePair"] == TypeInclude::Full);
    assert!(imports["IObservableMap"] == TypeInclude::Full);
    assert!(imports["EventRegistrationToken"] == TypeInclude::Minimal);
    assert!(imports["IMapChangedEventArgs"] == TypeInclude::Minimal);
    assert!(imports["IMapView"] == TypeInclude::Minimal);
    assert!(imports["MapChangedEventHandler"] == TypeInclude::Minimal);

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

    // TODO: add a composable class test (like Composition/Xaml)
}
