use chrono::prelude::*;
use roxmltree::Node;
use std::time::Duration;

#[derive(Debug)]
pub struct Entry {
    pub title: String,
    // link: String,
    pub pdf_link: String,
    pub authors: String,
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

pub fn get_title(node: &Node) -> String {
    match node.descendants().find(|m| m.has_tag_name("title")) {
        Some(n) => match &n.text() {
            Some(t) => t.to_string(),
            None => "Missing Title".to_string(),
        },
        None => "Missing Title".to_string(),
    }
}

pub fn get_summary(node: &Node) -> String {
    match node.descendants().find(|m| m.has_tag_name("summary")) {
        Some(n) => match &n.text() {
            Some(t) => t.replace('\n', " ").replace('\t', " "),
            None => "No summary found".to_string(),
        },
        None => "No summary found".to_string(),
    }
}

pub fn get_pdf_link(node: &Node) -> String {
    let link_node = node.descendants().find(|m| {
        let attrs = m
            .attributes()
            .iter()
            .find(|a| a.name() == "title" && a.value() == "pdf");
        m.has_tag_name("link") && attrs.is_some()
    });
    match link_node {
        Some(n) => match &n
            .attributes()
            .iter()
            .find(|a| a.name() == "href")
            .map(|a| a.value())
        {
            Some(v) => v.to_string(),
            None => "https://arxiv.org".to_string(),
        },
        None => "https://arxiv.org".to_string(),
    }
}

pub fn get_last_updated(node: &Node) -> String {
    match node.descendants().find(|m| m.has_tag_name("updated")) {
        Some(n) => match &n.text() {
            Some(t) => t.to_string(),
            None => "Unknown".to_string(),
        },
        None => "Unknown".to_string(),
    }
}

pub fn get_authors(node: &Node) -> String {
    let authors = node
        .descendants()
        .filter(|m| m.has_tag_name("author"))
        .map(|m| {
            m.descendants()
                .filter(|c| c.has_tag_name("name"))
                .map(|a| a.text().unwrap_or("").to_string())
        })
        .flatten()
        .collect::<Vec<String>>();
    authors.join(", ")
}
