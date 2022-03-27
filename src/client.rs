use std::collections::HashMap;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};
use crate::models::{SearchResult, AlbumInfo, SearchResponse, AlbumDetail, Disc};
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

    pub async fn album(&self, id: &str) -> Result<AlbumDetail> {
        let response = self.client.get(&format!("https://vgmdb.net/album/{id}"))
            .header("Cookie", "TODO")
            .send().await?;
        let html = response.text().await?;
        let document = Document::from(html.as_str());

        // 1. title
        let title = document.select(Name("h1")).next().unwrap();
        let title = parse_multi_language(&title);

        let info = document
            .select(Attr("id", "album_infobit_large"))
            .nth(0).unwrap()
            .select(Name("tr"))
            .collect::<Vec<_>>();

        // 2. catalog
        let catalog = info[0].select(Name("td").and(Attr("width", "100%"))).next().unwrap().text();
        // 3. release date
        let release_date = info[2].select(Name("td").descendant(Name("a"))).next().unwrap().text();
        let release_date = parse_date(release_date.trim())?;

        let mut album = AlbumDetail {
            link: "".to_string(),
            title,
            catalog: Some(catalog), // TODO: N/A
            release_date,
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
                        let mut tracks = HashMap::new();
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
        let album = client.album("31040").await?;
        println!("{:#?}", album);
        Ok(())
    }
}
