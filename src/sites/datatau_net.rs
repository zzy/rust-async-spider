use std::result::Result;
use reqwest::Error;

use super::{Site, Article, get_html, selector, parse_title, parse_link};

pub async fn crawling() -> Result<Site, Error> {
    let body = get_html("https://datatau.net/").await?;
    let articles = body
        .select(&selector().await)
        .map(|element| Article {
            title: parse_title(element),
            link: parse_link(element),
        })
        .collect();
    let site = Site { name: "datatau.net".to_string(), articles };

    Ok(site)
}
