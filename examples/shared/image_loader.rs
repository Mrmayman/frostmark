use std::sync::LazyLock;

use reqwest::Client;

// Just a quick and dirty setup for showcase

#[derive(Debug, Clone)]
pub struct Image {
    pub bytes: Vec<u8>,
    pub url: String,
    #[allow(unused)]
    pub is_svg: bool,
}

pub async fn download_image(url: String) -> Result<Image, String> {
    static CLIENT: LazyLock<Client> = LazyLock::new(|| Client::new());
    let response = CLIENT
        .get(&url)
        .send()
        .await
        .map_err(|err| err.to_string())?;

    if !response.status().is_success() {
        Err(format!("Error {} from url: {url}", response.status()))
    } else {
        let bytes = response
            .bytes()
            .await
            .map_err(|err| err.to_string())?
            .to_vec();
        Ok(Image {
            is_svg: bytes.starts_with(b"<svg "),
            url,
            bytes,
        })
    }
}
