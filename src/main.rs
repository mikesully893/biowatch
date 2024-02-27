use db::CompanyInfoRow;
use postgres::Error;

pub mod db;
pub mod news_scraper;


fn main() {
    let mut query = db::query_db().unwrap();

    //println!("The query has {} rows", &query.unwrap().len());

    for row in query.iter() {
        //println!("{:?}", row.);
        //row.
        
    }
    // let vec_key: Vec<String> = query.unwrap()
    //     .into_iter()
    //     .map(|CompanyInfoRow { url, .. }| url).collect()
    //     ;

    //news_scraper::scrape_press_website();
}
