//! Return YouTube Dislike (estudo 44): leitura pública de `GET /votes`.

use anyhow::anyhow;
use serde::{Deserialize, Serialize};

const API: &str = "https://returnyoutubedislikeapi.com";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Votes {
    pub id: String,
    #[serde(rename(deserialize = "dateCreated"), default)]
    pub date_created: String,
    #[serde(default)]
    pub likes: u64,
    #[serde(default)]
    pub dislikes: u64,
    #[serde(default)]
    pub rating: f64,
    #[serde(rename(deserialize = "viewCount"), default)]
    pub view_count: u64,
    #[serde(default)]
    pub deleted: bool,
}

pub async fn votes(input: &str) -> anyhow::Result<Votes> {
    let id = super::sponsorblock::video_id(input)
        .ok_or_else(|| anyhow!("nao reconheci um video do YouTube em: {}", input))?;
    let client = super::client()?;
    let resp = client
        .get(format!("{}/votes?videoId={}", API, id))
        .send()
        .await?;
    if !resp.status().is_success() {
        return Err(anyhow!("Return YouTube Dislike: HTTP {}", resp.status()));
    }
    Ok(resp.json().await?)
}

#[cfg(test)]
mod tests {
    use super::Votes;

    #[test]
    fn votes_reads_api_camel_case_and_writes_snake_case() {
        let api = r#"{"id":"dQw4w9WgXcQ","dateCreated":"2026-09-13T00:00:00Z",
            "likes":100,"rawDislikes":3,"rawLikes":90,"dislikes":5,
            "rating":4.8,"viewCount":123456,"deleted":false}"#;
        let v: Votes = serde_json::from_str(api).unwrap();
        assert_eq!(v.view_count, 123456);
        assert_eq!(v.date_created, "2026-09-13T00:00:00Z");

        let out = serde_json::to_value(&v).unwrap();
        let obj = out.as_object().unwrap();
        for key in [
            "id",
            "date_created",
            "likes",
            "dislikes",
            "rating",
            "view_count",
            "deleted",
        ] {
            assert!(obj.contains_key(key), "missing {key}");
        }
        assert!(!obj.contains_key("viewCount"));
        assert!(!obj.contains_key("dateCreated"));
        assert_eq!(out["view_count"], 123456);
    }
}
