use news_scraper::{NewsSite, Notification, PressReleaseLink};
use csv::Writer;

pub mod db;
pub mod news_scraper;

fn main() {
    let query = db::query_db().unwrap();
    let mut notifications: Vec<Notification> = Vec::new();

    for row in query.iter() {
        let businesswire_url = format!("https://www.businesswire.com/portal/site/home/search/?searchType=all&searchTerm={}&searchPage=1", &row.symbol);
        let businesswire_struct = PressReleaseLink {symbol: row.symbol.clone(), url: businesswire_url, news_site: NewsSite::BusinessWire};
        let mut businesswire_notifications = news_scraper::scrape_news_website(&businesswire_struct);

        if !businesswire_notifications.as_ref().unwrap().is_empty() {
            notifications.append(businesswire_notifications.as_mut().unwrap());
            continue;
        }

        let globalnewswire_url = format!("https://www.globenewswire.com/search/keyword/{}?pageSize=10", &row.symbol);
        let globalnewswire_struct = PressReleaseLink {symbol: row.symbol.clone(), url: globalnewswire_url, news_site: NewsSite::GlobalNewsWire};
        let mut globalnewswire_notifications = news_scraper::scrape_news_website(&globalnewswire_struct);

        notifications.append(globalnewswire_notifications.as_mut().unwrap());
    }
    
    let mut csv_writer = Writer::from_path("noticiations.csv").unwrap();
    csv_writer.write_record(&["Stock", "News Link"]).unwrap();

    for notification in notifications {
        let symbol = notification.symbol;
        let press_release_url = notification.press_release_url;
        csv_writer.write_record(&[symbol, press_release_url]).unwrap();
    }
    csv_writer.flush().unwrap();
}
