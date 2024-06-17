use super::*;

fn include_ext(relative_path: &str) -> TokenStream {
    quote! {
        core::include!(
            core::concat!(
                core::env!("CARGO_MANIFEST_DIR"),
                "/src/includes/",
                #relative_path
            )
        );
    }
}

fn include_exts(relative_paths: &[&str]) -> TokenStream {
    relative_paths.iter().map(|&p| include_ext(p)).collect()
}

pub fn gen_mod(writer: &Writer, namespace: &str) -> TokenStream {
    if writer.sys {
        return quote!();
    }

    match namespace {
        "Windows.Foundation.Numerics" => include_exts(&[
            "Foundation/Numerics/Matrix3x2.rs",
            "Foundation/Numerics/Matrix4x4.rs",
            "Foundation/Numerics/Vector2.rs",
            "Foundation/Numerics/Vector3.rs",
            "Foundation/Numerics/Vector4.rs",
        ]),
        "Windows.Win32.Networking.WinSock" => include_exts(&[
            "Win32/Networking/WinSock/IN_ADDR.rs",
            "Win32/Networking/WinSock/IN6_ADDR.rs",
            "Win32/Networking/WinSock/SOCKADDR_IN.rs",
            "Win32/Networking/WinSock/SOCKADDR_IN6.rs",
            "Win32/Networking/WinSock/SOCKADDR_INET.rs",
        ]),
        "Windows.Win32.System.Rpc" => include_ext("Win32/System/Rpc/RPC_STATUS.rs"),
        "Windows.Win32.System.Com" => include_ext("Win32/System/Com/IDispatch.rs"),
        _ => quote!(),
    }
}

pub fn gen_impl(namespace: &str) -> TokenStream {
    match namespace {
        "Windows.Foundation.Collections" => include_ext("Foundation/Collections.rs"),
        _ => quote!(),
    }
}
