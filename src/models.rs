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

#[derive(Debug)]
pub struct AlbumDetail {
    pub link: String,

    pub title: MultiLanguageString,
    pub catalog: Option<String>,
    pub release_date: String,
    pub discs: Vec<Disc>,
}

pub(crate) type MultiLanguageString = HashMap<String, String>;

#[derive(Debug)]
pub struct Disc {
    pub title: String,
    pub tracks: Vec<MultiLanguageString>,
}
