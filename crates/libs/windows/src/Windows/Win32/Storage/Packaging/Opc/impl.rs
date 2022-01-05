pub trait IOpcCertificateEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
pub trait IOpcCertificateSetImpl: Sized {
    fn Add();
    fn Remove();
    fn GetEnumerator();
}
pub trait IOpcDigitalSignatureImpl: Sized {
    fn GetNamespaces();
    fn GetSignatureId();
    fn GetSignaturePartName();
    fn GetSignatureMethod();
    fn GetCanonicalizationMethod();
    fn GetSignatureValue();
    fn GetSignaturePartReferenceEnumerator();
    fn GetSignatureRelationshipReferenceEnumerator();
    fn GetSigningTime();
    fn GetTimeFormat();
    fn GetPackageObjectReference();
    fn GetCertificateEnumerator();
    fn GetCustomReferenceEnumerator();
    fn GetCustomObjectEnumerator();
    fn GetSignatureXml();
}
pub trait IOpcDigitalSignatureEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
pub trait IOpcDigitalSignatureManagerImpl: Sized {
    fn GetSignatureOriginPartName();
    fn SetSignatureOriginPartName();
    fn GetSignatureEnumerator();
    fn RemoveSignature();
    fn CreateSigningOptions();
    fn Validate();
    fn Sign();
    fn ReplaceSignatureXml();
}
pub trait IOpcFactoryImpl: Sized {
    fn CreatePackageRootUri();
    fn CreatePartUri();
    fn CreateStreamOnFile();
    fn CreatePackage();
    fn ReadPackageFromStream();
    fn WritePackageToStream();
    fn CreateDigitalSignatureManager();
}
pub trait IOpcPackageImpl: Sized {
    fn GetPartSet();
    fn GetRelationshipSet();
}
pub trait IOpcPartImpl: Sized {
    fn GetRelationshipSet();
    fn GetContentStream();
    fn GetName();
    fn GetContentType();
    fn GetCompressionOptions();
}
pub trait IOpcPartEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
pub trait IOpcPartSetImpl: Sized {
    fn GetPart();
    fn CreatePart();
    fn DeletePart();
    fn PartExists();
    fn GetEnumerator();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcPartUriImpl: Sized + IOpcUriImpl + IUriImpl {
    fn ComparePartUri();
    fn GetSourceUri();
    fn IsRelationshipsPartUri();
}
pub trait IOpcRelationshipImpl: Sized {
    fn GetId();
    fn GetRelationshipType();
    fn GetSourceUri();
    fn GetTargetUri();
    fn GetTargetMode();
}
pub trait IOpcRelationshipEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
pub trait IOpcRelationshipSelectorImpl: Sized {
    fn GetSelectorType();
    fn GetSelectionCriterion();
}
pub trait IOpcRelationshipSelectorEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
pub trait IOpcRelationshipSelectorSetImpl: Sized {
    fn Create();
    fn Delete();
    fn GetEnumerator();
}
pub trait IOpcRelationshipSetImpl: Sized {
    fn GetRelationship();
    fn CreateRelationship();
    fn DeleteRelationship();
    fn RelationshipExists();
    fn GetEnumerator();
    fn GetEnumeratorForType();
    fn GetRelationshipsContentStream();
}
pub trait IOpcSignatureCustomObjectImpl: Sized {
    fn GetXml();
}
pub trait IOpcSignatureCustomObjectEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
pub trait IOpcSignatureCustomObjectSetImpl: Sized {
    fn Create();
    fn Delete();
    fn GetEnumerator();
}
pub trait IOpcSignaturePartReferenceImpl: Sized {
    fn GetPartName();
    fn GetContentType();
    fn GetDigestMethod();
    fn GetDigestValue();
    fn GetTransformMethod();
}
pub trait IOpcSignaturePartReferenceEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
pub trait IOpcSignaturePartReferenceSetImpl: Sized {
    fn Create();
    fn Delete();
    fn GetEnumerator();
}
pub trait IOpcSignatureReferenceImpl: Sized {
    fn GetId();
    fn GetUri();
    fn GetType();
    fn GetTransformMethod();
    fn GetDigestMethod();
    fn GetDigestValue();
}
pub trait IOpcSignatureReferenceEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
pub trait IOpcSignatureReferenceSetImpl: Sized {
    fn Create();
    fn Delete();
    fn GetEnumerator();
}
pub trait IOpcSignatureRelationshipReferenceImpl: Sized {
    fn GetSourceUri();
    fn GetDigestMethod();
    fn GetDigestValue();
    fn GetTransformMethod();
    fn GetRelationshipSigningOption();
    fn GetRelationshipSelectorEnumerator();
}
pub trait IOpcSignatureRelationshipReferenceEnumeratorImpl: Sized {
    fn MoveNext();
    fn MovePrevious();
    fn GetCurrent();
    fn Clone();
}
pub trait IOpcSignatureRelationshipReferenceSetImpl: Sized {
    fn Create();
    fn CreateRelationshipSelectorSet();
    fn Delete();
    fn GetEnumerator();
}
pub trait IOpcSigningOptionsImpl: Sized {
    fn GetSignatureId();
    fn SetSignatureId();
    fn GetSignatureMethod();
    fn SetSignatureMethod();
    fn GetDefaultDigestMethod();
    fn SetDefaultDigestMethod();
    fn GetCertificateEmbeddingOption();
    fn SetCertificateEmbeddingOption();
    fn GetTimeFormat();
    fn SetTimeFormat();
    fn GetSignaturePartReferenceSet();
    fn GetSignatureRelationshipReferenceSet();
    fn GetCustomObjectSet();
    fn GetCustomReferenceSet();
    fn GetCertificateSet();
    fn GetSignaturePartName();
    fn SetSignaturePartName();
}
#[cfg(feature = "Win32_System_Com")]
pub trait IOpcUriImpl: Sized + IUriImpl {
    fn GetRelationshipsPartUri();
    fn GetRelativeUri();
    fn CombinePartUri();
}
