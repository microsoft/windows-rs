use windows_gen::*;

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

    let imports = get_imports(
        "Windows.Win32.Graphics.Direct3D12",
        "D3D12_INDIRECT_ARGUMENT_DESC",
    );
    assert_eq!(imports.len(), 2);
    assert_eq!(imports["D3D12_INDIRECT_ARGUMENT_DESC"], TypeInclude::Full);
    assert_eq!(
        imports["D3D12_INDIRECT_ARGUMENT_TYPE"],
        TypeInclude::Minimal
    );

    let imports = get_imports("Windows.Win32.UI.ColorSystem", "WhitePoint");
    assert_eq!(imports.len(), 2);
    assert_eq!(imports["WhitePoint"], TypeInclude::Full);
    assert_eq!(imports["XYYPoint"], TypeInclude::Minimal);

    let imports = get_imports("Windows.Win32.Graphics.Dxgi", "DXGI_FRAME_STATISTICS_MEDIA");
    assert_eq!(imports.len(), 2);
    assert_eq!(imports["DXGI_FRAME_STATISTICS_MEDIA"], TypeInclude::Full);
    assert_eq!(
        imports["DXGI_FRAME_PRESENTATION_MODE"],
        TypeInclude::Minimal
    );

    let imports = get_imports(
        "Windows.Win32.Graphics.Dxgi",
        "DXGI_FRAME_PRESENTATION_MODE",
    );
    assert_eq!(imports.len(), 1);
    assert_eq!(imports["DXGI_FRAME_PRESENTATION_MODE"], TypeInclude::Full);

    let imports = get_imports("Windows.Foundation", "IUriRuntimeClassFactory");
    assert_eq!(imports.len(), 2);
    assert_eq!(imports["IUriRuntimeClassFactory"], TypeInclude::Full);
    assert_eq!(imports["Uri"], TypeInclude::Minimal);

    let imports = get_imports("Windows.Foundation", "IAsyncAction");
    assert_eq!(imports.len(), 4);
    assert_eq!(imports["IAsyncAction"], TypeInclude::Full);
    assert_eq!(imports["IAsyncInfo"], TypeInclude::Full);
    assert_eq!(imports["AsyncActionCompletedHandler"], TypeInclude::Minimal);
    assert_eq!(imports["AsyncStatus"], TypeInclude::Minimal);

    let imports = get_imports("Windows.Foundation", "AsyncActionCompletedHandler");
    assert_eq!(imports.len(), 3);
    assert_eq!(imports["AsyncActionCompletedHandler"], TypeInclude::Full);
    assert_eq!(imports["IAsyncAction"], TypeInclude::Minimal);
    assert_eq!(imports["AsyncStatus"], TypeInclude::Minimal);

    let imports = get_imports("Windows.Foundation.Collections", "StringMap");
    assert_eq!(imports.len(), 10);
    assert_eq!(imports["StringMap"], TypeInclude::Full);
    assert_eq!(imports["IMap"], TypeInclude::Full);
    assert_eq!(imports["IIterable"], TypeInclude::Full);
    assert_eq!(imports["IIterator"], TypeInclude::Full);
    assert_eq!(imports["IKeyValuePair"], TypeInclude::Full);
    assert_eq!(imports["IObservableMap"], TypeInclude::Full);
    assert_eq!(imports["EventRegistrationToken"], TypeInclude::Minimal);
    assert_eq!(imports["IMapChangedEventArgs"], TypeInclude::Minimal);
    assert_eq!(imports["IMapView"], TypeInclude::Minimal);
    assert_eq!(imports["MapChangedEventHandler"], TypeInclude::Minimal);

    let imports = get_imports(
        "Windows.Win32.Graphics.Direct3D11",
        "D3D11_DEPTH_STENCIL_VIEW_DESC",
    );
    assert_eq!(imports.len(), 9);
    assert_eq!(imports["D3D11_DEPTH_STENCIL_VIEW_DESC"], TypeInclude::Full);
    assert_eq!(imports["DXGI_FORMAT"], TypeInclude::Minimal);
    assert_eq!(imports["D3D11_DSV_DIMENSION"], TypeInclude::Minimal);
    assert_eq!(imports["D3D11_TEX1D_DSV"], TypeInclude::Minimal);
    assert_eq!(imports["D3D11_TEX1D_ARRAY_DSV"], TypeInclude::Minimal);
    assert_eq!(imports["D3D11_TEX2D_DSV"], TypeInclude::Minimal);
    assert_eq!(imports["D3D11_TEX2D_ARRAY_DSV"], TypeInclude::Minimal);
    assert_eq!(imports["D3D11_TEX2DMS_DSV"], TypeInclude::Minimal);
    assert_eq!(imports["D3D11_TEX2DMS_ARRAY_DSV"], TypeInclude::Minimal);

    // TODO: add a composable class test (like Composition/Xaml)
}
