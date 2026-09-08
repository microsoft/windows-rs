fn main() -> windows::core::Result<()> {
    use windows::{Foundation::Uri, Web::Syndication::SyndicationClient, core::*};

    async fn main_async() -> Result<()> {
        let uri = Uri::CreateUri(h!("https://github.com/microsoft/windows-rs/releases.atom"))?;
        let client = SyndicationClient::new()?;

        client.SetRequestHeader(h!("User-Agent"), h!("windows-rs RSS sample"))?;

        let feed = client.RetrieveFeedAsync(&uri)?.await?;

        for item in feed.Items()? {
            println!("{:?}", item.Title()?.Text()?);
        }

        Ok(())
    }

    futures::executor::block_on(main_async())
}
