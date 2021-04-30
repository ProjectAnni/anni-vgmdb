use serde::de::DeserializeOwned;
use crate::models::{SearchResponse, AlbumDetail};

pub struct VGMClient {
    client: reqwest::Client,
}

impl VGMClient {
    pub fn new() -> Self {
        Self {
            client: Default::default(),
        }
    }

    pub async fn request<T, S>(&self, path: S) -> reqwest::Result<T>
        where T: DeserializeOwned,
              S: AsRef<str> {
        Ok(self.client.get(format!("https://vgmdb.info/{}?format=json", path.as_ref()))
            .send().await?
            .json().await?)
    }

    pub async fn search(&self, query: &str) -> reqwest::Result<SearchResponse> {
        self.request(format!("search/{}", query)).await
    }

    pub async fn album(&self, id: &str) -> reqwest::Result<AlbumDetail> {
        self.request(format!("album/{}", id)).await
    }
}

#[cfg(test)]
mod tests {
    use crate::client::VGMClient;

    #[tokio::test]
    async fn test_search() -> Result<(), Box<dyn std::error::Error>> {
        let result = VGMClient::new().search("幼なじみが絶対に負けないラブコメ").await?;
        println!("{:#?}", result);
        Ok(())
    }

    #[tokio::test]
    async fn test_album() -> Result<(), Box<dyn std::error::Error>> {
        let client = VGMClient::new();
        let response = client.search("幼なじみが絶対に負けないラブコメ").await?;
        let ref albums = response.results().albums;
        let detail = albums[0].detail(&client).await?;
        println!("{:#?}", detail);
        Ok(())
    }
}
