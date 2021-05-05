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

    pub arrangers: Vec<NamedItem>,
    pub composers: Vec<NamedItem>,
    pub lyricists: Vec<NamedItem>,
    pub performers: Vec<NamedItem>,

    pub covers: Vec<AlbumArt>,
    discs: Vec<Disc>,
    pub media_format: String,
    pub meta: Meta,
    pub picture_full: String,
    pub picture_small: String,
    pub picture_thumb: String,
    pub publish_format: String,

    pub category: Option<String>,
    #[serde(default)]
    pub categories: Vec<String>,

    #[serde(default)]
    pub organizations: Vec<NamedItem>,
    pub distributor: Option<NamedItem>,

    pub publisher: Option<NamedItem>,
    #[serde(default)]
    pub platforms: Vec<String>,
    #[serde(default)]
    pub products: Vec<NamedItem>,

    pub release_date: Option<String>,
    pub release_price: Option<Price>,
    #[serde(default)]
    pub websites: HashMap<String, WebsiteItem>,

    pub votes: u32,
    pub rating: Option<f32>,
    #[serde(default)]
    pub related: Vec<RelatedAlbum>,
    #[serde(default)]
    pub reprints: Vec<ReprintedAlbum>,
    #[serde(default)]
    pub stores: Vec<WebsiteItem>,
}

impl AlbumDetail {
    pub fn vgmdb_link(&self) -> &str {
        self.vgmdb_link.as_str()
    }

    pub fn name(&self) -> &str {
        self.names.get("jp")
            .map(|k| k.as_str())
            .unwrap_or(
                self.names.get("Japanese")
                    .map(|k| k.as_str())
                    .unwrap_or(self.name.as_str())
            )
    }

    pub fn catalog(&self) -> &str {
        self.catalog.as_str()
    }

    pub fn notes(&self) -> &str {
        self.notes.as_str()
    }

    pub fn classification(&self) -> &str {
        self.classification.as_str()
    }

    pub fn discs(&self) -> &[Disc] {
        &self.discs
    }
}

type MultiLanguageString = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
struct ArtistInfo {
    aliases: Vec<String>,
    link: String,
    names: MultiLanguageString,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamedItem {
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
pub struct AlbumArt {
    name: String,
    full: String,
    medium: String,
    thumb: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Price {
    price: PriceInner,
    currency: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PriceInner {
    Number(f32),
    String(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebsiteItem {
    link: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReprintedAlbum {
    link: String,
    catalog: String,
    note: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RelatedAlbum {
    catalog: String,
    link: String,
    names: MultiLanguageString,
    #[serde(rename = "type")]
    album_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Disc {
    disc_length: String,
    name: String,
    tracks: Vec<Track>,
}

impl Disc {
    pub fn length(&self) -> &str {
        self.disc_length.as_str()
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn tracks(&self) -> &[Track] {
        &self.tracks
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
    names: MultiLanguageString,
    track_length: String,
}

impl Track {
    pub fn name(&self) -> &str {
        if let Some(value) = self.names.get("jp") {
            return value.as_str();
        } else if let Some(value) = self.names.get("Japanese") {
            return value.as_str();
        } else {
            for (_, value) in &self.names {
                return value.as_str();
            }
            unreachable!()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Meta {
    added_date: String,
    edited_date: String,
    fetched_date: Option<String>,
    ttl: u32,
    visitors: u32,
}
