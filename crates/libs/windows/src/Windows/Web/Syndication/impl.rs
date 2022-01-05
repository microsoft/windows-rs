#[cfg(feature = "implement_exclusive")]
pub trait ISyndicationAttributeImpl: Sized {
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Namespace(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNamespace(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Value(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISyndicationAttributeFactoryImpl: Sized {
    fn CreateSyndicationAttribute(&self, attributename: &::windows::core::HSTRING, attributenamespace: &::windows::core::HSTRING, attributevalue: &::windows::core::HSTRING) -> ::windows::core::Result<SyndicationAttribute>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISyndicationCategoryImpl: Sized + ISyndicationNodeImpl {
    fn Label(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLabel(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Scheme(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetScheme(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Term(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTerm(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISyndicationCategoryFactoryImpl: Sized {
    fn CreateSyndicationCategory(&self, term: &::windows::core::HSTRING) -> ::windows::core::Result<SyndicationCategory>;
    fn CreateSyndicationCategoryEx(&self, term: &::windows::core::HSTRING, scheme: &::windows::core::HSTRING, label: &::windows::core::HSTRING) -> ::windows::core::Result<SyndicationCategory>;
}
pub trait ISyndicationClientImpl: Sized {
    fn ServerCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetServerCredential(&self, value: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn ProxyCredential(&self) -> ::windows::core::Result<super::super::Security::Credentials::PasswordCredential>;
    fn SetProxyCredential(&self, value: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<()>;
    fn MaxResponseBufferSize(&self) -> ::windows::core::Result<u32>;
    fn SetMaxResponseBufferSize(&self, value: u32) -> ::windows::core::Result<()>;
    fn Timeout(&self) -> ::windows::core::Result<u32>;
    fn SetTimeout(&self, value: u32) -> ::windows::core::Result<()>;
    fn BypassCacheOnRetrieve(&self) -> ::windows::core::Result<bool>;
    fn SetBypassCacheOnRetrieve(&self, value: bool) -> ::windows::core::Result<()>;
    fn SetRequestHeader(&self, name: &::windows::core::HSTRING, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn RetrieveFeedAsync(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<super::super::Foundation::IAsyncOperationWithProgress<SyndicationFeed, RetrievalProgress>>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISyndicationClientFactoryImpl: Sized {
    fn CreateSyndicationClient(&self, servercredential: &::core::option::Option<super::super::Security::Credentials::PasswordCredential>) -> ::windows::core::Result<SyndicationClient>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISyndicationContentImpl: Sized + ISyndicationNodeImpl + ISyndicationTextImpl {
    fn SourceUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetSourceUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISyndicationContentFactoryImpl: Sized {
    fn CreateSyndicationContent(&self, text: &::windows::core::HSTRING, r#type: SyndicationTextType) -> ::windows::core::Result<SyndicationContent>;
    fn CreateSyndicationContentWithSourceUri(&self, sourceuri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<SyndicationContent>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISyndicationErrorStaticsImpl: Sized {
    fn GetStatus(&self, hresult: i32) -> ::windows::core::Result<SyndicationErrorStatus>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISyndicationFeedImpl: Sized + ISyndicationNodeImpl {
    fn Authors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationPerson>>;
    fn Categories(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationCategory>>;
    fn Contributors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationPerson>>;
    fn Generator(&self) -> ::windows::core::Result<SyndicationGenerator>;
    fn SetGenerator(&self, value: &::core::option::Option<SyndicationGenerator>) -> ::windows::core::Result<()>;
    fn IconUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetIconUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Items(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationItem>>;
    fn LastUpdatedTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetLastUpdatedTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn Links(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationLink>>;
    fn ImageUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetImageUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Rights(&self) -> ::windows::core::Result<ISyndicationText>;
    fn SetRights(&self, value: &::core::option::Option<ISyndicationText>) -> ::windows::core::Result<()>;
    fn Subtitle(&self) -> ::windows::core::Result<ISyndicationText>;
    fn SetSubtitle(&self, value: &::core::option::Option<ISyndicationText>) -> ::windows::core::Result<()>;
    fn Title(&self) -> ::windows::core::Result<ISyndicationText>;
    fn SetTitle(&self, value: &::core::option::Option<ISyndicationText>) -> ::windows::core::Result<()>;
    fn FirstUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn LastUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn NextUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn PreviousUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SourceFormat(&self) -> ::windows::core::Result<SyndicationFormat>;
    fn Load(&self, feed: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LoadFromXml(&self, feeddocument: &::core::option::Option<super::super::Data::Xml::Dom::XmlDocument>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISyndicationFeedFactoryImpl: Sized {
    fn CreateSyndicationFeed(&self, title: &::windows::core::HSTRING, subtitle: &::windows::core::HSTRING, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<SyndicationFeed>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISyndicationGeneratorImpl: Sized {
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn Version(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetVersion(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISyndicationGeneratorFactoryImpl: Sized {
    fn CreateSyndicationGenerator(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<SyndicationGenerator>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISyndicationItemImpl: Sized + ISyndicationNodeImpl {
    fn Authors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationPerson>>;
    fn Categories(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationCategory>>;
    fn Contributors(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationPerson>>;
    fn Content(&self) -> ::windows::core::Result<SyndicationContent>;
    fn SetContent(&self, value: &::core::option::Option<SyndicationContent>) -> ::windows::core::Result<()>;
    fn Id(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetId(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LastUpdatedTime(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetLastUpdatedTime(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn Links(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationLink>>;
    fn PublishedDate(&self) -> ::windows::core::Result<super::super::Foundation::DateTime>;
    fn SetPublishedDate(&self, value: &super::super::Foundation::DateTime) -> ::windows::core::Result<()>;
    fn Rights(&self) -> ::windows::core::Result<ISyndicationText>;
    fn SetRights(&self, value: &::core::option::Option<ISyndicationText>) -> ::windows::core::Result<()>;
    fn Source(&self) -> ::windows::core::Result<SyndicationFeed>;
    fn SetSource(&self, value: &::core::option::Option<SyndicationFeed>) -> ::windows::core::Result<()>;
    fn Summary(&self) -> ::windows::core::Result<ISyndicationText>;
    fn SetSummary(&self, value: &::core::option::Option<ISyndicationText>) -> ::windows::core::Result<()>;
    fn Title(&self) -> ::windows::core::Result<ISyndicationText>;
    fn SetTitle(&self, value: &::core::option::Option<ISyndicationText>) -> ::windows::core::Result<()>;
    fn CommentsUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetCommentsUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn EditUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn EditMediaUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn ETag(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn ItemUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn Load(&self, item: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn LoadFromXml(&self, itemdocument: &::core::option::Option<super::super::Data::Xml::Dom::XmlDocument>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISyndicationItemFactoryImpl: Sized {
    fn CreateSyndicationItem(&self, title: &::windows::core::HSTRING, content: &::core::option::Option<SyndicationContent>, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<SyndicationItem>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISyndicationLinkImpl: Sized + ISyndicationNodeImpl {
    fn Length(&self) -> ::windows::core::Result<u32>;
    fn SetLength(&self, value: u32) -> ::windows::core::Result<()>;
    fn MediaType(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetMediaType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Relationship(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetRelationship(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Title(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetTitle(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn ResourceLanguage(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetResourceLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISyndicationLinkFactoryImpl: Sized {
    fn CreateSyndicationLink(&self, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<SyndicationLink>;
    fn CreateSyndicationLinkEx(&self, uri: &::core::option::Option<super::super::Foundation::Uri>, relationship: &::windows::core::HSTRING, title: &::windows::core::HSTRING, mediatype: &::windows::core::HSTRING, length: u32) -> ::windows::core::Result<SyndicationLink>;
}
pub trait ISyndicationNodeImpl: Sized {
    fn NodeName(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNodeName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn NodeNamespace(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNodeNamespace(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn NodeValue(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetNodeValue(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Language(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetLanguage(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn BaseUri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetBaseUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
    fn AttributeExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<SyndicationAttribute>>;
    fn ElementExtensions(&self) -> ::windows::core::Result<super::super::Foundation::Collections::IVector<ISyndicationNode>>;
    fn GetXmlDocument(&self, format: SyndicationFormat) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISyndicationNodeFactoryImpl: Sized {
    fn CreateSyndicationNode(&self, nodename: &::windows::core::HSTRING, nodenamespace: &::windows::core::HSTRING, nodevalue: &::windows::core::HSTRING) -> ::windows::core::Result<SyndicationNode>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISyndicationPersonImpl: Sized + ISyndicationNodeImpl {
    fn Email(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetEmail(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Name(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetName(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Uri(&self) -> ::windows::core::Result<super::super::Foundation::Uri>;
    fn SetUri(&self, value: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISyndicationPersonFactoryImpl: Sized {
    fn CreateSyndicationPerson(&self, name: &::windows::core::HSTRING) -> ::windows::core::Result<SyndicationPerson>;
    fn CreateSyndicationPersonEx(&self, name: &::windows::core::HSTRING, email: &::windows::core::HSTRING, uri: &::core::option::Option<super::super::Foundation::Uri>) -> ::windows::core::Result<SyndicationPerson>;
}
pub trait ISyndicationTextImpl: Sized + ISyndicationNodeImpl {
    fn Text(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetText(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Type(&self) -> ::windows::core::Result<::windows::core::HSTRING>;
    fn SetType(&self, value: &::windows::core::HSTRING) -> ::windows::core::Result<()>;
    fn Xml(&self) -> ::windows::core::Result<super::super::Data::Xml::Dom::XmlDocument>;
    fn SetXml(&self, value: &::core::option::Option<super::super::Data::Xml::Dom::XmlDocument>) -> ::windows::core::Result<()>;
}
#[cfg(feature = "implement_exclusive")]
pub trait ISyndicationTextFactoryImpl: Sized {
    fn CreateSyndicationText(&self, text: &::windows::core::HSTRING) -> ::windows::core::Result<SyndicationText>;
    fn CreateSyndicationTextEx(&self, text: &::windows::core::HSTRING, r#type: SyndicationTextType) -> ::windows::core::Result<SyndicationText>;
}
