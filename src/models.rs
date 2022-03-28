use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};
use crate::{VGMClient, Result, VGMError};
use crate::utils::{parse_date, parse_multi_language};

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

    pub async fn get_album(&self, index: Option<usize>) -> Result<AlbumDetail> {
        match &self.inner {
            SearchResult::Album(data) => AlbumDetail::from_str(data),
            SearchResult::List(list) => {
                let index = index.unwrap_or(0);
                let info = &list[index];
                self.client.album(&info.id).await
            }
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
    pub id: String,
}

#[derive(Debug)]
pub struct AlbumDetail {
    pub link: String,

    pub title: MultiLanguageString,
    pub catalog: Option<String>,
    pub release_date: String,
    pub discs: Vec<Disc>,
}

impl FromStr for AlbumDetail {
    type Err = VGMError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let document = Document::from(s);

        // 1. title
        let title = document.select(Name("h1")).next().unwrap();
        let title = parse_multi_language(&title);

        let info = document
            .select(Attr("id", "album_infobit_large"))
            .nth(0).unwrap();

        // 2. catalog
        let mut catalog = None;
        // 3. release date
        let mut release_date = None;

        for line in info.select(Name("tr")) {
            let key = line.select(Name("b")).next().unwrap().text();
            let value = line.last_child().unwrap().text();
            if key == "Catalog Number" {
                catalog = Some(value.trim().to_string());
                if let Some("N/A") = catalog.as_deref() {
                    catalog = None;
                }
            } else if key == "Release Date" {
                release_date = parse_date(value.trim()).ok();
            }
        }

        let mut album = AlbumDetail {
            link: "".to_string(), // TODO: get link
            title,
            catalog,
            release_date: release_date.unwrap(),
            discs: vec![],
        };

        // 4. track_list
        let track_list_nav = document.select(Attr("id", "tlnav")).next().unwrap();
        let track_list = document.select(Attr("id", "tracklist")).next().unwrap();
        for list in track_list.select(Attr("class", "tl")) {
            let reference = list.attr("id").unwrap();
            let language = track_list_nav.select(Attr("rel", reference)).next().unwrap().text();

            let mut discs = Vec::new();
            for disc in list.select(Attr("style", "font-size:8pt").descendant(Name("b"))) {
                let disc_title = disc.text();
                let mut table = disc.parent().unwrap();
                loop {
                    table = table.next().unwrap();
                    if let Some("table") = table.name() {
                        break;
                    }
                }
                let mut tracks = Vec::new();
                for track in table.select(Name("tr")) {
                    let track_name = track.select(Name("td").and(Attr("width", "100%"))).next().unwrap().text();
                    let track_name = track_name.trim().to_string();
                    tracks.push(track_name);
                }
                discs.push((disc_title, tracks));
            }

            if album.discs.is_empty() {
                // initialize MultiLanguage tracks
                album.discs.append(&mut discs.into_iter().map(|(title, tracks)| {
                    let tracks = tracks.into_iter().map(|track| {
                        let mut tracks = MultiLanguageString::default();
                        tracks.insert(language.to_string(), track);
                        tracks
                    }).collect();
                    Disc {
                        title,
                        tracks,
                    }
                }).collect::<Vec<_>>());
            } else {
                for (disc, (_, tracks)) in album.discs.iter_mut().zip(discs.into_iter()) {
                    for (track, new_track) in disc.tracks.iter_mut().zip(tracks.into_iter()) {
                        track.insert(language.to_string(), new_track);
                    }
                }
            }
        }

        Ok(album)
    }
}


#[derive(Debug)]
pub struct Disc {
    pub title: String,
    pub tracks: Vec<MultiLanguageString>,
}

#[derive(Debug, Default)]
pub struct MultiLanguageString(HashMap<String, String>);

impl MultiLanguageString {
    pub fn insert(&mut self, language: String, value: String) {
        self.0.insert(language, value);
    }

    pub fn get(&self) -> Option<&str> {
        self.0.get("ja")
            .or_else(|| self.0.get("Japanese"))
            .or_else(|| self.0.get("English"))
            .or_else(|| self.0.values().next())
            .map(|s| s.as_str())
    }
}
