use super::*;

pub fn encode_interface(encoder: &mut Encoder, _item: &syntax::Interface) -> Result<(), Error> {
    encoder.output.TypeDef(
        encoder.namespace,
        encoder.name,
        metadata::writer::TypeDefOrRef::default(),
        metadata::TypeAttributes::Public
            | metadata::TypeAttributes::Abstract
            | metadata::TypeAttributes::Interface
            | metadata::TypeAttributes::WindowsRuntime,
    );

    Ok(())
}
