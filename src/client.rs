use std::str::FromStr;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};
use crate::models::{SearchResult, AlbumInfo, SearchResponse, AlbumDetail};
use crate::Result;
use crate::utils::{parse_date, parse_multi_language};

#[derive(Default)]
pub struct VGMClient {
    client: reqwest::Client,
}

impl VGMClient {
    pub async fn search_albums(&self, query: &str) -> Result<SearchResponse<'_>> {
        let response = self.client.get(&format!("https://vgmdb.net/search?type=album&q={query}"))
            .header("Cookie", "TODO")
            .send().await?;
        if response.url().path().starts_with("/album") {
            Ok(SearchResponse::new(
                self,
                SearchResult::Album(AlbumDetail::from_str(&response.text().await?)?),
            ))
        } else {
            let html = response.text().await?;
            let document = Document::from(html.as_str());

            let mut results = Vec::new();
            for row in document.select(Attr("id", "albumresults").descendant(Name("tr").and(Attr("rel", "rel_invalid")))) {
                let cells = row.select(Name("td")).collect::<Vec<_>>();

                // 1. get catalog
                let catalog = cells[0].text();
                let catalog = if catalog != "N/A" {
                    Some(catalog)
                } else {
                    None
                };

                // 2. get multi-language title
                let title = parse_multi_language(&cells[2]);

                // 3. get release date
                let release_date = cells[3].text();
                let release_date = parse_date(release_date.trim())?;

                // 4. get album link
                let link = cells[2].select(Name("a")).next().unwrap().attr("href").unwrap().to_string();

                results.push(AlbumInfo {
                    catalog,
                    title,
                    release_date,
                    id: link.strip_prefix("https://vgmdb.net/album/").unwrap().to_string(),
                });
            }

            Ok(SearchResponse::new(self, SearchResult::List(results)))
        }
    }

    pub async fn album(&self, id: &str) -> Result<AlbumDetail> {
        let response = self.client.get(&format!("https://vgmdb.net/album/{id}"))
            .header("Cookie", "TODO")
            .send().await?;
        let html = response.text().await?;
        AlbumDetail::from_str(html.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::client::VGMClient;

    #[tokio::test]
    async fn test_search() -> Result<(), Box<dyn std::error::Error>> {
        let client = VGMClient::default();
        let result = client.search_albums("BNEI-ML").await?;
        println!("{:#?}", result);
        Ok(())
    }

    #[tokio::test]
    async fn test_album() -> Result<(), Box<dyn std::error::Error>> {
        let client = VGMClient::default();
        let result = client.search_albums("LACA-9356~7").await?;
        let album = result.into_album(None).await?;
        println!("{:#?}", album);
        Ok(())
    }
}
