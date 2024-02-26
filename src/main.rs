use postgres::Error;

pub mod db;
pub mod news_scraper;


fn main() {
    let query = db::query_db();

    println!("The query has {} rows", query.unwrap().len());

    news_scraper::scrape_press_website();
}
