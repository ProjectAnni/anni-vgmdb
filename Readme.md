# anni-vgmdb

[![crates.io](https://img.shields.io/crates/v/anni-vgmdb.svg)](https://crates.io/crates/anni-vgmdb)
[![API](https://docs.rs/anni-vgmdb/badge.svg)](https://docs.rs/anni-vgmdb)

## Example

```rust
use anni_vgmdb::VGMClient;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = VGMClient::default();
    let search = client.search_albums("TEST").await?;
    for album in search.albums() {
        println!("{:?}", album);
    }

    if !search.is_empty() {
        let album = search.into_album(None).await?;
        println!("{:?}", album);
    }

    Ok(())
}
```