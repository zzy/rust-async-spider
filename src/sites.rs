use colored::Colorize;
use reqwest::{blocking, Error};
use scraper::{ElementRef, Html, Selector};
use std::{fmt, result::Result};

#[derive(Debug)]
pub struct Site {
    name: String,
    stories: Vec<Story>,
}

impl fmt::Display for Site {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.name.blue().bold())?;
        for story in &self.stories {
            writeln!(f, "{}", story)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Story {
    title: String,
    link: Option<String>,
}

impl fmt::Display for Story {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.link {
            Some(link) => write!(f, "\t{}\n\t\t({})", self.title.green(), link),
            None => write!(f, "\t{}", self.title),
        }
    }
}

pub async fn this_week_in_rust_org() -> Result<Site, Error> {
    let s = Selector::parse("div.col-md-12 a").unwrap();
    let body = get_html("https://this-week-in-rust.org/blog/archives/index.html").await?;
    let stories = body
        .select(&s)
        .map(|element| Story {
            title: parse_title(element),
            link: parse_link(element),
        })
        .collect();
    let site = Site {
        name: "this-week-in-rust.org".to_string(),
        stories,
    };

    Ok(site)
}

async fn get_html(uri: &str) -> Result<Html, Error> {
    Ok(Html::parse_document(&blocking::get(uri)?.text()?))
}

fn parse_link(element: ElementRef) -> Option<String> {
    let mut link: Option<String> = None;
    if let Some(link_str) = element.value().attr("href") {
        let link_str = link_str.to_owned();
        link = Some(link_str);
    }

    link
}

fn parse_title(element: ElementRef) -> String {
    element.inner_html()
}
