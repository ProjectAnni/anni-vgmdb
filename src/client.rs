use select::document::Document;
use select::predicate::{Attr, Name, Predicate};
use crate::models::{SearchResult, AlbumInfo, SearchResponse};
use crate::Result;
use crate::utils::{parse_date, parse_multi_language};

#[derive(Default)]
pub struct VGMClient {
    client: reqwest::Client,
}

impl VGMClient {
    // pub async fn request<T, S>(&self, path: S, param: Option<String>) -> Result<T>
    //     where T: DeserializeOwned,
    //           S: AsRef<str> {
    // }

    pub async fn search_albums(&self, query: &str) -> Result<SearchResponse<'_>> {
        let response = self.client.get(&format!("https://vgmdb.net/search?type=album&q={query}"))
            .header("Cookie", "TODO")
            .send().await?;
        if response.url().path().starts_with("/album") {
            Ok(SearchResponse::new(
                self,
                SearchResult::Album(response.text().await?),
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
                    link,
                });
            }

            Ok(SearchResponse::new(self, SearchResult::List(results)))
        }
    }

    // pub async fn album(&self, catalog: &str) -> Result<AlbumDetail> {
    //     let result = self.search(catalog).await?;
    //     if result.results().albums.is_empty() {
    //         return Err(VGMError::NoAlbumFound);
    //     }
    //     Ok(result.results().albums[0].detail(&self).await?)
    // }
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

    // #[tokio::test]
    // async fn test_album() -> Result<(), Box<dyn std::error::Error>> {
    //     let client = VGMClient::default();
    //     let album = client.album("BNEI-ML/RI-2017").await?;
    //     println!("{:#?}", album);
    //     Ok(())
    // }
}
