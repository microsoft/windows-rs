fn main() {
    windows::build! {
        Windows::Foundation::Collections::IVector,
        Windows::Foundation::{IAsyncOperationWithProgress, Uri},

        Windows::Web::Syndication::{
            ISyndicationText, RetrievalProgress, SyndicationClient, SyndicationFeed, SyndicationItem,
        },
    };
}
