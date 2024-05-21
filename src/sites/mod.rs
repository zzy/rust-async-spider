pub mod this_week_in_rust_org;
pub mod lobste_rs;
pub mod datatau_net;

use std::{fmt, result::Result};
use reqwest::Error;
use scraper::{ElementRef, Html, Selector};
use colored::Colorize;

#[derive(Debug)]
pub struct Site {
    name: String,
    articles: Vec<Article>,
}

impl fmt::Display for Site {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.name.blue().bold())?;
        for article in &self.articles {
            writeln!(f, "{}", article)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Article {
    title: String,
    link: Option<String>,
}

impl fmt::Display for Article {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.link {
            Some(link) => write!(f, "\t{}\n\t\t({})", self.title.green(), link),
            None => write!(f, "\t{}", self.title),
        }
    }
}

async fn get_html(uri: &str) -> Result<Html, Error> {
    let recv_str = reqwest::get(uri).await?.text().await?;
    let body_html = Html::parse_document(&recv_str);

    Ok(body_html)
}

async fn selector() -> Selector {
    Selector::parse("a.storylink").unwrap()
}

pub fn parse_title(element: ElementRef) -> String {
    element.inner_html()
}

pub fn parse_link(element: ElementRef) -> Option<String> {
    let mut link: Option<String> = None;
    if let Some(link_str) = element.value().attr("href") {
        let link_str = link_str.to_owned();
        link = Some(link_str);
    }

    link
}
