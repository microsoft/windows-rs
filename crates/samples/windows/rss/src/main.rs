use windows::{core::*, Foundation::Uri, Web::Syndication::SyndicationClient};

fn main() -> Result<()> {
    let uri = Uri::CreateUri(h!("https://blog.rust-lang.org/feed.xml"))?;
    let client = SyndicationClient::new()?;
    let feed = client.RetrieveFeedAsync(&uri)?.get()?;

    for item in feed.Items()? {
        println!("{}", item.Title()?.Text()?);
    }

    Ok(())
}
