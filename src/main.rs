use reqwest::blocking;

struct PressReleaseLink {
    url: String,
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



fn main() {
    let response = blocking::get("https://ir.vikingtherapeutics.com/press-releases",)
    .unwrap().text().unwrap();

    let document = scraper::Html::parse_document(&response);

    let mut press_releases: Vec<PressReleaseLink> = Vec::new();

    let html_pressrelease_selector = scraper::Selector::parse("div").unwrap();
    let html_pressreleases = document.select(&html_pressrelease_selector);

    for html_pressrelease in html_pressreleases {
        let url = html_pressrelease
            .select(&scraper::Selector::parse("a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            .map(str::to_owned);

    
        if let Some(url) = url {
            if !keyword_check(&url) {
                continue;
            }
            let press_release_link = PressReleaseLink {url};
            press_releases.push(press_release_link);
        }
    }
    for (index, release) in press_releases.iter().enumerate() {
        println!("Press Release #{}", index + 1);
        println!("Url: {:?}", release.url);
    }
}
