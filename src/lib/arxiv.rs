use chrono::prelude::*;
use std::time::Duration;

#[derive(Debug)]
pub struct Entry {
    pub title: String,
    // link: String,
    pub pdf_link: String,
    // authors: Vec<String>,
    pub summary: String,
    // published: DateTime<Utc>,
}

pub struct Client {
    pub client: reqwest::blocking::Client,
}

impl Default for Client {
    fn default() -> Self {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        Self { client }
    }
}
