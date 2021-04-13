use std::result::Result;
use surf::Error;
use scraper::Selector;

use super::{Site, Article, get_html, parse_title, parse_link};

pub async fn crawling() -> Result<Site, Error> {
    let s = Selector::parse("div.col-md-12 a").unwrap();
    let body = get_html("https://this-week-in-rust.org").await?;
    let articles = body
        .select(&s)
        .map(|element| Article {
            title: parse_title(element),
            link: parse_link(element),
        })
        .collect();
    let site = Site { name: "this-week-in-rust.org".to_string(), articles };

    Ok(site)
}
