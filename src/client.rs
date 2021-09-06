use serde::de::DeserializeOwned;
use crate::models::{SearchResponse, AlbumDetail};
use crate::{Result, VGMError};

pub struct VGMClient {
    host: String,
    client: reqwest::Client,
}

impl VGMClient {
    pub fn new(mut host: String) -> Self {
        if !host.ends_with("/") {
            host += "/"
        }
        Self {
            host,
            client: Default::default(),
        }
    }

    pub async fn request<T, S>(&self, path: S, param: Option<String>) -> Result<T>
        where T: DeserializeOwned,
              S: AsRef<str> {
        Ok(self.client.get(format!("{host}{path}?format=json&{param}",
                                   host = self.host,
                                   path = path.as_ref(),
                                   param = param.unwrap_or_default()))
            .send().await?
            .json().await?)
    }

    pub async fn search(&self, query: &str) -> Result<SearchResponse> {
        Ok(self.request("search", Some(format!("q={}", query))).await?)
    }

    pub async fn album(&self, catalog: &str) -> Result<AlbumDetail> {
        let result = self.search(catalog).await?;
        if result.results().albums.is_empty() {
            return Err(VGMError::NoAlbumFound);
        }
        Ok(result.results().albums[0].detail(&self).await?)
    }
}

impl Default for VGMClient {
    fn default() -> Self {
        Self::new("https://vgmdb.info/".to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::client::VGMClient;

    #[tokio::test]
    async fn test_search() -> Result<(), Box<dyn std::error::Error>> {
        // THE IDOLM@STER SHINY COLORS GR@DATE WING 05
        let result = VGMClient::default().search("LACM-14986").await?;
        println!("{:#?}", result);
        Ok(())
    }

    #[tokio::test]
    async fn test_album() -> Result<(), Box<dyn std::error::Error>> {
        let client = VGMClient::default();
        let album = client.album("BNEI-ML/RI-2017").await?;
        println!("{:#?}", album);
        Ok(())
    }
}
