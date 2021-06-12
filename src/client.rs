use serde::de::DeserializeOwned;
use crate::models::{SearchResponse, AlbumDetail};
use crate::{Result, VGMError};

pub struct VGMClient {
    client: reqwest::Client,
}

impl VGMClient {
    pub fn new() -> Self {
        Self {
            client: Default::default(),
        }
    }

    pub async fn request<T, S>(&self, path: S) -> Result<T>
        where T: DeserializeOwned,
              S: AsRef<str> {
        Ok(self.client.get(format!("https://vgmdb.info/{}{}format=json", path.as_ref(), if path.as_ref().contains("?") { "&" } else { "?" }))
            .send().await?
            .json().await?)
    }

    pub async fn search(&self, query: &str) -> Result<SearchResponse> {
        Ok(self.request(format!("search?q={}", query)).await?)
    }

    pub async fn album(&self, catalog: &str) -> Result<AlbumDetail> {
        let result = self.search(catalog).await?;
        if result.results().albums.is_empty() {
            return Err(VGMError::NoAlbumFound);
        }
        Ok(result.results().albums[0].detail(&self).await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::client::VGMClient;

    #[tokio::test]
    async fn test_search() -> Result<(), Box<dyn std::error::Error>> {
        // THE IDOLM@STER SHINY COLORS GR@DATE WING 05
        let result = VGMClient::new().search("LACM-14986").await?;
        println!("{:#?}", result);
        Ok(())
    }

    #[tokio::test]
    async fn test_album() -> Result<(), Box<dyn std::error::Error>> {
        let client = VGMClient::new();
        let album = client.album("BNEI-ML/RI-2017").await?;
        println!("{:#?}", album);
        Ok(())
    }
}
