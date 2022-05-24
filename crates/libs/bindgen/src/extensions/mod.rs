use super::*;
mod in6_addr;
mod in_addr;
mod matrix3x2;
mod matrix4x4;
mod sockaddr_in;
mod sockaddr_in6;
mod sockaddr_inet;
mod timespan;
mod vector2;
mod vector3;
mod vector4;
mod win32_error;

pub fn gen(type_name: TypeName) -> TokenStream {
    match type_name {
        TypeName::IN_ADDR => in_addr::gen(),
        TypeName::IN6_ADDR => in6_addr::gen(),
        TypeName::Matrix3x2 => matrix3x2::gen(),
        TypeName::Matrix4x4 => matrix4x4::gen(),
        TypeName::SOCKADDR_IN => sockaddr_in::gen(),
        TypeName::SOCKADDR_IN6 => sockaddr_in6::gen(),
        TypeName::SOCKADDR_INET => sockaddr_inet::gen(),
        TypeName::TimeSpan => timespan::gen(),
        TypeName::Vector2 => vector2::gen(),
        TypeName::Vector3 => vector3::gen(),
        TypeName::Vector4 => vector4::gen(),
        TypeName::WIN32_ERROR => win32_error::gen(),
        _ => quote! {},
    }
}
