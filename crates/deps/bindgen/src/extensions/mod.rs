use super::*;
mod matrix3x2;
mod matrix4x4;
mod timespan;
mod vector2;
mod vector3;
mod vector4;

pub fn gen(def: &TypeDef) -> TokenStream {
    match def.type_name() {
        TypeName::TimeSpan => timespan::gen(),
        TypeName::Vector2 => vector2::gen(),
        TypeName::Vector3 => vector3::gen(),
        TypeName::Vector4 => vector4::gen(),
        TypeName::Matrix3x2 => matrix3x2::gen(),
        TypeName::Matrix4x4 => matrix4x4::gen(),
        _ => quote! {},
    }
}
