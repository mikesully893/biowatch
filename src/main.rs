use reqwest::blocking;

struct PressReleaseLink {
    url: String,
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
            .select(&scraper::Selector::parse(" a").unwrap())
            .next()
            .and_then(|a| a.value().attr("href"))
            //.and_then(|a| a.inner_html())
            .map(str::to_owned);

        // let press_release_link = PressReleaseLink {
        //     url
        // };

        // match url {
        //     None => (),
        //     Some(url) => {let press_release_link = PressReleaseLink {url};
        //     press_releases.push(press_release_link);}
        // }
        if let Some(url) = url {
            let press_release_link = PressReleaseLink {url};
            press_releases.push(press_release_link);
        }

        // match &press_release_link.url {
            
        //     Some(i) => press_releases.push(press_release_link),
        // }
        
    }

    for (index, release) in press_releases.iter().enumerate() {
        println!("Press Release #{}", index + 1);
        println!("Url: {:?}", release.url);
    }
}
