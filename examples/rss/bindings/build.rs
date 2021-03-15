fn main() {
    windows::build!(
        windows::foundation::collections::IVector,
        windows::foundation::{IAsyncOperationWithProgress, Uri},

        windows::web::syndication::{
            ISyndicationText, RetrievalProgress, SyndicationClient, SyndicationFeed, SyndicationItem,
        },
    );
}
