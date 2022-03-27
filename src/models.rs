use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::VGMClient;

pub struct SearchResponse<'client> {
    client: &'client VGMClient,
    inner: SearchResult,
}

impl<'client> SearchResponse<'client> {
    pub(crate) fn new(client: &'client VGMClient, inner: SearchResult) -> Self {
        SearchResponse {
            client,
            inner,
        }
    }
}

impl<'client> Debug for SearchResponse<'client> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

#[derive(Debug)]
pub enum SearchResult {
    Album(String),
    List(Vec<AlbumInfo>),
}

#[derive(Debug)]
pub struct AlbumInfo {
    pub catalog: Option<String>,
    pub title: MultiLanguageString,
    pub release_date: String,
    pub link: String,
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

    pub votes: u32,
    pub rating: Option<f32>,
}

impl AlbumDetail {
    pub fn vgmdb_link(&self) -> &str {
        self.vgmdb_link.as_str()
    }

    pub fn name(&self) -> &str {
        self.names.get("ja")
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

pub(crate) type MultiLanguageString = HashMap<String, String>;

#[derive(Debug, Serialize, Deserialize)]
struct ArtistInfo {
    aliases: Vec<String>,
    link: String,
    names: MultiLanguageString,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamedItem {
    link: Option<String>,
    names: MultiLanguageString,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AlbumArt {
    name: String,
    full: String,
    medium: String,
    thumb: String,
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
        if let Some(value) = self.names.get("ja") {
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
