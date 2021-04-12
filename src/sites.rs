use std::{fmt, result::Result};
use surf::Error;
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

pub async fn hacker_news() -> Result<Site, Error> {
    let body = get_html("https://news.ycombinator.com/").await?;
    let articles = body
        .select(&selector().await)
        .map(|element| Article {
            title: parse_title(element),
            link: parse_link(element),
        })
        .collect();
    let site = Site { name: "Hacker News".to_string(), articles };

    Ok(site)
}

pub async fn datatau() -> Result<Site, Error> {
    let body = get_html("https://datatau.net/").await?;
    let articles = body
        .select(&selector().await)
        .map(|element| Article {
            title: parse_title(element),
            link: parse_link(element),
        })
        .collect();
    let site = Site { name: "DataTau".to_string(), articles };

    Ok(site)
}

pub async fn lobsters() -> Result<Site, Error> {
    let s = Selector::parse("a.u-url").unwrap();
    let body = get_html("https://lobste.rs/").await?;
    let articles = body
        .select(&s)
        .map(|element| Article {
            title: parse_title(element),
            link: parse_link(element),
        })
        .collect();
    let site = Site { name: "Lobste.rs".to_string(), articles };

    Ok(site)
}
async fn get_html(uri: &str) -> Result<Html, Error> {
    let recv_str = surf::get(uri).recv_string().await?;
    let body_html = Html::parse_document(&recv_str);

    Ok(body_html)
}

async fn selector() -> Selector {
    Selector::parse("a.storylink").unwrap()
}

fn parse_title(element: ElementRef) -> String {
    element.inner_html()
}

fn parse_link(element: ElementRef) -> Option<String> {
    let mut link: Option<String> = None;
    if let Some(link_str) = element.value().attr("href") {
        let mut link_str = link_str.to_owned();
        if link_str.starts_with("item?") {
            link_str = format!("https://news.ycombinator.com/{}", link_str);
        }
        link = Some(link_str);
    }

    link
}
