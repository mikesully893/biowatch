use reqwest::blocking;

struct PressReleaseLink {
    url: String,
}

fn keyword_check(url: &String) -> bool {
    let keywords = ["Phase-1", "Phase-I", "Phase-2", "Phase-II", "Preliminary"];
    for word in keywords {
        if url.contains(&word) {
            return true
        };
    };
    false
}

//fn add_pressrelease_link(url: String)

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
