use std::fmt::Formatter;

use serde::Deserialize;

use tokio::process::Command;

#[derive(Debug, Deserialize)]
pub struct YTPlayListResponse {
    pub url: String,
}

#[derive(Debug)]
pub enum YTPlayListError {
    ListOfUrlsError(Vec<u8>),
}

impl std::fmt::Display for YTPlayListError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ListOfUrlsError(e) => {
                let string_err = String::from_utf8(e.clone());
                write!(f, "ListOfUrlsError: {:?}", string_err)
            }
        }
    }
}

impl std::error::Error for YTPlayListError {}

pub async fn get_list_of_urls(url: String) -> anyhow::Result<Vec<YTPlayListResponse>> {
    let output = Command::new("youtube-dl")
        .args(&["-j", "--flat-playlist", &url])
        .output()
        .await?;

    if !output.status.success() {
        Err(YTPlayListError::ListOfUrlsError(output.stderr).into())
    } else {
        let output = String::from_utf8(output.stdout)?;
        let mut json_output = vec![];
        for line in output.lines() {
            let json: YTPlayListResponse = serde_json::from_str(line)?;
            json_output.push(json);
        }

        Ok(json_output)
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AccessToken {
    access_token: String,
}

async fn get_spotify_access_token(client: reqwest::Client) -> anyhow::Result<AccessToken> {
    let res = client
        .get("https://open.spotify.com/get_access_token")
        .send()
        .await?;

    let token: AccessToken = res.json().await?;

    Ok(token)
}

#[derive(Debug, Deserialize)]
pub struct SpotifyPlaylistResponse {
    pub items: Vec<Items>,
}

#[derive(Debug, Deserialize)]
pub struct Items {
    pub track: Track,
}

#[derive(Debug, Deserialize)]
pub struct Track {
    pub name: String,
    pub artists: Vec<Artists>,
}

#[derive(Debug, Deserialize)]
pub struct Artists {
    pub name: String,
}

pub async fn get_list_of_spotify_tracks(
    client: reqwest::Client,
    playlist_id: &str,
) -> anyhow::Result<SpotifyPlaylistResponse> {
    match get_spotify_access_token(client.clone()).await {
        Ok(token) => {
            let res = client
                .get(&format!(
                    "https://api.spotify.com/v1/playlists/{}/tracks",
                    playlist_id
                ))
                .bearer_auth(token.access_token)
                .send()
                .await?;

            let playlist: SpotifyPlaylistResponse = res.json().await?;

            Ok(playlist)
        }
        Err(e) => Err(e),
    }
}

#[cfg(test)]
mod tests {
    use super::{get_list_of_spotify_tracks, get_spotify_access_token};

    #[tokio::test]
    async fn test_spotify_access_token() {
        let client = reqwest::Client::new();
        let token = get_spotify_access_token(client).await.unwrap();
        println!("{:?}", token);
    }

    #[tokio::test]
    async fn test_spotify_playlist_tracks() {
        let client = reqwest::Client::new();
        let res = get_list_of_spotify_tracks(client, "5I1uPiJpPmphKfQHDjWHFa")
            .await
            .unwrap();

        println!("{:?}", res);
    }
}