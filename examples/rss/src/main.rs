use bindings::{
    windows::foundation::Uri,
    windows::web::syndication::SyndicationClient,
};

fn main() -> windows::Result<()> {
    let uri = Uri::create_uri("https://kennykerr.ca/feed")?;
    let client = SyndicationClient::new()?;
    let feed = client.retrieve_feed_async(uri)?.get()?;

    for item in feed.items()? {
        println!("{}", item.title()?.text()?);
    }

    Ok(())
}
