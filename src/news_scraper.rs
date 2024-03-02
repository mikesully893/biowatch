use reqwest::{blocking, Error};
use scraper::{Html, Selector};
use chrono::{Utc, Datelike};

pub struct PressReleaseLink {
    pub symbol: String,
    pub url: String,
    pub news_site: NewsSite
}

pub struct Notification {
    pub symbol: String,
    pub press_release_url: String
}

pub enum NewsSite {
    GlobalNewsWire,
    BusinessWire,
}

fn keyword_check(url: &str) -> bool {
    let config_text = include_str!("config.txt");
    let keywords: Vec<String> = config_text.lines().map(|s| s.to_string()).collect();

    for word in &keywords {
        if url.contains(word) {
            return true
        };
    };
    false
}

fn date_check(url: &PressReleaseLink) -> bool {
    let url_string = &url.url;
    let date = get_date(&url.news_site);

    if url_string.contains(&date) {
        return true
    };
    false
}

fn get_date(url: &NewsSite) -> String {
    let current_date = Utc::now();
    let year = current_date.year();
    let month = current_date.month();
    let day = current_date.day();
    
    match url {
        NewsSite::BusinessWire => format!("{}{}{}", year, month, day),
        NewsSite::GlobalNewsWire => format!("{}/{}/{}", year, month, day),
    }
}

pub fn scrape_news_website(url_struct: &PressReleaseLink) -> Result<Vec<Notification>, Error>{
    let response = blocking::get(&url_struct.url)?
    .text()?;

    let document = Html::parse_document(&response);

    let mut press_releases: Vec<Notification> = Vec::new();

    let html_pressrelease_selector = Selector::parse("div").unwrap();
    let html_pressreleases = document.select(&html_pressrelease_selector);

    for html_pressrelease in html_pressreleases {
        let url = html_pressrelease
            .select(&Selector::parse("a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned);

        if !date_check(&url_struct) {
            continue;
        }
    
        if let Some(url) = url {
            if !keyword_check(&url) {
                continue;
            }
            press_releases.push(Notification { symbol: url_struct.symbol.clone(), press_release_url: url });
        }
    }
    
    for (index, release) in press_releases.iter().enumerate() {
        println!("Press Release #{}", index + 1);
        println!("Url: {:?}", &release.press_release_url);
        println!("Symbol: {:?}", &release.symbol);
    }
    Ok(press_releases)
}