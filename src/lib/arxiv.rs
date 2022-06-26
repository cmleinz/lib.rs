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

// fn get_docs() -> Result<(), Box<dyn std::error::Error>> {
//     let client = reqwest::blocking::Client::builder()
//         .timeout(Duration::from_secs(5))
//         .build()
//         .unwrap();
//     let resp = client
//         .get("http://export.arxiv.org/api/query?search_query=all:electron&start=0&max_results=10")
//         .send()?;
//     let text = resp.text()?;
//     println!("{}", text);
//     let doc = roxmltree::Document::parse(&text).unwrap();
//     let elem = doc
//         .descendants()
//         .filter(|n| n.tag_name().name() == "title")
//         .map(|n| n.text())
//         .flatten()
//         .collect::<Vec<_>>();
//     for title in elem {
//         println!("{}", title);
//     }
//     Ok(())
// }
