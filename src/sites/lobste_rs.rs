use std::result::Result;
use surf::Error;
use scraper::Selector;

use super::{Site, Article, get_html, parse_title, parse_link};

pub async fn crawling() -> Result<Site, Error> {
    let s = Selector::parse("a.u-url").unwrap();
    let body = get_html("https://lobste.rs/").await?;
    let articles = body
        .select(&s)
        .map(|element| Article {
            title: parse_title(element),
            link: parse_link(element),
        })
        .collect();
    let site = Site { name: "lobste.rs".to_string(), articles };

    Ok(site)
}
