use super::*;

pub fn gen_mod(writer: &Writer, namespace: &str) -> TokenStream {
    if namespace == "Windows.Win32.UI.WindowsAndMessaging" {
        return include_str!("mod/Win32/UI/WindowsAndMessaging/WindowLong.rs").into();
    }

    if writer.sys {
        return "".into();
    }

    match namespace {
        "Windows.Foundation.Numerics" => concat!(include_str!("mod/Foundation/Numerics/Matrix3x2.rs"), include_str!("mod/Foundation/Numerics/Matrix4x4.rs"), include_str!("mod/Foundation/Numerics/Vector2.rs"), include_str!("mod/Foundation/Numerics/Vector3.rs"), include_str!("mod/Foundation/Numerics/Vector4.rs"),),
        "Windows.Foundation" => concat!(include_str!("mod/Foundation/TimeSpan.rs"),),
        "Windows.Win32.Foundation" => concat!(include_str!("mod/Win32/Foundation/BOOL.rs"), include_str!("mod/Win32/Foundation/BOOLEAN.rs"), include_str!("mod/Win32/Foundation/NTSTATUS.rs"), include_str!("mod/Win32/Foundation/VARIANT_BOOL.rs"), include_str!("mod/Win32/Foundation/WIN32_ERROR.rs"),),
        "Windows.Win32.Networking.WinSock" => concat!(include_str!("mod/Win32/Networking/WinSock/IN_ADDR.rs"), include_str!("mod/Win32/Networking/WinSock/IN6_ADDR.rs"), include_str!("mod/Win32/Networking/WinSock/SOCKADDR_IN.rs"), include_str!("mod/Win32/Networking/WinSock/SOCKADDR_IN6.rs"), include_str!("mod/Win32/Networking/WinSock/SOCKADDR_INET.rs"),),
        "Windows.Win32.UI.WindowsAndMessaging" => {
            include_str!("mod/Win32/UI/WindowsAndMessaging/WindowLong.rs")
        }
        _ => "",
    }
    .into()
}

pub fn gen_impl(namespace: &str) -> TokenStream {
    match namespace {
        "Windows.Foundation.Collections" => concat!(include_str!("impl/Foundation/Collections/Iterable.rs"), include_str!("impl/Foundation/Collections/MapView.rs"), include_str!("impl/Foundation/Collections/VectorView.rs"),),
        _ => "",
    }
    .into()
}
