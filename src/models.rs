use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::{VGMClient, Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    link: String,
    results: SearchResults,
}

impl SearchResponse {
    pub fn results(&self) -> &SearchResults {
        &self.results
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResults {
    pub albums: Vec<AlbumInfo>,
    artists: Vec<ArtistInfo>,
    orgs: Vec<NamedItem>,
    products: Vec<Product>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumInfo {
    catalog: String,
    link: String,
    release_date: String,
    titles: MultiLanguageString,
}

impl AlbumInfo {
    pub async fn detail(&self, client: &VGMClient) -> Result<AlbumDetail> {
        Ok(client.request(&self.link).await?)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumDetail {
    link: String,
    vgmdb_link: String,

    name: String,
    names: MultiLanguageString,
    catalog: String,
    notes: String,
    classification: String,

    arrangers: Vec<NamedItem>,
    composers: Vec<NamedItem>,
    lyricists: Vec<NamedItem>,
    performers: Vec<NamedItem>,

    covers: Vec<AlbumArt>,
    discs: Vec<Disc>,
    media_format: String,
    meta: Meta,
    picture_full: String,
    picture_small: String,
    picture_thumb: String,
    publish_format: String,

    category: Option<String>,
    #[serde(default)]
    categories: Vec<String>,

    #[serde(default)]
    organizations: Vec<NamedItem>,
    distributor: Option<NamedItem>,

    publisher: Option<NamedItem>,
    #[serde(default)]
    platforms: Vec<String>,
    #[serde(default)]
    products: Vec<NamedItem>,

    release_date: Option<String>,
    release_price: Option<Price>,
    #[serde(default)]
    websites: HashMap<String, WebsiteItem>,

    votes: u32,
    rating: Option<f32>,
    #[serde(default)]
    related: Vec<RelatedAlbum>,
    #[serde(default)]
    reprints: Vec<ReprintedAlbum>,
    #[serde(default)]
    stores: Vec<WebsiteItem>,
}

type MultiLanguageString = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
struct ArtistInfo {
    aliases: Vec<String>,
    link: String,
    names: MultiLanguageString,
}

#[derive(Debug, Serialize, Deserialize)]
struct NamedItem {
    link: String,
    names: MultiLanguageString,
}

#[derive(Debug, Serialize, Deserialize)]
struct Product {
    link: String,
    names: MultiLanguageString,
    #[serde(rename = "type")]
    product_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct AlbumArt {
    name: String,
    full: String,
    medium: String,
    thumb: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Price {
    price: PriceInner,
    currency: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum PriceInner {
    Number(f32),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
struct WebsiteItem {
    link: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ReprintedAlbum {
    link: String,
    catalog: String,
    note: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct RelatedAlbum {
    catalog: String,
    link: String,
    names: MultiLanguageString,
    #[serde(rename = "type")]
    album_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Disc {
    disc_length: String,
    name: String,
    tracks: Vec<Track>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Track {
    names: MultiLanguageString,
    track_length: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Meta {
    added_date: String,
    edited_date: String,
    fetched_date: Option<String>,
    ttl: u32,
    visitors: u32,
}
